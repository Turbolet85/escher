proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "In the host_log clause: since 2026-10-07-sink-target-allowlist `tests/host_log.rs` (3 tests, unix only; 2 until 2026-10-24-stand-requirement-sweep added `a_recording_sessions_clients_log_nothing_of_it_at_trace` — the clients of a `start crud --record` session, their own stderr read at `trace`, 0 needles of each class (examples/seven_guis/tests/host_log.rs:40-130); 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`) — the rest of the clause unchanged."
    sidecar: "test-scope-summary: host_log 2 → 3 — the start --record clients' own stderr at trace (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Changes list `examples/seven_guis/tests/host_log.rs` 2 → 3 tests and its Count 8 moves `test-plan-detail/test-scope-summary.md:15` (\"`tests/host_log.rs` (2 tests\"); the gate `cargo test -p seven_guis --locked --test host_log --test mcp_log --test recording` reads 3 · 2 · 4."
    basis: "examples/seven_guis/tests/host_log.rs:40"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "In the text-extent clause: by blitz-tests' `text_extent` 6 (3 until 2026-10-24-stand-requirement-sweep added the three limit tests), both layout modes, …; its own auto-discovered target, run by the workspace `test` leg, `run text_extent`, `run all` and, since 2026-10-24-stand-requirement-sweep, the `stand-repeat` leg, a literal `--test text_extent` in its named list (the founder's «Список в скрипте (Рекомендую)», 2026-10-24) — still not by `run stand`; its stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect) asserted since that chunk by those three tests (tests/blitz-tests/tests/text_extent.rs:127-205; .github/scripts/ci-leg.sh:54-58) — the select's and `PARALLEL_TRAVERSAL_LOCK`'s \"asserted by no check\" clauses unchanged."
    sidecar: "test-scope-summary: text_extent 3 → 6, its stated limits asserted, the target inside stand-repeat (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Count 5 moves `test-scope-summary.md:22` (`text_extent` 3; \"its stated limits … are asserted by no check\"), and Count 1 moves its runner clause (\"not by `run stand`\") as `stand-repeat` names `text_extent` among its ten (ci-leg.sh 54-58); the select's clause @c1162 and the lock's at :23 are no-change by Count 4."
    basis: "tests/blitz-tests/tests/text_extent.rs:127"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "…; `layout_as_text_input` has no unit test of its own, the select's stated limits are asserted by no check, nor is the process-wide `Parallel` traversal lock, while the text extent's stated limits are asserted by `text_extent` since 2026-10-24-stand-requirement-sweep"
    sidecar: "test-plan §1: the text extent's stated limits asserted by text_extent; the select's stay unasserted (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Count 5 moves `test-plan.md:19` (\"the text extent's … stated limits are asserted by no check\", the text extent's half only; the select's stays); text_extent's three limit tests pass in both layout modes under both builds (Outcome)."
    basis: "tests/blitz-tests/tests/text_extent.rs:127"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "After the text-extent clause: `text_extent`'s RTL and vertical limit tests, both layout modes, under `-p blitz-tests` and `--workspace` — `right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box` asserting first that `#rtl` carries `dir=\"rtl\"`, `vertical_text_reads_as_a_horizontal_line` asserting first that its twin's line is wider than it is high (tests/blitz-tests/tests/text_extent.rs:127-152; tests/blitz-tests/tests/text_extent.rs:154-174)"
    sidecar: "test-scope-summary: text_extent's RTL and vertical limit tests, each asserting its condition first (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Symbols list the two tests at text_extent.rs 127-152 and 154-174, both layout modes, green under `-p blitz-tests` and `--workspace`; Deviations (Step 3, the condition first) state the two first assertions."
    basis: "tests/blitz-tests/tests/text_extent.rs:127"
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "After the text-extent clause: `a_transformed_box_reads_its_lines_at_its_client_rect` — the content box equals `get_client_bounding_rect`, which is 2 × the box's own transform-blind `offset_rect` (its untransformed twin) about the same centre, and the lines are unscaled (tests/blitz-tests/tests/text_extent.rs:176-205)"
    sidecar: "test-scope-summary: text_extent's transformed-box limit test, its twin the box's own offset_rect (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Symbols state the test at text_extent.rs 176-205 and its reading; Deviations (Step 3, the transformed box) name `offset_rect` as the untransformed twin."
    basis: "tests/blitz-tests/tests/text_extent.rs:176"
    fact: F8
note: "Citations in the picked entities sit after lines this chunk added and may have shifted, not re-pointed here (no new number is stated by the report): test-scope-summary.md:8's `test_ci_workflows.py:378-492` (after the added 359-362), :15-19's `host_log.rs:28-345` and `host_log.rs:251-264` (after the added 11-15 · 40-131), :22's `text_extent.rs:42-118` (after the added 5-8 · 10 · 25-28) — for cites; F2-F6 and F9 (stand_requirement_sweep, stand_lint, stand_act_spans) have no picked entity to carry them and are left to the new-detail pass."
