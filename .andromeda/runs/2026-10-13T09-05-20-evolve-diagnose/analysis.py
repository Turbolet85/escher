"""Diagnosis pass analysis — escher-0.1.0 · Epoch 5 - Agent surfaces.

Run from the project root: python3 -I {run_dir}/analysis.py {run_dir} {scratch_dir}
Writes q-retractions.json, q-health.json, q-typed.json, q-chains.json into run_dir;
working dumps (untyped records, problem facts) into scratch_dir for the hand read.
"""
import json, re, sys, collections, os

RUN, SCR = sys.argv[1], sys.argv[2]
EPOCH = 'Epoch 5 - Agent surfaces'
LEDGER = '.andromeda/friction-log.ndjson'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
fold = lambda s: re.sub('^andromeda-', '', s or '')


def dump(path, obj):
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


# ---- pre-pass (canonical, whole ledger) ----
recs, skips, bad_ts = [], 0, []
for line in open(LEDGER, encoding='utf-8'):
    try: r = json.loads(line)
    except ValueError: skips += 1; continue
    if not ISO.fullmatch(str(r.get('ts'))): bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    recs.append(r)
by_id = {r['id']: r for r in recs if r.get('id')}
excl_ids, excl_probs, unres, clause_notes = set(), set(), [], {}
for r in recs:
    rl = r.get('retracts')
    if not rl: continue
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
dump(f'{RUN}/q-retractions.json', retr)

# ---- filtered stream ----
for r in recs:
    r['skill'] = fold(r.get('skill'))
    p = r.get('problem')
    p = [] if p is None else (p if isinstance(p, list) else [p])
    rid = r.get('id')
    if (rid, None) in excl_probs:
        p = []
    else:
        p = [f for i, f in enumerate(p) if (rid, i) not in excl_probs]
    r['_problem'] = p
flt = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]
ep = [r for r in flt if norm(r.get('epoch')) == EPOCH]
steps = [r for r in ep if r.get('kind') == 'step']
fr = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0 health ----
EXP = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.Counter())
for s in steps:
    cov[s.get('chunk')][s['skill']] += 1
cov_rows = []
for ch, c in sorted(cov.items(), key=lambda kv: str(kv[0])):
    cov_rows.append({'chunk': ch, 'by_skill': dict(c),
                     'gaps': {k: EXP[k] - c.get(k, 0) for k in EXP if ch and c.get(k, 0) < EXP[k]}})
step_steps = collections.Counter((s['skill'], s.get('step')) for s in steps)
untyped_by_step = collections.Counter((f['skill'], f.get('step')) for f in fr if f.get('untyped') or not f.get('type'))
fr_by_step = collections.Counter((f['skill'], f.get('step')) for f in fr)
prob_fill = sum(1 for s in steps if s['_problem'])
id_fill = sum(1 for r in ep if r.get('id'))
# signal line
K = [s for s in steps if ISO.fullmatch(str(s.get('ts'))) and str(s.get('ts')) >= '2026-10-09']
E = len(steps) - len(K)
J = [s for s in K if s.get('step') == 'curation' and s['skill'] == 'wrap-session']
Jids = {id(s) for s in J}
N = M = 0; odd = []
for s in K:
    for pr in s.get('produced') or []:
        for sig in pr.get('signals') or []:
            if not isinstance(sig, str): continue
            for w in ('guard-refused', 'exact-hit'):
                if sig.startswith(w):
                    m = re.fullmatch(w + r':(\d+)', sig)
                    if not m: odd.append(sig); continue
                    if w == 'guard-refused': N += int(m.group(1))
                    elif id(s) in Jids: M += int(m.group(1))
                    else: odd.append(sig)
sig_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if E: sig_line += f' (from 2026-10-09; {E} earlier step records are era)'
gr_per = []
for s in K:
    for pr in s.get('produced') or []:
        for sig in pr.get('signals') or []:
            if isinstance(sig, str) and re.fullmatch(r'guard-refused:\d+', sig) and sig != 'guard-refused:0':
                gr_per.append([s.get('chunk'), s['skill'], s.get('step'), sig])
health = {'epoch': EPOCH, 'records': len(ep), 'step': len(steps), 'friction': len(fr),
          'friction_retracted_in_epoch': sum(1 for r in recs if r.get('kind') == 'friction' and r.get('id') in excl_ids and norm(r.get('epoch')) == EPOCH),
          'unparseable': skips, 'malformed_ts_in_epoch': [b for b in bad_ts if True],
          'coverage': cov_rows,
          'step_runs': {f'{a}/{b}': n for (a, b), n in sorted(step_steps.items())},
          'untyped_rate': {f'{a}/{b}': f'{untyped_by_step[(a, b)]}/{fr_by_step[(a, b)]}' for (a, b) in sorted(fr_by_step)},
          'untyped_total': sum(untyped_by_step.values()),
          'problem_fact_fill': f'{prob_fill}/{len(steps)}',
          'problem_facts': sum(len(s['_problem']) for s in steps),
          'id_fill': f'{id_fill}/{len(ep)}',
          'signal_line': sig_line, 'signal_odd': odd, 'guard_refused_nonzero': gr_per,
          'outcomes': dict(collections.Counter(s.get('outcome') for s in steps))}
dump(f'{RUN}/q-health.json', health)

# ---- Stage 1 typed ----
UNIV_PREFIX = ('contract.grammar-irregularity', 'contract.in-pass-correction')
def wt(i):
    return 1 + (i.get('iterations') or 0) + (i.get('retries') or 0) + (i.get('reformulations') or 0) + 2 * (i.get('dialogue_rounds') or 0) + 3 * (i.get('halted') or 0) + 3 * (i.get('soft_exit') or 0)
def group(key_fn):
    g = collections.defaultdict(list)
    for f in fr:
        k = key_fn(f)
        if k: g[k].append(f)
    out = []
    for k, L in g.items():
        imp = collections.Counter()
        for f in L:
            for a, b in (f.get('impact') or {}).items():
                if isinstance(b, (int, float)) and not isinstance(b, bool): imp[a] += b
                elif b is True: imp[a] += 1
        halts = imp.get('halted', 0) + imp.get('soft_exit', 0)
        sr = step_steps.get(tuple(k.split('/')[:2]), None) if k.count('/') == 2 else None
        out.append({'key': k, 'n': len(L), 'weight': sum(wt(f.get('impact') or {}) for f in L),
                    'impact': dict(imp), 'halt_or_soft': halts, 'step_runs': sr,
                    'rate': f'{len(L)}/{sr}' if sr else None,
                    'above': len(L) >= 3 or (len(L) >= 2 and halts > 0),
                    'chunks': sorted({str(f.get('chunk')) for f in L}),
                    'cases': [{'id': f.get('id'), 'chunk': f.get('chunk'), 'what': f.get('what'),
                               'impact': f.get('impact'), 'artifacts': f.get('artifacts'),
                               'evidence': f.get('evidence'), 'clause_retracted': clause_notes.get(f.get('id'))} for f in L]})
    return sorted(out, key=lambda x: (-x['n'] * x['weight'], x['key']))
per_step = group(lambda f: f"{f['skill']}/{f.get('step')}/{f.get('type') or 'UNTYPED'}" if f.get('type') else None)
def univ_key(f):
    t = f.get('type') or ''
    if t.startswith('recall.') or t.startswith('contract.grammar-irregularity') or t.startswith('contract.in-pass-correction') or t.startswith('tooling.') or t.startswith('contract.'):
        return 'TYPE:' + t
    return None
by_type = group(lambda f: 'TYPE:' + f['type'] if f.get('type') else None)
dump(f'{RUN}/q-typed.json', {'per_step': per_step, 'by_type_alone': by_type})

# ---- Stage 2 untyped dump ----
unt = [{'id': f.get('id'), 'chunk': f.get('chunk'), 'skill': f['skill'], 'step': f.get('step'), 'what': f.get('what'),
        'impact': f.get('impact'), 'evidence': f.get('evidence')} for f in fr if f.get('untyped') or not f.get('type')]
prior = [{'id': f.get('id'), 'epoch': norm(f.get('epoch')), 'chunk': f.get('chunk'), 'skill': f['skill'], 'step': f.get('step'),
          'what': f.get('what')} for f in flt if f.get('kind') == 'friction' and (f.get('untyped') or not f.get('type'))
         and norm(f.get('epoch')) == 'Epoch 4 - Driver core']
dump(f'{SCR}/untyped.json', {'epoch': unt, 'prior_epoch4': prior})

# ---- Stage 3 chains ----
chains = []
for c in [r for r in ep]:
    anchors = []
    if c.get('kind') == 'friction' and (c.get('type') or '').startswith('input.'):
        anchors = [(a, 'friction:' + c['type'], c.get('what')) for a in (c.get('artifacts') or [])]
        anchors += [(c.get('consumed_artifact'), 'friction', c.get('what'))] if c.get('consumed_artifact') else []
    if c.get('kind') == 'step':
        for co in c.get('consumed') or []:
            if co.get('quality') in ('thin', 'wrong', 'missing'):
                anchors.append((co.get('artifact'), co.get('quality'), co.get('note')))
    for art, q, note in anchors:
        idx = ep.index(c)
        prods = [p for p in ep[:idx] if p.get('kind') == 'step' and p.get('chunk') == c.get('chunk')
                 and any(pr.get('artifact') == art for pr in (p.get('produced') or []))]
        prod = prods[-1] if prods else None
        chains.append({'chunk': c.get('chunk'), 'consumer': f"{c['skill']}/{c.get('step')}", 'consumer_id': c.get('id'),
                       'artifact': art, 'verdict': q, 'note': note,
                       'producer': f"{prod['skill']}/{prod.get('step')}" if prod else None,
                       'producer_id': prod.get('id') if prod else None,
                       'producer_outcome': prod.get('outcome') if prod else None,
                       'producer_signals': [s for pr in (prod.get('produced') or []) if pr.get('artifact') == art for s in (pr.get('signals') or [])] if prod else None,
                       'operator_pass': [co for co in (c.get('consumed') or []) if co.get('artifact') == 'operator-pass'] if c.get('kind') == 'step' else None})
shape = collections.defaultdict(set)
for ch in chains:
    shape[(ch['producer'], ch['artifact'], ch['consumer'])].add(ch['chunk'])
dump(f'{RUN}/q-chains.json', {'links': chains, 'shapes': [{'producer': k[0], 'artifact': k[1], 'consumer': k[2], 'chunks': sorted(map(str, v)), 'k': len(v)} for k, v in sorted(shape.items(), key=lambda kv: -len(kv[1]))]})

# ---- Stage 4 problem facts dump (epoch + lookback) ----
facts = []
for r in flt:
    if r.get('kind') != 'step': continue
    for i, f in enumerate(r['_problem']):
        if not isinstance(f, dict): continue
        facts.append({'epoch': norm(r.get('epoch')), 'id': r.get('id'), 'i': i, 'chunk': r.get('chunk'),
                      'step': f"{r['skill']}/{r.get('step')}", 'outcome': r.get('outcome'),
                      'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
dump(f'{SCR}/facts.json', facts)
deg = [{'epoch': norm(r.get('epoch')), 'id': r.get('id'), 'chunk': r.get('chunk'), 'step': f"{r['skill']}/{r.get('step')}"}
       for r in flt if r.get('kind') == 'step' and r.get('outcome') == 'ok-degraded']
dump(f'{SCR}/degraded.json', deg)
print(json.dumps({'records': len(ep), 'step': len(steps), 'friction': len(fr), 'untyped': len(unt),
                  'facts_epoch': sum(1 for f in facts if f['epoch'] == EPOCH), 'chains': len(chains),
                  'typed_groups': len(per_step), 'above': sum(1 for g in per_step if g['above']),
                  'retracted': len(excl_ids), 'unres': len(unres), 'sig': sig_line}, ensure_ascii=False))
