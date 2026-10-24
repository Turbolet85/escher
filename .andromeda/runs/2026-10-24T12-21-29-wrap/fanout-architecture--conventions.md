proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 15
    section: '## Conventions'
    change: 'Append to the **Manifests** entry (conv-manifests), before its docs.rs clause: the WPT runner takes `mimalloc` by its own version requirement, `mimalloc = "0.1.48"`, and not through `[workspace.dependencies]`, the root manifest unchanged by the take, a measured exception to "declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`" (wpt/runner/Cargo.toml:15)'
    sidecar: 'conventions.md Manifests: the WPT runner''s direct `mimalloc = "0.1.48"` recorded as an exception to the workspace-dependency convention (2026-10-24-upstream-sync-polish-and-ship, upstream 9eeebb302)'
    rationale: 'The report''s Changes → Dependencies (report.md:31) makes mimalloc "a direct dependency of the WPT runner (`mimalloc = "0.1.48"`, one added lock edge)", and Changes → Crates / modules (report.md:27) says "the root manifest is byte-identical" — so the new declaration is a version string, not `{ workspace = true }`, while conventions.md:15 states every dependency is declared once in `[workspace.dependencies]`; New text, by line lists wpt/runner/Cargo.toml added 15'
    basis: .andromeda/architecture-detail/conventions.md:15
note: 'The take moves lines under many conventions.md citations — `layout/inline.rs:278`, `:501` (line 27) and `:245-251` (line 46) now under `layout/inline/layout.rs` (report.md:10, :89 "conventions.md 2 — changed"), and the `construct.rs`, `stylo.rs`, `node/{node,element,mod}.rs`, `mutator.rs`, `resolve.rs`, `layout/{mod,replaced}.rs`, dioxus-native `lib.rs` and `wpt/runner/src/main.rs` cites on lines 8, 15, 21, 27, 39, 40, 46, 52 and 54 — left to P2''s citation sweep (`cites.py apply`, report.md:95), which owns path and digits; and `PARALLEL_TRAVERSAL_LOCK`''s poison recovery `unwrap_or_else(|e| e.into_inner())` (stylo.rs added 29-35, 157-163) is a new instance of the body''s Conventions → Error handling line (architecture.md:118), which has no detail file here.'
