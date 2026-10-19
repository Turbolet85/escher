"""Graph (A4) + zero-ref (A5 input) collector — audit-pass.md canonical SQL, fired one at a time
through `scripts/code-graph.py query {run_dir} code-audit "<sql>"`.

usage: graph_queries.py RUN_DIR
Writes RUN_DIR/c-graph.json and RUN_DIR/zero-ref.json (raw zero-ref rows, input to sum_dead.py).
A symbol is printed as `{crate} {path}`: the SCIP prefix `rust-analyzer cargo {crate} {version} ` folded.
"""
import json, re, subprocess, sys

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
        sys.exit(f'query failed rc={p.returncode}: {p.stderr[-800:]}')
    return json.loads(p.stdout)


def short(sym):
    return re.sub(r'^rust-analyzer cargo (\S+) \S+ ', r'\1 ', sym)


cyc = q(CYCLES)
fi = q(FAN_IN)
fo = q(FAN_OUT)
ed = q(EDGES)
zr = q(ZERO)
paths = sorted({r['path'] for r in cyc})
g = {
    'cycles': len(paths), 'cycle_paths': paths,
    'fan_in_top': [[short(r['callee']), r['n']] for r in fi],
    'fan_out': [[r['from_crate'], r['n']] for r in fo],
    'cross_unit_edges': list(ed[0].values())[0],
}
json.dump(g, open(f'{rd}/c-graph.json', 'w', encoding='utf-8'), indent=1)
json.dump(zr, open(f'{rd}/zero-ref.json', 'w', encoding='utf-8'))
print(json.dumps({k: g[k] for k in ('cycles', 'cross_unit_edges')}), 'fan_in_top', g['fan_in_top'][:5],
      'zero-ref rows', len(zr))
