"""Size discipline — remove raw tool output from THIS run dir after summarization (the only sanctioned delete scope).

Usage: cleanup.py <run_dir>
"""
import json
import os
import shutil
import sys

rd = os.path.realpath(sys.argv[1])
assert os.path.basename(rd).endswith('-code-audit') and '/.andromeda/runs/' in rd
RAW = ['tokei.json', 'jscpd-abs', 'jscpd.stdout', 'rca', 'rca.stdout', 'q-fanin.out', 'q-fanout.out', 'q-edges.out',
       'q-zeroref.out', 'machete.stdout', 'mutants-dioxus-native-dom', 'mutation-dioxus-native-dom.stdout',
       'mutation-start.txt', 'dup-rows.json', 'c-dead-class.json']
for name in RAW:
    p = os.path.realpath(os.path.join(rd, name))
    assert p.startswith(rd + os.sep)
    if os.path.isdir(p):
        shutil.rmtree(p)
    elif os.path.exists(p):
        os.remove(p)

# the query trace: keep every record's sql / rows / db_state / plane; cap the one uncapped result (the zero-ref rows)
tp = os.path.join(rd, 'tree-query-code-audit.json')
trace = json.load(open(tp, encoding='utf-8'))
for r in trace:
    if isinstance(r.get('result'), list) and len(r['result']) > 20:
        r['result_capped'] = 'first 20 of %d rows kept by the code audit (size discipline); rerun the sql at this sha for all' % len(r['result'])
        r['result'] = r['result'][:20]
json.dump(trace, open(tp, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(sorted(os.listdir(rd)))
print(sum(os.path.getsize(os.path.join(rd, f)) for f in os.listdir(rd)), 'bytes')
