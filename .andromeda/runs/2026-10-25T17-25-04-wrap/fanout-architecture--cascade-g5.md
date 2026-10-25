proposals:
  - detector: D-cascade
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: "After the 2026-10-15 run's clause, append: and since 2026-10-25-cold-agent-bar the founder's one live run of 2026-10-25 carried the stand's new screens — Cells' contents, the flight type select's options and value, a lint's findings and a shot — to the model provider, on «Запускать сейчас (Рекомендую)» within «Один запуск, до $3 (Рекомендую)»: 6 tasks, 66 calls, 0 wrong, $0.8244, `claude` 2.1.308, beside 0.1.0's 4 tasks and 43 calls (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md)"
    sidecar: "occupied-resources Outbound hosts: the 2026-10-25 live run's carriage of the stand's new screens to the model provider, on the founder's I6 word within his F3 budget"
    rationale: "Report F15 (live-run) names occupied-resources.md:17 stale; Expected amendments route the carriage as a boundary widening on the founder's own F3 that the wrap halts on once"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:175-178
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: "Beside the test base URLs, add: the stand's load page URL `http://stand.test/load.css`, since 2026-10-25-cold-agent-bar the one stylesheet `stand::boot_load`'s page names, answered in process by the stand's `LoadNet` with the fixed `LOADED_CSS` and never reached on the network — no `blitz_net`, socket or listener (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175)"
    sidecar: "occupied-resources Outbound hosts: the load page's `http://stand.test/load.css`, answered in process by `LoadNet`, registered"
    rationale: "Report Symbols / APIs: private `LOAD_URL` `http://stand.test/load.css` answered by `LoadNet` (stand.rs:167-175), a new named endpoint the registry's URL rows do not hold"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:33-37
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 24
    section: "## Occupied Resources"
    change: "In the cold-agent pipe's state area `target/cold-agent/`, after `check.json`, add: and, since 2026-10-25-cold-agent-bar, `record/`, made owner-only (`mkdir -m 700`), where the live `escher-session mcp` child keeps its kept checks under `--record` — one per accepted `start`, never printed or copied — removed by `cleanup` with the state, no kept check from it committed"
    sidecar: "occupied-resources Filesystem: the cold-agent state area's owner-only `record/` for the live child's kept checks"
    rationale: "Report Schema / config: the pipe's state set gains `target/cold-agent/record/`, owner-only, `cleanup` removing it with the state; Outcome (security) no kept check committed"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:87-88
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 24
    section: "## Occupied Resources"
    change: "Extend the recorded live runs' committed copies: and, since 2026-10-25-cold-agent-bar, `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-*` (`live-run.md`, `live-verdict.json`, `live-check.json`, `live-transcript.jsonl`, the transcript host-path-masked)"
    sidecar: "occupied-resources Filesystem: the 2026-10-25 live run's committed evidence copies listed"
    rationale: "Report Outcome (obs): the evidence copies `live-check.json` and a masked `live-transcript.jsonl` as step 12 directs, beside `live-verdict.json` and `live-run.md`; the row lists only the 0.1.0 runs' copies"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:390-393
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 39
    section: "## Occupied Resources"
    change: "In `scripts/cold-agent.sh run` spawns one `claude` client … `sh` → `escher-session mcp`, write the live child as `escher-session mcp --record target/cold-agent/record` since 2026-10-25-cold-agent-bar (the directory made owner-only), `RUST_LOG=info` on that child only, its stderr to `target/cold-agent/server.log`"
    sidecar: "occupied-resources Process-wide state: the cold-agent live server child runs under `--record`"
    rationale: "Report F14 (pipe) names occupied-resources.md:39 stale: the live server child runs `mcp --record \"$3\"` into `target/cold-agent/record`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:79-80
  - detector: D-cascade
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 40
    section: "## Occupied Resources"
    change: "Say the judge's two replay children run without `--record` (since 2026-10-25-cold-agent-bar), and after the 2026-10-15 replay crossing's words add: the 2026-10-25 run's on the founder's «Запускать сейчас (Рекомендую)» within «Один запуск, до $3 (Рекомендую)» (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "occupied-resources Process-wide state: the replay children run without `--record`; the 2026-10-25 run's word recorded"
    rationale: "Report F14 and F15 name occupied-resources.md:40 stale: replays run without `--record`; the 2026-10-25 run fired once on inputs#I6 within inputs#I3"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:178-179
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: "## Occupied Resources"
    change: "Write `host_log` on `crud` and, since 2026-10-25-cold-agent-bar, on `cells` — a sentinel committed into a part-2 cell, then `snapshot --part 2`, 0 id and 0 name needles under both builds — each with `RUST_LOG=trace` (examples/seven_guis/tests/host_log.rs:134-277)"
    sidecar: "occupied-resources Process-wide state: `host_log` also spawns the host role on `cells` for its paged part read"
    rationale: "Report F13: `host_log` 3 → 4 tests, `serve cells` at `RUST_LOG=trace`, `snapshot --part 2`; the row says the host role is spawned on `crud` alone"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157-159
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 44
    section: "## Occupied Resources"
    change: "Add: since 2026-10-25-cold-agent-bar the stand's load page — `stand::boot_load`'s network stand-in `LoadNet` — answers each `fetch` from a thread of its own, one a load-page boot spawns per fetch, sending the fixed `LOADED_CSS` after `LOAD_ANSWER_DELAY` (200 ms), with no socket, listener, print or env read (examples/seven_guis/src/stand.rs:167-175)"
    sidecar: "occupied-resources Process-wide state: `LoadNet`'s per-fetch thread registered"
    rationale: "Report Symbols / APIs: `impl NetProvider` 167-175, each `fetch` answering `LOADED_CSS` from a thread of its own after `LOAD_ANSWER_DELAY`; silence census entry 10 reads 0"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:33-37
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 44
    section: "## Occupied Resources"
    change: "Qualify «the `escher-session` host itself is one process holding one session and one listener on its main thread, spawning no thread» as spawning no thread of its own — serving `load`, the stand's `LoadNet` spawns one per fetch — and the `mcp` role's «no thread of its own» likewise, under a `start load`"
    sidecar: "occupied-resources Process-wide state: the host's and the `mcp` role's «no thread» qualified by the load page's per-fetch thread"
    rationale: "Report F1: the host serves six tasks, `load` among them, and `LoadNet` answers each fetch from a thread of its own — the host's «spawning no thread» no longer holds whole for a served `load`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:49-51
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 211
    section: "## Occupied Resources"
    change: "After «the script timer thread», add: the stand's load page's network stand-in `LoadNet`, one thread per fetch"
    sidecar: "architecture §Occupied Resources: the broad threads line names `LoadNet`'s per-fetch thread"
    rationale: "Report Symbols / APIs: `LoadNet` answers each fetch from a thread of its own — a new thread source the broad line's thread list does not name"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:35-37
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 50
    section: "## Occupied Resources"
    change: "After `recorded_needles`, add: and, since 2026-10-25-cold-agent-bar, `cells_needles(part)` — part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds — for `host_log`'s Cells part read (examples/seven_guis/tests/common/mod.rs:425-454)"
    sidecar: "occupied-resources: the seven_guis shared module's log-check readers gain `cells_needles`"
    rationale: "Report Symbols / APIs: seven_guis tests `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454), a caller of `boot_cells`, read by `host_log`'s new test"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:54-55
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "Write `ENABLED_TAB_ORDER` as (the flight booker after `return` is chosen on its `flight-type` select, CRUD after a row is selected), dropping «after `flight-return`»"
    sidecar: "existing-scopes blitz-tests: `ENABLED_TAB_ORDER`'s flight state named by the select, the retired `flight-return` dropped"
    rationale: "Report Retired: `flight-one-way` · `flight-return` gone; F2 the mode is the select `flight-type`, `flight-type` held in `ENABLED_TAB_ORDER`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91-93
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "Write `stand_keyboard`, 15 tests in both layout modes (14 until 2026-10-25-cold-agent-bar, which added the flight type select moving its choice by its arrows — tests/blitz-tests/tests/stand_keyboard.rs:249-309), and Enter and Space on each focused stand button, 16 rows (20 until then, `BUTTONS` 10 → 8)"
    sidecar: "existing-scopes blitz-tests: `stand_keyboard` 14 → 15 tests, its activation table 20 → 16 rows"
    rationale: "Report F7 (activation-rows) names existing-scopes.md:106 stale: the activation table 20 → 16 rows, `BUTTONS` 10 → 8, tests 14 → 15; new test `the_flight_type_select_moves_its_choice_by_its_arrows`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:138-140
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 107
    section: "## Existing Scopes"
    change: "Write the walk beyond boot as the flight booker after `return` is chosen on `flight-type` — five stops, `flight-return-date` joining the boot four `back-btn` → `flight-type` → `flight-start` → `flight-book`, reached by Tab and ArrowDown — in place of «after `flight-return` (six stops)»"
    sidecar: "existing-scopes blitz-tests: the flight booker's enabled walk restated on the select, five stops"
    rationale: "Report F2 (flight-mode) names existing-scopes.md:107 stale; Retired names `flight-return`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:114-116
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 109
    section: "## Existing Scopes"
    change: "Write «a step that changes a control's selected or invalid reading returning that node as it reads after the step» — since 2026-10-25-cold-agent-bar a flight type choice returning the select's value and both options' selected reading, the stand stating no pressed control (`aria-pressed` still pinned on a fixture by `accessibility_states`)"
    sidecar: "existing-scopes blitz-tests: the stand's act checks no longer exercise a pressed reading"
    rationale: "Report F18: the stand states no `aria-pressed` anywhere; renamed `stand_act_states::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`; Deviations: the pressed reading leaves the stand"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:190-193
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "Write `stand_contrast`'s control rows as the 17 `controls(task)` rows (18 until 2026-10-25-cold-agent-bar), pass 13 · fail 0 · exempt-inactive 3 · no-text 1 (14 · 0 · 3 · 1 until then), `flight-type` reading `#1a1a1a` on `#ffffff` at 17.40, 15px 400, and with return chosen `flight-type` and `flight-return-date` each 17.40; and `STATE_ROWS`, 18: 17 pass, 1 exempt (19: 18 · 1 until then)"
    sidecar: "existing-scopes blitz-tests: `stand_contrast` control rows 18 → 17, `STATE_ROWS` 19 → 18, the select's pair at 17.40"
    rationale: "Report F5 (contrast-rows) names existing-scopes.md:112 stale"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125-129
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "In «the five former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` now `#ffffff` on `#3a5ce5` at 5.47», say the four that stand read so, `flight-one-way` retired at 2026-10-25-cold-agent-bar with the flight booker's mode buttons, its row given way to `flight-type`"
    sidecar: "existing-scopes blitz-tests: the retired `flight-one-way` leaves the contrast history's standing rows"
    rationale: "Report Retired: `flight-one-way` gone; F5 its accent row gives way to `flight-type`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:126-128
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 114
    section: "## Existing Scopes"
    change: "Write `WITNESSES_020` as twelve rows since 2026-10-25-cold-agent-bar (nine rows and 46 witnesses until then), which added v020-07 `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-08 `stand_act_states.rs::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` and v020-09 `stand_cells_served.rs::cells_is_walked_in_parts_through_the_driver` with a `Served` subject — a page served beside the lean four — re-pointed v010-17's flight witness and v020-05's lint witness to the renamed tests and carried the two new files in `source`"
    sidecar: "existing-scopes blitz-tests: `WITNESSES_020` nine → twelve rows, the `Served` subject added"
    rationale: "Report F20 (witnesses) names existing-scopes.md:114 stale; Symbols / APIs list the three rows and the re-pointed witnesses"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:56-60
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "Add: since 2026-10-25-cold-agent-bar a served Cells screen — `stand_cells_served`, 4 tests in both layout modes, in `stand-repeat` by its `stand_` prefix: Cells booted through `stand::boot_cells` and held as a session, its screen 47,949 bytes, 740 lines, walked in 5 parts of `SNAPSHOT_TEXT_BUDGET` through the driver; committed cells reading their values and a cell past the viewport in the last part; a click right after a `scroll` landing on the shown overlay scrollbar; and the lint at boot reading its measured findings (tests/blitz-tests/tests/stand_cells_served.rs:91-133; tests/blitz-tests/tests/stand_cells_served.rs:135-174; tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:194-209)"
    sidecar: "existing-scopes blitz-tests: `stand_cells_served` and the served Cells screen's measured size and parts"
    rationale: "Report Spec claims disproved: Cells 47,949 bytes, 740 lines, 5 parts (`stand_cells_served::cells_is_walked_in_parts_through_the_driver`); Outcome: `stand_cells_served` green in both layout modes (4)"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:263-265
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "Add: since 2026-10-25-cold-agent-bar the served load page — `stand_load_page`, 2 tests in both layout modes, in `stand-repeat` by its `stand_` prefix: the page booted through `stand::boot_load` and held as a session, a `click` on `load-fetch` waiting out its load (`LoadNet`, `LOAD_ANSWER_DELAY` 200 ms) and returning with `load-done` reading Loaded, and the page keyed with no finding at boot (tests/blitz-tests/tests/stand_load_page.rs:34-68; tests/blitz-tests/tests/stand_load_page.rs:70-84)"
    sidecar: "existing-scopes blitz-tests: `stand_load_page` listed"
    rationale: "Report Files: new `tests/blitz-tests/tests/stand_load_page.rs` (84 lines), one test target per file; Outcome: green in both layout modes (2)"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:13-14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "Add `stand_cells_served` (since 2026-10-25-cold-agent-bar, tests/blitz-tests/tests/stand_cells_served.rs:19-21) to the checks that read `tests/session_common/mod.rs` through `mod session_common;`, and `stand_load_page` where its file declares the module, re-read at apply"
    sidecar: "existing-scopes blitz-tests: `session_common`'s readers gain `stand_cells_served`"
    rationale: "Report New text, by line: `stand_cells_served.rs` 19-21 «use session_common::{»; the readers listed stop at `stand_replay`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:548-549
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 118
    section: "## Existing Scopes"
    change: "Write the native-only module `stand` as the headless stand booting the lean four in TaskShell and, since 2026-10-25-cold-agent-bar, Cells (`boot_cells`, still no `LeanTask`) and the load page (`boot_load`, whose options alone carry the in-process network stand-in `LoadNet`, every other boot `net_provider: None`) (examples/seven_guis/src/stand.rs:131-135; examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175)"
    sidecar: "existing-scopes seven_guis: the `stand` module boots Cells and the load page beside the lean four"
    rationale: "Report F1 (served-tasks) names existing-scopes.md:118 stale; F17 the stand is offline but for the load page's in-process stand-in; Expected amendments name the site"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:98-101
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 118
    section: "## Existing Scopes"
    change: "Write `escher-session` as the driver's command line over six served tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, its private `Served` table (examples/seven_guis/src/session_host.rs:12-27) — in place of «over the four lean tasks», and «the one place a served task, the stand's boot and the timer's tick handle meet a driver `Session`»"
    sidecar: "existing-scopes seven_guis: `escher-session` over six served tasks"
    rationale: "Report Symbols / APIs: `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]` (session_host.rs:12-27); F1"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:49-51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 118
    section: "## Existing Scopes"
    change: "After the shared module `tests/common/mod.rs`, add: which since 2026-10-25-cold-agent-bar also holds `cells_needles(part)`, part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds (examples/seven_guis/tests/common/mod.rs:425-454)"
    sidecar: "existing-scopes seven_guis: the shared test module's `cells_needles`"
    rationale: "Report Symbols / APIs: seven_guis tests `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454)"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:54-55
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 134
    section: "## Existing Scopes"
    change: "Re-count `tests/session_common/mod.rs`'s readers with `stand_cells_served` (its `use session_common::{…}`, tests/blitz-tests/tests/stand_cells_served.rs:19-21) and `stand_load_page` where it declares the module, the reading dated to 2026-10-25-cold-agent-bar; `tests/fixtures/mod.rs`'s 24 holds (the two new files declare no `mod fixtures;`)"
    sidecar: "existing-scopes: `session_common`'s reader count moves with the new served-page checks"
    rationale: "Report New text, by line: `stand_cells_served.rs` 19-21 «use session_common::{»; F10 the two new files declare no `mod fixtures;`"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:150-151
    dependent-of: D-cascade
note: citations this chunk moved — cross-cutting-patterns.md:14 and occupied-resources.md:38 · :44 (`session_host.rs`), occupied-resources.md:24 (`cold-agent.sh`), :44 (`host_log.rs`), :50 (`common/mod.rs:371-514`), :65 (`cold-agent.sh:505`, the sweep read no-change), existing-scopes.md:107 (`stand_keyboard.rs`), :110 (`stand_act_*`), :114 (`stand_requirement_sweep.rs`) — are left to the P2 citation sweep; the report names no state directory for `host_log`'s new Cells test (occupied-resources.md:25's list).
