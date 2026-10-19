"""Delete this run's own transients — paths inside RUN_DIR only (the skill's sanctioned deletion scope).

usage: cleanup.py RUN_DIR NAME...
Each NAME resolves under RUN_DIR; anything resolving outside it is refused.
Also strips result rows (>20) from the code-graph adoption trace, keeping its SQL, row counts and db_state.
"""
import json, os, shutil, sys

rd = os.path.realpath(sys.argv[1])
assert rd.endswith('/.andromeda/runs/2026-10-18T22-58-09-code-audit'), rd
for n in sys.argv[2:]:
    p = os.path.realpath(os.path.join(rd, n))
    if os.path.commonpath([p, rd]) != rd or p == rd:
        sys.exit(f'refused: {n} resolves outside the run dir')
    if os.path.isdir(p):
        shutil.rmtree(p)
    elif os.path.exists(p):
        os.remove(p)
    print('removed', n)
t = os.path.join(rd, 'tree-query-code-audit.json')
d = json.load(open(t, encoding='utf-8'))
for e in d:
    if isinstance(e.get('result'), list) and len(e['result']) > 20:
        e['result'] = f"(dropped by the audit: {len(e['result'])} rows; summarized in c-dead.json / c-graph.json)"
json.dump(d, open(t, 'w', encoding='utf-8'), indent=1)
print('trace entries', len(d))
