proposals:
  - detector: D-security-input
    severity: escalate
    file: .andromeda/security-plan-detail/input-validation.md
    near: 25
    section: "## Input Validation"
    change: "In the CLI arguments (cold-agent.sh) row, replace «the four lean apps one at a time, their goals, a snapshot of each final screen» with: the six apps one at a time — counter, flight booker, timer, CRUD, cells and the load page — and their goals (since 2026-10-25-cold-agent-bar: the flight booker's return booking chosen on its flight type control plus one screenshot; Cells `A0` 5, `B1` `=A0*2` reading 10, `Z25` `=B1+1` reading 11 plus one visual lint; the load page's resource loaded and its message shown), a snapshot of each final screen, the check as the reply's last fenced json block — still naming no element id (`test_the_prompt_names_no_stand_id`); the live run carried the new screens (Cells' contents, the select's options and value, a lint's findings, a shot) to the model provider on the founder's «Запускать сейчас (Рекомендую)» (inputs#I6 of escher-0.2.0/chunks/2026-10-25-cold-agent-bar) within his F3 «Один запуск, до $3 (Рекомендую)» (inputs#I3) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "input-validation: the cold-agent prompt names six apps (cells and the load page added) and their goals; the live run's carriage of the new screens recorded on the founder's word (2026-10-25)"
    rationale: "Report Changes › Symbols / APIs (the pipe: `STAND_PROMPT` names six apps and their goals, still naming no stand id) and F14 (`pipe`, input-validation.md:25 stale); F15 and Expected amendments › security-plan §Threat Model Summary / §Surfaces name this site for the live run's carriage of the new screens — a boundary widening on the founder's own F3, playbook «Boundary widening» (.andromeda/playbook.md:36), so the wrap halts on it once as the answer card"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:70
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 63
    section: "## Input Validation"
    change: "In the CLI arguments (escher-session) row, replace «the stand's four slugs (`counter`, `flight-booker`, `timer`, `crud`, a four-entry table)» with: the stand's six slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load` — since 2026-10-25-cold-agent-bar the private six-entry `TASKS: [(&str, Served); 6]` over `enum Served { Lean(LeanTask), Cells, Load }`, the usage line's `apps:` and the MCP `start` tool's `enum` following as the program's own fixed text, the Timer keeping `with_time(timer_step)`)"
    sidecar: "input-validation: escher-session hands command_line six slugs (cells and load added beside the lean four)"
    rationale: "Report Changes › Symbols / APIs (`escher-session` session_host.rs:12-27: `enum Served` and `TASKS` of six) and F1 (`served-tasks`, input-validation.md:66 listed stale — the four-slug claim of this row); the `start` slug stays a closed table, so the validation the row states holds"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:49
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 66
    section: "## Input Validation"
    change: "Re-point the row's citation `examples/seven_guis/src/session_host.rs:13-18` to `examples/seven_guis/src/session_host.rs:12-27` (the `Served` enum and the six-entry `TASKS` table)"
    sidecar: "input-validation: the escher-session slug-table citation re-pointed to session_host.rs:12-27"
    rationale: "Report F1 lists input-validation.md:66 stale under `served-tasks`; the report's New text, by line names session_host.rs added 12-27 with `enum Served` @14 — the four-entry table the old 13-18 cited is gone"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:465
    dependent-of: D-security-input
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 34
    section: "## Input Validation"
    change: "Add a row after the Cold-agent check row — **Boundary:** Cold-agent run reading (cold-agent.sh, the transcript's answers and the live child's kept checks); **What to validate:** a paged snapshot answer in the transcript and the kept checks the live `mcp --record` child leaves; **How (stack-specific):** since 2026-10-25-cold-agent-bar `paged` takes a JSON line as a part only when it parses as an object with exactly the keys `text` · `part` · `parts` and a string `text`, anything else (a `ValueError` included) read as plain text, and `screen_text` joins the latest answer of each part, so `tasks_met` (accepted snapshot answers only, an app reset on its accepted `start`) and the replay's expect match read a paged screen's decoded text (scripts/cold-agent.sh:178-187; scripts/cold-agent.sh:190-193); the live server child runs `mcp --record` into `target/cold-agent/record` (made `mkdir -m 700`), replays run without `--record`, and `recording` reads that directory for the verdict's `recorded_sessions` · `recording_kept`, an `OSError` reading none kept and a run whose accepted starts left no kept check failing `recording-not-kept` (scripts/cold-agent.sh:257-271); the kept checks are never printed or copied, `cleanup` removing the directory with the state; the verdict carries counts and closed words only; pinned by `test_cold_agent.py` (37 cases: `test_a_paged_answer_is_read_as_its_parts_joined`, `test_a_start_that_left_no_kept_check_fails`) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "input-validation: new row — the cold-agent pipe's strict paged-answer read and its owner-only kept-check directory under --record"
    rationale: "Report Changes › Symbols / APIs (the pipe's new `paged` · `screen_text` · `recording`, `--record` on the live child, `tasks_met` decoding paged answers), Schema / config (state set `target/cold-agent/record/`, owner-only) and Coverage of new surfaces (the pipe's verdict fields and `--record` child: validation ✓, kept checks parsed, never printed) — new input the pipe reads, in the category's one-row-per-boundary shape"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:75
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 371
    section: "## Input Validation"
    change: "In the Example apps — 7GUIs inputs row, add after the flight booker's date clause: its flight type is one `select#flight-type` (since 2026-10-25-cold-agent-bar, in place of two buttons) whose `oninput` writes the `flight_type` signal by a closed match — `\"return\"` → return, every other value → one-way (examples/seven_guis/src/tasks/flight_booker.rs:66-86)"
    sidecar: "input-validation: the flight booker's flight-type select validated by a closed match on its input value"
    rationale: "Report Changes › Symbols / APIs (flight_booker.rs:66-86: `oninput` writes the signal by a closed match) and Coverage of new surfaces (`select#flight-type` → validation ✓, closed match on the `input` value); F2 moves the flight booker's mode to the select"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:45
    fact: F2
note: "Line 25's cold-agent.sh citations (18-31 · 33-38 · 547-556 · 564-567), line 34's (F14 citation) and line 66's session_host.rs:34-56 moved with this chunk's edits but their new ranges are not in the report — left to the P2 citation sweep; D-security-auth and D-security-deps read no drift (Dependencies none; the live run's login unchanged, F15 no-change); line 125's «the stand boots offline» still holds — `boot_load`'s `LoadNet` answers one fixed URL with fixed bytes in process (F17 lists no site here); the line-25 proposal carries F15's boundary-widening card site."
