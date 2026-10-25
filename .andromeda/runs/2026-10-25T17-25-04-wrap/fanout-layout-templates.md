proposals:
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'After the stand-mount clause (`the headless stand skips Home and mounts one lean task … in TaskShell through task_in_shell …`) add: since 2026-10-25-cold-agent-bar the stand also mounts two served screens in TaskShell beside the lean four, neither a `LeanTask` — Cells through `boot_cells` (`cells_root`), and the load page through `boot_load`: TaskShell titled "Load" over `load_root` / `LoadPage`, the button `load-fetch` ("Load", white on the stand accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`) and the paragraph `load-done` ("Loaded", `display: none` until the sheet `LOADED_CSS` fetched from `LOAD_URL` lands, answered by the in-process stand-in `LoadNet` after `LOAD_ANSWER_DELAY`, 200 ms), styled by `.load-root` · `.load-body` · `.load-btn` (+`:hover` `:active`) · `.load-done`, no CSS selecting by id, every other boot keeping `net_provider: None` (examples/seven_guis/src/stand.rs:131-135; examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175; examples/seven_guis/src/stand.rs:177-243 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the stand mounts Cells (`boot_cells`) and the load page (`boot_load`, `load-fetch` · `load-done`, its in-process `LoadNet`) in TaskShell beside the lean four (2026-10-25-cold-agent-bar)'
    rationale: 'The report adds a new user-facing screen, the load page (Symbols / APIs: `boot_load`, `LoadPage`, its CSS, ids `load-fetch` and `load-done`), and a served Cells mount (`boot_cells`); layout-templates describes the stand as mounting one lean task alone and has no entry for the load page — an undocumented surface (Changes → Symbols / APIs; F1; F17; Expected amendments: layout-templates §Surface: desktop-native, site layout-templates.md:10).'
    basis: examples/seven_guis/src/stand.rs:141-152
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'In the host-role clause replace `it boots one lean task through seven_guis::stand only` with: it boots one of six served tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — through `seven_guis::stand`, a lean task by its boot, Cells by `boot_cells` and the load page by `boot_load`, through the private `Served` table `TASKS` (examples/seven_guis/src/session_host.rs:12-27)'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the `escher-session` host role boots six served tasks, not one lean task only (2026-10-25-cold-agent-bar)'
    rationale: 'F1: `escher-session` serves six tasks (was the four lean tasks); Symbols / APIs: `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]` (session_host.rs:12-27). The clause restates the one-lean-task mount the primary retires.'
    basis: examples/seven_guis/src/session_host.rs:12-27
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'In the `mcp`-role clause replace `booting one lean task in process on a start tool call through the same boot` with: booting one of the same six served tasks in process on a `start` tool call through the same boot, the `start` tool''s `enum` naming the six (examples/seven_guis/src/session_host.rs:12-27)'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the `mcp` role boots one of the six served tasks on `start` (2026-10-25-cold-agent-bar)'
    rationale: 'F1 and Symbols / APIs: the usage line''s `apps:` and the MCP `start` tool''s `enum` follow `TASKS`, six entries; the clause still says one lean task.'
    basis: examples/seven_guis/src/session_host.rs:12-27
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Primary screens'
    change: 'In the `escher-session` entry replace `start <counter|flight-booker|timer|crud> [--record <dir>]` with `start <task> [--record <dir>]`, the task one plain word of the six served — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — the usage line''s `apps:` and the MCP `start` tool''s `enum` listing the same six (examples/seven_guis/src/session_host.rs:12-27)'
    sidecar: 'layout-templates §Surface: cli → Primary screens: `escher-session start` takes six task words, `cells` and `load` beside the lean four (2026-10-25-cold-agent-bar)'
    rationale: 'F1 (`served-tasks`, stale site layout-templates.md:75 @c189); Expected amendments: layout-templates §Surface: cli, the served task list, site :75; Symbols / APIs: `TASKS` six, the usage line''s `apps:` and the MCP `start` enum follow.'
    basis: examples/seven_guis/src/session_host.rs:12-27
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Primary screens'
    change: 'In the `scripts/cold-agent.sh` entry replace `the four lean apps in one client session` with: six apps in one client session — counter, flight booker, timer, CRUD, cells and the load page — under the fixed prompt `STAND_PROMPT`, which still names no stand id; and beside the transcript and the check add: the live run''s server child runs `mcp --record` into `target/cold-agent/record` (made `mkdir -m 700`, owner-only, removed by `cleanup` with the state, its kept checks never printed), the replays running without `--record`'
    sidecar: 'layout-templates §Surface: cli → Primary screens: the cold-agent pipe drives six apps and keeps the live child''s recording in `target/cold-agent/record` (2026-10-25-cold-agent-bar)'
    rationale: 'F14 (`pipe`, stale site layout-templates.md:74): six apps, `--record` on the live child into `target/cold-agent/record`; Schema / config: the pipe''s state set `target/cold-agent/record/`, owner-only, `cleanup` removing it; Symbols / APIs: `STAND_PROMPT` names six apps, still naming no stand id.'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'After the lean-task id list add the flight booker''s new mode region: since 2026-10-25-cold-agent-bar its mode is one `select#flight-type` (class `type-select`, named by `aria-label` "Flight type" alone) holding `option#flight-type-one-way` (value `one-way`, "one-way flight") and `option#flight-type-return` (value `return`, "return flight"), each `selected` from the `flight_type` signal and its `oninput` a closed match; styled 15px `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, its `:focus` dropping the outline for the `#3a5ce5` border as the date inputs do; its Tab sequence `back-btn` → `flight-type` → `flight-start` → `flight-book` (enabled: + `flight-return-date`, five stops); the snapshot reads it `ComboBox "Flight type"` with value `one-way` and two `ListBoxOption`s, each at the select''s client rect (examples/seven_guis/src/tasks/flight_booker.rs:66-86; examples/seven_guis/src/tasks/flight_booker.rs:160-171 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the flight booker''s mode region is the select `flight-type` with two options, its style, Tab stop and snapshot reading (2026-10-25-cold-agent-bar)'
    rationale: 'Symbols / APIs: the flight booker (flight_booker.rs:66-86) holds one `select#flight-type` with two options, its CSS 160-171, its snapshot reading; F2 gives its Tab sequence; Expected amendments: layout-templates §Surface: desktop-native, the flight booker''s mode control, site :10. The region the chunk adds is not described.'
    basis: examples/seven_guis/src/tasks/flight_booker.rs:66-86
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: desktop-native → IA notes'
    change: 'Restate the visual-lint entry: the stand''s layout reads to the visual lint as measured — at boot the counter, timer and CRUD read 0 findings in both layout modes and, since 2026-10-25-cold-agent-bar, the flight booker one `overlap`, `flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select''s client rect (a driver lint defect left unresolved, `packages/escher-driver/src/lint.rs` `overlaps`; `stand_lint` re-pinned as measured on the founder''s «Перезакрепить как измерено (Рекомендую)»), the load page reading 0; the twelve-Creates `clipped` reading of `crud-person-14` standing (tests/blitz-tests/tests/stand_lint.rs:277-285; tests/blitz-tests/tests/stand_load_page.rs:70-84 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'layout-templates §Surface: desktop-native → IA notes: the lean stand no longer lints clean at boot — the flight booker reads one `overlap` of its select''s two options (2026-10-25-cold-agent-bar)'
    rationale: 'F3 (`lint-at-boot`, stale site layout-templates.md:46): the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`), the other three lean tasks 0; Spec claims disproved: the 0-finding lean stand measured false, `[0, 1, 0, 0]`; Insufficient fixes: the driver defect in `lint.rs` `overlaps` unresolved; inputs#I5; Symbols / APIs: `stand_load_page` reads no finding at boot.'
    basis: tests/blitz-tests/tests/stand_lint.rs:277-285
    dependent-of: D-layout-surface
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'In the lean-task id list replace `flight-one-way` · `flight-return` · `flight-start` · … with `flight-type` · `flight-start` · `flight-return-date` · `flight-book` · `flight-booked`, the select''s options carrying `flight-type-one-way` · `flight-type-return` (examples/seven_guis/src/tasks/flight_booker.rs:66-86)'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the retired mode buttons `flight-one-way` · `flight-return` leave the flight booker''s id list for `flight-type` and its two options (2026-10-25-cold-agent-bar)'
    rationale: 'Retired: the flight booker''s two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`); F2 (`flight-mode`, stale sites layout-templates.md:10 @c2481 · @c2500); Symbols / APIs: the two buttons are gone, `select#flight-type` with `option#flight-type-one-way` and `option#flight-type-return` stands.'
    basis: examples/seven_guis/src/tasks/flight_booker.rs:66-86
  - detector: D-retired
    severity: warning
    section: '§Surface: desktop-native → Primary screens'
    change: 'In the 2026-10-10-snapshot-states clause replace `the flight booker''s mode buttons flight-one-way and flight-return are toggle buttons stating aria-pressed "true" or "false"` with a dated retirement: those two `aria-pressed` toggle buttons and their `.flight-type-row` · `.type-btn` · `.type-btn-active` CSS are gone since 2026-10-25-cold-agent-bar, the select `flight-type` stating its choice by its options'' `selected`, so the stand states no `aria-pressed`; the date inputs'' `aria-invalid` and the CRUD listbox and `aria-selected` options standing, `the three classes that style those states` becoming the classes that style the invalid and selected states; its `flight_booker.rs:69` and `:76` citations giving way to examples/seven_guis/src/tasks/flight_booker.rs:66-86'
    sidecar: 'layout-templates §Surface: desktop-native → Primary screens: the flight booker''s `aria-pressed` toggle buttons and their CSS are retired; the stand states no `aria-pressed` (2026-10-25-cold-agent-bar)'
    rationale: 'Retired: the two `aria-pressed` mode buttons and their CSS (`.flight-type-row`, `.type-btn`, `.type-btn-active`) — the stand''s only `aria-pressed` controls; F2 (`flight-mode`, stale sites layout-templates.md:10 @c3863 · @c3877 · @c3898) and F18 (`stand-pressed`, stale site layout-templates.md:10 @c3941).'
    basis: examples/seven_guis/src/tasks/flight_booker.rs:66-86
    dependent-of: D-retired
note: 'Citations this chunk moved, on line numbers the report does not restate, are left to the P2 citation sweep — layout-templates.md:10 (examples/seven_guis/src/stand.rs:50-76, session_host.rs:42-51 @c2262, flight_booker.rs:66-122 · :90-105 · :93 · :101), :46 (stand_lint.rs:287-314 · :354-389 · :415-442), :74 (cold-agent.sh:33-38 · :564-567), :75 (session_host.rs:13-18 · :34-55 @c7705 · @c7752); line 10 @c6026 «outside the lean four» stays true (no-change).'
