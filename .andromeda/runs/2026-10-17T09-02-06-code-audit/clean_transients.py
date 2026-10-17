"""Delete this run's raw tool output (summarized into c-*.json) — paths inside THIS run dir only.

usage: clean_transients.py RUN_DIR
"""
import os, shutil, sys

rd = os.path.realpath(sys.argv[1])
assert os.path.basename(rd) == '2026-10-17T09-02-06-code-audit', rd
DIRS = ['rca', 'jscpd-abs', 'mutants-seven_guis', 'mutants-escher-driver', 'mutants-blitz-dom']
FILES = ['tokei.json', 'metadata.json', 'zero-ref.raw.json', 'coverage.raw.log', 'machete.out', 'rca.log', 'jscpd.log',
         'hand-seven_guis.log', 'hand-escher-driver.log', 'hand-blitz-dom.log',
         'mutants-seven_guis.log', 'mutants-escher-driver.log', 'mutants-blitz-dom.log']
for n in DIRS + FILES:
    p = os.path.realpath(os.path.join(rd, n))
    assert os.path.dirname(p) == rd, p
    if os.path.isdir(p) and not os.path.islink(p):
        shutil.rmtree(p)
    elif os.path.isfile(p):
        os.remove(p)
    print('removed', n)
