proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "Read `148 .rs files, 147 test targets and the unbuilt all.rs` in place of 147 and 146, and open the parenthesised history with: 147 and 146 before upstream's `pseudo_element_flow_classification.rs`, taken at 2026-10-27-upstream-sync-engine-debts (`2963b65a2`), its own auto-discovered target with no manifest line, upstream's `all.rs` gaining its one `mod` line and staying unbuilt — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md; then the standing `145 and 144 before stand_cells_served.rs …` clause unchanged"
    sidecar: "test-strategy: the blitz-tests directory reads 148 .rs files and 147 targets with upstream's pseudo_element_flow_classification (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report Counts `bt-files`: blitz-tests directory 147 → 148 .rs files, 146 → 147 test targets, 1 changed at test-plan-detail/test-strategy.md:54; Crates / modules: the auto-discovered target, all.rs:54 gaining its mod line, all.rs staying test = false; Outcome: 148 files, no Running tests/all.rs line"
    basis: .andromeda/test-plan-detail/test-strategy.md:54
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 77
    section: "## 4. Unit Test Strategy"
    change: "Keep the search and add after it: — the reading at that search; since 2026-10-27-upstream-sync-engine-debts upstream's split (`e6f4615ec`) moved the crate's runtime and element bindings into 13 new Rust modules, `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` and `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`, none holding a test module, the chunk's added script tests all in `tests/dom.rs`"
    sidecar: "unit-test-strategy: the blitz-vibey-script inline-test absence re-read over the 13 modules the runtime and element split added (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report Crates / modules: blitz-vibey-script gains the runtime/ and dom/element/ module trees (13 Rust modules, 9 JS); New text lists every block of those 13 files and none is a test module; Counts `dom-rs`: the three added tests in tests/dom.rs; the task's Touches name this absence's file count as stale while the absence holds"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 8
    section: "## 5. Integration Test Strategy"
    change: "Add after the fetch and window error clause: since 2026-10-27-upstream-sync-engine-debts (upstream `cee6b8731`) `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` pin `document_may_load` — a `file:` URL loads by `fetch()`, module `import` and `<script src>` only for a `file:` document, refused with a `FetchError` otherwise — its refusal witnessed on the linux, macOS and Windows CI legs (packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950)"
    sidecar: "integration-test-strategy: Script ↔ DOM gains the two file:-load tests pinning document_may_load (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Task New facts F4; report Coverage of new surfaces: document_may_load tests integ remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, macOS and Windows CI the file: URL witnesses; Symbols / APIs: checked by fetch(), import and script src; New text tests/dom.rs 904-928, 930-950"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 8
    section: "## 5. Integration Test Strategy"
    change: "Add beside the style coverage: since 2026-10-27-upstream-sync-engine-debts (upstream `efdc5472a`) `style_property_accessors` pins CSS properties as accessors on `CSSStyleDeclaration`, their names taken from Stylo's property list (`runtime/css.rs` `css_property_names`, `bootstrap/style.js` `installStyleProperties`) (packages/blitz-vibey-script/tests/dom.rs:455-487)"
    sidecar: "integration-test-strategy: Script ↔ DOM gains style_property_accessors, the CSSStyleDeclaration accessors test (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Task New facts F5; report Coverage of new surfaces: CSS property accessors tests unit tests/dom.rs style_property_accessors, names from Stylo's property list; Symbols / APIs runtime/css.rs css_property_names (8-18), bootstrap/style.js installStyleProperties (26-48); New text tests/dom.rs 455-487"
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "New row after the Inline HTML row: | Files on disk (script tests) | `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the `file:`-load tests — test data on disk beside the in-code seeds, and a second test-only `TMPDIR` read beside escher-driver's client unit test | (packages/blitz-vibey-script/tests/dom.rs:860-869) |"
    sidecar: "test-data-fixtures: a seed row for tests/dom.rs temp_dir_with_files, the file:-load tests' sibling files under the temp dir (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Task New facts F6; report Symbols / APIs: one new test-only env read, TMPDIR through std::env::temp_dir() in tests/dom.rs temp_dir_with_files (860-869 @861, the call at :863), a second test-only site beside escher-driver's client unit test; Counts `tmpdir` 1 → 2 sites"
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 25
    section: "## 7. Test Data & Fixtures"
    change: "Strategy cell: inline raw strings per test, and since 2026-10-27-upstream-sync-engine-debts one `FILE_LOAD_TEST_HTML` page with its `FILE_LOAD_TEST_FILES` shared by the two `file:`-load tests (packages/blitz-vibey-script/tests/dom.rs:871-892; packages/blitz-vibey-script/tests/dom.rs:895-902)"
    sidecar: "test-data-fixtures: the script tests' inline HTML row names the shared FILE_LOAD_TEST_HTML and FILE_LOAD_TEST_FILES (2026-10-27-upstream-sync-engine-debts)"
    rationale: "tests/dom.rs is a reached file; report New text lists FILE_LOAD_TEST_HTML (871-892) and FILE_LOAD_TEST_FILES (895-902) beside the two file:-load tests (904-928, 930-950), so the row's per-test claim no longer covers every script test seed"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "New continuation line after line 10: — upstream's `pseudo_element_flow_classification` (since 2026-10-27-upstream-sync-engine-debts) is reached by the workspace `test` leg and by `agent-run.sh`'s `run all` / `run {name}` only: it has no `stand_` prefix, is not one of `stand-repeat`'s ten named targets and is not in the a11y leg's fourteen files, each leg's script unchanged"
    sidecar: "ci-integration: the Legs record names the legs that reach pseudo_element_flow_classification (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Task New facts F3; report Harness / gate surface: no ci-leg.sh, agent-run.sh or CI-script change — the new target picked up by the workspace leg and run all with no script change, not by the a11y leg or run stand; run stand 205 · 0 · 7 over 50 files and the a11y leg 14 · 85 unmoved"
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 43
    section: "## 9. CI Integration"
    change: "New continuation line after line 43: Re-read with upstream's eleven commits on CI#40487650912, the pre-CI commit `e795a088c` (green 17/17): linux 209 · 1125 · 0 · 12; macos and coverage 190 · 1122 · 0 · 12; windows 190 · 1076 · 0 · 9; the a11y job 14 · 85 · 0 · 0 and the stand repeat 204 · 864 · 0 · 21, unmoved — +1 result line and +8 passed on each of the four test jobs over CI#40434241601, every move the take's by name — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/ci.md"
    sidecar: "ci-integration: the per-OS tallies re-read on CI#40487650912 after the 2026-10-27 upstream take"
    rationale: "Report Counts `ws-tally`: per-OS on CI#40487650912 against CI#40434241601 — Test [default features] 209 · 1125 · 0 · 12; Test (macos) and Coverage report 190 · 1122 · 0 · 12; Test (windows) 190 · 1076 · 0 · 9; a11y 14 · 85 · 0 · 0 and Stand checks repeated 204 · 864 · 0 · 21 unmoved; 2 changed, test-plan-detail/ci-integration.md:43 the per-OS rows; Outcome: per-OS rows explained by name"
    basis: .andromeda/test-plan-detail/ci-integration.md:43
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 64
    section: "## 9. CI Integration"
    change: "Append to line 64 (or open a continuation line after it, should the line pass 2 000 chars): ; re-counted with upstream's eleven commits: 209 result lines, 1125 passed · 0 failed · 12 ignored — +1 result line (`pseudo_element_flow_classification`) and +8 passed (`pseudo_element_flow_classification` 3, blitz-vibey-script `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/counts.md (a `ci-leg.sh fast` run's tally probe)"
    sidecar: "ci-integration: the local workspace tally re-counted at 209 · 1125 · 0 · 12 after the 2026-10-27 upstream take"
    rationale: "Report Counts `ws-tally`: workspace tally 208 · 1117 · 0 · 12 → 209 · 1125 · 0 · 12, +1 line the new target, +8 passed its 3, tests/dom.rs 3, dioxus-native-dom 1, apps/browser/src/tab.rs 1 (evidence/counts.md); 2 changed, :64 the local tally; Outcome gate `lines 209 passed 1125 failed 0 ignored 12`"
    basis: .andromeda/test-plan-detail/ci-integration.md:64
note: F2 (the new target an exception to the both-layout-modes rule) belongs in integration-test-strategy but no picked entity there carries it, so the new-detail pass places it; F1 and F7–F11 are test-scope-summary's, and the 32-file slice s10 count is also stated at test-plan.md:42 and in test-scope-summary; whether `input_focused_on_mount_enables_ime` holds a ShellProvider fake recording `set_ime_enabled` (a mocking row) is not stated by the report
