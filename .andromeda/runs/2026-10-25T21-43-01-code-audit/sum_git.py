"""Churn (B1) + hotspots (B2) summarizer.

usage: sum_git.py RUN_DIR BASELINE_SHA BASELINE_HOTSPOTS_JSON
numstat: git log --reverse --numstat --format=@%H {base}..HEAD -- '*.rs' {source-path excludes}.
Per file, adds in its 2nd..nth touching commit are churn; churn % = churned adds / all adds;
files_churned = files touched by 2+ commits; binary '-' rows skipped; a rename counts to its new path.
Hotspot score = commits x max cognitive in file (a measured 0 scores 0).
"""
import json, re, subprocess, sys
from collections import defaultdict

rd, base, base_hot = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
cmd = ['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..HEAD', '--', '*.rs',
       ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']
out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return re.sub('//+', '/', m.group(1) + m.group(3) + m.group(4))
    if ' => ' in p:
        return p.split(' => ', 1)[1]
    return p


touches = defaultdict(int)
adds_all = churned = 0
commits = 0
for line in out.splitlines():
    if line.startswith('@'):
        commits += 1
        continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    a, f = int(parts[0]), newpath(parts[2])
    touches[f] += 1
    adds_all += a
    if touches[f] > 1:
        churned += a
cx = json.load(open(f'{rd}/c-complexity.json', encoding='utf-8'))['per_file_max_cognitive']
hot = sorted(((f, n * cx.get(f, 0)) for f, n in touches.items()), key=lambda x: (-x[1], x[0]))[:10]
base_files = {h[0] for h in base_hot}
churn = {'pct': round(100 * churned / adds_all, 1) if adds_all else None,
         'files_churned': sum(n > 1 for n in touches.values())}
json.dump({'churn': churn, 'commits': commits, 'files_touched': len(touches), 'adds': adds_all,
           'churned_adds': churned, 'touches': dict(sorted(touches.items(), key=lambda x: -x[1])[:30]),
           'hotspots': [list(h) for h in hot],
           'hotspot_entrants': [h[0] for h in hot if h[0] not in base_files]},
          open(f'{rd}/c-churn.json', 'w', encoding='utf-8'), indent=1)
print(json.dumps(churn), 'commits', commits, 'files', len(touches), 'adds', adds_all, 'hot', hot[:3],
      'entrants', [h[0] for h in hot if h[0] not in base_files])
