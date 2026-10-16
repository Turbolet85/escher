"""Timeout sites per complete unit, from the tool's own outcomes.json, into c-mutation-timeouts.json.

usage: python3 timeouts.py {run_dir_rel}
A timeout is scored neither caught nor missed; this file is its slot (collectors.md C1).
"""
import glob
import json
import os
import sys

RD = sys.argv[1]
res = {}
for p in sorted(glob.glob(os.path.join(RD, 'c-mutation-*.json'))):
    if p.endswith('timeouts.json'):
        continue
    d = json.load(open(p, encoding='utf-8'))
    s = d['unit'].replace('/', '-')
    o = json.load(open(os.path.join(RD, 'mutants-%s' % s, 'mutants.out', 'outcomes.json'), encoding='utf-8'))
    mj = json.load(open(os.path.join(RD, 'mutants-%s' % s, 'mutants.out', 'mutants.json'), encoding='utf-8'))

    def ident(m):
        sp = m['span']
        return (m['file'], sp['start']['line'], sp['start']['column'], sp['end']['line'], sp['end']['column'],
                m.get('replacement'), m.get('genre'))
    names = {ident(m): m.get('name') for m in mj}
    rows = []
    for x in o['outcomes']:
        sc = x.get('scenario')
        m = sc.get('Mutant') if isinstance(sc, dict) else None
        if m and x.get('summary') == 'Timeout':
            site, _, mut = (m.get('name') or names[ident(m)]).partition(': ')
            rows.append([site, mut])
    rows.sort()
    assert len(rows) == d['counts']['timeout'], (d['unit'], len(rows), d['counts']['timeout'])
    res[d['unit']] = rows
json.dump(res, open(os.path.join(RD, 'c-mutation-timeouts.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps(res, indent=1))
