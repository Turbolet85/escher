# Code Audit — escher · Epoch 2 — Element identity · 2026-10-06T17:49:21Z
mode trend · HEAD 42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca · baseline d4113768df08d45f0c6494f91134031b17ac9023 · span 1
1 commit past the boundary `47bbf38f` (`42b80ad9` chore(session): the founder's PROVISIONAL batch) · source delta files: none — every metric is the boundary state; the baseline record's overshoot: `commits: 0` (at its boundary)
no ancestry break · no trend-break (all eight tool tokens equal the baseline's)

## Proposals
None. No threshold fired at span 1.

## Informational
- count-under-ratio · `duplication.clones` 264 → 268 (+4) and `duplicated_lines` 2935 → 2976 (+41) while `pct` 3.961 → 3.916 (-0.045 pt); population `total_lines` +2.56 % (duplicated lines +1.40 %) · split: src 154→157 pairs / 1595→1625 L · test 106→107 pairs / 1288→1299 L · mixed 4→4 pairs / 52→52 L · top standing pair `tests/blitz-tests/tests/device_coalescing.rs` ↔ `tests/blitz-tests/tests/resize_restyle.rs` 43 L, standing since the Epoch 1 — Foundation baseline record
- top-N entrants · `sizes.top`: `packages/blitz-dom/src/layout/inline.rs` 862 (entered; packages/blitz-dom/src/node/element.rs 850 left the top 10, still > 800) · `duplication.top`, `complexity.top`, `dead.top`, `fan_in_top`: no entrants
- churn · 15.98 % of source adds landed in a file's 2nd..nth touching commit this epoch (10 files churned of 28 touched) — informational while the ledger held < 2 records at read (it held 1 before this append)
- hotspots (first values — the baseline skipped them, `no-baseline`) · `packages/blitz-dom/src/layout/inline.rs` 387 · `packages/blitz-dom/src/node/node.rs` 189 · `packages/blitz-dom/src/document.rs` 96 · `packages/blitz-dom/src/node/text.rs` 66 · `packages/blitz-vibey-script/src/runtime.rs` 56 · `packages/blitz-shell/src/window.rs` 48 · `packages/blitz-paint/src/render.rs` 30 · `packages/dioxus-native-dom/src/element_id.rs` 26 · `packages/stylo_taffy/src/convert.rs` 24 · `tests/blitz-tests/tests/stand_element_ids.rs` 21
- mutation · `dioxus-native-dom` first scored this run: 86.27 % (44 caught / 51; 6 unviable · 0 timeout · 0 not measured on this host; `complete 57/57`) — scope: the epoch's touched files of the unit (57 of the crate's 315 mutants); no prior score, so mutation-drop has no subject
- corrections carried in this record (to the baseline `d4113768`): `commands.complexity` (the recorded form omits the `mkdir -p` its replay needs) · `recipes.dead` (schema-gap fill — the recipe text now rides the ledger)
- C1 invocation note · attempts 1–3 of the mutation tier aborted before testing any mutant (`c1-attempts.txt`): cargo-mutants 27.1.0 runs its UNMUTATED baseline over the mutated package only, ignoring `--test-package` / `--test-workspace` (src/lab.rs `run_baseline`), so the `--test=stand_*` args named targets absent from `dioxus-native-dom`; per-mutant runs honour `--test-package`. Run 4 used `--baseline=skip` (the baseline record's own form) after the unmutated tree was proven green by hand with the same test set — a runner-portability fact, not a test failure. Run 4 was STOPPED at 4 caught / 17 unviable: the unviable builds failed `Disk quota exceeded (os error 122)` on the /tmp tmpfs holding the two build copies — an environment fault presenting as unviable; its partial tally is never read. Run 5 moved the copies to the gitignored `target/mutants-tmp` via TMPDIR and hit the 15-min wall-clock cap at 40/57 (partial, never scored — `budget-exhausted` by letter); the operator chose a re-run under a 30-min cap: run 6 is the scored invocation. Attempt evidence: `c1-attempts.txt`

### Survivors — `dioxus-native-dom` (7)
| site | mutation |
|---|---|
| `packages/dioxus-native-dom/src/dioxus_document.rs:166:9` | replace DioxusDocument::create_head_element with () |
| `packages/dioxus-native-dom/src/dioxus_document.rs:211:9` | replace DioxusDocument::flush_queued_mounted_events with () |
| `packages/dioxus-native-dom/src/dioxus_document.rs:236:9` | replace <impl Document for DioxusDocument>::id -> usize with 0 |
| `packages/dioxus-native-dom/src/dioxus_document.rs:236:9` | replace <impl Document for DioxusDocument>::id -> usize with 1 |
| `packages/dioxus-native-dom/src/dioxus_document.rs:402:12` | delete ! in <impl EventHandler for DioxusEventHandler<'_>>::handle_event |
| `packages/dioxus-native-dom/src/dioxus_document.rs:405:12` | delete ! in <impl EventHandler for DioxusEventHandler<'_>>::handle_event |
| `packages/dioxus-native-dom/src/element_id.rs:212:43` | replace && with || in place_children |

not measured on this host (x86_64-unknown-linux-gnu): 0
no project union verdict

## Below threshold — no action
- duplication-up · `duplication.pct` 3.961 → 3.916 (-0.045 pt) — fell; fires at ≥ +0.5 pt AND ≥ +15 %
- complexity-creep · `complexity.over_ceiling` 74 → 77 (+3, +4.1 %) — the absolute floor (+3) is met, the relative one (+25 %) is not. The +3 equals the three functions over cognitive 15 in files CREATED this epoch: `walk` packages/dioxus-native-dom/src/element_id.rs:72 (26) · `unkeyed_elements_read_their_component_path` tests/blitz-tests/tests/stand_element_ids.rs:171 (21) · `ids_hold_across_a_remount` tests/blitz-tests/tests/stand_id_persistence.rs:189 (17)
- dead-growth · `dead.zero_ref_candidates` 108 → 107 (-1) — same recipe, verbatim (recipes.dead); raw zero-ref rows 1379 → 1415, test-path exclusions 504 → 536, trait-impl 556 → 561; other FP classes unchanged
- coverage-drop · `coverage.line` 53.74 → 54.75 (+1.01 pt) — a rise, never celebrated (trend-only; mutation is the honest half); branch null at both records (llvm-cov reports no branch data)
- mutation-drop · not evaluable: `dioxus-native-dom` has no prior scored value (first scored this run); `escher-telemetry` (scored 88.89 at the baseline) was not touched this epoch and not scoped
- monotonic · not evaluable this run: it needs two diffs (three chained records) — the baseline record `d4113768` has no `baseline_sha`, so the chain holds one diff. Tracked scalars this diff: `duplication.pct` 3.961→3.916 · `complexity.over_ceiling` 74→77 · `dead.zero_ref_candidates` 108→107 · `sizes.file_max` 2540→2452 · `sizes.over_800` 12→12 · `coverage.line` 53.74→54.75
- new-cycle · `graph.cycles` 0 → 0; `cross_unit_edges` 88 → 88; `fan_out` per unit identical; `fan_in_top` same 20 symbols (top: `blitz-traits node_id/NodeId#` 544 → 566)
- sizes · `file_max` 2540 → 2452 (packages/blitz-dom/src/document.rs, −88) · `over_800` 12 → 12 · p50 99 → 100 · p90 546 → 543
- complexity percentiles · cyclomatic p50/p90 1.0/6.0 → 1.0/6.0 · cognitive p50/p90 0.0/3.0 → 0.0/3.0 · `compute_inline_layout_inner` 140 → 129 (no longer the max; max now `synthesize_presentational_hints_for_legacy_attributes` 135, unchanged)
- unused deps (cargo-machete) · 25 → 25 — the identical list
- populations (never movements) · `totals.loc` 59272 → 60862 · `totals.files` 286 → 291 · `duplication.total_lines` 74102 → 76000

## Skips
- mutation:blitz-dom — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:blitz-paint — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:blitz-shell — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:blitz-vibey-script — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:stylo_taffy — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:seven_guis — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:wpt — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
- mutation:dioxus-native — declined (touched this epoch; scope chosen at the attended confirm: the dioxus-native-dom epoch files only)
