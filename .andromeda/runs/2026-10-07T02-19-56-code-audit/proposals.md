# Code Audit — escher · Epoch 3 — Observation model · 2026-10-07T02:38:29Z
mode trend · HEAD b7e3a43b · baseline 42b80ad9 (Epoch 2 — Element identity) · span 1

Overshoot: 0 commits. HEAD is the epoch boundary itself, the commit that flipped `2026-10-07-change-tracking-and-diff` to `complete`. The baseline record sat 1 commit past its own boundary with 0 source files in that delta, so nothing of Epoch 2's is carried into this diff.

No ancestry break and no trend-break: all eight tool version tokens equal the baseline record's.

How far the numbers can be trusted: before use, each summarizer was run on an export of the baseline commit and reproduced the Epoch 2 record's stored values exactly (sizes, duplication with its split and top list, complexity, churn, hotspots). The dead-code classing cannot be replayed at the baseline (it needs that commit's code graph); at HEAD it reproduces five of the baseline's seven class counts exactly and its top-10 list symbol for symbol.

This record carries one correction to an earlier record: the Epoch 1 record's `commands.complexity`, as the Epoch 2 record corrected it, still held an unresolved `{run_dir}`; it now names that run's directory.

The ledger now holds 3 records:

| Recorded (UTC) | Epoch | Mode | HEAD | Source files | Rust code lines |
|---|---|---|---|---|---|
| 2026-10-06T05:36:13Z | Epoch 1 — Foundation | baseline | d4113768 | 286 | 59272 |
| 2026-10-06T17:49:21Z | Epoch 2 — Element identity | trend | 42b80ad9 | 291 | 60862 |
| 2026-10-07T02:38:29Z | Epoch 3 — Observation model | trend | b7e3a43b | 302 | 65280 |

## Proposals

### M1 — monotonic · complexity.over_ceiling — 74 → 77 → 78

`over_ceiling` counts functions whose cognitive complexity is above 15. It is one of the six tracked scalars, and it worsened at both of the last two diffs, which is what the `monotonic` check fires on.

**Movement:** 77 → 78 (Epoch 2 — Element identity → Epoch 3 — Observation model), after 74 → 77 (Epoch 1 — Foundation → Epoch 2 — Element identity). The single-epoch rule `complexity-creep` did not fire: it needs +3 and +25%, and this epoch moved +1.

**Evidence:** the ledger stores only each record's top-10, so the functions that crossed the ceiling were found by re-running the same collector (rust-code-analysis 0.0.25) on an export of each record's commit. The recomputed counts equal the stored ones (74, 77, 78). Every crossing over the two diffs:

| Diff | Direction | Function | File | Kind | Cognitive | Origin |
|---|---|---|---|---|---|---|
| Epoch 1 → 2 | in | `walk` | packages/dioxus-native-dom/src/element_id.rs | source | — → 26 | escher, the stable-element-ids chunk |
| Epoch 1 → 2 | in | `inline_fragment_boxes` | packages/blitz-dom/src/node/node.rs | source | — → 22 | upstream merge (Blitz #1060) |
| Epoch 1 → 2 | out | `inline_fragment_rects` | packages/blitz-dom/src/document.rs | source | 29 → 0 | upstream merge (the same change) |
| Epoch 1 → 2 | in | `unkeyed_elements_read_their_component_path` | tests/blitz-tests/tests/stand_element_ids.rs | test | — → 21 | escher, stand check |
| Epoch 1 → 2 | in | `ids_hold_across_a_remount` | tests/blitz-tests/tests/stand_id_persistence.rs | test | — → 17 | escher, stand check |
| Epoch 2 → 3 | in | `every_node_is_one_line_with_its_five_fields` | tests/blitz-tests/tests/stand_snapshot_text.rs | test | — → 16 | escher, stand check |

Split by path, the count of source functions over the ceiling went 74 → 75 → 75 and the count of test functions 0 → 2 → 3. So this epoch's +1 is a test function, and source held level.

Three source functions already over the ceiling grew this epoch, all in `packages/blitz-dom/src/mutator.rs`, where the changed-set marks were added: `add_children_to_parent` 17 → 21, `set_attribute` 34 → 35, `clear_attribute` 24 → 25. They do not move the count.

The complete list of the 78 functions over the ceiling at HEAD is in `c-complexity.json` (`over_ceiling_list`).

**Suspected shape:** over two epochs the scalar was moved by escher's own code (one id-walking source function and three stand-check test functions); the one upstream change in the table moved an over-ceiling body from `document.rs` to `node.rs` and nets to zero. The 70-odd inherited engine functions above the ceiling did not change membership.

**Proposal:** two directions for the founder's judgment. One is to flatten the stand checks that crossed the ceiling: each of the three wraps its whole body in the `for task` × `for incremental` double loop and loops over nodes inside it, so moving the per-task, per-mode body into its own function (or one shared driver for that double loop) would remove the nesting that puts them over. The other is to decide whether test functions belong in this scalar at all: read as source-only it is 74 → 75 → 75 and would not have fired. Separately, `walk` in `element_id.rs` (26) is the one escher-authored source function above the ceiling.

## Informational

- **Duplication top-10, 3 entrants.** `stand_accessibility_ids.rs` ↔ `stand_snapshot.rs` (43 lines) and `stand_snapshot.rs` ↔ `stand_snapshot_text.rs` (43 lines) are the per-task control tables the stand checks each restate. `mutation_writer.rs` ↔ `dioxus_falsy_boolean_attrs.rs` (29 lines) is the 27-name boolean-attribute list, written once in the bridge and once in its test as the test's own oracle. The 3 pairs that left the top-10 still exist, at lower rank.
- **Clones among the stand checks.** Pairs where both sides are a `stand_*.rs` check went from 1 pair (11 lines) at the baseline to 13 pairs (220 lines). The working route proves Epoch 4's driver entries on the same stand tasks.
- **Hotspots top-10, 6 entrants:** `blitz-dom/src/events/keyboard.rs` (50), `blitz-dom/src/mutator.rs` (35), `tests/blitz-tests/tests/stand_snapshot.rs` (26), `dioxus-native-dom/src/snapshot.rs` (21), `dioxus-native-dom/src/mutation_writer.rs` (19), `tests/blitz-tests/tests/stand_id_persistence.rs` (17). Hotspot score is commits touching the file in the window times the file's highest cognitive complexity. The window held no upstream merge this epoch, so the engine files that led the baseline's list (`layout/inline.rs` 387, `node/node.rs` 189) left it, and the list now shows where the epoch's own 6 source-touching commits landed. The highest score is 50, against 387 at the baseline.
- **No entrants** in the complexity top-10, the sizes top-10, the fan-in top-20 or the dead-code candidate top-10.
- **Churn 15.98% → 5.94%.** Churn is the share of added lines that landed in a file's second or later commit inside the window. 307 of 5168 added lines were churn, in 4 files; 262 of them are `snapshot.rs`, which three commits wrote in turn.
- **Mutation score: none this epoch.** The one scoped unit ran out of its 15-minute budget (see Skips), so `mutation-drop` has nothing to read. Its last scored value stays 86.27, from Epoch 2 — Element identity.

## Below threshold — no action

- **Duplication 3.916% → 4.050%** (+0.134 points, +3.4% relative; the rule needs +0.5 points and +15%). Clones 268 → 287, duplicated lines 2976 → 3284, scanned lines 76000 → 81086. By path: source pairs 157 → 160 (lines 1625 → 1666), test pairs 107 → 119 (lines 1299 → 1508), mixed pairs 4 → 8 (lines 52 → 110). The ratio rose with the counts, so this is not the count-under-ratio case.
- **Complexity, single-epoch rule:** over the ceiling 77 → 78 (+1). Percentiles unchanged: cyclomatic p50 1, p90 6; cognitive p50 0, p90 3. The maximum is unchanged (`synthesize_presentational_hints_for_legacy_attributes`, 135).
- **Largest file 2452 → 2659 lines** (`packages/blitz-dom/src/document.rs`, +254 / −10 this epoch, the changed set and its seven in-file unit tests). Files over 800 lines: 12 → 12. File size p50 100 → 110, p90 543 → 543.
- **Dead-code candidates 107 → 107.** 13 of them sit in files this epoch touched; none was added this epoch (all are inherited `BaseDocument` and `View` methods). Unused dependencies 25 → 25, the same list. These are candidates, never proven dead: entry points, trait methods reached by dispatch, derive-invoked functions, macro-generated bindings, test-only helpers and format-string captures are classed out first.
- **Line coverage 54.75% → 56.57%.** A rising number is not evidence of better tests; only a fall is a signal. Branch coverage is not instrumented.
- **Dependency graph:** cycles 0 → 0, cross-crate edges 88 → 88, per-crate fan-out identical.
- **Populations (never a movement):** Rust code lines 60862 → 65280, source files 291 → 302, crates 28 → 28.

The six tracked scalars across the three records. All six were measured at every record, so all were evaluable; only the second row worsened at both diffs.

| Tracked scalar | Epoch 1 — Foundation | Epoch 2 — Element identity | Epoch 3 — Observation model | Worsened at both diffs |
|---|---|---|---|---|
| `duplication.pct` | 3.961 | 3.916 | 4.05 | no |
| `complexity.over_ceiling` | 74 | 77 | 78 | yes |
| `dead.zero_ref_candidates` | 108 | 107 | 107 | no |
| `sizes.file_max` | 2540 | 2452 | 2659 | no |
| `sizes.over_800` | 12 | 12 | 12 | no |
| `coverage.line` | 53.74 | 54.75 | 56.57 | no |

## Skips

- `mutation:dioxus-native-dom` — budget-exhausted. The invocation was stopped at its 15-minute cap with 39 of 57 mutants tested, so by rule it has no score, no counts and no survivor list in the record. It was not re-run, on the operator's word at the confirm. Two measured facts for sizing the next run: the rate was 39 mutants in 15 minutes at `-j 4`, cold builds included; and each mutant's build compiled every `blitz-tests` test target, not only the seven named ones, because the `--cargo-test-arg` filters reach the test phase only.
- `mutation:dioxus-native-dom (actionable.rs, lib.rs, mutation_writer.rs, snapshot.rs, snapshot_diff.rs, snapshot_text.rs — the unit's other touched files, 142 of its 199 touched-file mutants)` — declined at this run's confirm.
- `mutation:blitz-dom` — declined at this run's confirm.
- `mutation:blitz-shell` — declined at this run's confirm.
- `mutation:seven_guis` — declined at this run's confirm.

