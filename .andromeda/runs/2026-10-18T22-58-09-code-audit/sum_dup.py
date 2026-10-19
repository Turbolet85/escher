"""Duplication (A1) summarizer — collectors.md pinned shape.

usage: sum_dup.py JSCPD_REPORT REPO_ROOT OUT_JSON
Paths made repo-relative. top = the 10 largest clone fragments as [fileA, fileB, lines]
(ordered by lines desc, then the report's own order). split: a file with a `tests` directory
segment is test; a pair of two test files is test, of none src, else mixed.
"""
import json, os, sys

rep_p, root, out = sys.argv[1:4]
root = os.path.abspath(root)
d = json.load(open(rep_p, encoding='utf-8'))
t = d['statistics']['total']


def rel(p):
    return os.path.relpath(p, root).replace('\\', '/')


def is_test(p):
    return 'tests' in p.split('/')[:-1]


rows = []
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for i, c in enumerate(d['duplicates']):
    a, b = rel(c['firstFile']['name']), rel(c['secondFile']['name'])
    n = c['lines']
    rows.append((a, b, n, i))
    k = 'test' if is_test(a) and is_test(b) else 'src' if not is_test(a) and not is_test(b) else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += n
top = [[a, b, n] for a, b, n, _ in sorted(rows, key=lambda r: (-r[2], r[3]))[:10]]
res = {'pct': round(t['percentage'], 3), 'duplicated_lines': t['duplicatedLines'],
       'total_lines': t['lines'], 'clones': t['clones'], 'top': top, 'split': split}
json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
print(json.dumps(res))
