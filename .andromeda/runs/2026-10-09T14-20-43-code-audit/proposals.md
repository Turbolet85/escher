# Code Audit — escher · Epoch 4 — Driver core · 2026-10-09T18:54:10Z
mode trend · HEAD 72149121de1fa9e58f74099a1c5bdf9d9d8216da · baseline b7e3a43b8540343b8e655dfc700640511ef2b707 (Epoch 3 — Observation model) · span 1
overshoot: this run 0 commits (at the boundary: HEAD is the 2026-10-07-driver-command-spans complete flip) · the baseline record 0 commits
ancestry: the baseline is an ancestor of HEAD · trend-break: none — every tool version token equals the baseline record's
ledger: 4 parseable records, 0 unparseable

Nothing here is applied, queued or gated on. A finding is a reading for the founder; a fix, if any, goes through a route entry or a chunk at his call.

## Proposals
None. No check of the threshold table fires at this boundary: the 6 single-epoch readings and the 6 tracked scalars are listed under **Below threshold**. The one test gap this run found — a surviving mutant — has no threshold rule in trend mode and is listed in full under **Informational**, in the survivors listing.

## Informational

### Mutation scores
Host `x86_64-unknown-linux-gnu` · score = caught/(caught+missed) · every unit's unmutated test set was run by hand first and was green. In every unit each caught mutant built and then failed a test; a mutant that did not build is `unviable` and is outside the score.

| Unit | Scope | Mutants | Caught | Missed | Unviable | Timeout | Score | Wall |
|---|---|---|---|---|---|---|---|---|
| `escher-driver` | whole unit | 213 | 170 | 0 | 43 | 0 | 100.0 | 59 min |
| `escher-telemetry` | whole unit | 33 | 28 | 0 | 5 | 0 | 100.0 | 10 min |
| `dioxus-native-dom` | 2 file(s) | 61 | 53 | 0 | 8 | 0 | 100.0 | 18 min |
| `blitz-test-harness` | 1 file(s) | 22 | 21 | 0 | 1 | 0 | 100.0 | 8 min |
| `dioxus-native-dom/snapshot` | 4 file(s) | 71 | 60 | 1 | 10 | 0 | 98.36 | 20 min |
| `blitz-dom/scrolling` | 4 functions | 91 | 80 | 0 | 11 | 0 | 100.0 | 42 min |
| `blitz-dom/changed-set` | 18 functions | 106 | 102 | 0 | 4 | 0 | 100.0 | 59 min |

- `dioxus-native-dom` covers `packages/dioxus-native-dom/src/dioxus_document.rs`, `packages/dioxus-native-dom/src/element_id.rs`.
- `blitz-test-harness` covers `packages/blitz-test-harness/src/settle.rs`.
- `dioxus-native-dom/snapshot` covers `packages/dioxus-native-dom/src/actionable.rs`, `packages/dioxus-native-dom/src/snapshot.rs`, `packages/dioxus-native-dom/src/snapshot_diff.rs`, `packages/dioxus-native-dom/src/snapshot_text.rs`.
- `blitz-dom/scrolling` covers every listed mutant whose enclosing function holds a line added in b7e3a43b..HEAD: `BaseDocument::containing_block_chain`, `BaseDocument::offset_within`, `BaseDocument::scroll_into_view`, `BaseDocument::visible_region` — in `packages/blitz-dom/src/scrolling.rs`. The exact filter is in the record's `mutation.timing` and `commands` entries for this unit.
- `blitz-dom/changed-set` covers every listed mutant whose enclosing function holds a `changed_nodes` read or write: `BaseDocument::has_changes`, `BaseDocument::snapshot_node_and`, `BaseDocument::take_changed_nodes`, `BaseDocument::apply_generated_text_input_event`, `DocumentMutator<'_>::add_children_to_parent`, `DocumentMutator<'_>::append_text_to_node`, `DocumentMutator<'_>::clear_attribute`, `DocumentMutator<'_>::remove_and_drop_all_children`, `DocumentMutator<'_>::remove_and_drop_node_with`, `DocumentMutator<'_>::remove_custom_widget`, `DocumentMutator<'_>::remove_node`, `DocumentMutator<'_>::remove_style_property`, `DocumentMutator<'_>::remove_sub_document`, `DocumentMutator<'_>::set_attribute`, `DocumentMutator<'_>::set_custom_widget`, `DocumentMutator<'_>::set_node_text`, `DocumentMutator<'_>::set_style_property`, `DocumentMutator<'_>::set_sub_document` — in `packages/blitz-dom/src/document.rs`, `packages/blitz-dom/src/events/keyboard.rs`, `packages/blitz-dom/src/mutator.rs`. The exact filter is in the record's `mutation.timing` and `commands` entries for this unit.

How each score reads against the ledger:
- `escher-driver` — a first score: no earlier record scores this unit, so there is no movement to judge.
- `blitz-test-harness` — a first score: no earlier record scores this unit, so there is no movement to judge.
- `dioxus-native-dom/snapshot` — a first score: no earlier record scores this unit, so there is no movement to judge.
- `blitz-dom/scrolling` — a first score: no earlier record scores this unit, so there is no movement to judge.
- `blitz-dom/changed-set` — a first score: no earlier record scores this unit, so there is no movement to judge.
- `escher-telemetry` — **not comparable**, no delta: 100.0 over the whole unit (33 mutants) at Epoch 4; 88.89 at Epoch 1 over 12 mutants, a scope its record does not settle (its command names a file pattern, which names no list).
- `dioxus-native-dom` — comparable with Epoch 2 at equal scope (the same two files): 86.27 → 100.0. Read under **Below threshold**.

### Survivors — the corrective listing
1 mutant(s) survived across the 7 units: the test set ran green with the mutation in place.

| Site | Mutation | What the run measured about it |
|---|---|---|
| `packages/dioxus-native-dom/src/snapshot.rs:187:9` | `delete match arm "textarea" in value` | The mutation removes the `textarea` arm of the match in `value`, the snapshot's reader of a text-entry control's value, so a textarea takes the match's fallback. The unit's test set ran green with it in place (94 tests unmutated: the crate's lib tests and six stand checks). No test that reads a snapshot names a textarea: the one test file that names one, `tests/blitz-tests/tests/accessibility_roles.rs:134`, asserts its role and reads no snapshot, and `examples/seven_guis` holds none (0 occurrences). A direction, if the founder wants it closed: a minimal-fixture check that types into a textarea and reads its value from the snapshot. |

Not measured on this host (`x86_64-unknown-linux-gnu`): 0 — no missed mutant sits in code this host's build excludes.

### Field-deletion mutants tallied apart
cargo-mutants 27.1.0 applies no name filter to its `delete field … from struct … expression` mutants (measured: a filter that matches nothing still lists them). A function-filtered unit's invocation therefore also tests the field deletions of its files that lie outside its functions. They are outside the unit's counts and score.

| Invocation | Site | Mutation | Outcome |
|---|---|---|---|
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:186:25` | `delete field target from struct ScrollRequest expression in BaseDocument::scroll` | caught |
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:187:25` | `delete field amount from struct ScrollRequest expression in BaseDocument::scroll` | caught |
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:215:17` | `delete field target from struct ScrollRequest expression in BaseDocument::scroll_inner_content_by` | caught |
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:216:17` | `delete field amount from struct ScrollRequest expression in BaseDocument::scroll_inner_content_by` | caught |
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:247:21` | `delete field target from struct ScrollRequest expression in BaseDocument::scroll_inner_content_by` | caught |
| `blitz-dom/scrolling` | `packages/blitz-dom/src/scrolling.rs:248:21` | `delete field amount from struct ScrollRequest expression in BaseDocument::scroll_inner_content_by` | caught |
| `blitz-dom/changed-set` | `packages/blitz-dom/src/document.rs:1345:21` | `delete field family_name from struct parley::fontique::FontInfoOverride expression in BaseDocument::load_resource` | caught |
| `blitz-dom/changed-set` | `packages/blitz-dom/src/document.rs:1346:21` | `delete field weight from struct parley::fontique::FontInfoOverride expression in BaseDocument::load_resource` | caught |
| `blitz-dom/changed-set` | `packages/blitz-dom/src/document.rs:1347:21` | `delete field style from struct parley::fontique::FontInfoOverride expression in BaseDocument::load_resource` | caught |
| `blitz-dom/changed-set` | `packages/blitz-dom/src/mutator.rs:153:13` | `delete field damage from struct style::data::ElementData expression in DocumentMutator<'_>::create_element` | caught |

### Top-list entrants
- **Fan-in** (3 entrants in the top 20): `seven_guis stand/LeanTask#` 123; `blitz-test-harness harness/Harness#` 116; `blitz-test-harness harness/Harness#doc.` 108. All three are test-side: the stand's task type and the headless harness, called from the new driver checks.
- **Hotspots** (8 entrants in the top 10; score = touching commits × the file's highest cognitive complexity): `tests/blitz-tests/tests/stand_act_ids.rs` 24; `packages/escher-driver/src/schema.rs` 22; `examples/seven_guis/src/session_host.rs` 16; `packages/escher-driver/src/command.rs` 16; `packages/escher-driver/src/session.rs` 16; `packages/escher-telemetry/src/format.rs` 14; `packages/blitz-dom/src/scrolling.rs` 13; `packages/blitz-test-harness/src/settle.rs` 13.
- **Duplication top 10** (2 rows absent from the baseline's list, both of them re-entries, not new clones): `examples/custom_widget.rs` ↔ `examples/wgpu_texture/src/dioxus_native.rs` 25 lines (listed at Epoch 1, Epoch 2); `tests/blitz-tests/tests/scrollbar_drag.rs` ↔ `tests/blitz-tests/tests/stale_interaction_state.rs` 24 lines (listed at Epoch 1, Epoch 2). They return because `stand_accessibility_ids.rs` ↔ `stand_snapshot.rs` (43 lines) and `stand_snapshot.rs` ↔ `stand_snapshot_text.rs` (43 lines) left the list: jscpd reports neither pair at HEAD.
- **Complexity top 10**: 0 entrants. **Sizes top 10**: 0 entrants.

### Count under ratio — duplication
Clones 287 → 302 (+15) and duplicated lines 3284 → 3355 (+71) while the ratio fell 4.05 % → 3.724 % (−0.326 pt): the population (jscpd's total lines) grew 81086 → 90088 (+11.1 %). Split by path, pairs/lines — src 160/1666 → 164/1702 · test 119/1508 → 131/1553 · mixed 8/110 → 7/100. Top standing pair: `tests/blitz-tests/tests/device_coalescing.rs` ↔ `tests/blitz-tests/tests/resize_restyle.rs`, 43 lines, listed since the Epoch 1 record (the first record, so its entry predates the ledger).

### Churn
5.94 % → 12.13 % of added source lines landed in a file's second or later touching commit; files churned 4 → 16 of 53 touched, over 8 source-touching commits (1184 of 9761 added lines). The threshold table holds no rule for churn once the ledger has two records, so this is a reading only. The epoch built the driver in successive chunks over the same files:

| File | Touching commits | Churned adds | All adds |
|---|---|---|---|
| `packages/escher-driver/src/execute.rs` | 3 | 321 | 530 |
| `packages/escher-telemetry/src/format.rs` | 2 | 248 | 403 |
| `tests/blitz-tests/tests/session_common/mod.rs` | 3 | 170 | 315 |
| `packages/escher-driver/src/session.rs` | 4 | 119 | 205 |
| `tests/blitz-tests/tests/stand_act_ids.rs` | 2 | 71 | 277 |
| `tests/blitz-tests/tests/stand_act_refused.rs` | 2 | 70 | 225 |
| `packages/escher-driver/src/lib.rs` | 6 | 48 | 80 |
| `packages/escher-telemetry/src/lib.rs` | 2 | 33 | 43 |
| `packages/escher-driver/src/schema.rs` | 2 | 32 | 476 |
| `packages/escher-driver/src/command.rs` | 2 | 25 | 597 |

### Dependency graph
Units 28 → 29; cross-unit edges 88 → 94; cycles 0 → 0. The 6 new edges are all the new unit's: `blitz-tests` → `escher-driver`, `escher-driver` → `blitz-dom`, `escher-driver` → `blitz-test-harness`, `escher-driver` → `blitz-traits`, `escher-driver` → `dioxus-native-dom`, `seven_guis` → `escher-driver`.

### How the mutation tier ran
- Totals: 7 invocations, 607 mutants tested, 0 timeouts, 215 min of mutation wall time summed over the units. Each unit's unmutated pass was run by hand first under the unit's own environment and exited 0; every invocation then ran with `--baseline=skip`.
- Footprint, and why it changed. The first launch (`escher-driver` at `-j 4` with cargo's default build jobs) contended the host: the operator measured load 209 on 32 cores, IO stalled 465 s and 35 linkers at once at 14:39Z, with another builder on the host. It was stopped on his direction. Every unit after that ran at `-j 2`, `CARGO_BUILD_JOBS=4` per cargo, `--jobserver-tasks 6`, `RUST_TEST_THREADS=4` and `--timeout 600` (the earlier records used 120).
- Priority. The two `blitz-dom` units ran with the whole invocation under `nice -n 19 ionice -c 3`, on the founder's word relayed by the operator, because the host was in interactive use. The disk's IO scheduler is `none`, which does not act on the idle IO class, so only the CPU priority is known to have had effect.
- CPU pin. At 17:28Z the operator pinned the running `blitz-dom/scrolling` process tree to CPUs 12-15,28-31 with taskset, at 58 of its 97 mutants tested; the unit was not restarted. Its cap reads the rate measured after the pin (35.3 s per mutant, cap 5136 s). `blitz-dom/changed-set` and its unmutated pass were launched under the same pin from the start.
- Caps. No earlier record holds a per-mutant time, so every unit started under the 1800 s floor and was re-sized once at the floor from its own rate × 1.5. Three were re-sized: `escher-driver` 5277 s (16.52 s per mutant), `blitz-dom/scrolling` 5136 s, `blitz-dom/changed-set` 5211 s (31.58 s per mutant). None passed 2 h and none ran out its cap.
- Stopped launches: 5, all discarded, none read as a result. `escher-driver` twice (the `-j 4` launch at 26 tested; a 30-second launch at 8 jobserver tasks, 0 tested). `blitz-dom/scrolling` three times (33 of 97 tested when the hosting sessions were killed at about 16:54Z; 0 tested at the founder's HOLD; 0 tested at a one-mutant launch that his correction replaced with two). Kept in the run dir under `stopped` names: their status and restart records and their stdout logs. The raw output directories of the three stopped `blitz-dom/scrolling` launches (19 MB together) were moved out of the run dir to the session's scratchpad on the operator's word after the report was first rendered. The raw output of the seven completed invocations was summarized and removed.
- Left on disk outside the run dir: two build copies of the invocation that died at 16:54Z, `target/mutants-tmp/cargo-mutants-escher-cJVhWs.tmp` (13 GB) and `target/mutants-tmp/cargo-mutants-escher-LMO3y5.tmp` (9.1 GB). No process held them when read from /proc at 17:01Z, and no cargo-mutants process was alive after the last unit. They are gitignored and were not deleted, on the operator's word. Every completed invocation removed its own copies.
- CPU share at the reduced footprint, sampled at 16:47Z before the priority and the pin: this run kept 6.5 cores busy on average (peak 13.5) of 32, while the whole host averaged 22.7.
- `dioxus-native-dom` (element_id.rs + dioxus_document.rs): 61 mutants at this run against 57 at Epoch 2, and its test set is three test files wider than the Epoch 2 score's (`stand_id_edits`, `stand_actionable_keys`, `stand_snapshot`). All 7 mutations the Epoch 2 record lists as survivors are caught at this run. Commit `8d156de1` in this epoch's range is titled "seven surviving mutants killed".
- Build cache: the hand passes added per-package test builds to the main `target/`, so the next build there may pay a partial rebuild.

### Measurement notes
- Summarizer check. Before the HEAD numbers were read, this run's summarizers were replayed on the baseline commit's tree. They reproduced the Epoch 3 record exactly for sizes, duplication (ratio, lines, clones, top list and split), complexity (scalars and top list), churn and hotspots.
- The dead-code recipe was not replayed on the baseline tree: it reads the code-graph cache, which is built for HEAD, so the replay ran without it. At HEAD it is applied from the baseline record's `recipes.dead`, carried verbatim into this record.
- Code graph: all 7 queries read `db_state: fresh` at HEAD.
- Duplication split rule: a file under a `tests` directory segment is test; a `*_tests.rs` file under `src/` reads src. This is the rule that reproduces the Epoch 3 record's split (160/1666 · 119/1508 · 8/110) from its own tree.
- Hotspot order: score descending, then path. The last four rows of the top 10 tie at 13 and stand in path order.
- Coverage: the coverage leg exited 0 with 135 test result lines, none with a failure.

### Corrections carried in this record
| Target record | Field | What and why |
|---|---|---|
| `42b80ad9` | `mutation.timing.dioxus-native-dom` | schema-gap fill: the record scores dioxus-native-dom with no `mutation.timing` entry, so the score's scope is read from that record's own `commands.mutation`, whose two `--file` flags name these paths (`{unit}` = dioxus-native-dom) and whose `-j 2` is the jobs; planned and tested are its `counts.mutants` (its unit state reads complete 57/57); wall and cap are not recorded there and stay unknown |

## Below threshold — no action
| Check | Fires when | Baseline → this run | Fires |
|---|---|---|---|
| new-cycle | cycles > baseline | 0 → 0 | no |
| duplication-up | pct ≥ baseline + 0.5 pt and ≥ +15 % | 4.05 % → 3.724 % (−0.326 pt) | no |
| complexity-creep | functions over the ceiling ≥ baseline + 3 and ≥ +25 % | 78 → 74 | no |
| dead-growth | zero-reference candidates ≥ baseline + 5 | 107 → 105 | no |
| coverage-drop | line coverage ≤ baseline − 2 pt | 56.57 % → 59.21 % (+2.64 pt) | no |
| mutation-drop · dioxus-native-dom | score ≤ its last scored value − 10 pt, at equal scope | 86.27 (Epoch 2) → 100.0 (+13.73 pt) | no |

`monotonic` — a tracked scalar worse at both of the last two diffs (Epoch 2 → Epoch 3 → Epoch 4):

| Scalar | Epoch 2 | Epoch 3 | Epoch 4 | Fires |
|---|---|---|---|---|
| `duplication.pct` | 3.916 | 4.05 | 3.724 | no |
| `complexity.over_ceiling` | 77 | 78 | 74 | no |
| `dead.zero_ref_candidates` | 107 | 107 | 105 | no |
| `sizes.file_max` | 2452 | 2659 | 2659 | no |
| `sizes.over_800` | 12 | 12 | 12 | no |
| `coverage.line` | 54.75 | 56.57 | 59.21 | no |

Other movements, for the eye only:
- Source size: 302 → 338 files, 65280 → 72623 code lines. File size p50 110 → 119, p90 543 → 537, largest 2659 → 2659 (`packages/blitz-dom/src/document.rs`).
- Complexity: cyclomatic p90 6 → 5, cognitive p90 3 → 3; the highest stays `synthesize_presentational_hints_for_legacy_attributes` at 135. No function of the two escher crates is over the ceiling of 15.
- Dead-code candidates: unused dependencies 25 → 25, the same list; zero-reference rows 1515 → 1679, of which the test-path exclusion takes 630 → 781. None of the 105 candidates is in `escher-driver` or `escher-telemetry`.
- Coverage: branch coverage is not instrumented by the project's coverage leg (null in every record).

## Skips
| Metric | Reason |
|---|---|
| mutation:seven_guis | declined |
| mutation:blitz-test-harness (harness.rs, input.rs — the unit's other touched files, 70 of its 92 touched-file mutants) | declined |
| mutation:blitz-dom (scrolling.rs outside the four functions this epoch changed — 287 of the file's 378 mutants, 6 of them tested apart as field deletions) | declined |

- `seven_guis` was declined at the first confirm: 23 mutants in its two touched files (`stand.rs`, `session_host.rs`), 238 in the whole unit.
- `blitz-test-harness` was scored on `settle.rs` only (22 mutants), the file this epoch added; `harness.rs` (32) and `input.rs` (38) were declined.
- `blitz-dom` lists 4862 mutants as a whole unit and 378 in `scrolling.rs`, its one touched file. The operator scoped it to the code that is escher's own: the four scrolling functions this epoch changed (91) and the changed-set functions built in Epoch 3 (106). The rest of `scrolling.rs` is inherited upstream code and was declined.
- Two touched units list 0 mutants and are not skips: `blitz-tests` and `blitz-examples`.
- No collector was missing; all eight Tier A/B collectors ran.
