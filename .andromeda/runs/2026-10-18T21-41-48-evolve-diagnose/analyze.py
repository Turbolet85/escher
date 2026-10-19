import json, re, collections, sys

RUN = '.andromeda/runs/2026-10-18T21-41-48-evolve-diagnose/'
EPOCH = 'Epoch 2 - Engine debts'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('\u2013', '-').replace('\u2014', '-')
bare = lambda s: re.sub('^andromeda-', '', s or '')
vnorm = lambda v: (v if (v or '').startswith('escher-') else 'escher-' + (v or ''))

# ---- pre-pass (whole ledger) ----
recs, skips, bad_ts = [], 0, []
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        skips += 1
        continue
    if not ISO.fullmatch(str(r.get('ts'))):
        bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    r['_line'] = len(recs) + skips + 1
    r['skill'] = bare(r.get('skill'))
    r['_ep'] = (vnorm(r.get('version')), norm(r.get('epoch')))
    recs.append(r)
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs, unres, clause_notes = set(), set(), [], {}
for r in recs:
    rl = r.get('retracts')
    if not rl:
        continue
    for t in (rl if isinstance(rl, list) else [rl]):
        if not isinstance(t, dict):
            unres.append([r.get('id'), 'prose-form-pre-boundary', str(t)[:80]]); continue
        tid, scope = t.get('id'), t.get('scope')
        tgt = by_id.get(tid)
        if tid is None or tgt is None or scope not in ('record', 'problem', 'clause'):
            unres.append([r.get('id'), 'unknown-id-or-scope', json.dumps(t)[:80]]); continue
        if scope == 'record':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'record-scope-on-step', tid]); continue
            excl_ids.add(tid)
        elif scope == 'clause':
            if tgt.get('kind') == 'step':
                unres.append([r.get('id'), 'clause-scope-on-step', tid]); continue
            if not t.get('note'):
                unres.append([r.get('id'), 'clause-without-note', tid]); continue
            clause_notes.setdefault(tid, []).append(t['note'])
        else:
            excl_probs.add((tid, t.get('index')))
retr_of_retr = sorted(r['id'] for r in recs if r.get('retracts') and r.get('id') in excl_ids)
retr = {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
        'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
        'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr}
json.dump(retr, open(RUN + 'q-retractions.json', 'w'), indent=1, ensure_ascii=False)

def probs(r):
    p = r.get('problem')
    if p is None:
        return []
    p = p if isinstance(p, list) else [p]
    out = []
    for i, f in enumerate(p):
        if (r.get('id'), i) in excl_probs or (r.get('id'), None) in excl_probs:
            continue
        out.append((i, f))
    return out

filt = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]
ep = [r for r in filt if norm(r.get('epoch')) == EPOCH]
eps_versions = collections.Counter(r['_ep'][0] for r in ep)
steps = [r for r in ep if r.get('kind') == 'step']
fric = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0 ----
cov = collections.OrderedDict()
for r in steps:
    c = r.get('chunk') or 'null'
    cov.setdefault(c, collections.Counter())[r['skill'] + '/' + r['step']] += 1
step_runs = collections.Counter((r['skill'], r['step']) for r in steps)
untyped_rate = {}
for r in fric:
    k = r['skill'] + '/' + r['step']
    untyped_rate.setdefault(k, [0, 0])
    untyped_rate[k][1] += 1
    if r.get('untyped') or not r.get('type'):
        untyped_rate[k][0] += 1
pf_pop = sum(1 for r in steps if probs(r))
id_fill = sum(1 for r in ep if r.get('id'))
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
E = len(steps) - len(K)
J = [r for r in K if r['skill'] == 'wrap-session' and r['step'] == 'curation']
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
                    odd.append([r.get('id'), s])
            elif s.startswith('guard-refused') or s.startswith('exact-hit'):
                odd.append([r.get('id'), s])
signal_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records' + (f' (from 2026-10-09; {E} earlier step records are era)' if E else '')
ep_retr = [r.get('id') for r in ep if r.get('retracts')]
health = {'epoch': EPOCH, 'versions': eps_versions, 'records': len(ep), 'step': len(steps), 'friction': len(fric),
          'coverage': {c: dict(v) for c, v in cov.items()}, 'step_runs': {'/'.join(k): v for k, v in step_runs.items()},
          'untyped_rate': untyped_rate, 'problem_fact_fill': [pf_pop, len(steps)], 'id_fill': [id_fill, len(ep)],
          'signal_line': signal_line, 'signal_odd': odd, 'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts],
          'epoch_retraction_records': ep_retr,
          'epoch_records_retracted': [r.get('id') for r in recs if norm(r.get('epoch')) == EPOCH and r.get('id') in excl_ids]}
json.dump(health, open(RUN + 'q-health.json', 'w'), indent=1, ensure_ascii=False)

# ---- Stage 1 ----
def W(r):
    i = r.get('impact') or {}
    g = lambda k: (i.get(k) or 0) if isinstance(i.get(k) or 0, (int, float)) else 1
    return 1 + g('iterations') + g('retries') + g('reformulations') + 2 * g('dialogue_rounds') + 3 * g('halted') + 3 * g('soft_exit')
# evolve-system.md §Universal types (the nine); recall.* also group by type alone (diagnosis-pass Stage 1)
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
             'contract.skill-reference-drift', 'contract.grammar-irregularity',
             'contract.jointly-contradictory-instructions'}
groups = collections.OrderedDict()
def add(key, r):
    groups.setdefault(key, []).append(r)
for r in fric:
    t = r.get('type') or 'UNTYPED'
    add(r['skill'] + '/' + r['step'] + '/' + t, r)
typecount = collections.Counter(r.get('type') for r in fric if r.get('type'))
for r in fric:
    t = r.get('type')
    if t and (t.startswith('recall.') or t in UNIVERSAL):
        add('*/*/' + t, r)
typed = []
for k, rs in groups.items():
    halt = any(((r.get('impact') or {}).get('halted') or (r.get('impact') or {}).get('soft_exit')) for r in rs)
    n = len(rs)
    sk, st = k.split('/')[0], k.split('/')[1]
    typed.append({'key': k, 'n': n, 'weight': sum(W(r) for r in rs), 'halt_or_soft': bool(halt),
                  'above': n >= 3 or (n >= 2 and halt), 'rate': (f"{n}/{step_runs.get((sk, st), 0)}" if sk != '*' else None),
                  'chunks': sorted(set(r.get('chunk') or 'null' for r in rs)),
                  'cases': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'what': r.get('what'), 'impact': r.get('impact'),
                             'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'), 'untyped': r.get('untyped'),
                             'clause': clause_notes.get(r.get('id'))} for r in rs]})
typed.sort(key=lambda g: (-g['n'] * g['weight'], g['key']))
json.dump(typed, open(RUN + 'q-typed.json', 'w'), indent=1, ensure_ascii=False)

# ---- Stage 2 raw ----
untyped = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'], 'type': r.get('type'),
            'what': r.get('what')} for r in fric if r.get('untyped') or not r.get('type')]
prev_untyped = [{'ep': r['_ep'], 'id': r.get('id'), 'skill': r['skill'], 'step': r['step'], 'what': r.get('what')}
                for r in filt if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type')) and r['_ep'] == ('escher-0.2.0', 'Epoch 1 - Foundation')]
json.dump({'epoch': untyped, 'previous_epoch_untyped': prev_untyped}, open(RUN + 'q-untyped-raw.json', 'w'), indent=1, ensure_ascii=False)

# ---- Stage 3 ----
anchors = []
for r in fric:
    if (r.get('type') or '').startswith('input.'):
        anchors.append({'kind': 'friction', 'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + r['step'],
                        'type': r.get('type'), 'artifacts': r.get('artifacts'), 'what': r.get('what')})
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'kind': 'step', 'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + r['step'],
                            'artifact': c.get('artifact'), 'quality': c.get('quality'), 'note': c.get('note')})
for a in anchors:
    art = a.get('artifact')
    prods = []
    if art:
        for r in steps:
            if r.get('chunk') == a['chunk'] and r['_line'] < (by_id.get(a['id']) or {}).get('_line', 10**9):
                for p in r.get('produced') or []:
                    if p.get('artifact') == art:
                        prods.append({'id': r.get('id'), 'step': r['skill'] + '/' + r['step'], 'outcome': r.get('outcome'),
                                      'signals': p.get('signals'), 'note': p.get('note')})
    a['producers'] = prods
json.dump(anchors, open(RUN + 'q-chains.json', 'w'), indent=1, ensure_ascii=False)

# ---- Stage 4 raw ----
facts = []
for r in filt:
    if r.get('kind') != 'step':
        continue
    for i, f in probs(r):
        if not isinstance(f, dict):
            continue
        facts.append({'ep': ' / '.join(r['_ep']), 'in_epoch': norm(r.get('epoch')) == EPOCH, 'id': r.get('id'), 'idx': i,
                      'chunk': r.get('chunk'), 'step': r['skill'] + '/' + r['step'], 'outcome': r.get('outcome'),
                      'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
json.dump(facts, open(RUN + 'q-level-raw.json', 'w'), indent=1, ensure_ascii=False)
degraded = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + r['step']} for r in steps if r.get('outcome') == 'ok-degraded']

print(json.dumps({'retr': {k: (v if k != 'malformed_ts' else len(v)) for k, v in retr.items()}, 'health': health}, indent=1, ensure_ascii=False, default=str))
print('DEGRADED', degraded)
print('TYPED')
for g in typed:
    print(('ABOVE ' if g['above'] else '      ') + f"{g['key']} n={g['n']} w={g['weight']} halt={g['halt_or_soft']} rate={g['rate']} {g['chunks']}")
print('UNTYPED', len(untyped))
print('ANCHORS', len(anchors))
ef = [f for f in facts if f['in_epoch']]
print('FACTS in-epoch', len(ef), collections.Counter((f['solution'], f['nature']) for f in ef))
