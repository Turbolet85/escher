group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «is the driver's command line over the four lean tasks and the one place a lean task, the stand's boot and the timer's tick handle meet a driver `Session`»
  source: .andromeda/architecture-detail/existing-scopes.md:120 «`escher-session` — the driver's command line over six served tasks (the four lean ones, Cells and the load page) and, in its host role»
  replace-with: «is the driver's command line over six served tasks — the four lean ones, Cells and the load page — and the one place a served task, the stand's boot and the timer's tick handle meet a driver `Session`»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «the four slugs `counter` · `flight-booker` · `timer` · `crud` and the stand's boot»
  source: .andromeda/architecture-detail/standard-contracts.md:249 «the six slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (a private `enum Served { Lean(LeanTask), Cells, Load }` table, `TASKS: [(&str, Served); 6]`»
  replace-with: «the six slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (a private `TASKS` table, from which the usage line's `apps:` and the MCP `start` tool's `enum` are written) and the stand's boot»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «the boot starts a session through `stand` only and hands the timer's session its time step (`stand::timer_step` over the tick handle, through `Session::with_time`; the other three carry none»
  source: .andromeda/architecture-detail/standard-contracts.md:249 «the other five tasks' carrying no time step and still taking `advance`, which moves their session's clock alone»
  replace-with: «the boot starts a session through `stand` only (Cells through `stand::boot_cells`, the load page through `stand::boot_load`) and hands the timer's session its time step (`stand::timer_step` over the tick handle, through `Session::with_time`; the other five carry none»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «its `start` tool boots one of the four through the same boot»
  source: .andromeda/architecture-detail/standard-contracts.md:249 «an MCP server on the binary's standard streams whose `start` tool boots the same six through the same boot»
  replace-with: «its `start` tool, its `enum` naming the six, boots one of the six served tasks through the same boot»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «(on `crud`, per package and under the workspace build)»
  source: .andromeda/architecture-detail/occupied-resources.md:56 «`host_log` on `crud` and on `cells` (its paged part read: a sentinel committed into a part-2 cell, then `snapshot --part 2`;»
  replace-with: «(on `crud`, per package and under the workspace build, and on `cells`, its paged part read — a sentinel committed into a part-2 cell, then `snapshot --part 2`, in `hl-cells`)»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles`)»
  source: .andromeda/architecture-detail/occupied-resources.md:70 «`stamped_lines`, `found`, `occurrences`, the unix-only `recorded_needles` and `cells_needles(part)` (part `part`'s author keys of the in-process Cells screen, booted through `stand::boot_cells`,»
  replace-with: «`stamped_lines`, `found`, `occurrences`, the unix-only `recorded_needles` and `cells_needles(part)` (part `part`'s author keys of the in-process Cells screen, booted through `stand::boot_cells`, and the middle column-A cell it holds, for `host_log`'s Cells part read))»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`escher` (`target/debug/escher-session`, spawned through the per-run `mcp.json`, `RUST_LOG=info`, stderr to `target/cold-agent/server.log`), and its judge spawns two more `escher-session mcp` children to replay the agent's check, full and ablated»
  source: .andromeda/architecture-detail/occupied-resources.md:53 «which spawns `sh` → `escher-session mcp --record target/cold-agent/record` (`target/debug/escher-session`, `RUST_LOG=info` set on that child only, its stderr to `target/cold-agent/server.log`)»
  replace-with: «`escher` (`target/debug/escher-session mcp --record target/cold-agent/record`, the owner-only record directory `run` makes and `cleanup` removes, its kept checks never printed, spawned through the per-run `mcp.json`, `RUST_LOG=info`, stderr to `target/cold-agent/server.log`), and its judge spawns two more `escher-session mcp` children, without `--record`, to replay the agent's check, full and ablated»

- leaf: .claude/docs/services/seven_guis.md:18   class: omission
  leaf-text: «and TaskShell (header with back button, title, spacer over a scrolling body).»
  source: .andromeda/layout-templates.md:14 «the headless stand also mounts two screens in TaskShell that are no `LeanTask`, each by a boot of its own: Cells through `boot_cells` (`cells_root`), and the load page through `boot_load`»
  replace-with: «and TaskShell (header with back button, title, spacer over a scrolling body); headless only, two screens beside the lean four, each by a boot of its own — Cells (`boot_cells`) and the load page (`boot_load`: TaskShell titled "Load" over the button `load-fetch` "Load", white on `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, and the paragraph `load-done` "Loaded", `display: none` until the stylesheet the click links lands, no CSS selecting by id).»

- leaf: .claude/docs/services/seven_guis.md:19   class: omission
  leaf-text: «each a fresh `VirtualDom` mounting the task in TaskShell (no Home) with a no-op Back;»
  source: .andromeda/architecture-detail/standard-contracts.md:128 «`boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts a load page in `TaskShell` titled "Load"»
  replace-with: «each a fresh `VirtualDom` mounting the task in TaskShell (no Home) with a no-op Back; `boot_cells(options)`, Cells in TaskShell, no `LeanTask`, its screen past the snapshot text's budget and read in parts; `boot_load(options)`, the load page in TaskShell, its `net_provider` replaced by the private `LoadNet` — each `fetch` answering the fixed `LOADED_CSS` for the fixed `LOAD_URL` `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), no `is_noop` override, so the harness counts its load — every other boot offline;»

- leaf: .claude/docs/services/seven_guis.md:30   class: retired
  leaf-text: «Three states are stated in markup, by attribute alone (2026-10-10-snapshot-states; no element, class, style, key, id or order moved): the flight booker's mode buttons `flight-one-way` / `flight-return` state `aria-pressed`, its date inputs `flight-start` / `flight-return-date` state `aria-invalid`, `crud-list` is a `listbox` and each CRUD row an `option` with `aria-selected` — always the literal texts `"true"` and `"false"`. A snapshot reads them as `pressed`, `invalid` and `selected`.»
  source: .andromeda/layout-templates.md:10 «and the stand states no `aria-pressed`, the flight booker's mode a select whose options state its choice by `selected` (below)»
  replace-with: «Two states are stated in markup, by attribute alone (2026-10-10-snapshot-states; no element, class, style, key, id or order moved): the flight booker's date inputs `flight-start` / `flight-return-date` state `aria-invalid`, `crud-list` is a `listbox` and each CRUD row an `option` with `aria-selected` — always the literal texts `"true"` and `"false"`. A snapshot reads them as `invalid` and `selected`. The stand states no `aria-pressed`: the flight booker's mode is one `select#flight-type` (class `type-select`, named by `aria-label` "Flight type" alone) holding `option#flight-type-one-way` (value `one-way`) and `option#flight-type-return` (value `return`), each `selected` from the `flight_type` signal — the snapshot reading it at boot `ComboBox "Flight type"` with value `one-way` and two `ListBoxOption`s, each at the select's client rect.»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «reads the 18 `controls(task)` rows over the four lean tasks at boot through `Harness::text_contrast` against a stated record (pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since the accent moved one step darker — pass 9 · fail 5 until then —»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «the 17 `controls(task)` rows over the four lean tasks at boot each read through `Harness::text_contrast` against a stated record, pass 13 · fail 0 · exempt-inactive 3 · no-text 1»
  replace-with: «reads the 17 `controls(task)` rows over the four lean tasks at boot through `Harness::text_contrast` against a stated record (pass 13 · fail 0 · exempt-inactive 3 · no-text 1 since the accent moved one step darker — pass 9 · fail 5 until then, the fifth former fail's control retired; `flight-type` `#1a1a1a` on `#ffffff` at 17.40 —»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «(`STATE_ROWS`, 19: 18 pass, 1 exempt)»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «(`STATE_ROWS`, 18: 17 pass, 1 exempt — with return chosen, `flight-type` and `flight-return-date` each 17.40)»
  replace-with: «(`STATE_ROWS`, 18: 17 pass, 1 exempt — with return chosen, `flight-type` and `flight-return-date` each 17.40)»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «`stand_keyboard` (14 tests, both layout modes, 11 until 2026-10-14-stand-a11y-assertions, which added the last three and put it in the `a11y` CI leg;»
  source: .andromeda/architecture-detail/existing-scopes.md:106 «`stand_keyboard`, 15 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg; the fifteenth the flight type select»
  replace-with: «`stand_keyboard` (15 tests, both layout modes, 11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg, the fifteenth the flight type select moving its choice by its arrows;»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «the flight booker after `flight-return` (six stops)»
  source: .andromeda/architecture-detail/existing-scopes.md:107 «the flight booker after `return` is chosen on `flight-type` (five stops, `flight-return-date` joining `back-btn` → `flight-type` → `flight-start` → `flight-book`, reached by Tab and ArrowDown)»
  replace-with: «the flight booker after `return` is chosen on `flight-type` (five stops, `flight-return-date` joining `back-btn` → `flight-type` → `flight-start` → `flight-book`, reached by Tab and ArrowDown)»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «(nine rows, 46 witnesses), read by its third test — a `Unit` witness»
  source: .andromeda/architecture-detail/existing-scopes.md:115 «`WITNESSES_020`, the 0.2.0 record, nine rows and 49 witnesses, a `Served` witness naming a check on a page the stand serves beside the lean four (Cells or the load page)»
  replace-with: «(nine rows, 49 witnesses), read by its third test — a `Served` witness naming a check on a page the stand serves beside the lean four (Cells or the load page), a `Unit` witness»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «at boot each of the four lean tasks reads 0 findings in both layout modes, through the verb as through the reader,»
  source: .andromeda/layout-templates.md:54 «and the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select's client rect»
  replace-with: «at boot the counter, the timer and CRUD read 0 findings in both layout modes, through the verb as through the reader, and the flight booker one `overlap`, `flight-type-one-way` × `flight-type-return` (the snapshot giving each option its select's client rect — the driver lint's reading of a select's options, not a layout defect, owned by a `.andromeda/residuals.md` line),»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «no stand markup, id, class or style moved.»
  source: .andromeda/architecture-detail/existing-scopes.md:135 «blitz-tests' served-page checks: `stand_cells_served`, 4 tests in both layout modes — Cells booted through `stand::boot_cells` and held as a session,»
  replace-with: «no stand markup, id, class or style moved. Two served-page checks (both layout modes, in `stand-repeat` by their `stand_` prefix, both reading `tests/session_common/mod.rs`): `stand_cells_served` (4 tests) holds Cells booted through `stand::boot_cells` as a session — its screen 47,949 bytes in 740 lines walked in 5 parts of `SNAPSHOT_TEXT_BUDGET`, committed cells reading their values, a cell past the viewport in the last part, a click right after a `scroll` landing on the shown overlay scrollbar, and the lint at boot reading its measured findings — and `stand_load_page` (2 tests) the load page booted through `stand::boot_load` — a `click` on `load-fetch` waiting out its load and returning with `load-done` reading "Loaded", and the page keyed (`unkeyed_actionable()` empty) with no finding at boot.»
