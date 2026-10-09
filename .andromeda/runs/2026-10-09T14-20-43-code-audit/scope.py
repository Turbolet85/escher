"""Coverage summary + the Tier C scope: files touched since the baseline, mapped to workspace units."""
import json
import os
import re
import subprocess

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'

# ---- A6 coverage: the TOTAL row of the leg's report
log = os.path.join(RUN, 'coverage-raw.log')
if os.path.isfile(log):
    total = [l for l in open(log, encoding='utf-8', errors='replace') if l.startswith('TOTAL')][-1].split()
    # TOTAL regions missed cover% functions missed executed% lines missed cover% branches missed cover
    results = [l for l in open(log, encoding='utf-8', errors='replace') if 'test result:' in l]
    cov = {
        'line': float(total[9].rstrip('%')), 'branch': None if total[10] == '0' else float(total[12].rstrip('%')),
        'lines': int(total[7]), 'lines_missed': int(total[8]),
        'regions_pct': float(total[3].rstrip('%')), 'functions_pct': float(total[6].rstrip('%')),
        'branches_instrumented': int(total[10]),
        'leg_exit': open(os.path.join(RUN, 'coverage-exit.txt')).read().strip(),
        'test_result_lines': len(results),
        'test_result_lines_with_failures': sum(1 for l in results if ' 0 failed' not in l),
    }
    with open(os.path.join(RUN, 'c-coverage.json'), 'w', encoding='utf-8', newline='') as f:
        json.dump(cov, f, indent=1)
        f.write('\n')
    print('coverage', cov)

# ---- Tier C scope
meta = json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked'],
                                 capture_output=True, text=True, check=True).stdout)
root = meta['workspace_root'].rstrip('/') + '/'
units = sorted(((os.path.dirname(p['manifest_path'])[len(root):], p['name']) for p in meta['packages']),
               key=lambda u: -len(u[0]))
names = subprocess.run(['git', 'diff', '--name-status', f'{BASE}..HEAD'], capture_output=True, text=True,
                       check=True).stdout.splitlines()
EXCL = ('.andromeda/', '.claude/', 'scripts/', 'docs/', 'refs/')
by_unit, unmapped = {}, []
for line in names:
    parts = line.split('\t')
    status, path = parts[0], parts[-1]
    if path.startswith(EXCL):
        continue
    is_src = path.endswith('.rs')
    is_manifest = os.path.basename(path) in ('Cargo.toml', 'Cargo.lock', 'build.rs')
    if not (is_src or is_manifest):
        continue
    for d, n in units:
        if d and path.startswith(d + '/'):
            by_unit.setdefault(n, []).append((status, path[len(d) + 1:], 'src' if is_src else 'manifest'))
            break
    else:
        unmapped.append((status, path))
for n in sorted(by_unit):
    rows = by_unit[n]
    mutable = [p for s, p, k in rows if k == 'src' and p.startswith('src/') and s != 'D']
    print(f'{n}: {len(rows)} touched | under src/: {len(mutable)}')
    for s, p, k in rows:
        print('    ', s, p, '' if k == 'src' else '(manifest)')
print('unmapped (workspace root):', unmapped)
with open(os.path.join(RUN, 'scope.json'), 'w', encoding='utf-8', newline='') as f:
    json.dump({'by_unit': by_unit, 'unmapped': unmapped, 'unit_dirs': {n: d for d, n in units}}, f, indent=1)
    f.write('\n')
