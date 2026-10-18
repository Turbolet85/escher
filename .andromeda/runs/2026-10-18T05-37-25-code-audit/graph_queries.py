"""A4 graph + A5 zero-ref rows: fires audit-pass.md's canonical SQL through
scripts/code-graph.py query {run_dir} code-audit "<sql>", one at a time.

usage: graph_queries.py {run_dir}
Writes {run_dir}/g-{name}.json (the parsed rows) and {run_dir}/c-graph.json.
"""
import json
import os
import subprocess
import sys

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


def run(sql):
    p = subprocess.run([sys.executable, 'scripts/code-graph.py', 'query', rd, 'code-audit', sql],
                       capture_output=True, text=True, encoding='utf-8')
    if p.returncode != 0:
        sys.exit(f'query failed ({p.returncode}): {p.stderr[-400:]}')
    out = p.stdout
    return json.loads(out[out.index('['):])


res = {}
for k, sql in Q.items():
    res[k] = run(sql)
    json.dump(res[k], open(os.path.join(rd, f'g-{k}.json'), 'w', encoding='utf-8'))
    print(k, len(res[k]))


def ordered(rows):
    return [list(r.values()) for r in rows]


paths = sorted({r['path'] for r in res['cycles']})
def short(sym):  # 'rust-analyzer cargo {crate} {version} {path}' -> '{crate} {path}'
    parts = sym.split(' ', 4)
    return f'{parts[2]} {parts[4]}' if len(parts) == 5 and parts[0] == 'rust-analyzer' else sym


fan_in = [[short(a), b] for a, b in ordered(res['fan_in'])]
fan_in.sort(key=lambda r: (-r[1], r[0]))
fan_out = [[a, b] for a, b in ordered(res['fan_out'])]
fan_out.sort(key=lambda r: (-r[1], r[0]))
graph = {
    'cycles': len(paths), 'cycle_paths': paths, 'fan_in_top': fan_in[:20],
    'fan_out': fan_out, 'cross_unit_edges': ordered(res['edges'])[0][0],
}
json.dump(graph, open(os.path.join(rd, 'c-graph.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: graph[k] for k in ('cycles', 'cross_unit_edges')}))
