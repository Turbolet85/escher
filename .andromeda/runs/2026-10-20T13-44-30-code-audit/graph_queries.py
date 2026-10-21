"""Fire the canonical graph SQL (audit-pass.md, verbatim) + the A5 zero-ref query, one at a time,
through scripts/code-graph.py query {run_dir} code-audit "<sql>"; write c-graph.json and
zero_ref_rows.json (the dead summarizer's input, deleted after it).

usage: graph_queries.py {run_dir}
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
    out = subprocess.run([sys.executable, 'scripts/code-graph.py', 'query', rd, 'code-audit', sql],
                         capture_output=True, text=True, check=True)
    if out.stderr.strip():
        sys.stderr.write(out.stderr)
    return json.loads(out.stdout)


def strip(sym):
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', sym)
    return f'{m.group(1)} {m.group(2)}' if m else sym


cyc = q(CYCLES)
fi = q(FAN_IN)
fo = q(FAN_OUT)
ed = q(EDGES)
zr = q(ZERO)
g = {
    'cycles': len({r['path'] for r in cyc}),
    'cycle_paths': sorted({r['path'] for r in cyc}),
    'fan_in_top': [[strip(r['callee']), r['n']] for r in fi],
    'fan_out': [[r['from_crate'], r['n']] for r in fo],
    'cross_unit_edges': list(ed[0].values())[0],
}
json.dump(g, open(f'{rd}/c-graph.json', 'w', encoding='utf-8'), indent=1)
json.dump(zr, open(f'{rd}/zero_ref_rows.json', 'w', encoding='utf-8'))
print('graph', {k: v for k, v in g.items() if k in ('cycles', 'cross_unit_edges')}, 'zero-ref rows', len(zr))
