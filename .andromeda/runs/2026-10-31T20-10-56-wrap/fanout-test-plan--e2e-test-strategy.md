proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 8
    section: "## 6. E2E Test Strategy"
    change: "In the Command line bullet, `cli_commands` (14) now reads `cli_commands` (15); nothing else on the line moves."
    sidecar: "E2E Test Strategy: `cli_commands` count 14 → 15, its new test the hosted select list (2026-10-31-select-popup)."
    rationale: "Report, Counts / qualifiers moved, `cli-count`: `cli_commands` 14 → 15, e2e-test-strategy:8 named among the sites changed; the line still reads 14. The 11 passed on line 9 is a dated CI witness and stands."
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 10
    section: "## 6. E2E Test Strategy"
    change: "New item after the Command line bullet: - **Command line, the select list (since 2026-10-31-select-popup):** `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses` drives a `select` through a hosted process: a click opens its list, a `screenshot` of the open list answers a `png` equal to the one taken in process, and a click on a row chooses it, the host's state directory `cc-popup` under `CARGO_TARGET_TMPDIR` (examples/seven_guis/tests/cli_commands.rs:419-464)"
    sidecar: "E2E Test Strategy: new item, the command-line check of the select list through the binary (2026-10-31-select-popup)."
    rationale: "Report, Coverage of new surfaces, `the list's pixels on the screenshot carriers` (`cli_commands` hosted shot equal to in-process), the Test-only state directories bullet (`cc-popup`) and New text, by line (cli_commands.rs 419-464). The body's broad line on the Command line driver stands as written."
    basis: examples/seven_guis/tests/cli_commands.rs:419-464
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 12
    section: "## 6. E2E Test Strategy"
    change: "New item after the MCP bullet: - **MCP, the select list (since 2026-10-31-select-popup):** `mcp_log` gains one test, `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`: a list opened and chosen through the `mcp` role, 0 option needles on its stderr at `trace`; it records nothing and holds no `ml-popup` directory, since recording there would have needed a `cfg!(unix)` site (examples/seven_guis/tests/mcp_log.rs:68-113)"
    sidecar: "E2E Test Strategy: new item, `mcp_log`'s check of a select list through the role, recording nothing (2026-10-31-select-popup)."
    rationale: "Report, Coverage of new surfaces (process: `cli_commands`, `host_log`, `mcp_log` +1 each), Outcome's security row (`host_log`, `mcp_log` 0 needles at `trace`), Reverted / negative API facts (no recording, no `ml-popup`) and New text, by line (mcp_log.rs 68-113). The report states +1 and no total, and `mcp_log` (1) and (2) on lines 11-12 are dated readings, so neither is edited."
    basis: examples/seven_guis/tests/mcp_log.rs:68-113
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 18
    section: "## 6. E2E Test Strategy"
    change: "New item after Cells' handle calls: - **The select list, headless:** the list is painted through `paint_scene` alone, no shell or renderer moved, and driven through the harness, the session and the binary; its windowed path read and not run (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "E2E Test Strategy: new item, the select list measured headlessly, its windowed path read and not run (2026-10-31-select-popup)."
    rationale: "Report, Outcome's (arch) row on `paint_scene` (`packages/blitz-shell` and `packages/dioxus-native` absent from the 41, windowed list read and not run) and Coverage of new surfaces, the same limit; it is the shape of line 18's Cells limit and serves the body's NOT YET MEASURED on windowed runs."
note: "D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-retired find no violation in this file; `host_log`'s new test is left out, since this file names that target only on line 25 and its detail belongs to the body's §3 and §5."
