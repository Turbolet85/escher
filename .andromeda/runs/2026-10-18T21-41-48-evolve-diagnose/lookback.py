import json, re, collections
RUN = '.andromeda/runs/2026-10-18T21-41-48-evolve-diagnose/'
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
vnorm = lambda v: (v if (v or '').startswith('escher-') else 'escher-' + (v or ''))
recs = []
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
THEMES = {
    'stamp-ahead': r'ahead of the clock|stamp-ahead|estimated (time|stamp)|hand-typed ts|estimated \(',
    'dialog-hold': r'unanswered|58[- ]min|dialog hold|dialog fallback|went unanswered',
    'gated-arm': r'gated (arm|take-up|wrap|entry)|BLOCKED-ON|--gated',
    'base-finder': r'pre-CI (commit|parent)|oldest pre-CI|base fell|--base|--baseline',
    'cd-cwd': r'\bcd\b',
    'delegate-provisional': r'PROVISIONAL|delegate',
}
out = {}
for name, pat in THEMES.items():
    rows = []
    for r in recs:
        ep = vnorm(r.get('version')) + ' / ' + norm(r.get('epoch'))
        texts = []
        if r.get('kind') == 'friction':
            texts.append(('friction', r.get('type'), r.get('what') or ''))
        p = r.get('problem')
        p = [] if p is None else (p if isinstance(p, list) else [p])
        for i, f in enumerate(p):
            if isinstance(f, dict):
                texts.append(('fact#%d %s/%s' % (i, f.get('nature'), f.get('solution')), None, f.get('note') or ''))
        for kind, typ, t in texts:
            if re.search(pat, t):
                rows.append({'ep': ep, 'id': r.get('id'), 'chunk': r.get('chunk'), 'step': (r.get('skill') or '') + '/' + (r.get('step') or ''),
                             'kind': kind, 'type': typ, 'text': t[:220]})
    out[name] = rows
    print('\n####', name, len(rows), dict(collections.Counter(x['ep'] for x in rows)))
gr = collections.Counter()
grs = collections.Counter()
for r in recs:
    if r.get('kind') != 'step':
        continue
    ep = vnorm(r.get('version')) + ' / ' + norm(r.get('epoch'))
    grs[ep] += 1
    for p in r.get('produced') or []:
        for s in p.get('signals') or []:
            m = re.fullmatch(r'guard-refused:(\d+)', s) if isinstance(s, str) else None
            if m:
                gr[ep] += int(m.group(1))
print('\nGUARD-REFUSED by epoch', {k: (gr[k], grs[k]) for k in grs})
EP = 'escher-0.2.0 / Epoch 2 - Engine debts'
print('\nIn-epoch guard-refused per step:')
for r in recs:
    if r.get('kind') == 'step' and vnorm(r.get('version')) + ' / ' + norm(r.get('epoch')) == EP:
        for p in r.get('produced') or []:
            for s in p.get('signals') or []:
                if isinstance(s, str) and s.startswith('guard-refused'):
                    print(' ', r.get('id'), r.get('chunk'), r.get('skill'), r.get('step'), s, '|', (p.get('note') or '')[:200])
json.dump(out, open(RUN + 'q-lookback.json', 'w'), indent=1, ensure_ascii=False)
for name in ('stamp-ahead', 'dialog-hold'):
    print('\n==', name)
    for x in out[name]:
        print(' ', x['ep'], x['id'], x['step'], x['kind'], x['type'], '|', x['text'][:160])
