"""Evidence for the monotonic findings: over-ceiling functions and >800 files, baseline vs HEAD.

usage: python3 evidence.py {run_dir}   (reads base-rca2/, rca/, base-tokei.json, tokei.json, *srcfiles.txt)
"""
import json
import os
import sys

rd = sys.argv[1]


def fns(rca, src):
    out = {}
    for f in [l.strip() for l in open(src, encoding='utf-8') if l.strip()]:
        p = os.path.join(rca, f + '.json')
        if not os.path.isfile(p):
            continue
        st = [json.load(open(p, encoding='utf-8'))]
        while st:
            s = st.pop()
            st.extend(s.get('spaces', []))
            if s['kind'] == 'function' and s['metrics']['cognitive']['sum'] > 15:
                key = (f, s.get('name'))
                out.setdefault(key, []).append((s['start_line'], s['metrics']['cognitive']['sum']))
    return out


def sizes(tok):
    d = json.load(open(tok, encoding='utf-8'))
    out = {}
    for lang, v in d.items():
        if lang == 'Total':
            continue
        for r in v.get('reports', []):
            n = os.path.normpath(r['name'])
            out[n] = out.get(n, 0) + r['stats']['code']
    return out


b = fns(os.path.join(rd, 'base-rca2'), os.path.join(rd, 'base-srcfiles.txt'))
h = fns(os.path.join(rd, 'rca'), os.path.join(rd, 'srcfiles.txt'))
bc = sum(len(v) for v in b.values())
hc = sum(len(v) for v in h.values())
entered, left, grew = [], [], []
for k in sorted(set(b) | set(h)):
    bv, hv = sorted(x[1] for x in b.get(k, [])), sorted(x[1] for x in h.get(k, []))
    if len(hv) > len(bv):
        entered.append([k[0], k[1], [x for x in h[k]]])
    elif len(hv) < len(bv):
        left.append([k[0], k[1], [x for x in b[k]]])
    elif hv != bv:
        grew.append([k[0], k[1], bv, hv])
bs, hs = sizes(os.path.join(rd, 'base-tokei.json')), sizes(os.path.join(rd, 'tokei.json'))
over_b = {f: n for f, n in bs.items() if n > 800}
over_h = {f: n for f, n in hs.items() if n > 800}
res = {'over_ceiling': {'baseline': bc, 'head': hc, 'entered': entered, 'left': left, 'changed_value': grew},
       'over_800': {'baseline': sorted(over_b.items(), key=lambda x: -x[1]),
                    'head': sorted(over_h.items(), key=lambda x: -x[1]),
                    'entered': sorted(set(over_h) - set(over_b)), 'left': sorted(set(over_b) - set(over_h)),
                    'deltas': sorted(([f, over_b.get(f), over_h.get(f)] for f in set(over_b) | set(over_h)),
                                     key=lambda x: -(x[2] or 0))}}
json.dump(res, open(os.path.join(rd, 'c-evidence-monotonic.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps(res, indent=1))
