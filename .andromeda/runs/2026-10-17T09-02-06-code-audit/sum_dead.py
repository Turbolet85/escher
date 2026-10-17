"""Dead code (A5) summarizer: the baseline record's recipes.dead applied verbatim.

usage: sum_dead.py RUN_DIR MACHETE_OUT
Reads RUN_DIR/zero-ref.raw.json (symbol, crate, name, kind, file, def_line rows), writes RUN_DIR/c-dead.json.
"""
import collections, json, os, re, sys

rd, machete_p = sys.argv[1:3]
rows = json.load(open(f'{rd}/zero-ref.raw.json', encoding='utf-8'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')
TESTSEG = re.compile(r'^(test|tests|.*_test|.*_tests)$')


def strip(sym):
    return PFX.sub('', sym)


src_cache = {}


def lines(f):
    if f not in src_cache:
        try:
            src_cache[f] = open(f, encoding='utf-8').read().splitlines()
        except OSError:
            src_cache[f] = []
    return src_cache[f]


cls = collections.Counter()
excluded = 0
residual = []
for r in rows:
    sp = strip(r['symbol'])
    f = r['file'] or ''
    if '/tests/' in '/' + sp or '/tests/' in '/' + f:
        excluded += 1
        continue
    if TRAIT.search(sp):
        cls['trait-impl (dispatch)'] += 1
        continue
    residual.append(r)


def segs(r):
    sp = strip(r['symbol'])
    return [s for s in re.split(r'[/#]', sp) if s]


def attr_test_above(r):
    ls, n = lines(r['file'] or ''), r['def_line'] or 0  # def_line is 0-based
    above = ls[max(0, n - 4):n]
    return any('#[test]' in l or '#[cfg(test)]' in l for l in above)


line_count = collections.Counter((r['file'], r['def_line']) for r in residual)
cands = []
for r in residual:
    sp = strip(r['symbol'])
    f = r['file'] or ''
    if sp == 'main().' or sp == 'crate/':
        c = 'entry-point'
    elif any(TESTSEG.match(s) for s in segs(r)) or any(TESTSEG.match(os.path.splitext(s)[0]) for s in f.split('/')) or attr_test_above(r):
        c = 'test-only'
    elif sp.startswith('__bitflags_flag_names/') or '/__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
        c = 'derive/attr-invoked'
    elif line_count[(r['file'], r['def_line'])] > 1:
        c = 'runtime-invoked (macro-generated binding)'
    elif r['name'] and any(('{' + r['name'] + '}') in l or ('{' + r['name'] + ':') in l for l in lines(f)):
        c = 'format-capture'
    else:
        c = 'candidate'
        cands.append(r)
    cls[c] += 1


def short(sym):  # "{crate} {path}" — the baseline record's printed form
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', sym)
    return f'{m.group(1)} {m.group(2)}' if m else sym


cands.sort(key=lambda r: (r['file'] or '', r['def_line'] or 0, r['symbol']))
json.dump({
    'zero_ref_candidates': cls['candidate'],
    'top': [[short(r['symbol']), f"{r['file']}:{(r['def_line'] or 0) + 1}"] for r in cands[:10]],
    'fp_classes': dict(sorted(cls.items())),
    'zero_ref_rows': len(rows), 'excluded_test_path': excluded,
    'all_candidates': [[short(r['symbol']), f"{r['file']}:{(r['def_line'] or 0) + 1}"] for r in cands],
}, open(f'{rd}/c-dead.json', 'w', encoding='utf-8'), indent=1)
print('rows', len(rows), 'excluded', excluded, dict(cls))
