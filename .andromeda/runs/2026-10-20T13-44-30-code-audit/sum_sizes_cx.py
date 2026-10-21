"""Sizes (tokei) + complexity (rust-code-analysis) summarizer.

usage: sum_sizes_cx.py {tokei.json} {rca_dir} {srcfiles.txt} {out_prefix}
Writes {out_prefix}sizes.json, {out_prefix}complexity.json, {out_prefix}cx_perfile.json.
Percentiles nearest-rank. Population = srcfiles.txt (one list for totals, percentiles, top, max).
Complexity: every space of kind `function` (closures included, rca names them as functions),
cognitive.sum / cyclomatic.sum; over_ceiling = cognitive > 15.
"""
import json, math, os, sys

tokei_p, rca_dir, src_p, out = sys.argv[1:5]
src = [l.strip() for l in open(src_p, encoding='utf-8') if l.strip()]
srcset = set(src)


def nr(vals, p):
    v = sorted(vals)
    if not v:
        return None
    k = max(1, math.ceil(p / 100 * len(v)))
    return v[k - 1]


tk = json.load(open(tokei_p, encoding='utf-8'))
per = {}
for rep in tk['Rust']['reports']:
    name = os.path.relpath(rep['name']) if os.path.isabs(rep['name']) else rep['name']
    name = name[2:] if name.startswith('./') else name
    per[name] = rep['stats']['code']
missing = sorted(srcset - set(per))
extra = sorted(set(per) - srcset)
codes = [per[f] for f in src if f in per]
top = sorted(((f, per[f]) for f in src if f in per), key=lambda x: (-x[1], x[0]))[:10]
sizes = {
    'file_p50': nr(codes, 50), 'file_p90': nr(codes, 90), 'file_max': max(codes),
    'over_800': sum(1 for c in codes if c > 800), 'top': [[f, c] for f, c in top],
    'totals': {'loc': sum(codes), 'files': len(codes)},
    'population_missing': missing, 'population_extra': extra,
}
json.dump(sizes, open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)

fns = []


def walk(sp, file):
    if sp['kind'] == 'function':
        m = sp['metrics']
        fns.append((sp['name'], file, sp['start_line'], m['cognitive']['sum'], m['cyclomatic']['sum']))
    for c in sp.get('spaces') or []:
        walk(c, file)


nfiles = 0
for f in src:
    p = os.path.join(rca_dir, f + '.json')
    if not os.path.isfile(p):
        continue
    nfiles += 1
    walk(json.load(open(p, encoding='utf-8')), f)

cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
ranked = sorted(fns, key=lambda x: -x[3])  # stable: ties keep population order
cx = {
    'functions': len(fns), 'files_read': nfiles,
    'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
    'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
    'over_ceiling': sum(1 for c in cog if c > 15),
    'max': {'fn': ranked[0][0], 'file': ranked[0][1], 'val': ranked[0][3]},
    'top': [[n, f'{f}:{l}', c] for n, f, l, c, _ in ranked[:10]],
}
json.dump(cx, open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)

perfile = {}
for n, f, l, c, y in fns:
    a = perfile.setdefault(f, [0, 0])
    a[0] = max(a[0], c)
    a[1] = max(a[1], y)
json.dump(perfile, open(out + 'cx_perfile.json', 'w', encoding='utf-8'))
print('sizes', {k: v for k, v in sizes.items() if k != 'top'})
print('complexity', {k: v for k, v in cx.items() if k != 'top'})
