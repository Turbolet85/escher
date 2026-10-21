"""Size discipline: delete this run's raw tool output, ONLY inside this run's own dir (the sanctioned scope).

usage: cleanup.py {run_dir}
"""
import json, os, shutil, sys

rd = os.path.realpath(sys.argv[1])
assert os.path.basename(rd) == '2026-10-20T13-44-30-code-audit', rd

DIRS = ['base', 'base-rca', 'base-jscpd', 'rca', 'jscpd-abs', 'mutants-dioxus-native-dom', 'mutants-blitz-traits',
        'mutants-blitz-vibey-script', 'mutants-escher-driver', 'mutants-blitz-dom']
FILES = ['base-tokei.json', 'tokei.json', 'jscpd.out', 'coverage.out', 'machete.out', 'handpass-broad.log',
         'handpass-driver.log', 'zero_ref_rows.json', 'c-cx_perfile.json', 'base-c-cx_perfile.json'] + [
    f for f in os.listdir(rd) if f.startswith('mutants-') and f.endswith('.log')]


def inside(p):
    p = os.path.realpath(p)
    assert os.path.commonpath([p, rd]) == rd and p != rd, p
    return p


p = os.path.join(rd, 'tree-query-code-audit.json')
t = json.load(open(p, encoding='utf-8'))
for r in t:
    if r['rows'] > 100:
        r['result'] = f"[{r['rows']} rows — cut to the count by the audit (size discipline); summarized in c-dead.json]"
json.dump(t, open(p, 'w', encoding='utf-8'), indent=1)

for d in DIRS:
    p = inside(os.path.join(rd, d))
    if os.path.isdir(p):
        shutil.rmtree(p)
for f in FILES:
    p = inside(os.path.join(rd, f))
    if os.path.isfile(p):
        os.remove(p)
print(sorted(os.listdir(rd)))
