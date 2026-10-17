"""Compare the summarizers' output on a git archive of the baseline sha with the baseline record.

usage: check_repro.py LEDGER BASELINE_SHA PREFIX   (PREFIX + sizes.json / complexity.json / dup.json)
"""
import json, sys

ledger, sha, pre = sys.argv[1:4]
recs = [json.loads(l) for l in open(ledger, encoding='utf-8') if l.strip()]
b = [r for r in recs if r['sha'] == sha][-1]
s = json.load(open(pre + 'sizes.json', encoding='utf-8'))
c = json.load(open(pre + 'complexity.json', encoding='utf-8'))
d = json.load(open(pre + 'dup.json', encoding='utf-8'))
bad = 0
for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top'):
    ok = s[k] == b['sizes'][k]; bad += not ok; print('sizes', k, 'OK' if ok else f'DIFF {s[k]} vs {b["sizes"][k]}')
for k in ('loc', 'files'):
    ok = s['totals'][k] == b['totals'][k]; bad += not ok; print('totals', k, 'OK' if ok else 'DIFF')
for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top'):
    ok = c[k] == b['complexity'][k]; bad += not ok; print('complexity', k, 'OK' if ok else f'DIFF {c[k]} vs {b["complexity"][k]}')
for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split'):
    ok = d[k] == b['duplication'][k]; bad += not ok; print('dup', k, 'OK' if ok else f'DIFF {d[k]} vs {b["duplication"][k]}')
print('mismatches', bad)
