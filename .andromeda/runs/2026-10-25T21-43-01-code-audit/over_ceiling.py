"""Evidence for the monotonic complexity.over_ceiling finding: the functions over cognitive 15 at each of the
three records' shas, matched by (file, fn name, occurrence) — a function's line moves with edits; a file move
(the upstream sync's layout/inline.rs -> layout/inline/layout.rs) is mapped by the rename list given.

usage: over_ceiling.py OUT RENAMES_JSON LABEL=ROOT:RCA_DIR:SRCFILES ...
"""
import json, os, sys
from collections import Counter

out, renames = sys.argv[1], json.loads(sys.argv[2])
sets = {}
for arg in sys.argv[3:]:
    label, rest = arg.split('=', 1)
    root, rca, srcf = rest.split(':')
    root = os.path.abspath(root)
    fset = {l.strip() for l in open(srcf, encoding='utf-8') if l.strip()}
    rows = []

    def walk(sp, f):
        if sp.get('kind') == 'function':
            rows.append((f, sp.get('name'), sp.get('start_line'), sp['metrics']['cognitive']['sum']))
        for c in sp.get('spaces') or []:
            walk(c, f)

    seen = set()
    for dp, _, fs in os.walk(rca):
        for n in fs:
            if n.endswith('.json'):
                d = json.load(open(os.path.join(dp, n), encoding='utf-8'))
                p = d['name'] if os.path.isabs(d['name']) else os.path.join(root, d['name'])
                f = os.path.relpath(os.path.abspath(p), root).replace(os.sep, '/')
                if f in fset and f not in seen:
                    seen.add(f)
                    walk(d, f)
    occ = Counter()
    keyed = {}
    for f, fn, line, v in sorted(rows, key=lambda r: (r[0], r[2])):
        f2 = renames.get(f, f)
        occ[(f2, fn)] += 1
        keyed[(f2, fn, occ[(f2, fn)])] = (line, v, f)
    sets[label] = keyed
labels = list(sets)
res = {}
for a, b in zip(labels, labels[1:]):
    A = {k for k, v in sets[a].items() if v[1] > 15}
    Bs = {k for k, v in sets[b].items() if v[1] > 15}
    res[f'{a}->{b}'] = {
        'count': [len(A), len(Bs)],
        'entered': [[k[0], k[1], sets[b][k][0], sets[b][k][1], sets[a].get(k, (None, None))[1]] for k in sorted(Bs - A)],
        'left': [[k[0], k[1], sets[a][k][0], sets[a][k][1], sets[b].get(k, (None, None))[1]] for k in sorted(A - Bs)],
    }
json.dump(res, open(out, 'w', encoding='utf-8'), indent=1)
for k, v in res.items():
    print(k, v['count'], 'entered', len(v['entered']), 'left', len(v['left']))
    for e in v['entered']:
        print('  +', e)
    for e in v['left']:
        print('  -', e)
