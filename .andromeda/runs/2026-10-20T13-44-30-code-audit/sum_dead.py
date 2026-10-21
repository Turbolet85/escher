"""Dead-code summarizer: cargo-machete unused deps + zero-ref candidates by the baseline record's
recipes.dead, applied verbatim.

usage: sum_dead.py {run_dir} {machete.out}
input: {run_dir}/zero_ref_rows.json (rows of `symbol` with no `refs` row; def_line 0-based, printed +1)
"""
import json, re, sys
from collections import Counter

rd, machete_p = sys.argv[1:3]
rows = json.load(open(f'{rd}/zero_ref_rows.json', encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
TESTSEG = re.compile(r'^(test|tests|.*_test|.*_tests)$')
_src = {}


def src(f):
    if f not in _src:
        try:
            _src[f] = open(f, encoding='utf-8').read().split('\n')
        except OSError:
            _src[f] = []
    return _src[f]


classes = Counter()
resid = []
for r in rows:
    sp = PFX.sub('', r['symbol'])
    if '/tests/' in '/' + sp or '/tests/' in '/' + (r['file'] or ''):
        classes['test-excluded'] += 1
        continue
    if TRAIT.search(sp):
        classes['trait-impl'] += 1
        continue
    resid.append((r, sp))

pending = []
for r, sp in resid:
    if sp in ('main().', 'crate/'):
        classes['entry-point'] += 1
        continue
    segs = [s for s in sp.split('/')[:-1]] + [p.rsplit('.', 1)[0] for p in (r['file'] or '').split('/')]
    lines = src(r['file'])
    above = lines[max(0, r['def_line'] - 4):r['def_line']]
    if any(TESTSEG.match(s) for s in segs) or any('#[test]' in l or '#[cfg(test)]' in l for l in above):
        classes['test-only'] += 1
        continue
    if any(k in sp for k in ('__bitflags_flag_names/', 'Props#', 'StoreImplExt#')):
        classes['derive/attr-invoked'] += 1
        continue
    pending.append((r, sp))

per_line = Counter((r['file'], r['def_line']) for r, _ in pending)
cands = []
for r, sp in pending:
    if per_line[(r['file'], r['def_line'])] > 1:
        classes['runtime-invoked'] += 1
        continue
    text = '\n'.join(src(r['file']))
    n = r['name']
    if n and ('{' + n + '}' in text or '{' + n + ':' in text):
        classes['format-capture'] += 1
        continue
    cands.append((r, sp))

cands.sort(key=lambda x: (x[0]['file'], x[0]['def_line'], x[0]['symbol']))
unused = []
cur = None
for line in open(machete_p, encoding='utf-8'):
    line = line.rstrip('\n')
    m = re.match(r'^(\S+) -- (\S+):$', line)
    if m:
        cur = m.group(1)
        continue
    m = re.match(r'^\t(\S+)$', line)
    if m and cur:
        unused.append([cur, m.group(1)])
out = {
    'unused_deps': unused,
    'zero_ref_candidates': len(cands),
    'top': [[f"{x[0]['crate']} {x[1]}", f"{x[0]['file']}:{x[0]['def_line'] + 1}"] for x in cands[:10]],
    'zero_ref_rows': len(rows),
    'classes': dict(classes),
    'fp_classes_named': ['entry points', 'runtime-invoked surfaces', 'test-only helpers',
                         'trait-impl methods reached by dispatch', 'derive/attr-invoked fns',
                         'format-string captures'],
    'all_candidates': [[f"{x[0]['crate']} {x[1]}", f"{x[0]['file']}:{x[0]['def_line'] + 1}"] for x in cands],
}
json.dump(out, open(f'{rd}/c-dead.json', 'w', encoding='utf-8'), indent=1)
print({k: v for k, v in out.items() if k not in ('top', 'all_candidates', 'unused_deps', 'fp_classes_named')},
      'unused_deps', len(unused))
