proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "`ci-scripts` leg — 108 tests in eight test files (§4; 104 until 2026-10-25-cold-agent-bar, whose test_cold_agent.py went 33 → 37; 100 in seven until test_capability_statement.py added its 4; …), the rest of the line kept as written"
    sidecar: "test-scope-summary: the ci-scripts leg 104 → 108 (test_cold_agent.py 33 → 37), 104 kept as the dated reading"
    rationale: "Report Counts F9: `ci-scripts-104` stale at test-scope-summary.md:8; Outcome: `ci-scripts` Ran 108; Harness / gate surface: contract tests 33 → 37"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "`tests/host_log.rs` (4 tests, unix only; 3 until 2026-10-25-cold-agent-bar added `a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`: `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell first, then `snapshot --part 2` read through the host's own sink with 0 id and 0 name needles under both builds (examples/seven_guis/tests/host_log.rs:134-277). This is the paged part read the host could not take while it booted only the four lean tasks; 1 until 2026-10-19-session-recording …). The third test's `host_log.rs:42-132` citation is re-pointed to where it now stands, after the file grew at 15-17 and 134-278"
    sidecar: "test-scope-summary: host_log 3 → 4 tests, the Cells paged part read through the host's sink measured at 0 needles under both builds"
    rationale: "Report Counts F13: `host-sink-part` stale at test-scope-summary.md:15, host_log 3 → 4; Symbols: the new host_log test; New text: host_log.rs added 15-17 · 134-278, the test at 134-277"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 14
    section: "## 1. Test Scope Summary"
    change: "in the shared `tests/common/mod.rs` log checks' fragment: since 2026-10-25-cold-agent-bar, `cells_needles(part)` returns the author keys of part `part` of the in-process Cells screen and the middle column-A cell that part holds — the needles of `host_log`'s Cells part read (examples/seven_guis/tests/common/mod.rs:425-454). The fragment's `common/mod.rs:371-514` citation is re-pointed, since lines were added at 6-7 ahead of it"
    sidecar: "test-scope-summary: seven_guis' shared test module gains cells_needles(part)"
    rationale: "Task New facts F9; report Symbols: `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454); New text: common/mod.rs added 6-7 · 425-455"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:14
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 18
    section: "## 1. Test Scope Summary"
    change: "beside the two timer-step unit tests: since 2026-10-25-cold-agent-bar, three more stand unit tests — `stand::tests::the_stand_options_stay_offline` (every boot but `boot_load` keeps `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight` (examples/seven_guis/src/stand.rs:283-329). `cargo test -p seven_guis --locked --lib stand::` reads 5 passed. The timer-step tests' `stand.rs:331-343` citation is re-pointed past the new tests, and the lib's `3 passed` stays as a dated reading"
    sidecar: "test-scope-summary: the stand module's unit tests 2 → 5 (offline options, Cells boot, the load page's load in flight)"
    rationale: "Task New facts F3; report Symbols (the stand unit tests) and Gates: `cargo test -p seven_guis --locked --lib stand::` green (5 passed); New text: stand.rs added 283-330"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:18
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 18
    section: "## 1. Test Scope Summary"
    change: "`cli_commands` starts, reads and stops all six served tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (the four lean tasks until 2026-10-25-cold-agent-bar)"
    sidecar: "test-scope-summary: cli_commands starts the six served slugs, not four"
    rationale: "Report Symbols: `escher-session` `TASKS: [(&str, Served); 6]`; Outcome Smoke: `cli_commands` starts the six slugs through the binary; F1 (`escher-session` serves six tasks)"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "re-point the closing citations that this chunk's additions moved: `examples/seven_guis/tests/host_log.rs:30-492` (the file grew at 15-17 and 134-278), `examples/seven_guis/tests/common/mod.rs:1-514` (grew at 6-7 and 425-455) and `examples/seven_guis/src/stand.rs:246-344` (grew at 1-9, 12, 17, 131-245 and 283-330), each to the range it now spans; add the measurement source escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md for host_log's fourth test and the stand's three unit tests"
    sidecar: "test-scope-summary: the host_log, common/mod.rs and stand.rs citations re-pointed after the cold-agent bar's additions"
    rationale: "Report Counts F13: citation at test-scope-summary.md:19 (`host_log.rs:N`, its body shifted by lines 134-278); New text lists the added ranges of common/mod.rs and stand.rs"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 61
    section: "## 1. Test Scope Summary"
    change: "after `lint.rs 2 (…)`: since 2026-10-25-cold-agent-bar, a measured defect that no unit test pins — the lint's `overlaps` (packages/escher-driver/src/lint.rs) reads a select's options as overlapping, because the snapshot gives each option its select's client rect. It is unresolved, with its owner to be pinned at the cold-agent bar's P5; on the founder's word `stand_lint` is re-pinned to the measured finding (`the_stand_at_boot_reads_its_measured_findings`, the flight booker's one `overlap`)"
    sidecar: "test-scope-summary: escher-driver's lint row states the unresolved select-option overlap defect, pinned as measured by stand_lint"
    rationale: "Task New facts F8; report Insufficient fixes (`stand_lint` re-pinned, the driver defect in lint.rs `overlaps` unresolved, owner at P5) and Decisions (inputs#I5); the driver's own unit counts stand (diff over packages 0)"
    basis: .andromeda/test-plan-detail/test-scope-summary.md:61
    fact: F8
note: F1 (the stand's boot_cells / boot_load and the load page's ids, and that neither is a LeanTask) and F4 · F5 (stand_cells_served, stand_load_page) fit the blitz-tests stand-check entries of test-scope-summary (lines 93-123 by the report), none of which was picked here, so the new-detail pass places them.
