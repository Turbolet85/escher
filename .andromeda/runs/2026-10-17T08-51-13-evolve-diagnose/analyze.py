"""Evolve-diagnose analysis pass — escher-0.2.0 · Epoch 1 — Foundation.

Run by path from the project root. Writes the q-*.json twins beside itself.
"""
import collections
import json
import os
import re

RUN = os.path.dirname(os.path.abspath(__file__))
LEDGER = '.andromeda/friction-log.ndjson'
TARGET_VERSION = 'escher-0.2.0'
TARGET_EPOCH = 'Epoch 1 - Foundation'
SIGNAL_BOUNDARY = '2026-10-09'
# evolve-system.md §Universal types (nine) — plus every recall.* type, grouped by type alone (diagnosis-pass Stage 1)
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
             'contract.skill-reference-drift', 'contract.grammar-irregularity',
             'contract.jointly-contradictory-instructions'}
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')


def norm_epoch(s):
    s = re.sub(r'^\s*#+\s*', '', s or '').strip()
    return re.sub(r'\s+', ' ', s.replace('–', '-').replace('—', '-'))


def norm_version(v):
    v = v or ''
    return v if v.startswith('escher-') or not v else 'escher-' + v


def bare_skill(s):
    return re.sub('^andromeda-', '', s or '')


def plist(p):
    if p is None:
        return []
    return p if isinstance(p, list) else [p]


def dump(name, obj):
    with open(os.path.join(RUN, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


# ---- pre-pass ----
recs, skips, bad_ts = [], 0, []
for line in open(LEDGER, encoding='utf-8'):
    if not line.strip():
        continue
    try:
        r = json.loads(line)
    except ValueError:
        skips += 1
        continue
    if not ISO.fullmatch(str(r.get('ts'))):
        bad_ts.append([r.get('chunk'), r.get('skill'), r.get('step'), r.get('kind'), str(r.get('ts'))[:40]])
    r['skill'] = bare_skill(r.get('skill'))
    r['_ep'] = norm_epoch(r.get('epoch'))
    r['_ver'] = norm_version(r.get('version'))
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
dump('q-retractions.json', {
    'records': len(recs), 'unparseable': skips, 'malformed_ts': bad_ts,
    'retracted_ids': sorted(excl_ids), 'retracted_problems': sorted(map(list, excl_probs), key=str),
    'clause_retracted': clause_notes, 'unresolvable': unres, 'retraction_of_retraction': retr_of_retr})

# filtered stream
filt = [r for r in recs if not (r.get('kind') == 'friction' and r.get('id') in excl_ids)]


def facts(r):
    out = []
    for i, p in enumerate(plist(r.get('problem'))):
        if (r.get('id'), i) in excl_probs or (r.get('id'), None) in excl_probs:
            continue
        if isinstance(p, dict):
            out.append((i, p))
    return out


def in_target(r):
    return r['_ver'] == TARGET_VERSION and r['_ep'] == TARGET_EPOCH


ep = [r for r in filt if in_target(r)]
steps = [r for r in ep if r.get('kind') == 'step']
fric = [r for r in ep if r.get('kind') == 'friction']

# ---- Stage 0 ----
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}
per_chunk = collections.defaultdict(lambda: collections.Counter())
per_chunk_steps = collections.defaultdict(list)
for r in steps:
    per_chunk[r.get('chunk')][r['skill']] += 1
    per_chunk_steps[r.get('chunk')].append(r['skill'] + '/' + (r.get('step') or ''))
coverage = {}
for ch, cnt in per_chunk.items():
    coverage[str(ch)] = {'counts': dict(cnt), 'steps': per_chunk_steps[ch],
                         'gaps': {k: v - cnt.get(k, 0) for k, v in EXPECTED.items() if ch and cnt.get(k, 0) < v}}
untyped_by_step = collections.defaultdict(lambda: [0, 0])
for r in fric:
    k = r['skill'] + '/' + (r.get('step') or '')
    untyped_by_step[k][1] += 1
    if r.get('untyped') or not r.get('type'):
        untyped_by_step[k][0] += 1
fill = sum(1 for r in steps if plist(r.get('problem')))
idfill = sum(1 for r in ep if r.get('id'))
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= SIGNAL_BOUNDARY]
E = len(steps) - len(K)
J = [r for r in K if r['skill'] == 'wrap-session' and r.get('step') == 'curation']
N = M = 0
odd = []
for r in K:
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            s = str(s)
            m = re.fullmatch(r'(guard-refused|exact-hit):(\d+)', s)
            if m:
                if m.group(1) == 'guard-refused':
                    N += int(m.group(2))
                elif r in J:
                    M += int(m.group(2))
                else:
                    odd.append(s)
            elif s.startswith('guard-refused') or s.startswith('exact-hit'):
                odd.append(s)
sig_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if E:
    sig_line += f' (from 2026-10-09; {E} earlier step records are era)'
outcomes = collections.Counter((r['skill'] + '/' + (r.get('step') or ''), r.get('outcome')) for r in steps)
dump('q-health.json', {
    'records': len(ep), 'step': len(steps), 'friction': len(fric), 'unparseable_whole_ledger': skips,
    'malformed_ts_in_epoch': [b for b in bad_ts],
    'coverage': coverage, 'untyped_by_step': {k: {'untyped': v[0], 'of': v[1]} for k, v in untyped_by_step.items()},
    'problem_fact_fill': [fill, len(steps)], 'id_fill': [idfill, len(ep)],
    'signal_line': sig_line, 'signal_odd': odd,
    'outcomes': {f'{a}={b}': n for (a, b), n in outcomes.items()},
    'retracted_in_ledger': len(excl_ids), 'unresolvable': len(unres), 'clause_retracted': len(clause_notes)})

# ---- Stage 1 ----
step_runs = collections.Counter(r['skill'] + '/' + (r.get('step') or '') for r in steps)


def imp(r, k):
    return int((r.get('impact') or {}).get(k) or 0)


def weight(r):
    return (1 + imp(r, 'iterations') + imp(r, 'retries') + imp(r, 'reformulations')
            + 2 * imp(r, 'dialogue_rounds') + 3 * imp(r, 'halted') + 3 * imp(r, 'soft_exit'))


def group(rows, keyf):
    g = collections.defaultdict(list)
    for r in rows:
        g[keyf(r)].append(r)
    out = []
    for k, rs in g.items():
        sk = rs[0]['skill'] + '/' + (rs[0].get('step') or '')
        halt = any(imp(r, 'halted') or imp(r, 'soft_exit') for r in rs)
        n = len(rs)
        out.append({
            'key': k, 'n': n, 'weight': sum(weight(r) for r in rs),
            'rate': f'{n}/{step_runs.get(sk, 0)}',
            'chunks': sorted({str(r.get('chunk')) for r in rs}),
            'halt_or_soft_exit': halt,
            'above_threshold': n >= 3 or (n >= 2 and halt),
            'cases': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
                       'what': r.get('what'), 'impact': r.get('impact'), 'evidence': r.get('evidence'),
                       'artifacts': r.get('artifacts'),
                       'clause_retracted': clause_notes.get(r.get('id'))} for r in rs]})
    return sorted(out, key=lambda x: (-x['n'] * x['weight'], x['key']))


typed = [r for r in fric if r.get('type') and not r.get('untyped')]
per_step = group(typed, lambda r: f"{r['skill']}/{r.get('step')}/{r['type']}")
universal = group([r for r in typed if r['type'] in UNIVERSAL or r['type'].startswith('recall.')],
                  lambda r: f"*/*/{r['type']}")
by_type = group(typed, lambda r: f"(any)/{r['type']}")
dump('q-typed.json', {'per_step': per_step, 'universal_by_type': universal,
                      'by_type_alone_observation': by_type, 'step_runs': dict(step_runs)})

# ---- Stage 2 ----
untyped = [r for r in fric if r.get('untyped') or not r.get('type')]
prev_untyped = [r for r in filt if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type'))
                and r['_ver'] == 'escher-0.1.0' and r['_ep'] == 'Epoch 6 - Polish & ship']
dump('q-untyped.json', {
    'target': [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
                'what': r.get('what'), 'impact': r.get('impact'), 'proposed': r.get('proposed_type')} for r in untyped],
    'previous_epoch_lookback': [{'id': r.get('id'), 'chunk': r.get('chunk'),
                                 'step': r['skill'] + '/' + (r.get('step') or ''), 'what': r.get('what')}
                                for r in prev_untyped]})

# ---- Stage 3 ----
anchors = []
for r in fric:
    if (r.get('type') or '').startswith('input.'):
        anchors.append(('friction', r, r.get('artifacts') or []))
for r in steps:
    for c in r.get('consumed') or []:
        if c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append(('verdict', r, [c.get('artifact')], c))
chains = []
order = {id(r): i for i, r in enumerate(filt)}
for a in anchors:
    kind, rec, arts = a[0], a[1], a[2]
    for art in arts:
        prods = [p for p in steps if p.get('chunk') == rec.get('chunk') and order[id(p)] < order[id(rec)]
                 and any(x.get('artifact') == art for x in p.get('produced') or [])]
        prod = prods[-1] if prods else None
        chains.append({
            'anchor_kind': kind, 'chunk': rec.get('chunk'), 'consumer': rec['skill'] + '/' + (rec.get('step') or ''),
            'consumer_id': rec.get('id'), 'artifact': art,
            'verdict': a[3] if len(a) > 3 else {'type': rec.get('type'), 'what': rec.get('what')},
            'operator_pass': [c for c in rec.get('consumed') or [] if c.get('artifact') == 'operator-pass'],
            'producer': (prod['skill'] + '/' + (prod.get('step') or '')) if prod else None,
            'producer_id': prod.get('id') if prod else None,
            'producer_outcome': prod.get('outcome') if prod else None,
            'producer_signals': [x for x in (prod.get('produced') or []) if x.get('artifact') == art] if prod else None,
            'producer_frictions': [{'type': f.get('type'), 'what': f.get('what')} for f in fric
                                   if prod and f.get('chunk') == prod.get('chunk') and f['skill'] == prod['skill']
                                   and f.get('step') == prod.get('step')]})
shape = collections.defaultdict(set)
for c in chains:
    shape[(c['producer'], c['artifact'], c['consumer'])].add(c['chunk'])
dump('q-chains.json', {'chains': chains,
                       'shapes': [{'producer': k[0], 'artifact': k[1], 'consumer': k[2], 'chunks': sorted(map(str, v)),
                                   'candidate': len(v) >= 2} for k, v in shape.items()]})

# ---- Stage 4 (raw fact pile; themes clustered by hand into q-level.json) ----
pile = []
for r in steps:
    for i, p in facts(r):
        pile.append({'id': r.get('id'), 'idx': i, 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
                     'nature': p.get('nature'), 'solution': p.get('solution'), 'note': p.get('note')})
prev = []
for r in filt:
    if r.get('kind') == 'step' and r['_ver'] == 'escher-0.1.0':
        for i, p in facts(r):
            if p.get('solution') in ('workaround', 'prohibition', 'removed-cause', 'deferred', 'overridden'):
                prev.append({'id': r.get('id'), 'idx': i, 'epoch': r['_ep'], 'chunk': r.get('chunk'),
                             'step': r['skill'] + '/' + (r.get('step') or ''), 'nature': p.get('nature'),
                             'solution': p.get('solution'), 'note': p.get('note')})
deferral_signals = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
                     'artifact': p.get('artifact'), 'signals': p.get('signals'), 'note': p.get('note')}
                    for r in steps for p in r.get('produced') or [] if 'deferral-open' in (p.get('signals') or [])]
tooling = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
            'type': r.get('type'), 'what': r.get('what')} for r in fric if (r.get('type') or '').startswith('tooling.')]
degraded = [{'id': r.get('id'), 'chunk': r.get('chunk'), 'step': r['skill'] + '/' + (r.get('step') or ''),
             'outcome': r.get('outcome')} for r in steps if r.get('outcome') not in ('ok', None)]
dump('q-facts-raw.json', {'target_pile': pile, 'prior_lookback_0_1_0': prev, 'deferral_open_signals': deferral_signals,
                          'tooling_frictions': tooling, 'non_ok_outcomes': degraded})
print(json.dumps({'records': len(ep), 'steps': len(steps), 'friction': len(fric), 'typed_groups': len(per_step),
                  'above': [g['key'] for g in per_step + universal if g['above_threshold']],
                  'untyped': len(untyped), 'anchors': len(anchors), 'facts': len(pile), 'prior_facts': len(prev),
                  'signal': sig_line, 'retracted': len(excl_ids), 'unresolvable': len(unres)}, ensure_ascii=False))
