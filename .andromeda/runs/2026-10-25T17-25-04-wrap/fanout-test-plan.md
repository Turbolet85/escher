proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (Verdict)"
    change: "Verdict: add the fields `wrong_call_bar_pct` (5) · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`; `outcome` is `passed` only with all six apps (counter · flight-booker · timer · crud · cells · load) in `tasks_met`, each of `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` above 0, `recording_kept`, and `wrong_calls × 100 ≤ 5 × tool_calls`; the reasons gain `capability-unused · recording-not-kept · wrong-calls-over-bar`, after `task-unmet` and before the check's; `tasks_met` reads accepted snapshot answers only, resets an app on its accepted `start` and joins the latest answer of each part of a paged screen; replace «`wrong_calls` … recorded, never deciding the outcome» with «`wrong_calls` decides the outcome by the founder's 5 % bar (F4)»"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent verdict — seven new fields, three new reasons, six apps in tasks_met, wrong_calls now decides by the 5 % bar"
    rationale: "Report Symbols / APIs (the pipe) and Schema / config: verdict fields `wrong_call_bar_pct` (5) · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`, reasons `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`) after `task-unmet` and before the check's, `wrong_calls` now decides; `tasks_met` reads accepted snapshot answers only, resets on accepted `start`, joins paged parts; F14 lists test-plan.md:184 stale (`pipe`)."
    basis: "scripts/cold-agent.sh:483-488"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (`run stand`)"
    change: "`run stand`: the server's `mcp.json` args run `mcp --record \"$3\"` with the absolute `target/cold-agent/record` (made `mkdir -m 700`) as a third argument; `STAND_PROMPT` asks for six apps one at a time — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells (`A0` 5, `B1` `=A0*2` reading 10, `Z25` `=B1+1` reading 11, plus one visual lint) and the load page (its resource loaded and its message shown) — each started, brought to its goal, snapshotted and stopped before the next, still naming no stand id"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent run stand — six apps and their goals in the prompt, the live server child recording under --record into target/cold-agent/record"
    rationale: "Report Symbols / APIs (the pipe): `STAND_PROMPT` names six apps (counter, flight booker, timer, CRUD, cells, the load page) and their goals, still naming no stand id; the live server child runs `mcp --record \"$3\"` into `target/cold-agent/record` (made `mkdir -m 700`); F14 lists test-plan.md:182 stale (`pipe`)."
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (Invocation)"
    change: "Invocation: the one task `stand` — the six apps (counter, flight booker, timer, CRUD, cells and the load page) in one client session, in place of «the four lean apps»"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent invocation — the stand task holds six apps"
    rationale: "Report Symbols / APIs (the pipe): `STAND_PROMPT` names six apps; F14 (`pipe`) — test-plan.md:180 restates the four-app claim the primary retires."
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (The server and the check's judge)"
    change: "`span_counts` also counts accepted ` span=\"tool_call\" ` lines per counted verb (`lint` · `screenshot` · `advance`); the replay children run without `--record`, and the replay's expect match also reads a paged answer's decoded text (`screen_text`, `paged`)"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent judge — span_counts counts accepted lint/screenshot/advance calls; replays unrecorded; paged answers decoded for the expect match"
    rationale: "Report Symbols / APIs (the pipe): new functions `paged` · `screen_text` (178-193); `span_counts` also returns accepted `tool_call` lines per `COUNTED` verb (`lint` · `screenshot` · `advance`); replays run without `--record`; the replay's expect match also reads a paged answer's decoded text; Deviations: «The pipe's replay also decodes a paged answer (`screen_text`)»."
    basis: "scripts/cold-agent.sh:178-193"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (State)"
    change: "State: `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}` and the owner-only `target/cold-agent/record/` (made `mkdir -m 700`, the live server child's kept checks, never printed, copied or committed); `cleanup` removes it with the rest of that directory"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent state — target/cold-agent/record/ joins the state set, owner-only, removed by cleanup"
    rationale: "Report Schema / config: the pipe's state set gains `target/cold-agent/record/`, owner-only, `cleanup` removing it with the state; Outcome (obs, security): kept checks never printed or copied, `target/cold-agent/record/` uncommitted. test-plan.md:186 states the set with «only», which this retires."
    basis: "scripts/cold-agent.sh:257-271"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Cold-agent run pipe (Proof)"
    change: "Proof: the contract tests `test_cold_agent.py` (§4, 37) and, at 2026-10-25-cold-agent-bar, the founder's one live run of 2026-10-25 on «Запускать сейчас (Рекомендую)» within «Один запуск, до $3 (Рекомендую)», fired once: `passed`, reasons none, `isolated` (the twelve `mcp__escher__*` tools, one server `escher`), `counts_agree` (66 · 66), `tasks_met` all six, `lint_calls` 1 · `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2, `recorded_sessions` 6 with `recording_kept`, `check_passed` and `check_can_fail` (59 steps, 16 expects), 66 tool calls (advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 · type 10), 0 wrong of 66 against 5 %, 0.8244 USD, `claude` 2.1.308 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json and evidence/live-run.md; the 2026-10-15 four-app run (eleven tools, 43 calls, 0 wrong, 0.3197 USD, `claude` 2.1.296) kept as dated, and «the live run is not re-fired» dropped"
    sidecar: "2026-10-25-cold-agent-bar: §3 cold-agent proof — contract tests 37; the 2026-10-25 six-app live run (66 calls, 0 wrong, $0.8244, claude 2.1.308) recorded beside the dated 2026-10-15 run"
    rationale: "Report F9 (`test_cold_agent.py` 33 → 37), F15 (the founder's one run of 2026-10-25 — 6 tasks, 66 calls, 0 wrong, $0.8244, Claude Code 2.1.308), Outcome v020-11 (the verdict's atoms and calls by verb), Decisions (inputs#I6 «Запускать сейчас (Рекомендую)»); F9, F14 and F15 list test-plan.md:187 stale."
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (Cold-agent pipe)"
    change: "Cold-agent pipe: test_cold_agent.py — 37 cases — one case per verdict path, among them the six-app pass with no wrong call, the 5 % bar (five wrong of a hundred pass, six fail `wrong-calls-over-bar`), a paged answer read as its parts joined, each unused capability failing `capability-unused` (four sub-cases) and a `start` that left no kept check failing `recording-not-kept` (two); its fake server answers Cells as a paged JSON line of two parts and the load page and, under `--record`, keeps one check per accepted start"
    sidecar: "2026-10-25-cold-agent-bar: §4 cold-agent contract tests — 37 cases, the bar, paged answer, unused-capability and kept-check cases"
    rationale: "Report Harness / gate surface: contract tests 33 → 37 (`test_a_six_app_run_passes_with_no_wrong_call` renamed, `test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar`, `test_a_paged_answer_is_read_as_its_parts_joined`, `test_each_unused_capability_fails` (four), `test_a_start_that_left_no_kept_check_fails` (two) added); the fake answers Cells paged and the load page, keeping one check per accepted start under `--record`; F9 lists test-plan.md:217 stale (`ci-scripts-104`)."
    basis: ".github/scripts/test_cold_agent.py:401-461"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session host ↔ what it writes"
    change: "Replace «a paged `part` read through the host's own sink is unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)» with: a paged `part` read through the host's own sink is a standing check since 2026-10-25-cold-agent-bar — `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` serves `cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and reads `snapshot --part 2`, 0 id and 0 name needles under both builds (the residual discharged); the host serves six tasks"
    sidecar: "2026-10-25-cold-agent-bar: §5 host sink — the paged part read through the host is measured by host_log's Cells test; the four-lean-tasks claim retired"
    rationale: "Report F13 (`host_log` 3 → 4 tests; `serve cells` at `RUST_LOG=trace`, sentinel in a part-2 cell, `snapshot --part 2`, 0 id and 0 name needles under both builds; residuals.md:38 discharged), F1 (`escher-session` serves six tasks), Spec claims disproved («the host boots only the four lean tasks and none pages», test-plan.md:267); I4 row 1."
    basis: "examples/seven_guis/tests/host_log.rs:134-277"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Session ↔ held instance"
    change: "Add: beyond the lean four, the stand's served pages are driven through a held `Session` in both layout modes — Cells walked in parts through the driver (`stand_cells_served`, a cell past the viewport read in the last part, an `advance` past the overlay scrollbar's fade before a click after a scroll) and the load page's click waiting out its stand-in's load before its message shows (`stand_load_page`)"
    sidecar: "2026-10-25-cold-agent-bar: §5 session checks — Cells walked in parts and the load page's load wait driven through a held session"
    rationale: "Report Changes (new `stand_cells_served.rs`, `stand_load_page.rs`), Outcome (`stand_cells_served` and `stand_load_page` green in both layout modes, 4 · 2), Deviations (`stand_cells_served`'s `advance 5000` after the scroll), Coverage of new surfaces (tests unit + integ for `boot_cells` and `boot_load`)."
    basis: "tests/blitz-tests/tests/stand_cells_served.rs:91-133"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Wall time in the session checks"
    change: "Add a third wall-time exception: `stand_load_page`, whose subject is a load answered after `stand::LOAD_ANSWER_DELAY` (200 ms) by the stand's `LoadNet` from a thread of its own, waited out by the acting verb's load wait"
    sidecar: "2026-10-25-cold-agent-bar: §5 wall time — stand_load_page's 200 ms LoadNet answer joins the wall-time exceptions"
    rationale: "Report Symbols / APIs: `LoadNet`'s each `fetch` answers `LOADED_CSS` from a thread of its own after `LOAD_ANSWER_DELAY` = 200 ms; v020-07 witness `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`."
    basis: "examples/seven_guis/src/stand.rs:167-175"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Runners and invocation (the seven_guis headless stand)"
    change: "Add: beside the lean tasks the stand boots Cells through `stand::boot_cells(HarnessOptions)` (Task::Cells in TaskShell, no `LeanTask`) and the load page through `stand::boot_load(HarnessOptions)` (TaskShell titled \"Load\", its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept), both read by the served-page checks and by `escher-session`'s six tasks"
    sidecar: "2026-10-25-cold-agent-bar: §3 stand boots — boot_cells and boot_load beside the lean tasks"
    rationale: "Report Symbols / APIs: `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` (stand.rs:131-135), Cells no `LeanTask`; `boot_load(HarnessOptions)` (141-152) with `net_provider` `Some(Arc::new(LoadNet))` through `..options`; callers `session_host.rs`, `stand_cells_served`, seven_guis `common::cells_needles`; F1 (`served-tasks`)."
    basis: "examples/seven_guis/src/stand.rs:131-152"
  - detector: D-retired
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests stand checks)"
    change: "The tables: `controls(task)` 17 rows naming 14 controls (the flight booker's `flight-type`, `Role::ComboBox`, in place of the two mode buttons); `tab_order` and `ENABLED_TAB_ORDER` hold `flight-type` — the flight booker's Tab `back-btn` → `flight-type` → `flight-start` → `flight-book`, and its enabled state after the return flight is chosen on `flight-type`, five stops with `flight-return-date`, in place of «after `flight-return`, six stops»"
    sidecar: "2026-10-25-cold-agent-bar: §3 stand tables — controls 18/15 → 17/14, the flight booker's tab orders through the select flight-type; the retired flight-return button dropped"
    rationale: "Report Retired (the flight booker's two `aria-pressed` mode buttons `flight-one-way`, `flight-return`); Symbols / APIs (`common` tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order`, `ENABLED_TAB_ORDER`, `rendered`); F2 (Tab and enabled five stops; test-plan.md:146 stale), F4 (controls 15 → 14, `controls(task)` rows 18 → 17; test-plan.md:146 stale)."
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests session_common)"
    change: "`change(task, session, ticks)`: one command that changes what a task shows — a click, the flight booker's `type flight-type return` through the driver, or three delivered ticks for the timer"
    sidecar: "2026-10-25-cold-agent-bar: §3 session_common change — the flight booker moves by type flight-type return"
    rationale: "Report Symbols / APIs: «blitz-tests `session_common::change` moves the flight booker by `type flight-type return` through the driver»; the clicked mode button is retired (Retired)."
    basis: "tests/blitz-tests/tests/session_common/mod.rs:305-318"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests session_common readers)"
    change: "The module's readers: 31 with `stand_cells_served` and `stand_load_page` (since 2026-10-25-cold-agent-bar; the Cells walk reads the module's call builders, the load page its `snapshot` builder), 29 with `stand_snapshot_parts`"
    sidecar: "2026-10-25-cold-agent-bar: §3 session_common readers 29 → 31 with the two served-page checks"
    rationale: "Report New text: `stand_cells_served.rs` 19-21 «use session_common::{»; `stand_load_page.rs` 28-31 «match session.run(&snapshot()) {» (the `snapshot()` builder the body places in session_common since 2026-10-10-driver-cli)."
    basis: "tests/blitz-tests/tests/stand_cells_served.rs:19-21"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract (Proof)"
    change: "Append to the `run stand` re-counts: re-counted at 2026-10-25-cold-agent-bar, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 205 · failed 0 · ignored 7 over 50 files — `stand_cells_served` 4 and `stand_load_page` 2, two new files picked up by their `stand_` prefix with no script change, beside new tests in `stand_keyboard` and `stand_snapshot`; `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "2026-10-25-cold-agent-bar: §3 agent-run proof — run stand 205 · 0 · 7 over 50 files"
    rationale: "Report F10 (`agent-run.sh run stand` 48 → 50 files, passed 197 → 205, ignored 7; test-plan.md:170-176 dated, so the running history takes a new re-count), Outcome gate `bash scripts/agent-run.sh run stand` — green (50 files, passed 205, failed 0, ignored 7)."
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures → Builders and options"
    change: "`seven_guis::stand::options(incremental)` keeps `net_provider: None` for every caller but `stand::boot_load`, which replaces it with `Some(Arc::new(LoadNet))` through `..options`, every other field kept (pinned by the unit test `the_stand_options_stay_offline`)"
    sidecar: "2026-10-25-cold-agent-bar: §7 stand options — offline for every caller but boot_load's LoadNet"
    rationale: "Report Symbols / APIs: `boot_load`'s options' `net_provider` replaced by `Some(Arc::new(LoadNet))` through `..options`; `stand::options` keeps `net_provider: None` for every other caller (unit test `the_stand_options_stay_offline`); F17 lists test-plan.md:336 stale (`stand-offline`)."
    basis: "examples/seven_guis/src/stand.rs:283-291"
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs (Network)"
    change: "Network: `HeldNet` is escher's one held network stand-in; beside it the stand's `LoadNet` (`seven_guis::stand`, read only by `boot_load`) answers each fetch with a fixed sheet from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms) — no `blitz_net`, print, env read, socket or listener — and, like `HeldNet`, overrides no `is_noop`, so the harness counts its loads"
    sidecar: "2026-10-25-cold-agent-bar: §8 network stand-ins — the stand's LoadNet beside HeldNet"
    rationale: "Report Symbols / APIs (`struct LoadNet`, `impl NetProvider` 167-175: each `fetch` answers `LOADED_CSS` from a thread of its own after `LOAD_ANSWER_DELAY`; no `is_noop` override, so the harness counts its load; no `blitz_net`, print, env read, socket or listener), F17 (a third hand-written network stand-in; test-plan.md:352 stale)."
    basis: "examples/seven_guis/src/stand.rs:167-175"
note: "The mocking stand-in table (mocking-stubbing-discipline.md:14 · :15, F17) and the stand-check detail rows (test-scope-summary, integration-test-strategy:17 · :21, unit-test-strategy:29 · :43, ci-integration:8) are the cascade's; the session_common reader count (29 → 31) rests on stand_load_page's `snapshot()` being the session_common builder — confirm at the apply. D-tests-framework: no drift (cargo test and python unittest as before)."
