proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 163
    section: '## Standard Contracts'
    change: 'On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, `task-header-spacer` in each task as `covered` (a box 0 high, at whose centre the hit answers the header that holds it), and the flight booker''s two options `flight-type-one-way` · `flight-type-return` as `covered`, each read at its select''s client rect (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md; the 70 · 7 reading it replaces, with the two `aria-pressed` mode buttons, at escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md).'
    sidecar: 'standard-contracts.md:163 — the stand click census restated 70 · 7 → 68 · 9, the flight booker''s two select options refused `covered` (2026-10-25-cold-agent-bar, F6).'
    rationale: 'Report Counts / qualifiers moved F6 (`act-ids-census`, stale standard-contracts.md:163): of 77 snapshot nodes a click is accepted on 68 and refused on 9, where it was 70 · 7, the two options refused `covered`. The Retired bullet removes `flight-one-way` and `flight-return`, and Symbols / APIs puts the `select#flight-type` options at the select''s client rect.'
    basis: .andromeda/architecture-detail/standard-contracts.md:163
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 169
    section: '## Standard Contracts'
    change: 'A paged `snapshot` crosses MCP as its JSON line, the screen text''s quotes escaped inside it, so a substring match over the raw answer misses the screen''s own text, and a reader decodes the line before it matches; the cold-agent pipe does this (`paged` · `screen_text`) both for `tasks_met` and for its replay''s expect match (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).'
    sidecar: 'standard-contracts.md:169 — a paged snapshot''s MCP line escapes the screen text''s quotes, so a reader decodes before it matches (2026-10-25-cold-agent-bar, F11).'
    rationale: 'Report Decisions & corrections: «Measured: a paged snapshot answers over MCP as a JSON line whose quotes are escaped, so a substring match over the raw answer misses the screen''s own text». Symbols / APIs (the pipe''s `paged` · `screen_text` at cold-agent.sh:178-193, and the replay''s expect match reading a paged answer''s decoded text). The fact rests on the MCP result statement at this line.'
    basis: scripts/cold-agent.sh:178-193
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 178
    section: '## Standard Contracts'
    change: 'Measured against the bound: a full recorded Cells session keeps a check of 312,431 bytes and the load page''s keeps one of 1,660, both under the 1,048,576-byte `MAX_CHECK_BYTES` (probe through the built binary; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).'
    sidecar: 'standard-contracts.md:178 — the recording''s byte bound measured against a full Cells session (312,431 bytes) and the load page (1,660) (2026-10-25-cold-agent-bar, F12).'
    rationale: 'Report Decisions & corrections: «Measured: a full recorded Cells session keeps a check of 312,431 bytes, the load page''s 1,660, under the 1,048,576 bound». The entity states that bound (`MAX_CHECK_BYTES`).'
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: '## Standard Contracts'
    change: 'A measured defect of the `overlap` reading: the snapshot gives each of a select''s options its select''s client rect, so the lint reads two options of one select as overlapping. On the stand the flight booker reads one `overlap` at boot (`flight-type-one-way` × `flight-type-return`) and the other three lean tasks read 0, which `stand_lint` pins as measured in both layout modes. The defect is open, and its fix is a driver change for every document in the lint''s overlap reading (`packages/escher-driver/src/lint.rs`, `overlaps`), its owner pinned at the 2026-10-25-cold-agent-bar wrap (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).'
    sidecar: 'standard-contracts.md:181 — the lint reads a select''s two options as overlapping, measured on the flight booker at boot and left open (2026-10-25-cold-agent-bar, F7).'
    rationale: 'Report Insufficient fixes (`stand_lint` re-pinned to the measured `overlap`, the defect unresolved, its fix in lint.rs `overlaps`), Spec claims disproved (a 0-finding lean stand is false, `[0, 1, 0, 0]`), and F3 `lint-at-boot`. The founder''s word on the re-pin is in inputs#I5.'
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 182
    section: '## Standard Contracts'
    change: 'On the stand the load wait has one served witness, the load page (`seven_guis::stand::boot_load`). Its in-process `LoadNet` answers every fetch of the fixed `http://stand.test/load.css` with the fixed `LOADED_CSS` after `pub const LOAD_ANSWER_DELAY` (200 ms), and with no `is_noop` override the harness counts that fetch as a load in flight. A `click` on `load-fetch` therefore waits the load out inside `LOAD_WAIT` and returns with `load-done` shown, in both layout modes (`stand_load_page`; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).'
    sidecar: 'standard-contracts.md:182 — the load wait''s served stand witness, the load page whose `LoadNet` answers after 200 ms as a counted load (2026-10-25-cold-agent-bar, F3).'
    rationale: 'Report Symbols / APIs (`boot_load` stand.rs:141-152, `impl NetProvider for LoadNet` 167-175: each fetch answered after `LOAD_ANSWER_DELAY` with no `is_noop` override, so the harness counts its load) and the v020-07 witness `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message` (34-68).'
    basis: examples/seven_guis/src/stand.rs:167-175
    fact: F3
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 163
    section: '## Standard Contracts'
    change: '- **Headless stand (seven_guis, native only):** `seven_guis::stand` boots the four lean tasks, and beside them Cells (`boot_cells`, no `LeanTask`) and the load page (`boot_load`), in a pinned 800x600 Light viewport with one bundled font. Every boot is offline except the load page''s: its options carry the in-process `LoadNet` stand-in, which answers its one fetch after `LOAD_ANSWER_DELAY` (200 ms). `timer_step` hands the Timer its ticks for a driver session. The module installs no telemetry, reads no env var and names no `blitz_net`.'
    sidecar: 'architecture.md:163 — the headless stand boots Cells and the load page beside the lean four, offline except the load page''s `LoadNet` stand-in (2026-10-25-cold-agent-bar, F1 · F17).'
    rationale: 'Report Symbols / APIs: the new public `boot_cells` (stand.rs:131-135), `boot_load` (141-152) and `LOAD_ANSWER_DELAY`, with `stand::options` keeping `net_provider: None` for every other caller. F1 `served-tasks` and F17 `stand-offline` both name architecture.md:163 stale, and the Expected amendments list it under §Standard Contracts → Headless stand. An unregistered public API breaks D-arch-resources.'
    basis: examples/seven_guis/src/stand.rs:131-152
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 187
    section: '## Standard Contracts'
    change: '- **CLIs:** the command lines of the examples and apps (`paint_bench`, `screenshot`, `preact_script`, `multi_doc`, browser, rdme, wgpu_texture, wpt), and `escher-session <verb> [arguments] --session <dir>`. That is the driver''s command line over six served stand tasks: `counter`, `flight-booker`, `timer`, `crud`, `cells` and `load`, its usage line''s apps and the MCP `start` tool''s `enum` both written from one private table. It has a `serve` host role and an `mcp` role, answers one line of JSON and exits `0` accepted · `1` refused · `2` usage · `3` session error.'
    sidecar: 'architecture.md:187 — `escher-session` restated over six served stand tasks, where it served the four lean ones (2026-10-25-cold-agent-bar, F1).'
    rationale: 'Report Symbols / APIs: `escher-session` (session_host.rs:12-27) holds `TASKS: [(&str, Served); 6]`, and the usage line''s `apps:` and the MCP `start` enum follow it. F1 `served-tasks` names architecture.md:187 stale, and the Expected amendments list §Standard Contracts → CLIs.'
    basis: examples/seven_guis/src/session_host.rs:12-27
note: 'Out of this group''s picked set, though the report names them stale in this category file: standard-contracts.md:120 (stand-offline), :211 (stand-pressed), :218 · :233 (served-tasks) and :261 (pipe, ci-scripts 104 → 108). In 17.1 two line citations can have moved with reached files, and the report gives no new number for either: line 151''s `stand_act_ids.rs:55` (the F6 citation row) and line 156''s `cli_commands.rs:686-711`. Both are left to the P2 citation sweep.'
