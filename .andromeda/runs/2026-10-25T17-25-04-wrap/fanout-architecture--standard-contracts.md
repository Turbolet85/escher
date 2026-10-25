proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: '## Standard Contracts'
    change: 'In the Headless stand item, after «it installs no telemetry, reads no env var and names no `blitz_net`.» insert «Since 2026-10-25-cold-agent-bar two more boots stand beside the lean four, neither a `LeanTask`: `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell` through the private `cells_root` (examples/seven_guis/src/stand.rs:131-139), and `boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts the load page in `TaskShell` titled "Load" — its ids `load-fetch` (Button "Load", the stand accent `#3a5ce5` with white, hover `#2a4cd3`, active `#1f3fc1`) and `load-done` (Paragraph "Loaded", `display: none` until its sheet lands), no CSS selecting by id — its options'' `net_provider` replaced by `Some(Arc::new(LoadNet))` through `..options`, every other field kept (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:177-243); `pub const LOAD_ANSWER_DELAY: Duration` is 200 ms, and the private `LoadNet`, a third hand-written network stand-in beside blitz-tests'' `HeldNet` and `stand_load_wait`''s `LateNet`, answers each `fetch` of the private `LOAD_URL` (`http://stand.test/load.css`) with the fixed `LOADED_CSS` from a thread of its own after that delay — no `is_noop` override, so the harness counts its load, and no `blitz_net`, print, env read, socket or listener (examples/seven_guis/src/stand.rs:167-175); `options` keeps `net_provider: None` for every other caller, so every other boot stays offline (the unit test `the_stand_options_stay_offline`, examples/seven_guis/src/stand.rs:283-291).»'
    sidecar: 'standard-contracts: the headless stand gains `boot_cells` and `boot_load` (the load page, `LOAD_ANSWER_DELAY` 200 ms, the in-process `LoadNet` stand-in), every other boot offline (2026-10-25-cold-agent-bar, F1 · F17)'
    rationale: 'The report''s Symbols / APIs add the public `seven_guis::stand::boot_cells` (stand.rs:131-135, `cells_root` 137-139) and `boot_load` with `pub const LOAD_ANSWER_DELAY` and the private `LoadNet` (141-152, 167-175, the page 177-243), `stand::options` keeping `net_provider: None` for every other caller; F17 (`stand-offline`) marks standard-contracts.md:120 stale, and Expected amendments name §Standard Contracts → Headless stand with that site — a new public API unregistered in its contract is D-arch-resources drift.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:27-40
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 233
    section: '## Standard Contracts'
    change: 'In the CLIs item, replace «the driver''s command line over the four lean stand tasks since 2026-10-10-driver-cli: `main` installs the telemetry sink first, then hands its argv, the stand''s four slugs `counter` · `flight-booker` · `timer` · `crud` and the stand''s boot» with «the driver''s command line over the stand''s served tasks — the four lean tasks since 2026-10-10-driver-cli and, since 2026-10-25-cold-agent-bar, six: `main` installs the telemetry sink first, then hands its argv, the six slugs of its private `TASKS: [(&str, Served); 6]` table over `enum Served { Lean(LeanTask), Cells, Load }` — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, the usage line''s `apps:` and the MCP `start` tool''s `enum` following — and the stand''s boot (examples/seven_guis/src/session_host.rs:12-27)»; replace «whose `start` tool boots the same four through the same boot» with «whose `start` tool boots the same six through the same boot»; replace «(`stand::options(true)`; the timer through `stand::boot_timer`)» with «(`stand::options(true)`; the timer through `stand::boot_timer`, `cells` through `stand::boot_cells`, `load` through `stand::boot_load`)»; and replace «the other three tasks'' carrying no time step» with «the other five tasks'' carrying no time step»'
    sidecar: 'standard-contracts: `escher-session` serves six tasks — the lean four, `cells` and `load` — through its private `TASKS` table, the usage line and MCP `start` enum following (2026-10-25-cold-agent-bar, F1)'
    rationale: 'The report''s Symbols / APIs: `escher-session` (session_host.rs:12-27) holds `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]`, the usage line''s `apps:` and the MCP `start` tool''s `enum` following, the Timer keeping `with_time(timer_step)`; F1 (`served-tasks`) marks standard-contracts.md:233 stale and Expected amendments name §Standard Contracts → CLIs with that site.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:49-51
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 108
    section: '## Standard Contracts'
    change: 'In the Test harness item, replace «an offline harness (the stand''s) and a document passed to `wrap` carry no counter» with «an offline harness (every stand boot but the load page''s) and a document passed to `wrap` carry no counter, while the stand''s load page (`stand::boot_load`), whose `LoadNet` overrides no `is_noop`, has its load counted (§Standard Contracts → Headless stand)»'
    sidecar: 'standard-contracts: the harness''s load counter reaches the stand''s load page, every other stand boot still offline (2026-10-25-cold-agent-bar, F17)'
    rationale: 'Sweep of the stand-offline claim after the Headless stand proposal: the report''s Symbols / APIs say `boot_load` replaces its options'' `net_provider` with `Some(Arc::new(LoadNet))`, «no `is_noop` override, so the harness counts its load»; F17 (`stand-offline`): «the stand is offline but for the load page''s in-process stand-in» — line 108''s «an offline harness (the stand''s)» now holds for every stand boot but that one.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:185-189
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 218
    section: '## Standard Contracts'
    change: 'In the `unkeyed_actionable` clause, after «on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer''s dialog open included» insert «, and, since 2026-10-25-cold-agent-bar, on the load page (`stand::boot_load`), its two elements keyed `load-fetch` and `load-done` (tests/blitz-tests/tests/stand_load_page.rs:70-84), so it is empty on the six tasks `escher-session` serves»'
    sidecar: 'standard-contracts: `unkeyed_actionable` reads empty on the load page too, the six served tasks (2026-10-25-cold-agent-bar, F1)'
    rationale: 'F1 (`served-tasks`) marks standard-contracts.md:218 stale; the Outcome reads «`unkeyed_actionable` empty on the six served tasks: MET … Cells and the load page by their own checks» and Coverage of new surfaces says the load page''s two elements are keyed, lint 0; New text, by line lists `stand_load_page.rs` 70-84 «fn the_load_page_is_keyed_and_reads_no_finding_at_boot».'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:382-385
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 163
    section: '## Standard Contracts'
    change: 'Replace «On the stand a driver `click` is accepted on 70 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 7: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, and `task-header-spacer` in each task as `covered` — a box 0 high, at whose centre the hit answers the header that holds it» with «On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, `task-header-spacer` in each task as `covered` — a box 0 high, at whose centre the hit answers the header that holds it — and, since 2026-10-25-cold-agent-bar, the flight booker''s two options `flight-type-one-way` and `flight-type-return` as `covered`, each reading its select''s client rect as its `bounds` (§Standard Contracts → Dioxus DOM bridge); 70 and 7 until then»'
    sidecar: 'standard-contracts: the stand''s click census 70 · 7 → 68 · 9 of 77, the flight booker''s two options refused `covered` (2026-10-25-cold-agent-bar, F6)'
    rationale: 'F6 (`act-ids-census`): «`stand_act_ids`: of 77 snapshot nodes a click is accepted on 68 and refused on 9 (was 70 · 7 — the two options refused `covered`)», standard-contracts.md:163 stale; Symbols / APIs: the snapshot reads the select''s two `ListBoxOption`s «each at the select''s client rect».'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:133-137
    fact: F6
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 211
    section: '## Standard Contracts'
    change: 'Replace «exactly the active flight mode reads pressed» with «exactly the chosen flight type''s option reads selected (since 2026-10-25-cold-agent-bar, `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`; until then exactly the active flight mode button read pressed, the stand''s only `aria-pressed` controls, retired with the flight booker''s two mode buttons, the `pressed` reading now pinned on a fixture by `accessibility_states` alone)»'
    sidecar: 'standard-contracts: the stand''s snapshot-state reading is the flight booker''s chosen option read selected, the retired mode buttons'' pressed reading gone from the stand (2026-10-25-cold-agent-bar, F18)'
    rationale: 'The report''s Retired bullet names the flight booker''s two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`), «the stand''s only `aria-pressed` controls»; Symbols / APIs rename `stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed` → `exactly_the_chosen_flight_type_reads_selected`; F18 (`stand-pressed`) marks standard-contracts.md:211 stale; Deviations: «`accessibility_states` still pins `aria-pressed` on a fixture».'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91-93
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: '## Standard Contracts'
    change: 'In the cold-agent pipe clause, replace «all four lean apps met from the tool''s own `snapshot` answers» with «all six served apps met — counter, flight booker, timer, CRUD, cells and the load page (the four lean apps until 2026-10-25-cold-agent-bar) — from the tool''s own accepted `snapshot` answers, an app reset on its accepted `start` and a paged screen read as the latest answer of each part joined, at least one `lint`, `screenshot` and `advance` call and one part read (`capability-unused` else), and each live `start` leaving a kept check (`recording-not-kept` else) — the live server child run as `mcp --record` into `target/cold-agent/record/` (made `mkdir -m 700`, owner-only, removed by `cleanup` with the state), the replays run without `--record`»; replace «`wrong_calls` recorded and deciding nothing» with «and `wrong_calls` deciding by the founder''s 5 % bar — `wrong_calls × 100 > 5 × tool_calls` fails `wrong-calls-over-bar`, the three reasons after `task-unmet` and before the check''s — the verdict adding `wrong_call_bar_pct` (5) · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`, `span_counts` also counting accepted `tool_call` lines per `lint` · `screenshot` · `advance`; the fixed prompt `STAND_PROMPT` names the six apps and their goals and still no stand id»; and replace «pinned by `test_cold_agent.py` (a `claude` shim and a fake `escher-session`;» with «pinned by `test_cold_agent.py`, 37 cases (33 until 2026-10-25-cold-agent-bar; a `claude` shim and a fake `escher-session` that answers Cells as a paged JSON line of two parts and the load page and, under `--record`, keeps one check per accepted start;»'
    sidecar: 'standard-contracts: the cold-agent pipe meets six apps, requires each capability used and each session kept, and decides by `wrong_calls` at the founder''s 5 % bar; seven verdict fields, three reasons, `test_cold_agent.py` 33 → 37 (2026-10-25-cold-agent-bar, F14 · F9)'
    rationale: 'F14 (`pipe`): «six apps, its verdict''s seven new fields and three reasons, `wrong_calls` deciding by the founder''s 5 % bar, `--record` on the live child, `tasks_met` decoding paged answers», standard-contracts.md:261 stale (also F9 `ci-scripts-104`, `test_cold_agent.py` 33 → 37); Symbols / APIs (the pipe) state the prompt, `DONE`, the new functions, `span_counts`, `tasks_met`, the record directory and the verdict fields and reasons; Harness / gate surface states the contract tests 33 → 37 and the fake server''s Cells, load and `--record` answers.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:70-83
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: '## Standard Contracts'
    change: 'In the visual lint item, after «a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same» insert «; a measured limit since 2026-10-25-cold-agent-bar: a select''s options, each reading its select''s client rect as its `bounds` (§Standard Contracts → Dioxus DOM bridge), read as overlapping, so on the stand the flight booker reads one `overlap` at boot (`flight-type-one-way` × `flight-type-return`) and the other three lean tasks 0 — `stand_lint::the_stand_at_boot_reads_its_measured_findings`, `[0, 1, 0, 0]` in both layout modes, re-pinned as measured on the founder''s «Перезакрепить как измерено (Рекомендую)» — its fix a driver change for every document in `lint.rs`''s `overlaps`, not taken»'
    sidecar: 'standard-contracts: the lint reads a select''s two options as overlapping — the flight booker''s one `overlap` at boot, `stand_lint` re-pinned as measured (2026-10-25-cold-agent-bar, F21 · F3)'
    rationale: 'F21: «the driver lint reads a select''s options as overlapping (the snapshot gives each option its select''s client rect)», a new fact with no master site; F3: «the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return` …), the other three lean tasks 0»; Insufficient fixes: `stand_lint` re-pinned on inputs#I5, the defect unresolved, its fix «a driver change for every document (`packages/escher-driver/src/lint.rs`, `overlaps`)»; Spec claims disproved: `[0, 1, 0, 0]`, both layout modes.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:200-205
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 161
    section: '## Standard Contracts'
    change: 'After «it is never refused for failing to reach its target, its result says so.» insert «A measured hazard since 2026-10-25-cold-agent-bar: right after a `scroll` the engine''s overlay scrollbar is shown, and a `click` whose point lies in its band — a Cells cell — lands on it: the main-button pointerdown is taken as a thumb drag (`hit_with_scrollbar` while the scrollbar''s opacity is above 0) and nothing is dispatched to the element; on the session''s clock the scrollbar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown, so an `advance` past the fade comes before the click, in both layout modes (tests/blitz-tests/tests/stand_cells_served.rs:176-192).»'
    sidecar: 'standard-contracts: a `click` right after a `scroll` can land on the shown overlay scrollbar and dispatch nothing; an `advance` past the fade comes first (2026-10-25-cold-agent-bar, F21)'
    rationale: 'F21: «a click at a cell inside a scrolled box right after the scroll lands on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the thumb keeps it shown, so an `advance` past the fade is needed before the click», no master site; Decisions & corrections: «Measured: right after `scroll`, the engine''s overlay scrollbar takes a main-button pointerdown at a cell in its band as a thumb drag (`events/pointer.rs`, `hit_with_scrollbar` with `scrollbar_opacity > 0`)»; New text, by line: stand_cells_served.rs 176-192 «a_click_right_after_the_scroll_lands_on_the_shown_scrollbar», `for incremental in [false, true]`.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:355-357
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 214
    section: '## Standard Contracts'
    change: 'After «on the stand each lean task''s screen at boot reads Counter 755 · FlightBooker 1287 · Timer 1259 · Crud 2049 bytes, the same in both layout modes, on the dev host (Linux)» insert «, while a served screen can pass the budget — since 2026-10-25-cold-agent-bar Cells, served beside the lean four (`stand::boot_cells`), reads 47,949 bytes, 740 lines, in 5 parts, walked part by part through the driver in both layout modes (tests/blitz-tests/tests/stand_cells_served.rs:91-133)»'
    sidecar: 'standard-contracts: Cells, a served stand screen, passes `SNAPSHOT_TEXT_BUDGET` — 47,949 bytes in 5 parts, walked through the driver (2026-10-25-cold-agent-bar, F16)'
    rationale: 'F16: «every stand screen under `SNAPSHOT_TEXT_BUDGET` no longer holds for a served screen: Cells reads 47,949 bytes in 5 parts»; Spec claims disproved: «false for a served screen — Cells 47,949 bytes, 740 lines, 5 parts (`stand_cells_served::cells_is_walked_in_parts_through_the_driver`)»; New text, by line: stand_cells_served.rs 91-133, `for incremental in [false, true]`.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:263-265
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 182
    section: '## Standard Contracts'
    change: 'In the load wait item, after «the two committed Counter checks meet no load» insert «; since 2026-10-25-cold-agent-bar the stand''s load page (`stand::boot_load`, its `LoadNet` answering after `LOAD_ANSWER_DELAY`, 200 ms) is the load wait''s stand reading: a driver `click` on `load-fetch` waits out its load inside the settled step and the screen after it shows `load-done`, in both layout modes (tests/blitz-tests/tests/stand_load_page.rs:34-68)»'
    sidecar: 'standard-contracts: the load wait read on the stand''s load page — a click waits out its 200 ms load and shows `load-done` (2026-10-25-cold-agent-bar, F17)'
    rationale: 'F17 (`stand-offline`): the load page''s in-process stand-in `boot_load` / `LoadNet`; Symbols / APIs: `LOAD_ANSWER_DELAY` 200 ms, `load-done` `display: none` until the sheet lands, and `stand_requirement_sweep`''s v020-07 witness `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`; New text, by line: stand_load_page.rs 19-23 `Session::start("load", …)` and 34-68, `for incremental in [false, true]`.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:56-60
    fact: F17
note: D-arch-decisions reads clean (Dependencies none; `LoadNet` a hand-written `NetProvider` over blitz-traits, already a seven_guis dependency); the `path:N` citations at lines 151 (`stand_act_ids.rs:55`), 211 (`stand_snapshot_state.rs`), 234 (`session_host.rs:8-60`) and 261 (`cold-agent.sh`, `test_cold_agent.py`) sit in files this chunk moved and are left to the P2 citation sweep; line 214's FlightBooker 1287-byte screen is not re-read by the report and stays as written; the lint `overlap` defect's route owner is still to be pinned at this wrap's P5.
