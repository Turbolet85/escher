"""The blitz-tests / seven_guis test targets that can reach a unit's code.

usage: python3 targets.py {metadata.json} {unit}
Reach is dependency-aware: the unit's identifier set is the unit plus every workspace package
that depends on it, transitively. A blitz-tests target reaches the unit when its file names one of
those identifiers, or declares a shared module (`mod common;` / `mod session_common;`) whose file
names one. Every seven_guis test target reaches it when seven_guis is in the set (those tests run
the `escher-session` binary). Printed as `--test={target}` per line, sorted; the packages on line 1: the unit, blitz-tests,
seven_guis when reached, and every escher test-holding crate in the set (their `--lib` tests).
"""
import glob
import json
import os
import re
import sys

meta = json.load(open(sys.argv[1], encoding='utf-8'))
unit = sys.argv[2]
names = {p['name'] for p in meta['packages']}
rdeps = {n: set() for n in names}
for p in meta['packages']:
    for d in p['dependencies']:
        if d['name'] in names and d.get('path'):
            rdeps[d['name']].add(p['name'])
closure, todo = {unit}, [unit]
while todo:
    for r in rdeps[todo.pop()]:
        if r not in closure:
            closure.add(r)
            todo.append(r)
idents = {n.replace('-', '_') for n in closure}

targets, pkgs = [], [unit]
d = 'tests/blitz-tests/tests'
shared = {}
for m in ('common', 'session_common'):
    t = open(os.path.join(d, m, 'mod.rs'), encoding='utf-8').read()
    shared[m] = any(re.search(r'\b%s\b' % i, t) for i in idents)
for f in sorted(glob.glob(os.path.join(d, '*.rs'))):
    t = open(f, encoding='utf-8').read()
    hit = any(re.search(r'\b%s\b' % i, t) for i in idents)
    for m, uses in shared.items():
        if uses and re.search(r'^\s*(pub\s+)?mod\s+%s\s*;' % m, t, re.M):
            hit = True
    if hit:
        targets.append(os.path.basename(f)[:-3])
if targets and 'blitz-tests' not in pkgs:
    pkgs.append('blitz-tests')
if 'seven_guis' in closure:
    if 'seven_guis' not in pkgs:
        pkgs.append('seven_guis')
    targets += [os.path.basename(f)[:-3] for f in sorted(glob.glob('examples/seven_guis/tests/*.rs'))]
LIBS = ('escher-driver', 'escher-telemetry', 'dioxus-native-dom', 'blitz-test-harness', 'seven_guis')
pkgs += [n for n in LIBS if n in closure and n not in pkgs]
print(','.join(pkgs))
print('\n'.join('--test=' + t for t in sorted(set(targets))))
