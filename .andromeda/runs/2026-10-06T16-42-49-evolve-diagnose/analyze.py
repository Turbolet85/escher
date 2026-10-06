import json, re, collections, sys, os

RUN = os.path.dirname(os.path.abspath(__file__))
LEDGER = os.path.join(RUN, '..', '..', 'friction-log.ndjson')
TARGET = 'Epoch 2 — Element identity'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
bare = lambda s: re.sub(r'^andromeda-', '', s or '')

def dump(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, indent=1, ensure_ascii=False)

# ---- pre-pass: tolerant parse + retractions over the WHOLE ledger
recs, skips, bad_ts = [], 0, []
for line in open(LEDGER, encoding='utf-8'):
    if not line.strip():
        continue
    try:
        r = json.loads(line)
    except ValueError:
        skips += 1; continue
    if not ISO.fullmatch(str(r.get('ts'))):
        bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    r['skill'] = bare(r.get('skill'))
    r['_epoch'] = norm(r.get('epoch'))
    recs.append(r)
for i, r in enumerate(recs):
    r['_line'] = i + 1
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
retractors = [{'id': r['id'], 'epoch': r['_epoch'], 'retracts': r['retracts'], 'what': r.get('what')} for r in recs if r.get('retracts')]
dump('q-retractions.json', {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
     'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
     'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr,
     'retraction_records': retractors})

def problems(r):
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

stream = [r for r in recs if r.get('id') not in excl_ids]
E = norm(TARGET)
ep = [r for r in stream if r['_epoch'] == E]
steps = [r for r in ep if r['kind'] == 'step']
fr = [r for r in ep if r['kind'] == 'friction']

# ---- Stage 0: health
EXPECTED = {'phase': 5, 'implement': 3, 'wrap': 5}
cov = collections.defaultdict(lambda: collections.defaultdict(list))
for r in steps:
    cov[r.get('chunk')][r['skill']].append(r['step'])
health_chunks = {}
for ch, sk in cov.items():
    row = {}
    for s, lst in sk.items():
        row[s] = {'n': len(lst), 'expected': EXPECTED.get(s), 'steps': lst}
    health_chunks[str(ch)] = row
untyped_rate = collections.defaultdict(lambda: [0, 0])
for r in fr:
    k = r['skill'] + '/' + r['step']
    untyped_rate[k][1] += 1
    if r.get('untyped'):
        untyped_rate[k][0] += 1
prob_filled = sum(1 for r in steps if problems(r))
id_fill = sum(1 for r in ep if r.get('id'))
outcomes = collections.Counter((r['skill'], r['step'], r.get('outcome')) for r in steps)
dump('q-health.json', {'epoch': TARGET, 'records': len(ep), 'step': len(steps), 'friction': len(fr),
     'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts],
     'coverage': health_chunks,
     'untyped_rate': {k: {'untyped': v[0], 'friction': v[1]} for k, v in untyped_rate.items()},
     'problem_fact_fill': [prob_filled, len(steps)], 'id_fill': [id_fill, len(ep)],
     'step_outcomes': [[*k, v] for k, v in sorted(outcomes.items(), key=str)]})

# ---- Stage 1: typed
# evolve-system.md §Universal types (nine) + recall.* (diagnosis-pass §Stage 1)
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
             'contract.skill-reference-drift', 'contract.grammar-irregularity', 'contract.jointly-contradictory-instructions'}
def wt(imp):
    imp = imp or {}
    g = lambda k: imp.get(k) or 0
    return 1 + g('iterations') + g('retries') + g('reformulations') + 2 * g('dialogue_rounds') + 3 * g('halted') + 3 * g('soft_exit')
step_runs = collections.Counter((r['skill'], r['step']) for r in steps)
groups = collections.defaultdict(list)
for r in fr:
    t = r.get('type') or 'UNTYPED'
    groups[(r['skill'], r['step'], t)].append(r)
def summarize(key, rs, denom):
    imp = collections.Counter()
    for r in rs:
        for k, v in (r.get('impact') or {}).items():
            if isinstance(v, (int, float)):
                imp[k] += v
    hs = sum(1 for r in rs if (r.get('impact') or {}).get('halted') or (r.get('impact') or {}).get('soft_exit'))
    n = len(rs)
    above = n >= 3 or (n >= 2 and hs >= 2)
    return {'key': key, 'n': n, 'step_runs': denom, 'rate': round(n / denom, 2) if denom else None,
            'impact': dict(imp), 'weight': sum(wt(r.get('impact')) for r in rs),
            'halt_or_softexit_occurrences': hs, 'above_threshold': above,
            'chunks': sorted({str(r.get('chunk')) for r in rs}),
            'cases': [{'id': r.get('id'), 'line': r['_line'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'],
                       'type': r.get('type'), 'untyped': r.get('untyped', False), 'what': r.get('what'),
                       'impact': r.get('impact'), 'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'),
                       'clause_retracted': clause_notes.get(r.get('id'))} for r in rs]}
typed = [summarize('/'.join(k), rs, step_runs.get((k[0], k[1]))) for k, rs in groups.items()]
# universal: by type alone across steps
ug = collections.defaultdict(list)
for r in fr:
    t = r.get('type') or ''
    if t in UNIVERSAL or t.startswith('recall.'):
        ug[t].append(r)
universal = [summarize('*/*/' + t, rs, None) for t, rs in ug.items()]
# by type alone generally (informational)
tg = collections.defaultdict(list)
for r in fr:
    tg[r.get('type') or 'UNTYPED'].append(r)
by_type = [summarize('type:' + t, rs, None) for t, rs in tg.items()]
for L in (typed, universal, by_type):
    L.sort(key=lambda g: (-g['n'] * g['weight'], -g['n']))
dump('q-typed.json', {'epoch': TARGET, 'step_runs': {'/'.join(k): v for k, v in step_runs.items()},
     'groups': typed, 'universal_by_type': universal, 'all_by_type_info': by_type})

# ---- Stage 2 input: untyped, this epoch + previous for recurrence
def ut(rs):
    return [{'id': r.get('id'), 'line': r['_line'], 'epoch': r['_epoch'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'],
             'type': r.get('type'), 'what': r.get('what'), 'impact': r.get('impact'), 'evidence': r.get('evidence')} for r in rs]
dump('q-untyped.json', {'target': ut([r for r in fr if r.get('untyped')]),
     'prior': ut([r for r in stream if r['kind'] == 'friction' and r.get('untyped') and r['_epoch'] != E])})

# ---- Stage 3: chains
anchors = []
for r in fr:
    if (r.get('type') or '').startswith('input.'):
        anchors.append({'src': 'friction', 'id': r.get('id'), 'line': r['_line'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'],
                        'artifact': (r.get('artifacts') or [None])[0], 'artifacts': r.get('artifacts'), 'what': r.get('what'), 'type': r.get('type')})
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'src': 'step', 'id': r.get('id'), 'line': r['_line'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'],
                            'artifact': c.get('artifact'), 'quality': c.get('quality'), 'note': c.get('note')})
chains = []
for a in anchors:
    arts = [a['artifact']] + [x for x in (a.get('artifacts') or []) if x != a['artifact']]
    prods = []
    for p in steps:
        if p.get('chunk') != a['chunk'] or p['_line'] >= a['line']:
            continue
        for pr in p.get('produced') or []:
            if pr.get('artifact') in arts:
                pf = [{'id': f.get('id'), 'type': f.get('type'), 'what': f.get('what')} for f in fr
                      if f.get('chunk') == p.get('chunk') and f['skill'] == p['skill'] and f['step'] == p['step']]
                prods.append({'id': p.get('id'), 'line': p['_line'], 'skill': p['skill'], 'step': p['step'], 'outcome': p.get('outcome'),
                              'artifact': pr.get('artifact'), 'signals': pr.get('signals'), 'producer_frictions': pf})
    chains.append({'anchor': a, 'producers': prods,
                   'hypothesis': any(p['outcome'] in ('ok', 'ok-degraded') and (p['signals'] or p['producer_frictions']) for p in prods)})
shape = collections.defaultdict(set)
for c in chains:
    for p in c['producers']:
        shape[(p['skill'] + '/' + p['step'], p['artifact'], c['anchor']['skill'] + '/' + c['anchor']['step'])].add(str(c['anchor']['chunk']))
dump('q-chains.json', {'epoch': TARGET, 'anchors': len(anchors), 'chains': chains,
     'shapes': [{'producer': k[0], 'artifact': k[1], 'consumer': k[2], 'chunks': sorted(v), 'cross_step_candidate': len(v) >= 2}
                for k, v in sorted(shape.items(), key=lambda kv: -len(kv[1]))]})

# ---- Stage 4 input: problem facts (target + prior for lookback), signals
def facts(rs):
    out = []
    for r in rs:
        if r['kind'] != 'step':
            continue
        for i, f in problems(r):
            out.append({'id': r.get('id'), 'idx': i, 'line': r['_line'], 'epoch': r['_epoch'], 'chunk': r.get('chunk'), 'skill': r['skill'],
                        'step': r['step'], 'outcome': r.get('outcome'), **(f if isinstance(f, dict) else {'raw': f})})
    return out
sigs = []
for r in steps:
    for pr in r.get('produced') or []:
        for s in pr.get('signals') or []:
            sigs.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'], 'artifact': pr.get('artifact'), 'signal': s})
dump('q-facts.json', {'target': facts(ep), 'prior': facts([r for r in stream if r['_epoch'] != E]), 'signals': sigs,
     'degraded': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'], 'outcome': r.get('outcome')}
                  for r in stream if r['kind'] == 'step' and r.get('outcome') not in ('ok', None)],
     'tooling_friction': [{'id': r.get('id'), 'epoch': r['_epoch'], 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r['step'], 'type': r.get('type'), 'what': r.get('what'), 'impact': r.get('impact')}
                          for r in stream if r['kind'] == 'friction' and (r.get('type') or '').startswith('tooling.')]})
print('ok', len(ep), len(steps), len(fr), 'excl', len(excl_ids), len(excl_probs), 'unres', len(unres))
