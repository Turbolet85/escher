"""Phases 4-5: diff this run's record against the baseline (re-found by baseline_sha, never the last line),
classify per audit-pass.md §Thresholds, render proposals.md per proposal-template.md.

usage: render.py RUN_DIR LEDGER
"""
import json, re, sys

rd, ledger = sys.argv[1:3]
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
recs = [json.loads(l) for l in open(ledger, encoding='utf-8') if l.strip()]
cur = json.load(open(f'{rd}/record.json', encoding='utf-8'))
me = [i for i, r in enumerate(recs) if r['ts'] == cur['ts'] and r['sha'] == cur['sha']]
assert me, 'this run\'s record is not in the ledger'
before = recs[:me[-1]]
B = [r for r in before if r['sha'] == cur['baseline_sha']][-1]
BB = [r for r in before if r['sha'] == B['baseline_sha']][-1]
nrec = len(recs)

props, info, below = [], [], []
fmt = lambda x: 'null' if x is None else (f'{x:g}' if isinstance(x, float) else str(x))


def tb(tool):
    a, b = cur['tool_versions'].get(tool), B['tool_versions'].get(tool)
    return a is not None and b is not None and a.split()[0] != b.split()[0]


# new-cycle
g = cur['graph']['cycles']
(props if g > B['graph']['cycles'] else below).append(('new-cycle', f"graph.cycles {B['graph']['cycles']} → {g}"))
# duplication-up
d0, d1 = B['duplication']['pct'], cur['duplication']['pct']
fire = d1 >= d0 + 0.5 and d1 >= d0 * 1.15
(props if fire else below).append(('duplication-up', f'duplication.pct {d0} → {d1} ({d1 - d0:+.3f} pt; fires at ≥ +0.5 pt AND ≥ +15 %)'))
# complexity-creep
c0, c1 = B['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
fire = c1 >= c0 + 3 and c1 >= c0 * 1.25
(props if fire else below).append(('complexity-creep', f'complexity.over_ceiling {c0} → {c1} ({c1 - c0:+d}; fires at ≥ +3 AND ≥ +25 %)'))
# dead-growth
z0, z1 = B['dead']['zero_ref_candidates'], cur['dead']['zero_ref_candidates']
(props if z1 >= z0 + 5 else below).append(('dead-growth', f'dead.zero_ref_candidates {z0} → {z1} ({z1 - z0:+d}; fires at ≥ +5)'))
# coverage-drop
v0, v1 = B['coverage']['line'], cur['coverage']['line']
(props if v1 <= v0 - 2 else below).append(('coverage-drop', f'coverage.line {v0} → {v1} ({v1 - v0:+.2f} pt; fires at ≤ −2 pt)'))

# mutation-drop / not comparable
def scope_of(rec, key):
    t = rec.get('mutation', {}).get('timing', {}).get(key)
    if not t: return None
    return (t['scope'], tuple(t.get('files') or []), t['planned'] if t['scope'].startswith('shard') else None)


def label(sc):
    if sc is None: return 'UNKNOWN'
    return sc[0] + (f" [{', '.join(sc[1])}]" if sc[1] else '')


for key, s1 in cur['mutation']['scores'].items():
    last = next(((r, r['mutation']['scores'][key]) for r in reversed(before) if key in r.get('mutation', {}).get('scores', {})), None)
    if not last:
        info.append(('mutation', f'{key}: {s1} at {label(scope_of(cur, key))} — no prior scored value')); continue
    r0, s0 = last
    a, b = scope_of(cur, key), scope_of(r0, key)
    if a is None or b is None or a != b:
        info.append(('mutation-drop · not comparable', f"{key}: {s0} at {label(b)} ({r0['epoch']}) · {s1} at {label(a)} ({cur['epoch']}) — no delta"))
    else:
        (props if s1 <= s0 - 10 else below).append(('mutation-drop', f"{key}: {s0} → {s1} at equal scope {label(a)} ({r0['epoch']} → {cur['epoch']}); fires at ≤ −10 pt"))
for key, st in cur['mutation']['unit_states'].items():
    c = cur['mutation']['counts'].get(key)
    if c: info.append(('mutation counts', f"{key}: {st} · caught {c['caught']} · missed {c['missed']} · not measured {c['not_measured']} · timeout {c['timeout']} · unviable {c['unviable']}"))

# monotonic: the six tracked scalars, chain cur ← B ← BB
TRACK = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1),
         ('sizes.file_max', 1), ('sizes.over_800', 1), ('coverage.line', -1)]
get = lambda r, p: r[p.split('.')[0]].get(p.split('.')[1])
for p, sgn in TRACK:
    x2, x1, x0 = get(BB, p), get(B, p), get(cur, p)
    if None in (x2, x1, x0):
        below.append(('monotonic', f'{p}: not evaluable (null at one of the three records)')); continue
    w1, w2 = (x1 - x2) * sgn > 0, (x0 - x1) * sgn > 0
    (props if w1 and w2 else below).append(('monotonic', f"{p}: {fmt(x2)} → {fmt(x1)} → {fmt(x0)} ({BB['epoch']} → {B['epoch']} → {cur['epoch']}){' — worsened at both diffs' if w1 and w2 else ''}"))

# count-under-ratio
for cnt in ('clones', 'duplicated_lines'):
    n0, n1 = B['duplication'][cnt], cur['duplication'][cnt]
    if n1 > n0 and d1 <= d0:
        t0, t1 = B['duplication']['total_lines'], cur['duplication']['total_lines']
        info.append(('count-under-ratio', f"duplication.{cnt} {n0} → {n1} · pct {d0} → {d1} · total_lines {t0} → {t1} ({(t1 - t0) / t0 * 100:+.2f} %) · split {cur['duplication']['split']} · top standing pair {cur['duplication']['top'][0]}"))
    else:
        below.append(('count-under-ratio', f'duplication.{cnt} {n0} → {n1} (no rise; pct {d0} → {d1}) — not applicable'))

# top-N entrants
def entrants(name, a, b, keyf):
    ka = {keyf(x) for x in b}
    new = [x for x in a if keyf(x) not in ka]
    if new: info.append(('top-N entrants', f"{name}: {len(new)} new — " + '; '.join(' · '.join(map(str, x)) for x in new)))
    else: below.append(('top-N entrants', f'{name}: no new entrant'))


entrants('duplication.top (pair + fragment lines)', cur['duplication']['top'], B['duplication']['top'], lambda x: tuple(x))
entrants('complexity.top (fn + file)', cur['complexity']['top'], B['complexity']['top'], lambda x: (x[0], x[1].rsplit(':', 1)[0]))
entrants('sizes.top (file)', cur['sizes']['top'], B['sizes']['top'], lambda x: x[0])
entrants('graph.fan_in_top (symbol)', cur['graph']['fan_in_top'], B['graph']['fan_in_top'], lambda x: x[0])
entrants('hotspots (file)', cur['hotspots'], B['hotspots'], lambda x: x[0])
entrants('dead.top (symbol)', cur['dead']['top'], B['dead']['top'], lambda x: x[0])
info.append(('churn', f"churn {cur['churn']['pct']} % over {cur['churn']['files_churned']} churned file(s) ({B['churn']['pct']} % / {B['churn']['files_churned']} at the baseline) — informational, no threshold"))
for t in sorted(set(cur['tool_versions']) | set(B['tool_versions'])):
    if t not in B['tool_versions']:
        info.append(('tool added', f"{t} {cur['tool_versions'][t]}: new in this record — the mutation runner, which was plain `cargo test` at the baseline; a scored unit's counts are read under a different runner"))
    elif tb(t):
        info.append(('trend-break — tool upgrade', f"{t} {B['tool_versions'][t]} → {cur['tool_versions'].get(t)}"))
# the other movements, below threshold
for p in ('file_p50', 'file_p90', 'file_max', 'over_800'):
    below.append(('sizes', f"sizes.{p} {B['sizes'][p]} → {cur['sizes'][p]}"))
for p in ('loc', 'files'):
    below.append(('totals (population)', f"totals.{p} {B['totals'][p]} → {cur['totals'][p]} — a population, never a movement"))
below.append(('graph', f"cross_unit_edges {B['graph']['cross_unit_edges']} → {cur['graph']['cross_unit_edges']}"))
below.append(('dead', f"unused_deps {len(B['dead']['unused_deps'])} → {len(cur['dead']['unused_deps'])} (lists {'equal' if sorted(map(tuple, B['dead']['unused_deps'])) == sorted(map(tuple, cur['dead']['unused_deps'])) else 'differ'})"))

# ---- render
H = []
H.append(f"# Code Audit — escher · {cur['epoch']} · {cur['ts']}")
H.append(f"mode {cur['mode']} · HEAD {cur['sha']} · baseline {cur['baseline_sha']} ({B['epoch']}, escher-0.1.0) · span {cur['span']}")
H.append('')
H.append(f"Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-16-fast-failing-mutants complete flip); the baseline record's 0 commits.")
H.append(f"Label: «{cur['epoch']}» repeats the ledger's first record's label (escher-0.1.0's Epoch 1, d4113768). This record is escher-0.2.0's Epoch 1, a different boundary, recorded as `trend` against 0.1.0's Epoch 6 at span 1 on the operator's choice at the epoch dialogue — a label join across the two proposal documents needs the sha beside it.")
H.append('No ancestry break; no trend-break (every tool token equal to the baseline\'s; cargo-nextest 0.9.146 is new — Informational).')
H.append('')
H.append('## Proposals')
if not props:
    H.append('None. No threshold fired at this boundary.')
for i, (k, t) in enumerate(props, 1):
    H.append(f'### M{i} — {k}')
    H.append(t)
H.append('')
H.append('## Informational')
for k, t in info:
    H.append(f'- **{k}** — {t}')
surv = cur['mutation']['survivors']
nm = cur['mutation']['not_measured']
n_missed = sum(c['missed'] for c in cur['mutation']['counts'].values())
n_nm = sum(c['not_measured'] for c in cur['mutation']['counts'].values())
assert len(surv) == n_missed and len({tuple(s) for s in surv}) == n_missed
assert len(nm) == n_nm
H.append(f"- **mutation survivors** — {n_missed} (counts.missed summed over scoped units)" + (':' if surv else '.'))
for s in surv: H.append(f'  - `{s[0]}` — {s[1]}')
H.append(f"- **not measured on this host ({cur['mutation']['host']})** — {n_nm}" + (':' if nm else '.'))
for s in nm: H.append(f'  - `{s[0]}` — {s[1]} — {s[2]}')
H.append('  - Project union verdict: none registered — test-plan §10 names cargo-mutants\' host-run tier and no per-leg union verdict.')
H.append('')
H.append('## Below threshold — no action')
for k, t in below:
    H.append(f'- {k} — {t}')
H.append('')
H.append('## Skips')
if not cur['skips']: H.append('None.')
for s in cur['skips']:
    H.append(f"- {s['metric']} — {s['reason']}")
H.append('')
H.append(f'Ledger: {nrec} parseable records after this run\'s append.')
open(f'{rd}/proposals.md', 'w', encoding='utf-8').write('\n'.join(H) + '\n')
print(f'{len(props)} proposals · {len(info)} informational · {len(below)} below threshold · {len(cur["skips"])} skips')
