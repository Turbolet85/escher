"""Graph (A4) + zero-ref (A5) queries: audit-pass.md canonical SQL, fired one at a time through
scripts/code-graph.py query {run_dir} code-audit "<sql>". Writes {run_dir}/q-{name}.json (raw rows,
deleted after summarization) and prints row counts.

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
for name, sql in Q.items():
    p = subprocess.run([sys.executable, 'scripts/code-graph.py', 'query', rd, 'code-audit', sql],
                       capture_output=True, text=True)
    if p.returncode != 0:
        sys.exit(f'{name}: rc {p.returncode}: {p.stderr[-500:]}')
    rows = json.loads(p.stdout)
    json.dump(rows, open(f'{rd}/q-{name}.json', 'w', encoding='utf-8'))
    print(name, len(rows))
