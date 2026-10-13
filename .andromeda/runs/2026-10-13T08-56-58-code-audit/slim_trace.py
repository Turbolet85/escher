#!/usr/bin/env python3
"""Size discipline: keep every query's sql, row count, db_state and plane in the adoption trace; drop the result rows
of a query over 25 rows (the zero-ref dump — summarized in c-dead.json, regenerable at the record's sha)."""
import json, os, sys

p = os.path.join(sys.argv[1], 'tree-query-code-audit.json')
t = json.load(open(p, encoding='utf-8'))
for e in t:
    if isinstance(e.get('result'), list) and len(e['result']) > 25:
        e['result'] = None
        e['result_dropped'] = 'over 25 rows - summarized in c-dead.json'
json.dump(t, open(p, 'w', encoding='utf-8'), ensure_ascii=False, indent=0)
print(len(t), 'entries;', os.path.getsize(p), 'bytes')
