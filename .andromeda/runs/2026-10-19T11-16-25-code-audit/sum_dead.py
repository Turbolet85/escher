"""Dead code (A5) summarizer — recipes.dead, verbatim from the baseline record.

usage: sum_dead.py RUN_DIR MACHETE_OUT [REPO_ROOT]
Rows: RUN_DIR/zero-ref-rows.json (canonical zero-ref SQL without LIMIT, rows of `symbol`).
1) exclude test: '/'+stripped-symbol-path OR '/'+file contains '/tests/'
   (stripped = symbol less the '^rust-analyzer cargo \\S+ \\S+ ' prefix).
2) trait-impl: stripped matches 'impl#\\[[^\\]]*\\]\\[[^\\]]*\\]'.
3) residual classed first-match: entry-point (fn main() at crate root | module crate/) ·
   test-only (a stripped-path segment test|tests|*_test|*_tests, or #[test]/#[cfg(test)] within
   the 4 lines above the def) · derive/attr-invoked (__bitflags_flag_names/ | Props# |
   StoreImplExt#) · runtime-invoked (>1 residual symbol on one def line = macro-generated
   binding) · format-capture ({name} or {name: in the def file) · else candidate.
def_line is 0-based in the DB, printed +1.
"""
import collections
import json
import os
import re
import sys

rd, machete = sys.argv[1:3]
root = sys.argv[3] if len(sys.argv) > 3 else '.'
rows = json.load(open(os.path.join(rd, 'zero-ref-rows.json'), encoding='utf-8'))

PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')

_src = {}


def lines(f):
    if f not in _src:
        try:
            _src[f] = open(os.path.join(root, f), encoding='utf-8').read().split('\n')
        except OSError:
            _src[f] = []
    return _src[f]


def strip(sym):
    return PFX.sub('', sym)


classes = collections.Counter()
residual = []
for r in rows:
    sp = strip(r['symbol'])
    if '/tests/' in '/' + sp or '/tests/' in '/' + (r['file'] or ''):
        classes['test-excluded'] += 1
        continue
    if TRAIT.search(sp):
        classes['trait-impl'] += 1
        continue
    residual.append(r)

per_line = collections.Counter((r['file'], r['def_line']) for r in residual)


def seg_test(sp):
    for s in re.split(r'[/#]', sp):
        s = s.rstrip('.').rstrip('()')
        if s in ('test', 'tests') or s.endswith('_test') or s.endswith('_tests'):
            return True
    return False


cands = []
for r in residual:
    sp = strip(r['symbol'])
    src = lines(r['file'])
    dl = r['def_line']
    above = '\n'.join(src[max(0, dl - 4):dl])
    if sp == 'main().' or sp == 'crate/':
        classes['entry-point'] += 1
    elif seg_test(sp) or '#[test]' in above or '#[cfg(test)]' in above:
        classes['test-only'] += 1
    elif '__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
        classes['derive/attr-invoked'] += 1
    elif per_line[(r['file'], dl)] > 1:
        classes['runtime-invoked'] += 1
    elif r['name'] and ('{' + r['name'] + '}' in '\n'.join(src) or '{' + r['name'] + ':' in '\n'.join(src)):
        classes['format-capture'] += 1
    else:
        cands.append(r)

cands.sort(key=lambda r: (r['file'], r['def_line'], r['symbol']))


def show(r):
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', r['symbol'])
    name = f'{m.group(1)} {m.group(2)}' if m else r['symbol']
    return [name, f"{r['file']}:{r['def_line'] + 1}"]


deps = []
cur = None
for l in open(machete, encoding='utf-8'):
    m = re.match(r'^(\S+) -- \./.*Cargo\.toml:', l)
    if m:
        cur = m.group(1)
        continue
    if cur and l.startswith('\t') and l.strip():
        deps.append([cur, l.strip()])
    elif not l.strip():
        cur = None

dead = {'unused_deps': deps, 'zero_ref_candidates': len(cands), 'top': [show(r) for r in cands[:10]]}
json.dump({'dead': dead, 'zero_ref_rows': len(rows), 'classes': dict(classes),
           'candidates_full': [show(r) for r in cands],
           'note': 'candidates, never dead; FP classes: entry points · runtime-invoked · test-only · '
                   'trait-impl dispatch · derive/attr-invoked · format-string captures'},
          open(os.path.join(rd, 'c-dead.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({'rows': len(rows), 'classes': dict(classes), 'candidates': len(cands), 'deps': len(deps)}))
