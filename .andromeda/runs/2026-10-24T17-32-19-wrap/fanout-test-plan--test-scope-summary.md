proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: '## 1. Test Scope Summary'
    change: 'In the text-extent clause, "`text_extent` 3" → "`text_extent` 6 (3 until 2026-10-24-stand-requirement-sweep)", and "its stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect) are asserted by no check" → "its stated limits asserted since 2026-10-24-stand-requirement-sweep, each in both layout modes, green under `-p blitz-tests` and `--workspace`, on the page paragraphs `#ltr` · `#rtl` (`dir="rtl"`) · `#vertical` (`writing-mode: vertical-lr`) · `#scaled` (`transform: scale(2)`) read through the helper `extent`: right-to-left text reads as a left-to-right line from the content box (`right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box`, `#rtl` carrying `dir="rtl"` asserted first), vertical text reads as a horizontal line (`vertical_text_reads_as_a_horizontal_line`, its twin''s line asserted wider than high first), and a transformed box reads its lines at its client rect — the content box equal to `get_client_bounding_rect`, which is 2 × `offset_rect` about the same centre, the lines unscaled (`a_transformed_box_reads_its_lines_at_its_client_rect`) (tests/blitz-tests/tests/text_extent.rs:25-28; tests/blitz-tests/tests/text_extent.rs:120-125; tests/blitz-tests/tests/text_extent.rs:127-152; tests/blitz-tests/tests/text_extent.rs:154-174; tests/blitz-tests/tests/text_extent.rs:176-205)"; the select''s limits clause on the same line stays.'
    sidecar: 'test-scope-summary: text_extent 3 → 6, its three stated limits (RTL, vertical, transformed box) now asserted (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Symbols (text_extent.rs: three tests, both layout modes, green under -p blitz-tests and --workspace) and Counts fact 5 (extent-limits), which names test-scope-summary.md:22 moved: "text_extent 3; its stated limits … are asserted by no check".'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:138-142
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 42
    section: '## 1. Test Scope Summary'
    change: '`password_caret_click` "an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand`" → "an engine check on a parsed page with no `stand_` prefix, so not selected by `run stand`, and run by the stand-repeat leg by name since 2026-10-24-stand-requirement-sweep — one of the ten blitz-tests targets outside the `stand_` prefix that the leg names by a fixed `--test` list after its `stand_*.rs` glob, on the same `cargo test -p blitz-tests --locked` line every round (.github/scripts/ci-leg.sh:54-58), ratified by the founder (2026-10-24) «Список в скрипте (Рекомендую)»".'
    sidecar: 'test-scope-summary: password_caret_click now also run by stand-repeat by name, one of the ten named targets (founder, 2026-10-24).'
    rationale: 'Report Changes → Schema / config (ci-leg.sh stand-repeat 54-58 names ten targets, password_caret_click among them) and Counts fact 1 (leg-select / ten-witnesses), which names test-scope-summary.md:42 moved ("not selected by `run stand`"); Decisions: the founder''s word inputs#I1.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66-72
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: '## 1. Test Scope Summary'
    change: 'In the text-extent clause, "its own auto-discovered target, run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`" → "its own auto-discovered target, run by the workspace `test` leg, `run text_extent`, `run all` and, since 2026-10-24-stand-requirement-sweep, the stand-repeat leg by name, not by `run stand`".'
    sidecar: 'test-scope-summary: text_extent run by stand-repeat by name since 2026-10-24-stand-requirement-sweep.'
    rationale: 'Report Counts fact 1 lists test-scope-summary.md:22 (text_extent "not by `run stand`") among the rows whose runner clause gains stand-repeat; the agent-run selections are unchanged (Harness / gate surface).'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:97-99
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 96
    section: '## 1. Test Scope Summary'
    change: '`accessibility_states` "run by the workspace `test` leg and per package and not by the `a11y` leg" → "run by the workspace `test` leg, per package and, since 2026-10-24-stand-requirement-sweep, the stand-repeat leg by name, and not by the `a11y` leg".'
    sidecar: 'test-scope-summary: accessibility_states run by stand-repeat by name since 2026-10-24-stand-requirement-sweep; the a11y leg''s six unchanged.'
    rationale: 'Report Counts fact 1 names test-scope-summary.md:96 (accessibility_states) moved; Changes → Schema / config leaves the a11y leg unchanged.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:97-99
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 99
    section: '## 1. Test Scope Summary'
    change: 'The closing runner clause after `inert_subtree` 4, "each its own auto-discovered target, run by the workspace `test` leg and per package and not by the `a11y` leg''s six" → "each its own auto-discovered target, run by the workspace `test` leg and per package — `accessibility_active_descendant_unread` and `inert_subtree` also by the stand-repeat leg by name since 2026-10-24-stand-requirement-sweep, `accessibility_merged_states` not — and not by the `a11y` leg''s six".'
    sidecar: 'test-scope-summary: inert_subtree and accessibility_active_descendant_unread run by stand-repeat by name since 2026-10-24-stand-requirement-sweep.'
    rationale: 'Report Counts fact 1 names test-scope-summary.md:99 (the inert_subtree and active-descendant clause) moved; the ten named in ci-leg.sh hold inert_subtree and accessibility_active_descendant_unread, not accessibility_merged_states.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:66-72
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: '## 1. Test Scope Summary'
    change: '`select_interaction` "(run by the workspace `test` leg, by name and by `run all`, not by `run stand`)" → "(run by the workspace `test` leg, by name, by `run all` and, since 2026-10-24-stand-requirement-sweep, by the stand-repeat leg by name, not by `run stand`)"; the `stand_act_select` "by its prefix" clause stays.'
    sidecar: 'test-scope-summary: select_interaction run by stand-repeat by name since 2026-10-24-stand-requirement-sweep.'
    rationale: 'Report Counts fact 1 names test-scope-summary.md:122 (select_interaction "not by `run stand`") moved, and keeps the line''s "by its prefix" rows as still true.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:97-105
    dependent-of: D-tests-framework
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: '## 1. Test Scope Summary'
    change: '`tests/host_log.rs` "(2 tests, unix only; 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`)" → "(3 tests, unix only; 2 until 2026-10-24-stand-requirement-sweep, 1 until 2026-10-19-session-recording added a recording host that logs nothing of its check at `trace`)", and after "beside 0 ids and 0 names" add: "; and since 2026-10-24-stand-requirement-sweep the clients'' own stderr under a recording session, `a_recording_sessions_clients_log_nothing_of_it_at_trace` (`#[cfg(unix)]`): each client command run through `common::run` with `RUST_LOG=trace`, in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started { dir, pid }` — `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint`, `stop` — reading over all the clients'' stderr 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences (examples/seven_guis/tests/host_log.rs:40-130)".'
    sidecar: 'test-scope-summary: host_log 2 → 3, the start --record clients'' own stderr read at trace with 0 needles (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Files/Symbols (host_log.rs 2 → 3 tests, the new test at 40-130) and Counts fact 8 (host-log), which names test-scope-summary.md:15 ("`tests/host_log.rs` (2 tests") moved; Outcome: host_log 3 under both builds, 0 of each needle class.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:162-168
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: '## 1. Test Scope Summary'
    change: 'After the new host_log clause the D-tests-coverage proposal at line 15 writes (ending "(examples/seven_guis/tests/host_log.rs:40-130)"), add: "; a paged `part` read through the `escher-session` host''s own sink stays unmeasured — the host boots only the four lean tasks and none pages, the paged read measured in process by `stand_act_spans` alone — owned by a `.andromeda/residuals.md` line «a paged `part` read through the escher-session host''s sink» (target: next), placed at this chunk''s wrap on the operator''s word «yes — apply.» on P5 card 2 (inputs#I2)".'
    sidecar: 'test-scope-summary: the host-role paged part read stated as an owned limit beside host_log (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Counts fact 7 (paged-limit): a stated limit with an owner, new text "placed beside fact 8''s sites" — test-scope-summary.md:15 is fact 8''s site in this file; Decisions → the operator''s word on P5 card 2, Beyond-the-plan row 1.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:158-161
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 108
    section: '## 1. Test Scope Summary'
    change: '`stand_act_spans` "35 calls per mode — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone —" → "36 calls per mode (35 until 2026-10-24-stand-requirement-sweep) — one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone, and, since that chunk, a paged part read: the shared page `fixtures::many_items` held as `paged` through `fixtures::hold` and read by `snapshot_part(2)` as `Outcome::Part { part: 2, parts ≥ 2 }` (tests/blitz-tests/tests/stand_act_spans.rs:685-693), `Needles::read` also taking the `#items > button` texts as name needles and the page''s 400 `item-NNN` ids joining the id needles (tests/blitz-tests/tests/stand_act_spans.rs:243), no id or name of the paged page on a line —".'
    sidecar: 'test-scope-summary: stand_act_spans 35 → 36 calls per mode, a paged part 2 read in the sink capture (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Symbols (stand_act_spans.rs: 685-693, 243, 727) and Counts fact 6 (spans-calls), which names test-scope-summary.md:108 ("35 calls per mode — one `snapshot` with `part` 1 among them") moved; Outcome: 36 calls per mode with a paged part 2 of ≥ 2, no id or name of the paged page on a line.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:151-157
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: '## 1. Test Scope Summary'
    change: '"`stand_requirement_sweep` 2" → "`stand_requirement_sweep` 3 (2 until 2026-10-24-stand-requirement-sweep)"; and after the `WITNESSES` clause (`every_requirement_names_its_witness`, byte-unchanged, 16 rows) add: "and, since 2026-10-24-stand-requirement-sweep, the 0.2.0 record `WITNESSES_020` — 9 rows v020-01…09 in order, 46 witnesses, a witness''s subject now also `Subject::Unit` (a crate''s unit test, run by the workspace `test` leg, outside `stand-repeat` by construction), v020-01 and v020-02 holding `Unit` witnesses alone (six unit tests in `packages/blitz-dom`, `packages/escher-driver`, `packages/dioxus-native-dom` and `examples/seven_guis/src/tasks/timer.rs`) and v020-08 one (`node/select.rs`) — beside `STAND_REPEAT_NAMED` (the ten named targets), `STAND_REPEAT_SEVEN_GUIS` (the eight seven_guis targets), `CI_LEG` and 28 new `source()` arms, `.github/scripts/ci-leg.sh` among them through `include_str!`; its test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` asserts the nine rows in order, each naming a witness found as `fn {test}(` in its file, v020-01/02 `Unit` alone and every other row a non-`Unit` witness, every non-`Unit` blitz-tests witness a `stand_*` file or one of `STAND_REPEAT_NAMED` and every `examples/seven_guis/tests/` witness one of the eight, and the leg''s text holding the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18 names (tests/blitz-tests/tests/stand_requirement_sweep.rs:227-229; tests/blitz-tests/tests/stand_requirement_sweep.rs:538-818; tests/blitz-tests/tests/stand_requirement_sweep.rs:820-832; tests/blitz-tests/tests/stand_requirement_sweep.rs:834-844; tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)".'
    sidecar: 'test-scope-summary: stand_requirement_sweep 2 → 3, the 0.2.0 record WITNESSES_020 and its stand-repeat leg check (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Symbols (stand_requirement_sweep.rs: Subject::Unit, WITNESSES_020, STAND_REPEAT_NAMED, STAND_REPEAT_SEVEN_GUIS, the new test and its assertions; WITNESSES byte-unchanged) and Counts fact 3 (sweep-record), which names test-scope-summary.md:117 moved.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:25-39
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: '## 1. Test Scope Summary'
    change: '"`stand_lint` 6" → "`stand_lint` 7 (6 until 2026-10-24-stand-requirement-sweep)"; "its sixth, on the `lint` verb" stays.'
    sidecar: 'test-scope-summary: stand_lint 6 → 7 (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Files (stand_lint.rs 6 → 7 tests) and Counts fact 4 (lint-coread), which names test-scope-summary.md:120 ("`stand_lint` 6") moved.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:124-128
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: '## 1. Test Scope Summary'
    change: '"a clipped or off-screen node holding text also reading `truncated` is asserted by no check" → "and, since 2026-10-24-stand-requirement-sweep, a clipped or off-screen node holding text also reading `truncated` is asserted — its seventh, `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`, in both layout modes, on the in-file page `reading_page()`: `fx-cut-text` (a nowrap line past its 100 × 40 `overflow: hidden` box) reads `clipped` and `truncated`, `fx-padded-text` (a 60 px paragraph whose `padding-right: 100px` alone passes its box) `clipped`, `fx-off-text` (wholly right of the 800 px viewport) `truncated` and `off-screen` — the `fx-cut-text` truncated reading with a visible part narrower than its text, the `fx-off-text` one with none — and the `lint` verb answering the same (tests/blitz-tests/tests/stand_lint.rs:110-141; tests/blitz-tests/tests/stand_lint.rs:226-274)".'
    sidecar: 'test-scope-summary: the lint co-reading (clipped or off-screen with truncated) now asserted by stand_lint (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Symbols (stand_lint.rs: reading_page() 110-141 and the new test 226-274, its five findings, the verb answering the same), Deviations step 2 (rows 1 and 3 told apart) and Counts fact 4, which names test-scope-summary.md:121 ("is asserted by no check") moved.'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:40-45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 123
    section: '## 1. Test Scope Summary'
    change: '`stand_snapshot_parts` "on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`" → "on the shared fixture page `fixtures::many_items` (`ITEMS` = 400; in this file until 2026-10-24-stand-requirement-sweep moved it and `ITEMS` unchanged into `tests/fixtures/mod.rs`, where `stand_act_spans` reads it too — tests/blitz-tests/tests/fixtures/mod.rs:64-86), whose screen passes `SNAPSHOT_TEXT_BUDGET`".'
    sidecar: 'test-scope-summary: stand_snapshot_parts'' paging page is now the shared fixtures::many_items (2026-10-24-stand-requirement-sweep).'
    rationale: 'Report Changes → Files/Symbols (fixtures/mod.rs: `pub const ITEMS: usize = 400` and `pub fn many_items()` at 64-86, moved unchanged from stand_snapshot_parts.rs, which now reads `fixtures::many_items`) and Counts fact 10 (fixture-pages: now read by stand_snapshot_parts and stand_act_spans).'
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:57-58
note: The chunk's insertions shift citation ranges the file still holds (text_extent.rs:42-118 at :22, stand_act_spans.rs:555-567 at :108, stand_requirement_sweep.rs ranges at :117, stand_lint.rs ranges at :120-121, host_log.rs ranges at :17 and :19, fixtures/mod.rs:17-130 at :90) — no new coordinate is stated by the report, so they are left to cites.py; D-tests-obs-harness reads no drift (agent-run selections and the log format unchanged).
