import json, re
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
recs = []
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
EP = 'Epoch 2 - Engine debts'
for r in recs:
    if norm(r.get('epoch')) != EP or r.get('kind') != 'step':
        continue
    st = re.sub('^andromeda-', '', r.get('skill') or '') + '/' + (r.get('step') or '')
    if st in ('wrap-session/report', 'wrap-session/reconcile', 'phase/take-up', 'phase/research', 'implement/code', 'wrap-session/route-resolve'):
        print('\n##', r.get('id'), r.get('chunk'), st, r.get('outcome'))
        for c in r.get('consumed') or []:
            print('   C', c.get('artifact'), c.get('quality'), '|', (c.get('note') or '')[:200])
        for p in r.get('produced') or []:
            print('   P', p.get('artifact'), p.get('signals'), '|', (p.get('note') or '')[:200])
print('\n## base-finder lookback rows (0.2.0 Epoch 1 and 0.1.0 Epoch 3)')
lb = json.load(open('.andromeda/runs/2026-10-18T21-41-48-evolve-diagnose/q-lookback.json', encoding='utf-8'))
for x in lb['base-finder']:
    print(' ', x['ep'], x['id'], x['chunk'], x['step'], x['kind'], x['type'], '|', x['text'][:200])
print('\n## delegate-provisional / override lookback')
for x in lb['delegate-provisional']:
    print(' ', x['ep'], x['id'], x['chunk'], x['step'], x['kind'], x['type'], '|', x['text'][:160])
print('\n## overridden facts, whole ledger')
for r in recs:
    p = r.get('problem')
    p = [] if p is None else (p if isinstance(p, list) else [p])
    for i, f in enumerate(p):
        if isinstance(f, dict) and f.get('solution') == 'overridden':
            print(' ', r.get('version'), norm(r.get('epoch')), r.get('id'), r.get('chunk'), r.get('step'), '|', (f.get('note') or '')[:150])
