"""Duplication (A1) summarizer.

usage: sum_dup.py JSCPD_REPORT REPO_ROOT OUT [BASELINE_TOP_JSON]
statistics.total -> pct (2 dp) / duplicated_lines / total_lines / clones.
top = 10 largest fragments by `lines`, ties in report order (stable sort).
split: a file with a `tests` directory segment is test; a pair of two test files is test,
of none is src, else mixed.
"""
import json
import os
import sys

report, root, out = sys.argv[1:4]
base_top = json.loads(sys.argv[4]) if len(sys.argv) > 4 else None
root = os.path.abspath(root).replace('\\', '/').rstrip('/') + '/'

d = json.load(open(report, encoding='utf-8'))
tot = d['statistics']['total']


def rel(p):
    p = p.replace('\\', '/')
    return p[len(root):] if p.startswith(root) else p


def is_test(p):
    return 'tests' in p.split('/')[:-1]


rows = []
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for x in d['duplicates']:
    a, b = rel(x['firstFile']['name']), rel(x['secondFile']['name'])
    n = x['lines']
    rows.append([a, b, n, x['firstFile']['start'], x['secondFile']['start']])
    t = is_test(a) + is_test(b)
    k = 'test' if t == 2 else 'src' if t == 0 else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += n

top = sorted(rows, key=lambda r: -r[2])[:10]
summary = {
    'pct': round(tot['percentage'], 2),
    'duplicated_lines': tot['duplicatedLines'],
    'total_lines': tot['lines'],
    'clones': tot['clones'],
    'top': [r[:3] for r in top],
    'split': split,
}
extra = {'top_with_starts': top}
if base_top is not None:
    bset = {tuple(r) for r in base_top}
    extra['new_in_top'] = [r[:3] for r in top if tuple(r[:3]) not in bset]
json.dump({'duplication': summary, **extra}, open(out, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in summary.items() if k != 'top'}))
