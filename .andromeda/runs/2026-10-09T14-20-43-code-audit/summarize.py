"""Pinned summarizers for the code audit's Tier A/B collectors.

usage: summarize.py <raw_dir> <out_dir> <abs_root> <git_range|-> [--no-dead]

raw_dir   holds srcfiles.txt, tokei.json, jscpd-abs/jscpd-report.json, rca/ (and q-*.json unless --no-dead)
out_dir   receives c-{metric}.json
abs_root  the absolute checkout root jscpd's --absolute paths start with
git_range the churn window (baseline..HEAD), or - to skip the churn family

Percentiles are nearest-rank on the sorted values. Every list is ordered by a total key, so a re-run of
the same inputs writes the same bytes.
"""
import json
import math
import os
import re
import subprocess
import sys

raw, out, abs_root, git_range = sys.argv[1:5]
NO_DEAD = '--no-dead' in sys.argv
abs_root = abs_root.rstrip('/') + '/'
PATHSPEC = ["*.rs", ":!:.andromeda/**", ":!:.claude/**", ":!:scripts/**", ":!:docs/**", ":!:refs/**"]


def rank(vals, p):
    s = sorted(vals)
    if not s:
        return None
    return s[max(1, math.ceil(p / 100 * len(s))) - 1]


def dump(name, obj):
    with open(os.path.join(out, f'c-{name}.json'), 'w', encoding='utf-8', newline='') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')
    return obj


def is_test_path(p):
    # a `tests` DIRECTORY segment; a `*_tests.rs` file under src/ reads src (the rule that reproduces the
    # Epoch 3 record's split from its own tree: 160/1666 · 119/1508 · 8/110)
    return 'tests' in p.split('/')[:-1]


srcfiles = [l.strip() for l in open(os.path.join(raw, 'srcfiles.txt'), encoding='utf-8') if l.strip()]

# ---- A3 sizes: per-file Rust code lines over the one population (srcfiles.txt)
tok = json.load(open(os.path.join(raw, 'tokei.json'), encoding='utf-8'))
per_file = {}
for rep in tok['Rust']['reports']:
    name = rep['name'][2:] if rep['name'].startswith('./') else rep['name']
    per_file[name] = rep['stats']['code']
missing = sorted(set(srcfiles) - set(per_file))
extra = sorted(set(per_file) - set(srcfiles))
codes = list(per_file.values())
sizes_top = sorted(per_file.items(), key=lambda kv: (-kv[1], kv[0]))[:10]
sizes = dump('sizes', {
    'file_p50': rank(codes, 50), 'file_p90': rank(codes, 90), 'file_max': max(codes),
    'over_800': sum(1 for c in codes if c > 800), 'top': [[f, c] for f, c in sizes_top],
    'totals': {'loc': sum(codes), 'files': len(codes)},
    'population': {'srcfiles': len(srcfiles), 'tokei_reports': len(per_file), 'missing_from_tokei': missing,
                   'not_in_srcfiles': extra},
})
print('sizes', {k: sizes[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'totals')},
      'missing', len(missing), 'extra', len(extra))

# ---- A1 duplication
js = json.load(open(os.path.join(raw, 'jscpd-abs/jscpd-report.json'), encoding='utf-8'))
tot = js['statistics']['total']


def rel(p):
    return p[len(abs_root):] if p.startswith(abs_root) else p


rows = []
split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
for d in js['duplicates']:
    a, b = rel(d['firstFile']['name']), rel(d['secondFile']['name'])
    ta, tb = is_test_path(a), is_test_path(b)
    cls = 'test' if ta and tb else 'src' if not ta and not tb else 'mixed'
    split[cls]['pairs'] += 1
    split[cls]['lines'] += d['lines']
    lo, hi = sorted([(a, d['firstFile']['start']), (b, d['secondFile']['start'])])
    rows.append((d['lines'], lo[0], hi[0], lo[1], hi[1], cls))
rows.sort(key=lambda r: (-r[0], r[1], r[2], r[3], r[4]))
dup = dump('duplication', {
    'pct': round(tot['percentage'], 3), 'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'],
    'clones': tot['clones'], 'top': [[r[1], r[2], r[0]] for r in rows[:10]],
    'top_fragments': [{'a': r[1], 'b': r[2], 'lines': r[0], 'a_start': r[3], 'b_start': r[4], 'class': r[5]}
                      for r in rows[:10]],
    'split': split, 'sources': tot['sources'],
})
print('duplication', {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'split')})

# ---- A2 complexity: every space of kind function (closures included); cognitive.sum / cyclomatic.sum
funcs = []


def walk(space, file):
    if space['kind'] == 'function':
        funcs.append((space['name'], file, space['start_line'],
                      space['metrics']['cognitive']['sum'], space['metrics']['cyclomatic']['sum']))
    for c in space['spaces']:
        walk(c, file)


rca_missing = []
for f in srcfiles:
    p = os.path.join(raw, 'rca', f + '.json')
    if not os.path.isfile(p):
        rca_missing.append(f)
        continue
    walk(json.load(open(p, encoding='utf-8')), f)
cog = [x[3] for x in funcs]
cyc = [x[4] for x in funcs]
by_cog = sorted(funcs, key=lambda x: (-x[3], x[1], x[2], x[0]))
over = [x for x in by_cog if x[3] > 15]
file_max_cog = {}
for x in funcs:
    file_max_cog[x[1]] = max(file_max_cog.get(x[1], 0), x[3])
cplx = dump('complexity', {
    'cyclomatic_p50': rank(cyc, 50), 'cyclomatic_p90': rank(cyc, 90),
    'cognitive_p50': rank(cog, 50), 'cognitive_p90': rank(cog, 90),
    'over_ceiling': len(over),
    'max': {'fn': by_cog[0][0], 'file': by_cog[0][1], 'val': by_cog[0][3]},
    'top': [[x[0], f'{x[1]}:{x[2]}', x[3]] for x in by_cog[:10]],
    'functions': len(funcs), 'files_measured': len(srcfiles) - len(rca_missing), 'files_without_output': rca_missing,
    'over_ceiling_list': [[x[0], f'{x[1]}:{x[2]}', x[3]] for x in over],
})
print('complexity', {k: cplx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                             'over_ceiling', 'max', 'functions')}, 'no-output', len(rca_missing))

# ---- B1 churn / B2 hotspots
if git_range != '-':
    log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', git_range, '--'] + PATHSPEC,
                         capture_output=True, text=True, check=True).stdout
    touches = {}
    commit = None
    binary_rows = 0
    for line in log.splitlines():
        if line.startswith('@'):
            commit = line[1:]
            continue
        parts = line.split('\t')
        if len(parts) != 3:
            continue
        adds, dels, path = parts
        if adds == '-':
            binary_rows += 1
            continue
        m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', path)
        if m:
            path = (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
        elif ' => ' in path:
            path = path.split(' => ', 1)[1]
        touches.setdefault(path, []).append((commit, int(adds), int(dels)))
    all_adds = sum(a for t in touches.values() for _, a, _ in t)
    churned = {f: sum(a for _, a, _ in t[1:]) for f, t in touches.items() if len(t) > 1}
    churned_adds = sum(churned.values())
    multi = sorted(((f, len(touches[f]), churned[f], sum(a for _, a, _ in touches[f])) for f in churned),
                   key=lambda r: (-r[2], r[0]))
    churn = dump('churn', {
        'pct': round(100 * churned_adds / all_adds, 2) if all_adds else None,
        'files_churned': sum(1 for f in churned if churned[f] > 0),
        'files_touched_twice_or_more': len(churned),
        'files_touched': len(touches), 'all_adds': all_adds, 'churned_adds': churned_adds,
        'commits_touching_source': len({c for t in touches.values() for c, _, _ in t}),
        'binary_rows_skipped': binary_rows,
        'top': [[f, n, ca, aa] for f, n, ca, aa in multi[:10]],
        'top_columns': ['file', 'touching commits', 'churned adds', 'all adds'],
    })
    print('churn', {k: churn[k] for k in ('pct', 'files_churned', 'files_touched_twice_or_more', 'files_touched',
                                           'all_adds', 'churned_adds', 'commits_touching_source')})
    hot = sorted(((f, len(t) * int(file_max_cog.get(f, 0)), len(t), file_max_cog.get(f)) for f, t in touches.items()),
                 key=lambda r: (-r[1], r[0]))
    gone = sorted(f for f in touches if f not in set(srcfiles))
    hs = dump('hotspots', {
        'top': [[f, s] for f, s, _, _ in hot[:10]],
        'top_detail': [{'file': f, 'score': s, 'commits': n, 'max_cognitive': c} for f, s, n, c in hot[:10]],
        'touched_files_absent_at_head': gone,
    })
    print('hotspots', hs['top'])

if NO_DEAD:
    sys.exit(0)

# ---- A4 graph
q = lambda n: json.load(open(os.path.join(raw, f'q-{n}.json'), encoding='utf-8'))
PREFIX = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')


def short(sym):
    m = PREFIX.match(sym)
    return f'{m.group(1)} {sym[m.end():]}' if m else sym


cycles = sorted(r['path'] for r in q('cycles'))
graph = dump('graph', {
    'cycles': len(cycles), 'cycle_paths': cycles,
    'fan_in_top': [[short(r['callee']), r['n']] for r in q('fan_in')],
    'fan_out': [[r['from_crate'], r['n']] for r in sorted(q('fan_out'), key=lambda r: (-r['n'], r['from_crate']))],
    'cross_unit_edges': q('edges')[0]['n'],
    'edge_list': [[r['from_crate'], r['to_crate']] for r in q('edge_list')],
    'units_in_graph': len(q('crates')),
})
print('graph', {k: graph[k] for k in ('cycles', 'cross_unit_edges', 'units_in_graph')})

# ---- A5 dead: the ledger's recipes.dead, applied in its order
zr = q('zero_ref')
texts = {}


def text(f):
    if f not in texts:
        try:
            texts[f] = open(f, encoding='utf-8').read()
        except OSError:
            texts[f] = None
    return texts[f]


def seg_test(seg):
    return seg in ('test', 'tests') or seg.endswith('_test') or seg.endswith('_tests')


excluded, trait_impl, residual = 0, 0, []
for r in zr:
    stripped = PREFIX.sub('', r['symbol'])
    if '/tests/' in '/' + stripped or '/tests/' in '/' + (r['file'] or ''):
        excluded += 1
    elif re.search(r'impl#\[[^\]]*\]\[[^\]]*\]', stripped):
        trait_impl += 1
    else:
        residual.append((r, stripped))
on_line = {}
for r, _ in residual:
    on_line[(r['file'], r['def_line'])] = on_line.get((r['file'], r['def_line']), 0) + 1
unread = set()


def classify(r, stripped):
    f = r['file'] or ''
    if stripped in ('main().', 'crate/'):
        return 'entry-point'
    path_segs = [s for s in stripped.split('/')[:-1]]
    file_segs = f.split('/')
    file_segs = file_segs[:-1] + [file_segs[-1][:-3] if file_segs[-1].endswith('.rs') else file_segs[-1]]
    if any(seg_test(s) for s in path_segs + file_segs):
        return 'test-only'
    src = text(f)
    if src is None:
        unread.add(f)
    else:
        lines = src.split('\n')
        d = r['def_line']  # 0-based (SCIP), so lines[d] is the def line and lines[d-4:d] the four above it
        above = lines[max(0, d - 4):d]
        if any('#[test]' in l or '#[cfg(test)]' in l for l in above):
            return 'test-only'
    if '__bitflags_flag_names/' in stripped or 'Props#' in stripped or 'StoreImplExt#' in stripped:
        return 'derive/attr-invoked'
    if on_line[(r['file'], r['def_line'])] > 1:
        return 'runtime-invoked (macro-generated binding)'
    if src is not None and ('{' + r['name'] + '}' in src or '{' + r['name'] + ':' in src):
        return 'format-capture'
    return 'candidate'


classes = {}
cands = []
for r, stripped in residual:
    c = classify(r, stripped)
    classes[c] = classes.get(c, 0) + 1
    if c == 'candidate':
        cands.append((r['file'] or '', r['def_line'], f"{r['crate']} {stripped}", r['kind']))
cands.sort()
classes['trait-impl (dispatch)'] = trait_impl
by_unit = {}
for f, d, s, k in cands:
    u = s.split(' ', 1)[0]
    by_unit[u] = by_unit.get(u, 0) + 1
mach = open(os.path.join(raw, 'machete-stdout.txt'), encoding='utf-8').read()
unused, cur = [], None
for line in mach.splitlines():
    m = re.match(r'^(\S+) -- \./(.*Cargo\.toml):$', line)
    if m:
        cur = m.group(1)
    elif cur and line.startswith('\t'):
        unused.append([cur, line.strip()])
    elif not line.strip():
        cur = None
dead = dump('dead', {
    'unused_deps': unused, 'zero_ref_candidates': len(cands),
    'top': [[s, f'{f}:{d + 1}'] for f, d, s, k in cands[:10]],
    'fp_classes': dict(sorted(classes.items())), 'zero_ref_rows': len(zr), 'excluded_test_path': excluded,
    'candidates_by_unit': dict(sorted(by_unit.items(), key=lambda kv: (-kv[1], kv[0]))),
    'candidates_full': [[s, f'{f}:{d + 1}', k] for f, d, s, k in cands],
    'def_files_unread': sorted(unread),
    'note': "candidates, never 'dead': zero-ref by the indexer after the recipe's test exclusion and FP classing",
})
print('dead', {k: dead[k] for k in ('zero_ref_candidates', 'fp_classes', 'zero_ref_rows', 'excluded_test_path')},
      'unused_deps', len(unused), 'unread', len(unread))
