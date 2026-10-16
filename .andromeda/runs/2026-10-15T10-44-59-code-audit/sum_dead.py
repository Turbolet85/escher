"""Dead-code (A5) summarizer — the ledger's recipes.dead, applied verbatim.

usage: python3 sum_dead.py {q-zero.json} {machete.txt} {repo_root} {out.json}
"""
import json
import os
import re
import sys
from collections import Counter, defaultdict

PREFIX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
TRAIT_IMPL = re.compile(r'impl#\[[^\]]*\]\[[^\]]*\]')


def stripped(sym):
    return PREFIX.sub('', sym)


def is_test_path(sym, f):
    return '/tests/' in '/' + stripped(sym) or '/tests/' in '/' + f


def test_seg(path):
    for seg in re.split(r'[/#]', path):
        if seg in ('test', 'tests') or seg.endswith('_test') or seg.endswith('_tests'):
            return True
    return False


_src = {}


def lines(root, f):
    if f not in _src:
        try:
            _src[f] = open(os.path.join(root, f), encoding='utf-8').read().split('\n')
        except OSError:
            _src[f] = []
    return _src[f]


def text(root, f):
    return '\n'.join(lines(root, f))


def machete(path):
    out, cur = [], None
    for raw in open(path, encoding='utf-8'):
        l = raw.rstrip('\n')
        m = re.match(r'^(\S+) -- (\S+):$', l)
        if m:
            cur = m.group(1)
            continue
        m = re.match(r'^\t(\S+)$', l)
        if m and cur:
            out.append([cur, m.group(1)])
    return out


def main():
    qz, mach, root, out = sys.argv[1:5]
    rows = json.load(open(qz, encoding='utf-8'))
    counts = Counter()
    resid = []
    for r in rows:
        if is_test_path(r['symbol'], r['file']):
            counts['excluded_test_path'] += 1
        elif TRAIT_IMPL.search(stripped(r['symbol'])):
            counts['trait-impl (dispatch)'] += 1
        else:
            resid.append(r)
    per_line = defaultdict(int)
    for r in resid:
        per_line[(r['file'], r['def_line'])] += 1
    cands = []
    for r in resid:
        sp = stripped(r['symbol'])
        L = lines(root, r['file'])
        above = L[max(0, r['def_line'] - 4):r['def_line']]
        if sp in ('main().', 'crate/'):
            c = 'entry-point'
        elif test_seg(sp) or test_seg(r['file']) or any(
                '#[test]' in a or '#[cfg(test)]' in a for a in above):
            c = 'test-only'
        elif '__bitflags_flag_names/' in sp or 'Props#' in sp or 'StoreImplExt#' in sp:
            c = 'derive/attr-invoked'
        elif per_line[(r['file'], r['def_line'])] > 1:
            c = 'runtime-invoked (macro-generated binding)'
        elif r['name'] and ('{%s}' % r['name'] in text(root, r['file'])
                            or '{%s:' % r['name'] in text(root, r['file'])):
            c = 'format-capture'
        else:
            c = 'candidate'
        counts[c] += 1
        if c == 'candidate':
            cands.append([PREFIX.sub(lambda m: m.group(0).split()[2] + ' ', r['symbol']).strip()
                          if False else ' '.join([r['symbol'].split()[2], sp]),
                          '%s:%d' % (r['file'], r['def_line'] + 1)])
    cands.sort(key=lambda x: (x[1].split(':')[0], int(x[1].split(':')[1]), x[0]))
    res = {'unused_deps': machete(mach), 'zero_ref_candidates': counts['candidate'],
           'top': cands[:20], 'fp_classes': dict(sorted(counts.items())),
           'zero_ref_rows': len(rows), 'excluded_test_path': counts['excluded_test_path'],
           'all_candidates': cands}
    json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
    print(json.dumps({k: v for k, v in res.items() if k not in ('top', 'all_candidates')}))


main()
