"""Sizes (A3) and complexity (A2) summarizer.

usage: python3 sum_sizes_cx.py {tokei.json} {rca_dir} {srcfiles.txt} {units} {out_prefix}
Population: the files in srcfiles.txt (the source-paths definition). Per-file code = tokei stats.code.
Complexity: every rca space of kind 'function' (closures included), cognitive.sum / cyclomatic.sum;
percentiles nearest-rank; over_ceiling = cognitive > 15; top by cognitive desc, then file, line.
Writes {out_prefix}sizes.json, {out_prefix}complexity.json, {out_prefix}perfile-cx.json.
"""
import json, math, os, sys

tok_path, rca_dir, src_path, units, out = sys.argv[1:6]
files = [l.strip() for l in open(src_path, encoding='utf-8') if l.strip()]
pop = set(files)


def nr(vals, p):
    s = sorted(vals)
    if not s:
        return None
    k = max(1, math.ceil(p / 100 * len(s)))
    return s[k - 1]


tok = json.load(open(tok_path, encoding='utf-8'))
code = {}
for r in tok['Rust']['reports']:
    n = r['name'].replace('\\', '/')
    if n.startswith('./'):
        n = n[2:]
    if n in pop:
        code[n] = r['stats']['code']
sizes = sorted(code.values())
top = sorted(code.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
s_out = {'file_p50': nr(sizes, 50), 'file_p90': nr(sizes, 90), 'file_max': max(sizes), 'over_800': sum(v > 800 for v in sizes),
         'top': [[f, v] for f, v in top], 'totals': {'loc': sum(sizes), 'files': len(code), 'units': int(units)},
         'files_missing_from_tokei': sorted(pop - set(code))}
json.dump(s_out, open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)

fns = []
for f in files:
    p = os.path.join(rca_dir, f + '.json')
    if not os.path.exists(p):
        continue
    j = json.load(open(p, encoding='utf-8'))

    def walk(s):
        if s['kind'] == 'function':
            fns.append({'fn': s['name'], 'file': f, 'line': s['start_line'],
                        'cog': s['metrics']['cognitive']['sum'], 'cyc': s['metrics']['cyclomatic']['sum']})
        for c in s.get('spaces', []):
            walk(c)
    walk(j)
cog = [x['cog'] for x in fns]
cyc = [x['cyc'] for x in fns]
order = sorted(fns, key=lambda x: (-x['cog'], x['file'], x['line']))
mx = order[0]
c_out = {'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90), 'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
         'over_ceiling': sum(v > 15 for v in cog), 'max': {'fn': mx['fn'], 'file': mx['file'], 'val': mx['cog']},
         'top': [{'fn': x['fn'], 'file': f"{x['file']}:{x['line']}", 'value': x['cog']} for x in order[:10]],
         'functions': len(fns)}
json.dump(c_out, open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
per = {}
for x in fns:
    per[x['file']] = max(per.get(x['file'], 0), x['cog'])
json.dump(per, open(out + 'perfile-cx.json', 'w', encoding='utf-8'), indent=0)
print(json.dumps({k: v for k, v in s_out.items() if k != 'top'}), json.dumps({k: v for k, v in c_out.items() if k != 'top'}))
