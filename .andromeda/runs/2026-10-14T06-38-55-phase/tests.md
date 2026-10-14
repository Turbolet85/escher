# tests extract

## Relevance
relevant: the chunk turns three stand harnesses that record readings into pass/fail assertions and moves stand checks into the `a11y` CI leg. Both are test-plan surfaces: the stand checks, the leg list and the 5-command `run stand` count.

## Constraints
- Every stand check is its own auto-discovered target under `tests/blitz-tests/tests/`, with no manifest line, one file per behaviour. A file with the `stand_` prefix is picked up by `run stand` with no script change. The fork keeps one target per file, and `BlitzTestsTargetsTest` pins that (per test-plan §2 Directory pattern).
- The stand's shared expectation is stated once in `tests/blitz-tests/tests/common/mod.rs`: `controls(task)` (18 rows naming 15 controls), `tab_order(task)` (CRUD six stops) and `focused`. A check reads the tables through `mod common;`. The tables are stated expectation, never a second role or name mapping. So naming `crud-list`, a roving `tabindex` or a re-coloured accent moves these tables, not a per-file copy (per test-plan §3 Crate-local test helpers).
- Headless stand checks boot through `seven_guis::stand::boot` with `stand::options(incremental)`: 800×600, the bundled DejaVu Sans, system fonts off, no network. Each runs in both layout modes, and a check needing a case the stand lacks uses an in-file fixture booted with the stand's options (per test-plan §3 blitz-test-harness (`Harness`)).
- SC 1.4.3 is read through `Harness::text_contrast`, which reads computed styles only and changes nothing. It returns `None` when an element on the ancestor walk has `opacity` below 1, a filter or a background image. `required_ratio` is 3.0 for large text, else 4.5. An assertion built on it inherits those limits (per test-plan §3 blitz-test-harness (`Harness`)).
- Today `ci-leg.sh a11y` runs `--test accessibility_hidden --test accessibility_roles --test focusability_updates` only. `stand_keyboard`, `stand_contrast` and `accessibility_states` run in the workspace `test` leg and per package, not in the a11y leg (per test-plan §9 Legs).
- The a11y job is pinned by `.github/scripts/test_ci_workflows.py`. `CiWorkflowTest` requires an a11y job named for accessibility, restore-only on the test job's `shared-key` with `save-if: false`. `LegScriptTest` requires the `a11y` leg to dispatch to cargo and write its log. A change to the leg's test list or to the job must keep the `ci-scripts` leg green (per test-plan §4 CI workflows and leg script).
- The count baselines this chunk must re-count against:
  - `run stand`'s `run.end` reads passed 153 · failed 0 · ignored 7 over 38 files. An empty run is never a pass (per test-plan §3 Agent-run contract).
  - The workspace reads 182 result lines, 928 passed · 0 failed · 12 ignored, read through `ci-leg.sh fast` (per test-plan §9 Local baseline).

## Patterns to follow
- `stand_contrast` is the record this chunk turns into an assertion:
  - 18 rows, one per `controls(task)` row over the four lean tasks at boot.
  - Exempt exactly where the snapshot reads `enabled == Some(false)`.
  - The record is identical in both layout modes.
  - The painted background is cross-checked by a pixel inside the left padding, within 1 per channel.
  
  Today it "records and gates nothing". SC 1.4.3's assertion is owned by this route entry (per test-plan §1 tests/blitz-tests).
- `stand_keyboard` is the SC 2.1.1 / 2.4.3 reading:
  - Tab and Shift+Tab walk `common::tab_order` per lean task and wrap.
  - The engine's focus, the snapshot's `focused` and the tree's focus agree at every step.
  - Enter and Space are checked over 20 stated and counted rows.
  - The engine honours no `inert`: `the_engine_honours_no_inert` is pinned.
  
  A focus-order change moves `tab_order` and this file (per test-plan §1 tests/blitz-tests).
- Tables are stated in the test with their row count asserted. Every stand-row test in the files above, and each harness and driver unit table, does this (per test-plan §4 What unit tests cover).
- `stand_accessibility_ids` holds the 15 controls' roles and names. It reads `DioxusDocument::accessibility_tree`, carrying `author_id` on every element node and no other. Naming the listbox restates this table (per test-plan §1 tests/blitz-tests).
- Pure-restyle states are driven by `:hover` rather than attribute mutation (`oof_dynamic_cb`, `anonymous_block_cache_invalidation`). This is the existing route to a hover or active contrast reading. Whether the harness's `move_mouse_to` / `mouse_down_at` resolve `:hover` / `:active` into what `text_contrast` reads is research's question (per test-plan §2 Pure-restyle paths).

## Anti-patterns to avoid
- No skip path on a stand check. The headless-stand checks assert unconditionally under the bundled font, unlike the older font-dependent tests that `eprintln!` and pass (per test-plan §2 Font-dependent tests).
- No assertion that passes without asserting. Two text-input tests assert only inside an `if` on measured layout and so pass vacuously. A contrast or keyboard row must assert on every row (per test-plan §4 Conditional assertions).
- No sleep and no wall-clock read in a stand check. Time moves only by delivered ticks or the harness's own clock (per test-plan §5 Session ↔ held instance).

## Contract bindings
- tests ↔ a11y: the a11y CI gate is the `a11y` leg of `.github/scripts/ci-leg.sh` and its job in `.github/workflows/ci.yml`.
  - Test-plan §9 Legs names its three files.
  - test-plan §9 Local baseline's last local reading, `6 · 6 · 3`, predates `focusability_updates` gaining a test at 2026-10-13-stand-keyboard-harness (test-plan §1 tests/blitz-tests). The leg's current count is a11y-plan §9's reading to confirm.
  - "Gating merges" depends on what blocks a merge on the fork. test-plan §9 records only the fast/slow `needs` split and the triggers (PRs, pushes to `build/**`), not branch protection or required checks. That is research's question.
- tests ↔ CI invariants: a leg-list change touches `.github/scripts/ci-leg.sh`. A job change touches what `CiWorkflowTest` and `LegScriptTest` pin, run by the `ci-scripts` leg (test-plan §4 CI workflows and leg script).
- tests ↔ obs: the 5-command discipline's `run stand` events are harness metadata, not an escher-telemetry sink (test-plan §3 Agent-run contract, citing obs-plan §3). A new stand check joins `run stand` by prefix, and obs-plan §3's silent census is obs's to restate.

## Acceptance criteria contributions
- `bash scripts/agent-run.sh run stand` ends `run.end` outcome `passed` with failed 0. The passed count is re-counted from 153 by exactly the stand tests added or removed, over the stated file count (per test-plan §3 Agent-run contract).
- `stand_contrast` asserts rather than records. Every in-scope stand control row meets its `required_ratio`, with no row left `fail`, and the record is identical in both layout modes and against the painted-pixel check (per test-plan §1 tests/blitz-tests).
- `bash .github/scripts/ci-leg.sh a11y` exits 0 with the stand assertions in its test list. `ci-leg.sh ci-scripts` (`test_ci_workflows.py` `CiWorkflowTest` and `LegScriptTest`) stays green (per test-plan §9 Legs).
- `bash .github/scripts/ci-leg.sh fast` passes, its workspace total re-counted from 182 result lines, 928 · 0 · 12 (per test-plan §9 Local baseline).
