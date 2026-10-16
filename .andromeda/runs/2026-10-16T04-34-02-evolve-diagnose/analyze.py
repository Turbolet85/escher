"""Evolve-diagnose analysis pass — Epoch 6 (escher-0.1.0). Read-only over the friction ledger.

Writes q-health.json, q-typed.json, q-untyped.json, q-chains.json, q-facts.json into its own dir.
"""
import collections
import json
import os
import re
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(RUN, '..', '..', '..'))
EPOCH = 'Epoch 6 - Polish & ship'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
UNIVERSAL = {
    'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
    'contract.structural-blind-spot', 'contract.token-proxy-check', 'tooling.output-cap-overflow',
    'contract.skill-reference-drift', 'contract.grammar-irregularity',
    'contract.jointly-contradictory-instructions',
}


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


def bare(s):
    return re.sub('^andromeda-', '', s or '')


def plist(p):
    if p is None:
        return []
    return p if isinstance(p, list) else [p]


retr = json.load(open(os.path.join(RUN, 'q-retractions.json'), encoding='utf-8'))
excl_ids = set(retr['retracted_ids'])
excl_probs = {(a, b) for a, b in retr['retracted_problems']}
clause = retr['clause_retracted']

allrecs = []
for line in open(os.path.join(ROOT, '.andromeda/friction-log.ndjson'), encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        continue
    r['skill'] = bare(r.get('skill'))
    r['_epoch'] = norm(r.get('epoch'))
    allrecs.append(r)

# filtered stream (retracted friction ids dropped; retracted problem facts dropped)
def keep(r):
    return not (r.get('kind') == 'friction' and r.get('id') in excl_ids)


filt = [r for r in allrecs if keep(r)]
for r in filt:
    facts = plist(r.get('problem'))
    out = []
    for i, f in enumerate(facts):
        if (r.get('id'), i) in excl_probs or (r.get('id'), None) in excl_probs:
            continue
        out.append(f)
    r['_problem'] = out

E = [r for r in filt if r['_epoch'] == EPOCH]
steps = [r for r in E if r.get('kind') == 'step']
fric = [r for r in E if r.get('kind') == 'friction']

# ---------- Stage 0: health ----------
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}
cov = collections.defaultdict(lambda: collections.defaultdict(list))
for r in steps:
    cov[str(r.get('chunk'))][r['skill']].append(r.get('step'))
coverage = {}
for ch, sk in sorted(cov.items()):
    coverage[ch] = {s: {'n': len(v), 'expected': EXPECTED.get(s), 'steps': v} for s, v in sk.items()}

untyped_by_step = collections.Counter()
fric_by_step = collections.Counter()
for r in fric:
    k = r['skill'] + '/' + str(r.get('step'))
    fric_by_step[k] += 1
    if r.get('untyped') or not r.get('type'):
        untyped_by_step[k] += 1

prob_filled = sum(1 for r in steps if r['_problem'])
id_fill = sum(1 for r in E if r.get('id'))

K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and str(r.get('ts')) >= '2026-10-09']
Eera = len(steps) - len(K)
J = [r for r in K if r.get('step') == 'curation' and r['skill'] == 'wrap-session']
Jids = {id(r) for r in J}
N = M = 0
odd = []
for r in K:
    for p in r.get('produced') or []:
        for s in (p.get('signals') or []) if isinstance(p, dict) else []:
            if not isinstance(s, str):
                continue
            m = re.fullmatch(r'(guard-refused|exact-hit):(\d+)', s)
            if m:
                if m.group(1) == 'guard-refused':
                    N += int(m.group(2))
                elif id(r) in Jids:
                    M += int(m.group(2))
                else:
                    odd.append([r.get('id'), s])
            elif s.startswith('guard-refused') or s.startswith('exact-hit'):
                odd.append([r.get('id'), s])
signal_line = f'guard-refused {N} over {len(K)} step records · exact-hit {M} over {len(J)} curation records'
if Eera:
    signal_line += f' (from 2026-10-09; {Eera} earlier step records are era)'

health = {
    'epoch': EPOCH, 'records': len(E), 'step': len(steps), 'friction': len(fric),
    'unparseable': retr['unparseable'], 'malformed_ts': retr['malformed_ts'],
    'coverage': coverage,
    'untyped_by_step': {k: [untyped_by_step[k], fric_by_step[k]] for k in fric_by_step},
    'problem_fact_fill': [prob_filled, len(steps)],
    'id_fill': [id_fill, len(E)],
    'signal_line': signal_line, 'signal_odd': odd,
    'retracted_in_epoch': [i for i in excl_ids if any(r.get('id') == i and r['_epoch'] == EPOCH for r in allrecs)],
    'clause_retracted_in_epoch': {k: v for k, v in clause.items() if any(r.get('id') == k and r['_epoch'] == EPOCH for r in allrecs)},
    'unresolvable': retr['unresolvable'],
}
json.dump(health, open(os.path.join(RUN, 'q-health.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

# ---------- Stage 1: typed ----------
denom = collections.Counter((r['skill'], r.get('step')) for r in steps)


def weight(r):
    i = r.get('impact') or {}
    return (1 + i.get('iterations', 0) + i.get('retries', 0) + i.get('reformulations', 0)
            + 2 * i.get('dialogue_rounds', 0) + 3 * i.get('halted', 0) + 3 * i.get('soft_exit', 0))


def hs(r):
    i = r.get('impact') or {}
    return bool(i.get('halted') or i.get('soft_exit'))


def row(r):
    w = r.get('what', '')
    if r.get('id') in clause:
        w += ' [clause retracted: ' + '; '.join(clause[r['id']]) + ']'
    return {'id': r.get('id'), 'chunk': r.get('chunk'), 'skill': r['skill'], 'step': r.get('step'),
            'what': w, 'impact': r.get('impact'), 'evidence': r.get('evidence'), 'artifacts': r.get('artifacts'),
            'weight': weight(r)}


groups = collections.defaultdict(list)
for r in fric:
    t = r.get('type') or 'UNTYPED'
    if t == 'UNTYPED' or r.get('untyped'):
        groups[('*', '*', 'UNTYPED:' + r['skill'] + '/' + str(r.get('step')))].append(r)
        continue
    groups[(r['skill'], r.get('step'), t)].append(r)
    if t in UNIVERSAL or t.startswith('recall.'):
        groups[('ALL', 'ALL', t)].append(r)

typed = []
for (sk, st, t), rs in groups.items():
    if t.startswith('UNTYPED:'):
        continue
    n = len(rs)
    w = sum(weight(r) for r in rs)
    thr = n >= 3 or (n >= 2 and any(hs(r) for r in rs))
    typed.append({'key': f'{sk}/{st}/{t}', 'n': n, 'weight_sum': w, 'score': n * w,
                  'rate': (f'{n}/{denom[(sk, st)]}' if sk != 'ALL' else None),
                  'chunks': sorted({str(r.get('chunk')) for r in rs}),
                  'halt_or_soft': sum(1 for r in rs if hs(r)),
                  'above_threshold': thr, 'cases': [row(r) for r in rs]})
typed.sort(key=lambda g: (-g['above_threshold'], -g['n'], -g['weight_sum']))
json.dump(typed, open(os.path.join(RUN, 'q-typed.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

# ---------- Stage 2: untyped (epoch + previous epoch lookback) ----------
prev = 'Epoch 5 - Agent surfaces'
unt = [row(r) | {'epoch': r['_epoch']} for r in filt
       if r.get('kind') == 'friction' and (r.get('untyped') or not r.get('type')) and r['_epoch'] in (EPOCH, prev)]
json.dump(unt, open(os.path.join(RUN, 'q-untyped.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

# ---------- Stage 3: chains ----------
chains = []
by_chunk = collections.defaultdict(list)
for idx, r in enumerate(E):
    by_chunk[r.get('chunk')].append((idx, r))
for idx, r in enumerate(E):
    anchors = []
    if r.get('kind') == 'friction' and str(r.get('type') or '').startswith('input.'):
        for a in r.get('artifacts') or []:
            anchors.append((a, 'friction:' + r['type'], r.get('what')))
    if r.get('kind') == 'step':
        for c in r.get('consumed') or []:
            if isinstance(c, dict) and c.get('quality') in ('thin', 'wrong', 'missing'):
                anchors.append((c.get('artifact'), 'consumed:' + c['quality'], c.get('note'),
                                c.get('signals')))
    for anc in anchors:
        art = anc[0]
        prod = None
        for j, p in by_chunk[r.get('chunk')]:
            if j >= idx or p.get('kind') != 'step':
                continue
            for pr in p.get('produced') or []:
                if isinstance(pr, dict) and pr.get('artifact') == art:
                    prod = (p, pr)
        pfr = []
        if prod:
            pfr = [f.get('type') for f in fric if f.get('chunk') == r.get('chunk')
                   and f['skill'] == prod[0]['skill'] and f.get('step') == prod[0].get('step')]
        chains.append({'chunk': r.get('chunk'), 'consumer': r['skill'] + '/' + str(r.get('step')),
                       'consumer_id': r.get('id'), 'artifact': art, 'verdict': anc[1], 'note': anc[2],
                       'consumer_signals': anc[3] if len(anc) > 3 else None,
                       'producer': (prod[0]['skill'] + '/' + str(prod[0].get('step'))) if prod else None,
                       'producer_id': prod[0].get('id') if prod else None,
                       'producer_outcome': prod[0].get('outcome') if prod else None,
                       'producer_signals': prod[1].get('signals') if prod else None,
                       'producer_note': prod[1].get('note') if prod else None,
                       'producer_frictions': pfr})
shape = collections.Counter((c['producer'], c['artifact'], c['consumer']) for c in chains)
json.dump({'anchors': chains,
           'shapes': [{'shape': list(k), 'chunks': sorted({str(c['chunk']) for c in chains
                                                           if (c['producer'], c['artifact'], c['consumer']) == k}),
                       'n': v} for k, v in shape.most_common()]},
          open(os.path.join(RUN, 'q-chains.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

# ---------- Stage 4: problem facts (epoch + all prior for lookback) ----------
facts = []
for r in filt:
    if r.get('kind') != 'step':
        continue
    for i, f in enumerate(r['_problem']):
        if not isinstance(f, dict):
            f = {'note': str(f)}
        facts.append({'epoch': r['_epoch'], 'chunk': r.get('chunk'), 'id': r.get('id'),
                      'skill': r['skill'], 'step': r.get('step'), 'outcome': r.get('outcome'),
                      'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
json.dump(facts, open(os.path.join(RUN, 'q-facts.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)

print(json.dumps(health, ensure_ascii=False, indent=1))
print('TYPED groups', len(typed), 'above', sum(g['above_threshold'] for g in typed))
for g in typed:
    print(('* ' if g['above_threshold'] else '  ') + g['key'], g['n'], g['weight_sum'], g['rate'], g['halt_or_soft'], len(g['chunks']))
print('UNTYPED', collections.Counter(u['epoch'] for u in unt))
print('CHAIN anchors', len(chains))
for s, v in shape.most_common():
    print('  ', s, v)
ef = [f for f in facts if f['epoch'] == EPOCH]
print('FACTS epoch', len(ef), collections.Counter((f['nature'], f['solution']) for f in ef))
print('outcomes', collections.Counter((r['skill'], r.get('outcome')) for r in steps))
