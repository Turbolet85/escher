"""Graph (A4) collector + the zero-ref rows for dead code (A5).

usage: graph_queries.py RUN_DIR
Fires scripts/code-graph.py query RUN_DIR code-audit "<sql>" once per canonical query
(audit-pass.md, verbatim; the zero-ref query without LIMIT and with its columns named,
as recipes.dead reads it). Writes RUN_DIR/c-graph.json and RUN_DIR/zero-ref-rows.json.
"""
import json
import os
import subprocess
import sys

rd = sys.argv[1]

CYCLES = """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start;"""
FAN_IN = "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20;"
FAN_OUT = "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC;"
EDGES = "SELECT count(*) FROM crate_edges;"
ZERO = ("SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s "
        "LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL")


def q(sql):
    p = subprocess.run([sys.executable, 'scripts/code-graph.py', 'query', rd, 'code-audit', sql],
                       capture_output=True, text=True)
    if p.returncode != 0:
        sys.exit(f'query failed ({p.returncode}): {p.stderr[-2000:]}')
    return json.loads(p.stdout)


cyc = q(CYCLES)
fin = q(FAN_IN)
fout = q(FAN_OUT)
edges = q(EDGES)
zero = q(ZERO)

paths = sorted({r['path'] for r in cyc})
graph = {
    'cycles': len(paths),
    'cycle_paths': paths,
    'fan_in_top': [[r['callee'], r['n']] for r in fin],
    'fan_out': [[r['from_crate'], r['n']] for r in fout],
    'cross_unit_edges': list(edges[0].values())[0],
}
json.dump({'graph': graph}, open(os.path.join(rd, 'c-graph.json'), 'w', encoding='utf-8'), indent=1)
json.dump(zero, open(os.path.join(rd, 'zero-ref-rows.json'), 'w', encoding='utf-8'))
print(json.dumps({'cycles': graph['cycles'], 'edges': graph['cross_unit_edges'], 'zero_rows': len(zero)}))
