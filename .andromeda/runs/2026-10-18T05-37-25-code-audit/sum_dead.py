"""A5 dead-code summarizer: the baseline record's `recipes.dead`, applied verbatim.

usage: sum_dead.py {run_dir} {machete.out}
Reads {run_dir}/g-zero_ref.json (rows of the canonical zero-ref SQL, no LIMIT).
1) test excluded: '/'+stripped symbol path OR '/'+file contains '/tests/'
   (stripped = symbol less '^rust-analyzer cargo \\S+ \\S+ ').
2) trait-impl: stripped path matches impl#\\[[^\\]]*\\]\\[[^\\]]*\\].
3) residual, first match: entry-point (stripped == 'main().' | stripped == 'crate/')
   · test-only (a path segment of the stripped symbol path or the file path is
   test|tests|*_test|*_tests, or #[test]/#[cfg(test)] in the 4 lines above the def)
   · derive/attr-invoked (__bitflags_flag_names/ | Props# | StoreImplExt#)
   · runtime-invoked (>1 residual symbol on one def line) · format-capture
   ({name} or {name: in the def file) · else candidate.
def_line is 0-based in the DB, printed +1.
"""
import collections
import json
import os
import re
import sys

rd, machete = sys.argv[1:3]
rows = json.load(open(os.path.join(rd, 'g-zero_ref.json'), encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
TESTSEG = re.compile(r'^(test|tests|.*_test|.*_tests)$')
_src = {}


def text(f):
    if f not in _src:
        try:
            _src[f] = open(f, encoding='utf-8').read()
        except OSError:
            _src[f] = ''
    return _src[f]


def segs(p):
    out = []
    for s in p.split('/'):
        s = s.split('#')[0]
        s = re.sub(r'\.rs$', '', s)
        out.append(s)
    return out


cls = collections.Counter()
residual = []
for r in rows:
    st = PFX.sub('', r['symbol'])
    if '/tests/' in '/' + st or '/tests/' in '/' + (r['file'] or ''):
        cls['excluded_test_path'] += 1
        continue
    if TRAIT.search(st):
        cls['trait-impl (dispatch)'] += 1
        continue
    residual.append((r, st))

per_line = collections.Counter((r['file'], r['def_line']) for r, _ in residual)
cands = []
for r, st in residual:
    f, dl = r['file'] or '', r['def_line']
    lines = text(f).splitlines()
    above = lines[max(0, dl - 4):dl] if dl is not None else []
    name = r['name'] or ''
    if st in ('main().', 'crate/'):
        c = 'entry-point'
    elif any(TESTSEG.match(s) for s in segs(st) + segs(f)) or any(
            '#[test]' in l or '#[cfg(test)]' in l for l in above):
        c = 'test-only'
    elif '__bitflags_flag_names/' in st or 'Props#' in st or 'StoreImplExt#' in st:
        c = 'derive/attr-invoked'
    elif per_line[(f, dl)] > 1:
        c = 'runtime-invoked (macro-generated binding)'
    elif name and ('{' + name + '}' in text(f) or '{' + name + ':' in text(f)):
        c = 'format-capture'
    else:
        c = 'candidate'
        cands.append([f"{r['crate']} {st}", f"{f}:{dl + 1}"])
    cls[c] += 1

deps, cur = [], None
for l in open(machete, encoding='utf-8'):
    m = re.match(r'^(\S+) -- \S+Cargo\.toml:$', l.rstrip())
    if m:
        cur = m.group(1)
    elif l.startswith('\t') and cur:
        deps.append([cur, l.strip()])
    elif not l.strip():
        cur = None
cands.sort(key=lambda c: (c[1].rsplit(':', 1)[0], int(c[1].rsplit(':', 1)[1]), c[0]))
ex = cls.pop('excluded_test_path', 0)
res = {
    'unused_deps': deps, 'zero_ref_candidates': len(cands), 'top': cands[:10],
    'fp_classes': dict(sorted({**cls, 'candidate': len(cands)}.items())),
    'zero_ref_rows': len(rows), 'excluded_test_path': ex, 'candidates_full': cands,
}
json.dump(res, open(os.path.join(rd, 'c-dead.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: res[k] for k in ('zero_ref_candidates', 'fp_classes', 'zero_ref_rows', 'excluded_test_path')}), len(deps))
