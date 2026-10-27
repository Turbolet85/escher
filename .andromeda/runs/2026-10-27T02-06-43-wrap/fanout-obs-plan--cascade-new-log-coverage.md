proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 124
    section: "## 6. Log Coverage"
    change: "- observed absent — a print, a `tracing` site or a subscriber in escher-driver's out-of-line test file `mcp_tests.rs` (16 `#[test]`, 15 on unix), its env reads the 2 compile-time `env!` alone (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`) · searched: the silence census over the file, reading 0 — the first census it appears in — and `grep -c 'env!'` reading 2; escher-driver still holding its 2 `info_span!` sites, 0 events and 0 `#[instrument]` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)"
    sidecar: "obs-plan log-coverage: observed-absent record added for escher-driver's mcp_tests.rs — silence census 0, its first census, env reads the 2 compile-time env! (2026-10-26-codebase-health)"
    rationale: "report Changes › Counts / qualifiers moved, the silence census bullet (report.md:91-92: census 0 over lint.rs, mcp_tests.rs and stand_requirement_sweep.rs, mcp_tests.rs's first reading in a census) and Symbols / APIs (report.md:36-38: env! 2, 16 #[test], 15 on unix); Outcome obs and security rows (report.md:243-248: info_span! 2, no event or #[instrument]); the group's neighbours at log-coverage.md:123-124 are census records of the same shape"
    basis: .andromeda/obs-plan-detail/log-coverage.md:124
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 124
    section: "## 6. Log Coverage"
    change: "- observed absent — a print, a log site, an env read or a span in escher-driver's `lint` module's private helpers `readings` (an entry's `(Defect, Reason)` pairs in push order — zero-size alone for a box with no area, else placement, then truncated, then contrast) and `placement` (the off-screen / clipped match), which `findings` now walks per entry, `Session::lint` untouched · searched: the silence census over `lint.rs`, reading 0; escher-driver still holding its 2 `info_span!` sites, 0 events and 0 `#[instrument]` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)"
    sidecar: "obs-plan log-coverage: observed-absent record added for lint.rs's new private readings and placement helpers — census 0 over the file (2026-10-26-codebase-health)"
    rationale: "report Changes › Symbols / APIs (report.md:23-32: readings and placement, findings walking readings per entry, Session::lint untouched), the silence census bullet (report.md:91-92) and Coverage of new surfaces (report.md:175-176: instrumentation n/a, census 0, info_span! 2, no event); to follow F1's record so the two keep the task's order"
    basis: .andromeda/obs-plan-detail/log-coverage.md:124
    fact: F2
note: Both records follow line 124 in F1-then-F2 order; the body's §3 silence lines (obs-plan.md:91 the out-of-line escher-driver test files, obs-plan.md:93 the visual lint, the report's named P2 site for mcp_tests.rs) are standing entities of another section — extending them is the strong readers' edit, and these records do not replace it.
