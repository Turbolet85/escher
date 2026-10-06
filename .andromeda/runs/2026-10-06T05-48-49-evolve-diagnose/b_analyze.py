import json, collections, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from a_load import load

RUN = os.path.dirname(__file__)
EPOCH = 'Epoch 1 - Foundation'
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
             'contract.skill-reference-drift', 'contract.grammar-irregularity',
             'contract.jointly-contradictory-instructions'}
EXPECTED = {'phase': ['take-up', 'distill', 'research', 'plan', 'validate'],
            'implement': ['code', 'fix-loop', 'smoke'],
            'wrap-session': ['report', 'reconcile', 'curation', 'route-resolve', 'gates']}


def dump(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


def weight(imp):
    imp = imp or {}
    g = lambda k: imp.get(k, 0) or 0
    return 1 + g('iterations') + g('retries') + g('reformulations') + 2 * g('dialogue_rounds') + 3 * g('halted') + 3 * g('soft_exit')


recs, skips, bad_ts = load()

# --- retraction pre-pass (whole ledger) ---
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
                            'clause_retracted': clause_notes, 'unresolvable': unres,
                            'retraction_of_retraction': retr_of_retr})

E = [r for r in recs if r['_epoch'] == EPOCH and r['id'] not in excl_ids]
for r in E:
    r['problem'] = [p for i, p in enumerate(r['problem'])
                    if (r['id'], i) not in excl_probs and (r['id'], None) not in excl_probs]
steps = [r for r in E if r['kind'] == 'step']
fr = [r for r in E if r['kind'] == 'friction']

# --- Stage 0: health ---
cov = {}
for c in sorted({r['chunk'] for r in steps if r['chunk']}):
    have = collections.Counter((r['skill'], r['step']) for r in steps if r['chunk'] == c)
    missing = [f'{s}/{st}' for s, sts in EXPECTED.items() for st in sts if not have[(s, st)]]
    extra = [f'{s}/{st}' for (s, st), n in have.items() if st not in EXPECTED.get(s, []) or n > 1]
    cov[c] = {'present': sum(have.values()), 'expected': 13, 'missing': missing, 'extra': extra}
null_steps = collections.Counter(f"{r['skill']}/{r['step']}" for r in steps if not r['chunk'])
step_runs = collections.Counter((r['skill'], r['step']) for r in steps)
untyped_by_step = collections.Counter((r['skill'], r['step']) for r in fr if r.get('untyped') or not r.get('type'))
fr_by_step = collections.Counter((r['skill'], r['step']) for r in fr)
untyped_rate = {f'{s}/{st}': {'untyped': untyped_by_step[(s, st)], 'friction': fr_by_step[(s, st)], 'step_runs': step_runs[(s, st)]}
                for (s, st) in sorted(fr_by_step) if untyped_by_step[(s, st)]}
health = {'epoch': EPOCH, 'records': len(E), 'step': len(steps), 'friction': len(fr),
          'unparseable': skips, 'malformed_ts': bad_ts, 'coverage': cov, 'chunk_null_steps': dict(null_steps),
          'untyped_total': sum(untyped_by_step.values()), 'untyped_by_step': untyped_rate,
          'problem_fill': {'steps_with_facts': sum(1 for r in steps if r['problem']), 'steps': len(steps),
                           'facts': sum(len(r['problem']) for r in steps)},
          'id_fill': {'with_id': sum(1 for r in E if r.get('id')), 'records': len(E)},
          'ts_range': [min(r['ts'] for r in E), max(r['ts'] for r in E)],
          'retractions': {'retracted': len(excl_ids), 'clause_retracted': {k: v for k, v in clause_notes.items()},
                          'unresolvable': unres}}
dump('q-health.json', health)

# --- Stage 1: typed ---
def group(rows, key):
    g = collections.defaultdict(list)
    for r in rows:
        g[key(r)].append(r)
    out = []
    for k, rs in g.items():
        imp = collections.Counter()
        for r in rs:
            for kk, vv in (r.get('impact') or {}).items():
                imp[kk] += vv or 0
        halt_like = sum(1 for r in rs if (r.get('impact') or {}).get('halted') or (r.get('impact') or {}).get('soft_exit'))
        n = len(rs)
        out.append({'key': k, 'n': n, 'weight': sum(weight(r.get('impact')) for r in rs),
                    'impact': dict(imp), 'halt_or_softexit_cases': halt_like,
                    'chunks': sorted({str(r['chunk']) for r in rs}),
                    'above_threshold': n >= 3 or (n >= 2 and halt_like > 0),
                    'cases': [{'line': r['_line'], 'id': r['id'], 'chunk': r['chunk'], 'step': f"{r['skill']}/{r['step']}",
                               'what': r['what'] + (''.join(f' [clause retracted: {n_}]' for n_ in clause_notes.get(r['id'], []))),
                               'impact': r.get('impact'), 'evidence': r.get('evidence')} for r in rs]})
    return sorted(out, key=lambda x: (-x['n'] * (x['weight'] / max(x['n'], 1)), x['key']))

typed = [r for r in fr if r.get('type') and not r.get('untyped')]
per_step = group(typed, lambda r: f"{r['skill']}/{r['step']}/{r['type']}")
cross = group([r for r in typed if r['type'] in UNIVERSAL or r['type'].startswith('recall.')], lambda r: f"*/{r['type']}")
denoms = {f'{s}/{st}': n for (s, st), n in step_runs.items()}
for g in per_step:
    s, st, _ = g['key'].split('/', 2)
    g['rate'] = f"{g['n']}/{denoms.get(f'{s}/{st}', 0)}"
dump('q-typed.json', {'per_step_groups': per_step, 'cross_step_type_groups': cross, 'step_run_denominators': denoms})

# --- Stage 3: chains ---
def nearest_producer(r, art):
    for p in reversed([x for x in steps if x['chunk'] == r['chunk'] and x['_line'] < r['_line']]):
        for pr in p.get('produced') or []:
            if pr.get('artifact') == art:
                return p, pr
    return None, None

anchors = []
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            p, pr = nearest_producer(r, c['artifact'])
            down = [f['_line'] for f in fr if f['chunk'] == r['chunk'] and f['skill'] == r['skill'] and f['step'] == r['step']]
            anchors.append({'consumer': f"{r['skill']}/{r['step']}", 'consumer_line': r['_line'], 'chunk': r['chunk'],
                            'artifact': c['artifact'], 'quality': c['quality'], 'consumer_frictions': down,
                            'producer': f"{p['skill']}/{p['step']}" if p else None, 'producer_line': p['_line'] if p else None,
                            'producer_outcome': p.get('outcome') if p else None,
                            'producer_signals': pr.get('signals') if pr else None})
for f in fr:
    if (f.get('type') or '').startswith('input.'):
        art = {'input.plan-step-ambiguous': 'plan', 'input.report-insufficient': 'report',
               'input.working-entry-thin': 'working-entry'}.get(f['type'])
        p, pr = nearest_producer(f, art)
        cons = next((s for s in steps if s['chunk'] == f['chunk'] and s['skill'] == f['skill'] and s['step'] == f['step']), None)
        verdict = next((c.get('quality') for c in (cons or {}).get('consumed') or [] if c.get('artifact') == art), None)
        anchors.append({'consumer': f"{f['skill']}/{f['step']}", 'consumer_line': f['_line'], 'chunk': f['chunk'],
                        'artifact': art, 'quality': f'friction:{f["type"]} (consumer step verdict: {verdict})',
                        'consumer_frictions': [f['_line']],
                        'producer': f"{p['skill']}/{p['step']}" if p else None, 'producer_line': p['_line'] if p else None,
                        'producer_outcome': p.get('outcome') if p else None,
                        'producer_signals': pr.get('signals') if pr else None})
shapes = collections.defaultdict(list)
for a in anchors:
    shapes[(a['producer'], a['artifact'], a['consumer'])].append(a)
shape_out = [{'shape': f'{k[0]} -{k[1]}-> {k[2]}', 'chunks': sorted({str(a["chunk"]) for a in v}), 'k': len({a['chunk'] for a in v}),
              'anchors': v} for k, v in shapes.items()]
shape_out.sort(key=lambda x: -x['k'])
dump('q-chains.json', {'anchors': anchors, 'shapes': shape_out})

# --- Stage 4: level ---
facts = [{'line': r['_line'], 'id': r['id'], 'chunk': r['chunk'], 'step': f"{r['skill']}/{r['step']}", 'index': i, **p}
         for r in steps for i, p in enumerate(r['problem'])]
# hand-read theme assignment (by line:index) — Pass A
THEMES = {
    'T1 project Bash guard (leading cd / heredoc-to-file) refusals': ['1:0', '5:0', '16:0', '24:0', '49:0', '64:0', '78:0', '81:0', '155:0', '117:1', '168:0'],
    'T2 shell/python writes where the letter names Write/Edit for document payloads': ['5:1', '48:0', '77:0', '82:1', '103:0', '122:0'],
    'T3 committed evidence carrying host paths refused by hygiene P1, masked by hand': ['128:0', '144:0', '165:0'],
    'T4 masters file:line citations re-pointed by an ad-hoc scratch script (no pipeline tool)': ['38:0', '95:0'],
    'T5 plan steps reordered so a red-before-green reading is possible': ['33:0', '56:0'],
    'T6 gate.py runner behaviour routed around (stdout buffering off a tty; exit 124 = own timeout)': ['12:1', '89:0'],
    'T7 tool output filtered / bundled against the run-bare or per-file letter': ['18:0', '108:0', '74:0'],
    'T8 plan omission filled at implement (dep / field the plan step did not list)': ['86:1', '115:0'],
    'singletons': ['9:0', '12:0', '50:0', '83:0', '86:0', '117:0'],
}
idx = {f"{f['line']}:{f['index']}": f for f in facts}
theme_out = {}
assigned = set()
for t, keys in THEMES.items():
    rows = [idx[k] for k in keys]
    assigned.update(keys)
    theme_out[t] = {'n': len(rows), 'natures': dict(collections.Counter(r['nature'] for r in rows)),
                    'solutions': dict(collections.Counter(r['solution'] for r in rows)),
                    'chunks': sorted({str(r['chunk']) for r in rows}), 'facts': rows}
pass_a_pool = [k for k, f in idx.items() if f['solution'] in ('workaround', 'prohibition', 'removed-cause')]
unassigned = sorted(set(pass_a_pool) - assigned)
signatures = {
    'override': [f for f in facts if f['solution'] == 'overridden'],
    'deferred': [f for f in facts if f['solution'] == 'deferred'] + [
        {'line': r['_line'], 'id': r['id'], 'chunk': r['chunk'], 'type': r['type'], 'what': r['what']} for r in fr if r.get('type') == 'tooling.gate-deferral'],
    'ok_degraded_outcomes': [{'line': r['_line'], 'chunk': r['chunk'], 'step': f"{r['skill']}/{r['step']}"} for r in steps if r['outcome'] == 'ok-degraded'],
    'halted_resolved_outcomes': [{'line': r['_line'], 'chunk': r['chunk'], 'step': f"{r['skill']}/{r['step']}"} for r in steps if r['outcome'] == 'halted-resolved'],
    'tooling_recurrence': {t: [r['_line'] for r in fr if r.get('type') == t] for t in sorted({r['type'] for r in typed if r['type'].startswith('tooling.')})},
}
dump('q-level.json', {'pass_a_pool_size': len(pass_a_pool), 'unassigned_pass_a': unassigned,
                      'themes': theme_out, 'signatures': signatures, 'all_facts': facts})

print(json.dumps({'health': {k: health[k] for k in ('records', 'step', 'friction', 'untyped_total', 'problem_fill', 'id_fill', 'ts_range')},
                  'coverage_gaps': {c: v for c, v in cov.items() if v['missing'] or v['extra']},
                  'null_steps': dict(null_steps), 'untyped_by_step': untyped_rate}, ensure_ascii=False))
print('--- per-step groups (n, weight, key, rate, above)')
for g in per_step:
    print(g['n'], g['weight'], g['key'], g['rate'], g['above_threshold'], [c['line'] for c in g['cases']])
print('--- cross-step type groups')
for g in cross:
    print(g['n'], g['weight'], g['key'], g['above_threshold'], [c['line'] for c in g['cases']])
print('--- chain shapes')
for s in shape_out:
    print(s['k'], s['shape'], s['chunks'], [(a['consumer_line'], a['producer_line'], a['producer_signals'], a['consumer_frictions']) for a in s['anchors']])
print('--- themes'); [print(v['n'], t, v['natures'], v['solutions']) for t, v in theme_out.items()]
print('unassigned', unassigned)
print('sig override', [(f['line'], f['note'][:60]) for f in signatures['override']])
print('sig deferred', [(f['line'], (f.get('note') or f.get('what'))[:70]) for f in signatures['deferred']])
print('ok-degraded', signatures['ok_degraded_outcomes'], 'halted-resolved', signatures['halted_resolved_outcomes'])
print('tooling', signatures['tooling_recurrence'])
