## 2026-10-27-upstream-sync-engine-debts — upstream's `pseudo_element_flow_classification` target and the legs that reach it
**Section:** §9 CI Integration (ci-integration.md: a new `values · ci` block after the Legs); §1 Test Scope Summary (test-scope-summary.md: a new blitz-dom (layout) — flow classification item); §2 Test Strategy (test-strategy.md: the Directory pattern's blitz-tests count)
**Change:**
- ci-integration gains the record **Legs reaching `pseudo_element_flow_classification`:** the target runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; it has no `stand_` prefix and is none of `stand-repeat`'s ten named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files.
- test-scope-summary gains **blitz-dom (layout) — flow classification:** the target's 3 tests, `inline_block_pseudo_only_child_exports_line_baseline`, `inline_block_pseudo_only_child_bottom_aligned` and `grid_row_grows_by_baseline_shim_of_pseudo_only_items`, pin that every box-generating `::before` or `::after` votes in `classify_flow_children`; it is its own auto-discovered target, its `mod` line standing in the unbuilt `all.rs`.
- The blitz-tests directory reads 148 `.rs` files, 147 test targets and the unbuilt `all.rs` (was 147 and 146).
**Why:** the take of upstream's eleven commits brought the new file as its own auto-discovered target, `all.rs` gaining its `mod` line and staying `test = false`; the report's Harness / gate surface reads no `ci-leg.sh`, `agent-run.sh` or CI-script change, the target picked up by the workspace leg and `run all` and by neither the `a11y` leg nor `run stand`; its Counts read the directory 147 → 148 files and 146 → 147 targets.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the per-OS and local workspace tallies re-read after the take
**Section:** §9 CI Integration (ci-integration.md: Per-OS test tallies; Local baseline)
**Change:**
- Per-OS test tallies gain a re-read with upstream's eleven commits on CI#40487650912, the pre-CI commit `e795a088c` (green 17/17): linux 209 · 1125 · 0 · 12; macos and coverage 190 · 1122 · 0 · 12; windows 190 · 1076 · 0 · 9; the a11y job 14 · 85 · 0 · 0 and the stand repeat 204 · 864 · 0 · 21, unmoved — +1 result line and +8 passed on each of the four test jobs over CI#40434241601, every move the take's by name.
- The local workspace tally is re-counted at 209 result lines, 1125 passed · 0 failed · 12 ignored — +1 result line (`pseudo_element_flow_classification`) and +8 passed (`pseudo_element_flow_classification` 3, blitz-vibey-script's `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1), timings not re-measured.
**Why:** the report's Counts read both tallies after the take and explain every move by name: the new target's result line and its 3 tests, and the 5 tests the take added in existing result lines.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the flow-classification file an exception to the both-layout-modes rule
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: a new `procedure · integration` block before Script ↔ DOM)
**Change:**
- A new group **Layout assertions ↔ both layout modes:** upstream's `pseudo_element_flow_classification` (3 tests — every box-generating `::before` or `::after` voting in flow classification) is an exception to the rule that pipeline tests run `for incremental in [false, true]`: the file asserts neither mode pair, and `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle).
**Why:** the report's Changes read the file's tests as integration tests that do not assert both layout modes, with `incremental_oracle` green; no group of the file held the body's Layout assertions line, so the fact opens one.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the script tests the take added
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Script ↔ DOM (blitz-vibey-script)); §1 Test Scope Summary (test-scope-summary.md: blitz-vibey-script)
**Change:**
- Script ↔ DOM adds CSS properties as accessors on `CSSStyleDeclaration`, named from Stylo's property list, by `style_property_accessors`, and a `file:` URL loaded by `fetch()`, a module `import` or `<script src>` only for a `file:` document (`document_may_load`), refused with a `FetchError` otherwise, by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`, the `file:` URLs read on the macOS and Windows CI legs beside linux's.
- blitz-vibey-script: `tests/dom.rs` holds 30 `#[test]` functions (was 27).
**Why:** the report's Counts read `tests/dom.rs` 27 → 30, the take adding two tests of the `file:`-load narrowing and one of the CSS property accessors, and its Coverage of new surfaces names those three as their tests; the citation sweep's stretched range over the file had left the count of 27 false.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — citations re-pointed past the take's added lines
**Section:** §5 Integration Test Strategy (integration-test-strategy.md); §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Shell and Script fetch rows); §7 Test Data & Fixtures (test-data-fixtures.md: the Inline literals, Viewports and Fetch body rows); §1 Test Scope Summary (test-scope-summary.md); §2 Test Strategy (test-strategy.md: Directory pattern); §4 Unit Test Strategy (unit-test-strategy.md: Test file location, blitz-dom (node)); test-plan.md §1 · §2 · §3 · §4
**Change:**
- Cited line ranges in `packages/blitz-dom/src/mutator.rs`, `packages/blitz-dom/src/node/element.rs`, `packages/blitz-vibey-script/tests/dom.rs`, `packages/dioxus-native-dom/src/dioxus_document.rs` and `tests/blitz-tests/tests/all.rs` now point where the code they cite stands after the take; each claim's text is unchanged.
- Three ranges stretch over tests the take added inside them: the Script ↔ DOM range over `tests/dom.rs` (taking in `style_property_accessors`), blitz-vibey-script's range over the same file, and the dioxus-native-dom range over `dioxus_document.rs`'s test module (taking in `input_focused_on_mount_enables_ime`).
**Why:** the take added lines above the cited code in each of those files, so the citation sweep re-pointed the ranges; the stretched ranges still hold their claims, the two counts they back moved by the entries on the script tests and on dioxus-native-dom.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the `keyboard_compat` coverage exclusion re-read and its owner moved
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the coverage floor's `keyboard_compat.rs` exclusion)
**Change:**
- `286874cc1`, which retires `keyboard_compat.rs`, is read as left out of the take of upstream/main `ebacd68bd`, so the file stays and both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand, the gate entry `test -f …keyboard_compat.rs && grep -c …` over `.github/scripts/ci-leg.sh` reading 2.
- The exclusion's owner is the `.andromeda/residuals.md` line of the left-out upstream delta, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (was target: next); the reading is measured at this chunk's report (was 2026-10-24-upstream-sync-polish-and-ship's).
**Why:** the report states `286874cc1` not taken and the gate entry green at 2; the left-out delta's residual line moved to the Epoch 3 entry on the operator's own word at the P5 review card (inputs#I5).
**Kept:** the exclusion and both llvm-cov lines, standing until escher takes upstream's retirement of the file.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the script tests' shared page and files on disk
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: the Inline HTML (script tests) row; a new Files on disk (script tests) row)
**Change:**
- Inline HTML (script tests) reads inline raw strings per test, and one `FILE_LOAD_TEST_HTML` page with its `FILE_LOAD_TEST_FILES` shared by the two `file:`-load tests (was inline raw strings per test).
- A new row, Files on disk (script tests): `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests — test data on disk beside the in-code seeds, through a test-only `TMPDIR` read (arch §Occupied Resources).
**Why:** the take's `file:`-load tests share one page and its files, so the per-test claim no longer covered every script test seed; the report's Symbols / APIs read one new test-only `TMPDIR` read through `std::env::temp_dir()` in `temp_dir_with_files`, its Counts the test-only sites 1 → 2.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the browser's tab test module
**Section:** §1 Test Scope Summary (test-scope-summary.md: the apps entity's s04 search; a new apps/browser (tab) item; test-plan.md §1 apps/browser); §4 Unit Test Strategy (test-plan.md §4 apps/browser)
**Change:**
- The s04 search hits only in the six browser files (was five).
- A new item **apps/browser (tab):** `apps/browser/src/tab.rs`, in the upstream-only browser app, holds a test module with one `#[test]`, `tab_lens_survives_webview_props_replacement` — a tab's lenses survive a replacement of its webview props — the browser's sixth tested file beside about_pages, browser_history, favicon, url_suggestions and persistence, run by the workspace `test` leg.
- The body's apps/browser inventory lists tab (1) after persistence (10), and §4's apps/browser cases add the tab lens case.
**Why:** the report's Symbols / APIs read upstream's `d63ed2c8c` giving `tab.rs` a test module, and the workspace tally's +8 passed counts `apps/browser/src/tab.rs` 1; the body's list of browser files holding tests had named five.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — `:open` recorded as untested
**Section:** §1 Test Scope Summary (test-scope-summary.md: a new blitz-dom (node) — `:open` item; test-plan.md §1 blitz-dom covered behaviours)
**Change:**
- A new item **blitz-dom (node) — `:open`:** `ElementData::flush_open_state` sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`, called on the element's construction and on an `open` attribute's write or removal, and `:open` matches that state; it has no test of its own — `details_element` and the a11y leg read green beside it.
- The body's covered-behaviours bullet names the gap: `:open` matching a `details` or `dialog` carrying `open` has no test of its own.
**Why:** the report's Coverage of new surfaces marks `:open` tests ✗ — the take added no test, `details_element` and the a11y leg green; it is a behaviour of every document the inventory now states as a gap, not fixed.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — dioxus-native-dom's IME test and an upstream test absent
**Section:** §1 Test Scope Summary (test-scope-summary.md: dioxus-native-dom; two new dioxus-native-dom items)
**Change:**
- dioxus-native-dom holds 82 unit tests in nine files (was 81).
- A new item **dioxus-native-dom — IME on mount:** upstream's `input_focused_on_mount_enables_ime`, in `dioxus_document.rs`'s test module, pins that a text input focused before its editor is constructed enables IME (`set_ime_enabled(true)`) at that construction.
- A new item **dioxus-native-dom — an upstream test absent:** upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's `dioxus_document.rs` test module.
**Why:** the report's Counts read the crate's lib 81 → 82 by the one test the take added; its Reverted / negative API facts read `task_polled_during_render_can_access_document` dropped from the merged hunk, its commit left out of the take.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the windowed shell's redraw route recorded as holding no check
**Section:** §1 Test Scope Summary (test-scope-summary.md: tests/blitz-tests, the `focus_redraw` clause)
**Change:**
- The `focus_redraw` clause adds: the windowed shell sends its redraw requests through its event loop, a route that holds no check of its own, the windowed `seven_guis_native` boot smoke its one reading and `focus_redraw` green beside it.
**Why:** the report's Symbols / APIs read the take routing the windowed shell's redraw requests through its event loop, and its Coverage of new surfaces reads that route tested only by the windowed boot smoke, `focus_redraw` green; the gap is recorded, not fixed.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — blitz-vibey-script's inline-test absence over the split modules
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: the `absence · unit` block, blitz-vibey-script source files)
**Change:**
- The s10 search over 32 files is read as the reading at that search, and the 13 Rust modules of the crate's `runtime/` and `dom/element/` trees, `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` and `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`, hold no test module either, the crate's script tests standing in `tests/`.
**Why:** the report's Crates / modules read upstream's split adding the `runtime/` and `dom/element/` module trees, 13 Rust modules, none a test module, and the take's added script tests all in `tests/dom.rs`; the absence holds while the searched file count is stale.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the agent-run Proof re-counted after the take
**Section:** §3 Test Harness Contract (test-plan.md: Agent-run contract → Proof)
**Change:**
- The Proof record gains its next entry: re-counted with upstream's `pseudo_element_flow_classification`, `scripts/agent-run.sh` unchanged, `run stand`'s `run.end` unmoved at passed 205 · failed 0 · ignored 7 over 50 files, and `run all` reads passed 633 · failed 0 · ignored 12 — `pseudo_element_flow_classification` 3, a new file with no `stand_` prefix, picked up by `run all` with no script change.
**Why:** the report's Counts read `run all` 630 → 633 · 0 · 12 by the new target's 3 and `run stand` 205 · 0 · 7 over 50 files unmoved, with no `agent-run.sh` change; the Proof bullet is the per-chunk record of those readings.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
