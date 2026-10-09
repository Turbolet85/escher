"""Planned mutant counts per touched unit: cargo-mutants' own listing (no build, no output dir)."""
import json
import os
import subprocess

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
scope = json.load(open(os.path.join(RUN, 'scope.json'), encoding='utf-8'))
dirs = scope['unit_dirs']
plan = {}
for unit in sorted(scope['by_unit']):
    r = subprocess.run(['cargo', 'mutants', '-p', unit, '--list', '--json'], capture_output=True, text=True)
    if r.returncode != 0:
        print(unit, 'LIST FAILED', r.returncode, r.stderr.strip()[-300:])
        continue
    ms = json.loads(r.stdout)
    per_file = {}
    for m in ms:
        per_file[m['file']] = per_file.get(m['file'], 0) + 1
    touched = sorted(f'{dirs[unit]}/{p}' for s, p, k in scope['by_unit'][unit] if k == 'src' and s != 'D')
    touched_counts = {f: per_file.get(f, 0) for f in touched if per_file.get(f, 0)}
    plan[unit] = {'unit_planned': len(ms), 'unit_files': len(per_file),
                  'touched_planned': sum(touched_counts.values()), 'touched_files': touched_counts}
    print(f"{unit}: unit {len(ms)} mutants over {len(per_file)} files | touched files {sum(touched_counts.values())}")
    for f, n in touched_counts.items():
        print('     ', n, f)
with open(os.path.join(RUN, 'mutation-plan.json'), 'w', encoding='utf-8', newline='') as f:
    json.dump(plan, f, indent=1)
    f.write('\n')
