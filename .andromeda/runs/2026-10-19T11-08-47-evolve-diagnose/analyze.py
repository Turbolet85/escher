"""Evolve-diagnose analysis pass — escher-0.2.0 · Epoch 3 — Determinism and traces.

Run by path from the project root: python3 {run_dir}/analyze.py {run_dir}
Writes q-retractions.json, q-health.json, q-typed.json, q-chains.json and the raw
dumps q-untyped-raw.json / q-problems-raw.json that Stages 2 and 4 read by hand.
"""
import collections
import json
import re
import sys

RUN = sys.argv[1]
LEDGER = '.andromeda/friction-log.ndjson'
VERSION = 'escher-0.2.0'
EPOCH = 'Epoch 3 - Determinism and traces'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
UNIVERSAL_PREFIXES = ('contract.',)  # universal types group by type alone too


def norm_epoch(s):
    s = re.sub(r'^\s*#+\s*', '', str(s or '')).strip()
    return re.sub('[\u2010-\u2015\u2212]', '-', s)


def norm_version(v):
    v = str(v or '')
    return v if v.startswith('escher-') else 'escher-' + v


def skill(r):
    return re.sub('^andromeda-', '', r.get('skill') or '')


def problems(r):
    p = r.get('problem')
    if p is None:
        return []
    return p if isinstance(p, list) else [p]


def dump(name, obj):
    with open(f'{RUN}/{name}', 'w', encoding='utf-8') as f:
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
retraction_records = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'retracts': r.get('retracts')}
                      for r in recs if r.get('retracts')]
dump('q-retractions.json', {
    'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
    'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
    'clause_retracted': clause_notes, 'unresolvable': unres,
    'retraction_of_retraction': retr_of_retr, 'retraction_records': retraction_records})


def filtered(rs):
    out = []
    for r in rs:
        if r.get('kind') == 'friction' and r.get('id') in excl_ids:
            continue
        if r.get('kind') == 'step' and any(p[0] == r.get('id') for p in excl_probs):
            r = dict(r)
            ps = problems(r)
            drop_all = (r.get('id'), None) in excl_probs
            r['problem'] = [] if drop_all else [p for i, p in enumerate(ps) if (r.get('id'), i) not in excl_probs]
        out.append(r)
    return out


allf = filtered(recs)
ep = [r for r in allf if norm_version(r.get('version')) == VERSION and norm_epoch(r.get('epoch')) == EPOCH]
steps = [r for r in ep if r.get('kind') == 'step']
fric = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0 ----
expected = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.Counter())
cov_steps = collections.defaultdict(list)
for r in steps:
    cov[r.get('chunk')][skill(r)] += 1
    cov_steps[r.get('chunk')].append(f"{skill(r)}/{r.get('step')}")
coverage = {}
for ch, c in cov.items():
    coverage[str(ch)] = {'counts': dict(c), 'steps': cov_steps[ch],
                         'gaps': {s: e - c.get(s, 0) for s, e in expected.items() if ch and c.get(s, 0) < e}}
step_runs = collections.Counter((skill(r), r.get('step')) for r in steps)
untyped_rate = {}
for (sk, st), n in step_runs.items():
    u = sum(1 for f in fric if skill(f) == sk and f.get('step') == st and f.get('untyped'))
    untyped_rate[f'{sk}/{st}'] = {'step_runs': n, 'untyped': u,
                                  'friction': sum(1 for f in fric if skill(f) == sk and f.get('step') == st)}
fill = sum(1 for r in steps if problems(r))
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
E = len(steps) - len(K)
J = [r for r in K if skill(r) == 'wrap-session' and r.get('step') == 'curation']
N = M = 0
odd = []
for r in K:
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            if not isinstance(s, str):
                continue
            for word in ('guard-refused', 'exact-hit'):
                if s.startswith(word):
                    m = re.fullmatch(word + r':(\d+)', s)
                    if not m or (word == 'exact-hit' and r not in J):
                        odd.append([r.get('id'), s])
                    elif word == 'guard-refused':
                        N += int(m.group(1))
                    else:
                        M += int(m.group(1))
signal_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if E:
    signal_line += f' (from 2026-10-09; {E} earlier step records are era)'
health = {'epoch_records': len(ep), 'step': len(steps), 'friction': len(fric),
          'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts if b[0] in cov],
          'malformed_ts_ledger': len(bad_ts), 'coverage': coverage, 'untyped_rate': untyped_rate,
          'problem_fill': f'{fill}/{len(steps)}',
          'id_fill': f"{sum(1 for r in ep if r.get('id'))}/{len(ep)}",
          'signal_line': signal_line, 'signal_odd': odd,
          'retracted_in_ledger': len(excl_ids), 'unresolvable': len(unres)}
dump('q-health.json', health)

# ---- Stage 1 ----


def weight(imp):
    imp = imp or {}
    return (1 + imp.get('iterations', 0) + imp.get('retries', 0) + imp.get('reformulations', 0)
            + 2 * imp.get('dialogue_rounds', 0) + 3 * imp.get('halted', 0) + 3 * imp.get('soft_exit', 0))


def group(key_fn):
    g = collections.defaultdict(list)
    for f in fric:
        k = key_fn(f)
        if k:
            g[k].append(f)
    out = []
    for k, fs in g.items():
        sk, st = skill(fs[0]), fs[0].get('step')
        hs = sum(1 for f in fs if (f.get('impact') or {}).get('halted') or (f.get('impact') or {}).get('soft_exit'))
        n = len(fs)
        out.append({'key': k, 'n': n, 'weight': sum(weight(f.get('impact')) for f in fs),
                    'rate': f"{n}/{step_runs.get((sk, st), 0)}" if '/' in k.split(' ')[0] and k.count('/') == 2 else None,
                    'halt_or_soft': hs, 'above': n >= 3 or (n >= 2 and hs >= 1),
                    'chunks': sorted({str(f.get('chunk')) for f in fs}),
                    'cases': [{'id': f.get('id'), 'chunk': f.get('chunk'), 'step': f"{skill(f)}/{f.get('step')}",
                               'what': f.get('what'), 'impact': f.get('impact'), 'evidence': f.get('evidence'),
                               'artifacts': f.get('artifacts'),
                               'clause_retracted': clause_notes.get(f.get('id'))} for f in fs]})
    return sorted(out, key=lambda x: -(x['n'] * x['weight']))


per_step = group(lambda f: f"{skill(f)}/{f.get('step')}/{f.get('type') or 'UNTYPED'}")
cross = group(lambda f: f"TYPE-ALONE {f.get('type')}" if f.get('type') and (
    f['type'].startswith(UNIVERSAL_PREFIXES) or f['type'].startswith('recall.')) else None)
by_type_any = group(lambda f: f"TYPE-ANY {f.get('type')}" if f.get('type') else None)
dump('q-typed.json', {'per_step': per_step, 'universal_and_recall_by_type': cross,
                      'all_types_by_type_lookout': [g for g in by_type_any if g['n'] >= 2]})

# ---- Stage 2 / 4 raw dumps ----
dump('q-untyped-raw.json', [{'id': f.get('id'), 'chunk': f.get('chunk'), 'step': f"{skill(f)}/{f.get('step')}",
                             'type': f.get('type'), 'what': f.get('what'), 'impact': f.get('impact'),
                             'evidence': f.get('evidence')} for f in fric if f.get('untyped')])
pfacts = []
for r in steps:
    for i, p in enumerate(problems(r)):
        pfacts.append({'step_id': r.get('id'), 'index': i, 'chunk': r.get('chunk'),
                       'step': f"{skill(r)}/{r.get('step')}", **p})
dump('q-problems-raw.json', pfacts)
# lookback: prior epochs' workaround/prohibition/removed-cause/deferred/overridden facts
look = []
for r in allf:
    if r.get('kind') != 'step' or r in steps:
        continue
    for i, p in enumerate(problems(r)):
        if p.get('solution') in ('workaround', 'prohibition', 'removed-cause', 'deferred', 'overridden'):
            look.append({'version': norm_version(r.get('version')), 'epoch': norm_epoch(r.get('epoch')),
                         'chunk': r.get('chunk'), 'step': f"{skill(r)}/{r.get('step')}", **p})
dump('q-problems-lookback-raw.json', look)
look_u = [{'version': norm_version(f.get('version')), 'epoch': norm_epoch(f.get('epoch')), 'chunk': f.get('chunk'),
           'step': f"{skill(f)}/{f.get('step')}", 'what': f.get('what')}
          for f in allf if f.get('kind') == 'friction' and f.get('untyped') and f not in fric
          and norm_version(f.get('version')) == VERSION]
dump('q-untyped-lookback-raw.json', look_u)

# ---- Stage 3 ----
anchors = []
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append((r, c))
for f in fric:
    if (f.get('type') or '').startswith('input.'):
        for a in (f.get('artifacts') or [None])[:1]:
            anchors.append((f, {'artifact': a, 'quality': 'input-friction', 'note': f.get('what')}))
order = {id(r): i for i, r in enumerate(ep)}
chains = []
for cons, c in anchors:
    art = c.get('artifact')
    producers = [p for p in steps if p.get('chunk') == cons.get('chunk') and order[id(p)] < order[id(cons)]
                 and any(x.get('artifact') == art for x in (p.get('produced') or []))]
    prod = producers[-1] if producers else None
    pf = [f for f in fric if prod and f.get('chunk') == prod.get('chunk') and skill(f) == skill(prod)
          and f.get('step') == prod.get('step')]
    ops = [x for x in (cons.get('consumed') or []) if x.get('artifact') == 'operator-pass']
    chains.append({'chunk': cons.get('chunk'), 'consumer': f"{skill(cons)}/{cons.get('step')}",
                   'consumer_id': cons.get('id'), 'artifact': art, 'quality': c.get('quality'),
                   'note': c.get('note'),
                   'producer': f"{skill(prod)}/{prod.get('step')}" if prod else None,
                   'producer_id': prod.get('id') if prod else None,
                   'producer_outcome': prod.get('outcome') if prod else None,
                   'producer_signals': [x.get('signals') for x in (prod.get('produced') or [])
                                        if x.get('artifact') == art] if prod else None,
                   'producer_frictions': [f.get('type') for f in pf],
                   'operator_pass': ops})
shapes = collections.defaultdict(set)
for ch in chains:
    shapes[(ch['producer'], ch['artifact'], ch['consumer'])].add(ch['chunk'])
dump('q-chains.json', {'anchors': chains,
                       'shapes': [{'shape': list(k), 'chunks': sorted(map(str, v)), 'candidate': len(v) >= 2}
                                  for k, v in shapes.items()]})

print(json.dumps({'health': {k: v for k, v in health.items() if k != 'coverage'},
                  'coverage': {k: (v['counts'], v['gaps']) for k, v in coverage.items()},
                  'typed_above': [(g['key'], g['n'], g['weight'], g['halt_or_soft']) for g in per_step + cross if g['above']],
                  'typed_all': [(g['key'], g['n'], g['weight'], g['halt_or_soft']) for g in per_step],
                  'cross': [(g['key'], g['n'], g['weight']) for g in cross],
                  'lookout': [(g['key'], g['n'], g['chunks']) for g in by_type_any if g['n'] >= 2],
                  'chains': [(c['chunk'], c['producer'], c['artifact'], c['consumer'], c['quality']) for c in chains],
                  'untyped': len([f for f in fric if f.get('untyped')]), 'pfacts': len(pfacts)},
                 ensure_ascii=False, indent=1))
