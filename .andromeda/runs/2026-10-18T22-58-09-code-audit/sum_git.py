"""Churn (B1) + hotspots (B2) summarizer — collectors.md pinned shapes.

usage: sum_git.py RUN_DIR BASELINE_SHA [END_SHA [COMPLEXITY_JSON [OUT_JSON]]]  (defaults HEAD, RUN_DIR/c-complexity.json, RUN_DIR/c-churn.json)
Reads `git log --reverse --numstat --format=@%H {baseline}..HEAD -- {source paths}`.
File-level approximation: per file, adds in its 2nd..nth touching commit count as churn;
churn % = churned adds / all adds. Binary `-` rows skipped; a rename counts to its new path.
Hotspot score = commits x max cognitive in file (c-complexity.json); a measured 0 scores 0.
"""
import collections, json, re, subprocess, sys

rd, base = sys.argv[1:3]
end = sys.argv[3] if len(sys.argv) > 3 else 'HEAD'
cx_p = sys.argv[4] if len(sys.argv) > 4 else f'{rd}/c-complexity.json'
out_p = sys.argv[5] if len(sys.argv) > 5 else f'{rd}/c-churn.json'
SRC = ['*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']
out = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..{end}', '--'] + SRC,
                     capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


touches = collections.Counter()
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
churned_files = sorted(f for f, n in touches.items() if n > 1)
churn = {'pct': round(100.0 * churned / adds_all, 1) if adds_all else 0.0,
         'files_churned': len(churned_files), 'adds_all': adds_all, 'adds_churned': churned,
         'commits': commits, 'files_touched': len(touches), 'churned_files': churned_files}
cx = json.load(open(cx_p, encoding='utf-8'))['per_file_max_cognitive']
scores = sorted(((f, n * cx.get(f, 0.0)) for f, n in touches.items()), key=lambda x: (-x[1], x[0]))
hot = [[f, s] for f, s in scores[:10]]
json.dump({'churn': churn, 'hotspots': hot, 'commit_counts': dict(sorted(touches.items()))},
          open(out_p, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: v for k, v in churn.items() if k != 'churned_files'}), churned_files)
print('hotspots', hot)
