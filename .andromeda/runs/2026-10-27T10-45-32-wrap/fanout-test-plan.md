proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope → apps/browser'
    change: 'The apps/browser bullet lists tab (1, `tab_lens_survives_webview_props_replacement`, taken from upstream at 2026-10-27-upstream-sync-engine-debts) beside about_pages (3), browser_history (4), favicon (5), url_suggestions (12 `#[test]` plus 4 `#[tokio::test]`) and persistence (10), so its «tests exist only in» list names every browser file that holds a test'
    sidecar: '§1 apps/browser — tab.rs joins the list of browser files holding tests: upstream d63ed2c8c adds `tab_lens_survives_webview_props_replacement` (2026-10-27-upstream-sync-engine-debts)'
    rationale: 'The report Symbols / APIs bullet says apps/browser tab lenses survive a webview props replacement with a test module in `apps/browser/src/tab.rs` (d63ed2c8c), its Counts bullet `ws-tally` counts +1 passed in `apps/browser/src/tab.rs`, and Coverage of new surfaces lists the unit test `tab_lens_survives_webview_props_replacement`; the body bullet says tests exist only in five other browser files, now false'
    basis: 'apps/browser/src/tab.rs:301-342'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → What unit tests cover → apps/browser'
    change: 'The apps/browser bullet adds the tab lens case — a tab keeps its lenses across a webview props replacement (`tab_lens_survives_webview_props_replacement`, since 2026-10-27-upstream-sync-engine-debts) — beside the about-page, history, favicon, suggestions and persistence cases'
    sidecar: '§4 apps/browser — the tab lens unit test joins the browser cases (upstream d63ed2c8c, 2026-10-27-upstream-sync-engine-debts)'
    rationale: 'Same fact as the primary: the report Symbols / APIs and Coverage of new surfaces bullets add the browser unit test `tab_lens_survives_webview_props_replacement`; §4 restates the browser test inventory case by case and omits it'
    basis: 'apps/browser/src/tab.rs:301-342'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours'
    change: 'The blitz-dom covered-behaviours bullet adds the gap: `:open` matching a `details` or `dialog` carrying `open` (`ElementData::flush_open_state`, since 2026-10-27-upstream-sync-engine-debts) has no test of its own — `details_element` and the a11y leg read green beside it'
    sidecar: '§1 blitz-dom — `:open` on `details` / `dialog` (upstream, 2026-10-27-upstream-sync-engine-debts) recorded as asserted by no check of its own'
    rationale: 'The report Coverage of new surfaces marks `:open` on `details` / `dialog` (every document) tests ✗ — the commit adds no test, `details_element` and the a11y leg green; the Symbols / APIs bullet names `ElementData::flush_open_state` and `NonTSPseudoClass::Open` now reading that state; §1 records untested blitz-dom behaviours on this bullet and does not name this one'
    basis: 'packages/blitz-dom/src/node/element.rs:543-549'
  - detector: D-tests-framework
    severity: warning
    section: '§3 Test Harness Contract → Agent-run contract → Proof'
    change: 'The Proof record gains its next entry — re-counted at 2026-10-27-upstream-sync-engine-debts, `scripts/agent-run.sh` unchanged: `run stand`s `run.end` unmoved at passed 205 · failed 0 · ignored 7 over 50 files, and `run all` passed 633 · failed 0 · ignored 12, +3 over 630 (upstream `pseudo_element_flow_classification` 3, a new file with no `stand_` prefix, picked up by `run all` with no script change)'
    sidecar: '§3 Proof — the agent-run record re-counted at 2026-10-27-upstream-sync-engine-debts: run stand 205 · 0 · 7 over 50 unmoved, run all 630 → 633 · 0 · 12 by the new target'
    rationale: 'The report Counts bullet reads `run all` run.end 630 · 0 · 12 → 633 · 0 · 12 (+3, the new target) and `run stand` 205 · 0 · 7 over 50 files unmoved; Harness / gate surface says no `agent-run.sh` change, the new target picked up by `run all` and not by `run stand`; the Proof bullet holding test-plan.md:172 is the per-chunk record of these readings, each earlier upstream sync adding its `run all` entry in this form, and its last entry reads `run all` not re-counted'
    basis: 'test-plan.md:172'
note: 'D-tests-obs-harness and D-retired found no drift (no harness, status or log-format change; Retired none). The `:open` gap is unowned in the report — it may want a route owner at route-resolve. The report `run-all` sweep row reads 0 changed because it matched only the 0.1.0 reading at test-plan.md:172; the Proof bullet holding it carries the per-chunk run all record, hence the fourth proposal. Count moves into category files (bt-files, dom-rs, dnd-lib, ws-tally) are the cascade.'
