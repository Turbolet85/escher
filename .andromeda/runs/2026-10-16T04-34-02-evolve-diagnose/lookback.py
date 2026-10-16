"""Stage-4 / Stage-2 cross-epoch lookback (read-only): prior-epoch problem facts and frictions per theme keyword set."""
import json
import os
import re
import collections

RUN = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(RUN, '..', '..', '..'))
facts = json.load(open(os.path.join(RUN, 'q-facts.json'), encoding='utf-8'))

THEMES = {
    'T1-read-whole': r'read whole|piped into (head|tail|grep)|through (command )?grep|through (head|tail|sed)|grep -v|\| ?tail|\| ?head',
    'T2-long-lines': r'\d+ KB|exact-once|exact-substring|replace script|single lines up to',
    'T3-dialog-expiry': r'58 min|expired|no answer within|dialog returned no answer',
    'T4-grep-function': r'ugrep|bounded repetition|command grep|xargs|/usr/bin/grep',
    'T5-state-dir': r'state-dir-too-long|94 bytes',
    'T6-snapshot-pair': r'snapshot pair|path check|between its snapshots|after-snapshot',
    'T7-clock-estimate': r'estimat\w+ (time|stamp)|ahead of the clock|stamp-ahead|date -u',
    'T8-verbatim-label': r'transliterat|verbatim|Cyrillic|respell',
    'T9-tmpdir': r'\$TMPDIR|TMPDIR',
    'T10-cascade-args': r'16-char|16-character|--pattern',
}


def norm(s):
    return re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')


fric = []
for line in open(os.path.join(ROOT, '.andromeda/friction-log.ndjson'), encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        continue
    if r.get('kind') == 'friction':
        fric.append(r)

out = {}
for name, pat in THEMES.items():
    rx = re.compile(pat, re.I)
    fs = [{'epoch': f['epoch'][:7], 'id': f['id'], 'chunk': f['chunk'], 'step': f['skill'] + '/' + str(f['step']),
           'nature': f['nature'], 'solution': f['solution'], 'note': (f['note'] or '')[:300]}
          for f in facts if rx.search(f['note'] or '')]
    fr = [{'epoch': norm(r.get('epoch'))[:7], 'id': r.get('id'), 'chunk': r.get('chunk'),
           'step': re.sub('^andromeda-', '', r.get('skill') or '') + '/' + str(r.get('step')),
           'type': r.get('type') or 'UNTYPED', 'what': (r.get('what') or '')[:300]}
          for r in fric if rx.search(r.get('what') or '')]
    out[name] = {
        'facts_by_epoch': dict(collections.Counter(x['epoch'] for x in fs)),
        'frictions_by_epoch': dict(collections.Counter(x['epoch'] for x in fr)),
        'facts': fs, 'frictions': fr,
    }

# overrides by epoch with notes (for the override signature)
out['overrides'] = [{'epoch': f['epoch'][:7], 'id': f['id'], 'chunk': f['chunk'], 'step': f['skill'] + '/' + str(f['step']),
                     'note': (f['note'] or '')[:220]} for f in facts if f['solution'] == 'overridden']
# tooling.* frictions per epoch (chronic-degrade)
out['tooling_by_epoch'] = {}
for r in fric:
    t = r.get('type') or ''
    if t.startswith('tooling.'):
        k = norm(r.get('epoch'))[:7]
        out['tooling_by_epoch'].setdefault(t, collections.Counter())[k] += 1
out['tooling_by_epoch'] = {k: dict(v) for k, v in out['tooling_by_epoch'].items()}
json.dump(out, open(os.path.join(RUN, 'q-lookback.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
for name in THEMES:
    print(name, 'facts', out[name]['facts_by_epoch'], 'frictions', out[name]['frictions_by_epoch'])
print('tooling', json.dumps(out['tooling_by_epoch']))
for o in out['overrides']:
    print('OVR', o['epoch'], o['id'], str(o['chunk'])[11:], o['step'], '|', o['note'][:200])
