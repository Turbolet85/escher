"""Pinned summarizers for Tier A1-A3 and Tier B (code audit).

Usage: summarize.py <tokei.json> <jscpd-report.json> <rca-dir> <strip-prefix> <churn-range> <out-dir>

Rules (each reproduced the Epoch 2 record's stored values on an export of 42b80ad9 before this run used them):
- sizes: tokei Rust `stats.code` per report; nearest-rank p50/p90; over_800 = code > 800; totals = sum / count.
- duplication: jscpd `statistics.total`; top-10 clones by lines (report order on ties); a path is `test` when it
  holds a `tests/` segment, a pair is `mixed` when its two sides differ.
- complexity: every rust-code-analysis space of kind `function` (closures included, named `<anonymous>`),
  `cognitive.sum` / `cyclomatic.sum`; over_ceiling = cognitive > 15; top-10 by cognitive.
- churn: per file, adds in its 2nd..nth touching commit are churn; binary `-` rows skipped; a rename counts to the
  new path. files_churned = files with churned adds > 0.
- hotspots: commits touching the file in the range x max cognitive in the file; a measured 0 scores 0 (the fallbacks,
  max cyclomatic then KLOC, apply only to a file the complexity tool returned no function for).
"""
import json
import math
import os
import re
import subprocess
import sys

tokei_p, jscpd_p, rca_dir, strip, churn_range, out = sys.argv[1:7]
PATHSPEC = ['*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']


def nr(vals, p):
    v = sorted(vals)
    return v[max(0, math.ceil(p / 100 * len(v)) - 1)]


def dump(name, obj):
    with open(os.path.join(out, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


# --- A3 sizes
rust = json.load(open(tokei_p, encoding='utf-8'))['Rust']
per_file = {r['name'][2:] if r['name'].startswith('./') else r['name']: r['stats']['code'] for r in rust['reports']}
vals = list(per_file.values())
sizes = {
    'file_p50': nr(vals, 50), 'file_p90': nr(vals, 90), 'file_max': max(vals),
    'over_800': sum(1 for v in vals if v > 800),
    'top': [[k, v] for k, v in sorted(per_file.items(), key=lambda kv: -kv[1])[:10]],
    'over_800_files': [[k, v] for k, v in sorted(per_file.items(), key=lambda kv: -kv[1]) if v > 800],
    'totals': {'loc': sum(vals), 'files': len(vals)},
}
assert sizes['totals']['loc'] == rust['code'], 'per-file sum differs from tokei total'
dump('c-sizes.json', sizes)

# --- A1 duplication
rep = json.load(open(jscpd_p, encoding='utf-8'))
tot = rep['statistics']['total']


def rel(p):
    return p[len(strip):] if p.startswith(strip) else p


def cls(p):
    return 'test' if '/tests/' in '/' + p else 'src'


split = {k: {'pairs': 0, 'lines': 0} for k in ('src', 'test', 'mixed')}
rows = []
for d in rep['duplicates']:
    a, b = rel(d['firstFile']['name']), rel(d['secondFile']['name'])
    k = cls(a) if cls(a) == cls(b) else 'mixed'
    split[k]['pairs'] += 1
    split[k]['lines'] += d['lines']
    rows.append([a, b, d['lines'], d['firstFile']['start'], d['secondFile']['start']])
assert len(rows) == tot['clones']
by_lines = sorted(rows, key=lambda r: -r[2])
dup = {
    'pct': round(tot['percentage'], 3), 'duplicated_lines': tot['duplicatedLines'], 'total_lines': tot['lines'],
    'clones': tot['clones'], 'top': [r[:3] for r in by_lines[:10]], 'split': split,
    'top_with_starts': by_lines[:10],
    'by_file': sorted(
        ([f, n] for f, n in __import__('collections').Counter(x for r in rows for x in set(r[:2])).items()),
        key=lambda kv: -kv[1])[:10],
}
dump('c-duplication.json', dup)
dump('dup-rows.json', rows)  # transient: the full pair list, for the entrant diff; deleted after P4

# --- A2 complexity
fns = []


def walk(space, file):
    if space['kind'] == 'function':
        fns.append((space['name'], file, space['start_line'],
                    space['metrics']['cognitive']['sum'], space['metrics']['cyclomatic']['sum']))
    for c in space['spaces']:
        walk(c, file)


n_files = 0
for root, _, files in os.walk(rca_dir):
    for f in files:
        d = json.load(open(os.path.join(root, f), encoding='utf-8'))
        n_files += 1
        walk(d, d['name'])
cog = [f[3] for f in fns]
cyc = [f[4] for f in fns]
by_cog = sorted(fns, key=lambda f: -f[3])
over = [f for f in by_cog if f[3] > 15]
cx = {
    'functions': len(fns), 'files': n_files,
    'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
    'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
    'over_ceiling': len(over),
    'max': {'fn': by_cog[0][0], 'file': by_cog[0][1], 'val': by_cog[0][3]},
    'top': [[f[0], '%s:%d' % (f[1], f[2]), f[3]] for f in by_cog[:10]],
    'over_ceiling_list': [[f[0], '%s:%d' % (f[1], f[2]), f[3]] for f in over],
}
dump('c-complexity.json', cx)
max_cog, max_cyc = {}, {}
for name, file, _, c, y in fns:
    max_cog[file] = max(max_cog.get(file, 0), c)
    max_cyc[file] = max(max_cyc.get(file, 0), y)

# --- B1 churn / B2 hotspots
if churn_range != 'none':
    log = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', churn_range, '--'] + PATHSPEC,
                         capture_output=True, text=True, check=True).stdout
    seen, commits, adds_all, adds_churn, churned = set(), {}, 0, 0, {}
    n_commits = 0
    for line in log.splitlines():
        if line.startswith('@'):
            n_commits += 1
            continue
        if not line.strip():
            continue
        a, d, path = line.split('\t', 2)
        if a == '-':
            continue
        m = re.match(r'(.*)\{(.*) => (.*)\}(.*)', path)
        if m:
            path = (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
        elif ' => ' in path:
            path = path.split(' => ', 1)[1]
        a = int(a)
        adds_all += a
        commits[path] = commits.get(path, 0) + 1
        if path in seen:
            adds_churn += a
            churned[path] = churned.get(path, 0) + a
        seen.add(path)
    churn = {
        'range': churn_range, 'commits_in_range': n_commits, 'files_touched': len(commits),
        'adds_all': adds_all, 'adds_churned': adds_churn,
        'pct': round(100 * adds_churn / adds_all, 2) if adds_all else None,
        'files_churned': sum(1 for v in churned.values() if v > 0),
        'files_multi_commit': sum(1 for v in commits.values() if v > 1),
        'top': [[k, v] for k, v in sorted(churned.items(), key=lambda kv: -kv[1])[:10]],
    }
    dump('c-churn.json', churn)
    scores = []
    for path, n in commits.items():
        if path in max_cog:  # a measured 0 is a value; the fallbacks are for a file the tool gave no function for
            w, basis = max_cog[path], 'cognitive'
        elif path in max_cyc:
            w, basis = max_cyc[path], 'cyclomatic'
        else:
            w, basis = per_file.get(path, 0) / 1000, 'kloc'
        scores.append([path, round(n * w, 3) if basis == 'kloc' else int(n * w), n, w, basis])
    scores.sort(key=lambda s: -s[1])
    dump('c-hotspots.json', {'formula': 'commits in range x max cognitive in file (fallback cyclomatic, then KLOC)',
                             'top': [s[:2] for s in scores[:10]], 'top_detail': scores[:10]})

print(json.dumps({'sizes': {k: v for k, v in sizes.items() if k not in ('top', 'over_800_files')},
                  'dup': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'split')},
                  'cx': {k: v for k, v in cx.items() if k not in ('top', 'over_ceiling_list')}}, ensure_ascii=False))
if churn_range != 'none':
    print(json.dumps({'churn': {k: v for k, v in churn.items() if k != 'top'}, 'hotspots': scores[:10]}))
