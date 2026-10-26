"""Diagnosis pass analysis for escher-0.2.0 · Epoch 5 — Polish & ship.

Run by path from the repo root. Writes q-health.json, q-typed.json, q-chains.json and
dump-untyped.json / dump-problems.json beside itself; prints a summary.
"""
import collections
import json
import os
import re
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
LEDGER = '.andromeda/friction-log.ndjson'
EPOCH = 'Epoch 5 - Polish & ship'
VERSION = 'escher-0.2.0'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
# evolve-system.md §Universal types — the nine cross-step classes, grouped by type alone
UNIVERSAL_TYPES = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
    'contract.skill-reference-drift', 'contract.grammar-irregularity', 'contract.jointly-contradictory-instructions',
}


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


def bare(skill):
    return re.sub('^andromeda-', '', skill or '')


def as_list(p):
    if p is None:
        return []
    return p if isinstance(p, list) else [p]


retr = json.load(open(os.path.join(RUN, 'q-retractions.json'), encoding='utf-8'))
excl_ids = set(retr['retracted_ids'])
excl_probs = {(a, b) for a, b in retr['retracted_problems']}
clause = retr['clause_retracted']

all_recs, skips = [], 0
for line in open(LEDGER, encoding='utf-8'):
    try:
        all_recs.append(json.loads(line))
    except ValueError:
        skips += 1

in_epoch = [r for r in all_recs if norm(r.get('epoch')) == EPOCH and r.get('version') == VERSION]
recs = [r for r in in_epoch if r.get('id') not in excl_ids]
for r in recs:
    r['skill'] = bare(r.get('skill'))
steps = [r for r in recs if r['kind'] == 'step']
fric = [r for r in recs if r['kind'] == 'friction']

# ---------- Stage 0 ----------
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(collections.Counter)
for s in steps:
    cov[s.get('chunk')][s['skill']] += 1
coverage = {}
for ch, c in cov.items():
    if ch is None:
        coverage['(session starts)'] = {'new-session': c.get('new-session', 0)}
        continue
    coverage[ch] = {k: {'got': c.get(k, 0), 'expected': v} for k, v in EXPECTED.items()}
step_runs = collections.Counter((s['skill'], s['step']) for s in steps)
untyped_by_step = collections.Counter((f['skill'], f['step']) for f in fric if f.get('untyped') or not f.get('type'))
fric_by_step = collections.Counter((f['skill'], f['step']) for f in fric)
untyped_rate = {f'{k[0]}/{k[1]}': f'{untyped_by_step.get(k, 0)}/{fric_by_step[k]}' for k in fric_by_step}
prob_fill = sum(1 for s in steps if as_list(s.get('problem')))
id_fill = sum(1 for r in in_epoch if r.get('id'))
K = [s for s in steps if ISO.fullmatch(str(s.get('ts'))) and s['ts'][:10] >= '2026-10-09']
E = len(steps) - len(K)
J = [s for s in K if s['skill'] == 'wrap-session' and s['step'] == 'curation']
N = M = 0
odd = []
for s in K:
    for p in s.get('produced') or []:
        for sig in p.get('signals') or []:
            if not isinstance(sig, str):
                continue
            m = re.fullmatch(r'(guard-refused|exact-hit):(\d+)', sig)
            if m:
                if m.group(1) == 'guard-refused':
                    N += int(m.group(2))
                elif s in J:
                    M += int(m.group(2))
                else:
                    odd.append(sig)
            elif sig.startswith('guard-refused') or sig.startswith('exact-hit'):
                odd.append(sig)
signal_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if E:
    signal_line += f' (from 2026-10-09; {E} earlier step records are era)'
outcomes = collections.Counter(f"{s['skill']}/{s['step']}:{s.get('outcome')}" for s in steps)
health = {
    'epoch': EPOCH, 'version': VERSION, 'records_in_epoch': len(in_epoch),
    'records_after_retractions': len(recs), 'step': len(steps), 'friction': len(fric),
    'retracted_in_epoch': sorted(i for i in excl_ids if any(r.get('id') == i for r in in_epoch)),
    'clause_retracted_in_epoch': {k: v for k, v in clause.items() if any(r.get('id') == k for r in in_epoch)},
    'unparseable': skips, 'malformed_ts': retr['malformed_ts'], 'coverage': coverage,
    'untyped_rate_per_step': untyped_rate, 'problem_fact_fill': f'{prob_fill}/{len(steps)}',
    'id_fill': f'{id_fill}/{len(in_epoch)}', 'signal_line': signal_line, 'signal_odd': odd,
    'non_ok_outcomes': {k: v for k, v in outcomes.items() if not k.endswith(':ok')},
    'unresolvable_retractions_whole_ledger': retr['unresolvable'],
}
json.dump(health, open(os.path.join(RUN, 'q-health.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)


# ---------- Stage 1 ----------
def imp(f, k):
    return (f.get('impact') or {}).get(k, 0) or 0


def weight(f):
    return (1 + imp(f, 'iterations') + imp(f, 'retries') + imp(f, 'reformulations')
            + 2 * imp(f, 'dialogue_rounds') + 3 * imp(f, 'halted') + 3 * imp(f, 'soft_exit'))


def is_universal(t):
    return t in UNIVERSAL_TYPES or (t or '').startswith('recall.')


groups = collections.defaultdict(list)
for f in fric:
    t = f.get('type') or 'UNTYPED'
    if is_universal(t):
        groups[('*', '*', t)].append(f)
    else:
        groups[(f['skill'], f['step'], t)].append(f)
typed = []
for (sk, st, t), fs in groups.items():
    n = len(fs)
    halt = any(imp(f, 'halted') or imp(f, 'soft_exit') for f in fs)
    w = sum(weight(f) for f in fs)
    denom = step_runs.get((sk, st)) if sk != '*' else None
    row = {
        'key': f'{sk}/{st}/{t}', 'n': n, 'weight': w, 'score': n * w,
        'rate': f'{n}/{denom}' if denom else None,
        'chunks': sorted({str(f.get('chunk')) for f in fs}),
        'iters': sum(imp(f, 'iterations') for f in fs), 'retries': sum(imp(f, 'retries') for f in fs),
        'dlg': sum(imp(f, 'dialogue_rounds') for f in fs), 'halts': sum(imp(f, 'halted') for f in fs),
        'softx': sum(imp(f, 'soft_exit') for f in fs), 'halt_impact': halt,
        'above_threshold': n >= 3 or (n >= 2 and halt),
        'cases': [{'id': f.get('id'), 'chunk': f.get('chunk'), 'step': f.get('step'), 'what': f.get('what'),
                   'impact': f.get('impact'), 'evidence': f.get('evidence'), 'artifacts': f.get('artifacts'),
                   'clause_retracted': clause.get(f.get('id'))} for f in fs],
    }
    if t == 'contract.grammar-irregularity':
        row['sites'] = dict(collections.Counter((f.get('artifacts') or ['?'])[0] for f in fs))
    typed.append(row)
typed.sort(key=lambda r: (-r['score'], -r['n']))
json.dump(typed, open(os.path.join(RUN, 'q-typed.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

# ---------- Stage 3 ----------
order = {id(r): i for i, r in enumerate(recs)}
anchors = []
for f in fric:
    if (f.get('type') or '').startswith('input.'):
        anchors.append({'kind': 'friction', 'rec': f, 'artifacts': [a for a in (f.get('artifacts') or [])]})
for s in steps:
    for c in s.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'kind': 'consumed', 'rec': s, 'artifacts': [c.get('artifact')], 'quality': c.get('quality'),
                            'note': c.get('note')})
# consumed artifact for an input.* friction: match the consumed entries of its checkpoint's step record
step_by_ckpt = {(s.get('chunk'), s['skill'], s['step']): s for s in steps}
chains = []
for a in anchors:
    r = a['rec']
    ch = r.get('chunk')
    cons_step = step_by_ckpt.get((ch, r['skill'], r['step']))
    arts = a['artifacts']
    if a['kind'] == 'friction' and cons_step:
        # take the consumed artifacts graded below ok, else all consumed
        arts = [c['artifact'] for c in cons_step.get('consumed') or [] if c.get('quality') in ('thin', 'wrong', 'missing')] or arts
    for art in arts:
        producers = [s for s in steps if s.get('chunk') == ch and order[id(s)] < order[id(cons_step or r)]
                     and any(p.get('artifact') == art for p in s.get('produced') or [])]
        prod = producers[-1] if producers else None
        pentry = next((p for p in (prod.get('produced') if prod else []) if p.get('artifact') == art), None)
        prod_fric = [f.get('type') for f in fric if prod and f.get('chunk') == ch and f['skill'] == prod['skill']
                     and f['step'] == prod['step']]
        op = None
        if cons_step:
            op = next((c for c in cons_step.get('consumed') or [] if c.get('artifact') == 'operator-pass'), None)
        chains.append({
            'chunk': ch, 'anchor_kind': a['kind'], 'consumer': f"{r['skill']}/{r['step']}",
            'anchor_id': r.get('id'), 'anchor_type': r.get('type'), 'anchor_what': r.get('what') or a.get('note'),
            'quality': a.get('quality'), 'artifact': art,
            'producer': f"{prod['skill']}/{prod['step']}" if prod else None, 'producer_id': prod.get('id') if prod else None,
            'producer_outcome': prod.get('outcome') if prod else None,
            'producer_signals': pentry.get('signals') if pentry else None,
            'producer_note': pentry.get('note') if pentry else None,
            'producer_frictions': prod_fric, 'operator_pass': op,
        })
shapes = collections.defaultdict(set)
for c in chains:
    if c['producer']:
        shapes[(c['producer'], c['artifact'], c['consumer'])].add(c['chunk'])
agg = [{'shape': f'{k[0]} →{k[1]}→ {k[2]}', 'chunks': sorted(v), 'k': len(v)} for k, v in shapes.items()]
agg.sort(key=lambda x: -x['k'])
json.dump({'links': chains, 'shapes': agg}, open(os.path.join(RUN, 'q-chains.json'), 'w', encoding='utf-8'),
          indent=1, ensure_ascii=False)

# ---------- dumps for Stages 2 and 4 (read-and-group by hand) ----------
untyped = [{'id': f.get('id'), 'chunk': f.get('chunk'), 'at': f"{f['skill']}/{f['step']}", 'type': f.get('type'),
            'what': f.get('what'), 'impact': f.get('impact'), 'evidence': f.get('evidence')}
           for f in fric if f.get('untyped') or not f.get('type')]
json.dump(untyped, open(os.path.join(RUN, 'dump-untyped.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
probs = []
for s in steps:
    for i, p in enumerate(as_list(s.get('problem'))):
        if (s.get('id'), i) in excl_probs or (s.get('id'), None) in excl_probs:
            continue
        if not isinstance(p, dict):
            p = {'note': str(p)}
        probs.append({'id': s.get('id'), 'idx': i, 'chunk': s.get('chunk'), 'at': f"{s['skill']}/{s['step']}",
                      'nature': p.get('nature'), 'solution': p.get('solution'), 'note': p.get('note')})
json.dump(probs, open(os.path.join(RUN, 'dump-problems.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)

# ---------- summary ----------
print('health:', json.dumps({k: health[k] for k in ('records_in_epoch', 'step', 'friction', 'problem_fact_fill',
                                                       'id_fill', 'signal_line', 'signal_odd', 'non_ok_outcomes',
                                                       'retracted_in_epoch', 'clause_retracted_in_epoch')},
                            ensure_ascii=False))
print('untyped rate:', health['untyped_rate_per_step'])
print('typed groups:', len(typed), 'above:', sum(r['above_threshold'] for r in typed))
for r in typed:
    print(f"  {'*' if r['above_threshold'] else ' '} {r['key']} n={r['n']} w={r['weight']} rate={r['rate']} "
          f"halt={r['halt_impact']} chunks={len(r['chunks'])}")
print('chain links:', len(chains), 'shapes:', [(a['shape'], a['k']) for a in agg])
print('untyped:', len(untyped), 'problem facts:', len(probs),
      collections.Counter(p['solution'] for p in probs))
sys.exit(0)
