"""Sizes (A3) + complexity (A2) summarizer — collectors.md pinned shapes.

usage: sum_sizes_cx.py TOKEI_JSON RCA_DIR SRCFILES OUT_PREFIX
Population = SRCFILES (one repo-relative path per line). Per-file code lines = tokei's
Rust report `stats.code` for that path. Per-function values = every rust-code-analysis space
of kind `function` (closures included), cognitive.sum and cyclomatic.sum. Percentiles nearest-rank.
"""
import json, math, os, sys

tokei_p, rca_dir, src_p, out = sys.argv[1:5]
src = [l.strip() for l in open(src_p, encoding='utf-8') if l.strip()]
pop = set(src)


def nr(vals, q):
    s = sorted(vals)
    return s[max(0, math.ceil(q * len(s)) - 1)] if s else None


tk = json.load(open(tokei_p, encoding='utf-8'))
code = {}
for rep in tk['Rust']['reports']:
    name = rep['name']
    if name.startswith('./'):
        name = name[2:]
    code[name] = rep['stats']['code']
missing = sorted(pop - set(code))
sizes = [code.get(f, 0) for f in src]
top = sorted(((f, code.get(f, 0)) for f in src), key=lambda x: (-x[1], x[0]))[:10]
s = {
    'file_p50': nr(sizes, 0.5), 'file_p90': nr(sizes, 0.9), 'file_max': max(sizes),
    'over_800': sum(1 for v in sizes if v > 800), 'top': [list(t) for t in top],
}
totals = {'loc': sum(sizes), 'files': len(src)}
json.dump({'sizes': s, 'totals': totals, 'missing_from_tokei': missing},
          open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)

fns = []  # (fn, file, line, cognitive, cyclomatic)


def walk(sp, f):
    if sp['kind'] == 'function':
        m = sp['metrics']
        fns.append((sp['name'], f, sp['start_line'], m['cognitive']['sum'], m['cyclomatic']['sum']))
    for c in sp.get('spaces') or []:
        walk(c, f)


per_file_max_cog = {}
for f in src:
    p = os.path.join(rca_dir, f + '.json')
    if not os.path.isfile(p):
        continue
    d = json.load(open(p, encoding='utf-8'))
    before = len(fns)
    walk(d, f)
    per_file_max_cog[f] = max((x[3] for x in fns[before:]), default=0.0)
cog = [x[3] for x in fns]
cyc = [x[4] for x in fns]
ranked = sorted(fns, key=lambda x: (-x[3], x[1], x[2]))
c = {
    'cyclomatic_p50': nr(cyc, 0.5), 'cyclomatic_p90': nr(cyc, 0.9),
    'cognitive_p50': nr(cog, 0.5), 'cognitive_p90': nr(cog, 0.9),
    'over_ceiling': sum(1 for v in cog if v > 15),
    'max': {'fn': ranked[0][0], 'file': ranked[0][1], 'val': ranked[0][3]},
    'top': [[x[0], f'{x[1]}:{x[2]}', x[3]] for x in ranked[:10]],
    'functions': len(fns),
}
json.dump({'complexity': c, 'per_file_max_cognitive': per_file_max_cog},
          open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
print('sizes', json.dumps(s), json.dumps(totals), 'missing', len(missing))
print('complexity', json.dumps({k: v for k, v in c.items() if k != 'top'}))
