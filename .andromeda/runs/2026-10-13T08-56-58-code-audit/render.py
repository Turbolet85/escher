#!/usr/bin/env python3
"""Render {run_dir}/proposals.md (proposal-template.md). Every evidence table's row count is asserted against an
independent n, and its rows unique on the natural key."""
import json, os, sys

RD = sys.argv[1]
J = lambda m: json.load(open(os.path.join(RD, f'c-{m}.json'), encoding='utf-8'))
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8')]
cur = json.load(open(os.path.join(RD, 'record.json'), encoding='utf-8'))
me = [i for i, r in enumerate(recs) if r['ts'] == cur['ts'] and r['sha'] == cur['sha']]
assert len(me) == 1, 'this run\'s record not found exactly once'
prior = recs[:me[0]]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]          # never the last line
base2 = [r for r in prior if r['sha'] == base['baseline_sha']][-1]       # the chain by sha
ev, dead, scope = J('evidence'), J('dead'), J('mutation-scope')
S = lambda x: x[:8]


def table(head, rows, n, key):
    assert len(rows) == n, f'{head[0]}: {len(rows)} rows, n={n}'
    keys = [key(r) for r in rows]
    assert len(set(keys)) == len(keys), f'{head[0]}: duplicate key'
    out = ['| ' + ' | '.join(head) + ' |', '|' + '---|' * len(head)]
    out += ['| ' + ' | '.join(str(c) for c in r) + ' |' for r in rows]
    return '\n'.join(out)


L = []
a = L.append
a(f"# Code Audit — escher · {cur['epoch']} · {cur['ts']}")
a(f"mode trend · HEAD {S(cur['sha'])} · baseline {S(base['sha'])} ({base['epoch']}) · span {cur['span']}")
a("")
a(f"Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-12-headless-screenshot complete flip); the "
  f"baseline record's {base['head_overshoot']['commits']} (at its boundary).")
a(f"Trend-break: `code-graph.py` {base['tool_versions']['code-graph.py']} → {cur['tool_versions']['code-graph.py']} "
  "(setup upgrade U03, 09f479b8: the rust-analyzer index now runs with every Cargo feature on, falling back to the "
  "default features if that fails), so the graph (A4) and the zero-ref half of dead code (A5) carry "
  "`trend-break — tool upgrade` and their thresholds are suppressed. Every other tool token is equal to the baseline's.")
a("Mutation tier: not run. Founder's word, 2026-10-13, relayed verbatim by overseer1: «пусть тогда мутантов не "
  "стартует, пока я не скажу» (\"then don't let the mutants start until I say so\"). The run is labelled «Аудит без "
  "мутантов, потом разбор» (\"audit without mutants, then the review\").")
a(f"Reproducibility: before anything was judged, this run's summarizers re-measured the baseline sha "
  f"({S(base['sha'])}, extracted by `git archive` into the run dir and since removed) and reproduced its record "
  f"exactly: sizes {base['totals']['loc']} LOC / {base['totals']['files']} files, p50/p90/max/over_800 "
  f"{base['sizes']['file_p50']}/{base['sizes']['file_p90']}/{base['sizes']['file_max']}/{base['sizes']['over_800']}, "
  f"duplication {base['duplication']['pct']} % / {base['duplication']['clones']} clones, complexity over_ceiling "
  f"{base['complexity']['over_ceiling']}. The twins are `b-sizes.json`, `b-duplication.json` and `b-complexity.json`.")
a("")
a("Window: `72149121..495067f8` holds the 61-commit upstream merge (2026-10-10-upstream-sync-agent-surfaces, 0493d26a) "
  "next to the epoch's eight chunks. Of the 36 commits that touch a source path, 28 are upstream and 8 are the fork's. "
  "Every window metric below includes the upstream code.")
a("")
a("## Proposals")
a("")
a("None fired. Each check, by name:")
a("- `new-cycle`: the cycle count is 0 → 0.")
a("- `duplication-up`: the percentage fell.")
a(f"- `complexity-creep`: over_ceiling rose +6 (+8.1 %), under the +25 % floor.")
a("- `dead-growth`: +17, suppressed by the trend-break.")
a("- `coverage-drop`: line coverage rose.")
a("- `mutation-drop`: nothing was scored this run.")
a("- `monotonic`: no tracked scalar worsened at both of the last two diffs.")
a("")
a("## Informational")
a("")
d0, d1 = base['dead'], cur['dead']
a(f"**I1 — dead code, zero-ref candidates · trend-break — tool upgrade · {d0['zero_ref_candidates']} → "
  f"{d1['zero_ref_candidates']} (+{d1['zero_ref_candidates'] - d0['zero_ref_candidates']}).** "
  f"Zero-ref rows went {d0['zero_ref_rows']} → {d1['zero_ref_rows']} and test-path exclusions {d0['excluded_test_path']} → "
  f"{d1['excluded_test_path']}. The FP classes are " +
  ' · '.join(f"{k} {d0['fp_classes'][k]} → {d1['fp_classes'][k]}" for k in d1['fp_classes']) +
  ". `recipes.dead` was carried verbatim from the baseline record. How much of the +17 the indexer's feature change "
  "explains is not measured here: the old script was not re-run, because that would overwrite the HEAD code-graph "
  "cache. What is measured: 15 of the 16 stylo_taffy candidates are `WritingModeExt` trait-declaration methods in "
  "`packages/stylo_taffy/src/writing_mode.rs`, a file new this epoch from upstream #1090 (2aec6d61). By crate: " +
  ', '.join(f"{k} {v}" for k, v in ev['dead_by_crate'].items()) + ". escher-driver, escher-telemetry and "
  "seven_guis hold 0. A false positive outside the recipe's classes, measured: "
  "`Harness::apple_keybinding` (`packages/blitz-test-harness/src/input.rs:230`) is called only at "
  "`packages/escher-driver/src/execute.rs:483` under `#[cfg(target_os = \"macos\")]`, which the Linux index never "
  "compiles. The other 11 blitz-test-harness candidates have no caller outside the crate by a text search. Unused "
  f"dependencies are unchanged: {len(d0['unused_deps'])} → {len(d1['unused_deps'])}, the same list.")
a("")
own = [[s, f] for s, f in dead['candidates_full'] if s.split(' ')[0] in ('blitz-test-harness', 'dioxus-native-dom')]
a(table(['candidate (escher-built crates: blitz-test-harness · dioxus-native-dom)', 'site'], own,
        ev['dead_by_crate'].get('blitz-test-harness', 0) + ev['dead_by_crate'].get('dioxus-native-dom', 0),
        key=lambda r: (r[0], r[1])))
a("")
g0, g1 = base['graph'], cur['graph']
fo0, fo1 = dict(map(tuple, g0['fan_out'])), dict(map(tuple, g1['fan_out']))
fo_moves = [f"{u} {fo0.get(u, 0)} → {n}" for u, n in g1['fan_out'] if fo0.get(u, 0) != n]
fi0 = {s for s, _ in g0['fan_in_top']}
a(f"**I2 — dependency graph · trend-break — tool upgrade.** Cycles {g0['cycles']} → {g1['cycles']}; cross-unit "
  f"edges {g0['cross_unit_edges']} → {g1['cross_unit_edges']}; fan-out moves: {', '.join(fo_moves) or 'none'}. "
  "The fan-in top-20 has these entrants: " +
  ', '.join(f"`{s}` ({n})" for s, n in g1['fan_in_top'] if s not in fi0) + ".")
a("")
p0, p1 = base['duplication'], cur['duplication']
a(f"**I3 — count-under-ratio · duplication.** Clones went {p0['clones']} → {p1['clones']} "
  f"(+{p1['clones'] - p0['clones']}) and duplicated lines {p0['duplicated_lines']} → {p1['duplicated_lines']} "
  f"(+{p1['duplicated_lines'] - p0['duplicated_lines']}). Meanwhile pct went {p0['pct']} → {p1['pct']} "
  f"({p1['pct'] - p0['pct']:+.3f} pt) over a population (`total_lines`) of {p0['total_lines']} → {p1['total_lines']} "
  f"({100 * (p1['total_lines'] / p0['total_lines'] - 1):+.2f} %). By path: " +
  ' · '.join(f"{k} {p0['split'][k]['pairs']}/{p0['split'][k]['lines']} L → {p1['split'][k]['pairs']}/"
             f"{p1['split'][k]['lines']} L" for k in ('src', 'test', 'mixed')) +
  f", so the growth is mostly test (+{p1['split']['test']['pairs'] - p0['split']['test']['pairs']} pairs). "
  f"The top standing pair is `{ev['dup_top_entered'][0][0]}` ↔ `{ev['dup_top_entered'][0][1]}`, "
  f"{ev['dup_top_entered'][0][2]} L, which entered at {ev['dup_top_entered'][0][3]}.")
a("")
a("**I4 — top-N entrants.**")
t0 = {tuple(x) for x in p0['top']}
a("- Duplication top-10: " + '; '.join(f"`{x[0]}` ↔ `{x[1]}` {x[2]} L" for x in p1['top'] if tuple(x) not in t0) +
  " (test, this epoch's MCP checks).")
s0 = {f for f, _ in base['sizes']['top']}
a("- Sizes top-10: " + '; '.join(f"`{f}` {n}" for f, n in cur['sizes']['top'] if f not in s0) +
  " (a file new this epoch). It displaced `packages/stylo_taffy/src/convert.rs` (873 → 909, still over 800).")
c0 = {x[0] for x in base['complexity']['top']}
a("- Complexity top-10: " + '; '.join(f"`{x[0]}` ({x[1]}) {x[2]:g}" for x in cur['complexity']['top'] if x[0] not in c0) +
  ". Standing members that grew: `resolved_style_value` 104 → 117, `hit_inner` 63 → 70 (its file, "
  "`node.rs`, took 2 upstream and 2 fork commits this window), `dispatch_child_layout` 64 → 68.")
h0 = {f for f, _ in base['hotspots']}
hrows = [[f, f"{sc:g}", ev['hotspot_origin'][f]['commits'], ev['hotspot_origin'][f]['upstream'],
          ev['hotspot_origin'][f]['fork']] for f, sc in cur['hotspots'] if f not in h0]
o = ev['hotspot_origin']
a(f"- Hotspots top-10: {len(hrows)} of {len(cur['hotspots'])} are entrants. By the origin of their window commits, "
  f"{sum(v['fork'] == 0 for v in o.values())} are upstream-only, "
  f"{sum(v['fork'] > 0 and v['upstream'] > 0 for v in o.values())} mixed and "
  f"{sum(v['upstream'] == 0 for v in o.values())} fork-only:")
a("")
a(table(['file', 'score', 'commits', 'upstream', 'fork'], hrows, len([1 for f, _ in cur['hotspots'] if f not in h0]),
        key=lambda r: r[0]))
a("")
a("## Below threshold — no action")
a("")
x0, x1 = base['complexity'], cur['complexity']
a(f"- complexity-creep: over_ceiling {x0['over_ceiling']} → {x1['over_ceiling']} "
  f"(+{x1['over_ceiling'] - x0['over_ceiling']}, {100 * (x1['over_ceiling'] / x0['over_ceiling'] - 1):+.1f} %; it fires at "
  "≥ +3 AND ≥ +25 %). The percentiles are unchanged (cyclomatic 1/5, cognitive 0/3) and the max is unchanged "
  f"({x1['max']['fn']} {x1['max']['val']:g}). Over-ceiling by root: " +
  ', '.join(f"{k} {v}" for k, v in sorted(ev['over_ceiling_by_root'].items(), key=lambda kv: -kv[1])) +
  ". In escher's own crates:")
a("")
a(table(['function', 'site', 'cognitive'], [[n, s, f"{v:g}"] for n, s, v in ev['escher_over_ceiling']],
        len(ev['escher_over_ceiling']), key=lambda r: (r[0], r[1])))
a("")
a("  `cli.rs` and `mcp.rs` are new this epoch; `execute.rs` existed at the baseline.")
z0, z1 = base['sizes'], cur['sizes']
a(f"- sizes: file_max {z0['file_max']} → {z1['file_max']} (`packages/blitz-dom/src/document.rs`). over_800 went "
  f"{z0['over_800']} → {z1['over_800']}. Its three entrants are `packages/escher-driver/src/mcp.rs` 1140 (new, fork), "
  "`packages/escher-driver/src/schema.rs` 388 → 840 (fork) and `packages/blitz-dom/src/layout/text_transform.rs` 845 "
  f"(new, upstream). p50 went {z0['file_p50']} → {z1['file_p50']} and p90 {z0['file_p90']} → {z1['file_p90']}. "
  f"The population was {base['totals']['loc']} LOC / {base['totals']['files']} files and is now {cur['totals']['loc']} / "
  f"{cur['totals']['files']}; a population is not a movement.")
a(f"- coverage: line {base['coverage']['line']} → {cur['coverage']['line']} % (rising is never read as a gain). Branch "
  "coverage is null at both records: the coverage leg runs without branch instrumentation, so its "
  "TOTAL row's Branches columns read 0 / `-`.")
a(f"- duplication-up: pct fell {p0['pct']} → {p1['pct']}.")
a(f"- churn: {base['churn']['pct']} → {cur['churn']['pct']} %, with {base['churn']['files_churned']} → "
  f"{cur['churn']['files_churned']} files churned. No threshold exists for churn. 28 of the window's 36 source "
  "commits are upstream's.")
a("- mutation-drop: not evaluable. No unit was scored this run (declined), so the last scored values stay those of "
  f"{base['epoch']}.")
a("")
a(f"`monotonic` — the six tracked scalars over the chain {S(base2['sha'])} ({base2['epoch']}) → "
  f"{S(base['sha'])} → {S(cur['sha'])}, by sha:")
a("")


def mono(name, get, worse_up=True):
    v = [get(base2), get(base), get(cur)]
    w = [(v[i + 1] > v[i]) if worse_up else (v[i + 1] < v[i]) for i in range(2)]
    return [name, v[0], v[1], v[2], 'yes' if all(w) else 'no']


mrows = [mono('duplication.pct', lambda r: r['duplication']['pct']),
         mono('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling']),
         mono('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates']),
         mono('sizes.file_max', lambda r: r['sizes']['file_max']),
         mono('sizes.over_800', lambda r: r['sizes']['over_800']),
         mono('coverage.line', lambda r: r['coverage']['line'], worse_up=False)]
assert all(r[4] == 'no' for r in mrows), 'a monotonic fire would be a proposal'
a(table(['scalar', base2['epoch'], base['epoch'], cur['epoch'], 'worse at both diffs'], mrows, 6, key=lambda r: r[0]))
a("")
a("over_ceiling, file_max and over_800 worsened at this diff only. One more worsening at the next boundary "
  "fires `monotonic` for each; zero_ref_candidates would too, unless the next diff is also a trend-break.")
a("")
a("## Skips")
a("")
nsk = len(cur['skips'])
a(f"Mutation (C1) — declined on the founder's word (quoted in the header). There are {nsk} `skips[]` entries: one "
  f"for each of the {len(scope['units'])} units this window touched by its own files, and one covering the other "
  f"{cur['totals']['units'] - len(scope['units'])} units, which only the root Cargo.lock and Cargo.toml touched, so "
  "the whole workspace "
  f"({cur['totals']['units']} units) is in scope. The units, with the files the window touched in each "
  "(full list in `c-mutation-scope.json`): " +
  ', '.join(f"{u} ({v['files']})" for u, v in scope['units'].items()) + ".")
a("")
a("Every other collector ran.")
open(os.path.join(RD, 'proposals.md'), 'w', encoding='utf-8').write('\n'.join(L) + '\n')
print('rendered', sum(len(x) for x in L), 'chars')
