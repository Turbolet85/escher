"""Churn + hotspots summarizer.

usage: sum_git.py {run_dir} {baseline_sha} {baseline_hotspots_json}
churn: git log --reverse --numstat --format=@%H {baseline}..HEAD over the source paths; per file the
adds in its 2nd..nth touching commit are churn; pct = churned adds / all adds; files_churned = files
touched by 2+ commits in the window. Binary `-` rows skipped; a rename counts to its new path.
hotspots: commits x max cognitive per file (c-cx_perfile.json; fallback max cyclomatic; a measured 0
scores 0); top-10 + entrants vs the baseline's top-10.
"""
import json, re, subprocess, sys
from collections import defaultdict

rd, base, base_hot = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
PATHS = ['*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']
out = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..HEAD', '--'] + PATHS,
                     capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


commits = defaultdict(int)
adds_all = 0
churned = 0
commit_count = 0
for line in out.splitlines():
    if line.startswith('@'):
        commit_count += 1
        continue
    parts = line.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    a, f = int(parts[0]), newpath(parts[2])
    commits[f] += 1
    adds_all += a
    if commits[f] >= 2:
        churned += a
churn = {'pct': round(100 * churned / adds_all, 1) if adds_all else None,
         'files_churned': sum(1 for v in commits.values() if v >= 2),
         'commits': commit_count, 'files_touched': len(commits), 'adds': adds_all, 'churned_adds': churned,
         'per_file_commits': dict(sorted(commits.items(), key=lambda x: (-x[1], x[0])))}
json.dump(churn, open(f'{rd}/c-churn.json', 'w', encoding='utf-8'), indent=1)

cx = json.load(open(f'{rd}/c-cx_perfile.json', encoding='utf-8'))
scores = []
for f, n in commits.items():
    if f in cx:
        scores.append([f, float(n * cx[f][0])])  # a measured cognitive 0 scores 0
scores.sort(key=lambda x: (-x[1], x[0]))
top = scores[:10]
prev = {x[0] for x in base_hot}
hot = {'top': top, 'entrants': [x[0] for x in top if x[0] not in prev]}
json.dump(hot, open(f'{rd}/c-hotspots.json', 'w', encoding='utf-8'), indent=1)
print('churn', {k: v for k, v in churn.items() if k != 'per_file_commits'})
print('hotspots', top)
print('entrants', hot['entrants'])
