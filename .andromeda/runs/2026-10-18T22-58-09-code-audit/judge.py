"""Phase 4 — diff and judge (audit-pass.md §Thresholds), the baseline re-found by sha.

usage: judge.py RUN_DIR
Reads RUN_DIR/record.json (this run) and the ledger; the baseline = the LAST record before this run's
append whose sha == record.baseline_sha; monotonic's third = the record whose sha == baseline.baseline_sha.
Writes RUN_DIR/judgement.json.
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
# exclude this run's own record if already appended
pre = [r for r in recs if r.get('ts') != cur['ts']]
base = [r for r in pre if r['sha'] == cur['baseline_sha']][-1]
prev = [r for r in pre if r['sha'] == base['baseline_sha']]
prev = prev[-1] if prev else None
span = cur['span']
P, I, B = [], [], []


def tv(rec, tool):
    v = (rec.get('tool_versions') or {}).get(tool)
    return v.split()[0] if v else None


def broke(tool):
    return tv(cur, tool) != tv(base, tool)


def get(rec, path):
    v = rec
    for k in path.split('.'):
        v = v.get(k) if isinstance(v, dict) else None
    return v


# new-cycle
if cur['graph']['cycles'] > base['graph']['cycles']:
    P.append(('new-cycle', 'graph.cycles', base['graph']['cycles'], cur['graph']['cycles']))
else:
    B.append(('new-cycle', 'graph.cycles', base['graph']['cycles'], cur['graph']['cycles']))
# duplication-up
b, c = base['duplication']['pct'], cur['duplication']['pct']
fire = c >= b + 0.5 and c >= b * 1.15
(P if fire and span == 1 and not broke('jscpd') else B).append(('duplication-up', 'duplication.pct', b, c))
# complexity-creep
b, c = base['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
fire = c >= b + 3 and c >= b * 1.25
(P if fire and span == 1 and not broke('rust-code-analysis') else B).append(('complexity-creep', 'complexity.over_ceiling', b, c))
# dead-growth
b, c = base['dead']['zero_ref_candidates'], cur['dead']['zero_ref_candidates']
(P if c >= b + 5 and span == 1 else B).append(('dead-growth', 'dead.zero_ref_candidates', b, c))
# coverage-drop
b, c = base['coverage']['line'], cur['coverage']['line']
(P if b is not None and c is not None and c <= b - 2 and span == 1 else B).append(('coverage-drop', 'coverage.line', b, c))
# monotonic (closed set of six)
SIX = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1),
       ('sizes.file_max', 1), ('sizes.over_800', 1), ('coverage.line', -1)]
mono = []
for path, sign in SIX:
    vals = [get(r, path) if r else None for r in (prev, base, cur)]
    if any(v is None for v in vals):
        mono.append((path, vals, 'not evaluable'))
        continue
    w1 = (vals[1] - vals[0]) * sign > 0
    w2 = (vals[2] - vals[1]) * sign > 0
    mono.append((path, vals, 'FIRES' if w1 and w2 else 'no'))
    if w1 and w2:
        P.append(('monotonic', path, vals, None))
# count-under-ratio
bd, cd = base['duplication'], cur['duplication']
if (cd['clones'] > bd['clones'] or cd['duplicated_lines'] > bd['duplicated_lines']) and cd['pct'] <= bd['pct']:
    I.append(('count-under-ratio', bd, cd))
# top-N entrants
def ent(pathb, key=lambda x: x[0]):
    bs = {key(x) for x in get(base, pathb) or []}
    return [x for x in get(cur, pathb) or [] if key(x) not in bs]
entr = {
    'duplication.top': ent('duplication.top', key=lambda x: (x[0], x[1], x[2])),
    'complexity.top': ent('complexity.top', key=lambda x: (x[0], x[1].rsplit(':', 1)[0])),
    'sizes.top': ent('sizes.top'),
    'graph.fan_in_top': ent('graph.fan_in_top'),
    'dead.top': ent('dead.top'),
    'hotspots': ent('hotspots'),
}
# mutation: last scored value at equal scope
mut = []
for u, s in cur['mutation']['scores'].items():
    t = cur['mutation']['timing'].get(u, {})
    last = None
    for r in reversed(pre):
        if u in (r.get('mutation') or {}).get('scores', {}):
            last = (r['epoch'], r['sha'][:10], r['mutation']['scores'][u], (r['mutation'].get('timing') or {}).get(u))
            break
    if last is None:
        mut.append((u, s, t.get('scope'), 'no prior score'))
        continue
    lt = last[3] or {}
    same = lt.get('scope') == t.get('scope') and lt.get('files') == t.get('files') and \
        (not str(t.get('scope', '')).startswith('shard') or lt.get('planned') == t.get('planned'))
    if not same:
        mut.append((u, s, t.get('scope'), f'not comparable with {last[2]} ({lt.get("scope")}, {last[0]} {last[1]})'))
    elif s <= last[2] - 10:
        P.append(('mutation-drop', u, last[2], s))
    else:
        mut.append((u, s, t.get('scope'), f'equal scope vs {last[2]}: no drop'))
res = {'baseline_ts': base['ts'], 'baseline_epoch': base['epoch'], 'prev_ts': prev and prev['ts'],
       'span': span, 'proposals': P, 'informational': I, 'below': B, 'monotonic': mono, 'entrants': entr,
       'mutation': mut,
       'trend_breaks': [t for t in cur['tool_versions'] if broke(t)]}
json.dump(res, open(f'{rd}/judgement.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(json.dumps(res, ensure_ascii=False, indent=1))
