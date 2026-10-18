"""B1 churn + B2 hotspots summarizer (collectors.md pinned).

usage: sum_git.py {run_dir} {churn.numstat}
numstat: `git log --reverse --numstat --format=@%H {base}..HEAD -- {source paths}`.
Per file, adds in its 2nd..nth touching commit are churn; churn % = churned
adds / all adds (1 dp); files_churned = files touched by 2+ commits. Binary `-`
rows skipped; a rename counts to its new path. Hotspot score = commits x max
cognitive in the file (from c-filecog.json; a file absent from it — deleted at
HEAD — scores 0, as does a measured 0). Top-10 by score, ties by path.
"""
import collections
import json
import os
import re
import sys

rd, ns = sys.argv[1:3]


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return re.sub('//+', '/', m.group(1) + m.group(3) + m.group(4))
    if ' => ' in p:
        return p.split(' => ', 1)[1]
    return p


commits = collections.Counter()
adds_all = 0
churned = 0
for line in open(ns, encoding='utf-8'):
    line = line.rstrip('\n')
    if not line or line.startswith('@'):
        continue
    a, d, p = line.split('\t', 2)
    if a == '-':
        continue
    p = newpath(p)
    a = int(a)
    adds_all += a
    if commits[p] >= 1:
        churned += a
    commits[p] += 1
cog = json.load(open(os.path.join(rd, 'c-filecog.json'), encoding='utf-8'))
hot = sorted(([f, n * cog.get(f, 0.0)] for f, n in commits.items()), key=lambda r: (-r[1], r[0]))
churn = {
    'pct': round(100.0 * churned / adds_all, 1) if adds_all else None,
    'files_churned': sum(1 for n in commits.values() if n >= 2),
    'adds': adds_all, 'churned_adds': churned, 'files_touched': len(commits),
    'commits_per_file': dict(sorted(commits.items(), key=lambda kv: (-kv[1], kv[0]))),
}
json.dump(churn, open(os.path.join(rd, 'c-churn.json'), 'w', encoding='utf-8'), indent=1)
json.dump(hot[:10], open(os.path.join(rd, 'c-hotspots.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: churn[k] for k in ('pct', 'files_churned', 'adds', 'churned_adds', 'files_touched')}))
print(json.dumps(hot[:10]))
