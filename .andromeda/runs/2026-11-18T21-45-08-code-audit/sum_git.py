"""Churn (B1) and hotspots (B2). usage: python3 sum_git.py {run_dir} {base} {head} {perfile-cx.json}
git log --reverse --numstat --format=@%H base..head -- '*.rs' (source-path excludes); per file, adds in its 2nd..nth
touching commits are churn; pct = churned adds / all adds (1 place); files_churned = files touched by 2+ commits;
binary '-' rows skipped; a rename line counts to the new path. hotspots = commits x max cognitive in file (a measured
0 scores 0), top 10.
"""
import json, os, re, subprocess, sys

run_dir, base, head, cx = sys.argv[1:5]
out = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..{head}', '--', '*.rs',
                      ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**'],
                     capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


touches, adds_all, churned, commits = {}, 0, 0, 0
for l in out.splitlines():
    if l.startswith('@'):
        commits += 1
        continue
    parts = l.split('\t')
    if len(parts) != 3 or parts[0] == '-':
        continue
    a, f = int(parts[0]), newpath(parts[2])
    touches[f] = touches.get(f, 0) + 1
    adds_all += a
    if touches[f] > 1:
        churned += a
per = json.load(open(cx, encoding='utf-8'))
hs = sorted(((f, float(n * per[f])) for f, n in touches.items() if f in per), key=lambda x: (-x[1], x[0]))[:10]
o = {'churn': {'pct': round(100 * churned / adds_all, 1) if adds_all else None, 'files_churned': sum(n > 1 for n in touches.values())},
     'hotspots': [[f, s] for f, s in hs], 'commits': commits, 'files_touched': len(touches), 'adds': adds_all, 'churned_adds': churned}
json.dump(o, open(os.path.join(run_dir, 'c-churn.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps(o))
