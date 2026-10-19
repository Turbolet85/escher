import json, re, collections
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
skips = 0
by = collections.OrderedDict()
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        r = json.loads(line)
    except ValueError:
        skips += 1
        continue
    key = (r.get('version'), norm(r.get('epoch')))
    d = by.setdefault(key, {'step': 0, 'friction': 0, 'chunks': [], 'first': r.get('ts'), 'last': None, 'raw_labels': set()})
    d[r.get('kind')] = d.get(r.get('kind'), 0) + 1
    d['last'] = r.get('ts')
    d['raw_labels'].add(r.get('epoch'))
    c = r.get('chunk')
    if c and c not in d['chunks']:
        d['chunks'].append(c)
print('unparseable', skips)
for k, d in by.items():
    print(k, 'step', d['step'], 'friction', d['friction'], d['first'], d['last'], 'labels', len(d['raw_labels']))
    print('   ', d['chunks'])
