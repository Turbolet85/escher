proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the stand-repeat parenthetical, keep the clause running `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` (48 files, … `stand_act_select` and `stand_snapshot_parts`; the count of 48 stays). After it add: ', then, since 2026-10-24-stand-requirement-sweep, ten named blitz-tests targets on the same line'. Change '`test_stand_repeat_leg_runs_three_rounds` naming it' to '`test_stand_repeat_leg_runs_three_rounds` naming it and, since that chunk, the ten words (.github/scripts/test_ci_workflows.py:359-362)'."
    sidecar: "ci-integration Legs: stand-repeat's blitz-tests line now runs ten named targets after the stand_*.rs glob, and its leg pin names them (2026-10-24-stand-requirement-sweep)"
    rationale: "Report Counts fact 1 moves ci-integration.md:8 ('one `--test` per … `stand_*.rs` — 48 files'), and fact 2 moves :8 for what `test_stand_repeat_leg_runs_three_rounds` pins. Schema / config: ci-leg.sh 54-58 holds the ten `--test` words, and test_ci_workflows.py 359-362 asserts them. The stand-files count of 48 is unmoved."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "After the stand_*.rs selection clause, add: 'The ten named targets are `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction` and `text_extent`. Each is a literal `--test {name}` word after the `stand_*.rs` glob, on the same `cargo test -p blitz-tests --locked` line every round, and each file keeps its name. The founder ratified this on 2026-10-24 with «Список в скрипте (Рекомендую)»; «Rename them stand_*» was passed over (.github/scripts/ci-leg.sh:54-58).'"
    sidecar: "ci-integration Legs: stand-repeat's ten named targets, literal --test words after the glob, on the founder's «Список в скрипте (Рекомендую)» (2026-10-24)"
    rationale: "Report Schema / config: ci-leg.sh stand-repeat 54-58, `stand+=(--test inert_subtree … --test text_extent)` after the glob, with a comment naming the founder's word. Decisions & corrections: inputs#I1, the files keep their names, «Rename them stand_*» passed over."
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "'`test_stand_repeat_leg_runs_three_rounds` pinning both words' → '`test_stand_repeat_leg_runs_three_rounds` pinning both words and, since 2026-10-24-stand-requirement-sweep, the ten named blitz-tests words (.github/scripts/test_ci_workflows.py:359-362)'; the rest of the seven_guis half is unchanged (`--locked`, no retry, no `--ignored`, no env var)."
    sidecar: "ci-integration Legs: the stand-repeat leg pin now also asserts the ten named words"
    rationale: "Report Counts fact 2 moves ci-integration.md:9 (it names what `test_stand_repeat_leg_runs_three_rounds` pins). Schema / config: `test_stand_repeat_leg_runs_three_rounds` (359-362) asserts the ten words, and the seven_guis half is unchanged."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 29
    section: "## 9. CI Integration"
    change: "After '… 28 runs over both builds, 0 needles (as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.md)', add: '; since 2026-10-24-stand-requirement-sweep a standing check also reads a recording session's clients' own stderr at `trace`: seven_guis `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`host_log` 3), green under the `-p seven_guis` build and the workspace build (examples/seven_guis/tests/host_log.rs:40-130)'."
    sidecar: "ci-integration Engine features by runner: a client's own stderr, read once at 2026-10-14, is now joined by a standing check at trace under --record, under both builds"
    rationale: "Report Counts fact 8 moves ci-integration.md:29, the 2026-10-14 one-off reading of a client's own stderr, which a standing check at trace under --record now joins. The Outcome reads `host_log` 3 under both builds (gates `-p seven_guis` and `--workspace`: 3 · 2)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 41
    section: "## 9. CI Integration"
    change: "Append after the `hit_test_dropped_nodes` reading: '; re-read at 2026-10-24-stand-requirement-sweep on CI#40114977936, the pre-CI commit `18b50a2a2` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 198 · 837 · 0 · 21. Over CI#40092092468 that is no new result line and +6 passed on linux, macos and coverage (`text_extent` 3, `stand_requirement_sweep` 1, `stand_lint` 1, `host_log` 1). Windows gains +5, because `host_log`'s new test is unix-gated. The stand repeat gains +30 result lines and +153 passed: the ten named targets join with 48 passed per round, and `stand_requirement_sweep`, `stand_lint` and `host_log` each add 1 per round — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/evidence/ci.md'."
    sidecar: "ci-integration Per-OS test tallies: the CI#40114977936 reading appended (206 · 1105; 187 · 1102; 187 · 1057; a11y 6 · 43; stand repeat 198 · 837)"
    rationale: "Report Counts fact 11 moves ci-integration.md:41 (the latest readings, with a new reading appended). Operator pass: CI#40114977936 on `18b50a2a20f3`, 17/17, with these per-job tallies. Fact 11 gives the base 1099 / 1096 / 1052 / 168 · 684. The gate running the ten reads 48 passed, and `host_log`'s new test is `#[cfg(unix)]`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 61
    section: "## 9. CI Integration"
    change: "Append after the `hit_test_dropped_nodes` re-count: '; re-counted at 2026-10-24-stand-requirement-sweep: 206 result lines, 1105 passed · 0 failed · 12 ignored — no new result line and +6 passed (`text_extent` 3, `stand_requirement_sweep` 1, `stand_lint` 1, `host_log` 1), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md)'."
    sidecar: "ci-integration Local baseline: the workspace re-count 206 · 1105 · 0 · 12 appended"
    rationale: "Report Counts fact 11 moves ci-integration.md:61 (the latest reading, with a new reading appended). Outcome gate: the workspace tally `lines 206 passed 1105 failed 0 ignored 12`. Changes: stand_requirement_sweep 2 → 3, stand_lint 6 → 7, text_extent 3 → 6, host_log 2 → 3, and no target added."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "'run the 48 `stand_*.rs` files and seven_guis' eight process checks three times with no retry' → 'run the 48 `stand_*.rs` files, the ten named blitz-tests targets (since 2026-10-24-stand-requirement-sweep, on the founder's «Список в скрипте (Рекомендую)», 2026-10-24; §9 Legs) and seven_guis' eight process checks three times with no retry'. Open the parenthetical with 'with the ten, CI#40114977936 read 198 · 837 · 0 · 21, +30 · +153, and the local leg read `stand-repeat: 3 of 3 rounds green`;' before 'with `stand_snapshot_parts` CI#39410865604 read 168 · 684 · 0 · 21'. Add (.github/scripts/ci-leg.sh:54-58) beside the leg's citation."
    sidecar: "quality-gates Flakiness budget: stand-repeat now runs the 48 stand files, the ten named targets and seven_guis' eight; CI#40114977936 read 198 · 837 · 0 · 21"
    rationale: "Report Counts fact 1 moves quality-gates-coverage-targets.md:22 ('run the 48 `stand_*.rs` files and seven_guis' eight'), and fact 11 moves its '168 · 684 · 0 · 21'. Harness / gate surface: `Stand checks repeated` at CI#40114977936 read 198 · 837 · 0 · 21. Outcome: the local `stand-repeat: 3 of 3 rounds green`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 374
    section: "## 9. CI Integration"
    change: "'the `stand-repeat` leg runs the stand checks and seven_guis' process checks for three rounds' → 'the `stand-repeat` leg runs the stand checks, ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds'; the rest of the line is unchanged."
    sidecar: "test-plan §9 Legs: stand-repeat also runs ten named blitz-tests targets"
    rationale: "Report Counts fact 1 moves test-plan.md:374 ('runs the stand checks and seven_guis' process checks'). Expected amendments: test-plan §9 Legs, ratified by the founder (2026-10-24) «Список в скрипте (Рекомендую)» (inputs#I1)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 410
    section: "## 10. Quality Gates & Coverage Targets"
    change: "'run the `stand_*.rs` files and seven_guis' process checks three times with no retry' → 'run the `stand_*.rs` files, ten named blitz-tests targets and seven_guis' process checks three times with no retry'."
    sidecar: "test-plan §10 Flakiness budget: the stand-repeat job also runs the ten named targets"
    rationale: "Report Counts fact 1 moves test-plan.md:410 ('run the `stand_*.rs` files and seven_guis' process checks'). Expected amendments: test-plan §10 Flakiness budget."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 28
    section: "## 4. Unit Test Strategy"
    change: "'two `stand-repeat` cases (the three rounds; the stop at the first red)' → 'two `stand-repeat` cases (the three rounds, which since 2026-10-24-stand-requirement-sweep also assert the ten named `--test` words; the stop at the first red)', and add .github/scripts/test_ci_workflows.py:359-362 to that clause's citation; test_ci_workflows.py's test count (33) is unchanged."
    sidecar: "unit-test-strategy CI workflows and leg script: the stand-repeat three-rounds case now also pins the ten named words"
    rationale: "Report Schema / config: `test_ci_workflows.py` `test_stand_repeat_leg_runs_three_rounds` (359-362) asserts the ten words. Four lines were added to an existing test, not a new test, so the CI-scripts count holds."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 21
    section: "## 5. Integration Test Strategy"
    change: "'still reading 0 needles on stderr at `trace`, its count (2 each) unmoved' → 'still reading 0 needles on stderr at `trace`, its count then unmoved (2 each; `host_log` 3 since 2026-10-24-stand-requirement-sweep, `mcp_log` still 2)'."
    sidecar: "integration-test-strategy Session host ↔ what it writes: host_log 2 → 3, mcp_log 2"
    rationale: "Report Changes: `examples/seven_guis/tests/host_log.rs`, 2 → 3 tests. Counts fact 8, `host_log` 2 → 3. Outcome: `host_log` 3 and `mcp_log` 2 under both builds. The '(2 each)' count on this line no longer holds for `host_log`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: "## 5. Integration Test Strategy"
    change: "After '… 28 runs with 0 needles, by the same kind of instrument (escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.md)', add: '— and since 2026-10-24-stand-requirement-sweep a standing check reads a recording session's clients' own stderr at `trace`: seven_guis `host_log`'s `a_recording_sessions_clients_log_nothing_of_it_at_trace` (examples/seven_guis/tests/host_log.rs:40-130)'."
    sidecar: "integration-test-strategy: the 2026-10-14 one-off reading of a client's own stderr is now joined by a standing check at trace under --record"
    rationale: "Report Counts fact 8 moves integration-test-strategy.md:22, the 2026-10-14 one-off reading of 'a client's own stderr', which a standing check at trace under --record now joins."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: "## 5. Integration Test Strategy"
    change: "Add: 'Since 2026-10-24-stand-requirement-sweep, seven_guis' `cfg(unix)` `a_recording_sessions_clients_log_nothing_of_it_at_trace` runs each client command as a process through `common::run` at `RUST_LOG=trace`: `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint` and `stop`. It uses the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started { dir, pid }` (examples/seven_guis/tests/host_log.rs:40-130).'"
    sidecar: "integration-test-strategy: host_log's recording-clients check, its six client commands at trace and its two state directories"
    rationale: "Report Symbols / APIs, `host_log.rs`: `#[cfg(unix)] a_recording_sessions_clients_log_nothing_of_it_at_trace` (40-130), its commands, its state directories and its guard. Counts fact 9 lists the state directories `hl-client` and `hl-client-rec`."
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: "## 5. Integration Test Strategy"
    change: "Add: 'Over all of those clients' stderr the check reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences, under the `-p seven_guis` build and the workspace build. It is also compiled for the `x86_64-pc-windows-gnu` target (`cargo check --tests --locked --target x86_64-pc-windows-gnu -p seven_guis`).'"
    sidecar: "integration-test-strategy: the recording clients' stderr reads 0 of each needle class at trace under both builds, and the check is compiled for the windows-gnu target"
    rationale: "Report Symbols / APIs, `host_log.rs`: 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles`. Outcome: `host_log` 3 under both builds. Decisions & corrections: the new `cfg(unix)` item was checked for `x86_64-pc-windows-gnu`."
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 21
    section: "## 5. Integration Test Strategy"
    change: "Add after the part-read clause: 'A paged `part` read through the `escher-session` host's own sink stays unmeasured, because the host boots only the four lean tasks and none of them pages. The measurement is owned by a new `.andromeda/residuals.md` line, «a paged `part` read through the escher-session host's sink» (target: next), approved by the operator's «yes — apply.» on the P5 card (2026-10-24, inputs#I2).'"
    sidecar: "integration-test-strategy: a paged part read through the host's own sink is stated as unmeasured and owned by a residuals line (target: next)"
    rationale: "Report Counts fact 7: a stated limit with an owner, inputs#I2's card row 1. The task's F12."
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 265
    section: "## 5. Integration Test Strategy"
    change: "After 'a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`', add: '; the clients of a recording session write no id, name, typed text or recording path on their own stderr at `trace`, and a standing check reads this'. The windowed-binary clause is unchanged."
    sidecar: "test-plan §5 Session host ↔ what it writes: a recording session's clients write nothing of it on their own stderr at trace, a standing check"
    rationale: "Report Counts fact 8 (`host_log` 2 → 3: the `start --record` clients' own stderr is read at `trace`, 0 needles, as a standing check). Expected amendments: test-plan §5, the `start --record` clients' own stderr read at `trace`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after the Inline HTML row: '| Shared fixture page (`rsx!`) | `many_items`: `ITEMS` (400) buttons under `#items`, ids `item-NNN`, a screen over the snapshot text budget. Since 2026-10-24-stand-requirement-sweep it is moved unchanged from `stand_snapshot_parts` into the shared fixtures module, and `stand_snapshot_parts` and `stand_act_spans` read it | (tests/blitz-tests/tests/fixtures/mod.rs:64-86) |'"
    sidecar: "test-data-fixtures: the shared page many_items (ITEMS = 400) moved into tests/blitz-tests/tests/fixtures/mod.rs"
    rationale: "Report Symbols / APIs, `fixtures/mod.rs`: `pub const ITEMS: usize = 400` and `pub fn many_items() -> Element` (64-86), moved unchanged from `stand_snapshot_parts.rs`. Counts fact 10: now read by `stand_snapshot_parts` and `stand_act_spans`."
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 320
    section: "## 7. Test Data & Fixtures"
    change: "Add to the shared pages after `covered` (`fx-under` under the `z-index: 5` overlay `fx-cover`; tests/blitz-tests/tests/fixtures/mod.rs:46-62): ', and since 2026-10-24-stand-requirement-sweep `many_items`, `ITEMS` (400) buttons under `#items`, a screen over the snapshot text budget, read by `stand_snapshot_parts` and `stand_act_spans` (tests/blitz-tests/tests/fixtures/mod.rs:64-86)'."
    sidecar: "test-plan §7 fixture pages: many_items (with ITEMS) joins password_labelled, password_named and covered"
    rationale: "Report Counts fact 10 moves test-plan.md:320 (the pages `password_labelled`, `password_named` and `covered`). Symbols / APIs: `fixtures/mod.rs` 64-86."
note: "No number is stated for these citation offsets, so they are left to cites.py: fixtures/mod.rs:17-130 (test-strategy:46-51) and :87-130 (mocking:14) sit past the 64-86 insertion; ci-leg.sh:48-65 and :18-67 (ci-integration:9, quality-gates:22) span the 54-58 insertion; test_ci_workflows.py:358-362, :323-371 and :378-492 (ci-integration:8, unit-test-strategy:28) sit at or past 359-362; host_log.rs:28-345, :34-38 and :251-264 (integration-test-strategy:21-22) sit past 11-15 and 40-131."
