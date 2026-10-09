"""Builds a cargo-mutants name filter (-F regexes over `file:line:col: `) for a set of listed mutants and proves,
by the tool's own listing under that filter, that it selects exactly that set.

usage: build_filter.py changed-set | scrolling-touched
"""
import json
import re
import subprocess
import sys

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'
BD = 'packages/blitz-dom/src/'
which = sys.argv[1]


def listing(files, extra=()):
    cmd = ['cargo', 'mutants', '--workspace', '--list', '--json']
    for f in files:
        cmd += ['--file', f]
    return json.loads(subprocess.run(cmd + list(extra), capture_output=True, text=True, check=True).stdout)


if which == 'changed-set':
    files = [BD + 'document.rs', BD + 'mutator.rs', BD + 'events/keyboard.rs']
    src = {f: open(f, encoding='utf-8').read().splitlines() for f in files}
    keep = lambda m: any('changed_nodes' in src[m['file']][i - 1] for i in
                         range(m['function']['span']['start']['line'], m['function']['span']['end']['line'] + 1))
    rule = 'every listed mutant whose enclosing function holds a `changed_nodes` read or write'
else:
    files = [BD + 'scrolling.rs']
    diff = subprocess.run(['git', 'diff', '-U0', f'{BASE}..HEAD', '--', files[0]], capture_output=True, text=True,
                          check=True).stdout
    added = set()
    for h in re.finditer(r'^@@ -\S+ \+(\d+)(?:,(\d+))? @@', diff, re.M):
        added |= set(range(int(h.group(1)), int(h.group(1)) + int(h.group(2) or 1)))
    keep = lambda m: bool(added & set(range(m['function']['span']['start']['line'],
                                            m['function']['span']['end']['line'] + 1)))
    rule = f'every listed mutant whose enclosing function holds a line added in {BASE[:8]}..HEAD'
allm = listing(files)
want = [m for m in allm if m.get('function') and keep(m)]
names = sorted(m['name'] for m in want)
assert len(set(names)) == len(names)
by_file = {}
for m in want:
    site = m['name'].split(': ', 1)[0]
    line, col = site.rsplit(':', 2)[1:]
    by_file.setdefault(m['file'], set()).add((int(line), int(col)))
regexes = ['^' + re.escape(f) + ':(' + '|'.join(f'{l}:{c}' for l, c in sorted(s)) + '): ' for f, s in
           sorted(by_file.items())]
got = sorted(m['name'] for m in listing(files, [a for r in regexes for a in ('-F', r)]))
# measured, cargo-mutants 27.1.0: a `delete field … from struct … expression` mutant passes every -F and -E
# regex (`-F '^nomatch$'` still lists them all), so the named files' field-deletion mutants ride along
riders = sorted(set(got) - set(names))
assert not set(names) - set(got), sorted(set(names) - set(got))[:5]
assert all(': delete field ' in r for r in riders), riders[:5]
fns = {}
for m in want:
    k = (m['file'], m['function']['function_name'], m['function']['span']['start']['line'],
         m['function']['span']['end']['line'])
    fns[k] = fns.get(k, 0) + 1
out = {'rule': rule, 'files': files, 'regexes': regexes, 'planned': len(got), 'in_rule': len(names),
       'riders': riders, 'riders_note': 'field-deletion mutants of the named files outside the rule: cargo-mutants '
       '27.1.0 applies no name filter to them (measured), so the invocation tests them too',
       'listed_in_files': len(allm),
       'functions': [{'file': f, 'function': n, 'lines': [a, b], 'mutants': c} for (f, n, a, b), c in sorted(fns.items())],
       'names': names}
with open(f'{RUN}/mutation-filter-blitz-dom-{which}.json', 'w', encoding='utf-8', newline='') as fh:
    json.dump(out, fh, indent=1)
    fh.write('\n')
print(which, '| in rule', len(names), '+ riders', len(riders), '= planned', len(got), 'of', len(allm), 'listed in',
      len(files), 'file(s) | functions', len(fns))
for r in riders:
    print('  rider', r)
for r in regexes:
    print('  regex', len(r), 'chars:', r[:110] + ('…' if len(r) > 110 else ''))
