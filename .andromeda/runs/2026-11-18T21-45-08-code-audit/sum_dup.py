"""Duplication (A1) summarizer. usage: python3 sum_dup.py {jscpd-report.json} {repo_root} {out.json}
pct = statistics.total.percentage rounded to 2; top = 10 largest clones (lines desc, ties in report order),
cells {file}:{start line} (repo-relative); split by a `tests` directory segment: both src, both test, or mixed.
"""
import json, os, sys

rep, root, out = sys.argv[1:4]
root = os.path.realpath(root)
j = json.load(open(rep, encoding='utf-8'))
t = j['statistics']['total']


def rel(p):
    p = os.path.realpath(p)
    return os.path.relpath(p, root).replace('\\', '/')


def is_test(p):
    return 'tests' in p.split('/')[:-1]


rows, split = [], {'src': {'pairs': 0, 'lines': 0}, 'test': {'pairs': 0, 'lines': 0}, 'mixed': {'pairs': 0, 'lines': 0}}
for d in j['duplicates']:
    a, b = rel(d['firstFile']['name']), rel(d['secondFile']['name'])
    n = d['lines']
    rows.append([f"{a}:{d['firstFile']['start']}", f"{b}:{d['secondFile']['start']}", n])
    k = 'test' if is_test(a) and is_test(b) else 'src' if not is_test(a) and not is_test(b) else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += n
top = sorted(enumerate(rows), key=lambda e: (-e[1][2], e[0]))[:10]
o = {'pct': round(t['percentage'], 2), 'duplicated_lines': t['duplicatedLines'], 'total_lines': t['lines'], 'clones': t['clones'],
     'top': [r for _, r in top], 'split': split}
json.dump(o, open(out, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in o.items() if k != 'top'}))
