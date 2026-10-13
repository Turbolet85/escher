#!/usr/bin/env python3
"""Delete this run's own transients — only paths inside this run dir (the sanctioned delete scope)."""
import os, shutil, sys

RD = os.path.dirname(os.path.abspath(__file__))
for name in sys.argv[1:]:
    p = os.path.abspath(os.path.join(RD, name))
    if os.path.commonpath([p, RD]) != RD or p == RD:
        sys.exit(f'refused: {name} is outside the run dir')
    if os.path.isdir(p):
        shutil.rmtree(p)
    elif os.path.exists(p):
        os.remove(p)
    print('removed', name)
