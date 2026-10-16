"""Size discipline: delete raw tool output and scratch — INSIDE this run dir only.

usage: python3 cleanup.py {run_dir_abs}
Kept: the c-*.json twins, record.json, proposals.md, the scripts, handpass-*.json, mutate-progress.log,
srcfiles.txt, base-srcfiles.txt, delta-numstat.txt, q-cycles/fanin/fanout/edges.json, tree-query-code-audit.json,
lists/targets-*.txt and lists/scope-plan.json.
"""
import glob
import os
import shutil
import sys

rd = os.path.realpath(sys.argv[1])
assert rd.endswith('/.andromeda/runs/2026-10-15T10-44-59-code-audit'), rd
pats = ['base-tree', 'base-rca', 'base-rca2', 'base-jscpd', 'base-tokei.json', 'rca', 'jscpd-abs', 'tokei.json',
        'mutants-*', 'lists/unit-*', 'lists/files-*', 'q-zero.json', 'q-zero.err', 'q-edges.out', 'q-edges.err',
        'coverage.stdout', 'handpass-*.log', 'metadata.json', 'dry-ledger.ndjson', 'doc-nontest.rs', 'rca.stdout',
        'jscpd.stdout', 'machete.txt', 'mutate.stdout', 'numstat.txt', 'fp-numstat.txt']
gone = []
for p in pats:
    for f in glob.glob(os.path.join(rd, p)):
        real = os.path.realpath(f)
        assert real.startswith(rd + '/'), real
        if os.path.isdir(real) and not os.path.islink(f):
            shutil.rmtree(real)
        else:
            os.remove(real)
        gone.append(os.path.relpath(real, rd))
print('deleted %d: %s' % (len(gone), ' '.join(sorted(gone))))
