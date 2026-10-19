"""Synthesize an --in-diff input for cargo-mutants: the lines a given commit introduced that still stand
at HEAD, as a unified diff against HEAD's tree (added lines only, new-side numbers = HEAD's).

usage: blame_diff.py COMMIT OUT_DIFF FILE...
`git blame --line-porcelain HEAD -- FILE`; a line whose blamed commit is COMMIT is an added line.
Consecutive such lines form one hunk `@@ -{n-1},0 +{n},{k} @@`.
"""
import subprocess, sys

commit, out = sys.argv[1], sys.argv[2]
files = sys.argv[3:]
full = subprocess.run(['git', 'rev-parse', commit], capture_output=True, text=True, check=True).stdout.strip()
parts, total = [], 0
for f in files:
    bl = subprocess.run(['git', 'blame', '--line-porcelain', 'HEAD', '--', f],
                        capture_output=True, text=True, check=True).stdout.split('\n')
    lines, cur = [], None
    for l in bl:
        if cur is None and len(l) >= 40 and l[:40].strip() and ' ' in l and all(c in '0123456789abcdef' for c in l[:40]):
            sha, _, final = l.split(' ')[:3]
            cur = (sha, int(final))
        elif l.startswith('\t') and cur is not None:
            lines.append((cur[1], cur[0] == full, l[1:]))
            cur = None
    hunks, run = [], []
    for n, mine, text in lines:
        if mine and run and run[-1][0] == n - 1:
            run.append((n, text))
        elif mine:
            if run:
                hunks.append(run)
            run = [(n, text)]
    if run:
        hunks.append(run)
    if not hunks:
        continue
    parts.append(f'diff --git a/{f} b/{f}\n--- a/{f}\n+++ b/{f}\n')
    for h in hunks:
        parts.append(f'@@ -{h[0][0] - 1},0 +{h[0][0]},{len(h)} @@\n' + ''.join(f'+{t}\n' for _, t in h))
        total += len(h)
    print(f, sum(len(h) for h in hunks), 'lines in', len(hunks), 'hunks')
open(out, 'w', encoding='utf-8').write(''.join(parts))
print('total added lines', total)
