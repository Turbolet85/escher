#!/usr/bin/env python3
"""Evidence for the informational lines: hotspot commit origin, escher-crate complexity, dead candidates by crate,
the dup top pair's entering epoch. Reads this run's c-*.json, rca/ and the ledger; prints, writes c-evidence.json."""
import json, os, re, subprocess, sys

RD, BASE = sys.argv[1], sys.argv[2]
ESCHER = ('packages/escher-driver/', 'packages/escher-telemetry/', 'examples/seven_guis/', 'packages/dioxus-native-dom/',
          'packages/blitz-test-harness/')
out = {}

# hotspot files: touching commits split upstream (subject ends with a PR number) vs fork
hs = json.load(open(os.path.join(RD, 'c-hotspots.json'), encoding='utf-8'))
origin = {}
for f, _ in hs['hotspots']:
    log = subprocess.run(['git', 'log', '--format=%s', f'{BASE}..HEAD', '--', f], capture_output=True, text=True).stdout
    subj = [s for s in log.splitlines() if s]
    up = sum(bool(re.search(r'\(#\d+\)$', s)) or not re.match(r'^(feat|fix|chore|docs|refactor)\(', s) for s in subj)
    origin[f] = {'commits': len(subj), 'upstream': up, 'fork': len(subj) - up}
out['hotspot_origin'] = origin

# over-ceiling (cognitive > 15) functions in escher's own crates
fns = []


def walk(s, f):
    if s.get('kind') == 'function':
        fns.append((s['name'], f, s['start_line'], s['metrics']['cognitive']['sum']))
    for c in s.get('spaces', []):
        walk(c, f)


for dp, _, fs in os.walk(os.path.join(RD, 'rca')):
    for fn in fs:
        if fn.endswith('.json'):
            d = json.load(open(os.path.join(dp, fn), encoding='utf-8'))
            walk(d, d['name'])
out['escher_over_ceiling'] = sorted([[n, f'{f}:{l}', c] for n, f, l, c in fns if c > 15 and f.startswith(ESCHER)],
                                    key=lambda x: -x[2])
out['over_ceiling_by_root'] = {}
for n, f, l, c in fns:
    if c > 15:
        k = '/'.join(f.split('/')[:2])
        out['over_ceiling_by_root'][k] = out['over_ceiling_by_root'].get(k, 0) + 1

# dead candidates by crate
dead = json.load(open(os.path.join(RD, 'c-dead.json'), encoding='utf-8'))
by = {}
for s, _ in dead['candidates_full']:
    by[s.split(' ')[0]] = by.get(s.split(' ')[0], 0) + 1
out['dead_by_crate'] = dict(sorted(by.items(), key=lambda kv: -kv[1]))

# dup top pairs: the epoch each first entered a ledger top list
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
dup = json.load(open(os.path.join(RD, 'c-duplication.json'), encoding='utf-8'))
entered = []
for a, b, n in dup['top']:
    first = next((r['epoch'] for r in recs if [a, b, n] in r['duplication']['top']), 'this epoch')
    entered.append([a, b, n, first])
out['dup_top_entered'] = entered
json.dump(out, open(os.path.join(RD, 'c-evidence.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(json.dumps(out, ensure_ascii=False, indent=1))
