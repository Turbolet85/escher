"""Fires the canonical audit-pass graph SQL through scripts/code-graph.py query, one at a time
(the trace append is not concurrency-safe). Each stdout lands in q-{name}.json for the summarizers."""
import os
import subprocess
import sys

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'

QUERIES = [
    ('cycles', """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start"""),
    ('fan_in', "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20"),
    ('fan_out', "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC"),
    ('edges', "SELECT count(*) AS n FROM crate_edges"),
    ('edge_list', "SELECT from_crate, to_crate FROM crate_edges ORDER BY from_crate, to_crate"),
    ('crates', "SELECT crate, count(*) AS n FROM symbol GROUP BY crate ORDER BY crate"),
    ('zero_ref', "SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s "
                 "LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL"),
]

only = sys.argv[1:] or [n for n, _ in QUERIES]
for name, sql in QUERIES:
    if name not in only:
        continue
    out = os.path.join(RUN, f'q-{name}.json')
    with open(out, 'w', encoding='utf-8', newline='') as f:
        r = subprocess.run([sys.executable, 'scripts/code-graph.py', 'query', RUN, 'code-audit', sql],
                           stdout=f, stderr=subprocess.PIPE, text=True)
    print(name, 'exit', r.returncode, 'bytes', os.path.getsize(out), '| stderr:', r.stderr.strip()[-400:])
