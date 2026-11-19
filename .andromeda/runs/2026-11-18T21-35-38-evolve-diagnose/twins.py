import json, re, statistics
C = '.andromeda/cache/evolve-diagnose/2026-11-18T21-35-38'
A = json.load(open(C + '/analysis.json', encoding='utf-8'))
recs = [json.loads(l) for l in open(C + '/friction.ndjson', encoding='utf-8')]

def fold_ep(e):
    e = (e or '').replace('–', '-').replace('—', '-')
    return re.sub(r'^\s*#+\s*', '', e).strip()

# chronic-degrade lookback: every tooling.* friction and ok-degraded outcome of 0.3.0
chronic = []
for r in recs:
    m = re.search(r'(\d+\.\d+\.\d+)$', r.get('version') or '')
    if not m or m.group(1) != '0.3.0': continue
    if (r.get('type') or '').startswith('tooling.') or r.get('outcome') == 'ok-degraded':
        chronic.append([fold_ep(r.get('epoch'))[:8], r.get('chunk'), r.get('step'), r.get('type') or r.get('outcome'), (r.get('what') or '')[:220]])

# cascade rows (row runs) + trend
rows, trend = [], {'distill': [], 'reconcile': []}
for c in A['cascade']:
    s = c['signals']
    kinds = sorted(k[7:] for k in s if k.startswith('agents-'))
    if not kinds:
        rows.append(f"{c['chunk']} · {c['step']} · era"); continue
    first = sum(s.get('first-' + k, 0) for k in kinds); agents = sum(s.get('agents-' + k, 0) for k in kinds)
    per = ' '.join(f"{k} {s.get('first-' + k, 0)}/{s.get('agents-' + k, 0)}" for k in kinds)
    resp = s.get('respawn-agents', 0)
    checks = ' '.join(f"{k[8:]} {v}" for k, v in s.items() if k.startswith('respawn-') and k != 'respawn-agents' and k != 'respawn-kept')
    wall = sum(v for k, v in s.items() if k.startswith('wall-'))
    fr = s.get('fit-rejected', '-') if c['step'] == 'reconcile' else '-'
    row = (f"{c['chunk']} · {c['step']} · first try {first}/{agents} ({per}) · re-spawned {resp}" + (f" ({checks})" if checks else '') +
           f" · wall {wall} s · rows: picked {s.get('classify-yes', '-')} of {s.get('entities', '-')} · one draw {s.get('one-draw-yes', '-')} · dead {s.get('draws-dead', '-')} · fit-rejected {fr}")
    if c['step'] == 'reconcile': row += f" · growth flags {s.get('growth-flag', '-')}"
    rows.append(row)
    trend[c['step']].append({'first': first / agents if agents else None, 'one_draw': s.get('one-draw-yes', 0) / s['classify-yes'] if s.get('classify-yes') else None,
                             'growth': s.get('growth-flag') if c['step'] == 'reconcile' else None})
def summ(vals):
    v = [x for x in vals if x is not None]
    if not v: return '-'
    f = lambda x: f"{x:.3f}" if isinstance(x, float) else str(x)
    return f"{f(v[0])} → {f(v[-1])}, min {f(min(v))}, median {f(statistics.median(v))}, max {f(max(v))}"
trend_s = {st: {k: summ([t[k] for t in ts]) for k in ('first', 'one_draw', 'growth')} for st, ts in trend.items()}

# by-check twin for retry.* group at reconcile
by_check = {'wrap-session/reconcile/retry.detector-respawn': {}}
for c in A['cascade']:
    if c['step'] != 'reconcile': continue
    for k, v in c['signals'].items():
        if k.startswith('respawn-') and k not in ('respawn-agents', 'respawn-kept'):
            d = by_check['wrap-session/reconcile/retry.detector-respawn'].setdefault(k, {'n': 0, 'chunks': []})
            d['n'] += v; d['chunks'].append(c['chunk'])

guard_arms = {'stamp-ahead (plan Generated stamp)': 6, 'cd arm': 17, 'pipe-into-filter arm': 10, 'heredoc arm': 3, 'recursive rm (deny rule)': 3,
              'write-guard (harness family Edit)': 1, 'redirect false positive (quoted -->)': 1}
health = {'records': 198, 'step': 133, 'friction': 65, 'coverage': A['coverage'], 'implement_runs': A['implement_runs'],
          'unparseable': 0, 'malformed_ts': A['bad_ts'], 'untyped_rate': A['untyped_rate'], 'problem_fill': A['problem_fill'], 'id_fill': A['id_fill'],
          'signals': A['signal_line'], 'guard_arms_from_notes': guard_arms, 'signals_agg': A['signals_agg'], 'cascade': rows, 'trend': trend_s,
          'retracted_in_epoch': A['retracted_in_epoch'], 'outcomes': A['outcomes']}
json.dump(health, open(C + '/q-health.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
json.dump(by_check, open(C + '/q-by-check.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
json.dump({'anchors': A['anchors'], 'operator_pass': A['operator_pass']}, open(C + '/q-chains.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
json.dump({'facts': A['facts'], 'prev_facts': A['prev_facts'], 'chronic_lookback': chronic, 'deferral_signals': A['deferral_signals']},
          open(C + '/q-level.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('\n'.join(rows)); print(json.dumps(trend_s, ensure_ascii=False)); print(json.dumps(by_check))
for x in chronic: print(x)
