proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 21
    section: "## 4. Unit Test Strategy"
    change: "after line 21, beside the escher-driver record, add the record: - **escher-driver (MCP listing test helpers):** since 2026-10-26-codebase-health mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` keeps its name, its outer assertions and its row walk and asserts each listed tool through three private `#[track_caller]` helpers — `assert_described` (the description and its result fields), `assert_schema` (the input schema's properties and their order) and `assert_property` (one argument's kind, enum and maximum) — whose messages name the row and index only; every assertion kept, the file still 16 `#[test]` (15 on unix) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md"
    sidecar: "unit-test-strategy: the MCP listing test's three #[track_caller] per-row helpers recorded beside the escher-driver unit-test record (F3, 2026-10-26-codebase-health)"
    rationale: "the report's Symbols / APIs (mcp_tests.rs bullet) and Outcome (listing test, every assertion kept, mcp_tests.rs 15 on unix, controls (c) (d)) state the helpers; the entity's mcp_tests.rs clauses name no listing-test structure"
    basis: ".andromeda/test-plan-detail/unit-test-strategy.md:18"
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 42
    section: "## 9. CI Integration"
    change: "after line 42, a continuation line of the Per-OS test tallies record: Re-read at 2026-10-26-codebase-health on CI#40434241601, the pre-CI commit `3c04042ed` (green 17/17): linux 208 · 1117 · 0 · 12; macos and coverage 189 · 1114 · 0 · 12; windows 189 · 1068 · 0 · 9; the a11y job 14 · 85 · 0 · 0; the stand repeat 204 · 864 · 0 · 21 — every job equal to the base's CI#40389488295 (`19f9ff91ca`), the chunk adding no test; the moves since CI#40134371519 — +2 result lines and +12 passed on linux, macos and coverage, +2 · +11 on windows, +1 passed on the a11y job, +6 · +27 on the stand repeat — the intervening chunks', never written here and not attributed per target here — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md"
    sidecar: "ci-integration: the per-OS tally record carried to CI#40434241601 (2026-10-26-codebase-health), equal to its base run"
    rationale: "the report's Cross-project / external claims gives CI#40434241601's six libtest job tallies equal to CI#40389488295's; the record's last reading is CI#40134371519 (linux 206 · 1105, a11y 14 · 84, stand repeat 198 · 837), so it no longer states the current tallies"
    basis: ".andromeda/test-plan-detail/ci-integration.md:42"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 65
    section: "## 9. CI Integration"
    change: "in line 65, replace «`ci-leg.sh coverage` exit 0, its instrumented test run 404 · 0 · 3 (cargo-llvm-cov runs no doctests), before 2026-10-06-telemetry-bootstrap's +9 tests / +1 ignored, not re-run locally since — as measured at escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/gates.md» with «`ci-leg.sh coverage` exit 0, its instrumented test run 404 · 0 · 3 (cargo-llvm-cov runs no doctests), before 2026-10-06-telemetry-bootstrap's +9 tests / +1 ignored — as measured at escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/gates.md; re-run on the dev host at 2026-10-26-codebase-health's implement, green, `coverage floor: 3 of 3 crates at or above their floors`, its instrumented run's tally not recorded there (CI#40434241601's `Coverage report` job 189 · 1114 · 0 · 12) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»"
    sidecar: "ci-integration: the local baseline's «coverage not re-run locally since» corrected — the leg ran green on the dev host at 2026-10-26-codebase-health"
    rationale: "the report's Outcome lists `bash .github/scripts/ci-leg.sh coverage` green at implement (and quality-gates-coverage-targets.md:16 already cites 2026-10-25-cold-agent-bar's own coverage gate), so «not re-run locally since» is false"
    basis: ".andromeda/test-plan-detail/ci-integration.md:65"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "at the end of line 16, after «(as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its `ci-leg.sh coverage` gate)», add: ; read again at 2026-10-26-codebase-health, the floors and `coverage_floor.py` unchanged: escher-driver 4930/5031 = 97.99 % (+12 counted lines, +12 covered, the lint's findings walk split into private helpers), dioxus-native-dom 2741/3202 = 85.60 % and blitz-test-harness 952/1008 = 94.44 % unchanged, `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its `ci-leg.sh coverage` gate, evidence/counts.md row 15)"
    sidecar: "quality-gates-coverage-targets: escher-driver's coverage reading carried to 4930/5031 = 97.99 % (2026-10-26-codebase-health)"
    rationale: "the report's Counts / qualifiers moved names this site (quality-gates-coverage-targets.md:16, last reading 4918/5019 = 97.99 %) as the one changed hit of pattern `coverage`, the reading now 4930/5031 = 97.99 %, the floor 97 % holding"
    basis: ".andromeda/test-plan-detail/quality-gates-coverage-targets.md:16"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 28
    section: "## 10. Quality Gates & Coverage Targets"
    change: "after line 28, beside the mutation tier, add the record: - **Complexity ceiling:** since 2026-10-26-codebase-health (v030-01) a cognitive-complexity ceiling of 15 is read on the dev host with rust-code-analysis-cli 0.0.25, test functions counted — no CI leg, a host tool no lockfile resolves; at that chunk 85 functions read over 15 (88 at its base), among them blitz-dom's `clear_interaction_state_for_removed_node` (17.0) and `get_text_selection_ranges` (32.0), both left alone — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (evidence/complexity.md)"
    sidecar: "quality-gates-coverage-targets: the dev-host cognitive-complexity ceiling of 15 recorded beside the mutation tier (F1, 2026-10-26-codebase-health)"
    rationale: "the report's Counts / qualifiers moved (population 88 → 85), Dev-tool versions (rust-code-analysis-cli 0.0.25, no lockfile), Harness / gate surface (a dev-host reading, no CI leg) and Deviations (get_text_selection_ranges 32.0) state it; like the mutation tier it is a dev-host-only gate on a host tool"
    basis: ".andromeda/test-plan-detail/quality-gates-coverage-targets.md:28"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 28
    section: "## 10. Quality Gates & Coverage Targets"
    change: "after line 28, following the complexity ceiling record, add the record: - **Complexity reading's sum:** rust-code-analysis's `cognitive.sum` adds nested spaces into their parent, so a closure or nested `fn` keeps its branches counted in the enclosing function and only splitting a function into module-level sibling items lowers its reading (`findings` read 16 over 3 spaces, each closure 0) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (research.md §Patterns detected)"
    sidecar: "quality-gates-coverage-targets: how the complexity reading sums nested spaces recorded beside the ceiling (F2, 2026-10-26-codebase-health)"
    rationale: "the report's Decisions & corrections states the rust-code-analysis fact the chunk's form rests on; it qualifies the ceiling record F1 adds"
    basis: ".andromeda/test-plan-detail/quality-gates-coverage-targets.md:28"
    fact: F2
note: "body §10 (.andromeda/test-plan.md:405-417) has no broad line for the complexity ceiling F1/F2 add, and body §4's Assertion helpers line (test-plan.md:202) names only assert_solves — the orchestrator may want a broad line for each; the stand-repeat reach gap (F5) credits no line in this group, its sites in test-scope-summary (group 1)"
