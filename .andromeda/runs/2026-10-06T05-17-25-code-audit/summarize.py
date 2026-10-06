"""Pinned summarizers for /andromeda-code-audit Tier A (A1-A5) — baseline run, Epoch 1.

Run from the repo root: python3 .andromeda/runs/2026-10-06T05-17-25-code-audit/summarize.py
Reads the raw tool outputs in the run dir, writes capped c-{metric}.json twins.
"""
import json, os, re, glob

RD = '.andromeda/runs/2026-10-06T05-17-25-code-audit'
REPO = os.getcwd()
SRC = [l.strip() for l in open(f'{RD}/srcfiles.txt', encoding='utf-8') if l.strip()]


def nearest_rank(vals, pct):
    s = sorted(vals)
    if not s:
        return None
    k = max(1, -(-pct * len(s) // 100))  # ceil(pct/100 * n), nearest-rank
    return s[int(k) - 1]


def is_test_path(p):
    return ('/' + p).find('/tests/') >= 0


def dump(name, obj):
    with open(f'{RD}/c-{name}.json', 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


# ---------- A1 duplication (jscpd, absolute paths -> repo-relative) ----------
rep = json.load(open(f'{RD}/jscpd-abs/jscpd-report.json', encoding='utf-8'))
tot = rep['statistics']['total']
rel = lambda p: os.path.relpath(p, REPO)
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
rows = []
for d in rep['duplicates']:
    a, b = rel(d['firstFile']['name']), rel(d['secondFile']['name'])
    ta, tb = is_test_path(a), is_test_path(b)
    cls = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
    split[cls]['pairs'] += 1
    split[cls]['lines'] += d['lines']
    rows.append([f"{a}:{d['firstFile']['startLoc']['line']}", f"{b}:{d['secondFile']['startLoc']['line']}", d['lines']])
rows.sort(key=lambda r: (-r[2], r[0], r[1]))
dup = {
    'pct': round(tot['percentage'], 3), 'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'],
    'clones': tot['clones'], 'sources': tot['sources'],
    'top': [[r[0].rsplit(':', 1)[0], r[1].rsplit(':', 1)[0], r[2]] for r in rows[:10]],
    'top_sites': rows[:10],
    'split': split,
    'split_rule': "test = a '/tests/' path segment (leading included); pair class test|src|mixed by its two files",
}
dump('duplication', dup)

# ---------- A2 complexity (rust-code-analysis, per FUNCTION space) ----------
funcs = []


def walk(space, file):
    for s in space.get('spaces', []):
        if s['kind'] == 'function':
            m = s['metrics']
            funcs.append({'fn': s['name'], 'file': file, 'line': s['start_line'],
                          'cognitive': m['cognitive']['sum'], 'cyclomatic': m['cyclomatic']['sum'],
                          'sloc': m['loc']['sloc']})
        walk(s, file)


for p in SRC:
    jp = f'{RD}/rca/{p}.json'
    if os.path.isfile(jp):
        walk(json.load(open(jp, encoding='utf-8')), p)
cog = [f['cognitive'] for f in funcs]
cyc = [f['cyclomatic'] for f in funcs]
top = sorted(funcs, key=lambda f: (-f['cognitive'], f['file'], f['line']))[:10]
cx = {
    'functions': len(funcs),
    'cyclomatic_p50': nearest_rank(cyc, 50), 'cyclomatic_p90': nearest_rank(cyc, 90), 'cyclomatic_max': max(cyc),
    'cognitive_p50': nearest_rank(cog, 50), 'cognitive_p90': nearest_rank(cog, 90),
    'over_ceiling': sum(1 for v in cog if v > 15),
    'max': {'fn': top[0]['fn'], 'file': top[0]['file'], 'val': top[0]['cognitive']},
    'top': [{'fn': f['fn'], 'file': f"{f['file']}:{f['line']}", 'value': f['cognitive'], 'cyclomatic': f['cyclomatic']} for f in top],
    'fn_sloc_p50': nearest_rank([f['sloc'] for f in funcs], 50), 'fn_sloc_p90': nearest_rank([f['sloc'] for f in funcs], 90),
    'fn_sloc_max': max(f['sloc'] for f in funcs),
    'rule': "per rca 'function' space (nested included, each its own row): metrics.cognitive.sum / cyclomatic.sum; ceiling cognitive > 15; nearest-rank",
}
dump('complexity', cx)

# per-file max cognitive (B2's future input)
pf = {}
for f in funcs:
    pf[f['file']] = max(pf.get(f['file'], 0), f['cognitive'])

# ---------- A3 sizes (tokei over the source-path population) ----------
tk = json.load(open(f'{RD}/tokei.raw.json', encoding='utf-8'))
per = {r['name']: r['stats']['code'] for r in tk['Rust']['reports']}
missing = sorted(set(SRC) - set(per))
vals = [per[p] for p in SRC if p in per]
topf = sorted(((p, per[p]) for p in SRC if p in per), key=lambda x: (-x[1], x[0]))
sizes = {
    'file_p50': nearest_rank(vals, 50), 'file_p90': nearest_rank(vals, 90), 'file_max': max(vals),
    'over_800': sum(1 for v in vals if v > 800), 'top': [[p, n] for p, n in topf[:10]],
    'totals': {'loc': sum(vals), 'files': len(vals)}, 'missing_from_tokei': missing,
    'population': "git ls-files '*.rs' minus .andromeda/ .claude/ scripts/ docs/ refs/ (srcfiles.txt) - tokei Rust code lines (embedded doc-comment Markdown blobs excluded)",
}
dump('sizes', sizes)

# ---------- A4 graph (trace rows of tree-query-code-audit.json) ----------
tr = json.load(open(f'{RD}/tree-query-code-audit.json', encoding='utf-8'))
by = lambda frag: [q for q in tr if frag in q['sql']][-1]
cyc_q = by('WITH RECURSIVE walk')
fan_in = by('FROM calls_m GROUP BY callee')
fan_out = by('GROUP BY from_crate')
edges = by('SELECT count(*) FROM crate_edges')
strip = lambda s: re.sub(r'^rust-analyzer cargo (\S+) \S+ ', r'\1 ', s)
graph = {
    'db_state': cyc_q.get('db_state'),
    'cycles': cyc_q['rows'], 'cycle_paths': [r['path'] for r in cyc_q['result']],
    'fan_in_top': [[strip(r['callee']), r['n']] for r in fan_in['result']][:20],
    'fan_out': [[r['from_crate'], r['n']] for r in fan_out['result']],
    'cross_unit_edges': list(edges['result'][0].values())[0],
}
dump('graph', graph)

# ---------- A5 dead code (zero-ref candidates, pinned classing) ----------
counts_q = by("count(*) FILTER (WHERE ('/' ||")
trait_q = by('AS trait_impl FROM nt')
resid = json.load(open(f'{RD}/dead-residual.raw.json', encoding='utf-8'))
zero_ref = counts_q['result'][0]['zero_ref']
test_excl = counts_q['result'][0]['test_excluded']
trait_impl = trait_q['result'][0]['trait_impl']
assert trait_q['result'][0]['non_test'] - trait_impl == len(resid), 'residual pull disagrees with the class counts'

src_cache = {}


def lines_of(f):
    if f not in src_cache:
        try:
            src_cache[f] = open(f, encoding='utf-8').read().split('\n')
        except OSError:
            src_cache[f] = []
    return src_cache[f]


TEST_SEG = re.compile(r'(^|/)(test|tests|\w+_tests?)/')
GEN = re.compile(r'(__bitflags_flag_names/|Props#|StoreImplExt#)')


def klass(r):
    p, name, kind, f, dl = r['p'], r['name'], r['kind'], r['file'], r['def_line']
    if (kind == 'fn' and name == 'main' and p == 'main().') or (kind == 'module' and p == 'crate/'):
        return 'entry-point'
    src = lines_of(f)
    pre = '\n'.join(src[max(0, dl - 4):dl + 1])
    if TEST_SEG.search(p) or re.search(r'#\[(test|cfg\(test\))\]', pre):
        return 'test-only'
    if GEN.search(p):
        return 'derive/attr-invoked'
    # multiple symbols defined on one source line = a macro expansion site (e.g. Boa accessor tables)
    if sum(1 for o in resid if o['file'] == f and o['def_line'] == dl) > 1:
        return 'runtime-invoked (macro-generated binding)'
    body = '\n'.join(src)
    if kind in ('term', 'fn') and re.search(r'\{' + re.escape(name) + r'[}:]', body):
        return 'format-capture'
    return 'candidate'


classes = {}
cands = []
for r in resid:
    k = klass(r)
    classes[k] = classes.get(k, 0) + 1
    if k == 'candidate':
        cands.append(r)
dead = {
    'zero_ref_rows': zero_ref,
    'excluded_test_path': test_excl,
    'fp_classes': {'trait-impl (dispatch)': trait_impl, **dict(sorted(classes.items()))},
    'zero_ref_candidates': len(cands),
    'candidates_by_kind': {k: sum(1 for c in cands if c['kind'] == k) for k in sorted({c['kind'] for c in cands})},
    'candidates_by_crate': dict(sorted({c['crate']: sum(1 for x in cands if x['crate'] == c['crate']) for c in cands}.items(), key=lambda kv: -kv[1])),
    'top': [[f"{c['crate']} {c['p']}", f"{c['file']}:{c['def_line'] + 1}"] for c in cands[:20]],
    'candidates': [[c['crate'], c['kind'], c['p'], f"{c['file']}:{c['def_line'] + 1}"] for c in cands],
    'unused_deps': None,
    'recipe': (
        "zero-ref rows = canonical audit-pass SQL without LIMIT (symbol LEFT JOIN refs, refs.callee NULL; rows of `symbol`, not distinct strings). "
        "1) exclude test: '/'||stripped-symbol-path OR '/'||file LIKE '%/tests/%' (stripped = regexp_replace(symbol,'^rust-analyzer cargo \\S+ \\S+ ','')). "
        "2) trait-impl: stripped path matches 'impl#\\[[^\\]]*\\]\\[[^\\]]*\\]'. "
        "3) residual classed first-match: entry-point (fn main() at crate root | module crate/) · test-only (path segment test|tests|*_test|*_tests, or #[test]/#[cfg(test)] within the 4 lines above the def) · "
        "derive/attr-invoked (__bitflags_flag_names/ | Props# | StoreImplExt#) · runtime-invoked (>1 residual symbol on one def line = macro-generated binding) · "
        "format-capture ({name} or {name: in the def file) · else candidate. Candidates are 'candidates', never 'dead'."
    ),
}
dump('dead', dead)
print(json.dumps({'dup': [dup['pct'], dup['clones'], dup['total_lines']], 'cx': [cx['functions'], cx['over_ceiling'], cx['max']],
                  'sizes': [sizes['totals'], sizes['file_max'], sizes['over_800'], missing], 'graph': [graph['cycles'], graph['cross_unit_edges']],
                  'dead': [zero_ref, test_excl, dead['fp_classes'], len(cands)]}, indent=0))
