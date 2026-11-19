import json, re
C = '.andromeda/cache/evolve-diagnose/2026-11-18T21-35-38'
def fold_ep(e):
    e = (e or '').replace('–', '-').replace('—', '-')
    return re.sub(r'^\s*#+\s*', '', e).strip()
def skill(r): return re.sub(r'^andromeda-', '', r.get('skill') or '')
recs = [json.loads(l) for l in open(C + '/friction.ndjson', encoding='utf-8')]
excl = set(json.load(open(C + '/q-retractions.json', encoding='utf-8'))['retracted_ids'])
TYPES = ('contract.mechanical-check', 'contract.cascade-miss', 'contract.false-positive-proposal', 'contract.instrument-validity', 'input.plan-step-ambiguous', 'contract.structural-blind-spot', 'contract.premise-falsified', 'recall.corpus-recurrence')
out = {'prior_typed': [], 'e6_guard_notes': []}
for r in recs:
    ver = re.search(r'(\d+\.\d+\.\d+)$', r.get('version') or '')
    ver = ver.group(1) if ver else ''
    ep = fold_ep(r.get('epoch'))
    if r.get('kind') == 'friction' and r.get('type') in TYPES and r.get('id') not in excl and not (ver == '0.3.0' and ep == 'Epoch 6 - Polish & ship'):
        if ver == '0.3.0' or (ver == '0.2.0' and ep.startswith('Epoch 5')):
            out['prior_typed'].append([ver, ep[:8], r.get('chunk'), f"{skill(r)}/{r.get('step')}", r.get('type'), (r.get('what') or '')[:260]])
    if ver == '0.3.0' and ep == 'Epoch 6 - Polish & ship' and r.get('kind') == 'step':
        for p in r.get('produced') or []:
            if isinstance(p, dict) and any(isinstance(s, str) and s.startswith('guard-refused') for s in p.get('signals') or []):
                out['e6_guard_notes'].append([r.get('chunk'), f"{skill(r)}/{r.get('step')}", [s for s in p['signals'] if s.startswith('guard-refused')], (p.get('note') or '')[:300]])
json.dump(out, open(C + '/q-lookback.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
for k, v in out.items():
    print('==', k)
    for x in v: print(' ', x)
