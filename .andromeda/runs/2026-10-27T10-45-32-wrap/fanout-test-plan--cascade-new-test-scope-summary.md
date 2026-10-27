proposals:
  - detector: D-new-detail
    severity: warning
    fact: F8
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 19
    section: ## 1. Test Scope Summary
    change: "- **apps/browser (tab):** since 2026-10-27-upstream-sync-engine-debts (upstream `d63ed2c8c`, the upstream-only browser app) `apps/browser/src/tab.rs` holds a test module, the browser's sixth tested file beside about_pages, browser_history, favicon, url_suggestions and persistence, with one `#[test]`, `tab_lens_survives_webview_props_replacement` — a tab's lenses survive a replacement of its webview props — run by the workspace `test` leg, its one passed among the tally's +8 (apps/browser/src/tab.rs:295-343) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md"
    sidecar: test-scope-summary gains the apps/browser (tab) record — tab.rs's one test module and its test tab_lens_survives_webview_props_replacement, taken with upstream d63ed2c8c
    rationale: the report's Changes name apps/browser tab lenses surviving a webview props replacement with test module 295-343 (`d63ed2c8c`) and the workspace tally's +8 passed counting apps/browser/src/tab.rs 1; no standing record of this file names the tab test
    basis: .andromeda/test-plan-detail/test-scope-summary.md:14
note: the body's apps/browser inventory (test-plan.md:13) and the apps entity's "five browser files above" at test-scope-summary.md:14 now read six with tab.rs — standing-entity edits left to the strong readers per the task's Touches
