"""Run the canonical graph SQL (audit-pass.md) through code-graph.py query; dump raw rows to graph-raw.json."""
import json, subprocess, sys

RD = sys.argv[1]
Q = {
    "cycles": """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start""",
    "fan_in": "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC, callee LIMIT 20",
    "fan_out": "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC, from_crate",
    "edges": "SELECT count(*) AS n FROM crate_edges",
    "zero_ref": "SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL",
}
out = {}
for k, sql in Q.items():
    p = subprocess.run([sys.executable, "scripts/code-graph.py", "query", RD, "code-audit", sql],
                       capture_output=True, text=True)
    if p.returncode != 0:
        sys.exit(f"{k}: rc={p.returncode} {p.stderr[-800:]}")
    out[k] = json.loads(p.stdout)
    print(k, len(out[k]))
json.dump(out, open(f"{RD}/graph-raw.json", "w", encoding="utf-8"))
