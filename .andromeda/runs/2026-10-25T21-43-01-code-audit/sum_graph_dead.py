"""Graph (A4) + dead code (A5) summarizer.

usage: sum_graph_dead.py RUN_DIR MACHETE_OUT ROOT
Dead classing: the baseline record's `recipes.dead`, verbatim:
 zero-ref rows = canonical audit-pass SQL without LIMIT (symbol LEFT JOIN refs, refs.callee NULL; rows of
 `symbol`, not distinct strings). 1) exclude test: '/'||stripped-symbol-path OR '/'||file LIKE '%/tests/%'
 (stripped = regexp_replace(symbol,'^rust-analyzer cargo \\S+ \\S+ ','')). 2) trait-impl: stripped path matches
 'impl#\\[[^\\]]*\\]\\[[^\\]]*\\]'. 3) residual classed first-match: entry-point (fn main() at crate root |
 module crate/) · test-only (path segment test|tests|*_test|*_tests, or #[test]/#[cfg(test)] within the 4 lines
 above the def) · derive/attr-invoked (__bitflags_flag_names/ | Props# | StoreImplExt#) · runtime-invoked (>1
 residual symbol on one def line = macro-generated binding) · format-capture ({name} or {name: in the def file)
 · else candidate.
def_line is 0-based in the DB, printed +1.
"""
import json, os, re, sys
from collections import Counter

rd, machete, root = sys.argv[1:4]
L = lambda n: json.load(open(f'{rd}/q-{n}.json', encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')


def short(sym):
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', sym)
    return f'{m.group(1)} {m.group(2)}' if m else sym


cycles = L('cycles')
fan_in = [[short(r['callee']), r['n']] for r in L('fan_in')]
fan_out = [[r['from_crate'], r['n']] for r in L('fan_out')]
edges = list(L('edges')[0].values())[0]

rows = L('zero_ref')
srcs = {}


def src(f):
    if f not in srcs:
        try:
            srcs[f] = open(os.path.join(root, f), encoding='utf-8').read().splitlines()
        except OSError:
            srcs[f] = []
    return srcs[f]


cls = Counter()
stage1 = []
for r in rows:
    sp = PFX.sub('', r['symbol'])
    if '/tests/' in '/' + sp or '/tests/' in '/' + (r['file'] or ''):
        cls['test-path'] += 1
        continue
    if re.search(r'impl#\[[^\]]*\]\[[^\]]*\]', sp):
        cls['trait-impl'] += 1
        continue
    stage1.append((r, sp))
perline = Counter((r['file'], r['def_line']) for r, _ in stage1)
cands = []
for r, sp in stage1:
    f, dl, name = r['file'] or '', r['def_line'], r['name'] or ''
    lines = src(f)
    segs = sp.split('/')[:-1] + [x for x in f.split('/')[:-1]]
    above = lines[max(0, dl - 4):dl] if lines else []
    if sp in ('main().', 'crate/'):
        cls['entry-point'] += 1
    elif any(s in ('test', 'tests') or s.endswith('_test') or s.endswith('_tests') for s in segs) or \
            any('#[test]' in a or '#[cfg(test)]' in a for a in above):
        cls['test-only'] += 1
    elif '__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
        cls['derive/attr-invoked'] += 1
    elif perline[(r['file'], r['def_line'])] > 1:
        cls['runtime-invoked'] += 1
    elif name and any(('{' + name + '}') in l or ('{' + name + ':') in l for l in lines):
        cls['format-capture'] += 1
    else:
        cands.append((short(r['symbol']), f'{f}:{dl + 1}'))
cands.sort(key=lambda x: (x[1].rsplit(':', 1)[0], int(x[1].rsplit(':', 1)[1]), x[0]))

unused = []
cur = None
for line in open(machete, encoding='utf-8'):
    m = re.match(r'^(\S+) -- (\S+):$', line.strip())
    if m:
        cur = m.group(1)
        continue
    s = line.strip()
    if cur and s and re.fullmatch(r'[A-Za-z0-9_\-]+', s):
        unused.append([cur, s])
json.dump({'graph': {'cycles': len(cycles), 'cycle_paths': [c['path'] for c in cycles], 'fan_in_top': fan_in,
                     'fan_out': fan_out, 'cross_unit_edges': edges}},
          open(f'{rd}/c-graph.json', 'w', encoding='utf-8'), indent=1)
json.dump({'dead': {'unused_deps': unused, 'zero_ref_candidates': len(cands), 'top': [list(c) for c in cands[:10]]},
           'zero_ref_rows': len(rows), 'classes': dict(cls), 'candidates_full': [list(c) for c in cands],
           'fp_classes_named': ['entry points', 'runtime-invoked surfaces', 'test-only helpers',
                                'trait-impl methods reached by dispatch', 'derive/attr-invoked fns',
                                'inline format-string captures']},
          open(f'{rd}/c-dead.json', 'w', encoding='utf-8'), indent=1)
print('cycles', len(cycles), 'edges', edges, 'zero-ref rows', len(rows), dict(cls), 'candidates', len(cands),
      'unused deps', len(unused))
