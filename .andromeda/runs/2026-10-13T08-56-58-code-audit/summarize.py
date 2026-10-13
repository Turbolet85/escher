#!/usr/bin/env python3
"""Code audit summarizers (pinned per collectors.md A1-A5, B1-B2).

usage: summarize.py <run_dir> <tree_root> <baseline_sha|-> <out_prefix>
  tree_root   the tree measured (the repo root, or an extracted tree for a reproduction check)
  out_prefix  'c-' for this run's evidence twins; another prefix for a reproduction check
Writes {run_dir}/{out_prefix}{metric}.json. Graph, dead, churn and hotspots run only when
tree_root is the repository itself.
"""
import json, math, os, re, subprocess, sys

RD, ROOT, BASE, PFX = sys.argv[1], os.path.abspath(sys.argv[2]), sys.argv[3], sys.argv[4]
REPO = subprocess.run(['git', 'rev-parse', '--show-toplevel'], capture_output=True, text=True).stdout.strip()
IS_REPO = os.path.realpath(ROOT) == os.path.realpath(REPO)
SRC_PATHSPEC = ['*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']


def nr(vals, p):  # nearest-rank percentile on sorted values
    s = sorted(vals)
    return s[max(0, math.ceil(p / 100 * len(s)) - 1)] if s else None


def write(metric, obj):
    with open(os.path.join(RD, f'{PFX}{metric}.json'), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


def rel(p):
    p = os.path.abspath(p)
    return os.path.relpath(p, ROOT).replace(os.sep, '/')


srcfiles = [l.strip() for l in open(os.path.join(RD, f'{PFX}srcfiles.txt' if PFX != 'c-' else 'srcfiles.txt'), encoding='utf-8') if l.strip()]
srcset = set(srcfiles)

# ---- A3 sizes (tokei, population = srcfiles) ----
tok = json.load(open(os.path.join(RD, f'{PFX}tokei.json' if PFX != 'c-' else 'tokei.json'), encoding='utf-8'))
per_file = {}
for lang, d in tok.items():
    if lang == 'Total' or not isinstance(d, dict):
        continue
    for r in d.get('reports', []):
        f = rel(os.path.join(ROOT, r['name']))
        if f in srcset:
            per_file[f] = per_file.get(f, 0) + r['stats']['code']
vals = list(per_file.values())
top = sorted(per_file.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
sizes = {'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
         'over_800': sum(v > 800 for v in vals), 'top': [list(t) for t in top]}
totals = {'loc': sum(vals), 'files': len(vals), 'population': len(srcfiles)}
write('sizes', {'sizes': sizes, 'totals': totals})

# ---- A1 duplication (jscpd) ----
jd = json.load(open(os.path.join(RD, f'{PFX}jscpd-abs' if PFX != 'c-' else 'jscpd-abs', 'jscpd-report.json'), encoding='utf-8'))
st = jd['statistics']['total']


def is_test(p):
    return 'tests' in p.split('/')[:-1]


split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
rows = []
for d in jd['duplicates']:
    a, b, n = rel(d['firstFile']['name']), rel(d['secondFile']['name']), d['lines']
    t = is_test(a) + is_test(b)
    k = 'test' if t == 2 else 'src' if t == 0 else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += n
    rows.append((a, b, n, d['firstFile']['startLoc']['line'], d['secondFile']['startLoc']['line']))
rows.sort(key=lambda r: (-r[2], r[0], r[1], r[3], r[4]))
dup = {'pct': round(st['percentage'], 3), 'duplicated_lines': st['duplicatedLines'], 'total_lines': st['lines'],
       'clones': st['clones'], 'top': [[a, b, n] for a, b, n, _, _ in rows[:10]], 'split': split,
       'top_starts': [[f'{a}:{sa}', f'{b}:{sb}', n] for a, b, n, sa, sb in rows[:10]]}
write('duplication', dup)

# ---- A2 complexity (rust-code-analysis, every space of kind function, closures included) ----
fns = []
rca_dir = os.path.join(RD, f'{PFX}rca' if PFX != 'c-' else 'rca')


def walk(s, f):
    if s.get('kind') == 'function':
        fns.append((s['name'], f, s['start_line'], s['metrics']['cognitive']['sum'], s['metrics']['cyclomatic']['sum']))
    for c in s.get('spaces', []):
        walk(c, f)


for dp, _, fs in os.walk(rca_dir):
    for fn in fs:
        if fn.endswith('.json'):
            d = json.load(open(os.path.join(dp, fn), encoding='utf-8'))
            walk(d, rel(os.path.join(ROOT, d['name'])) if not os.path.isabs(d['name']) else rel(d['name']))
cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
ftop = sorted(fns, key=lambda x: (-x[3], x[1], x[2]))[:10]
mx = ftop[0]
cx = {'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90), 'cognitive_p50': nr(cog, 50),
      'cognitive_p90': nr(cog, 90), 'over_ceiling': sum(c > 15 for c in cog),
      'max': {'fn': mx[0], 'file': mx[1], 'val': mx[3]},
      'top': [[n, f'{f}:{l}', v] for n, f, l, v, _ in ftop], 'functions': len(fns)}
write('complexity', cx)
file_max_cog = {}
for n, f, l, c, _ in fns:
    file_max_cog[f] = max(file_max_cog.get(f, 0), c)

if not IS_REPO:
    sys.exit(0)

# ---- A4 graph (canonical SQL, via code-graph.py query) ----


def q(sql):
    r = subprocess.run([sys.executable, os.path.join(REPO, 'scripts/code-graph.py'), 'query', RD, 'code-audit', sql],
                       capture_output=True, text=True, cwd=REPO)
    if r.returncode != 0:
        sys.exit('query failed: ' + r.stderr[-2000:])
    return json.loads(r.stdout)


CYC = """WITH RECURSIVE walk(start, cur, path, depth) AS (
  SELECT from_crate, to_crate, from_crate || '>' || to_crate, 1 FROM crate_edges
  UNION ALL
  SELECT w.start, e.to_crate, w.path || '>' || e.to_crate, w.depth + 1
  FROM crate_edges e JOIN walk w ON e.from_crate = w.cur
  WHERE w.depth < (SELECT count(DISTINCT from_crate) + 1 FROM crate_edges)
    AND (position(e.to_crate IN w.path) = 0 OR e.to_crate = w.start))
SELECT DISTINCT start, path FROM walk WHERE cur = start"""
cycles = q(CYC)
PFX_RE = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')
show = lambda s: PFX_RE.sub(r'\1 ', s)
fan_in = q("SELECT callee, count(DISTINCT caller) AS n FROM calls_m GROUP BY callee ORDER BY n DESC LIMIT 20")
fan_out = q("SELECT from_crate, count(DISTINCT to_crate) AS n FROM crate_edges GROUP BY from_crate ORDER BY n DESC")
edges = q("SELECT count(*) AS n FROM crate_edges")[0]['n']
graph = {'cycles': len(cycles), 'cycle_paths': sorted(r['path'] for r in cycles),
         'fan_in_top': [[show(r['callee']), r['n']] for r in fan_in],
         'fan_out': [[r['from_crate'], r['n']] for r in sorted(fan_out, key=lambda r: (-r['n'], r['from_crate']))],
         'cross_unit_edges': edges}
write('graph', graph)

# ---- A5 dead (cargo-machete + zero-ref, recipes.dead verbatim) ----
unused, cur = [], None
for line in open(os.path.join(RD, 'machete.txt'), encoding='utf-8'):
    m = re.match(r'^(\S+) -- \./.*Cargo\.toml:$', line.rstrip())
    if m:
        cur = m.group(1)
        continue
    if cur and line.startswith('\t') and line.strip():
        unused.append([cur, line.strip()])
    elif not line.strip():
        cur = None
zr = q("SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL")
STRIP = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
classes = {k: 0 for k in ('candidate', 'derive/attr-invoked', 'entry-point', 'format-capture',
                          'runtime-invoked (macro-generated binding)', 'test-only', 'trait-impl (dispatch)')}
excluded_test = 0
resid = []
for r in zr:
    sp = STRIP.sub('', r['symbol'])
    if '/tests/' in '/' + sp or '/tests/' in '/' + (r['file'] or ''):
        excluded_test += 1
        continue
    if TRAIT.search(sp):
        classes['trait-impl (dispatch)'] += 1
        continue
    resid.append((r, sp))
line_count = {}
for r, sp in resid:
    k = (r['file'], r['def_line'])
    line_count[k] = line_count.get(k, 0) + 1
src_cache = {}


def src(f):
    if f not in src_cache:
        try:
            src_cache[f] = open(os.path.join(REPO, f), encoding='utf-8').read()
        except (OSError, TypeError, UnicodeDecodeError):
            src_cache[f] = ''
    return src_cache[f]


cands = []
for r, sp in resid:
    segs = sp.split('/')[:-1]
    lines = src(r['file']).splitlines()
    dl = r['def_line'] if r['def_line'] is not None else -1
    above = lines[max(0, dl - 4):max(0, dl)] if dl >= 0 else []
    if sp == 'main().' or sp == 'crate/':
        c = 'entry-point'
    elif any(s in ('test', 'tests') or s.endswith('_test') or s.endswith('_tests') for s in segs) or \
            any('#[test]' in a or '#[cfg(test)]' in a for a in above):
        c = 'test-only'
    elif '__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
        c = 'derive/attr-invoked'
    elif line_count[(r['file'], r['def_line'])] > 1:
        c = 'runtime-invoked (macro-generated binding)'
    elif ('{' + r['name'] + '}') in src(r['file']) or ('{' + r['name'] + ':') in src(r['file']):
        c = 'format-capture'
    else:
        c = 'candidate'
        cands.append((r['file'], dl, show(r['symbol'])))
    classes[c] += 1
cands.sort()
dead = {'unused_deps': unused, 'zero_ref_candidates': classes['candidate'],
        'top': [[s, f'{f}:{l + 1}'] for f, l, s in cands[:10]], 'fp_classes': classes,
        'zero_ref_rows': len(zr), 'excluded_test_path': excluded_test,
        'candidates_full': [[s, f'{f}:{l + 1}'] for f, l, s in cands]}
write('dead', dead)

# ---- B1 churn / B2 hotspots ----
if BASE != '-':
    out = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{BASE}..HEAD', '--'] + SRC_PATHSPEC,
                         capture_output=True, text=True, cwd=REPO).stdout
    commit, touches = None, {}

    def newpath(p):
        m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
        if m:
            return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
        if ' => ' in p:
            return p.split(' => ')[1]
        return p

    for line in out.splitlines():
        if line.startswith('@'):
            commit = line[1:]
            continue
        parts = line.split('\t')
        if len(parts) != 3 or parts[0] == '-':
            continue
        f = newpath(parts[2])
        touches.setdefault(f, []).append((commit, int(parts[0])))
    all_adds = sum(a for t in touches.values() for _, a in t)
    churned = {f: sum(a for _, a in t[1:]) for f, t in touches.items() if len(t) > 1}
    churn = {'pct': round(100 * sum(churned.values()) / all_adds, 2) if all_adds else None,
             'files_churned': len(churned), 'files_touched': len(touches), 'adds': all_adds,
             'churned_adds': sum(churned.values()), 'commits': out.count('\n@') + out.startswith('@')}
    write('churn', churn)
    hs = sorted(([f, len(t) * file_max_cog.get(f, 0)] for f, t in touches.items()), key=lambda x: (-x[1], x[0]))[:10]
    write('hotspots', {'hotspots': hs, 'commits_per_file': {f: len(t) for f, t in sorted(touches.items())}})
