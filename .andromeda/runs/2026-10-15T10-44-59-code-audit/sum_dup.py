"""Duplication (A1) summarizer.

usage: python3 sum_dup.py {jscpd-report.json} {root_abs} {out.json|-}
Paths made repo-relative against root_abs. Split: a file under a `tests` directory segment
is test; a clone whose two files are both test is test, neither is src, else mixed.
top = the 10 largest clone fragments [a, b, lines], sorted (-lines, a, b).
"""
import json
import os
import sys


def is_test(p):
    return 'tests' in p.split('/')[:-1]


def main():
    rep, root, out = sys.argv[1:4]
    d = json.load(open(rep, encoding='utf-8'))
    t = d['statistics']['total']
    rows = []
    split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
    for c in d['duplicates']:
        a = os.path.relpath(c['firstFile']['name'], root)
        b = os.path.relpath(c['secondFile']['name'], root)
        n = c['lines']
        rows.append([a, b, n])
        ta, tb = is_test(a), is_test(b)
        k = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
        split[k]['pairs'] += 1
        split[k]['lines'] += n
    top = sorted(rows, key=lambda r: (-r[2], r[0], r[1]))[:10]
    res = {'pct': round(t['percentage'], 3), 'duplicated_lines': t['duplicatedLines'],
           'total_lines': t['lines'], 'clones': t['clones'], 'top': top, 'split': split}
    if out == '-':
        print(json.dumps({k: v for k, v in res.items() if k != 'top'}))
        print(json.dumps(top))
    else:
        res['all_pairs'] = sorted(rows, key=lambda r: (-r[2], r[0], r[1]))
        json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
        print(json.dumps({k: v for k, v in res.items() if k not in ('top', 'all_pairs')}))


main()
