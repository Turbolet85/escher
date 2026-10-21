"""Evolve-diagnose analysis pass: escher-0.2.0 Epoch 4. Writes q-*.json beside itself."""
import collections
import json
import os
import re
import sys

LEDGER = '.andromeda/friction-log.ndjson'
OUT = os.path.dirname(os.path.abspath(__file__))
EPOCH = 'Epoch 4 - Visual lint and real-app readiness'
PREV = 'Epoch 3 - Determinism and traces'
VERSION = 'escher-0.2.0'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
# evolve-system.md §Universal types (nine) + every recall.* type: grouped by type alone across steps
UNIVERSAL_SET = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
                 'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
                 'contract.skill-reference-drift', 'contract.grammar-irregularity',
                 'contract.jointly-contradictory-instructions'}


def universal(t):
    return t in UNIVERSAL_SET or t.startswith('recall.')
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


def bare(s):
    return re.sub('^andromeda-', '', s or '')


def save(name, obj):
    with open(os.path.join(OUT, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


# ---- pre-pass (whole ledger) ----
recs, skips, bad_ts = [], 0, []
for line in open(LEDGER, encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        skips += 1
        continue
    if not ISO.fullmatch(str(r.get('ts'))):
        bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    recs.append(r)
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs, unres, clause_notes = set(), set(), [], {}
for r in recs:
    rl = r.get('retracts')
    if not rl:
        continue
    for t in (rl if isinstance(rl, list) else [rl]):
        if not isinstance(t, dict):
            unres.append([r.get('id'), 'prose-form-pre-boundary', str(t)[:80]])
            continue
        tid, scope = t.get('id'), t.get('scope')
        tgt = by_id.get(tid)
        if tid is None or tgt is None or scope not in ('record', 'problem', 'clause'):
            unres.append([r.get('id'), 'unknown-id-or-scope', json.dumps(t)[:80]])
            continue
        if scope == 'record':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'record-scope-on-step', tid])
                continue
            excl_ids.add(tid)
        elif scope == 'clause':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'clause-scope-on-step', tid])
                continue
            if not t.get('note'):
                unres.append([r.get('id'), 'clause-without-note', tid])
                continue
            clause_notes.setdefault(tid, []).append(t['note'])
        else:
            excl_probs.add((tid, t.get('index')))
retr_of_retr = sorted(r['id'] for r in recs if r.get('retracts') and r.get('id') in excl_ids)
save('q-retractions.json', {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
                            'retracted_ids': sorted(excl_ids),
                            'retracted_problems': sorted(map(list, excl_probs), key=str),
                            'clause_retracted': clause_notes, 'unresolvable': unres,
                            'retraction_of_retraction': retr_of_retr})


def problems(r):
    p = r.get('problem')
    if p is None:
        return []
    lst = p if isinstance(p, list) else [p]
    out = []
    for i, f in enumerate(lst):
        if (r.get('id'), i) in excl_probs or (r.get('id'), None) in excl_probs:
            continue
        out.append((i, f))
    return out


def in_epoch(r, e=EPOCH):
    return norm(r.get('epoch')) == e and (r.get('version') in (None, VERSION))


filt = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]
ep = [r for r in filt if in_epoch(r)]
prev = [r for r in filt if in_epoch(r, PREV)]
steps = [r for r in ep if r.get('kind') == 'step']
fric = [r for r in ep if r.get('kind') == 'friction']
versions = collections.Counter(r.get('version') for r in ep)

# ---- Stage 0 ----
chunks = sorted({r['chunk'] for r in ep if r.get('chunk')})
cov = {}
for c in chunks:
    cnt = collections.Counter(bare(r['skill']) for r in steps if r.get('chunk') == c)
    stepnames = collections.defaultdict(list)
    for r in steps:
        if r.get('chunk') == c:
            stepnames[bare(r['skill'])].append(r['step'])
    cov[c] = {'counts': dict(cnt), 'steps': {k: v for k, v in stepnames.items()},
              'gaps': {k: EXPECTED[k] - cnt.get(k, 0) for k in EXPECTED if cnt.get(k, 0) < EXPECTED[k]}}
nochunk = [[bare(r['skill']), r['step'], r['ts']] for r in steps if not r.get('chunk')]
untyped_per_step = collections.defaultdict(lambda: [0, 0])
for r in fric:
    k = bare(r['skill']) + '/' + (r.get('step') or '')
    untyped_per_step[k][1] += 1
    if r.get('untyped') or not r.get('type'):
        untyped_per_step[k][0] += 1
pf = sum(1 for r in steps if problems(r))
idfill = sum(1 for r in ep if r.get('id'))
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r['ts'])[:10] >= '2026-10-09']
Ecount = len(steps) - len(K)
J = [r for r in K if r.get('step') == 'curation' and bare(r.get('skill')) == 'wrap-session']
N = M = 0
odd = []
for r in K:
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            if not isinstance(s, str):
                continue
            m = re.fullmatch(r'(guard-refused|exact-hit):(\d+)', s)
            if m:
                if m.group(1) == 'guard-refused':
                    N += int(m.group(2))
                elif r in J:
                    M += int(m.group(2))
                else:
                    odd.append([r['id'], s])
            elif s.startswith('guard-refused') or s.startswith('exact-hit'):
                odd.append([r['id'], s])
sig = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if Ecount:
    sig += f' (from 2026-10-09; {Ecount} earlier step records are era)'
outcomes = collections.Counter((bare(r['skill']) + '/' + r['step'], r.get('outcome')) for r in steps)
save('q-health.json', {'records': len(ep), 'step': len(steps), 'friction': len(fric),
                       'versions': dict(versions), 'chunks': chunks, 'coverage': cov,
                       'step_records_without_chunk': nochunk,
                       'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts if b[0] in chunks],
                       'untyped_per_step': {k: {'untyped': v[0], 'friction': v[1]} for k, v in untyped_per_step.items()},
                       'problem_fact_fill': [pf, len(steps)], 'id_fill': [idfill, len(ep)],
                       'signal_line': sig, 'signal_odd': odd,
                       'outcomes_non_ok': [[k[0], k[1], v] for k, v in outcomes.items() if k[1] != 'ok']})

# ---- Stage 1 ----
denom = collections.Counter(bare(r['skill']) + '/' + r['step'] for r in steps)


def imp(r, k):
    return (r.get('impact') or {}).get(k, 0) or 0


def weight(r):
    return (1 + imp(r, 'iterations') + imp(r, 'retries') + imp(r, 'reformulations')
            + 2 * imp(r, 'dialogue_rounds') + 3 * imp(r, 'halted') + 3 * imp(r, 'soft_exit'))


def case(r):
    return {'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': bare(r['skill']), 'step': r.get('step'),
            'type': r.get('type'), 'what': r.get('what'), 'impact': r.get('impact'),
            'artifacts': r.get('artifacts'), 'evidence': r.get('evidence'),
            'clause_retracted': clause_notes.get(r.get('id'))}


groups = collections.defaultdict(list)
for r in fric:
    t = r.get('type') or 'UNTYPED'
    if universal(t):
        groups['*/*/' + t].append(r)
    else:
        groups[bare(r['skill']) + '/' + (r.get('step') or '') + '/' + t].append(r)
typed = []
for k, rs in groups.items():
    sk, stp, t = k.split('/', 2)
    halt = any(imp(r, 'halted') or imp(r, 'soft_exit') for r in rs)
    n = len(rs)
    w = sum(weight(r) for r in rs)
    typed.append({'key': k, 'n': n, 'weight': w, 'score': w,
                  'rate': None if sk == '*' else f'{n}/{denom.get(sk + "/" + stp, 0)}',
                  'steps': sorted({bare(r['skill']) + '/' + r['step'] for r in rs}),
                  'chunks': sorted({r.get('chunk') for r in rs if r.get('chunk')}),
                  'halt_or_softexit': halt,
                  'above': t != 'UNTYPED' and (n >= 3 or (n >= 2 and halt)),
                  'cases': [case(r) for r in rs]})
    if t.startswith('contract.grammar-irregularity'):
        typed[-1]['sites'] = dict(collections.Counter((r.get('artifacts') or ['?'])[0] for r in rs))
typed.sort(key=lambda g: (-g['above'], -g['n'] * g['weight'], g['key']))
save('q-typed.json', {'denominators': dict(denom), 'groups': typed})

# ---- Stage 2 input ----
unt = [case(r) for r in fric if r.get('untyped') or not r.get('type')]
unt_prev = [case(r) for r in prev if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))]
save('q-untyped-raw.json', {'epoch': unt, 'previous_epoch': unt_prev})

# ---- Stage 3 ----
anchors = []
for r in fric:
    if (r.get('type') or '').startswith('input.'):
        anchors.append({'kind': 'friction', 'id': r['id'], 'chunk': r.get('chunk'),
                        'consumer': bare(r['skill']) + '/' + r['step'], 'type': r['type'],
                        'artifacts': r.get('artifacts'), 'what': r.get('what')})
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'kind': 'step', 'id': r['id'], 'chunk': r.get('chunk'),
                            'consumer': bare(r['skill']) + '/' + r['step'], 'artifact': c.get('artifact'),
                            'quality': c.get('quality'), 'note': c.get('note'),
                            'signals': c.get('signals')})
order = {id(r): i for i, r in enumerate(filt)}
chains = []
for a in anchors:
    if a['kind'] != 'step':
        continue
    cons = by_id.get(a['id'])
    prods = [r for r in steps if r.get('chunk') == a['chunk'] and order[id(r)] < order[id(cons)]
             and any(p.get('artifact') == a['artifact'] for p in r.get('produced') or [])]
    if not prods:
        chains.append({**a, 'producer': None})
        continue
    p = prods[-1]
    pe = [x for x in p.get('produced') if x.get('artifact') == a['artifact']][0]
    pf_ = [x['id'] for x in fric if x.get('chunk') == a['chunk'] and x['skill'] == p['skill'] and x['step'] == p['step']]
    chains.append({**a, 'producer': bare(p['skill']) + '/' + p['step'], 'producer_id': p['id'],
                   'producer_outcome': p.get('outcome'), 'producer_signals': pe.get('signals'),
                   'producer_note': pe.get('note'), 'producer_frictions': pf_})
shape = collections.defaultdict(list)
for c in chains:
    shape[(c.get('producer'), c.get('artifact'), c['consumer'])].append(c['chunk'])
save('q-chains.json', {'anchors': anchors, 'chains': chains,
                       'shapes': [{'producer': k[0], 'artifact': k[1], 'consumer': k[2],
                                   'chunks': sorted(set(v)), 'n': len(v)} for k, v in shape.items()]})

# ---- Stage 4 input ----
facts = []
for r in steps:
    for i, f in problems(r):
        facts.append({'id': r['id'], 'index': i, 'chunk': r.get('chunk'), 'step': bare(r['skill']) + '/' + r['step'],
                      'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
facts_prev = []
for r in prev:
    if r.get('kind') != 'step':
        continue
    for i, f in problems(r):
        facts_prev.append({'id': r['id'], 'index': i, 'chunk': r.get('chunk'), 'step': bare(r['skill']) + '/' + r['step'],
                           'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
deferral_signals = [[r['id'], r.get('chunk'), bare(r['skill']) + '/' + r['step'], p.get('artifact'), p.get('note')]
                    for r in steps for p in r.get('produced') or [] if 'deferral-open' in (p.get('signals') or [])]
degraded = [[r['id'], r.get('chunk'), bare(r['skill']) + '/' + r['step'], r.get('outcome')]
            for r in steps if r.get('outcome') not in ('ok', None)]
save('q-facts-raw.json', {'epoch': facts, 'previous_epoch': facts_prev,
                          'deferral_open': deferral_signals, 'non_ok_outcomes': degraded,
                          'solution_counts': dict(collections.Counter(f['solution'] for f in facts))})
print(json.dumps({'records': len(ep), 'steps': len(steps), 'fric': len(fric), 'versions': dict(versions),
                  'retracted': len(excl_ids), 'unres': len(unres), 'clause': len(clause_notes),
                  'signal': sig, 'groups': len(typed), 'above': sum(g['above'] for g in typed),
                  'untyped': len(unt), 'untyped_prev': len(unt_prev), 'anchors': len(anchors),
                  'facts': len(facts), 'facts_prev': len(facts_prev)}, ensure_ascii=False))
