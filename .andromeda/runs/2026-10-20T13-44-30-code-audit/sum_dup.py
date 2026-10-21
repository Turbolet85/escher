"""Duplication summarizer (jscpd JSON report).

usage: sum_dup.py {jscpd-report.json} {repo_root} {out.json}
pct · duplicated_lines · total_lines · clones from statistics.total; top = 10 largest fragments
(ties in report order) as [firstFile, secondFile, lines]; split by a `tests` directory segment:
test = both files under one, src = neither, mixed = one.
"""
import json, os, sys

rep_p, root, out = sys.argv[1:4]
root = os.path.abspath(root)
d = json.load(open(rep_p, encoding='utf-8'))
t = d['statistics']['total']


def rel(p):
    return os.path.relpath(p, root) if os.path.isabs(p) else p


def is_test(p):
    return 'tests' in p.split('/')[:-1]


rows = []
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for c in d['duplicates']:
    a, b = rel(c['firstFile']['name']), rel(c['secondFile']['name'])
    rows.append([a, b, c['lines']])
    ta, tb = is_test(a), is_test(b)
    k = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += c['lines']
top = sorted(rows, key=lambda r: -r[2])[:10]
s = {
    'pct': t['percentage'], 'duplicated_lines': t['duplicatedLines'], 'total_lines': t['lines'],
    'clones': t['clones'], 'top': top, 'split': split,
}
json.dump(s, open(out, 'w', encoding='utf-8'), indent=1)
print({k: v for k, v in s.items() if k != 'top'})
