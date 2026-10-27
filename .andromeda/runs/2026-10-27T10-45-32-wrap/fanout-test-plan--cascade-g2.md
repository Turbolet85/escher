proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "Add after the `hit_test_dropped_nodes` · `accessibility_merged_states` · `inert_subtree` line, in its shape: since 2026-10-27-upstream-sync-engine-debts's take (upstream `2963b65a2`), upstream's `pseudo_element_flow_classification` 3 — an inline-block whose only child is a box-generating pseudo exporting its line baseline (`inline_block_pseudo_only_child_exports_line_baseline`) and bottom-aligned (`inline_block_pseudo_only_child_bottom_aligned`), and a grid row growing by the baseline shim of pseudo-only items (`grid_row_grows_by_baseline_shim_of_pseudo_only_items`): every box-generating `::before`/`::after` votes in flow classification, the `display: contents` filter dropped (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78) — its own auto-discovered target, its `mod` line in the unbuilt `all.rs` (tests/blitz-tests/tests/all.rs:54), run by the workspace `test` leg and `run all` and not by the `a11y` leg or `run stand`;"
    sidecar: "test-scope-summary: tests/blitz-tests gains upstream's `pseudo_element_flow_classification` (3 tests, the pseudo vote in flow classification), its own target with its `mod` line in the unbuilt `all.rs` — 2026-10-27-upstream-sync-engine-debts"
    rationale: "Report Changes › Crates / modules (blitz-tests gains the auto-discovered target `pseudo_element_flow_classification`, `all.rs:54` its `mod` line, `all.rs` stays `test = false`), Symbols / APIs (`classify_flow_children`, `2963b65a2`), Harness / gate surface (picked up by the workspace leg and `run all`, not by the `a11y` leg or `run stand`) and New text (the file's three tests at 37-48, 50-60, 62-78); the entity inventories every upstream-taken target and names none for the pseudo vote"
    basis: tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 100
    section: "## 1. Test Scope Summary"
    change: "Add beside the F1 line, in the entity's gap-statement shape: `:open` matching a `details` or `dialog` carrying `open` — `ElementData::flush_open_state` setting `ElementState::OPEN`, written on construction and on an `open` attribute write or removal (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523), since 2026-10-27-upstream-sync-engine-debts — is read by no check: the take added none, and `details_element` and the `a11y` leg read green beside it;"
    sidecar: "test-scope-summary: `:open` on a `details` or `dialog` carrying `open` recorded as read by no check (the take added none; `details_element` and the a11y leg green beside it) — 2026-10-27-upstream-sync-engine-debts"
    rationale: "Report Changes › Symbols / APIs (`flush_open_state` at element.rs:543-549, called at :488-489 and from mutator.rs:368-371, :520-523; `NonTSPseudoClass::Open` reads the state, stylo.rs:510) and Coverage of new surfaces (`:open` → tests ✗, the commit adds no test; `details_element` and the a11y leg green); an engine behaviour change for every document with no check is a gap the inventory states"
    basis: packages/blitz-dom/src/node/element.rs:543-549
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 99
    section: "## 1. Test Scope Summary"
    change: "Extend the `focus_redraw` 4 clause, before `taken at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness`: — since 2026-10-27-upstream-sync-engine-debts the windowed shell routes its redraw requests through its event loop (upstream `6587e7e79`; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/window.rs:193-194), a route that holds no check of its own, read only by `focus_redraw` and the windowed `seven_guis_native` boot smoke —"
    sidecar: "test-scope-summary: the windowed shell's redraw route through its event loop recorded as holding no check of its own, read only by `focus_redraw` and the windowed boot smoke — 2026-10-27-upstream-sync-engine-debts"
    rationale: "Report Changes › Symbols / APIs (blitz-shell lib.rs:90, :93-98, :104-106, window.rs:193-194, the redraw route, `6587e7e79`) and Coverage of new surfaces (the shell's redraw route → tests smoke, the windowed `seven_guis_native` boot, + `focus_redraw` green); the entity's `focus_redraw` clause counts redraw requests on a `RecordingShell`, so the route's lack of a check of its own belongs beside it"
    basis: packages/blitz-shell/src/lib.rs:93-98
    fact: F10
note: the touched counts outside this group's one pick — `test-scope-summary.md:68` (tests/dom.rs 27 → 30 and its `:23-824` range), `:74` (dioxus-native-dom lib 81 → 82) and the s10 slice's file count behind the body's blitz-vibey-script inline-module absence — and F7 · F8 · F11 (dioxus-native-dom's IME test, the browser tab test module, the dropped `task_polled_during_render_can_access_document`) sit in the blitz-vibey-script, dioxus-native-dom and apps/browser entities, unpicked here, so the count sweep and the new-detail pass place them; F2 is integration-test-strategy's and F3 ci-integration's; D-tests-coverage reads `:open` (F9) and the redraw route (F10) as new paths with no test of their own, recorded here as gaps, not fixed; D-tests-framework and D-tests-obs-harness find no drift (no runner, harness, status or log-format change).
