"""A1 duplication summarizer (collectors.md pinned shape).

usage: sum_dup.py {jscpd-report.json} {repo_root} {out.json}
Paths made repo-relative against repo_root. pct rounded to 3 places.
Split: a file under a `tests` directory segment is test; a pair of two is
test, of none is src, else mixed. top = 10 longest fragments
[fileA, fileB, lines], ties by path.
"""
import json
import os
import sys

rep, root, out = sys.argv[1:4]
root = os.path.abspath(root)
d = json.load(open(rep, encoding='utf-8'))
t = d['statistics']['total']


def rel(p):
    return os.path.relpath(p, root) if os.path.isabs(p) else p


def is_test(p):
    return 'tests' in p.split('/')[:-1]


rows = []
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for c in d['duplicates']:
    a, b = rel(c['firstFile']['name']), rel(c['secondFile']['name'])
    n = int(c['lines'])
    rows.append([a, b, n])
    k = sum((is_test(a), is_test(b)))
    key = 'test' if k == 2 else 'src' if k == 0 else 'mixed'
    split[key]['pairs'] += 1
    split[key]['lines'] += n
top = sorted(rows, key=lambda r: (-r[2], r[0], r[1]))[:10]
res = {
    'pct': round(t['percentage'], 3), 'duplicated_lines': t['duplicatedLines'],
    'total_lines': t['lines'], 'clones': t['clones'], 'top': top, 'split': split,
    'sources': t['sources'],
}
json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: res[k] for k in res if k != 'top'}))
