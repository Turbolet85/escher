"""Read-only cross-epoch lookback for recurrence (diagnosis-pass §Inputs). Writes q-lookback.json."""
import collections
import json
import os
import re

OUT = os.path.dirname(os.path.abspath(__file__))
TYPES = ['tooling.host-shell', 'recall.corpus-recurrence', 'input.report-insufficient',
         'contract.in-pass-correction', 'contract.mechanical-check', 'input.plan-step-ambiguous',
         'tooling.light-gate-red', 'contract.cascade-miss']


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


retr = json.load(open(os.path.join(OUT, 'q-retractions.json'), encoding='utf-8'))
excl = set(retr['retracted_ids'])
excl_p = {(a, b) for a, b in retr['retracted_problems']}
by_type = collections.defaultdict(collections.Counter)
overridden = []
themes = {'provenance': re.compile(r'provenance', re.I),
          'bash-c-quoting': re.compile(r'bash -c|nested quot|triple', re.I),
          'stdout-suppressed': re.compile(r'/dev/null|piped into grep', re.I),
          'scripted-bulk-edit': re.compile(r'(python|scratch).{0,40}(replace|script).{0,60}(instead of|rather than).{0,20}Edit', re.I),
          'estimated-stamp': re.compile(r'stamp-ahead|estimated .{0,20}(stamp|Generated)|without reading the clock', re.I)}
theme_hits = collections.defaultdict(list)
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        continue
    key = (r.get('version') or '?') + ' / ' + norm(r.get('epoch'))
    if r.get('kind') == 'friction' and r.get('id') not in excl:
        if r.get('type') in TYPES:
            by_type[r['type']][key] += 1
        for t, rx in themes.items():
            if rx.search(r.get('what') or ''):
                theme_hits[t].append([key, r.get('chunk'), 'friction', r.get('id'), (r.get('what') or '')[:160]])
    if r.get('kind') == 'step':
        p = r.get('problem')
        lst = [] if p is None else (p if isinstance(p, list) else [p])
        for i, f in enumerate(lst):
            if (r.get('id'), i) in excl_p or (r.get('id'), None) in excl_p:
                continue
            if f.get('solution') == 'overridden':
                overridden.append([key, r.get('chunk'), re.sub('^andromeda-', '', r['skill']) + '/' + r['step'],
                                   r.get('id'), (f.get('note') or '')[:200]])
            for t, rx in themes.items():
                if rx.search(f.get('note') or ''):
                    theme_hits[t].append([key, r.get('chunk'), 'fact:' + str(f.get('solution')), r.get('id'),
                                          (f.get('note') or '')[:160]])
json.dump({'types_by_epoch': {k: dict(v) for k, v in by_type.items()}, 'overridden_facts': overridden,
           'theme_hits': theme_hits},
          open(os.path.join(OUT, 'q-lookback.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
for k, v in by_type.items():
    print(k, dict(v))
print('overridden by step:', collections.Counter(o[2] for o in overridden))
for o in overridden:
    print('  ', o[0][:40], o[1], o[2], '|', o[4][:150])
for t, hs in theme_hits.items():
    print('THEME', t, collections.Counter(h[0] for h in hs))
