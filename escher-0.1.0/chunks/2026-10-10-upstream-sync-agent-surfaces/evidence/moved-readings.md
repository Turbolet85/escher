# Moved readings — every number that moved under the merge

Read 2026-10-10, 01:19Z (before) and 01:25Z–01:28Z (after). Written whether or not a stop occurs; none occurred.

**No row is of the class `our check reads differently`.** No assertion of ours met a new value from the merged engine: every stand, fixture and driver check passed unedited in both layout modes on the first run of the gate block, under the workspace build and the per-package build. No expectation, tolerance, table of `tests/blitz-tests/tests/common/mod.rs`, stand markup, id, CSS, viewport or font was touched (the gate block's preservation entry reads `examples/seven_guis` unchanged; `git status` lists no file under `tests/blitz-tests/tests/` but the merge's own). Nothing waits on the operator's word.

Every number that moved is a tally, and every one is accounted for by a test the delta adds.

| number | old → new | read by | stated in | cause |
|---|---|---|---|---|
| workspace result lines | 154 → 158 | the count entry (`awk` over `target/ci-logs/test.log`) | test-plan §9 Local baseline | `upstream tests added` — four new targets: `anonymous_block_percentage_height.rs` (#1149), `autofocus_attribute.rs` (#1085), `text_transform.rs` (#1093) in blitz-tests; `blitz-vibey-script/tests/selection.rs` (#1094) |
| workspace passed | 657 → 719 | the count entry | test-plan §9 Local baseline | `upstream tests added` — 62 of the 63 `#[test]` lines the delta adds, by file in `counts.md`; the 63rd is the unbuilt `all.rs` guard (#1123) |
| workspace failed · ignored | 0 → 0 · 10 → 10 | the count entry | test-plan §9 Local baseline | unmoved |
| `blitz_dom` unit tests | 64 → 87 | per-target table | — | `upstream tests added` — `layout/text_transform.rs` 22 (#947, #1093, #1095), `debug.rs` 1 (#1099 / #1101) |
| `wpt` unit tests | 15 → 28 | per-target table | — | `upstream tests added` — `test_variants.rs` 10, `test_runners/ref_test.rs` 2, `main.rs` 1 (#1117) |
| `dom` (blitz-vibey-script) | 26 → 27 | per-target table | — | `upstream tests added` — `tests/dom.rs` (#1147) |
| `inline_fragment_rects` | 4 → 5 | per-target table | — | `upstream tests added` — the file's new test (#1150) |
| integration-test executables in the workspace build set | 103 → 107 | the build-set entry (`cargo test --workspace --locked --no-run`) | the plan's baseline | `upstream tests added` — the same four new targets; `all-target 0` before and after |
| CI-scripts tally | 70 → 78 | the report-only entry over `target/ci-logs/ci-scripts.log` | test-plan §4 / §9 | `upstream tests added` for 7 — `test_wpt_area_changes.py` 4 (new file), `test_wpt_diff_to_pr.py` 4 → 7; and 1 test of ours added by this chunk, `BlitzTestsTargetsTest` (not an expectation that moved — a new check) |
| `run all` passed | 386 → 396 | `bash scripts/agent-run.sh run all`, `run.end` | — | `upstream tests added` — 1 + 5 + 3 + 1 in the four blitz-tests files above. The old figure is derived, not a run: the blitz-tests rows of the pre-merge workspace table sum to 386 passed, 10 ignored over 98 targets; the new one is the run's own `run.end` (396 passed, 0 failed, 10 ignored) and equals the same sum over the post-merge table's 101 targets |
| blitz-dom features, workspace build | 14 → 16 | the report-only feature entry | test-plan §9 Engine features by runner | not a test tally: `writing-mode` and `text-transform-icu` arrive with upstream's manifests (`features.md`) |
| blitz-dom features, per-package build | 8 → 10 | the report-only feature entry | test-plan §9 Engine features by runner | not a test tally: `autofocus` and `text-transform-icu`, upstream's side of the merged blitz-tests line |

## Numbers read and found unmoved
- `run stand`: 110 passed, 0 failed, 5 ignored over 30 files — the last recorded figures. The stand selection lists 30 files.
- The a11y leg: 3 result lines, 15 passed, 0 failed (6 + 6 + 3).
- Every target of ours in the per-target table: 150 of the 154 pre-merge rows read the same passed / failed / ignored after the merge; the four that differ are upstream's, above.
- The 15 changed-set marks of `mutator.rs`; the one `skrifa` in the lock.

A tally delta no upstream test accounts for would be of the second class. There is none: 62 = 62, 8 = 7 + 1, 4 = 4.
