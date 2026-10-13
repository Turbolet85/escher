# Code Audit — escher · Epoch 5 — Agent surfaces · 2026-10-13T09:02:07Z
mode trend · HEAD 495067f8 · baseline 72149121 (Epoch 4 — Driver core) · span 1

Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-12-headless-screenshot complete flip); the baseline record's 0 (at its boundary).
Trend-break: `code-graph.py` d0425fb1 → f22becbb (setup upgrade U03, 09f479b8: the rust-analyzer index now runs with every Cargo feature on, falling back to the default features if that fails), so the graph (A4) and the zero-ref half of dead code (A5) carry `trend-break — tool upgrade` and their thresholds are suppressed. Every other tool token is equal to the baseline's.
Mutation tier: not run. Founder's word, 2026-10-13, relayed verbatim by overseer1: «пусть тогда мутантов не стартует, пока я не скажу» ("then don't let the mutants start until I say so"). The run is labelled «Аудит без мутантов, потом разбор» ("audit without mutants, then the review").
Reproducibility: before anything was judged, this run's summarizers re-measured the baseline sha (72149121, extracted by `git archive` into the run dir and since removed) and reproduced its record exactly: sizes 72623 LOC / 338 files, p50/p90/max/over_800 119/537/2659/12, duplication 3.724 % / 302 clones, complexity over_ceiling 74. The twins are `b-sizes.json`, `b-duplication.json` and `b-complexity.json`.

Window: `72149121..495067f8` holds the 61-commit upstream merge (2026-10-10-upstream-sync-agent-surfaces, 0493d26a) next to the epoch's eight chunks. Of the 36 commits that touch a source path, 28 are upstream and 8 are the fork's. Every window metric below includes the upstream code.

## Proposals

None fired. Each check, by name:
- `new-cycle`: the cycle count is 0 → 0.
- `duplication-up`: the percentage fell.
- `complexity-creep`: over_ceiling rose +6 (+8.1 %), under the +25 % floor.
- `dead-growth`: +17, suppressed by the trend-break.
- `coverage-drop`: line coverage rose.
- `mutation-drop`: nothing was scored this run.
- `monotonic`: no tracked scalar worsened at both of the last two diffs.

## Informational

**I1 — dead code, zero-ref candidates · trend-break — tool upgrade · 105 → 122 (+17).** Zero-ref rows went 1679 → 1872 and test-path exclusions 781 → 953. The FP classes are candidate 105 → 122 · derive/attr-invoked 92 → 94 · entry-point 52 → 52 · format-capture 4 → 4 · runtime-invoked (macro-generated binding) 20 → 20 · test-only 58 → 61 · trait-impl (dispatch) 567 → 566. `recipes.dead` was carried verbatim from the baseline record. How much of the +17 the indexer's feature change explains is not measured here: the old script was not re-run, because that would overwrite the HEAD code-graph cache. What is measured: 15 of the 16 stylo_taffy candidates are `WritingModeExt` trait-declaration methods in `packages/stylo_taffy/src/writing_mode.rs`, a file new this epoch from upstream #1090 (2aec6d61). By crate: blitz-dom 49, stylo_taffy 16, blitz-test-harness 12, blitz-traits 10, browser 8, blitz-paint 6, dioxus-native 6, debug_timer 5, blitz-net 2, blitz-shell 2, blitz-vibey-script 2, dioxus-native-dom 2, rdme 1, accesskit_xplat 1. escher-driver, escher-telemetry and seven_guis hold 0. A false positive outside the recipe's classes, measured: `Harness::apple_keybinding` (`packages/blitz-test-harness/src/input.rs:230`) is called only at `packages/escher-driver/src/execute.rs:483` under `#[cfg(target_os = "macos")]`, which the Linux index never compiles. The other 11 blitz-test-harness candidates have no caller outside the crate by a text search. Unused dependencies are unchanged: 25 → 25, the same list.

| candidate (escher-built crates: blitz-test-harness · dioxus-native-dom) | site |
|---|---|
| blitz-test-harness harness/impl#[`Harness<D>`]into_inner(). | packages/blitz-test-harness/src/harness.rs:126 |
| blitz-test-harness harness/impl#[`Harness<D>`]tick(). | packages/blitz-test-harness/src/harness.rs:152 |
| blitz-test-harness harness/impl#[`Harness<D>`]set_viewport_size(). | packages/blitz-test-harness/src/harness.rs:205 |
| blitz-test-harness input/impl#[`Harness<D>`]mouse_down_at(). | packages/blitz-test-harness/src/input.rs:112 |
| blitz-test-harness input/impl#[`Harness<D>`]mouse_up_at(). | packages/blitz-test-harness/src/input.rs:118 |
| blitz-test-harness input/impl#[`Harness<D>`]drag(). | packages/blitz-test-harness/src/input.rs:139 |
| blitz-test-harness input/impl#[`Harness<D>`]tap(). | packages/blitz-test-harness/src/input.rs:153 |
| blitz-test-harness input/impl#[`Harness<D>`]touch_down(). | packages/blitz-test-harness/src/input.rs:167 |
| blitz-test-harness input/impl#[`Harness<D>`]touch_move(). | packages/blitz-test-harness/src/input.rs:173 |
| blitz-test-harness input/impl#[`Harness<D>`]touch_up(). | packages/blitz-test-harness/src/input.rs:179 |
| blitz-test-harness input/impl#[`Harness<D>`]apple_keybinding(). | packages/blitz-test-harness/src/input.rs:230 |
| blitz-test-harness input/impl#[`Harness<D>`]ime(). | packages/blitz-test-harness/src/input.rs:248 |
| dioxus-native-dom events/impl#[NodeHandle]doc(). | packages/dioxus-native-dom/src/events.rs:150 |
| dioxus-native-dom events/impl#[NodeHandle]node_mut(). | packages/dioxus-native-dom/src/events.rs:171 |

**I2 — dependency graph · trend-break — tool upgrade.** Cycles 0 → 0; cross-unit edges 94 → 96; fan-out moves: browser 8 → 9, blitz-test-harness 4 → 5. The fan-in top-20 has these entrants: `blitz-traits shell/Viewport#` (129), `blitz-traits shell/ColorScheme#` (129), `blitz-html html_sink/HtmlProvider#` (112).

**I3 — count-under-ratio · duplication.** Clones went 302 → 340 (+38) and duplicated lines 3355 → 3870 (+515). Meanwhile pct went 3.724 → 3.646 (-0.078 pt) over a population (`total_lines`) of 90088 → 106157 (+17.84 %). By path: src 164/1702 L → 168/1761 L · test 131/1553 L → 163/1979 L · mixed 7/100 L → 9/130 L, so the growth is mostly test (+32 pairs). The top standing pair is `tests/blitz-tests/tests/device_coalescing.rs` ↔ `tests/blitz-tests/tests/resize_restyle.rs`, 43 L, which entered at Epoch 1 — Foundation.

**I4 — top-N entrants.**
- Duplication top-10: `tests/blitz-tests/tests/stand_mcp_masked.rs` ↔ `tests/blitz-tests/tests/stand_mcp_tools.rs` 31 L (test, this epoch's MCP checks).
- Sizes top-10: `packages/escher-driver/src/mcp.rs` 1140 (a file new this epoch). It displaced `packages/stylo_taffy/src/convert.rs` (873 → 909, still over 800).
- Complexity top-10: `build_inline_layout_into` (packages/blitz-dom/src/layout/construct.rs:1036) 58. Standing members that grew: `resolved_style_value` 104 → 117, `hit_inner` 63 → 70 (its file, `node.rs`, took 2 upstream and 2 fork commits this window), `dispatch_child_layout` 64 → 68.
- Hotspots top-10: 10 of 10 are entrants. By the origin of their window commits, 6 are upstream-only, 3 mixed and 1 fork-only:

| file | score | commits | upstream | fork |
|---|---|---|---|---|
| packages/blitz-dom/src/layout/inline.rs | 903 | 7 | 7 | 0 |
| packages/blitz-dom/src/resolved_style.rs | 351 | 3 | 3 | 0 |
| packages/blitz-dom/src/layout/mod.rs | 340 | 5 | 5 | 0 |
| packages/blitz-dom/src/node/node.rs | 210 | 4 | 2 | 2 |
| packages/blitz-dom/src/document.rs | 160 | 6 | 3 | 3 |
| packages/blitz-dom/src/layout/construct.rs | 116 | 2 | 2 | 0 |
| packages/blitz-dom/src/layout/table.rs | 110 | 2 | 2 | 0 |
| packages/blitz-paint/src/text.rs | 96 | 2 | 2 | 0 |
| packages/blitz-dom/src/mutator.rs | 72 | 3 | 2 | 1 |
| packages/escher-driver/src/cli.rs | 68 | 4 | 0 | 4 |

## Below threshold — no action

- complexity-creep: over_ceiling 74 → 80 (+6, +8.1 %; it fires at ≥ +3 AND ≥ +25 %). The percentiles are unchanged (cyclomatic 1/5, cognitive 0/3) and the max is unchanged (synthesize_presentational_hints_for_legacy_attributes 135). Over-ceiling by root: packages/blitz-dom 49, packages/blitz-paint 11, wpt/runner 6, packages/blitz-vibey-script 4, packages/escher-driver 3, apps/browser 2, packages/blitz-shell 2, examples/seven_guis 1, packages/dioxus-native-dom 1, packages/dioxus-native 1. In escher's own crates:

| function | site | cognitive |
|---|---|---|
| tokenize | examples/seven_guis/src/tasks/cells.rs:71 | 22 |
| set_attribute | packages/dioxus-native-dom/src/mutation_writer.rs:240 | 19 |
| read | packages/escher-driver/src/cli.rs:123 | 17 |
| execute | packages/escher-driver/src/execute.rs:140 | 17 |
| run | packages/escher-driver/src/mcp.rs:634 | 16 |

  `cli.rs` and `mcp.rs` are new this epoch; `execute.rs` existed at the baseline.
- sizes: file_max 2659 → 2756 (`packages/blitz-dom/src/document.rs`). over_800 went 12 → 15. Its three entrants are `packages/escher-driver/src/mcp.rs` 1140 (new, fork), `packages/escher-driver/src/schema.rs` 388 → 840 (fork) and `packages/blitz-dom/src/layout/text_transform.rs` 845 (new, upstream). p50 went 119 → 131 and p90 537 → 593. The population was 72623 LOC / 338 files and is now 86414 / 371; a population is not a movement.
- coverage: line 59.21 → 65.08 % (rising is never read as a gain). Branch coverage is null at both records: the coverage leg runs without branch instrumentation, so its TOTAL row's Branches columns read 0 / `-`.
- duplication-up: pct fell 3.724 → 3.646.
- churn: 12.13 → 23.21 %, with 16 → 43 files churned. No threshold exists for churn. 28 of the window's 36 source commits are upstream's.
- mutation-drop: not evaluable. No unit was scored this run (declined), so the last scored values stay those of Epoch 4 — Driver core.

`monotonic` — the six tracked scalars over the chain b7e3a43b (Epoch 3 — Observation model) → 72149121 → 495067f8, by sha:

| scalar | Epoch 3 — Observation model | Epoch 4 — Driver core | Epoch 5 — Agent surfaces | worse at both diffs |
|---|---|---|---|---|
| duplication.pct | 4.05 | 3.724 | 3.646 | no |
| complexity.over_ceiling | 78 | 74 | 80 | no |
| dead.zero_ref_candidates | 107 | 105 | 122 | no |
| sizes.file_max | 2659 | 2659 | 2756 | no |
| sizes.over_800 | 12 | 12 | 15 | no |
| coverage.line | 56.57 | 59.21 | 65.08 | no |

over_ceiling, file_max and over_800 worsened at this diff only. One more worsening at the next boundary fires `monotonic` for each; zero_ref_candidates would too, unless the next diff is also a trend-break.

## Skips

Mutation (C1) — declined on the founder's word (quoted in the header). There are 19 `skips[]` entries: one for each of the 18 units this window touched by its own files, and one covering the other 11 units, which only the root Cargo.lock and Cargo.toml touched, so the whole workspace (29 units) is in scope. The units, with the files the window touched in each (full list in `c-mutation-scope.json`): accesskit_xplat (1), blitz (1), blitz-dom (21), blitz-net (1), blitz-paint (2), blitz-shell (1), blitz-test-harness (3), blitz-tests (28), blitz-vibey-script (11), browser (1), dioxus-native (1), dioxus-native-dom (5), escher-driver (16), rdme (1), seven_guis (13), stylo_taffy (5), wasm_hello (1), wpt (6).

Every other collector ran.
