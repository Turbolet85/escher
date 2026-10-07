"""A5 zero-ref classing — the ledger's `recipes.dead`, applied verbatim.

Usage: dead_class.py <zero-ref rows json> <out json> [touched-files.txt]

Input rows: the canonical audit-pass zero-ref SQL without LIMIT (symbol, crate, name, kind, file, def_line; def_line 0-based).
1) exclude test: '/'+stripped symbol path OR '/'+file holds '/tests/' (stripped = the SCIP prefix removed).
2) trait-impl: stripped path matches impl#[..][..].
3) residual, first match: entry-point · test-only · derive/attr-invoked · runtime-invoked · format-capture · candidate.
Candidates are candidates, never dead.
"""
import collections
import json
import re
import sys

rows = json.load(open(sys.argv[1], encoding='utf-8'))
touched = set(open(sys.argv[3], encoding='utf-8').read().split()) if len(sys.argv) > 3 else set()
PREFIX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT_IMPL = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
_src = {}


def src(path):
    if path not in _src:
        try:
            _src[path] = open(path, encoding='utf-8').read()
        except OSError:
            _src[path] = ''
    return _src[path]


def test_segment(stripped):
    for seg in stripped.split('/')[:-1]:
        if seg in ('test', 'tests') or seg.endswith('_test') or seg.endswith('_tests'):
            return True
    return False


def test_attr(path, def_line):
    lines = src(path).split('\n')
    above = lines[max(0, def_line - 4):def_line]
    return any('#[test]' in l or '#[cfg(test)]' in l for l in above)


classes = collections.OrderedDict()
excluded_test, residual = 0, []
for r in rows:
    r['stripped'] = PREFIX.sub('', r['symbol'])
    if '/tests/' in '/' + r['stripped'] or '/tests/' in '/' + r['file']:
        excluded_test += 1
    elif TRAIT_IMPL.search(r['stripped']):
        classes.setdefault('trait-impl (dispatch)', []).append(r)
    else:
        residual.append(r)

per_line = collections.Counter((r['file'], r['def_line']) for r in residual)
for r in residual:
    s = r['stripped']
    if (r['name'] == 'main' and s == 'main().') or s == 'crate/':
        c = 'entry-point'
    elif test_segment(s) or test_attr(r['file'], r['def_line']):
        c = 'test-only'
    elif '__bitflags_flag_names/' in s or 'Props#' in s or 'StoreImplExt#' in s:
        c = 'derive/attr-invoked'
    elif per_line[(r['file'], r['def_line'])] > 1:
        c = 'runtime-invoked (macro-generated binding)'
    elif '{' + r['name'] + '}' in src(r['file']) or '{' + r['name'] + ':' in src(r['file']):
        c = 'format-capture'
    else:
        c = 'candidate'
    classes.setdefault(c, []).append(r)

cands = sorted(classes.get('candidate', []), key=lambda r: (r['file'], r['def_line'], r['symbol']))
fmt = lambda r: ['%s %s' % (r['crate'], r['stripped']), '%s:%d' % (r['file'], r['def_line'] + 1)]
out = {
    'zero_ref_rows': len(rows), 'excluded_test_path': excluded_test,
    'fp_classes': {k: len(v) for k, v in classes.items()},
    'zero_ref_candidates': len(cands),
    'top': [fmt(r) for r in cands[:10]],
    'top20': [fmt(r) for r in cands[:20]],
    'candidates_in_touched_files': [fmt(r) for r in cands if r['file'] in touched],
    'candidates_by_crate': sorted(collections.Counter(r['crate'] for r in cands).items(), key=lambda kv: -kv[1]),
    'fp_classes_untouched_files': {k: sum(1 for r in v if r['file'] not in touched) for k, v in classes.items()},
    'fp_classes_touched_files': {k: sum(1 for r in v if r['file'] in touched) for k, v in classes.items()},
    'note': 'counts are candidates, never dead; FP classes named: entry points, runtime-invoked surfaces, test-only '
            'helpers, trait-impl methods reached by dispatch, derive/attr-invoked fns, format-string captures',
}
assert excluded_test + sum(len(v) for v in classes.values()) == len(rows)
json.dump(out, open(sys.argv[2], 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(json.dumps({k: out[k] for k in ('zero_ref_rows', 'excluded_test_path', 'fp_classes', 'zero_ref_candidates',
                                      'fp_classes_untouched_files', 'fp_classes_touched_files')}, indent=1))
for row in out['candidates_in_touched_files']:
    print('  touched-file candidate', row)
print(out['top'])
