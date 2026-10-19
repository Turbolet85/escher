"""Phase 4 (diff + judge) and Phase 5 (render) -> RUN_DIR/proposals.md.

usage: judge_render.py RUN_DIR BASE_JSCPD_REPORT BASE_ROOT
The current record is the ledger's record whose ts equals RUN_DIR/record.json's; the baseline is
the LAST record BEFORE it whose sha == its baseline_sha; monotonic's third record is the
baseline's own baseline_sha, found the same way (audit-pass.md §Ledger I/O). Every evidence
table's row count is asserted against an independent n and its rows unique on their natural key.
"""
import collections
import json
import os
import sys

rd, base_jscpd, base_root = sys.argv[1:4]
R = os.path.relpath(rd)
mine = json.load(open(os.path.join(rd, 'record.json'), encoding='utf-8'))
recs = []
bad = 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        bad += 1
idx = [i for i, r in enumerate(recs) if r['ts'] == mine['ts'] and r['sha'] == mine['sha']]
assert len(idx) == 1, 'own record not found once'
cur = recs[idx[0]]


def before(i, sha):
    m = [k for k in range(i) if recs[k]['sha'] == sha]
    return m[-1] if m else None


bi = before(idx[0], cur['baseline_sha'])
base = recs[bi]
bbi = before(bi, base['baseline_sha'])
bb = recs[bbi]
E = lambda r: r['epoch']


def c(name):
    return json.load(open(os.path.join(rd, f'c-{name}.json'), encoding='utf-8'))


def table(header, rows, n, key=lambda r: tuple(r)):
    assert len(rows) == n, f'{header[0]}: {len(rows)} rows, n={n}'
    assert len({key(r) for r in rows}) == len(rows), f'{header[0]}: rows not unique'
    out = ['| ' + ' | '.join(header) + ' |', '|' + '---|' * len(header)]
    out += ['| ' + ' | '.join(str(x) for x in r) + ' |' for r in rows]
    return '\n'.join(out)


# ---- tool versions: trend-break check ----
tb = [k for k in cur['tool_versions'] if base['tool_versions'].get(k) != cur['tool_versions'][k]]

# ---- monotonic over the closed set of six ----
SIX = [('duplication.pct', lambda r: r['duplication']['pct'], 1),
       ('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling'], 1),
       ('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates'], 1),
       ('sizes.file_max', lambda r: r['sizes']['file_max'], 1),
       ('sizes.over_800', lambda r: r['sizes']['over_800'], 1),
       ('coverage.line', lambda r: r['coverage']['line'], -1)]
mono = []
mono_rows = []
for name, get, sign in SIX:
    v = [get(bb), get(base), get(cur)]
    if None in v:
        mono_rows.append([name, *v, 'not evaluable (null)'])
        continue
    worse = sign * (v[1] - v[0]) > 0 and sign * (v[2] - v[1]) > 0
    mono_rows.append([name, *v, 'FIRES' if worse else 'no'])
    if worse:
        mono.append((name, v))

# ---- single-epoch thresholds (span 1) ----
d_pct = cur['duplication']['pct'] - base['duplication']['pct']
d_rel = d_pct / base['duplication']['pct'] * 100
d_oc = cur['complexity']['over_ceiling'] - base['complexity']['over_ceiling']
d_dead = cur['dead']['zero_ref_candidates'] - base['dead']['zero_ref_candidates']
d_cov = cur['coverage']['line'] - base['coverage']['line']
fires = {
    'new-cycle': cur['graph']['cycles'] > base['graph']['cycles'],
    'duplication-up': d_pct >= 0.5 and d_rel >= 15,
    'complexity-creep': d_oc >= 3 and d_oc >= 0.25 * base['complexity']['over_ceiling'],
    'dead-growth': d_dead >= 5,
    'coverage-drop': d_cov <= -2,
}
assert not any(fires.values()), fires  # rendered below only as below-threshold lines

# ---- mutation: last scored value at EQUAL scope ----
mut_lines = []
for u, s in cur['mutation']['scores'].items():
    t = cur['mutation']['timing'][u]
    last = None
    for k in range(idx[0] - 1, -1, -1):
        if u in recs[k]['mutation'].get('scores', {}):
            last = recs[k]
            break
    scope = t['scope'] + (f" {t['files']}" if t['scope'] == 'files' else f" (planned {t['planned']})")
    if last is None:
        mut_lines.append(f'mutation · `{u}` — {s} at {scope}; no prior `scores` entry under this unit key in the ledger '
                         f'(first score; earlier blitz-dom runs were keyed by sub-label: `blitz-dom/clock`, '
                         f'`blitz-dom/engine-debts`, …) — nothing to compare')
        continue
    lt = last['mutation'].get('timing', {}).get(u)
    lscope = (lt['scope'] + (f" {lt['files']}" if lt['scope'] == 'files' else f" (planned {lt['planned']})")) if lt else 'UNKNOWN'
    equal = lt and lt['scope'] == t['scope'] and (
        (t['scope'] == 'files' and lt.get('files') == t['files']) or
        (t['scope'].startswith('shard') and lt['planned'] == t['planned']))
    if equal:
        mut_lines.append(f'mutation · `{u}` — {last["mutation"]["scores"][u]} → {s}, equal scope {scope}')
    else:
        mut_lines.append(f'mutation · `{u}` — `not comparable`: {last["mutation"]["scores"][u]} at {lscope} '
                         f'({E(last)}, {last["sha"][:9]}) · {s} at {scope} ({E(cur)}) — no delta')

# ---- top-N entrants ----
def entrants(a, b, key=lambda r: r[0]):
    bs = {key(r) for r in b}
    return [r for r in a if key(r) not in bs]


ent_dup = entrants(cur['duplication']['top'], base['duplication']['top'], key=lambda r: tuple(r))
ent_cx = entrants(cur['complexity']['top'], base['complexity']['top'])
ent_sz = entrants(cur['sizes']['top'], base['sizes']['top'])
ent_fi = entrants(cur['graph']['fan_in_top'], base['graph']['fan_in_top'])
ent_dead = entrants(cur['dead']['top'], base['dead']['top'])
ent_hot = entrants(cur['hotspots'], base['hotspots'])
deps_new = entrants(cur['dead']['unused_deps'], base['dead']['unused_deps'], key=lambda r: tuple(r))
deps_gone = entrants(base['dead']['unused_deps'], cur['dead']['unused_deps'], key=lambda r: tuple(r))

# ---- clone-pair delta (evidence for monotonic duplication.pct) ----
root = os.path.abspath('.') + '/'
broot = os.path.abspath(base_root) + '/'


def pairs(p, rt):
    d = json.load(open(p, encoding='utf-8'))
    n, L = collections.Counter(), collections.Counter()
    for x in d['duplicates']:
        k = tuple(sorted((x['firstFile']['name'].replace(rt, ''), x['secondFile']['name'].replace(rt, ''))))
        n[k] += 1
        L[k] += x['lines']
    return n, L


hn, hl = pairs(os.path.join(rd, 'jscpd-abs', 'jscpd-report.json'), root)
bn, bl = pairs(base_jscpd, broot)
pd = sorted([[f'{a} ↔ {b}', bn[(a, b)], hn[(a, b)], bl[(a, b)], hl[(a, b)]]
             for (a, b) in set(hn) | set(bn) if hn[(a, b)] != bn[(a, b)] or hl[(a, b)] != bl[(a, b)]],
            key=lambda r: (-(r[4] - r[3]), r[0]))
pd_net_clones = sum(r[2] - r[1] for r in pd)
pd_net_lines = sum(r[4] - r[3] for r in pd)
assert pd_net_clones == cur['duplication']['clones'] - base['duplication']['clones']
# pair lines are per-fragment sums; jscpd's duplicatedLines counts distinct lines, so the two
# nets differ by design and only the clone count is asserted.

sizes = c('sizes')
over = json.load(open(os.path.join(rd, 'tokei.json'), encoding='utf-8'))
over800 = sorted([[r['name'], r['stats']['code']] for r in over['Rust']['reports'] if r['stats']['code'] > 800],
                 key=lambda r: (-r[1], r[0]))
cx = c('complexity')
churn = c('churn')
dead = c('dead')

L = []
L.append(f"# Code Audit — escher · {E(cur)} · {cur['ts']}")
L.append(f"mode {cur['mode']} · HEAD {cur['sha']} · baseline {cur['baseline_sha']} ({E(base)}) · span {cur['span']}")
L.append('')
L.append(f"Overshoot: this run 0 commits (HEAD is the epoch's boundary, 2026-10-19-recorded-check-replay's complete flip). "
         f"The baseline record's own overshoot is 0 commits, but its note says its boundary state already held this epoch's "
         f"first two chunks (upstream-sync-determinism-and-traces, one-virtual-clock), attributed to {E(base)}; this "
         f"window ({cur['baseline_sha'][:9]}..{cur['sha'][:9]}, 7 commits, 4 touching source) therefore holds "
         f"2026-10-19-session-recording and 2026-10-19-recorded-check-replay only.")
L.append(f"Ancestry: OK. Trend-breaks: {', '.join(tb) if tb else 'none — every tool token equals the baseline record’s'}. "
         f"Ledger: {len(recs)} parseable records after the append, {bad} unparseable.")
L.append('')
L.append('## Proposals')
L.append('')
n = 0
for name, v in mono:
    n += 1
    if name == 'duplication.pct':
        L.append(f'### M{n} — monotonic · duplication.pct — +{v[1] - v[0]:.3f} pt then +{v[2] - v[1]:.2f} pt')
        L.append(f"**Movement:** {v[0]} → {v[1]} → {v[2]} ({E(bb)} → {E(base)} → {E(cur)}); worse at both of the last two "
                 f"diffs. This epoch alone: clones {base['duplication']['clones']} → {cur['duplication']['clones']}, duplicated "
                 f"lines {base['duplication']['duplicated_lines']} → {cur['duplication']['duplicated_lines']}, over a population "
                 f"(`total_lines`) of {base['duplication']['total_lines']} → {cur['duplication']['total_lines']} "
                 f"(+{(cur['duplication']['total_lines'] / base['duplication']['total_lines'] - 1) * 100:.1f} %).")
        L.append(f"**Evidence:** every file pair whose clone count or duplicated lines moved between {cur['baseline_sha'][:9]} "
                 f"and HEAD (jscpd 5.4.0, the record's own command; the baseline side re-run on a git archive of "
                 f"{cur['baseline_sha'][:9]}, reproducing its stored record exactly). Net {pd_net_clones:+d} clones, "
                 f"{pd_net_lines:+d} lines:")
        L.append('')
        L.append(table(['pair', 'clones before', 'clones now', 'lines before', 'lines now'], pd, len(pd)))
        L.append('')
        sp, bs = cur['duplication']['split'], base['duplication']['split']
        L.append(f"Split by path: src {bs['src']['pairs']}→{sp['src']['pairs']} pairs ({bs['src']['lines']}→{sp['src']['lines']} L) · "
                 f"test {bs['test']['pairs']}→{sp['test']['pairs']} ({bs['test']['lines']}→{sp['test']['lines']} L) · "
                 f"mixed {bs['mixed']['pairs']}→{sp['mixed']['pairs']} ({bs['mixed']['lines']}→{sp['mixed']['lines']} L). "
                 f"The previous diff's +0.181 pt belongs to {E(base)} and was judged at its own boundary.")
        L.append("**Suspected shape:** the growth sits in the two chunks' tests and driver code: the largest new pair is "
                 "`stand_replay.rs` re-declaring the stand Counter (its doc names it a deliberate variant whose click adds 2), "
                 "then the `host_log`/`mcp_log` log-check twins gaining a sixth shared fragment, and three small "
                 "`recording.rs`/`replay.rs` fragments shared with `command.rs`/`wire.rs`.")
        L.append("**Proposal:** a direction for the founder's judgment: give the two log checks one shared helper in "
                 "`examples/seven_guis/tests/common/` (six fragments, 95 L now). Look at whether the recording module's "
                 "request-field parsing can reuse `command.rs`/`wire.rs`'s own readers. Keep the Counter variant if a "
                 "self-contained changed app is the point of `stand_replay`, or else derive it from `counter.rs` with a "
                 "step parameter.")
    elif name == 'sizes.over_800':
        L.append(f'### M{n} — monotonic · sizes.over_800 — +{v[1] - v[0]} then +{v[2] - v[1]}')
        L.append(f"**Movement:** {v[0]} → {v[1]} → {v[2]} files over 800 code lines ({E(bb)} → {E(base)} → {E(cur)}). "
                 f"`file_max` held at {cur['sizes']['file_max']} (`packages/blitz-dom/src/document.rs`).")
        L.append(f"**Evidence:** entrants by diff (tokei 14.0.0, code lines, the record's population; the {bb['sha'][:9]} and "
                 f"{base['sha'][:9]} sides re-run on git archives and reproducing their stored 14 and 15): {E(base)} — "
                 f"`packages/blitz-dom/src/events/pointer.rs` 820; {E(cur)} — `packages/escher-driver/src/cli.rs` 683 → 876 "
                 f"(+269/−55 lines in 2026-10-19-session-recording) and `packages/escher-driver/src/recording.rs` new at 831. "
                 f"None left the set. The {len(over800)} files over 800 now:")
        L.append('')
        L.append(table(['file', 'code lines'], over800, cur['sizes']['over_800']))
        L.append('')
        L.append("**Suspected shape:** both escher entrants are about half inline test module. `cli.rs`'s "
                 "`#[cfg(test)] mod tests` runs from line 476 to its end at 1,032. `recording.rs`'s `off_unix_tests` and "
                 "`tests` modules run from line 453 to its end at 981. `cli.rs::read` also rose from cognitive 17 to 20 "
                 "(still the crate's highest).")
        L.append("**Proposal:** a direction for the founder's judgment: move the two inline test modules out to sibling "
                 "files, along the seam 0.2.0's «File splits» chunk used for `document.rs`, `mcp.rs` and `schema.rs`. "
                 "Each source body would then fall under 500 lines. `cli.rs::read`'s argv branches could take named "
                 "per-verb helpers before the next verb lands.")
    L.append('')
assert n == len(mono)

L.append('## Informational')
L.append('')
info = []
info += mut_lines
info.append(f"top-N entrant · duplication top: {', '.join(f'`{a}` ↔ `{b}` {k} L' for a, b, k in ent_dup) or 'none'}")
info.append(f"top-N entrant · hotspots: {len(ent_hot)} of {len(cur['hotspots'])} new: "
            + ', '.join(f'`{p}` {s:g}' for p, s in ent_hot)
            + f" (the window holds {churn['commits_in_window']} source commits; the baseline's ten were nine blitz-dom files and `packages/blitz-vibey-script/src/runtime.rs`)")
info.append(f"top-N entrant · complexity top: {ent_cx or 'none'} · sizes top: {ent_sz or 'none'} · dead top: "
            f"{ent_dead or 'none'}")
info.append(f"top-N entrant · fan-in top: {', '.join(f'`{s}` {k}' for s, k in ent_fi) or 'none'}")
info.append(f"count-under-ratio — not met: clones +{cur['duplication']['clones'] - base['duplication']['clones']} and "
            f"duplicated_lines +{cur['duplication']['duplicated_lines'] - base['duplication']['duplicated_lines']} rose "
            f"with `pct` rising too (+{d_pct:.2f} pt); the movement is M1's")
for x in info:
    L.append(f'- {x}')
L.append('')

L.append('## Below threshold — no action')
L.append('')
bt = [
    f"duplication-up · pct {base['duplication']['pct']} → {cur['duplication']['pct']} (+{d_pct:.2f} pt, +{d_rel:.1f} % rel; "
    f"fires at +0.5 pt and +15 %)",
    f"complexity-creep · over_ceiling {base['complexity']['over_ceiling']} → {cur['complexity']['over_ceiling']}; p50/p90 "
    f"cyclomatic {cur['complexity']['cyclomatic_p50']}/{cur['complexity']['cyclomatic_p90']} and cognitive "
    f"{cur['complexity']['cognitive_p50']}/{cur['complexity']['cognitive_p90']} unchanged; max unchanged "
    f"(`compute_inline_layout_inner` 139); two standing offenders rose: `cli.rs::read` 17 → 20, `mcp.rs::run` 16 → 18",
    f"dead-growth · zero_ref_candidates {base['dead']['zero_ref_candidates']} → {cur['dead']['zero_ref_candidates']} "
    f"({d_dead:+d}; none of the {cur['dead']['zero_ref_candidates']} sits in a file the window touched); unused deps "
    f"{len(base['dead']['unused_deps'])} → {len(cur['dead']['unused_deps'])} (new {deps_new or 'none'}, gone {deps_gone or 'none'})",
    f"coverage-drop · line {base['coverage']['line']} → {cur['coverage']['line']} ({d_cov:+.2f} pt; a rise is never "
    f"celebrated); branch null at both (llvm-cov reports 0/0 branches)",
    f"new-cycle · cycles {base['graph']['cycles']} → {cur['graph']['cycles']}; cross-unit edges "
    f"{base['graph']['cross_unit_edges']} → {cur['graph']['cross_unit_edges']}; fan-out per unit unchanged"
    + ('' if dict(cur['graph']['fan_out']) == dict(base['graph']['fan_out']) else ' (counts moved)'),
    f"churn · {base['churn']['pct']} % → {cur['churn']['pct']} % ({churn['adds_churned']} of {churn['adds_all']} adds; "
    f"{cur['churn']['files_churned']} files churned; no threshold row applies with the ledger past 2 records)",
    f"sizes · file_p50 {base['sizes']['file_p50']} → {cur['sizes']['file_p50']}, file_p90 {base['sizes']['file_p90']} → "
    f"{cur['sizes']['file_p90']}, file_max {base['sizes']['file_max']} → {cur['sizes']['file_max']}; population "
    f"{base['totals']['files']} → {cur['totals']['files']} files, {base['totals']['loc']} → {cur['totals']['loc']} code "
    f"lines (a population, never a movement)",
]
for name, b1, b2, c3, verdict in mono_rows:
    if verdict == 'no':
        bt.append(f"monotonic · {name} {b1} → {b2} → {c3} — not worse at both diffs")
    elif verdict.startswith('not'):
        bt.append(f"monotonic · {name} — {verdict}")
assert sum(1 for r in mono_rows) == 6
for x in bt:
    L.append(f'- {x}')
L.append('')

L.append('## Skips')
L.append('')
for s in cur['skips']:
    L.append(f"- {s['metric']} — {s['reason']}")
L.append('')
L.append(f"Mutation summary (host {cur['mutation']['host']}, score = {cur['mutation']['score_formula']}): "
         + ' · '.join(f"`{u}` {cur['mutation']['unit_states'][u]}, {cur['mutation']['counts'][u]['caught']} caught, "
                      f"{cur['mutation']['counts'][u]['missed']} missed, {cur['mutation']['counts'][u]['unviable']} unviable, "
                      f"{cur['mutation']['counts'][u]['timeout']} timeout"
                      for u in cur['mutation']['unit_states'])
         + f". Survivors: {len(cur['mutation']['survivors'])} (= Σ counts.missed "
         f"{sum(v['missed'] for v in cur['mutation']['counts'].values())}). Not measured on this host: "
         f"{len(cur['mutation']['not_measured'])} (= Σ counts.not_measured "
         f"{sum(v['not_measured'] for v in cur['mutation']['counts'].values())}); project union verdict: by pointer, "
         f"`.andromeda/test-plan.md:350` (§10, the «Mutation tier» line naming cargo-mutants), not fetched.")
assert len(cur['mutation']['survivors']) == sum(v['missed'] for v in cur['mutation']['counts'].values())
L.append('')
L.append(f"Evidence twins: `{R}/c-*.json`; ledger record appended to `.andromeda/code-metrics.ndjson` "
         f"(ts {cur['ts']}).")
open(os.path.join(rd, 'proposals.md'), 'w', encoding='utf-8').write('\n'.join(L) + '\n')
print(json.dumps({'proposals': len(mono), 'informational': len(info), 'below': len(bt), 'skips': len(cur['skips']),
                  'fires': fires, 'trend_breaks': tb}))
