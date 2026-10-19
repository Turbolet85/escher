"""Sizes (A3) + complexity (A2) summarizer.

usage: sum_sizes_cx.py TOKEI_JSON RCA_DIR SRCFILES OUT_PREFIX [STRIP_PREFIX]
Population: SRCFILES (git ls-files under the source-paths definition).
Percentiles nearest-rank. Complexity: every rca space of kind `function` (closures are
`function` spaces too), cognitive.sum / cyclomatic.sum; over_ceiling = cognitive > 15.
"""
import json
import math
import os
import sys

tokei_json, rca_dir, srcfiles, out_prefix = sys.argv[1:5]
strip = sys.argv[5] if len(sys.argv) > 5 else ''


def rel(p):
    p = p.replace('\\', '/')
    if strip and p.startswith(strip):
        p = p[len(strip):]
    return p.lstrip('./') if p.startswith('./') else p


def nr(vals, q):
    s = sorted(vals)
    if not s:
        return None
    k = max(1, math.ceil(q * len(s)))
    return s[k - 1]


pop = [l.strip() for l in open(srcfiles, encoding='utf-8') if l.strip()]
popset = set(pop)

t = json.load(open(tokei_json, encoding='utf-8'))
sizes = {}
for r in t.get('Rust', {}).get('reports', []):
    p = rel(r['name'])
    if p in popset:
        sizes[p] = r['stats']['code']
vals = list(sizes.values())
top = sorted(sizes.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
size_sum = {
    'file_p50': nr(vals, 0.5), 'file_p90': nr(vals, 0.9), 'file_max': max(vals),
    'over_800': sum(1 for v in vals if v > 800),
    'top': [[p, v] for p, v in top],
}
totals = {'loc': sum(vals), 'files': len(vals)}
missing = sorted(popset - set(sizes))

fns = []


def walk(s, f):
    if s['kind'] == 'function':
        m = s['metrics']
        fns.append((s.get('name') or '', f, s['start_line'], m['cognitive']['sum'], m['cyclomatic']['sum']))
    for c in s.get('spaces', []) or []:
        walk(c, f)


for p in pop:
    jf = os.path.join(rca_dir, p + '.json')
    if not os.path.isfile(jf):
        continue
    walk(json.load(open(jf, encoding='utf-8')), p)

cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
ranked = sorted(fns, key=lambda x: (-x[3], x[1], x[2]))
cx = {
    'cyclomatic_p50': nr(cyc, 0.5), 'cyclomatic_p90': nr(cyc, 0.9),
    'cognitive_p50': nr(cog, 0.5), 'cognitive_p90': nr(cog, 0.9),
    'over_ceiling': sum(1 for v in cog if v > 15),
    'max': {'fn': ranked[0][0], 'file': ranked[0][1], 'val': ranked[0][3]},
    'top': [[x[0], f'{x[1]}:{x[2]}', x[3]] for x in ranked[:10]],
}
per_file_max_cog = {}
for x in fns:
    per_file_max_cog[x[1]] = max(per_file_max_cog.get(x[1], 0), x[3])
over = sorted([[x[0], f'{x[1]}:{x[2]}', x[3]] for x in fns if x[3] > 15], key=lambda r: (-r[2], r[1]))

json.dump({'sizes': size_sum, 'totals': totals, 'missing_from_tokei': missing},
          open(out_prefix + 'sizes.json', 'w', encoding='utf-8'), indent=1)
json.dump({'complexity': cx, 'functions': len(fns), 'over_ceiling_list': over,
           'per_file_max_cognitive': per_file_max_cog},
          open(out_prefix + 'complexity.json', 'w', encoding='utf-8'), indent=1)
print(json.dumps({'totals': totals, 'sizes': {k: v for k, v in size_sum.items() if k != 'top'},
                  'cx': {k: v for k, v in cx.items() if k != 'top'}, 'fns': len(fns), 'missing': len(missing)}))
