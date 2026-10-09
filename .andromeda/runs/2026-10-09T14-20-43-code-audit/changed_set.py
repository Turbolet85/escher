"""Planned mutants of blitz-dom's changed-set code: every listed mutant whose enclosing function holds a
`changed_nodes` read or write (cargo-mutants' own listing, no build, no output dir)."""
import json
import os
import subprocess

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
FILES = ['packages/blitz-dom/src/document.rs', 'packages/blitz-dom/src/mutator.rs',
         'packages/blitz-dom/src/events/keyboard.rs', 'packages/blitz-dom/src/scrolling.rs']
cmd = ['cargo', 'mutants', '-p', 'blitz-dom', '--list', '--json']
for f in FILES:
    cmd += ['--file', f]
ms = json.loads(subprocess.run(cmd, capture_output=True, text=True, check=True).stdout)
lines = {f: open(f, encoding='utf-8').read().splitlines() for f in FILES}
out = {'files': {}, 'functions': [], 'mutants': []}
per_fn = {}
for m in ms:
    out['files'][m['file']] = out['files'].get(m['file'], 0) + 1
    fn = m.get('function')
    if not fn or m['file'].endswith('scrolling.rs'):
        continue
    a, b = fn['span']['start']['line'], fn['span']['end']['line']
    hits = [i for i in range(a, b + 1) if 'changed_nodes' in lines[m['file']][i - 1]]
    if not hits:
        continue
    k = (m['file'], fn['function_name'], a, b)
    per_fn.setdefault(k, {'n': 0, 'hits': hits})['n'] += 1
    out['mutants'].append(m['name'])
for (f, name, a, b), v in sorted(per_fn.items()):
    out['functions'].append({'file': f, 'function': name, 'lines': [a, b], 'changed_nodes_lines': v['hits'],
                             'mutants': v['n']})
out['changed_set_planned'] = len(out['mutants'])
assert len(set(out['mutants'])) == len(out['mutants'])
assert sum(x['mutants'] for x in out['functions']) == out['changed_set_planned']
with open(os.path.join(RUN, 'mutation-plan-blitz-dom.json'), 'w', encoding='utf-8', newline='') as f:
    json.dump(out, f, indent=1)
    f.write('\n')
print('listed per file', out['files'])
print('changed-set mutants', out['changed_set_planned'], 'over', len(out['functions']), 'functions')
for x in out['functions']:
    print(f"  {x['mutants']:3d} {x['file'].rsplit('/', 1)[1]}:{x['lines'][0]}-{x['lines'][1]} {x['function']} "
          f"(changed_nodes at {x['changed_nodes_lines']})")
