"""Graph (A4) and dead-code (A5) summarizer.

usage: python3 graph_dead.py {run_dir} {machete.out} {repo_root}
Fires `python3 scripts/code-graph.py query {run_dir} code-audit "<sql>"` one query at a time (audit-pass.md canonical
SQL verbatim; the zero-ref rows without LIMIT), reads each trace, deletes it. Dead classing = the ledger's recipes.dead,
verbatim. Writes c-graph.json and c-dead.json.
"""
import json, os, re, subprocess, sys

run_dir, machete, root = sys.argv[1:4]
TRACE = os.path.join(run_dir, 'tree-query-code-audit.json')
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')


def q(sql):
    r = subprocess.run(['python3', 'scripts/code-graph.py', 'query', run_dir, 'code-audit', sql], cwd=root,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f'query failed rc={r.returncode}: {r.stderr[-400:]}')
    t = json.load(open(TRACE, encoding='utf-8'))
    os.remove(TRACE)
    return t[-1]['result'], t[-1].get('db_state')


CYC = """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, [from_crate, to_crate], 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, list_append(w.path, e.to_crate), w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (NOT list_contains(w.path, e.to_crate) OR e.to_crate = w.start))
SELECT DISTINCT start, array_to_string(path, '>') AS path FROM walk WHERE cur = start;"""
FANIN = "SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20;"
FANOUT = "SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC;"
EDGES = "SELECT count(*) AS n FROM crate_edges;"
ZERO = ("SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s "
        "LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL")

states = []
cyc, st = q(CYC); states.append(st)
fanin, st = q(FANIN); states.append(st)
fanout, st = q(FANOUT); states.append(st)
edges, st = q(EDGES); states.append(st)
zero, st = q(ZERO); states.append(st)


def short(sym):
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', sym)
    return f'{m.group(1)} {m.group(2)}' if m else sym


paths = sorted({r['path'] for r in cyc})
g = {'cycles': len(paths), 'cycle_paths': paths,
     'fan_in_top': [[short(r['callee']), r['n']] for r in fanin],
     'fan_out': [[r['from_crate'], r['n']] for r in fanout], 'cross_unit_edges': edges[0]['n'], 'db_states': states}
json.dump(g, open(os.path.join(run_dir, 'c-graph.json'), 'w', encoding='utf-8'), indent=1)

# ---- dead: recipes.dead verbatim ----
rows = []
for r in zero:
    sp = PFX.sub('', r['symbol'])
    if '/tests/' in '/' + sp or '/tests/' in '/' + (r['file'] or ''):
        continue  # 1) test
    if re.search(r'impl#\[[^\]]*\]\[[^\]]*\]', sp):
        continue  # 2) trait-impl
    rows.append(dict(r, sp=sp))
by_line = {}
for r in rows:
    by_line[(r['file'], r['def_line'])] = by_line.get((r['file'], r['def_line']), 0) + 1
src_cache = {}


def lines(f):
    if f not in src_cache:
        try:
            src_cache[f] = open(os.path.join(root, f), encoding='utf-8').read().splitlines()
        except OSError:
            src_cache[f] = []
    return src_cache[f]


TSEG = re.compile(r'^(test|tests|.*_test|.*_tests)$')


def klass(r):
    sp, f = r['sp'], r['file'] or ''
    if sp == 'main().' or sp == 'crate/':
        return 'entry-point'
    segs = [s for s in re.split(r'[/#.]', sp) if s] + f.split('/')
    if any(TSEG.match(s.removesuffix('.rs')) for s in segs):
        return 'test-only'
    ls = lines(f)
    d = r['def_line'] or 0
    if any(('#[test]' in l or '#[cfg(test)]' in l) for l in ls[max(0, d - 4):d]):
        return 'test-only'
    if '__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
        return 'derive/attr-invoked'
    if by_line[(r['file'], r['def_line'])] > 1:
        return 'runtime-invoked'
    nm = r['name'] or ''
    txt = '\n'.join(ls)
    if nm and ('{' + nm + '}' in txt or '{' + nm + ':' in txt):
        return 'format-capture'
    return 'candidate'


classes = {}
cands = []
for r in rows:
    k = klass(r)
    classes[k] = classes.get(k, 0) + 1
    if k == 'candidate':
        cands.append(r)
cands.sort(key=lambda r: (r['file'] or '', r['def_line'] or 0, r['symbol']))
unused = []
cur = None
for l in open(machete, encoding='utf-8'):
    m = re.match(r'^(\S+) -- \S+Cargo\.toml:', l)
    if m:
        cur = m.group(1)
        continue
    if cur and l.startswith('\t') and l.strip():
        unused.append([cur, l.strip()])
    elif not l.strip():
        cur = None
d = {'unused_deps': unused, 'zero_ref_candidates': len(cands),
     'top': [[short(r['symbol']), f"{r['file']}:{(r['def_line'] or 0) + 1}"] for r in cands[:10]],
     'zero_ref_rows': len(zero), 'classes': classes,
     'fp_classes_note': 'candidates, never dead: entry points, runtime-invoked surfaces, test-only helpers, trait-impl methods, derive/attr-invoked fns, format-string captures'}
json.dump(d, open(os.path.join(run_dir, 'c-dead.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in g.items() if k not in ('fan_in_top', 'fan_out')}), json.dumps({k: v for k, v in d.items() if k not in ('top', 'unused_deps')}), len(unused))
