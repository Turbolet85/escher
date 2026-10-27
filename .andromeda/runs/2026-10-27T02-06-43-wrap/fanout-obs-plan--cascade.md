proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 123
    section: ## 6. Log Coverage
    change: "Add after line 123, in the absence group's shape: `- observed absent — a print, log site, span, field, env read or subscriber in the 2026-10-26-codebase-health factoring's private test helpers (`stand_requirement_sweep`'s `#[track_caller]` `assert_0_2_0_row` and `assert_0_2_0_witness` and its target predicate `inside_stand_repeat`; escher-driver `mcp_tests.rs`'s `#[track_caller]` `assert_described`, `assert_schema` and `assert_property`), their assertion messages naming a row and an index only, every message unchanged · searched: the silence census over `lint.rs`, `mcp_tests.rs` and `stand_requirement_sweep.rs`, reading 0 (exit 1) (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its Counts / qualifiers moved census bullet)`"
    sidecar: "log-coverage §6 absence: the codebase-health factoring's six private test helpers recorded silent (census 0), beside the 0.2.0 requirement record's test additions"
    rationale: "The report's Changes add six private test helpers (Symbols / APIs, `mcp_tests.rs` and `stand_requirement_sweep.rs`) and its census bullet reads 0 over the three files; Coverage of new surfaces states instrumentation n/a and PII n/a, messages naming row and index only — F3; entry 9.3's own claim still holds, the WITNESSES_020 table and leg check being unchanged."
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:91
    fact: F3
note: F1 (`mcp_tests.rs` silent, 2 `env!` reads, its first census) belongs with the out-of-line test files' silence group and F2 (`lint.rs`'s `readings` and `placement` silent) beside the lint's census line obs-plan.md:93 (body §3) — no picked entity carries either, so the new-detail pass places them; no detector fires otherwise (the new operations are private, non-hot-path, silent; no logger, stack or PII change).
