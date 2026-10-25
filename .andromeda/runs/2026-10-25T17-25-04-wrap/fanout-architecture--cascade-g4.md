proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 211
    section: "## Standard Contracts"
    change: "In the stand clause, replace «exactly the active flight mode reads pressed» with «exactly the chosen flight type's option reads selected — since 2026-10-25-cold-agent-bar the flight booker's `select#flight-type` (`ComboBox` \"Flight type\", options `flight-type-one-way` · `flight-type-return`) in place of its two `aria-pressed` mode buttons `flight-one-way` · `flight-return`, retired, so the stand states no `aria-pressed`, which `accessibility_states` still pins on a fixture»; the CRUD-row and date-field readings stay."
    sidecar: "standard-contracts.md:211 — the stand's pressed reading retired with the flight booker's mode buttons; the chosen flight type's option reads selected."
    rationale: "Report **Retired**: the two `aria-pressed` mode buttons, the stand's only `aria-pressed` controls, are gone; F18 `stand-pressed` lists standard-contracts.md:211 stale; the test was renamed `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected` (Symbols / APIs, renamed tests). The line's `stand_snapshot_state.rs:N` citations are the P2 citation sweep's (the file gained lines 6-8 · 121-131 · 375-392 …, New text, by line)."
    basis: .andromeda/architecture-detail/standard-contracts.md:211
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 211
    section: "## Standard Contracts"
    change: "After «— a paged one its JSON line `{\"text\",\"part\",\"parts\"}` —» add: «a JSON line whose quotes are escaped, so a substring match over the raw answer misses the screen's own text and a reader decodes the line first, as the cold-agent pipe does (`paged` · `screen_text`, scripts/cold-agent.sh:178-193) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»."
    sidecar: "standard-contracts.md:211 — a paged snapshot answer over MCP escapes the screen text's quotes; readers decode before matching."
    rationale: "Report Decisions & corrections: «Measured: a paged snapshot answers over MCP as a JSON line whose quotes are escaped, so a substring match over the raw answer misses the screen's own text»; Symbols / APIs: the pipe's new `paged` · `screen_text` (178-193)."
    basis: .andromeda/architecture-detail/standard-contracts.md:211
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 218
    section: "## Standard Contracts"
    change: "After «on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer's dialog open included» add «, and, since 2026-10-25-cold-agent-bar, on the load page `stand::boot_load` mounts, its two elements keyed `load-fetch` · `load-done` (tests/blitz-tests/tests/stand_load_page.rs:70-84)», so the check reads empty on all six served tasks."
    sidecar: "standard-contracts.md:218 — `unkeyed_actionable` empty on the load page too, the six served tasks."
    rationale: "Report F1 `served-tasks` lists standard-contracts.md:218 stale; Outcome (a11y): «`unkeyed_actionable` empty on the six served tasks: MET»; Coverage of new surfaces: the load page's two elements keyed; New text, by line: stand_load_page.rs 70-84 `the_load_page_is_keyed_and_reads_no_finding_at_boot`."
    basis: .andromeda/architecture-detail/standard-contracts.md:218
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 226
    section: "## Standard Contracts"
    change: "Append to the select entity: «Because each option reads its select's client rect, the driver's lint reads a select's options as overlapping: the stand's flight booker reads one `overlap` at boot (`flight-type-one-way` × `flight-type-return`), the other three lean tasks 0, `stand_lint` pinned as measured on the founder's word (`the_stand_at_boot_reads_its_measured_findings`); the defect is unresolved, its fix a driver change for every document in the lint's overlap reading (`packages/escher-driver/src/lint.rs`, `overlaps`) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»."
    sidecar: "standard-contracts.md:226 — a select's options at its client rect read as a lint `overlap`; the defect left open, measured on the flight booker."
    rationale: "Report **Insufficient fixes**: «the driver's lint reading a select's two options as overlapping, since both carry the select's client rect — unresolved; its fix is a driver change for every document»; F3 `lint-at-boot` `[0, 1, 0, 0]`; Decisions: «Перезакрепить как измерено (Рекомендую)» (inputs#I5)."
    basis: .andromeda/architecture-detail/standard-contracts.md:226
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 226
    section: "## Standard Contracts"
    change: "Append to the select entity: «A select's choice moves no class or text `Harness::dom_string` shows, so `stand_id_persistence` chooses the return flight and books in one re-render — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»."
    sidecar: "standard-contracts.md:226 — a select choice moves nothing `dom_string` shows."
    rationale: "Report Decisions & corrections: «Measured: a select choice moves no class or text `Harness::dom_string` shows (`stand_id_persistence` read the DOM unchanged)»; Reverted / negative API facts: the separate re-render step folded into the Book step."
    basis: .andromeda/architecture-detail/standard-contracts.md:226
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 233
    section: "## Standard Contracts"
    change: "Restate the `escher-session` clause over six served tasks: «the driver's command line over the stand's served tasks since 2026-10-10-driver-cli — the four lean ones until 2026-10-25-cold-agent-bar, six since: `main` … hands its argv, the stand's six slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` and the stand's boot to `escher_driver::command_line`»; «whose `start` tool boots the same six through the same boot»; «through `seven_guis::stand` only (`stand::options(true)`; the timer through `stand::boot_timer`, Cells — no `LeanTask` — through `stand::boot_cells`, the load page through `stand::boot_load`)»; «the other five tasks' carrying no time step and still taking `advance`»."
    sidecar: "standard-contracts.md:233 — `escher-session` serves six tasks (Cells and the load page beside the lean four)."
    rationale: "Report F1 `served-tasks`: standard-contracts.md:233 stale; Symbols / APIs: `TASKS: [(&str, Served); 6]` — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, the usage line's `apps:` and the MCP `start` enum follow; `boot_cells` / `boot_load` its boots. Line 234's `session_host.rs:8-60` citation is the P2 citation sweep's (F1 citation row standard-contracts.md:234)."
    basis: .andromeda/architecture-detail/standard-contracts.md:233
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 233
    section: "## Standard Contracts"
    change: "After the six slugs add: «— held in a private `enum Served { Lean(LeanTask), Cells, Load }` table, `TASKS: [(&str, Served); 6]`, from which the usage line's `apps:` and the MCP `start` tool's `enum` are written as program-fixed text, the Timer still carrying `with_time(timer_step)` (examples/seven_guis/src/session_host.rs:12-27)»."
    sidecar: "standard-contracts.md:233 — the `Served` table writes the usage line's apps and the MCP `start` enum."
    rationale: "Report Symbols / APIs: «`escher-session` (session_host.rs:12-27): a private `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]` … the usage line's `apps:` and the MCP `start` tool's `enum` follow (program's own fixed text). The Timer keeps `with_time(timer_step)`»."
    basis: .andromeda/architecture-detail/standard-contracts.md:233
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 187
    section: "## Standard Contracts"
    change: "Replace «the driver's command line over the four lean stand tasks» with «the driver's command line over six served stand tasks — the four lean ones, Cells and the load page»."
    sidecar: "architecture.md:187 — the CLIs line names six served tasks."
    rationale: "Report F1 `served-tasks` lists architecture.md:187 stale; Expected amendments: «architecture §Standard Contracts → CLIs (`escher-session` over six tasks, its usage line and MCP `start` enum) — sites architecture.md:187, standard-contracts.md:233»."
    basis: .andromeda/architecture.md:187
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 261
    section: "## Standard Contracts"
    change: "Restate the pipe's verdict clause: `STAND_PROMPT` names six apps (counter, flight booker, timer, CRUD, cells, the load page) and their goals, still no stand id; the live server child runs `mcp --record` into the owner-only `target/cold-agent/record/` (replays without `--record`); `passed` needs, beside the existing atoms, all six apps met from the tool's own accepted `snapshot` answers (an app reset on its accepted `start`, a paged screen's parts joined by their latest answers), `lint`, `screenshot`, `advance` each used and a part read (else `capability-unused`), one kept check per accepted start (else `recording-not-kept`), and `wrong_calls × 100 ≤ 5 × tool_calls` (else `wrong-calls-over-bar`), the three reasons after `task-unmet` and before the check's; the verdict adds `wrong_call_bar_pct` (5) · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`; the replay's expect match reads a paged answer's decoded text; replace «`wrong_calls` recorded and deciding nothing» with «`wrong_calls` deciding by the founder's 5 % bar since 2026-10-25-cold-agent-bar (recorded and deciding nothing until then)»; `test_cold_agent.py` 37 cases. The eight discovered test files stand."
    sidecar: "standard-contracts.md:261 — the cold-agent pipe: six apps, `--record` on the live child, seven verdict fields, three reasons, `wrong_calls` deciding by the 5 % bar."
    rationale: "Report F14 `pipe`: standard-contracts.md:261 stale; Symbols / APIs (the pipe) and Schema / config: the seven fields, three reasons and `target/cold-agent/record/`; Harness / gate surface: contract tests 33 → 37; F9 lists :261 stale under `ci-scripts-104`, but the line states no leg test count and its «eight test files» still holds. The line's `cold-agent.sh:N` · `test_cold_agent.py:N` citations are the P2 citation sweep's (the file grew by 127 lines in 25 ranges)."
    basis: .andromeda/architecture-detail/standard-contracts.md:261
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 163
    section: "## Standard Contracts"
    change: "Restate the Headless stand line: «`seven_guis::stand` boots the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`), whose one stylesheet fetch an in-process stand-in, `LoadNet`, answers after `LOAD_ANSWER_DELAY` (200 ms) — every other boot `net_provider: None`; `timer_step` hands the Timer its ticks for a driver session; it installs no telemetry, reads no env var and names no `blitz_net`»."
    sidecar: "architecture.md:163 — the headless stand boots Cells and a load page answered by the in-process `LoadNet`, every other boot offline."
    rationale: "Report Symbols / APIs: `boot_cells` (stand.rs:131-135), `boot_load` (141-152), `pub const LOAD_ANSWER_DELAY` 200 ms, `LoadNet` (167-175), `stand::options` keeping `net_provider: None`; F1 `served-tasks` and F17 `stand-offline` both list architecture.md:163 stale; Expected amendments: «architecture §Standard Contracts → Headless stand … sites architecture.md:163»."
    basis: .andromeda/architecture.md:163
note: the Headless stand detail (standard-contracts.md:120, F17 `stand-offline` stale) and F1 · F2 · F3 · F12 have no picked entity in this group — the new-detail pass places them; architecture.md:163 is proposed here as the §Standard Contracts broad line.
