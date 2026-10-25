# Report — 2026-10-25-cold-agent-bar

**Chunk:** Cold-agent bar — the stand extended to every 0.2.0 check, a fresh agent over it via the cold-agent pipe,
judged by the founder's bar
**Date:** 2026-10-25
**Commits:** `55bf34259 chore(2026-10-25-cold-agent-bar): operator pre-CI commit, for the run this chunk's verdict reads`
(the one commit since `last_wrap` 2026-10-25T12:31:26Z; base `e84dc818`, its parent — `git log --format='%h %s'
e84dc818..HEAD`). Pushed: `origin/build/escher-0.2.0` `e84dc818 → 55bf3425` (operator entry 26). Uncommitted after it:
`evidence/operator-pass.md`.

## Changes (structured — detectors read this)
- **Files:** 26 changed (`gate.py scope`: changed 26 · listed 24 · recorded 2), from base `e84dc818`:
  - new: `tests/blitz-tests/tests/stand_cells_served.rs` (209 lines), `tests/blitz-tests/tests/stand_load_page.rs`
    (84 lines);
  - stand code: `examples/seven_guis/src/tasks/flight_booker.rs`, `examples/seven_guis/src/stand.rs`,
    `examples/seven_guis/src/session_host.rs`;
  - seven_guis checks: `examples/seven_guis/tests/{cli_commands,host_log,common/mod}.rs`;
  - blitz-tests shared modules: `tests/blitz-tests/tests/{common/mod,session_common/mod}.rs`;
  - re-pinned stand checks: `stand_{keyboard,contrast,act_states,snapshot_state,act_disabled,act_ids,act_diff,
    element_ids,id_persistence,snapshot,session_recording,requirement_sweep}.rs` and the two recorded companions
    `stand_lint.rs`, `stand_accessibility_ids.rs`;
  - the pipe: `scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`;
  - ledgers and chunk records: `escher-0.2.0/verification-matrix.json` (v020-11), the chunk's `evidence/` and
    `inputs/` (I5, I6), `scope-record.md`.
  `stand_diff.rs` needed no change: Book still moves eight nodes, seven by bounds alone (entry 3 green).
- **Symbols / APIs** (coordinates from `## New text, by line`):
  - `seven_guis::stand::boot_cells(HarnessOptions) -> Harness<DioxusDocument>` (stand.rs:131-135) — mounts
    `Task::Cells` in `TaskShell` through `cells_root` (137-139); Cells stays no `LeanTask` (the plan's rejected fifth
    variant). Callers: `session_host.rs`, `stand_cells_served`, seven_guis `common::cells_needles`, the unit test.
  - `seven_guis::stand::boot_load(HarnessOptions)` (141-152) — the load page in `TaskShell` titled "Load"
    (`load_root` 177-183, `LoadPage` 185-200, its CSS `.load-root` · `.load-body` · `.load-btn` (+`:hover`
    `:active`) · `.load-done` 203-243), its options' `net_provider` replaced by `Some(Arc::new(LoadNet))` through
    `..options`; every field else kept. `pub const LOAD_ANSWER_DELAY: Duration` = 200 ms. Private: `LOAD_URL`
    `http://stand.test/load.css`, `LOADED_CSS` `.load-body .load-done { display: block; }`, `struct LoadNet`
    (`impl NetProvider` 167-175: each `fetch` answers `LOADED_CSS` from a thread of its own after
    `LOAD_ANSWER_DELAY`; no `is_noop` override, so the harness counts its load; no `blitz_net`, print, env read, socket
    or listener — silence census entry 10 reads 0). `stand::options` keeps `net_provider: None` for every other caller
    (unit test `the_stand_options_stay_offline`).
  - The load page's ids: `load-fetch` (Button "Load", the stand accent `#3a5ce5` / white, hover `#2a4cd3`, active
    `#1f3fc1`) and `load-done` (Paragraph "Loaded", `display: none` until the sheet lands). No CSS selects by id.
  - The flight booker (flight_booker.rs:66-86): the two buttons `flight-one-way` · `flight-return` (`aria_pressed`,
    `.type-btn` / `.type-btn-active`, `.flight-type-row`) are gone; one `select#flight-type` (`aria-label` "Flight
    type", class `type-select`) holds `option#flight-type-one-way` (value `one-way`, "one-way flight") and
    `option#flight-type-return` (value `return`, "return flight"), each `selected:` from the `flight_type` signal;
    `oninput` writes the signal by a closed match (`"return"` → return, every other value → one-way). Its CSS
    (160-171): 15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10; `:focus` drops the outline
    for the `#3a5ce5` border, as the date inputs do. The snapshot reads it `ComboBox "Flight type"` with value
    `one-way` and two `ListBoxOption`s (selected / unselected), each at the select's client rect.
  - `escher-session` (session_host.rs:12-27): a private `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS:
    [(&str, Served); 6]` — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`; the usage line's `apps:`
    and the MCP `start` tool's `enum` follow (program's own fixed text). The Timer keeps `with_time(timer_step)`.
  - blitz-tests `session_common::change` moves the flight booker by `type flight-type return` through the driver;
    `common` tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order`, `ENABLED_TAB_ORDER`, `rendered`.
  - seven_guis tests `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454): part `part`'s
    author keys of the in-process Cells screen and the middle column-A cell it holds.
  - `stand_requirement_sweep`: a `Subject::Served` (a page served beside the lean four) and three `WITNESSES_020` rows
    — v020-07 `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-08
    `stand_act_states.rs::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`, v020-09
    `stand_cells_served.rs::cells_is_walked_in_parts_through_the_driver`; v010-17's flight witness and v020-05's lint
    witness re-pointed to the renamed tests; `source` carries the two new files.
  - Renamed tests: `stand_act_states::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` →
    `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`;
    `stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed` → `exactly_the_chosen_flight_type_reads_selected`;
    `stand_lint::the_stand_at_boot_reads_no_finding` → `the_stand_at_boot_reads_its_measured_findings`. New tests:
    `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`,
    `stand_snapshot::the_flight_type_select_holds_its_two_options`, seven_guis `host_log::
    a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`, the stand unit tests
    `the_stand_options_stay_offline` · `cells_boots_in_the_shell` · `the_load_page_click_leaves_a_load_in_flight`, and
    the two new files' six tests.
  - The pipe (`scripts/cold-agent.sh`): `STAND_PROMPT` names six apps (counter, flight booker, timer, CRUD, cells, the
    load page) and their goals — the flight booker's return booking chosen on its flight type control plus one
    screenshot; Cells `A0` 5, `B1` `=A0*2` reading 10, `Z25` `=B1+1` reading 11 plus one visual lint; the load page's
    resource loaded and message shown — still naming no stand id. `DONE` adds `cells` (`GenericContainer "10"
    id="cell-B1"`, `GenericContainer "11" id="cell-Z25"`) and `load` (`Paragraph "Loaded" id="load-done"`). New
    functions `paged` · `screen_text` (178-193), `part_reads` (231-241), `started_in_order` (244-254), `recording`
    (257-271); `span_counts` also returns accepted `tool_call` lines per `COUNTED` verb (`lint` · `screenshot` ·
    `advance`); `tasks_met` reads accepted snapshot answers only, resets an app on its accepted `start`, and joins the
    latest answer of each part of a paged screen; the replay's expect match also reads a paged answer's decoded text.
    The live server child runs `mcp --record "$3"` into `target/cold-agent/record` (made `mkdir -m 700`); replays run
    without `--record`. Verdict fields added: `wrong_call_bar_pct` (5) · `lint_calls` · `screenshot_calls` ·
    `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`; reasons added `capability-unused` ·
    `recording-not-kept` · `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`), after `task-unmet` and
    before the check's. `wrong_calls` now decides.
- **Crates / modules:** none added or removed; seven_guis' `stand` module grows (above).
- **Dependencies:** none (`git diff --name-only e84dc818 -- … Cargo.toml Cargo.lock deny.toml …` reads 0, entry 11;
  `LoadNet` uses `blitz-traits`, already a native seven_guis dependency).
- **Schema / config:** the pipe's verdict schema (the seven fields and three reasons above) and its state set
  (`target/cold-agent/record/`, owner-only, `cleanup` removing it with the state). No driver schema, verb, argument,
  cause, session error, span field or log line moves.
- **Spec-master edits:** none.
- **Retired:** the flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) and their CSS
  (`.flight-type-row`, `.type-btn`, `.type-btn-active`) — they were the stand's only `aria-pressed` controls; no census
  row.
- **Counts / qualifiers moved** (each a moved fact with its pattern id from `{run_dir}/report-patterns.toml`, the
  sweep `cascade.py sweep --pre-pass` at baseline `55bf3425`; a row's disposition: `stale` — this chunk moved the
  fact · `citation` — a `path:N` whose line this chunk's edit moved, the P2 citation sweep's · `dated` — a reading
  taken at a named earlier point · `no-change` — a true claim sharing the token):
  - F1 `escher-session` serves six tasks (was the four lean tasks); the stand boots Cells and the load page beside the
    lean four — `served-tasks`: 56 standing rows · stale 13 (architecture.md:163, architecture.md:187,
    existing-scopes.md:118, standard-contracts.md:218 (lint, also F3), standard-contracts.md:233, input-validation.md:66,
    logging-monitoring.md:14, layout-templates.md:46 (F3), layout-templates.md:75 @c189, test-plan.md:267,
    integration-test-strategy.md:21, pii-scrubbing-compliance.md:11, screen-reader-support.md:37 (F3)) · citation 13
    (`session_host.rs:N`: cross-cutting-patterns.md:14, occupied-resources.md:38 · :44, standard-contracts.md:234,
    logging-monitoring.md:8 · :15, threat-model-summary.md:25, layout-templates.md:10 @c2262, layout-templates.md:75
    @c7705 · @c7752, obs-plan.md:10 · :71, log-coverage.md:34, session-lifecycle.md:9 @c3817) · no-change 30 (a check's
    scope over `LeanTask::ALL`, unchanged — existing-scopes.md:108 · :112 · :115, standard-contracts.md:163 · :214,
    test-scope-summary.md:93 · :96 · :104 · :105 · :115 · :118 · :119, obs-plan.md:76 · :77, a11y-plan.md:62 · :188,
    screen-reader-support.md:17 · :38, visual-design-verification.md:11; test children named `session_host…`:
    occupied-resources.md:41, test-plan.md:172, ci-integration.md:57, test-scope-summary.md:102, test-strategy.md:14,
    obs-plan.md:79; "four files" / "four-row" sharing the token: obs-plan.md:90 · :92, test-scope-summary.md:74;
    `escher-session <verb>` grammar: standard-contracts.md:232, session-lifecycle.md:9 @c51; layout-templates.md:10
    @c6026 "outside the lean four") — screen-reader-support.md:38 and visual-design-verification.md:14 · session-
    lifecycle.md:11 @c481 to be re-read at P2.
  - F2 the flight booker's mode is the select `flight-type` (Tab: `back-btn` → `flight-type` → `flight-start` →
    `flight-book`; enabled: + `flight-return-date`, five stops, reached by Tab and ArrowDown) — `flight-mode`: 16
    standing rows, all stale (existing-scopes.md:106 · :107 · :112, color-palette.md:27, layout-templates.md:10
    @c2481 · @c2500 · @c3863 · @c3877 · @c3898, test-plan.md:146, integration-test-strategy.md:17, test-scope-
    summary.md:108 · :116 · :117 · :118, a11y-plan.md:196, keyboard-navigation.md:38 · :98, screen-reader-
    support.md:44, visual-design-verification.md:12).
  - F3 the lint at boot: the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`, the two
    options at their select's client rect), the other three lean tasks 0 — `lint-at-boot`: 3 standing rows, stale 3
    (layout-templates.md:46, test-scope-summary.md:123, visual-design-verification.md:14), plus the F1 rows marked F3.
  - F4 the stand's controls 15 → 14 (`controls(task)` rows 18 → 17) — `controls-15`: 3 standing, stale 3
    (test-plan.md:146, test-scope-summary.md:93 · :104).
  - F5 `stand_contrast`: control rows 18 → 17 (tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1), `STATE_ROWS` 19 → 18 (tally 18 ·
    0 · 1 · 0 → 17 · 0 · 1 · 0); the `flight-one-way` accent row (white on `#3a5ce5`, 5.47, 14px 600) and the
    `flight-return` row (`#444444` on `#fafafa`, 9.33) give way to `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px
    400); return-chosen rows: `flight-type` 17.40 and `flight-return-date` 17.40 (was `flight-return` 5.47 ·
    `flight-one-way` 9.33 · `flight-return-date` 17.40) — `contrast-rows`: 7 standing · stale 4 (existing-scopes.md:112,
    test-scope-summary.md:116, a11y-plan.md:64, visual-design-verification.md:12) · dated 1 (bootstrap-phases-derive-
    for-route-setup-project.md:3, a discharge record naming 18 rows) · no-change 2 (obs-plan.md:76, a11y ci-
    integration.md:12, naming the tables).
  - F6 `stand_act_ids`: of 77 snapshot nodes a click is accepted on 68 and refused on 9 (was 70 · 7 — the two options
    refused `covered`); 14 controls (was 15), 14 changing at boot — `act-ids-census`: 13 standing · stale 1
    (standard-contracts.md:163) · citation 5 (existing-scopes.md:110, standard-contracts.md:151, error-handling.md:9,
    input-validation.md:201, test-scope-summary.md:113 — `stand_act_ids.rs:1-15` · `:60`, its doc now 1-16) ·
    no-change 7 (the file named in lists; `stand_act_ids` 2 tests unchanged).
  - F7 `stand_keyboard`: the activation table 20 → 16 rows (`BUTTONS` 10 → 8), its tests 14 → 15 — `activation-rows`:
    5 standing · stale 4 (existing-scopes.md:106, test-scope-summary.md:117, keyboard-navigation.md:38 · :97) ·
    no-change 1 (test-scope-summary.md:31, another table's 20 rows).
  - F8 the a11y leg 14 files, 84 → 85 tests — `a11y-84`: 6 standing · stale 3 (test-plan ci-integration.md:8, a11y
    ci-integration.md:10, visual-design-verification.md:14) · dated 3 (test-plan ci-integration.md:42 @c1471 · :65,
    a11y ci-integration.md:11).
  - F9 the `ci-scripts` leg 104 → 108 tests; `test_cold_agent.py` 33 → 37 cases — `ci-scripts-104`: 8 standing ·
    stale 6 (standard-contracts.md:261, test-plan.md:187 · :217, test-scope-summary.md:8, unit-test-strategy.md:29 ·
    :43) · dated 1 (unit-test-strategy.md:30, "27 → 33") · citation 1 (unit-test-strategy.md:44,
    `test_cold_agent.py:67-128`).
  - F10 `agent-run.sh run stand` 48 → 50 files, passed 197 → 205, ignored 7 — `run-stand`: 32 standing · stale 2
    (test-plan ci-integration.md:8 @c2220, quality-gates-coverage-targets.md:22 — "48 files") · dated 7
    (test-plan.md:170-176) · no-change 23 (the verb `run stand` named; the 22 `stand_*` fixture readers unchanged — the
    two new files declare no `mod fixtures;`).
  - F11 the workspace tally 206 lines · 1105 passed → 208 · 1117, 0 failed, 12 ignored — `tally`: 8 standing · dated 2
    (test-plan ci-integration.md:42 @c622 · @c1375, :63) · no-change 6 (line citations holding 1104-1105 or 1105-1109).
  - F12 coverage dioxus-native-dom 2740/3202 = 85.57 % → 2741/3202 = 85.60 %, blitz-test-harness 949/1008 = 94.15 % →
    952/1008 = 94.44 %, escher-driver 4918/5019 = 97.99 % unchanged — `coverage`: 1 standing, stale
    (quality-gates-coverage-targets.md:16).
  - F13 `host_log` 3 → 4 tests; the paged part read through the host's own sink is measured (residuals.md:38):
    `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, `snapshot --part 2` — 0 id and 0 name
    needles under both builds — `host-sink-part`: 51 standing · stale 5 (test-plan.md:267, test-scope-summary.md:15,
    logging-monitoring.md:14, pii-scrubbing-compliance.md:11, integration-test-strategy.md:21) · citation 6
    (`host_log.rs:N`, its body shifted by lines 134-278: occupied-resources.md:44, security-plan.md:301,
    test-scope-summary.md:19, integration-test-strategy.md:23, obs-plan.md:80, obs ci-integration.md:15) · dated 5
    (test-plan ci-integration.md:42 @c927 · @c957 · @c1138, :55, :63) · no-change 35 (`host_log` named as a check
    whose claim stands; existing-scopes.md:110 and log-coverage.md:114 are `stand_act_spans`' in-process part read).
  - F14 the cold-agent pipe: six apps, its verdict's seven new fields and three reasons, `wrong_calls` deciding by the
    founder's 5 % bar, `--record` on the live child, `tasks_met` decoding paged answers — `pipe`: 37 standing · stale
    11 (standard-contracts.md:261, input-validation.md:25, layout-templates.md:74, test-plan.md:182 · :184 · :187,
    obs-plan.md:169, log-coverage.md:14, occupied-resources.md:39 · :40, security-plan.md:63) · citation 13
    (`cold-agent.sh:N`, the file grown by 127 lines in 25 ranges: occupied-resources.md:24, stack-and-technologies.md:73,
    input-validation.md:34, test-plan.md:181 · :183 · :185 · :186, obs-plan.md:167 · :171 · :172, unit-test-
    strategy.md:43, security-plan.md:403 is prose — no-change; test-plan.md:180) · no-change 13 (the pipe named:
    architecture.md:193 · :218, standard-contracts.md:260, security-plan.md:57, input-validation.md:23 · :32,
    logging-monitoring.md:21, secret-management.md:14, test-plan.md:217, obs-plan.md:116, obs ci-integration.md:17,
    occupied-resources.md:17 · :65).
  - F15 the live run: the founder's one run of 2026-10-25 — 6 tasks, 66 calls, 0 wrong, $0.8244, Claude Code 2.1.308 —
    beside 0.1.0's 4 tasks, 43 calls, 0 wrong, $0.32; the carriage of the stand's new screens (Cells' contents, the
    select's options and value, a lint's findings, a shot) on «Запускать сейчас (Рекомендую)» (inputs#I6) within F3
    «Один запуск, до $3 (Рекомендую)» (inputs#I3) — `live-run`: 11 standing · stale 5 (occupied-resources.md:17 ·
    :40, threat-model-summary.md:35, test-plan.md:187, pii-scrubbing-compliance.md:32) · dated 1 (test-plan.md:170) ·
    no-change 5 (security-plan.md:290 · :306, logging-monitoring.md:21, secret-management.md:14 — the login and the
    masking, unchanged).
  - F16 every stand screen under `SNAPSHOT_TEXT_BUDGET` no longer holds for a served screen: Cells reads 47,949 bytes in
    5 parts — `stand-budget`: control unfired over the masters (refusal: `stand-budget · control UNFIRED over the seven
    masters (bodies and sets) at 55bf3425`); the claim stands in CLAUDE.md alone (a leaf).
  - F17 the stand is offline but for the load page's in-process stand-in (`boot_load` / `LoadNet`, a third hand-written
    network stand-in beside `HeldNet` and `stand_load_wait`'s `LateNet`) — `stand-offline`: 22 standing · stale 6
    (architecture.md:163, standard-contracts.md:120, test-plan.md:336 · :352, mocking-stubbing-discipline.md:14 ·
    :15 — the stand-in table, test-scope-summary.md:93) · no-change 16 (`HeldNet` / `LateNet` / an offline fixture or
    harness named).
  - F18 the stand no longer states `aria-pressed` anywhere: the flight booker's buttons are gone, the select states
    its choice by `selected` — `stand-pressed`: 22 standing · stale 6 (standard-contracts.md:211, layout-templates.md:10
    @c3941, test-scope-summary.md:94, a11y-plan.md:196, screen-reader-support.md:25 · :44) · no-change 16 (the engine's
    `aria-pressed` read, unchanged).
  - F19 renamed tests — `renamed-tests`: control unfired over the masters (refusal: `renamed-tests · control UNFIRED
    over the seven masters (bodies and sets) at 55bf3425`); no master site — the names stand in the 0.1.0 matrix's
    v010-17 ref and the 0.2.0 matrix's v020-05 ref (ledgers).
  - F20 `WITNESSES_020` gains three `Served` / flight rows and the `Served` subject — `witnesses`: 23 standing · stale 2
    (existing-scopes.md:114, test-scope-summary.md:119) · no-change 21 (the sweep named; its test counts unchanged at
    3).
  - F21 two new facts with no master site: the driver lint reads a select's options as overlapping (the snapshot
    gives each option its select's client rect); a click at a cell inside a scrolled box right after the scroll lands
    on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the
    thumb keeps it shown, so an `advance` past the fade is needed before the click — `new-hazards`: 24 standing,
    no-change 24 (the scrollbar fade and `ListBoxOption` facts they rest on stand true; standard-contracts.md:226 states
    the option's client rect).
  Owner pattern: control unfired (`title · control UNFIRED over the seven masters (bodies and sets) at 55bf3425`).
  `owner-generic`: 1 standing row (input-validation.md:197), another entry's owner — no-change; base 2 (playbook.md:58
  · :66) no-change. By-line pairs (5 lines over 2 000 chars): layout-templates.md:10 — `flight-mode` @c2481 · @c2500 ·
  @c3863 · @c3877 · @c3898 stale, @c3941 `stand-pressed` stale, `served-tasks` @c2262 citation · @c6026 no-change;
  layout-templates.md:75 — `served-tasks` @c189 stale · @c7705 · @c7752 citation; test-plan ci-integration.md:42 —
  `host-sink-part` @c927 · @c957 · @c1138 dated, `witnesses` @c882 · @c1094 dated, `tally` @c622 · @c1375 dated,
  `a11y-84` @c1471 dated; session-lifecycle.md:9 — `served-tasks` @c51 no-change · @c3817 citation;
  session-lifecycle.md:11 — `served-tasks` @c481 re-read at P2, `host-sink-part` @c2849 · @c8797 · @c9554 dated,
  `act-ids-census` @c5231 · @c10195 no-change, `witnesses` @c6057 · @c6315 no-change.
  Sweep reach: 20 patterns · 21 moved facts, 21 with a pattern (the report's own count; F16, F19 and the title by
  their unfired refusals) · owner pattern control unfired · by line 5 lines · 16 offsets.
- **Dev-tool versions:** `claude` — the cold-agent pipe's client (Claude Code) — read 2.1.308 on the dev host at
  2026-10-25 (operator entry 22, `claude --version`), the live run's `client_version` the same; the 2026-10-15 run's
  version not re-read here. No lockfile resolves it.
- **Harness / gate surface:** the cold-agent pipe's verdict shape and state set (above); its contract tests 33 → 37
  (`test_a_six_app_run_passes_with_no_wrong_call` renamed from the four-app case;
  `test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar`, `test_a_paged_answer_is_read_as_its_parts_joined`,
  `test_each_unused_capability_fails` (four sub-cases), `test_a_start_that_left_no_kept_check_fails` (two) added;
  `test_wrong_calls_are_counted_from_the_span_lines_and_agree_with_the_transcript` now fails `wrong-calls-over-bar`, 3
  wrong of 32); its fake server answers Cells as a paged JSON line of two parts and the load page, and under `--record`
  keeps one check per accepted start. `stand-repeat` takes the two new `stand_*` files by prefix. No CI step changes.
- **Cross-project / external claims:**
  - CI#40214713710 on `55bf34259031` (the pre-CI commit): `verdict: green · checks 17/17 · wall 990 s ·
    completed/success` — `evidence/operator-pass.md`.
  - The live run's model provider (the founder's one run, entry 23): `evidence/live-verdict.json` `outcome: passed`.
  - inputs (`inputs.py verify`, `inputs: 6 entries — unchanged 1 · … · n/a 5 · uncited 3 · unparsed 0`):
    - I1 · `../additional/escher-overseer/relays/cold-agent-bar-founder-2026-10-25.md` · copy no-repo · unchanged — the
      founder's word on the stand (cited by scope, research, plan).
    - I2 · message · n/a — the operator's directive for the phase (cited).
    - I3 · message · n/a — the founder's four P4 answers F1–F4 (cited).
    - I4 · message · n/a — the operator's «yes — apply.» on the P5 card `.andromeda/runs/2026-10-25T12-59-18-phase/
      p5-card-1.md` (inputs#I4 — cited here). The card's `## Beyond the plan` rows, verbatim, with the operator's own
      word:

      | # | proposal the word approves | owner |
      |---|---|---|
      | 1 | `.andromeda/residuals.md:38` (the host-sink paged part read, unmeasured) is discharged by step 9's measurement | «Cold-agent bar» (working-route.md:55), this chunk's wrap route-resolve |
      | 2 | `.andromeda/residuals.md:9` (the cold-agent wrong-call bar, `absorbed:v020-11`) is closed by the founder's F4, 5 % | «Cold-agent bar», this chunk's wrap route-resolve |
      | 3 | `docs/capabilities.md`:288-289 (the 2026-10-15 run «with eleven tools, 43…») is re-read and restated against this run's measured verdict | «Cold-agent bar», this chunk's wrap (the phase does not edit it) |
    - I5 · message · n/a — the founder's two /implement answers, relayed by the operator: «Перезакрепить как измерено
      (Рекомендую)» (re-pin `stand_lint` as measured) and «Без подсказки (Рекомендую)» (no prompt hint) (inputs#I5 —
      cited here).
    - I6 · message · n/a — the founder's word on the live run, relayed verbatim: «Запускать сейчас (Рекомендую)»
      (inputs#I6 — cited here).
- **Reverted / negative API facts:** a separate re-render step choosing the return flight in `stand_id_persistence`
  was written and folded into the Book step (a select choice changes nothing `dom_string` shows); an early click on
  Z25 inside the Cells walk was written and moved to its own test (the pointer it leaves on the thumb keeps the
  scrollbar shown and defeats the later `advance`).
- **Insufficient fixes (written, kept, not the remedy):** `stand_lint` re-pinned to the flight booker's measured
  `overlap` (inputs#I5) — the defect (the driver's lint reading a select's two options as overlapping, since both
  carry the select's client rect) is unresolved; its fix is a driver change for every document (`packages/escher-
  driver/src/lint.rs`, `overlaps`), owner to be pinned at this wrap's P5.
- **Spec claims disproved by measurement:**
  - The plan's carried sampler line «spacing#2.4: … the lean tasks' lint stays 0» (plan.md Implementation notes →
    Carried from the sampler) and the standing claims of a 0-finding lean stand (pattern `lint-at-boot`, F3 sites):
    measured false once the select stands — `stand_lint::the_stand_at_boot_reads_its_measured_findings`, both layout
    modes, `[0, 1, 0, 0]`.
  - «every stand screen under the 10,000-byte `SNAPSHOT_TEXT_BUDGET`» (CLAUDE.md, the snapshot row; pattern
    `stand-budget`, no master site): false for a served screen — Cells 47,949 bytes, 740 lines, 5 parts
    (`stand_cells_served::cells_is_walked_in_parts_through_the_driver`).
  - «the host boots only the four lean tasks and none pages» (pattern `served-tasks` / `host-sink-part`:
    test-plan.md:267, logging-monitoring.md:14, integration-test-strategy.md:21, pii-scrubbing-compliance.md:11):
    false since `serve cells` — `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`.
  - «the flight booker … state which control is pressed» (CLAUDE.md:38, and pattern `stand-pressed`'s six stale
    master rows): false — the stand states no `aria-pressed` (F18).
- **Expected amendments (from plan):**
  - architecture §Standard Contracts → Headless stand (Cells and the load page through `boot_cells` / `boot_load`, the
    load page's in-process stand-in after 200 ms, no `blitz_net`, every other boot offline) — carried: Symbols / APIs,
    F1, F17; sites `served-tasks` (architecture.md:163, existing-scopes.md:118) and `stand-offline`
    (standard-contracts.md:120).
  - architecture §Standard Contracts → CLIs (`escher-session` over six tasks, its usage line and MCP `start` enum) —
    carried: F1; sites architecture.md:187, standard-contracts.md:233, layout-templates.md:75 @c189.
  - security-plan §Threat Model Summary / §Surfaces (the 2026-10-25 live run's carriage of the new screens; a
    boundary widening on the founder's own F3, playbook «Boundary widening» `.andromeda/playbook.md:36` — the wrap halts
    on it once, as the answer card) — carried: F15; sites `live-run` (threat-model-summary.md:35, occupied-
    resources.md:17 · :40) and `pipe` (security-plan.md:63, input-validation.md:25).
  - test-plan §3 Cold-agent run pipe (six apps, goals, the verdict's new fields and reasons, `--record`, `wrong_calls`
    by the 5 % bar); §1 and §4 the counts; §5 the host part read — carried: F9, F10, F13, F14; sites `pipe`
    (test-plan.md:182 · :184 · :187), `ci-scripts-104`, `run-stand`, `host-sink-part` (test-plan.md:267).
  - obs-plan §6 the pipe's verdict fields; §8 and §3 the host-sink part read — carried: F13, F14; sites
    obs-plan.md:169, log-coverage.md:14, pii-scrubbing-compliance.md:11.
  - a11y-plan §5 · §6 · §7 (the select in the tables, its contrast row, its state reading in place of `aria-pressed`)
    — carried: F2, F5, F7, F8, F18; sites `flight-mode` (keyboard-navigation.md:38 · :98, screen-reader-support.md:44,
    visual-design-verification.md:12, a11y-plan.md:196), `contrast-rows` (a11y-plan.md:64), `stand-pressed`
    (screen-reader-support.md:25).
  - design-system §Color Palette (`flight-one-way` leaves the accent rows; the select's pair measured) — carried: F5;
    site color-palette.md:27.
  - layout-templates §Surface: desktop-native and §Surface: cli (the flight booker's mode control; the served task
    list) — carried: F1, F2; sites layout-templates.md:10, :75.
- **Coverage of new surfaces:**
  - `stand::boot_cells` / `escher-session start cells` → validation n/a (example code; the slug a closed table) ·
    instrumentation n/a (no new span; the host's `command` spans) · PII redacted✓ (`host_log` 0 needles, both builds) ·
    tests unit + integ (`cells_boots_in_the_shell`, `stand_cells_served` 4, `cli_commands`) · a11y n/a (Cells' own
    surface unchanged; `unkeyed_actionable` empty) · tokens n/a.
  - `stand::boot_load` + `LoadNet` / `start load` → validation n/a (fixed URL, fixed bytes) · instrumentation n/a ·
    PII n/a · tests unit + integ (`the_load_page_click_leaves_a_load_in_flight`, `stand_load_page` 2) · a11y ✓ for the
    page's two elements (keyed, lint 0; no keyboard or contrast row — the page is no `LeanTask`) · tokens design-token✓
    (the stand accent `#3a5ce5` / `#2a4cd3` / `#1f3fc1`, `#1a1a1a`, `#f5f5f5`).
  - `select#flight-type` → validation ✓ (closed match on the `input` value) · instrumentation n/a · PII n/a · tests
    integ (`stand_keyboard` 15, `stand_contrast`, `stand_snapshot`, `stand_snapshot_state`, `stand_act_states`,
    `stand_act_diff`, `stand_act_ids`, `stand_element_ids`) · a11y WCAG/focus/kbd✓ (Tab stop, arrows, 17.40, named
    "Flight type", focus cue `#3a5ce5` border) · tokens design-token✓.
  - the pipe's verdict fields and `--record` child → validation ✓ (kept checks parsed, never printed; the agent's check
    still bounded by `valid_step` · `MAX_CHECK_*`) · instrumentation ✓ (counts from `tool_call` span lines) · PII
    redacted✓ (events and verdict carry counts and closed words; planted-marker cases green) · tests unit
    (`test_cold_agent.py` 37) · a11y n/a · tokens n/a.

## Deviations from intent
- **Step 4's tables hold the select alone**, not its options: options take no focus, and the a11y acceptance needs
  every `controls(task)` id in `tab_order` ∪ `ENABLED_TAB_ORDER`; the options are pinned by
  `stand_snapshot::the_flight_type_select_holds_its_two_options` instead.
- **The lean tasks' lint is no longer 0** (the plan's carried sampler spacing#2.4): the flight booker reads one
  `overlap`; `stand_lint` re-pinned as measured on the founder's word (inputs#I5, «Перезакрепить как измерено
  (Рекомендую)»); the driver defect is surfaced (Insufficient fixes).
- **The pressed reading leaves the stand** — v010-17's flight witness moves to the select test (`selected`);
  `accessibility_states` still pins `aria-pressed` on a fixture.
- **`stand_cells_served`**: the walk to Z25 takes `advance 5000` after the scroll (the overlay scrollbar), and the
  early click is its own test; the prompt carries no hint (inputs#I5, «Без подсказки (Рекомендую)») — the live agent
  met Cells anyway.
- **`host_log`'s Cells read commits a sentinel first** (Cells' part-2 names at boot are empty cells and row numbers, no
  attributable needle).
- **The pipe's replay also decodes a paged answer** (`screen_text`), beside `tasks_met`, which the plan named alone.
- **`stand_id_persistence`** chooses and books in one re-render.
- **Test renames** leave stale names in two verified ledger refs (F19): 0.1.0 matrix v010-17; 0.2.0 matrix v020-05.
- Scope record (`gate.py scope`: `scope: clean — changed 26 · listed 24 · recorded 2 (companion 2 · mechanical 0 ·
  in-intent 0 · widening 0) · absorbed 0 · excluded 527`):
  - companion: `tests/blitz-tests/tests/stand_lint.rs` · serves step 5 · self (pins 0 lint findings per lean task)
  - companion: `tests/blitz-tests/tests/stand_accessibility_ids.rs` · serves step 5 · self (pins the stand's 15
    controls)

## Decisions & corrections
- The founder's word, relayed by the operator: «Перезакрепить как измерено (Рекомендую)» — `stand_lint` re-pinned to
  the measured `overlap`, the driver fix not taken in this chunk (inputs#I5).
- The founder's word, relayed: «Без подсказки (Рекомендую)» — the cold-agent prompt carries no hint about the shown
  scrollbar (inputs#I5).
- The founder's word, relayed verbatim: «Запускать сейчас (Рекомендую)» — the one live run fired once through
  `gate.py run --operator 23`, never re-fired (inputs#I6); result passed, 0 wrong of 66, $0.8244.
- The operator's «yes — apply.» on P5 card 1 (inputs#I4): its three Beyond-the-plan rows are this wrap's to land
  (residuals.md:38 discharged, residuals.md:9 closed, docs/capabilities.md:288-289 restated).
- The first ask for the live run's word (AskUserQuestion) returned «No answer from the driver within 58 min — do not
  proceed with this; end your turn and wait for instructions. (Held by viola …)»; the run stopped with an interim
  outcome and nothing fired until the relayed word.
- Sweep hazard: research's sweep `git grep -P 'flight-(one-way|return)(?!-date)'` found the 13 files naming the old
  ids, but three pins hold the changed control set by COUNT or by a table no id names — `stand_accessibility_ids`
  (`checked.len() == 15`), `stand_lint` (0 findings per lean task) and `stand_requirement_sweep`'s `source`
  `include_str!` table (a new witness file must be carried there) — each went red at the first gate firing (7 reds,
  first block firing in `.andromeda/runs/2026-10-25T14-36-08-implement/`).
- Measured: a select choice moves no class or text `Harness::dom_string` shows (`stand_id_persistence` read the DOM
  unchanged).
- Measured: right after `scroll`, the engine's overlay scrollbar takes a main-button pointerdown at a cell in its band
  as a thumb drag (`events/pointer.rs`, `hit_with_scrollbar` with `scrollbar_opacity > 0`); on the virtual clock it
  stays shown until `advance` passes its fade; a pointer resting on the thumb keeps it shown.
- Measured: a paged snapshot answers over MCP as a JSON line whose quotes are escaped, so a substring match over the
  raw answer misses the screen's own text (`test_a_paged_answer_is_read_as_its_parts_joined` asserts the escaped form
  in the transcript).
- Measured: a full recorded Cells session keeps a check of 312,431 bytes, the load page's 1,660, under the 1,048,576
  bound (probe through the built binary, `target/tmp/recprobe-1`).
- Red-before-green for the contract tests: the new `test_cold_agent.py` against HEAD's pipe in a scratch copy fails
  34 of 37.
- Guard refusals this chunk: a `cd` that would move the session cwd (re-issued with absolute paths); a `cat` heredoc
  to a file (re-issued through Edit); a recursive `rm` (refused by the host's permissions; `target/tmp/recprobe-1` and
  the scratchpad's `rbg-1` left on disk, fresh directories used).
- A stray `cat > /dev/null` in a probe command waited on stdin past the Bash limit and was stopped with TaskStop.
- Evolve: implement's checkpoints appended 8 records (code · fix-loop · smoke; frictions `input.research-files-wrong`,
  `input.plan-step-ambiguous`, `contract.test-expectation`, two untyped).

## Outcome
- Acceptance criteria, against the diff:
  - (tests) v020-11 — the live run: MET. `evidence/live-verdict.json` `outcome: passed`, reasons none; `isolated`
    (twelve `mcp__escher__*`, one server `escher`); `counts_agree` (66 · 66); `tasks_met` all six (counter ·
    flight-booker · timer · crud · cells · load), each from the tool's own snapshot answers; `lint_calls` 1 ·
    `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2; `recorded_sessions` 6, `recording_kept`;
    `check_passed` and `check_can_fail` (59 steps, 16 expects); `wrong_calls` 0 of 66 against 5 %. Calls by verb:
    advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 · type 10.
  - (tests) contract tests green, every step-11 path with its case: MET — Ran 37, OK; `ci-scripts` Ran 108.
  - (tests) `stand_cells_served` and `stand_load_page` green in both layout modes: MET (4 · 2).
  - (a11y) the a11y leg and `stand-repeat` green; `stand_keyboard` walks the restated tables, enabled state by
    keyboard alone; `stand_contrast` reads the select at 17.40; the select reads `ComboBox` "Flight type", options
    named and selected; `unkeyed_actionable` empty on the six served tasks: MET (a11y 14 files · 85 · 0;
    stand-repeat 3 of 3; Cells and the load page by their own checks).
  - (tests) `stand-repeat` ends `stand-repeat: 3 of 3 rounds green`: MET.
  - (arch) six slugs served, an unknown task a usage ending, every other boot `net_provider: None`, only `boot_load`
    holds the stand-in naming no `blitz_net`, the `git diff` probe 0, no platform adapter: MET (entries 4, 1, 10, 11,
    12).
  - (obs) silence census 0; `host_log`'s Cells part read 0 id and name needles under both builds; the pipe's events
    and verdict carry no scrub key or planted marker; `server.log`, `check.json`, the transcript and the kept checks
    never printed or copied: MET (entries 10, 4, 5, 7). The evidence copies `live-check.json` and a masked
    `live-transcript.jsonl` as step 12 directs.
  - (security) pipe census 0; the prompt names no stand id; usage exits 2 first; hygiene clean; no kept check
    committed: MET (entries 9, 7; operator entry 25 `hygiene: clean`; `target/cold-agent/record/` uncommitted).
  - (tests) `fast`, `doc`, `coverage` green and the pushed sha's CI `verdict: green`: MET.
  - Not an acceptance criterion but a carried plan line: the lean tasks' lint 0 — UNMET as measured (F3), routed by
    the founder's word to a re-pin; the driver defect awaits its owner at P5.
- Gates (implement run `.andromeda/runs/2026-10-25T14-36-08-implement/`, the block's second firing):
  - `cargo test -p seven_guis --locked --lib stand:: 2>&1` — green (5 passed)
  - `cargo test -p blitz-tests --locked --test stand_cells_served --test stand_load_page 2>&1` — green
  - `cargo test -p blitz-tests --locked --test stand_keyboard … --test stand_non_lean_tasks 2>&1` — green
  - `cargo test -p seven_guis --locked --test cli_commands … --test recording 2>&1` — green
  - `cargo test --workspace --locked --test host_log --test mcp_log 2>&1` — green
  - `cargo check --tests --locked --target x86_64-pc-windows-gnu -p seven_guis 2>&1` — green
  - `python3 -m unittest discover -s .github/scripts -p test_cold_agent.py 2>&1` — green (Ran 37, OK)
  - `bash -n scripts/cold-agent.sh` — green
  - `grep -c -E 'socket|…|ANTHROPIC_' scripts/cold-agent.sh` — green (exit 1, last line 0)
  - `cat examples/seven_guis/src/stand.rs … | grep -c -E 'println!|…|blitz_net' ` — green (exit 1, last line 0)
  - `git diff --name-only e84dc818… -- packages Cargo.toml … | wc -l` — green (0)
  - `cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` — green
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py guard …` — green, `guard: clean` (allowed 24,
    companions excluded 2 — `stand_accessibility_ids.rs` and `stand_lint.rs`, each serving step 5)
  - `bash scripts/agent-run.sh boot` — green
  - `bash scripts/agent-run.sh run stand` — green (50 files, passed 205, failed 0, ignored 7)
  - `bash .github/scripts/ci-leg.sh a11y` — green (14 files, 85 passed)
  - `bash .github/scripts/ci-leg.sh stand-repeat` — green (`stand-repeat: 3 of 3 rounds green`)
  - `bash .github/scripts/ci-leg.sh coverage` — green (3 of 3 crates at or above their floors)
  - `bash .github/scripts/ci-leg.sh fast` — green; its tally probe `grep -E '^test result:' … | awk …` — green
    (`lines 208 passed 1117 failed 0 ignored 12`)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - `claude --version` — leg operator, fired once by /implement on the founder's word: green (2.1.308) —
    `evidence/live-run.md`
  - `cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh run stand` — leg operator,
    fired once: green, every atom held (`passed`, `isolated`, `counts_agree`, `check_passed`, `check_can_fail`,
    `recording_kept`) — `evidence/live-run.md`
  - `grep -c '"outcome": "passed"' …/evidence/live-verdict.json` — green after step 12 (last line 1)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator: `hygiene: clean` —
    `evidence/operator-pass.md`
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.2.0` — leg operator: green, `history moved: refs/remotes/origin/build/escher-0.2.0 e84dc818→55bf3425`
    — `evidence/operator-pass.md`
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg operator:
    `verdict: green · checks 17/17` (CI#40214713710) — `evidence/operator-pass.md`
  - Smoke: the binary's boot path changed (`session_host.rs`); it ran as P2 gates (`cli_commands` starts the six slugs
    through the binary; the live run drove the `mcp` role over all six).
- Watches: none folded.
- Outcome basis: the operator pass ran — the pre-CI commit `55bf3425` (`git log --format='%h %s' e84dc818..HEAD`: that
  one commit) and the final HEAD's CI run CI#40214713710, green 17/17, recorded in `evidence/operator-pass.md`; this
  window holds the implement run whole; its persisted report `.andromeda/runs/2026-10-25T14-36-08-implement/
  outcome-2026-10-25-cold-agent-bar.md` (the one `*-implement` dir of this marker) agrees; post-implement artifacts:
  `evidence/live-*` (step 12), `evidence/operator-pass.md`.
- Process hygiene: implement's census — no `escher-session`, `seven_guis_native` or live `claude -p` left; the 11
  `claude` sessions are the operator's own. Re-measured here: `gate.py census --comm escher-session --comm
  seven_guis_native --args 'escher-session mcp' --args 'escher-session serve' --args cold-agent` → `census: none
  running`.
## New text, by line
Generated by `cites.py added` (cites v1.5); pasted by `splice.py`. No line of this section is typed or edited.
The diff: e84dc818 (the parent of the oldest pre-CI commit 55bf3425) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_cold_agent.py — added 123 line(s) in 34 range(s)
added: 25-26 · 29-36 · 51-55 · 57-58 · 64-70 · 77-78 · 83 · 86-89 · 101 · 103 · 117-120 · 132-139 · 240-241 · 244 · 246
       249 · 254-260 · 318 · 336-339 · 355 · 364-367 · 369-371 · 386-387 · 389 · 394 · 401-406 · 426 · 433-462 · 467
       490 · 502-504 · 597-599 · 601 · 609-611
- 31-34 «def cells(b1, z25):»
  - 52-55 «"load": [»
- 254-258 «def with_wrong(wrong, total):»
  - 336-338 «def live_spawn(self):»
  - 401-405 «def test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar(self):»
  - 433-441 «def test_a_paged_answer_is_read_as_its_parts_joined(self):»
  - 443-455 «def test_each_unused_capability_fails(self):»
  - 457-461 «def test_a_start_that_left_no_kept_check_fails(self):»
### examples/seven_guis/src/session_host.rs — added 25 line(s) in 4 range(s)
added: 1-5 · 12-27 · 43 · 48-50
  - 12-18 @14 «enum Served {»
### examples/seven_guis/src/stand.rs — added 174 line(s) in 5 range(s)
added: 1-9 · 12 · 17 · 131-245 · 283-330
- 131-135 @133 «pub fn boot_cells(options: HarnessOptions) -> Harness<DioxusDocument> {»
- 137-139 «fn cells_root() -> Element {»
- 141-152 @144 «pub fn boot_load(options: HarnessOptions) -> Harness<DioxusDocument> {»
  - 145-151 «Harness::from_vdom(»
- 167-175 «impl NetProvider for LoadNet {»
  - 168-174 «fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {»
- 177-183 «fn load_root() -> Element {»
  - 178-182 «rsx! {»
- 185-200 @186 «fn LoadPage() -> Element {»
  - 188-199 «rsx! {»
- 203-211 «.load-root {»
- 213-218 «.load-body {»
- 220-229 «.load-btn {»
- 231-233 «.load-btn:hover {»
- 235-237 «.load-btn:active {»
- 239-243 «.load-done {»
  - 283-291 @284 «fn the_stand_options_stay_offline() {»
  - 293-306 @294 «fn cells_boots_in_the_shell() {»
  - 308-329 @309 «fn the_load_page_click_leaves_a_load_in_flight() {»
### examples/seven_guis/src/tasks/flight_booker.rs — added 27 line(s) in 5 range(s)
added: 66-80 · 83-86 · 160-162 · 165-166 · 169-171
### examples/seven_guis/tests/cli_commands.rs — added 16 line(s) in 5 range(s)
added: 3-5 · 37-41 · 137 · 266 · 592-597
### examples/seven_guis/tests/common/mod.rs — added 33 line(s) in 2 range(s)
added: 6-7 · 425-455
- 425-454 @430 «pub fn cells_needles(part: usize) -> (Vec<String>, String) {»
  - 434-437 «let parts = harness»
  - 439-444 «let ids: Vec<String> = parts[part - 1]»
  - 447-451 «assert!(»
### examples/seven_guis/tests/host_log.rs — added 148 line(s) in 2 range(s)
added: 15-17 · 134-278
- 134-277 @136 «fn a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace() {»
  - 154-164 «let mut host = Host(»
  - 169-180 «let hello = loop {»
  - 186-209 «let calls = [»
  - 210-218 «let answers: Vec<String> = calls»
  - 221-224 «assert!(»
  - 225-229 «assert!(»
  - 233-239 «let status = loop {»
  - 240-243 «let stdout = stdout»
  - 244-247 «let stderr = stderr»
  - 252-256 «assert!(»
  - 259-262 «let command_spans = stderr»
  - 263-268 «assert_eq!(»
  - 270-276 «assert!(»
### scripts/cold-agent.sh — added 127 line(s) in 25 range(s)
added: 3-7 · 10-12 · 20 · 24 · 27-28 · 30 · 51 · 63-64 · 67-70 · 75-76 · 170-193 · 202-203 · 215-216 · 219-271 · 379
       391 · 413 · 431-432 · 444-445 · 449-454 · 483-488 · 503-504 · 506 · 571 · 577
  - 171-174 «for line in lines:»
- 178-187 «def paged(text):»
  - 179-181 @180 «try:»
  - 182-183 «except ValueError:»
  - 184-186 «if (isinstance(line, dict) and set(line) == {"text", "part", "parts"} and isinstance(line["text"], str)»
- 190-193 «def screen_text(text):»
- 231-241 «def part_reads(transcript, uses):»
  - 234-240 «for record in transcript:»
- 244-254 «def started_in_order(transcript, uses):»
  - 247-253 «for record in transcript:»
- 257-271 «def recording(starts):»
  - 260-261 «try:»
  - 262-263 «except OSError:»
  - 266-270 «for _, name in numbered:»
  - 483-484 «if 0 in (v["lint_calls"], v["screenshot_calls"], v["advance_calls"], v["part_reads"]):»
  - 485-486 «if not recording_kept:»
  - 487-488 «if wrong_calls * 100 > WRONG_CALL_BAR_PCT * v["tool_calls"]:»
### tests/blitz-tests/tests/common/mod.rs — added 5 line(s) in 5 range(s)
added: 25 · 52 · 66 · 72 · 143
### tests/blitz-tests/tests/session_common/mod.rs — added 9 line(s) in 4 range(s)
added: 305-306 · 309-313 · 316 · 318
### tests/blitz-tests/tests/stand_accessibility_ids.rs — added 1 line(s) in 1 range(s)
added: 181
### tests/blitz-tests/tests/stand_act_diff.rs — added 26 line(s) in 7 range(s)
added: 5-7 · 128 · 131-136 · 138 · 146-152 · 155 · 163-169
### tests/blitz-tests/tests/stand_act_disabled.rs — added 2 line(s) in 1 range(s)
added: 71-72
### tests/blitz-tests/tests/stand_act_ids.rs — added 18 line(s) in 8 range(s)
added: 4-8 · 42 · 60-61 · 88 · 166-167 · 170-172 · 236-237 · 240-241
### tests/blitz-tests/tests/stand_act_states.rs — added 25 line(s) in 6 range(s)
added: 3-6 · 80 · 84-86 · 88 · 90-91 · 95-108
### tests/blitz-tests/tests/stand_cells_served.rs — new file · 209 line(s)
- 19-21 «use session_common::{»
- 33-37 @34 «fn hold_cells(incremental: bool) -> Session {»
  - 35-36 «Session::start("cells", || stand::boot_cells(stand::options(incremental)))»
- 39-54 @41 «fn parts(session: &mut Session, mode: &str) -> Vec<String> {»
  - 44-52 «for n in 2..=count {»
- 56-60 @57 «fn reads(text: &str, id: &str, name: &str) -> bool {»
  - 58-59 «text.lines()»
- 62-71 @64 «fn step(session: &mut Session, call: &Call, mode: &str, step: usize) -> session_common::Acted {»
  - 66-69 «assert!(»
- 73-89 @76 «fn commit(session: &mut Session, id: &str, text: &str, mode: &str, first: usize) {»
  - 78-81 «assert!(»
  - 84-88 «assert!(»
- 91-133 @92 «fn cells_is_walked_in_parts_through_the_driver() {»
  - 93-132 «for incremental in [false, true] {»
- 135-174 @136 «fn committed_cells_read_their_values_and_a_cell_past_the_viewport_reads_in_the_last_part() {»
  - 137-173 «for incremental in [false, true] {»
- 176-192 @177 «fn a_click_right_after_the_scroll_lands_on_the_shown_scrollbar() {»
  - 178-191 «for incremental in [false, true] {»
- 194-209 @195 «fn the_lint_at_boot_reads_the_measured_findings() {»
  - 196-208 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_contrast.rs — added 20 line(s) in 11 range(s)
added: 12 · 68-69 · 279 · 295 · 299-300 · 413-414 · 417-423 · 479 · 595 · 634 · 651
### tests/blitz-tests/tests/stand_element_ids.rs — added 3 line(s) in 1 range(s)
added: 35-37
### tests/blitz-tests/tests/stand_id_persistence.rs — added 7 line(s) in 2 range(s)
added: 8 · 93-98
### tests/blitz-tests/tests/stand_keyboard.rs — added 76 line(s) in 6 range(s)
added: 7-15 · 45 · 208 · 214 · 220-221 · 249-310
- 249-309 @250 «fn the_flight_type_select_moves_its_choice_by_its_arrows() {»
  - 251-308 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_lint.rs — added 34 line(s) in 7 range(s)
added: 7-9 · 277-286 · 288-289 · 296 · 300-306 · 308-311 · 428-434
- 277-285 @280 «fn at_boot(task: LeanTask) -> &'static [(&'static str, Defect, &'static str)] {»
  - 281-284 «match task {»
### tests/blitz-tests/tests/stand_load_page.rs — new file · 84 line(s)
- 19-23 @20 «fn hold_load(incremental: bool) -> Session {»
  - 21-22 «Session::start("load", || stand::boot_load(stand::options(incremental)))»
- 25-32 @27 «fn screen(session: &mut Session, mode: &str) -> String {»
  - 28-31 «match session.run(&snapshot()) {»
- 34-68 @35 «fn the_load_page_click_waits_out_its_load_and_shows_the_message() {»
  - 36-67 «for incremental in [false, true] {»
- 70-84 @71 «fn the_load_page_is_keyed_and_reads_no_finding_at_boot() {»
  - 72-83 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_requirement_sweep.rs — added 26 line(s) in 11 range(s)
added: 30 · 81-82 · 86 · 229-230 · 236 · 531 · 688 · 765-769 · 805-809 · 830-834 · 945-946
### tests/blitz-tests/tests/stand_session_recording.rs — added 1 line(s) in 1 range(s)
added: 53
### tests/blitz-tests/tests/stand_snapshot.rs — added 55 line(s) in 2 range(s)
added: 4-5 · 118-170
  - 123-170 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 49 line(s) in 11 range(s)
added: 6-8 · 121-131 · 168 · 182 · 368 · 372-373 · 375-392 · 394 · 396-404 · 476 · 489
- 121-130 @123 «fn choose_flight_type(flight: &mut Harness<DioxusDocument>, returning: bool) {»
  - 125-129 «flight.press(Key::Named(if returning {»
