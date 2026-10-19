"""Churn (B1) + hotspots (B2) summarizer.

usage: sum_git.py RUN_DIR BASELINE_SHA [BASELINE_HOTSPOTS_JSON]
git log --reverse --numstat --format=@%H BASELINE..HEAD -- '*.rs' {source-path excludes}.
File-level: adds in a file's 2nd..nth touching commit are churn; churn % = churned adds / all
adds (1 dp); binary '-' rows skipped; a rename counts to its new path.
Hotspot score = commits in window x max cognitive in file (c-complexity.json); a measured 0
scores 0; top-10.
"""
import collections
import json
import os
import re
import subprocess
import sys

rd, base = sys.argv[1:3]
base_hot = json.loads(sys.argv[3]) if len(sys.argv) > 3 else None
cmd = ['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..HEAD', '--',
       '*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']
out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


commits = collections.Counter()
adds_all = 0
adds_churn = 0
churned = set()
ncommits = 0
for line in out.splitlines():
    if line.startswith('@'):
        ncommits += 1
        continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    a = int(parts[0])
    p = newpath(parts[2])
    commits[p] += 1
    adds_all += a
    if commits[p] > 1:
        adds_churn += a
        if a:
            churned.add(p)

cx = json.load(open(os.path.join(rd, 'c-complexity.json'), encoding='utf-8'))['per_file_max_cognitive']
scores = sorted(((p, n * cx.get(p, 0)) for p, n in commits.items()), key=lambda kv: (-kv[1], kv[0]))
hot = [[p, float(s)] for p, s in scores[:10] if s > 0]
churn = {'pct': round(100.0 * adds_churn / adds_all, 1) if adds_all else 0.0, 'files_churned': len(churned)}
res = {'churn': churn, 'hotspots': hot, 'commits_in_window': ncommits, 'files_touched': len(commits),
       'adds_all': adds_all, 'adds_churned': adds_churn, 'per_file_commits': dict(commits.most_common())}
if base_hot is not None:
    bset = {p for p, _ in base_hot}
    res['hotspot_entrants'] = [p for p, _ in hot if p not in bset]
json.dump(res, open(os.path.join(rd, 'c-churn.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in res.items() if k != 'per_file_commits'}))
