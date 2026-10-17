"""Fire audit-pass.md's canonical graph SQL (verbatim) and the A5 zero-ref query one at a time
through `scripts/code-graph.py query {run_dir} code-audit`, and summarize A4 into c-graph.json.

usage: graph_queries.py RUN_DIR
"""
import json, subprocess, sys

rd = sys.argv[1]
Q = {
    'cycles': """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start;""",
    'fan_in': "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20;",
    'fan_out': "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC;",
    'edges': "SELECT count(*) FROM crate_edges;",
    'zero_ref': "SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL",
}
res = {}
for k, sql in Q.items():
    p = subprocess.run(['python3', 'scripts/code-graph.py', 'query', rd, 'code-audit', sql],
                       capture_output=True, text=True)
    if p.returncode != 0:
        sys.exit(f'{k}: exit {p.returncode}: {p.stderr[-400:]}')
    res[k] = json.loads(p.stdout)
json.dump(res['zero_ref'], open(f'{rd}/zero-ref.raw.json', 'w', encoding='utf-8'))


def v(row, i):
    return list(row.values())[i]


fan_in = [[v(r, 0), v(r, 1)] for r in res['fan_in']]
fan_out = [[v(r, 0), v(r, 1)] for r in res['fan_out']]
cycles = sorted({v(r, 1) for r in res['cycles']})
out = {'cycles': len(cycles), 'cycle_paths': cycles, 'fan_in_top': fan_in, 'fan_out': fan_out,
       'cross_unit_edges': v(res['edges'][0], 0)}
json.dump(out, open(f'{rd}/c-graph.json', 'w', encoding='utf-8'), indent=1)
print('cycles', len(cycles), 'edges', out['cross_unit_edges'], 'zero_ref_rows', len(res['zero_ref']))
