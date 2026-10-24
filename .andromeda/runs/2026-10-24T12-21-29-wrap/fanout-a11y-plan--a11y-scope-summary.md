proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "Append to the Dioxus crates entry's re-read chain, after line 18's mcp-surface cite: `; re-read at 2026-10-24-upstream-sync-polish-and-ship on the ten-commit cherry-pick take and tokio 1.54.1: no package enters or leaves Cargo.lock (987 → 987) and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md`"
    sidecar: "a11y-scope-summary: the Dioxus crates entry records the 2026-10-24 upstream sync's re-read — lock 987 → 987, accesskit_(xplat|winit) 0 rows in the stand graph."
    rationale: "New detail, no violation: the report's Outcome gate list fires `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` green with last line 0, and Changes → Dependencies states no package enters or leaves Cargo.lock (987 → 987); this entry keeps a re-read per merged lock (2026-10-10-upstream-sync-agent-surfaces, 2026-10-10-mcp-surface), and line 18 is short, so the clause lands there under the 2 000-char line cap rather than on lines 16-17."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:188
note: "D-a11y-surface: no interactive UI element added (an engine sync; the one new test target is hit_test_dropped_nodes). D-a11y-obs-schema: unfired — report Changes → Schema / config says no scrub set, redaction shape or violation schema moved (escher-telemetry, escher-driver, dioxus-native-dom unchanged). Retired: none touches this file. Line 19's claim (browser `accessibility` not default) stands — the report says apps/browser's `default` gains only `\"mimalloc\"` (added line 14) — but its `apps/browser/Cargo.toml:14` / `:39` cites sit in a file the take touched that the citation sweep's list (report.md:95) omits; the orchestrator may re-check them."
