"""Sizes (A3) + complexity (A2) summarizer.

usage: python3 sum_sizes_cx.py {tokei.json} {rca_dir} {srcfiles.txt} {out_prefix|-}
Population = srcfiles.txt (the Source-paths definition). Percentiles nearest-rank.
Function = every rca space of kind `function`; value = metrics.cognitive.sum / cyclomatic.sum.
"""
import json
import math
import os
import sys


def nr(vals, p):
    s = sorted(vals)
    if not s:
        return None
    k = max(1, math.ceil(p / 100 * len(s)))
    return s[k - 1]


def main():
    tok_p, rca_dir, src_p, out = sys.argv[1:5]
    files = [l.strip() for l in open(src_p, encoding='utf-8') if l.strip()]
    tok = json.load(open(tok_p, encoding='utf-8'))
    code = {}
    for lang, d in tok.items():
        if lang == 'Total':
            continue
        for r in d.get('reports', []):
            name = os.path.normpath(r['name'])
            code[name] = code.get(name, 0) + r['stats']['code']
    per_file = []
    missing = []
    for f in files:
        if f in code:
            per_file.append((f, code[f]))
        else:
            missing.append(f)
    vals = [v for _, v in per_file]
    top = sorted(per_file, key=lambda x: (-x[1], x[0]))[:10]
    sizes = {
        'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
        'over_800': sum(1 for v in vals if v > 800), 'top': [list(t) for t in top],
    }
    totals = {'loc': sum(vals), 'files': len(per_file)}

    fns = []
    for f in files:
        p = os.path.join(rca_dir, f + '.json')
        if not os.path.isfile(p):
            continue
        d = json.load(open(p, encoding='utf-8'))
        st = [d]
        while st:
            s = st.pop()
            st.extend(s.get('spaces', []))
            if s['kind'] == 'function':
                fns.append({'fn': s.get('name'), 'file': f, 'line': s['start_line'],
                            'cog': s['metrics']['cognitive']['sum'],
                            'cyc': s['metrics']['cyclomatic']['sum']})
    cyc = [x['cyc'] for x in fns]
    cog = [x['cog'] for x in fns]
    ranked = sorted(fns, key=lambda x: (-x['cog'], x['file'], x['line']))
    mx = ranked[0]
    per_file_max_cog = {}
    for x in fns:
        per_file_max_cog[x['file']] = max(per_file_max_cog.get(x['file'], 0), x['cog'])
    complexity = {
        'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
        'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
        'over_ceiling': sum(1 for v in cog if v > 15),
        'max': {'fn': mx['fn'], 'file': mx['file'], 'val': mx['cog']},
        'top': [[x['fn'], '%s:%d' % (x['file'], x['line']), x['cog']] for x in ranked[:10]],
        'functions': len(fns),
    }
    res = {'totals': totals, 'sizes': sizes, 'complexity': complexity,
           'tokei_missing': missing}
    if out == '-':
        print(json.dumps({'totals': totals, 'sizes': {k: v for k, v in sizes.items() if k != 'top'},
                          'complexity': {k: v for k, v in complexity.items() if k != 'top'},
                          'missing': len(missing)}))
    else:
        json.dump({'totals': totals, **sizes, 'tokei_missing': missing},
                  open(out + 'sizes.json', 'w', encoding='utf-8'), indent=1)
        json.dump(complexity, open(out + 'complexity.json', 'w', encoding='utf-8'), indent=1)
        json.dump(per_file_max_cog, open(out + 'file-maxcog.json', 'w', encoding='utf-8'), indent=0)
        print(json.dumps({'totals': totals, 'sizes': {k: v for k, v in sizes.items() if k != 'top'},
                          'complexity': {k: v for k, v in complexity.items() if k != 'top'}}))


main()
