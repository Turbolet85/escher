"""Duplication (A1) summarizer.

usage: sum_dup.py JSCPD_REPORT ROOT OUT
statistics.total → pct · duplicated_lines · total_lines · clones; top = 10 largest fragments
(ties in report order); split by a `tests` directory segment: src (neither file), test (both),
mixed (one); per class {pairs, lines}.
"""
import json, os, sys

rep, root, out = sys.argv[1:4]
root = os.path.abspath(root)
d = json.load(open(rep, encoding='utf-8'))
t = d['statistics']['total']


def rel(p):
    return os.path.relpath(os.path.abspath(p), root).replace(os.sep, '/')


def is_test(p):
    return 'tests' in p.split('/')[:-1]


dups = []
for x in d['duplicates']:
    a, b = rel(x['firstFile']['name']), rel(x['secondFile']['name'])
    dups.append({'a': a, 'b': b, 'lines': x['lines'],
                 'a_start': x['firstFile'].get('start'), 'b_start': x['secondFile'].get('start')})
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for x in dups:
    ta, tb = is_test(x['a']), is_test(x['b'])
    k = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += x['lines']
top = sorted(enumerate(dups), key=lambda e: (-e[1]['lines'], e[0]))[:10]
summary = {'pct': round(t['percentage'], 2), 'duplicated_lines': t['duplicatedLines'],
           'total_lines': t['lines'], 'clones': t['clones'],
           'top': [[x['a'], x['b'], x['lines']] for _, x in top], 'split': split}
json.dump({'duplication': summary,
           'top_detail': [x for _, x in top]}, open(out, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in summary.items() if k != 'top'}))
