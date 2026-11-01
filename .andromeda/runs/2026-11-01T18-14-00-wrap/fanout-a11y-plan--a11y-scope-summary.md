proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 19
    section: "## 1. A11y Scope Summary"
    change: "Append to the end of the Dioxus crates entity, after its last `as measured at` clause, the prior takes' re-reads kept as history: ; re-read on the upstream take read at `e42ee1f25` (merge base `23b9b26a9`): `Cargo.toml`, `Cargo.lock`, `deny.toml` and every `*/Cargo.toml` unchanged by the take, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `^accesskit_(xplat|winit) ` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "§1 Dioxus crates: the stand graph's platform-adapter absence re-read on the upstream take at `e42ee1f25`, manifests and lock unchanged, 0 rows; the `ebacd68bd` re-read stays as history."
    rationale: "report.md:35 keeps `a11y-scope-summary.md:19` as the last take's history, no-change as such, with any sentence for this take beside it; report.md:30 and report.md:137 hold the manifests, lock and `deny.toml` unchanged, and report.md:155 holds the `cargo tree` census at 0 rows, the reading this entity records on each take. report.md:99 names only §3 and §6 for a11y-plan, so this reading is new detail for §1 rather than a planned amendment."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:155
note: "D-a11y-surface and D-a11y-obs-schema do not fire here: the report adds no interactive UI element (report.md:102-109) and moves no violation schema (report.md:31). No Retired item stands in this file, and report.md:58 reads `a11y-scope-summary.md:24` no-change on focus; the F1 A limit's new sentence belongs to §5 (keyboard-navigation), not this category."
