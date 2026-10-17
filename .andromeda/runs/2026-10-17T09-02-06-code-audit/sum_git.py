"""Churn (B1) and hotspots (B2) summarizer, plus the coverage (A6) TOTAL row reader.

usage: sum_git.py RUN_DIR BASELINE_SHA COVERAGE_LOG
Churn: per file, adds in its 2nd..nth touching commit are churn; churn % = churned adds / all adds;
binary `-` rows skipped; a rename counts to its new path.
Hotspots: commits x max cognitive in file (c-file-cognitive.json; a measured 0 scores 0).
"""
import json, re, subprocess, sys

rd, base, cov_p = sys.argv[1:4]
SRC = ['*.rs', ':!:.andromeda/**', ':!:.claude/**', ':!:scripts/**', ':!:docs/**', ':!:refs/**']
out = subprocess.run(['git', 'log', '--reverse', '--numstat', '--format=@%H', f'{base}..HEAD', '--'] + SRC,
                     capture_output=True, text=True, check=True).stdout


def newpath(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return (m.group(1) + m.group(3) + m.group(4)).replace('//', '/')
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


commits, seen, adds_all, adds_churn = {}, {}, 0, 0
per_file_churn = {}
for line in out.splitlines():
    if not line or line.startswith('@'):
        continue
    a, d, p = line.split('\t', 2)
    if a == '-':
        continue
    p = newpath(p)
    a = int(a)
    adds_all += a
    commits[p] = commits.get(p, 0) + 1
    if commits[p] > 1:
        adds_churn += a
        per_file_churn[p] = per_file_churn.get(p, 0) + a
churn = {'pct': round(100.0 * adds_churn / adds_all, 2) if adds_all else None,
         'files_churned': len(per_file_churn), 'adds_all': adds_all, 'adds_churned': adds_churn,
         'files_touched': len(commits), 'commits_per_file': dict(sorted(commits.items()))}
json.dump(churn, open(f'{rd}/c-churn.json', 'w', encoding='utf-8'), indent=1)

cog = json.load(open(f'{rd}/c-file-cognitive.json', encoding='utf-8'))
hs = sorted(([f, float(n * cog.get(f, 0.0))] for f, n in commits.items()), key=lambda x: (-x[1], x[0]))
json.dump({'hotspots': hs[:10]}, open(f'{rd}/c-hotspots.json', 'w', encoding='utf-8'), indent=1)

line = branch = None
for l in open(cov_p, encoding='utf-8'):
    if l.startswith('TOTAL'):
        cells = l.split()
        line = float(cells[9].rstrip('%'))
        branch = None if cells[12] == '-' else float(cells[12].rstrip('%'))
floors = [l.strip() for l in open(cov_p, encoding='utf-8') if '(floor ' in l or l.startswith('coverage floor')]
json.dump({'line': line, 'branch': branch, 'floors': floors}, open(f'{rd}/c-coverage.json', 'w', encoding='utf-8'), indent=1)
print('churn', churn['pct'], 'files_churned', churn['files_churned'], 'touched', len(commits), '| hotspots', hs[:3], '| coverage', line, branch)
