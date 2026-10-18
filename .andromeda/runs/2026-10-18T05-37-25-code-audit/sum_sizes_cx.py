"""A2 complexity + A3 sizes summarizer (collectors.md pinned shapes).

usage: sum_sizes_cx.py {tokei.json} {rca_dir} {srcfiles.txt} {out_prefix}
Population: srcfiles.txt (one path per line). Percentiles nearest-rank.
Function spaces: every rca space of kind `function`, closures included;
value = metrics.cognitive.sum / metrics.cyclomatic.sum.
"""
import json
import math
import os
import sys

tokei_p, rca_dir, src_p, out = sys.argv[1:5]
src = [l.strip() for l in open(src_p, encoding='utf-8') if l.strip()]
pop = set(src)


def nr(vals, q):
    v = sorted(vals)
    if not v:
        return None
    return v[max(1, math.ceil(q * len(v))) - 1]


# sizes
tk = json.load(open(tokei_p, encoding='utf-8'))
code = {}
for lang, d in tk.items():
    if lang == 'Total':
        continue
    for r in d.get('reports', []):
        n = os.path.relpath(r['name']) if os.path.isabs(r['name']) else r['name']
        if n.startswith('./'):
            n = n[2:]
        if n in pop:
            code[n] = code.get(n, 0) + r['stats']['code']
missing = sorted(pop - set(code))
vals = list(code.values())
top = sorted(code.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
sizes = {
    'file_p50': nr(vals, 0.5), 'file_p90': nr(vals, 0.9), 'file_max': max(vals),
    'over_800': sum(1 for v in vals if v > 800), 'top': [[f, n] for f, n in top],
}
totals = {'loc': sum(vals), 'files': len(code)}

# complexity
fns = []


def walk(s, f):
    if s['kind'] == 'function':
        m = s['metrics']
        fns.append((s['name'], f, s['start_line'], m['cognitive']['sum'], m['cyclomatic']['sum']))
    for x in s['spaces']:
        walk(x, f)


for f in src:
    p = os.path.join(rca_dir, f + '.json')
    if os.path.isfile(p):
        walk(json.load(open(p, encoding='utf-8')), f)
cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
ranked = sorted(fns, key=lambda x: (-x[3], x[1], x[2]))
complexity = {
    'cyclomatic_p50': nr(cyc, 0.5), 'cyclomatic_p90': nr(cyc, 0.9),
    'cognitive_p50': nr(cog, 0.5), 'cognitive_p90': nr(cog, 0.9),
    'over_ceiling': sum(1 for v in cog if v > 15),
    'max': {'fn': ranked[0][0], 'file': ranked[0][1], 'val': ranked[0][3]},
    'top': [[n, f'{f}:{l}', c] for n, f, l, c, _ in ranked[:10]],
    'functions': len(fns),
}
# per-file max cognitive, for hotspots
pf = {}
for n, f, l, c, _ in fns:
    pf[f] = max(pf.get(f, 0.0), c)
json.dump({'sizes': sizes, 'totals': totals, 'missing_from_tokei': missing},
          open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)
json.dump(complexity, open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
json.dump(pf, open(out + 'filecog.json', 'w', encoding='utf-8'), indent=0)
print('sizes', json.dumps(sizes)[:300], json.dumps(totals))
print('complexity', {k: complexity[k] for k in complexity if k != 'top'})
