proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 124
    section: "## 6. Log Coverage"
    change: "add after line 124, inside the same absence block, one item — observed absent — a print, log site, span, field, subscriber, env read or listener in the codebase-health factoring's private items (escher-driver `lint.rs`'s `readings` and `placement`, `mcp_tests.rs`'s `#[track_caller]` helpers `assert_described` · `assert_schema` · `assert_property`, `stand_requirement_sweep`'s `assert_0_2_0_row` · `assert_0_2_0_witness` · `inside_stand_repeat`), their messages naming a row and an index only · searched — the silence census over `packages/escher-driver/src/lint.rs`, `packages/escher-driver/src/mcp_tests.rs` (its first reading in a census) and `tests/blitz-tests/tests/stand_requirement_sweep.rs`, reading 0; escher-driver still holding its 2 `info_span!` sites, no event or `#[instrument]`; no public symbol, span field or log line moves (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)"
    sidecar: "2026-10-26-codebase-health — Log Coverage absence block gains the silence census over the three factored files (lint.rs, mcp_tests.rs first censused, stand_requirement_sweep.rs), reading 0"
    rationale: "the report's Counts / qualifiers moved records the silence census reading 0 over the three touched files, mcp_tests.rs's first reading in a census, and leaves where that reading lands to P2; its Coverage of new surfaces states instrumentation n/a (census 0, `info_span!` 2, no event) for every new private item — a measured absence this category's absence·logs block records per chunk (as at lines 123-124)"
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:91
note: "no instrumentation, stack or PII drift (private refactor, no dependency, no log site, Retired none); the same census reading is also routed by the report to the body's obs-plan.md:93 (the lint's census line, §3) — the orchestrator should land mcp_tests.rs's first census reading in one place, this item or that body line, not both."
