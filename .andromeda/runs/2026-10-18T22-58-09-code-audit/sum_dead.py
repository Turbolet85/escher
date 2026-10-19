"""Dead code (A5) summarizer — the baseline record's `recipes.dead`, applied verbatim.

usage: sum_dead.py RUN_DIR MACHETE_OUT
Input: RUN_DIR/zero-ref.json (graph_queries.py: canonical zero-ref SQL without LIMIT, rows of `symbol`).
1) exclude test: '/'+stripped-symbol-path OR '/'+file contains '/tests/'
   (stripped = symbol with '^rust-analyzer cargo \\S+ \\S+ ' removed).
2) trait-impl: stripped path matches 'impl#\\[[^\\]]*\\]\\[[^\\]]*\\]'.
3) residual classed first-match: entry-point (fn main() at crate root | module crate/) · test-only
   (path segment test|tests|*_test|*_tests, or #[test]/#[cfg(test)] within the 4 lines above the def) ·
   derive/attr-invoked (__bitflags_flag_names/ | Props# | StoreImplExt#) · runtime-invoked (>1 residual
   symbol on one def line = macro-generated binding) · format-capture ({name} or {name: in the def file) ·
   else candidate.
def_line is 0-based in the DB, printed +1. Candidates are 'candidates', never 'dead'.
"""
import collections, json, re, sys

rd, machete = sys.argv[1:3]
rows = json.load(open(f'{rd}/zero-ref.json', encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT_IMPL = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
_src = {}


def src(f):
    if f not in _src:
        try:
            _src[f] = open(f, encoding='utf-8').read()
        except OSError:
            _src[f] = ''
    return _src[f]


counts = collections.Counter()
residual = []
for r in rows:
    st = PFX.sub('', r['symbol'])
    if '/tests/' in '/' + st or '/tests/' in '/' + (r['file'] or ''):
        counts['test (excluded)'] += 1
        continue
    if TRAIT_IMPL.search(st):
        counts['trait-impl'] += 1
        continue
    residual.append((r, st))

per_line = collections.Counter((r['file'], r['def_line']) for r, _ in residual)
cands = []
for r, st in residual:
    segs = [s for s in st.split('/') if s]
    lines = src(r['file']).split('\n')
    dl = r['def_line']
    above = '\n'.join(lines[max(0, dl - 4):dl])
    if st == 'main().' or st == 'crate/':
        k = 'entry-point'
    elif any(s in ('test', 'tests') or s.endswith('_test') or s.endswith('_tests') for s in segs) \
            or '#[test]' in above or '#[cfg(test)]' in above:
        k = 'test-only'
    elif '__bitflags_flag_names/' in st or 'Props#' in st or 'StoreImplExt#' in st:
        k = 'derive/attr-invoked'
    elif per_line[(r['file'], dl)] > 1:
        k = 'runtime-invoked'
    elif '{' + r['name'] + '}' in src(r['file']) or '{' + r['name'] + ':' in src(r['file']):
        k = 'format-capture'
    else:
        k = 'candidate'
    counts[k] += 1
    if k == 'candidate':
        cands.append((r['file'], dl, r['crate'] + ' ' + st))

cands.sort()
unused = []
cur = None
for line in open(machete, encoding='utf-8'):
    m = re.match(r'^(\S+) -- \./.*Cargo\.toml:$', line.rstrip())
    if m:
        cur = m.group(1)
    elif line.startswith('\t') and cur:
        unused.append([cur, line.strip()])
    elif not line.strip():
        cur = None
res = {'unused_deps': unused, 'zero_ref_candidates': len(cands),
       'top': [[s, f'{f}:{dl + 1}'] for f, dl, s in cands[:10]],
       'zero_ref_rows': len(rows), 'classes': dict(sorted(counts.items())),
       'candidates_all': [[s, f'{f}:{dl + 1}'] for f, dl, s in cands]}
json.dump(res, open(f'{rd}/c-dead.json', 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in res.items() if k not in ('top', 'candidates_all', 'unused_deps')}),
      'unused_deps', len(unused))
