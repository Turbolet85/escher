"""scrolling.rs: planned mutants per function, and which functions hold a line added since the baseline."""
import json
import re
import subprocess

BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'
F = 'packages/blitz-dom/src/scrolling.rs'
ms = json.loads(subprocess.run(['cargo', 'mutants', '-p', 'blitz-dom', '--list', '--json', '--file', F],
                               capture_output=True, text=True, check=True).stdout)
diff = subprocess.run(['git', 'diff', '-U0', f'{BASE}..HEAD', '--', F], capture_output=True, text=True,
                      check=True).stdout
added = set()
for h in re.finditer(r'^@@ -\S+ \+(\d+)(?:,(\d+))? @@', diff, re.M):
    a, n = int(h.group(1)), int(h.group(2) or 1)
    added |= set(range(a, a + n))
fns = {}
for m in ms:
    fn = m['function']
    k = (fn['span']['start']['line'], fn['span']['end']['line'], fn['function_name'])
    fns[k] = fns.get(k, 0) + 1
tot = touched = 0
for (a, b, name), n in sorted(fns.items()):
    hit = len(added & set(range(a, b + 1)))
    tot += n
    touched += n if hit else 0
    print(f"  {n:3d} {a}-{b} {name}" + (f"   <- {hit} added lines" if hit else ''))
assert tot == len(ms)
print('total', tot, '| in functions holding an added line', touched, '| added lines', len(added))
