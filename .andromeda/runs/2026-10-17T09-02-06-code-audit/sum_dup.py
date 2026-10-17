"""Duplication (A1) summarizer, per collectors.md's pinned row.

usage: sum_dup.py JSCPD_REPORT_JSON ROOT OUT_JSON
Paths are made ROOT-relative. Split: a file with a `tests` directory segment is test; a pair of two
test files is test, of none is src, else mixed.
"""
import json, os, sys

rep_p, root, out_p = sys.argv[1:4]
root = os.path.abspath(root) + os.sep
rep = json.load(open(rep_p, encoding='utf-8'))


def rel(p):
    p = os.path.abspath(p)
    return p[len(root):] if p.startswith(root) else p


def is_test(p):
    return 'tests' in p.split('/')[:-1]


tot = rep['statistics']['total']
rows, split = [], {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for d in rep['duplicates']:
    a, b = rel(d['firstFile']['name']), rel(d['secondFile']['name'])
    n = d['lines']
    rows.append((a, b, n, d['firstFile'].get('start'), d['secondFile'].get('start')))
    t = is_test(a) + is_test(b)
    k = 'test' if t == 2 else 'src' if t == 0 else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += n
rows.sort(key=lambda r: (-r[2], r[0], r[1], r[3] or 0, r[4] or 0))
json.dump({
    'pct': round(tot['percentage'], 3), 'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'],
    'clones': tot['clones'], 'top': [[r[0], r[1], r[2]] for r in rows[:10]], 'split': split,
    'top_with_starts': [list(r) for r in rows[:10]],
}, open(out_p, 'w', encoding='utf-8'), indent=1)
print('pct', round(tot['percentage'], 3), 'clones', tot['clones'], 'dup', tot['duplicatedLines'], 'lines', tot['lines'], split)
