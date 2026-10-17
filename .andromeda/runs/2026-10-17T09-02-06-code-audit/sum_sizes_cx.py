"""Sizes (A3) and complexity (A2) summarizer, per collectors.md's pinned rows.

usage: sum_sizes_cx.py TOKEI_JSON RCA_DIR SRCFILES OUT_PREFIX [ROOT_STRIP]
Writes {OUT_PREFIX}sizes.json, {OUT_PREFIX}complexity.json, {OUT_PREFIX}file-cognitive.json.
"""
import json, math, os, sys

tokei_p, rca_dir, src_p, out = sys.argv[1:5]
strip = sys.argv[5] if len(sys.argv) > 5 else ''
src = [l.strip() for l in open(src_p, encoding='utf-8') if l.strip()]
srcset = set(src)


def rel(p):
    if strip and p.startswith(strip):
        p = p[len(strip):]
    return p.lstrip('./') if p.startswith('./') else p


def nr(vals, q):  # nearest-rank percentile on sorted values
    v = sorted(vals)
    return v[max(1, math.ceil(q * len(v))) - 1] if v else None


# ---- sizes: per-file Rust code lines over the source population
tk = json.load(open(tokei_p, encoding='utf-8'))
code = {}
for r in tk['Rust']['reports']:
    code[rel(r['name'])] = r['stats']['code']
missing = sorted(srcset - set(code))
sizes = [code.get(f, 0) for f in src]
ranked = sorted(((f, code.get(f, 0)) for f in src), key=lambda x: (-x[1], x[0]))
json.dump({
    'file_p50': nr(sizes, 0.5), 'file_p90': nr(sizes, 0.9), 'file_max': max(sizes),
    'over_800': sum(1 for s in sizes if s > 800),
    'top': [list(x) for x in ranked[:10]],
    'totals': {'loc': sum(sizes), 'files': len(src)},
    'missing_from_tokei': missing,
}, open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)

# ---- complexity: every space of kind function (rca emits closures as function)
fns = []


def walk(s, f):
    if s['kind'] == 'function':
        m = s['metrics']
        fns.append((s.get('name') or '', f, s['start_line'], m['cognitive']['sum'], m['cyclomatic']['sum']))
    for c in s.get('spaces', []):
        walk(c, f)


for f in src:
    p = os.path.join(rca_dir, f + '.json')
    if os.path.isfile(p):
        walk(json.load(open(p, encoding='utf-8')), f)
cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
top = sorted(fns, key=lambda x: (-x[3], x[1], x[2]))
per_file = {}
for x in fns:
    per_file[x[1]] = max(per_file.get(x[1], 0.0), x[3])
json.dump({
    'functions': len(fns),
    'cyclomatic_p50': nr(cyc, 0.5), 'cyclomatic_p90': nr(cyc, 0.9),
    'cognitive_p50': nr(cog, 0.5), 'cognitive_p90': nr(cog, 0.9),
    'over_ceiling': sum(1 for v in cog if v > 15),
    'max': {'fn': top[0][0], 'file': top[0][1], 'val': top[0][3]},
    'top': [[x[0], f'{x[1]}:{x[2]}', x[3]] for x in top[:10]],
}, open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
json.dump(per_file, open(out + 'file-cognitive.json', 'w', encoding='utf-8'), indent=0, sort_keys=True)
print('files', len(src), 'loc', sum(sizes), 'fns', len(fns), 'over_ceiling', sum(1 for v in cog if v > 15), 'missing', len(missing))
