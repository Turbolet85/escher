proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 8
    section: "## 6. E2E Test Strategy"
    change: "`cli_commands` (14): each of the four tasks starts, answers and stops" → "`cli_commands` (14): each of the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, the last two since 2026-10-25-cold-agent-bar, booted by `stand::boot_cells` / `stand::boot_load`) starts, answers and stops through the binary"
    sidecar: "e2e-test-strategy §6 Command line — `cli_commands` starts the six slugs `escher-session` serves (was the four lean tasks), Cells and the load page added at 2026-10-25-cold-agent-bar"
    rationale: "The report's Symbols / APIs give `escher-session` `TASKS: [(&str, Served); 6]` (session_host.rs:12-27) and its Outcome smoke line states `cli_commands` starts the six slugs through the binary; Coverage of new surfaces lists `cli_commands` as a test of `start cells` — F1 `served-tasks` (escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:8
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 19
    section: "## 6. E2E Test Strategy"
    change: "`host_binary` launches the `escher-session` binary, which boots one lean task headlessly and serves its lifecycle" → "`host_binary` launches the `escher-session` binary, which boots one task headlessly — one of the four lean tasks or, since 2026-10-25-cold-agent-bar, Cells or the load page — and serves its lifecycle"
    sidecar: "e2e-test-strategy §6 absence — the binary `host_binary` launches boots one of six served tasks, not only a lean one"
    rationale: "Second occurrence of the claim F1 retires (the binary serves only the four lean tasks): `escher-session`'s `enum Served { Lean(LeanTask), Cells, Load }` (session_host.rs:12-27), and `host_log` now runs `serve cells` (F13) — escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:19
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 12
    section: "## 6. E2E Test Strategy"
    change: "after the MCP item's trial reading add: since 2026-10-25-cold-agent-bar the cold-agent pipe's one live run (test-plan §3 → Cold-agent run pipe) drove the `mcp` role, run with `--record`, over all six served tasks — counter, flight booker, timer, CRUD, Cells and the load page — on the operator host, not in CI: `passed`, 66 tool calls, 0 wrong, `recording_kept`, `claude` 2.1.308 (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json)"
    sidecar: "e2e-test-strategy §6 MCP — the 2026-10-25 cold-agent live run as a real client's drive of the `mcp` role over the six served tasks, beside the 2026-10-10 trial"
    rationale: "The report's Outcome smoke line states the live run drove the `mcp` role over all six; v020-11 MET with `tasks_met` all six, 66 calls, 0 wrong, `recording_kept`, Claude Code 2.1.308 (F14, F15) — escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md; binds with test-plan §3 Cold-agent run pipe's Proof (test-plan.md:187), whose own update is that section's."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:11
    fact: F15
note: the `examples/seven_guis/tests/cli_commands.rs:N` citations on lines 8-10 may have moved (16 lines added at 3-5 · 37-41 · 137 · 266 · 592-597) — the P2 citation sweep's; line 8's 10,000-byte snapshot reading is dated on the four lean tasks and left as is (F16 a leaf); no detector found drift for D-tests-framework or D-tests-obs-harness, and no Retired item is named in this file.
