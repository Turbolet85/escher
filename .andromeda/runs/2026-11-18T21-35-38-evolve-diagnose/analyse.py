import json, re, statistics
from collections import defaultdict, Counter
C = '.andromeda/cache/evolve-diagnose/2026-11-18T21-35-38'
EPOCH, VER = 'Epoch 6 - Polish & ship', '0.3.0'
ISO = re.compile(r'^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$')
SIG = re.compile(r'^([a-z0-9]+(?:-[a-z0-9]+)*):(\d+)$')

def fold_ep(e):
    e = (e or '').replace('–', '-').replace('—', '-')
    return re.sub(r'^\s*#+\s*', '', e).strip()
def fold_ver(v):
    m = re.search(r'(\d+\.\d+\.\d+)$', v or '')
    return m.group(1) if m else ''
def skill(r): return re.sub(r'^andromeda-', '', r.get('skill') or '')
def plist(p): return p if isinstance(p, list) else ([p] if p else [])

recs = []
for line in open(C + '/friction.ndjson', encoding='utf-8'):
    try: recs.append(json.loads(line))
    except ValueError: pass
rt = json.load(open(C + '/q-retractions.json', encoding='utf-8'))
excl_ids = set(rt['retracted_ids'])
excl_probs = {(a, b) for a, b in rt['retracted_problems']}
clause = rt['clause_retracted']

def in_epoch(r, ep=EPOCH, ver=VER):
    return fold_ep(r.get('epoch')) == ep and (r.get('version') is None or fold_ver(r.get('version')) == ver)

E = [r for r in recs if in_epoch(r)]
steps = [r for r in E if r.get('kind') == 'step']
fr_all = [r for r in E if r.get('kind') == 'friction']
fr = [r for r in fr_all if r.get('id') not in excl_ids]
out = {}

# Stage 0 coverage
cov = defaultdict(Counter)
for r in steps:
    cov[r.get('chunk')][(skill(r), r.get('step'))] += 1
out['coverage'] = {str(k): {f'{a}/{b}': n for (a, b), n in v.items()} for k, v in cov.items()}
impl_outcomes = defaultdict(list)
for r in steps:
    if skill(r) == 'implement':
        impl_outcomes[r.get('chunk')].append([r.get('step'), r.get('outcome'), [p.get('signals') for p in r.get('produced') or [] if isinstance(p, dict)]])
out['implement_runs'] = impl_outcomes
out['bad_ts'] = [[r.get('chunk'), r.get('step'), r.get('ts')] for r in E if not ISO.fullmatch(str(r.get('ts')))]
# untyped rate per step
ut = Counter(); tot = Counter()
for r in fr:
    k = f"{skill(r)}/{r.get('step')}"; tot[k] += 1
    if r.get('untyped'): ut[k] += 1
out['untyped_rate'] = {k: [ut[k], tot[k]] for k in tot}
out['problem_fill'] = [sum(1 for r in steps if plist(r.get('problem'))), len(steps)]
out['id_fill'] = [sum(1 for r in E if r.get('id')), len(E)]
out['retracted_in_epoch'] = [r.get('id') for r in fr_all if r.get('id') in excl_ids]
out['clause_in_epoch'] = {r['id']: clause[r['id']] for r in E if r.get('id') in clause}

# signal line + cascade
K = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and r['ts'] >= '2026-10-09']
Eold = [r for r in steps if ISO.fullmatch(str(r.get('ts'))) and r['ts'] < '2026-10-09']
J = [r for r in K if skill(r) == 'wrap-session' and r.get('step') == 'curation']
N = M = 0; odd = []
casc = defaultdict(list)
for r in K:
    for p in r.get('produced') or []:
        if not isinstance(p, dict): continue
        for s in p.get('signals') or []:
            if not isinstance(s, str): continue
            m = SIG.match(s)
            if s.startswith('guard-refused'):
                if m and m.group(1) == 'guard-refused': N += int(m.group(2))
                else: odd.append(s)
            elif s.startswith('exact-hit'):
                if m and m.group(1) == 'exact-hit' and r in J: M += int(m.group(2))
                else: odd.append(s)
            elif m:
                casc[(skill(r), r.get('step'), m.group(1))].append(int(m.group(2)))
out['signal_line'] = {'N': N, 'K': len(K), 'M': M, 'J': len(J), 'E': len(Eold), 'odd': odd}
agg = {}
for (sk, st, nm), vals in casc.items():
    if nm == 'draws': v = max(vals)
    elif nm == 'respawn-kept': v = statistics.median(vals)
    else: v = sum(vals)
    agg[f'{sk}/{st}/{nm}'] = v
out['signals_agg'] = agg
# cascade rows
rows = []
for r in steps:
    if (skill(r), r.get('step')) not in (('phase', 'distill'), ('wrap-session', 'reconcile')): continue
    sg = {}
    for p in r.get('produced') or []:
        if isinstance(p, dict):
            for s in p.get('signals') or []:
                m = SIG.match(s) if isinstance(s, str) else None
                if m: sg[m.group(1)] = sg.get(m.group(1), 0) + int(m.group(2))
    rows.append({'chunk': r.get('chunk'), 'step': r.get('step'), 'signals': sg})
out['cascade'] = rows

# typed friction records for every group (full evidence)
fr_rows = []
for r in fr:
    w = r.get('what') or ''
    if r.get('id') in clause: w += ' [clause retracted: ' + '; '.join(clause[r['id']]) + ']'
    fr_rows.append({'id': r.get('id'), 'chunk': r.get('chunk'), 'key': f"{skill(r)}/{r.get('step')}/{r.get('type') or ('UNTYPED' if not r.get('untyped') else 'untyped')}",
                    'untyped': bool(r.get('untyped')), 'what': w, 'impact': r.get('impact'), 'evidence': r.get('evidence'), 'artifacts': r.get('artifacts')})
out['friction'] = fr_rows

# problem facts (filtered)
facts = []
for r in steps:
    for i, f in enumerate(plist(r.get('problem'))):
        if (r.get('id'), None) in excl_probs or (r.get('id'), i) in excl_probs: continue
        if not isinstance(f, dict): f = {'note': str(f)}
        facts.append({'id': r.get('id'), 'i': i, 'chunk': r.get('chunk'), 'step': f"{skill(r)}/{r.get('step')}", 'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')})
out['facts'] = facts

# chains: anchors
anchors = []
for r in steps:
    for c in r.get('consumed') or []:
        if isinstance(c, dict) and c.get('quality') in ('thin', 'wrong', 'missing'):
            anchors.append({'chunk': r.get('chunk'), 'consumer': f"{skill(r)}/{r.get('step')}", 'artifact': c.get('artifact'), 'quality': c.get('quality'), 'note': c.get('note'), 'id': r.get('id')})
for r in fr:
    if (r.get('type') or '').startswith('input.'):
        anchors.append({'chunk': r.get('chunk'), 'consumer': f"{skill(r)}/{r.get('step')}", 'artifact': (r.get('artifacts') or [None])[0], 'quality': 'friction:' + r['type'], 'note': r.get('what'), 'id': r.get('id')})
# producers
order = {id(r): i for i, r in enumerate(recs)}
for a in anchors:
    prods = []
    for r in steps:
        if r.get('chunk') != a['chunk']: continue
        for p in r.get('produced') or []:
            if isinstance(p, dict) and p.get('artifact') == a['artifact']:
                prods.append({'producer': f"{skill(r)}/{r.get('step')}", 'outcome': r.get('outcome'), 'signals': p.get('signals'), 'note': p.get('note'), 'id': r.get('id')})
    a['producers'] = prods
out['anchors'] = anchors
# operator-pass entries on report records
op = []
for r in steps:
    for c in r.get('consumed') or []:
        if isinstance(c, dict) and c.get('artifact') == 'operator-pass':
            op.append({'chunk': r.get('chunk'), 'step': f"{skill(r)}/{r.get('step')}", 'quality': c.get('quality'), 'signals': c.get('signals'), 'note': c.get('note')})
out['operator_pass'] = op

# deferrals
defs = []
for r in steps:
    for p in r.get('produced') or []:
        if isinstance(p, dict) and any(isinstance(s, str) and s.startswith('deferral-open') for s in p.get('signals') or []):
            defs.append({'chunk': r.get('chunk'), 'step': f"{skill(r)}/{r.get('step')}", 'artifact': p.get('artifact'), 'signals': p.get('signals'), 'note': p.get('note')})
out['deferral_signals'] = defs
out['outcomes'] = Counter(f"{skill(r)}/{r.get('step')}:{r.get('outcome')}" for r in steps)

# prior epoch lookback: untyped whats and workaround facts of 0.3.0 Epoch 5
P = [r for r in recs if in_epoch(r, 'Epoch 5 - Errors and explanation', '0.3.0')]
out['prev_untyped'] = [[r.get('chunk'), f"{skill(r)}/{r.get('step')}", r.get('what')] for r in P if r.get('kind') == 'friction' and r.get('untyped') and r.get('id') not in excl_ids]
pf = []
for r in P:
    if r.get('kind') != 'step': continue
    for i, f in enumerate(plist(r.get('problem'))):
        if (r.get('id'), None) in excl_probs or (r.get('id'), i) in excl_probs: continue
        if isinstance(f, dict) and f.get('solution') in ('workaround', 'prohibition', 'removed-cause', 'overridden'):
            pf.append([r.get('chunk'), f"{skill(r)}/{r.get('step')}", f.get('nature'), f.get('solution'), f.get('note')])
out['prev_facts'] = pf
# prior-epoch typed group counts for chronic-degrade
ptc = Counter(f"{skill(r)}/{r.get('step')}/{r.get('type')}" for r in P if r.get('kind') == 'friction' and r.get('type') and r.get('id') not in excl_ids)
out['prev_typed'] = dict(ptc.most_common())
json.dump(out, open(C + '/analysis.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1, default=list)
print(len(E), len(steps), len(fr_all), len(fr))
