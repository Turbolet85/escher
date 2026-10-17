"""Read-only lookback: friction records of given types across the whole ledger (by version · epoch)."""
import json
import re
import sys


def norm(s):
    s = re.sub(r'^\s*#+\s*', '', s or '').strip()
    return re.sub(r'\s+', ' ', s.replace('–', '-').replace('—', '-'))


types = set(sys.argv[1].split(','))
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        continue
    if r.get('kind') == 'friction' and r.get('type') in types:
        v = r.get('version') or ''
        print('-', v if v.startswith('escher-') else 'escher-' + v, '|', norm(r.get('epoch')), '|', r.get('chunk'),
              re.sub('^andromeda-', '', r.get('skill') or '') + '/' + (r.get('step') or ''), r.get('type'),
              json.dumps(r.get('impact')), '|', (r.get('what') or '')[:200])
