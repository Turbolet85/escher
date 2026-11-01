proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 74
    section: '## 5. Integration Test Strategy'
    change: '- **DOM ↔ resource loading, a stylesheet that stops blocking:** `render_blocking_stylesheet` adds three tests — `failed_stylesheet_unblocks_rendering`, a failed load; `empty_or_non_css_stylesheet_unblocks_rendering`, over an empty body, an HTML body and a lone `\xff` byte; and `invalid_stylesheet_url_does_not_block_rendering`, a bad `href` starting no load (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)'
    sidecar: 'Integration Test Strategy gains a record of the three stylesheet tests `render_blocking_stylesheet` adds on the DOM ↔ resource loading boundary.'
    rationale: 'F9; the report states `render_blocking_stylesheet` +3 (report.md:37, report.md:103), what a failed, empty, non-CSS or bad-`href` stylesheet now does (report.md:14) and the three tests by name (report.md:383, report.md:387, report.md:389); the file holds no record of this target to extend.'
    basis: tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-107
    fact: F9
note: near 74 puts the new record inside the last procedure block, before its closing marker at line 76, since the file holds no group of this boundary to join
