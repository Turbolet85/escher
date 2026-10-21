"""P4 diff-and-judge + P5 render (audit-pass.md §Thresholds; proposal-template.md).

usage: judge.py {run_dir}
Reads this run's record from the ledger by its own ts (after P3's append), the baseline as the last
record BEFORE it whose sha == baseline_sha, and the baseline's own baseline by the same chain rule.
Writes {run_dir}/judgment.json and {run_dir}/proposals.md; asserts every evidence table's row count.
"""
import json, sys

rd = sys.argv[1]
mine = json.load(open(f'{rd}/record.json', encoding='utf-8'))
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
idx = max(i for i, r in enumerate(recs) if r['ts'] == mine['ts'] and r['sha'] == mine['sha'])
cur = recs[idx]
bi = max(i for i in range(idx) if recs[i]['sha'] == cur['baseline_sha'])
base = recs[bi]
b2i = max(i for i in range(bi) if recs[i]['sha'] == base['baseline_sha'])
base2 = recs[b2i]
n_records = len(recs)

E_A, E_B = base['epoch'], cur['epoch']
props, info, below = [], [], []

# --- trend-break check (leading token against leading token)
breaks = [k for k, v in cur['tool_versions'].items() if base['tool_versions'].get(k) != v]

# --- thresholds, span 1
g0, g1 = base['graph']['cycles'], cur['graph']['cycles']
(props if g1 > g0 else below).append(('new-cycle', f'graph.cycles {g0} → {g1}'))
d0, d1 = base['duplication']['pct'], cur['duplication']['pct']
fires = d1 >= d0 + 0.5 and d1 >= d0 * 1.15
(props if fires else below).append(('duplication-up', f'duplication.pct {d0} → {d1} ({d1 - d0:+.2f} pt)'))
c0, c1 = base['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
fires = c1 >= c0 + 3 and c1 >= c0 * 1.25
(props if fires else below).append(('complexity-creep', f'complexity.over_ceiling {c0} → {c1} ({c1 - c0:+d}, '
                                     f'{100 * (c1 - c0) / c0:+.1f} %; fires at +3 AND +25 %)'))
z0, z1 = base['dead']['zero_ref_candidates'], cur['dead']['zero_ref_candidates']
(props if z1 >= z0 + 5 else below).append(('dead-growth', f'dead.zero_ref_candidates {z0} → {z1} ({z1 - z0:+d})'))
l0, l1 = base['coverage']['line'], cur['coverage']['line']
(props if l1 <= l0 - 2 else below).append(('coverage-drop', f'coverage.line {l0} → {l1} ({l1 - l0:+.2f} pt)'))

# --- mutation-drop: last SCORED value of the unit at EQUAL scope, walking back
mut_lines = []
for u, s in cur['mutation']['scores'].items():
    t = cur['mutation']['timing'].get(u, {})
    prev = None
    for r in reversed(recs[:idx]):
        if u in (r.get('mutation') or {}).get('scores', {}):
            prev = r
            break
    if prev is None:
        info.append(('mutation', f'{u}: {s} at scope {t.get("scope")} ({E_B}) — no prior score of this unit'))
        continue
    pt = (prev['mutation'].get('timing') or {}).get(u)
    ps = prev['mutation']['scores'][u]
    same = pt is not None and pt.get('scope') == t.get('scope') and (
        t.get('scope') != 'files' or sorted(pt.get('files') or []) == sorted(t.get('files') or []))
    if t.get('scope') == 'in-diff':
        same = False  # an in-diff listing names the window's lines: never the same population twice
    pscope = (pt or {}).get('scope', 'UNKNOWN')
    pdesc = pscope + (f' {pt.get("files")}' if pscope == 'files' else '')
    cdesc = t.get('scope') + (f' {t.get("files")}' if t.get('scope') == 'files' else '')
    if same:
        (props if s <= ps - 10 else below).append(('mutation-drop', f'{u} {ps} → {s} at scope {cdesc}'))
    else:
        info.append(('mutation — not comparable', f'{u}: {ps} at scope {pdesc} ({prev["epoch"]}) · {s} at scope '
                                                  f'{cdesc} ({E_B})'))

# --- monotonic: the six tracked scalars over base2 → base → cur
TRACK = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1),
         ('sizes.file_max', 1), ('sizes.over_800', 1), ('coverage.line', -1)]


def get(r, path):
    a, b = path.split('.')
    return (r.get(a) or {}).get(b)


mono = []
for path, sign in TRACK:
    v = [get(base2, path), get(base, path), get(cur, path)]
    if None in v:
        below.append(('monotonic', f'{path} not evaluable this run: {v}'))
        continue
    worse1, worse2 = sign * (v[1] - v[0]) > 0, sign * (v[2] - v[1]) > 0
    line = f'{path} {v[0]} → {v[1]} → {v[2]} ({base2["epoch"]} → {E_A} → {E_B})'
    (props if worse1 and worse2 else below).append(('monotonic', line))

# --- count-under-ratio
dc0, dc1 = base['duplication']['clones'], cur['duplication']['clones']
dl0, dl1 = base['duplication']['duplicated_lines'], cur['duplication']['duplicated_lines']
tl0, tl1 = base['duplication']['total_lines'], cur['duplication']['total_lines']
if (dc1 > dc0 or dl1 > dl0) and d1 <= d0:
    sp0, sp1 = base['duplication']['split'], cur['duplication']['split']
    split = ' · '.join(f'{k} {sp0[k]["pairs"]}→{sp1[k]["pairs"]} pairs / {sp0[k]["lines"]}→{sp1[k]["lines"]} L'
                       for k in ('src', 'test', 'mixed'))
    top = cur['duplication']['top'][0]
    entered = next((r['epoch'] for r in recs[:idx + 1] if top in r.get('duplication', {}).get('top', [])), '?')
    info.append(('count-under-ratio', f'clones {dc0} → {dc1} ({dc1 - dc0:+d}), duplicated_lines {dl0} → {dl1} '
                                      f'({dl1 - dl0:+d}); pct {d0} → {d1} ({d1 - d0:+.2f} pt); population total_lines '
                                      f'{tl0} → {tl1} ({100 * (tl1 - tl0) / tl0:+.1f} %); split {split}; top standing '
                                      f'pair {top[0]} ↔ {top[1]} ({top[2]} L), in the top list since {entered}'))

# --- top-N entrants
def entrants(name, cur_list, base_list, key):
    bk = {key(x) for x in base_list}
    new = [x for x in cur_list if key(x) not in bk]
    for x in new:
        info.append((f'top-N entrant · {name}', json.dumps(x, ensure_ascii=False)))


entrants('duplication.top', cur['duplication']['top'], base['duplication']['top'], lambda x: tuple(x))
entrants('complexity.top', cur['complexity']['top'], base['complexity']['top'], lambda x: (x[0], x[1].rsplit(':', 1)[0]))
entrants('sizes.top', cur['sizes']['top'], base['sizes']['top'], lambda x: x[0])
entrants('graph.fan_in_top', cur['graph']['fan_in_top'], base['graph']['fan_in_top'], lambda x: x[0])
entrants('dead.top', cur['dead']['top'], base['dead']['top'], lambda x: tuple(x))
entrants('hotspots', cur['hotspots'], base['hotspots'], lambda x: x[0])

# --- movements without a threshold rule
for path in ('totals.loc', 'totals.files', 'sizes.file_p50', 'sizes.file_p90', 'sizes.file_max', 'sizes.over_800',
             'complexity.cyclomatic_p90', 'complexity.cognitive_p90', 'graph.cross_unit_edges', 'churn.pct',
             'churn.files_churned'):
    below.append(('movement', f'{path} {get(base, path)} → {get(cur, path)}'))
ud0 = {tuple(x) for x in base['dead']['unused_deps']}
ud1 = {tuple(x) for x in cur['dead']['unused_deps']}
below.append(('movement', f'dead.unused_deps {len(ud0)} → {len(ud1)} (added {sorted(ud1 - ud0)}, gone {sorted(ud0 - ud1)})'))

json.dump({'proposals': props, 'informational': info, 'below': below, 'trend_breaks': breaks,
           'baseline_ts': base['ts'], 'base2_ts': base2['ts'], 'records': n_records},
          open(f'{rd}/judgment.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(len(props), 'proposals ·', len(info), 'informational ·', len(below), 'below threshold · breaks', breaks)
for p in props:
    print('PROPOSAL', p)
