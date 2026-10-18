"""Delete this run's raw tool output — only paths inside this run dir (the skill's sanctioned delete scope)."""
import os
import shutil
import sys

rd = os.path.realpath(sys.argv[1])
assert rd.endswith('-code-audit') and '/.andromeda/runs/' in rd, rd
RAW = ['rca', 'jscpd-abs', 'repro', 'tokei.json', 'rca.log', 'jscpd.log', 'coverage.raw.log', 'churn.numstat',
       'machete.out', 'q0.out', 'q0.err', 'g-zero_ref.json', 'g-cycles.json', 'g-fan_in.json', 'g-fan_out.json',
       'g-edges.json']
for name in RAW:
    p = os.path.realpath(os.path.join(rd, name))
    assert os.path.dirname(p) == rd, p
    if os.path.isdir(p):
        shutil.rmtree(p)
    elif os.path.exists(p):
        os.remove(p)
    print('removed' if not os.path.exists(p) else 'KEPT', name)
