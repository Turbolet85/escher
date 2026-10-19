import json
RUN = '.andromeda/runs/2026-10-18T21-41-48-evolve-diagnose/'
typed = json.load(open(RUN + 'q-typed.json', encoding='utf-8'))
print('=== ALL FRICTION CASES BY GROUP (per-step groups; then universal type-alone groups) ===')
for g in typed:
    print(f"\n## {'ABOVE ' if g['above'] else ''}{g['key']} n={g['n']} w={g['weight']} rate={g['rate']}")
    if g['key'].startswith('*/*/') and not g['above']:
        continue
    for c in g['cases']:
        print(' -', c['id'], c['chunk'], '| impact', json.dumps(c['impact']), '| ev', c['evidence'], '| art', c['artifacts'])
        print('   WHAT:', c['what'])
        if c['clause']:
            print('   CLAUSE:', c['clause'])
u = json.load(open(RUN + 'q-untyped-raw.json', encoding='utf-8'))
print('\n=== UNTYPED (epoch) ===')
for x in u['epoch']:
    print(' -', x)
print('\n=== UNTYPED (0.2.0 Epoch 1, lookback) ===')
for x in u['previous_epoch_untyped']:
    print(' -', x['id'], x['skill'], x['step'], '|', x['what'])
print('\n=== CHAIN ANCHORS ===')
for a in json.load(open(RUN + 'q-chains.json', encoding='utf-8')):
    print(' -', json.dumps(a, ensure_ascii=False))
print('\n=== PROBLEM FACTS in epoch ===')
facts = json.load(open(RUN + 'q-level-raw.json', encoding='utf-8'))
for f in facts:
    if f['in_epoch']:
        print(' -', f['id'], f['idx'], f['chunk'], f['step'], f['nature'], f['solution'], '|', f['note'])
