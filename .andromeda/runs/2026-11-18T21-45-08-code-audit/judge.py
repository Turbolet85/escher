"""P4 inputs: this run's c-*.json against the baseline record (re-found by sha) and the baseline's own baseline.
usage: python3 judge.py {run_dir} {baseline_sha} -> prints deltas, monotonic chain, top-N entrants, prior mutation."""
import json, sys

rd, bsha = sys.argv[1:3]
recs = [r['record'] for r in json.load(open(f'{rd}/ledger.json', encoding='utf-8'))]
recs = [json.loads(r) if isinstance(r, str) else r for r in recs]
b = [r for r in recs if r['sha'] == bsha][-1]
bb = [r for r in recs if r['sha'] == b['baseline_sha']][-1]
c = {k: json.load(open(f'{rd}/c-{k}.json', encoding='utf-8')) for k in ('sizes', 'complexity', 'duplication', 'graph', 'dead', 'churn')}
cur = {'duplication.pct': c['duplication']['pct'], 'complexity.over_ceiling': c['complexity']['over_ceiling'],
       'dead.zero_ref_candidates': c['dead']['zero_ref_candidates'], 'sizes.file_max': c['sizes']['file_max'],
       'sizes.over_800': c['sizes']['over_800'], 'coverage.line': float(sys.argv[3]) if len(sys.argv) > 3 else None}


def get(r, path):
    a, k = path.split('.')
    return (r.get(a) or {}).get(k)


print('MONOTONIC (bb -> b -> cur)')
for k, v in cur.items():
    x0, x1 = get(bb, k), get(b, k)
    worse = (lambda a, z: z < a) if k == 'coverage.line' else (lambda a, z: z > a)
    both = None not in (x0, x1, v) and worse(x0, x1) and worse(x1, v)
    print(f'  {k}: {x0} -> {x1} -> {v}  worsened-both={both}')
for k in ('clones', 'duplicated_lines', 'total_lines'):
    print('  dup', k, b['duplication'][k], '->', c['duplication'][k])
print('  totals', b['totals'], '->', c['sizes']['totals'])
print('  p50/p90', b['sizes']['file_p50'], b['sizes']['file_p90'], '->', c['sizes']['file_p50'], c['sizes']['file_p90'])
print('  cx', {k: b['complexity'][k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90')},
      {k: c['complexity'][k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90')})
print('  graph edges', b['graph']['cross_unit_edges'], '->', c['graph']['cross_unit_edges'], 'cycles', b['graph']['cycles'], '->', c['graph']['cycles'])
print('  churn', b['churn'], '->', c['churn']['churn'])


def entr(name, old, new, key=lambda x: tuple(x) if isinstance(x, list) else json.dumps(x, sort_keys=True)):
    o = {key(x) for x in old}
    e = [x for x in new if key(x) not in o]
    print(f'ENTRANTS {name}: {len(e)}')
    for x in e:
        print('   ', x)


entr('sizes.top', [x[0] for x in b['sizes']['top']], [x[0] for x in c['sizes']['top']], key=lambda x: x)
entr('complexity.top', [(x['fn'], x['file'].split(':')[0]) for x in b['complexity']['top']],
     [(x['fn'], x['file'].split(':')[0]) for x in c['complexity']['top']])
entr('duplication.top', b['duplication']['top'], c['duplication']['top'])
entr('graph.fan_in_top', [x[0] for x in b['graph']['fan_in_top']], [x[0] for x in c['graph']['fan_in_top']], key=lambda x: x)
entr('hotspots', [x[0] for x in b['hotspots']], [x[0] for x in c['churn']['hotspots']], key=lambda x: x)
entr('dead.top', b['dead']['top'], c['dead']['top'])
entr('dead.unused_deps', b['dead']['unused_deps'], c['dead']['unused_deps'])
entr('dead.unused_deps (gone)', c['dead']['unused_deps'], b['dead']['unused_deps'])
print('fan_out', b['graph']['fan_out'] == c['graph']['fan_out'])
print('PRIOR MUTATION per unit (record ts, epoch, state, score, counts, scope)')
for u in ('seven_guis', 'browser', 'blitz-paint', 'escher-driver', 'blitz-dom', 'wpt', 'dioxus-native', 'wgpu_texture', 'wasm_hello'):
    rows = []
    for r in recs:
        m = r.get('mutation') or {}
        if u in m.get('unit_states', {}):
            t = (m.get('timing') or {}).get(u, {})
            rows.append((r['ts'][:10], r.get('version'), r['epoch'][:8], m['unit_states'][u], m.get('scores', {}).get(u),
                         (m.get('counts') or {}).get(u), t.get('scope')))
    print(' ', u)
    for x in rows[-5:]:
        print('    ', x)
