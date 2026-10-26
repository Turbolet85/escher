"""P4: diff this run's record against its baseline (re-found by baseline_sha, never the last line) and classify
every movement by audit-pass.md §Thresholds. Writes {run_dir}/judge.json.

usage: judge.py RUN_DIR   (run from the repo root, after the P3 append)
"""
import json, re, sys

rd = sys.argv[1]
cur = json.load(open(f'{rd}/record.json', encoding='utf-8'))
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
# this run's own record is the last line after the append; the baseline is the last record BEFORE it with sha == baseline_sha
assert recs[-1]['ts'] == cur['ts'] and recs[-1]['sha'] == cur['sha'], 'the append is not the last line'
prior = recs[:-1]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]
bb = [r for r in prior if r['sha'] == base['baseline_sha']][-1]
span = cur['span']
P, I, B = [], [], []

tv_break = [k for k in cur['tool_versions'] if base['tool_versions'].get(k) != cur['tool_versions'][k]]


def get(r, path):
    for k in path.split('.'):
        r = None if r is None else r.get(k)
    return r


# fixed checks
c0, c1 = base['graph']['cycles'], cur['graph']['cycles']
(P if c1 > c0 else B).append({'check': 'new-cycle', 'metric': 'graph.cycles', 'was': c0, 'now': c1})
d0, d1 = base['duplication']['pct'], cur['duplication']['pct']
fire = d1 >= d0 + 0.5 and d1 >= d0 * 1.15
(P if fire else B).append({'check': 'duplication-up', 'metric': 'duplication.pct', 'was': d0, 'now': d1,
                           'delta': round(d1 - d0, 2)})
o0, o1 = base['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
fire = o1 >= o0 + 3 and o1 >= o0 * 1.25
(P if fire else B).append({'check': 'complexity-creep', 'metric': 'complexity.over_ceiling', 'was': o0, 'now': o1,
                           'delta': o1 - o0})
z0, z1 = base['dead']['zero_ref_candidates'], cur['dead']['zero_ref_candidates']
(P if z1 >= z0 + 5 else B).append({'check': 'dead-growth', 'metric': 'dead.zero_ref_candidates', 'was': z0, 'now': z1,
                                   'delta': z1 - z0})
l0, l1 = base['coverage']['line'], cur['coverage']['line']
(P if l1 <= l0 - 2 else B).append({'check': 'coverage-drop', 'metric': 'coverage.line', 'was': l0, 'now': l1,
                                   'delta': round(l1 - l0, 2)})
for path in ('sizes.file_max', 'sizes.over_800', 'sizes.file_p50', 'sizes.file_p90', 'complexity.cyclomatic_p90',
             'complexity.cognitive_p90', 'graph.cross_unit_edges', 'churn.pct', 'churn.files_churned'):
    B.append({'check': 'movement', 'metric': path, 'was': get(base, path), 'now': get(cur, path)})

# monotonic: the closed six, worsened at BOTH of the last two diffs (bb -> base, base -> cur)
SIX = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1), ('sizes.file_max', 1),
       ('sizes.over_800', 1), ('coverage.line', -1)]
mono = []
for path, sign in SIX:
    a, b, c = get(bb, path), get(base, path), get(cur, path)
    if None in (a, b, c):
        B.append({'check': 'monotonic', 'metric': path, 'values': [a, b, c], 'note': 'not evaluable (null)'})
        continue
    w1, w2 = (b - a) * sign > 0, (c - b) * sign > 0
    row = {'check': 'monotonic', 'metric': path, 'values': [a, b, c], 'epochs': [bb['epoch'], base['epoch'], cur['epoch']]}
    (P if w1 and w2 else B).append(row)

# count-under-ratio
cl0, cl1 = base['duplication']['clones'], cur['duplication']['clones']
dl0, dl1 = base['duplication']['duplicated_lines'], cur['duplication']['duplicated_lines']
tl0, tl1 = base['duplication']['total_lines'], cur['duplication']['total_lines']
if (cl1 > cl0 or dl1 > dl0) and d1 <= d0:
    I.append({'check': 'count-under-ratio', 'clones': [cl0, cl1], 'duplicated_lines': [dl0, dl1], 'pct': [d0, d1],
              'population_rel': round((tl1 - tl0) / tl0 * 100, 2), 'split': [base['duplication']['split'],
                                                                             cur['duplication']['split']]})
else:
    B.append({'check': 'count-under-ratio', 'note': 'ratio rose with the counts (not this rule)', 'clones': [cl0, cl1],
              'duplicated_lines': [dl0, dl1], 'pct': [d0, d1], 'population_rel': round((tl1 - tl0) / tl0 * 100, 2),
              'split': [base['duplication']['split'], cur['duplication']['split']]})

# top-N entrants
def ent(name, key, curl, basel):
    bk = {key(x) for x in basel}
    new = [x for x in curl if key(x) not in bk]
    if new:
        I.append({'check': 'top-N entrants', 'list': name, 'entrants': new})


ent('complexity.top', lambda x: (x[0], x[1].rsplit(':', 1)[0]), cur['complexity']['top'], base['complexity']['top'])
ent('sizes.top', lambda x: x[0], cur['sizes']['top'], base['sizes']['top'])
ent('duplication.top', lambda x: tuple(x), cur['duplication']['top'], base['duplication']['top'])
ent('graph.fan_in_top', lambda x: x[0], cur['graph']['fan_in_top'], base['graph']['fan_in_top'])
ent('hotspots', lambda x: x[0], cur['hotspots'], base['hotspots'])
ent('dead.top', lambda x: x[0], cur['dead']['top'], base['dead']['top'])
ud = {tuple(x) for x in base['dead']['unused_deps']}
newud = [x for x in cur['dead']['unused_deps'] if tuple(x) not in ud]
if newud:
    I.append({'check': 'top-N entrants', 'list': 'dead.unused_deps', 'entrants': newud})

# mutation: scores compare only at EQUAL scope with the unit's last SCORED value
def scope_of(r, u):
    t = r.get('mutation', {}).get('timing', {}).get(u)
    if not t:
        return None
    return (t['scope'], t.get('in_diff') or tuple(t.get('files') or []), t.get('planned'))


for u, s in cur['mutation']['scores'].items():
    last = next((r for r in reversed(prior) if u in (r.get('mutation') or {}).get('scores', {})), None)
    if last is None:
        I.append({'check': 'mutation', 'unit': u, 'note': 'no prior score — first scored at this boundary', 'now': s,
                  'scope': scope_of(cur, u)})
        continue
    sc0, sc1 = scope_of(last, u), scope_of(cur, u)
    comparable = sc0 is not None and sc0[0] in ('unit', 'files', 'shard') and sc0 == sc1
    row = {'check': 'mutation-drop', 'unit': u, 'was': last['mutation']['scores'][u], 'was_epoch': last['epoch'],
           'was_scope': sc0, 'now': s, 'now_scope': sc1}
    if not comparable:
        row['note'] = 'not comparable'
        I.append(row)
    elif s <= row['was'] - 10:
        P.append(row)
    else:
        B.append(row)
for u, st in cur['mutation']['unit_states'].items():
    if u not in cur['mutation']['scores']:
        I.append({'check': 'mutation', 'unit': u, 'note': f'no score: {st}'})

if span > 1:
    for x in P:
        x['span'] = span
    I.extend(P)
    P = []
out = {'baseline': {'sha': base['sha'], 'epoch': base['epoch'], 'ts': base['ts']},
       'baseline_of_baseline': {'sha': bb['sha'], 'epoch': bb['epoch']}, 'tool_version_breaks': tv_break,
       'proposals': P, 'informational': I, 'below': B, 'ledger_records': len(recs)}
json.dump(out, open(f'{rd}/judge.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(f"{len(P)} proposals · {len(I)} informational · {len(B)} below threshold · breaks {tv_break}")
for x in P:
    print('P', json.dumps(x, ensure_ascii=False)[:300])
