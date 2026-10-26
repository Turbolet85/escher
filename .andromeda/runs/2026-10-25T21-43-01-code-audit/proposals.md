# Code Audit — escher · Epoch 5 — Polish & ship · 2026-10-26T00:43:30Z
mode trend · HEAD 5e15b78d64c24837e3bab2bc6a1e41a955eeedb6 · baseline cbe90e22413191bb5d38504c62a6ebc93b4361f6 (Epoch 4 — Visual lint and real-app readiness) · span 1
Overshoot: this run 0 commits (HEAD is the boundary, `5e15b78d64`); the baseline's 0 (at its boundary). Ancestry OK. No trend-break: every tool's leading version token equals the baseline's (jscpd 5.4.0, tokei 14.0.0, rust-code-analysis 0.0.25, code-graph.py f22becbb, cargo-machete 0.9.2, cargo-llvm-cov 0.9.1, cargo-mutants 27.1.0, cargo-nextest 0.9.146, rustc 1.99.0).
The baseline's sizes, totals, complexity, duplication and dead-code figures were reproduced exactly by this run's summarizers on a git archive of `cbe90e2241` (dead: same 115 count, same top-10, same 26 unused deps), so every delta below is one population against itself. Ledger: 11 parseable records after this run's append.

## Proposals

### M1 — monotonic · complexity.over_ceiling — 84 → 87 → 88
**Movement:** 84 → 87 → 88 (functions over cognitive 15; Epoch 3 — Determinism and traces → Epoch 4 — Visual lint and real-app readiness → Epoch 5 — Polish & ship) — worse at both of the last two diffs, +3 then +1. Neither single diff fires complexity-creep (≥ +3 AND ≥ +25 %).
**Evidence:** the functions over the ceiling at each record's sha, re-read by rust-code-analysis 0.0.25 on git archives of `7eff48f7aa`, `cbe90e2241` and HEAD, matched by (file, name, occurrence) with the upstream sync's rename `layout/inline.rs → layout/inline/layout.rs` mapped (`c-over-ceiling.json`). Entered / left per diff, the value before in the last column (`—` = the function did not exist in that file):

Epoch 3 — Determinism and traces → Epoch 4 — Visual lint and real-app readiness (84 → 87):

|  | site | fn | cognitive | other side |
|---|---|---|---|---|
| + | `packages/blitz-dom/src/document.rs:1007` | `clear_interaction_state_for_removed_node` | 17.0 | 11.0 |
| + | `packages/blitz-dom/src/mutator.rs:1058` | `<anonymous>` | 22.0 | 0.0 |
| + | `packages/escher-driver/src/lint.rs:247` | `findings` | 16.0 | — |
| + | `packages/escher-driver/src/mcp_tests.rs:399` | `the_listing_is_the_twelve_verbs_of_the_table` | 18.0 | — |
| − | `packages/blitz-dom/src/mutator.rs:998` | `<anonymous>` | 19.0 | 0.0 |

Epoch 4 — Visual lint and real-app readiness → Epoch 5 — Polish & ship (87 → 88):

|  | site | fn | cognitive | other side |
|---|---|---|---|---|
| + | `packages/blitz-dom/src/layout/inline/build.rs:23` | `build_inline_layout_into` | 58.0 | — |
| + | `packages/blitz-dom/src/layout/inline/build.rs:119` | `build_inline_layout_recursive` | 51.0 | — |
| + | `tests/blitz-tests/tests/stand_requirement_sweep.rs:1016` | `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` | 18.0 | — |
| − | `packages/blitz-dom/src/layout/construct.rs:1055` | `build_inline_layout_into` | 58.0 | — |
| − | `packages/blitz-dom/src/layout/construct.rs:1151` | `build_inline_layout_recursive` | 51.0 | — |

Read net of moves: Epoch 3 → 4 is three real crossings — `clear_interaction_state_for_removed_node` 11 → 17 (blitz-dom `document.rs`), and two functions new above it, `escher-driver` `lint.rs::findings` (16) and the `mcp_tests.rs` listing test (18); the `mutator.rs` `<anonymous>` pair is one closure re-keyed by a line move (19 → 22, over the ceiling on both sides). Epoch 4 → 5 is one real crossing — the `stand_requirement_sweep.rs` test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18, a new file section of +482 lines); `build_inline_layout_into` (58) and `build_inline_layout_recursive` (51) moved unchanged from `layout/construct.rs` to `layout/inline/build.rs` in the upstream sync's split and are no growth.

**Suspected shape:** the creep is small crossings just over the ceiling (16–18), half of them in test functions (`mcp_tests.rs`, `stand_requirement_sweep.rs`) that assert a table row by row inline; the upstream engine's large offenders (max `compute_inline_layout_inner` 139) hold still.
**Proposal:** a direction for the founder's judgment — where an escher-written function crosses 15, factor its row-by-row assertion or branch ladder into a table walk or a named helper (the two test functions and `lint.rs::findings` are escher's own; `clear_interaction_state_for_removed_node` is a fork change to an engine function); whether test functions belong in this population at all is a measurement question the ledger's recipe does not settle today.

## Informational

- top-N entrants · complexity.top — 3: `compute_inline_layout_inner` `packages/blitz-dom/src/layout/inline/layout.rs:230` (139.0); `build_inline_layout_into` `packages/blitz-dom/src/layout/inline/build.rs:23` (58.0); `compute_replaced_layout` `packages/blitz-dom/src/layout/replaced.rs:52` (56.0) — the first two by the upstream sync's file moves (`layout/inline.rs` → `layout/inline/layout.rs`, `build_inline_layout_into` out of `layout/construct.rs`), values unchanged; `compute_replaced_layout` 55 → 56 (an upstream edit of `layout/replaced.rs`, +13/−4)
- top-N entrants · sizes.top — 3: `tests/blitz-tests/tests/stand_requirement_sweep.rs` (1007); `packages/blitz-dom/src/layout/inline/layout.rs` (927); `packages/blitz-dom/src/node/element.rs` (927) — `stand_requirement_sweep.rs` 553 → 1007 (the 2026-10-24-stand-requirement-sweep chunk, +482/−5; the one new file over 800, so `sizes.over_800` 17 → 18); `layout/inline/layout.rs` the moved `layout/inline.rs` (925 → 927); `node/element.rs` 916 → 927 (an upstream edit, +17)
- top-N entrants · duplication.top — 1: `packages/blitz-dom/src/layout/inline/layout.rs` × `packages/blitz-dom/src/layout/inline/layout.rs` (29 L)
- top-N entrants · hotspots — 7: `packages/blitz-dom/src/layout/inline/layout.rs` (139.0); `packages/blitz-dom/src/layout/mod.rs` (68.0); `packages/blitz-dom/src/layout/inline/build.rs` (58.0); `packages/blitz-dom/src/layout/replaced.rs` (56.0); `apps/browser/src/toolbar.rs` (41.0); `packages/blitz-dom/src/layout/construct.rs` (37.0); `wpt/runner/src/main.rs` (37.0) — the window's 3 source commits are the operator pre-CI commits; the entrants are the upstream sync's layout split and its browser/wpt edits
- mutation · blitz-dom — not comparable: 100.0 at scope in-diff (this window's diff, 70 planned, Epoch 5 — Polish & ship) · 100.0 at scope in-diff (180 planned, Epoch 4 — Visual lint and real-app readiness) — two different windows' diffs, and `in-diff` is none of the schema's three scopes, so no delta
- mutation · wpt — no prior score — first scored at this boundary: 0.0 at scope in-diff (7 planned)
- mutation · seven_guis — no score: unviable-dominant 12/15
- churn · 8.2 % over 3 source commits, 3 files churned (baseline 13.7 %, 21) — the window's only `.rs`-touching commits are its three operator pre-CI commits

### Mutation tier — complete per unit

**blitz-dom** — complete 70/70 (total_mutants/mutants.json 70/70) · score 100.0 (caught/(caught+missed)) · counts {"mutants": 70, "caught": 53, "missed": 0, "not_measured": 0, "timeout": 0, "unviable": 17} · wall 3041 s of cap 5263 s · jobs 2 · scope in-diff · the founder's «Только дифф, 70 мутантов (Рекомендую)»

**seven_guis** — unviable-dominant 12/15 · score none (caught/(caught+missed)) · counts {"mutants": 15, "caught": 3, "missed": 0, "not_measured": 0, "timeout": 0, "unviable": 12} · wall 297 (not in `mutation.timing`: unviable-dominant) s of cap 1800 s · jobs 2 · scope in-diff · the founder's «Только дифф, 15 мутантов (Рекомендую)»

**wpt** — complete 7/7 (total_mutants/mutants.json 7/7) · score 0.0 (caught/(caught+missed)) · counts {"mutants": 7, "caught": 0, "missed": 5, "not_measured": 0, "timeout": 0, "unviable": 2} · wall 89 s of cap 1800 s · jobs 2 · scope in-diff · the founder's «Только дифф, 7 мутантов (Рекомендую)»

Survivors (5):

| site | mutation |
|---|---|
| `wpt/runner/src/test_runners/attr_test.rs:136:5` | replace check_node_layout -> Vec<String> with vec![] |
| `wpt/runner/src/test_runners/attr_test.rs:136:5` | replace check_node_layout -> Vec<String> with vec![String::new()] |
| `wpt/runner/src/test_runners/attr_test.rs:136:5` | replace check_node_layout -> Vec<String> with vec!["xyzzy".into()] |
| `wpt/runner/src/test_runners/attr_test.rs:164:25` | delete match arm "data-expected-width" in check_node_layout |
| `wpt/runner/src/test_runners/attr_test.rs:165:25` | delete match arm "data-expected-height" in check_node_layout |


## Below threshold — no action

- new-cycle · graph.cycles — 0 → 0
- duplication-up · duplication.pct — 3.76 → 3.8 (+0.04)
- complexity-creep · complexity.over_ceiling — 87 → 88 (+1)
- dead-growth · dead.zero_ref_candidates — 115 → 115 (+0)
- coverage-drop · coverage.line — 68.67 → 68.73 (+0.06)
- movement · sizes.file_max — 2784 → 2784
- movement · sizes.over_800 — 17 → 18
- movement · sizes.file_p50 — 143 → 144
- movement · sizes.file_p90 — 556 → 576
- movement · complexity.cyclomatic_p90 — 5.0 → 5.0
- movement · complexity.cognitive_p90 — 3.0 → 3.0
- movement · graph.cross_unit_edges — 97 → 97
- movement · churn.pct — 13.7 → 8.2
- movement · churn.files_churned — 21 → 3
- monotonic · duplication.pct — 3.88 → 3.76 → 3.8 (not worse at both diffs)
- monotonic · dead.zero_ref_candidates — 116 → 115 → 115 (not worse at both diffs)
- monotonic · sizes.file_max — 2680 → 2784 → 2784 (not worse at both diffs)
- monotonic · sizes.over_800 — 17 → 17 → 18 (not worse at both diffs)
- monotonic · coverage.line — 68.42 → 68.67 → 68.73 (not worse at both diffs)
- duplication volume — clones 410 → 417, lines 4632 → 4746, pct 3.76 → 3.8 (rose with its counts, so not count-under-ratio), population +1.55 %; split src 181 → 181 pairs · test 218 → 226 · mixed 11 → 10 — the growth is test-side; the top standing pair `examples/seven_guis/src/tasks/counter.rs` × `tests/blitz-tests/tests/stand_replay.rs` (59 L) stands since Epoch 4
- totals — loc 102363 (baseline 100715), files 429 (423), units 28 — a population, never a movement

## Skips

- mutation:dioxus-native (touched: src/lib.rs +2, upstream; no test target, the windowed launch reached by no headless test; the founder's «Не запускать (Рекомендую)») — declined
- mutation:browser (touched: document_loader.rs, toolbar.rs +1 each and its manifest, upstream; no test on those files; the founder's «Не запускать (Рекомендую)») — declined
- mutation:blitz-examples (touched: examples/multi_doc.rs, a root example; cargo mutants --list reads 0 mutants — no mutable target, a no-op) — declined
- mutation:blitz-tests (only its test files changed in the window: tests/blitz-tests/tests/*; no src) — declined
- mutation:seven_guis (complete 15/15: caught 3, missed 0, timeout 0, unviable 12 — the unviable are compile errors of the replacement (E0599 no `Harness::new`, E0277 no `Default` for `Element`), no disk or lock fault in any log; unscored by the letter) — unviable-dominant
- coverage.branch — not collected by the project's coverage leg (`-` in its TOTAL row), null as at the baseline

## Judgement calls this run made on its own (the founder asleep — the conservative option, named)

- seven_guis is recorded `unviable-dominant` (12 unviable > 3 caught + 0 missed + 0 timeout) and left UNSCORED, as the letter's mechanical rule says, although its 12 unviable logs read genuine compile errors of the replacement (E0599 no `Harness::new`, E0277 no `Default` for `Element`) and no environment fault — the conservative reading: no 100.0 claimed on 3 tested mutants; its counts sit in `mutation.counts` and `skips[]`.
- The three units' scope is recorded `in-diff` (with `in_diff` naming this run's window.diff), the baseline record's own form, though it is none of the schema's three (`unit` · `files` · `shard k/n`); every score is therefore `not comparable` with any prior one — no delta, no rise, no drop is read from blitz-dom's 100.0 against Epoch 4's 100.0.
- wpt ran against its own 28 bin unit tests under cargo-mutants' own unmutated baseline (`Success`), with no hand pass and no `--baseline=skip`, since its tests live in the unit; its 0.0 (0/5) is a first score and is reported as survivors, not judged as a drop.
- `blitz-tests` (test files only in the window) and `blitz-examples` (`examples/multi_doc.rs`, a root example: `--list` reads 0 mutants) were not put to the founder: both are skipped `declined`, the reason word the baseline used for a tests-only unit and the closest the closed set holds for a no-op.
- `Cargo.lock`'s window moves (tokio 1.53.2 → 1.54.1, windows-sys resolutions, mimalloc) are third-party; they mark touched only the units whose manifests moved (browser, wpt), not every workspace member.
- Each test set is the baseline record's set plus only the test targets added since (blitz-dom: hit_test_dropped_nodes, stand_cells_served, stand_load_page; seven_guis: stand_cells_served, stand_load_page); targets the baseline left out (`all`, `telemetry_init_idempotent`, `telemetry_panic_hook`) stay out.
- `CARGO_BUILD_JOBS=4` was kept beside the founder's `-j 2`, `taskset -c 12-15,28-31` and `--timeout 600`, as the baseline's firing form carried it — a cap on each build's parallelism, never a widening of the footprint.
- The monotonic `complexity.over_ceiling` fire is rendered as the letter fires it (88 vs 87 vs 84); its evidence maps the upstream sync's file move and the two functions moved between files as moves, never as growth, and names the one real Epoch 4 → 5 crossing.
- `graph.fan_out` reads the same 22 (unit, n) pairs as the baseline in a different tie order (DuckDB's order of equal counts); recorded in this run's order, read as no movement.
