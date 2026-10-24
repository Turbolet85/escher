proposals:
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In **Legs**' stand-repeat clause, after the `stand_*.rs` glob's file list (48 files, unmoved) and before `test_stand_repeat_leg_runs_three_rounds` naming it, add: and, since 2026-10-24-stand-requirement-sweep, on the same `cargo test -p blitz-tests --locked` line every round, ten named targets outside the `stand_` prefix, a fixed `--test` list in the script, the files keeping their names — `--test inert_subtree --test accessibility_states --test scrolled_box_absolute_position --test hit_clipped_at_scrolling_box --test password_caret_click --test transformed_client_rect --test accessibility_active_descendant_unread --test virtual_clock_engine --test select_interaction --test text_extent` (the founder's «Список в скрипте (Рекомендую)», 2026-10-24; .github/scripts/ci-leg.sh:54-58) — `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` reading the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18 named targets (the ten and seven_guis' eight) from the script's text; the seven_guis half, the a11y and coverage legs unchanged"
    sidecar: "ci-integration §9 Legs: stand-repeat runs the stand_*.rs glob plus ten named blitz-tests targets (founder 2026-10-24), read back by stand_requirement_sweep's leg check"
    rationale: "The report's Schema / config moves stand-repeat's selection — ten `--test` words after the `stand_*.rs` glob on the same line (ci-leg.sh 54-58), ratified by the founder (inputs#I1); Counts fact 1 lists ci-integration.md:8 as Moved (\"one `--test` per … `stand_*.rs` — 48 files\"), the leg's text now naming more than the glob."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "`test_stand_repeat_leg_runs_three_rounds` pinning both words → pinning both words and, since 2026-10-24-stand-requirement-sweep, the ten named blitz-tests targets' `--test` words (.github/scripts/test_ci_workflows.py:359-362)"
    sidecar: "ci-integration §9 Legs: the stand-repeat leg pin gains the ten named targets' words"
    rationale: "Counts fact 2 (`leg-pin`) lists ci-integration.md:8 · :9 as Moved — they name what `test_stand_repeat_leg_runs_three_rounds` pins, and the report's Schema / config says that test (359-362) now asserts the ten words."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:113
    dependent-of: D-tests-framework
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 29
    section: "## 9. CI Integration"
    change: "Append after the 2026-10-14-quality-gates reading (28 runs, 0 needles): since 2026-10-24-stand-requirement-sweep a client's own stderr is also a standing check — `host_log`'s `cfg(unix)` `a_recording_sessions_clients_log_nothing_of_it_at_trace` (examples/seven_guis/tests/host_log.rs:40-130) runs `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint` and `stop` as client processes at `RUST_LOG=trace`, in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, and reads over all the clients' stderr 0 id, 0 name, 0 sentinel, 0 recording-path and 0 recorded needles; `host_log` 3 and `mcp_log` 2 green under both builds"
    sidecar: "ci-integration §9 Engine features by runner: the start --record clients' own stderr read at trace by host_log as a standing check (host_log 2 → 3), 0 needles under both builds"
    rationale: "Counts fact 8 (`host-log`) lists ci-integration.md:29 as Moved — the 2026-10-14 one-off reading of a client's own stderr is now joined by a standing check at `trace` under `--record`; Changes › Symbols states the test, its commands and its 0-needle readings, and Outcome reads `host_log` 3 and `mcp_log` 2 under both builds."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 29
    section: "## 9. CI Integration"
    change: "Follow the client-stderr sentence on this line with: a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — while a paged read is measured in process, `stand_act_spans` reading part 2 of a fixture screen that pages (`fixtures::many_items`) in its sink capture, 36 calls per mode; the host-role reading is owned by a `.andromeda/residuals.md` line «a paged `part` read through the escher-session host's sink» (target: next)"
    sidecar: "ci-integration §9 Engine features by runner: the host-role paged part read stated as an owned limit beside the in-process paged read"
    rationale: "Counts fact 7 (`paged-limit`): the limit is new text, placed beside fact 8's sites, ci-integration.md:29 among them; its owner is inputs#I2's Beyond-the-plan row 1, a residuals line placed at this wrap; fact 6 and Changes › Symbols state stand_act_spans' part 2 read (685-693), 35 → 36 calls."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 41
    section: "## 9. CI Integration"
    change: "Append after the `hit_test_dropped_nodes` reading: ; re-read at 2026-10-24-stand-requirement-sweep on CI#40114977936, the pre-CI commit `18b50a2a2` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 198 · 837 · 0 · 21 — no result line and +6 passed on linux, macos and coverage over CI#40092092468 (`text_extent` +3, `stand_lint` +1, `stand_requirement_sweep` +1, `host_log` +1), +5 on windows (`host_log`'s new test unix-only), +30 result lines and +153 passed on the stand repeat (the ten named targets entering it, 48 passed a round, with `stand_lint`, `stand_requirement_sweep` and `host_log` +1 each, three rounds) — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/evidence/ci.md"
    sidecar: "ci-integration §9 Per-OS test tallies: CI#40114977936 reading appended (linux 206 · 1105, stand repeat 198 · 837)"
    rationale: "Counts fact 11 (`tallies`) lists ci-integration.md:41 as Moved, the latest reading with a new reading appended: linux 206 · 1099 → 206 · 1105, macos and coverage 187 · 1096 → 187 · 1102, windows 187 · 1052 → 187 · 1057, stand repeat 168 · 684 → 198 · 837, a11y unmoved; Outcome records the per-job tallies of CI#40114977936."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:180
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 61
    section: "## 9. CI Integration"
    change: "Append after the `hit_test_dropped_nodes` re-count: ; re-counted at 2026-10-24-stand-requirement-sweep: 206 result lines, 1105 passed · 0 failed · 12 ignored — no result line added and +6 passed (`text_extent` 3, `stand_lint` 1, `stand_requirement_sweep` 1, `host_log` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`)"
    sidecar: "ci-integration §9 Local baseline: workspace re-count 206 · 1105 appended"
    rationale: "Counts fact 11 lists ci-integration.md:61 as Moved (workspace 206 · 1099 → 206 · 1105); the Outcome's workspace tally gate reads `lines 206 passed 1105 failed 0 ignored 12`; Changes › Files gives the four test-count moves (text_extent 3 → 6, stand_lint 6 → 7, stand_requirement_sweep 2 → 3, host_log 2 → 3)."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:365
note: "Lines 8-9 cite .github/scripts/ci-leg.sh:48-65 and :18-67 and .github/scripts/test_ci_workflows.py:358-362, ranges the chunk added lines inside (ci-leg.sh 54-58, test_ci_workflows.py 359-362) — a cites.py re-point, not proposed here by number; the body's own §9/§10 broad lines (test-plan.md:374, :410) are Moved by fact 1 but lie outside this file."
