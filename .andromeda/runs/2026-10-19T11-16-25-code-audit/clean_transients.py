"""Remove this run's own tool report dirs (audit-pass.md §Size discipline).

usage: clean_transients.py RUN_DIR NAME [NAME ...]
Each NAME is resolved under RUN_DIR; a target whose real path is not strictly inside RUN_DIR's
real path, that is a symlink, or that is not a directory is refused and nothing is removed.
"""
import os
import shutil
import sys

rd = os.path.realpath(sys.argv[1])
if not os.path.basename(rd).endswith('-code-audit') or os.sep + os.path.join('.andromeda', 'runs') + os.sep not in rd + os.sep:
    sys.exit(f'refused: {rd} is not a code-audit run dir')
targets = []
for name in sys.argv[2:]:
    p = os.path.join(rd, name)
    real = os.path.realpath(p)
    if os.path.islink(p) or os.path.commonpath([rd, real]) != rd or real == rd or not os.path.isdir(real):
        sys.exit(f'refused: {name} (outside the run dir, a link, or not a directory); nothing removed')
    targets.append(real)
for t in targets:
    shutil.rmtree(t)
    print('removed', os.path.relpath(t, rd))
