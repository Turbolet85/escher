proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 14
    section: "## 1. Test Scope Summary"
    change: "In the s04 search's parenthetical, '(hits only in the five browser files above)' reads '(hits only in the browser files above — five until 2026-10-27-upstream-sync-engine-debts, six since its take gave `apps/browser/src/tab.rs` a test module)'; the rest of the entity unchanged."
    sidecar: "test-scope-summary: the apps search's hit count, five browser files → six (`apps/browser/src/tab.rs`'s test module, 2026-10-27-upstream-sync-engine-debts)."
    rationale: "The report's Symbols / APIs bullet: apps/browser gains a test module in `tab.rs` (`295-343`, `tab_lens_survives_webview_props_replacement`, `d63ed2c8c`); the workspace tally's +8 passed counts `apps/browser/src/tab.rs` 1 — the s04 search now hits a sixth browser file."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 30
    section: "## 1. Test Scope Summary"
    change: "New item after the blitz-dom (layout) entity: '- **blitz-dom (layout) — flow classification:** since 2026-10-27-upstream-sync-engine-debts upstream's blitz-tests target `pseudo_element_flow_classification` (3 tests — `inline_block_pseudo_only_child_exports_line_baseline`, `inline_block_pseudo_only_child_bottom_aligned`, `grid_row_grows_by_baseline_shim_of_pseudo_only_items`) pins that every box-generating `::before`/`::after` votes in `classify_flow_children`, the `display: contents` filter dropped; its `mod` line stands in the unbuilt `all.rs` (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78)'"
    sidecar: "test-scope-summary: blitz-dom (layout) gains the pseudo-element flow-classification target `pseudo_element_flow_classification` (3), upstream's, 2026-10-27-upstream-sync-engine-debts."
    rationale: "The report's Symbols / APIs: `classify_flow_children` (`layout/construct.rs:860`, `:867-872`) — every box-generating pseudo votes (`2963b65a2`); Crates / modules: blitz-tests gains the auto-discovered target, `all.rs:54` its `mod` line; Outcome: 3 passed, its own target."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:14
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 31
    section: "## 1. Test Scope Summary"
    change: "New item after the blitz-dom (node) entity: '- **blitz-dom (node) — `:open`:** since 2026-10-27-upstream-sync-engine-debts `ElementData::flush_open_state` sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`, written on construction and on an `open` attribute write or removal, and `:open` matches it — it has no test; `details_element` and the a11y leg read green beside it (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510)'"
    sidecar: "test-scope-summary: blitz-dom (node) records `:open` (`ElementData::flush_open_state`, upstream's) as untested, 2026-10-27-upstream-sync-engine-debts."
    rationale: "The report's Symbols / APIs: `flush_open_state` new and pub at `node/element.rs:543-549`, called from `ElementData::new` and the mutator's `open` arms, `NonTSPseudoClass::Open` reading it at `stylo.rs:510`; Coverage of new surfaces: `:open` tests ✗ (the commit adds no test; `details_element` and the a11y leg green)."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:85
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 68
    section: "## 1. Test Scope Summary"
    change: "'tests/dom.rs holds 27 `#[test]` functions covering the JS DOM APIs' reads 'tests/dom.rs holds 30 `#[test]` functions covering the JS DOM APIs (27 until 2026-10-27-upstream-sync-engine-debts, whose take added three — two of the `file:`-load narrowing, one of the CSS property accessors)'; the rest of the entity unchanged."
    sidecar: "test-scope-summary: blitz-vibey-script tests/dom.rs 27 → 30 `#[test]` (2026-10-27-upstream-sync-engine-debts, `cee6b8731` +2, `efdc5472a` +1)."
    rationale: "The report's Counts: blitz-vibey-script `tests/dom.rs` 27 → 30 `#[test]` (`cee6b8731` +2, `efdc5472a` +1), the changed site `test-plan-detail/test-scope-summary.md:68`; Outcome: `cargo test -p blitz-vibey-script --locked --test dom` 30 passed."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:30
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 74
    section: "## 1. Test Scope Summary"
    change: "'81 unit tests in nine files' reads '82 unit tests in nine files (81 until 2026-10-27-upstream-sync-engine-debts, whose take added `input_focused_on_mount_enables_ime` to dioxus_document.rs's test module, the file count unchanged)'; the rest of the entity unchanged."
    sidecar: "test-scope-summary: dioxus-native-dom lib 81 → 82 unit tests, nine files (2026-10-27-upstream-sync-engine-debts)."
    rationale: "The report's Counts: dioxus-native-dom lib 81 → 82 (`input_focused_on_mount_enables_ime`), the changed site `test-plan-detail/test-scope-summary.md:74`; Symbols / APIs: the one test added in `dioxus_document.rs`, the file's diff removing 0 lines; Outcome: `cargo test -p dioxus-native-dom --locked --lib` 82 passed."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 79
    section: "## 1. Test Scope Summary"
    change: "New item after the dioxus-native-dom entity: '- **dioxus-native-dom — IME on mount:** since 2026-10-27-upstream-sync-engine-debts upstream's `input_focused_on_mount_enables_ime`, in dioxus_document.rs's test module, pins that a text input focused before its editor is constructed enables IME (`set_ime_enabled(true)`) at that construction (packages/dioxus-native-dom/src/dioxus_document.rs:725-759)'"
    sidecar: "test-scope-summary: dioxus-native-dom records upstream's IME-on-mount test, 2026-10-27-upstream-sync-engine-debts."
    rationale: "The report's Symbols / APIs: `create_text_editor` (`layout/construct.rs:313-317`) calls `set_ime_enabled(true)` for an input already focused (`7fc9ee3af`); dioxus-native-dom's one test added, `input_focused_on_mount_enables_ime` (`dioxus_document.rs:725-759 @729`); Coverage of new surfaces: tests unit."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:18
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 79
    section: "## 1. Test Scope Summary"
    change: "New item after the dioxus-native-dom entity: '- **dioxus-native-dom — an upstream test not taken:** upstream's `task_polled_during_render_can_access_document` is absent from the fork — it stood in `7fc9ee3af`'s conflict hunk in dioxus_document.rs and was dropped, its commit `286874cc1` not taken (2026-10-27-upstream-sync-engine-debts)'"
    sidecar: "test-scope-summary: dioxus-native-dom records upstream's `task_polled_during_render_can_access_document` as absent from the fork, 2026-10-27-upstream-sync-engine-debts."
    rationale: "The report's Reverted / negative API facts: `task_polled_during_render_can_access_document` stood in `7fc9ee3af`'s conflict hunk and was dropped (`286874cc1`'s, not taken); Symbols / APIs: not taken."
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:70
    fact: F11
note: F8 (tab.rs's test) belongs beside the unpicked apps/browser entity (body test-plan.md:13 and its detail row, both listing five files) and F10 (the shell's redraw route) beside blitz-shell, neither picked here — the new-detail pass places them; F2–F6 are other categories'; D-tests-coverage reads `:open` shipped untested (report Coverage ✗), recorded by the F9 proposal.
