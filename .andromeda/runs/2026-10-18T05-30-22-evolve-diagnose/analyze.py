# Diagnosis pass for escher-0.2.0 · Epoch 2 — Engine debts (epoch-to-date).
# Run from the project root: python3 {run_dir}/analyze.py {run_dir}
import json, re, sys, collections, os

RUN = sys.argv[1]
VERSION = 'escher-0.2.0'
EPOCH = 'Epoch 2 - Engine debts'
PRIOR = 'Epoch 1 - Foundation'  # 0.2.0's own Epoch 1 (version-qualified below)
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
             'contract.skill-reference-drift', 'contract.grammar-irregularity',
             'contract.jointly-contradictory-instructions'}
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
bare = lambda s: re.sub('^andromeda-', '', s or '')


def dump(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


# ---- retraction pre-pass (whole ledger) ----
recs, skips, bad_ts = [], 0, []
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
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
dump('q-retractions.json', {'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
     'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
     'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr})


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


filtered = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]


def in_epoch(r, version, epoch):
    if norm(r.get('epoch')) != epoch:
        return False
    v = r.get('version')
    if v:
        return v == version
    # versionless records: 0.2.0 began 2026-10-16
    ts = str(r.get('ts'))
    return (ts >= '2026-10-16') == (version == 'escher-0.2.0')


E = [r for r in filtered if in_epoch(r, VERSION, EPOCH)]
P = [r for r in filtered if in_epoch(r, VERSION, PRIOR)]
OLDER = [r for r in filtered if not in_epoch(r, VERSION, EPOCH)]
steps = [r for r in E if r.get('kind') == 'step']
fr = [r for r in E if r.get('kind') == 'friction']
for r in E + OLDER:
    r['skill_b'] = bare(r.get('skill'))

# ---- Stage 0 ----
expected = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.Counter())
for r in steps:
    cov[r.get('chunk')][(r['skill_b'], r.get('step'))] += 1
coverage = {}
for ch, c in cov.items():
    per_skill = collections.Counter()
    for (sk, st), n in c.items():
        per_skill[sk] += n
    coverage[str(ch)] = {'per_skill': dict(per_skill), 'steps': {f'{a}/{b}': n for (a, b), n in c.items()},
                         'gaps': {sk: expected[sk] - per_skill.get(sk, 0) for sk in expected
                                  if ch and per_skill.get(sk, 0) < expected[sk]}}
untyped_rate = collections.Counter()
fr_per_step = collections.Counter()
for r in fr:
    fr_per_step[(r['skill_b'], r.get('step'))] += 1
    if r.get('untyped') or not r.get('type'):
        untyped_rate[(r['skill_b'], r.get('step'))] += 1
pf_fill = sum(1 for r in steps if problems(r))
id_fill = sum(1 for r in E if r.get('id'))
# signal line
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
Eera = len(steps) - len(K)
J = [r for r in K if r['skill_b'] == 'wrap-session' and r.get('step') == 'curation']
Jids = {id(r) for r in J}
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
                    if not m:
                        odd.append(s)
                    elif word == 'exact-hit' and id(r) not in Jids:
                        odd.append(s)
                    elif word == 'guard-refused':
                        N += int(m.group(1))
                    else:
                        M += int(m.group(1))
signal = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if Eera:
    signal += f' (from 2026-10-09; {Eera} earlier step records are era)'
outcomes = collections.Counter((r['skill_b'], r.get('step'), r.get('outcome')) for r in steps)
health = {'records': len(E), 'step': len(steps), 'friction': len(fr), 'unparseable': skips,
          'malformed_ts_in_epoch': [b for b in bad_ts if False], 'malformed_ts_ledger': len(bad_ts),
          'coverage': coverage, 'friction_per_step': {f'{a}/{b}': n for (a, b), n in fr_per_step.items()},
          'untyped_per_step': {f'{a}/{b}': n for (a, b), n in untyped_rate.items()},
          'problem_fact_fill': [pf_fill, len(steps)], 'id_fill': [id_fill, len(E)], 'signal_line': signal,
          'signal_odd': odd,
          'step_outcomes': {f'{a}/{b}/{c}': n for (a, b, c), n in outcomes.items()},
          'retracted_in_epoch': [i for i in excl_ids if by_id.get(i) in E or (by_id.get(i) and in_epoch(by_id[i], VERSION, EPOCH))],
          'retractions_ledger': {'retracted': len(excl_ids), 'unresolvable': len(unres),
                                 'clause': len(clause_notes), 'retr_of_retr': retr_of_retr},
          'null_epoch_records': [[r.get('ts'), r.get('kind'), r.get('skill'), r.get('step')] for r in recs if not r.get('epoch')]}
dump('q-health.json', health)


# ---- Stage 1 ----
def weight(r):
    i = r.get('impact') or {}
    return (1 + i.get('iterations', 0) + i.get('retries', 0) + i.get('reformulations', 0)
            + 2 * i.get('dialogue_rounds', 0) + 3 * i.get('halted', 0) + 3 * i.get('soft_exit', 0))


def haltish(r):
    i = r.get('impact') or {}
    return bool(i.get('halted') or i.get('soft_exit'))


step_runs = collections.Counter((r['skill_b'], r.get('step')) for r in steps)
groups = collections.defaultdict(list)
for r in fr:
    t = r.get('type') or 'UNTYPED'
    groups[(r['skill_b'], r.get('step'), t)].append(r)
    if t in UNIVERSAL or t.startswith('recall.'):
        groups[('*', '*', t)].append(r)
typed = []
for (sk, st, t), rs in groups.items():
    n = len(rs)
    w = sum(weight(r) for r in rs)
    above = n >= 3 or (n >= 2 and any(haltish(r) for r in rs))
    typed.append({'key': f'{sk}/{st}/{t}', 'n': n, 'weight': w, 'score': n * w / n if n else 0,
                  'rate': (f'{n}/{step_runs[(sk, st)]}' if sk != '*' else None),
                  'chunks': sorted({str(r.get('chunk')) for r in rs}), 'above': above,
                  'halt_or_soft': sum(1 for r in rs if haltish(r)),
                  'cases': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{r['skill_b']}/{r.get('step')}",
                             'what': r.get('what'), 'impact': r.get('impact'), 'evidence': r.get('evidence'),
                             'artifacts': r.get('artifacts'), 'clause': clause_notes.get(r.get('id'))} for r in rs]})
typed.sort(key=lambda g: (-g['n'] * g['weight'], g['key']))
dump('q-typed.json', typed)

# ---- Stage 2 material ----
untyped = [r for r in fr if r.get('untyped') or not r.get('type')]
prior_untyped = [r for r in P if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))]
dump('q-untyped-raw.json', {'epoch': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{bare(r.get('skill'))}/{r.get('step')}",
                                       'type': r.get('type'), 'what': r.get('what'), 'impact': r.get('impact'),
                                       'evidence': r.get('evidence')} for r in untyped],
                            'prior_epoch': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{bare(r.get('skill'))}/{r.get('step')}",
                                             'what': r.get('what')} for r in prior_untyped]})

# ---- Stage 3 ----
anchors = []
for r in fr:
    if (r.get('type') or '').startswith('input.'):
        anchors.append(('friction', r, r.get('artifacts') or []))
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append(('verdict', r, [c.get('artifact')], c))
chains = []
order = {id(r): i for i, r in enumerate(E)}
for a in anchors:
    kind, r = a[0], a[1]
    arts = a[2]
    producers = []
    for s in steps:
        if s.get('chunk') != r.get('chunk') or order[id(s)] >= order[id(r)]:
            continue
        prod = {p.get('artifact'): p for p in s.get('produced') or []}
        for art in arts:
            if art in prod:
                pf = [f for f in fr if f.get('chunk') == s.get('chunk') and f['skill_b'] == s['skill_b'] and f.get('step') == s.get('step')]
                producers.append({'producer': f"{s['skill_b']}/{s.get('step')}", 'id': s.get('id'), 'artifact': art,
                                  'outcome': s.get('outcome'), 'signals': prod[art].get('signals'),
                                  'producer_frictions': [f.get('type') for f in pf]})
    entry = {'anchor_kind': kind, 'chunk': r.get('chunk'), 'consumer': f"{r['skill_b']}/{r.get('step')}", 'id': r.get('id'),
             'artifacts': arts, 'producers': producers}
    if kind == 'friction':
        entry['type'] = r.get('type'); entry['what'] = r.get('what')
    else:
        entry['verdict'] = a[3]
        entry['operator_pass'] = [c for c in r.get('consumed') or [] if c.get('artifact') == 'operator-pass']
    chains.append(entry)
shape = collections.defaultdict(set)
for c in chains:
    for p in c['producers']:
        shape[(p['producer'], p['artifact'], c['consumer'])].add(c['chunk'])
dump('q-chains.json', {'anchors': chains,
                       'shapes': [{'shape': list(k), 'chunks': sorted(v), 'pattern_candidate': len(v) >= 2} for k, v in shape.items()]})

# ---- Stage 4 material ----
facts = []
for src, pool in (('epoch', E), ('prior', P)):
    for r in pool:
        if r.get('kind') != 'step':
            continue
        for i, f in problems(r):
            if not isinstance(f, dict):
                f = {'note': str(f)}
            facts.append({'src': src, 'id': r.get('id'), 'index': i, 'chunk': r.get('chunk'),
                          'step': f"{bare(r.get('skill'))}/{r.get('step')}", **f})
outcome_degraded = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{r['skill_b']}/{r.get('step')}", 'outcome': r.get('outcome')}
                    for r in steps if r.get('outcome') not in ('ok', None)]
deferral_signals = []
for r in steps:
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            if isinstance(s, str) and s.startswith('deferral'):
                deferral_signals.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{r['skill_b']}/{r.get('step')}", 'signal': s, 'artifact': p.get('artifact')})
all_signals = collections.Counter()
for r in steps:
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            all_signals[str(s).split(':')[0]] += 1
older_tooling = collections.Counter()
for r in OLDER:
    if r.get('kind') == 'friction' and (r.get('type') or '').startswith('tooling.'):
        older_tooling[(norm(r.get('epoch')), r.get('version'), r.get('type'))] += 1
dump('q-level-raw.json', {'facts': facts, 'non_ok_outcomes': outcome_degraded, 'deferral_signals': deferral_signals,
                          'signal_words': dict(all_signals),
                          'tooling_types_older': [[*k, v] for k, v in sorted(older_tooling.items(), key=str)]})

print(json.dumps({'records': len(E), 'step': len(steps), 'friction': len(fr), 'chunks': sorted({str(r.get('chunk')) for r in E}),
                  'signal': signal, 'groups': len([g for g in typed]), 'above': [g['key'] for g in typed if g['above']],
                  'untyped': len(untyped), 'prior_untyped': len(prior_untyped), 'anchors': len(chains), 'facts_epoch': sum(1 for f in facts if f['src'] == 'epoch'),
                  'facts_prior': sum(1 for f in facts if f['src'] == 'prior'), 'unres': len(unres)}, ensure_ascii=False, indent=1))
