"""Sizes (A3) + complexity (A2) summarizer.

usage: sum_sizes_cx.py ROOT TOKEI_JSON RCA_DIR SRCFILES OUT_PREFIX
Population: the SRCFILES list (repo-relative). Per-file size = tokei's Rust report stats.code.
Complexity: every rca space of kind `function` (closures included), cognitive.sum / cyclomatic.sum.
Percentiles nearest-rank. over_ceiling = cognitive > 15.
"""
import json, math, os, sys

root, tokei_json, rca_dir, srcfiles, out = sys.argv[1:6]
root = os.path.abspath(root)
files = [l.strip() for l in open(srcfiles, encoding='utf-8') if l.strip()]
fset = set(files)


def rel(p):
    p = os.path.abspath(os.path.join(root, p)) if not os.path.isabs(p) else p
    return os.path.relpath(p, root).replace(os.sep, '/')


def nr(vals, p):
    s = sorted(vals)
    if not s:
        return None
    return s[max(0, math.ceil(p / 100 * len(s)) - 1)]


tk = json.load(open(tokei_json, encoding='utf-8'))
size = {}
for rep in tk.get('Rust', {}).get('reports', []):
    r = rel(rep['name'])
    if r in fset:
        size[r] = rep['stats']['code']
missing = sorted(fset - set(size))
vals = [size[f] for f in files if f in size]
top = sorted(((f, size[f]) for f in size), key=lambda x: (-x[1], x[0]))[:10]
sizes = {'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
         'over_800': sum(v > 800 for v in vals), 'top': [list(t) for t in top]}
totals = {'loc': sum(vals), 'files': len(vals)}

fns = []


def walk(sp, f):
    if sp.get('kind') == 'function':
        m = sp['metrics']
        fns.append({'fn': sp.get('name'), 'file': f, 'line': sp.get('start_line'),
                    'cog': m['cognitive']['sum'], 'cyc': m['cyclomatic']['sum']})
    for c in sp.get('spaces') or []:
        walk(c, f)


seen = set()
for dp, _, fs in os.walk(rca_dir):
    for n in fs:
        if not n.endswith('.json'):
            continue
        d = json.load(open(os.path.join(dp, n), encoding='utf-8'))
        f = rel(d['name'])
        if f not in fset or f in seen:
            continue
        seen.add(f)
        walk(d, f)
cog = [x['cog'] for x in fns]
cyc = [x['cyc'] for x in fns]
order = sorted(fns, key=lambda x: (-x['cog'], x['file'], x['line']))
mx = order[0]
complexity = {'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
              'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
              'over_ceiling': sum(c > 15 for c in cog),
              'max': {'fn': mx['fn'], 'file': mx['file'], 'val': mx['cog']},
              'top': [[x['fn'], f"{x['file']}:{x['line']}", x['cog']] for x in order[:10]]}
perfile = {}
for x in fns:
    perfile[x['file']] = max(perfile.get(x['file'], 0), x['cog'])
json.dump({'sizes': sizes, 'totals': totals, 'missing_from_tokei': missing},
          open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)
json.dump({'complexity': complexity, 'functions': len(fns), 'files_read': len(seen),
           'files_unread': sorted(fset - seen)[:20], 'per_file_max_cognitive': perfile},
          open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
print('sizes', json.dumps(sizes)[:300], json.dumps(totals))
print('complexity', json.dumps({k: v for k, v in complexity.items() if k != 'top'}), 'fns', len(fns),
      'files', len(seen), 'unread', len(fset - seen), 'tokei-missing', len(missing))
