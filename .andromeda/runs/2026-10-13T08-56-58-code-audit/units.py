#!/usr/bin/env python3
"""Touched files (baseline..HEAD) mapped to workspace units — Tier C scope (declined this run)."""
import json, os, subprocess, sys

RD, BASE = sys.argv[1], sys.argv[2]
m = json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked'],
                              capture_output=True, text=True, check=True).stdout)
root = m['workspace_root']
pk = sorted(((os.path.relpath(os.path.dirname(p['manifest_path']), root).replace(os.sep, '/'), p['name'])
             for p in m['packages']), key=lambda x: -len(x[0]))
touched = subprocess.run(['git', 'diff', '--name-only', f'{BASE}..HEAD'], capture_output=True, text=True,
                         check=True).stdout.split()
units, other = {}, []
for f in touched:
    for d, n in pk:
        if d not in ('.', '') and f.startswith(d + '/'):
            units.setdefault(n, []).append(f)
            break
    else:
        other.append(f)
out = {'units': {n: {'files': len(fs), 'rs': sum(f.endswith('.rs') for f in fs),
                     'non_rs': [f for f in fs if not f.endswith('.rs')]} for n, fs in sorted(units.items())},
       'root_level': [f for f in other if not f.startswith(('.andromeda/', '.claude/', 'escher-0'))]}
json.dump(out, open(os.path.join(RD, 'c-mutation-scope.json'), 'w', encoding='utf-8'), indent=1)
for n, v in out['units'].items():
    print(n, v['files'], v['rs'], v['non_rs'][:4])
print('root-level:', out['root_level'])
