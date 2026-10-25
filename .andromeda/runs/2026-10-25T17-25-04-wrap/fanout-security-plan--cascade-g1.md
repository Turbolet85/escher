proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 35
    section: "## Threat Model Summary"
    change: "In the Trust boundary's cold-agent clause replace «the cold-agent pipe's one live run is a second carriage» with «the cold-agent pipe's first live run (2026-10-15) is a second carriage»; the rest of the clause (43 tool calls, no `screenshot` called) stays as that run's reading."
    sidecar: "threat-model-summary#4.1: the 2026-10-15 cold-agent run named the first of the pipe's two live runs (2026-10-25-cold-agent-bar)."
    rationale: "Report F15: the founder's one run of 2026-10-25 (6 tasks, 66 calls, 0 wrong, $0.8244) stands beside 0.1.0's 4 tasks, 43 calls; `live-run` stale at threat-model-summary.md:35, so «one live run» no longer names the pipe's only run."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:35
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 37
    section: "## Threat Model Summary"
    change: "Add a continuation line after the `lint` clause: since 2026-10-25-cold-agent-bar the pipe's second live run is a third carriage — through the `claude` client (2.1.308) and the operator's own login it carried six apps' screens to the model provider: element ids, accessible names and control values in snapshot texts, paged parts and diffs, Cells' contents, the flight-type select's options and value, one lint's findings and one screenshot, over 66 tool calls, 0 wrong, $0.8244 — on the founder's «Запускать сейчас (Рекомендую)» (inputs#I6 of escher-0.2.0/chunks/2026-10-25-cold-agent-bar) within his «Один запуск, до $3 (Рекомендую)» (inputs#I3), fired once and never re-fired — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md."
    sidecar: "threat-model-summary#4.1: the 2026-10-25 live run's carriage of the stand's new screens, a shot and a lint's findings among them, recorded as a third carriage on the founder's word (2026-10-25-cold-agent-bar)."
    rationale: "Report F15 and Decisions & corrections: the one live run fired once on «Запускать сейчас (Рекомендую)» (inputs#I6), passed, 0 wrong of 66, $0.8244; Outcome v020-11: `lint_calls` 1, `screenshot_calls` 1, `part_reads` 2; Expected amendments name this carriage a boundary widening on the founder's own F3."
    fact: F6
  - detector: D-cascade
    severity: escalate
    file: .andromeda/security-plan.md
    near: 27
    section: "## Threat Model Summary"
    change: "End the line with «— the cold-agent runs carried the screen so, the 2026-10-25 run a screenshot and a lint's findings among it.» in place of «— the cold-agent run carried the screen so.»"
    sidecar: "Threat Model Summary, the MCP stdio vector: the cold-agent runs, the 2026-10-25 one carrying a shot and a lint's findings (2026-10-25-cold-agent-bar)."
    rationale: "Report F15 and the task's Touches: threat-model-summary.md:35 is stale beside body Threat Model line 27; the 2026-10-25 run carried a screenshot and a lint's findings where the 2026-10-15 run called no `screenshot`."
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 25
    section: "## Input Validation"
    change: "In the `STAND_PROMPT` clause replace «the four lean apps one at a time, their goals,» with «since 2026-10-25-cold-agent-bar six apps one at a time — counter, flight booker, timer, CRUD, cells and the load page — and their goals (the flight booker's return booking chosen on its flight type control plus one screenshot; Cells' `A0` 5, `B1` `=A0*2` reading 10 and `Z25` `=B1+1` reading 11 plus one visual lint; the load page's resource loaded and its message shown),»; the allowlist `stand`, the exit-2 order and «naming no element id and no app slug» kept."
    sidecar: "input-validation#1.6: the cold-agent prompt names six apps and their goals, still no element id (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (the pipe): `STAND_PROMPT` names six apps and their goals, still naming no stand id; F14 `pipe` stale at input-validation.md:25; the Touches re-read input-validation.md:23 and body line 57 as standing (allowlist `stand`, usage exit 2 first)."
    basis: .andromeda/security-plan-detail/input-validation.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 34
    section: "## Input Validation"
    change: "Add to the How: since 2026-10-25-cold-agent-bar the pipe reads a tool's snapshot answer through `paged` and `screen_text`, decoding it as a paged line only when it is a JSON object with exactly the keys `text`, `part` and `parts` and a string `text`, else reading the raw answer; `tasks_met` reads accepted snapshot answers only, resets an app on its accepted `start` and joins the latest answer of each part, and the replay's expect match reads a paged answer's decoded text the same way (scripts/cold-agent.sh:178-187; scripts/cold-agent.sh:190-193) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md."
    sidecar: "input-validation#1.9: the pipe's closed decode of a paged snapshot answer for `tasks_met` and the replay (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (the pipe): new `paged` · `screen_text` (178-193), `tasks_met` reads accepted snapshot answers and joins each part's latest, the replay's expect match reads a paged answer's decoded text; New text lists `paged` 178-187 and `screen_text` 190-193; contract test `test_a_paged_answer_is_read_as_its_parts_joined`."
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 63
    section: "## Input Validation"
    change: "Append to the line: «since 2026-10-25-cold-agent-bar a paged snapshot answer is read only as a JSON object of exactly `text` · `part` · `parts`, else raw; the live child's kept checks are counted per accepted `start` and never printed; and the verdict is decided by `wrong_calls` against the founder's 5 % bar.»"
    sidecar: "Input Validation, cold-agent check: the paged decode, the kept checks read and never printed, the 5 % wrong-call bar (2026-10-25-cold-agent-bar)."
    rationale: "Report F14 `pipe` stale at security-plan.md:63: six apps, the verdict's new reasons with `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`), `--record` on the live child, `tasks_met` decoding paged answers; Coverage: kept checks parsed, never printed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 45
    section: "## Input Validation"
    change: "Add to the How: since 2026-10-25-cold-agent-bar the cold-agent pipe's live server child runs under these rules — `escher-session mcp --record` into `target/cold-agent/record`, which the pipe makes with `mkdir -m 700` and its `cleanup` removes with the rest of its state; the pipe's replays run without `--record`."
    sidecar: "input-validation#1.12: the cold-agent pipe's live child records under the recording-directory rules into `target/cold-agent/record` (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (the pipe): the live server child runs `mcp --record` into `target/cold-agent/record` (made `mkdir -m 700`), replays without `--record`; Schema / config: the state set gains `target/cold-agent/record/`, owner-only, `cleanup` removing it."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 48
    section: "## Input Validation"
    change: "Add to the How: since 2026-10-25-cold-agent-bar the cold-agent pipe reads kept checks too — its `recording` step reads the live child's record directory for one numbered kept check per accepted `start` (verdict `recording_kept`, reason `recording-not-kept`), reads a directory it cannot read as no check kept, and prints no check's content (scripts/cold-agent.sh:257-271)."
    sidecar: "input-validation#1.13: the cold-agent pipe's `recording` step as a reader of the live child's kept checks (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (the pipe): new `recording` (257-271), verdict field `recording_kept` and reason `recording-not-kept`; New text lists `recording` 257-271 with its `except OSError:` arm 262-263; Outcome (obs): the kept checks never printed or copied."
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 63
    section: "## Input Validation"
    change: "Replace «with the stand's four slugs (`counter`, `flight-booker`, `timer`, `crud`, a four-entry table)» with «with the stand's six slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) — since 2026-10-25-cold-agent-bar a closed six-entry `TASKS` table over the private `Served` enum (`Lean`, `Cells`, `Load`), an unknown task a usage ending, the usage line's `apps:` and the MCP `start` tool's enum following it as the program's own fixed text inside the ratified carriers»."
    sidecar: "input-validation#1.16: `escher-session` serves six slugs from a closed `TASKS` table (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs: `escher-session` (session_host.rs:12-27) holds `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]`, the usage line's `apps:` and the MCP `start` enum following; F1 `served-tasks` stale at input-validation.md:66; Outcome (arch): an unknown task a usage ending."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 66
    section: "## Input Validation"
    change: "Re-point the first `session_host.rs` citation of the closing list, the slug table's, to `examples/seven_guis/src/session_host.rs:12-27` (the `Served` enum and the six-entry `TASKS` table)."
    sidecar: "input-validation#1.16: the slug table's citation re-pointed to session_host.rs:12-27 (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs places `Served` and `TASKS` at session_host.rs:12-27, and New text lists session_host.rs's added 12-27 with `enum Served` at 12-18; F1 names input-validation.md:66 stale."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 70
    section: "## Input Validation"
    change: "After «`start` on an app outside the binary's apps refused `malformed`» add «— since 2026-10-25-cold-agent-bar the binary's apps are six (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), the `start` tool's `app` enum listing them as the program's own fixed table text inside the ratified carrier (the founder's ruling, 2026-10-12), the framing and bounds unchanged»."
    sidecar: "input-validation#1.17: the MCP `start` tool's app enum lists six slugs (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs: the usage line's `apps:` and the MCP `start` tool's `enum` follow `TASKS` (program's own fixed text); the task's Touches: the `start` enum grows to six slugs, fixed table text inside the ratified carrier, the MCP group's bounds and framing re-read and standing."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 125
    section: "## Input Validation"
    change: "Replace «and the stand boots offline on its bundled font» with «and the stand boots on its bundled font, offline but for the load page (`stand::boot_load`, since 2026-10-25-cold-agent-bar), whose one fixed stylesheet request `http://stand.test/load.css` the in-process `LoadNet` answers — no `blitz_net`, no `@font-face` source and no agent- or user-supplied URL»."
    sidecar: "input-validation#1.35: the stand offline but for the load page's in-process `LoadNet` (2026-10-25-cold-agent-bar)."
    rationale: "Report F17 `stand-offline`: the stand is offline but for the load page's in-process stand-in (`boot_load` / `LoadNet`, a third hand-written network stand-in); Symbols / APIs: `LOAD_URL` `http://stand.test/load.css`, no `blitz_net`, every other caller `net_provider: None`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 371
    section: "## Input Validation"
    change: "Add to the How: since 2026-10-25-cold-agent-bar the flight booker's mode is `select#flight-type`, whose `oninput` writes the `flight_type` signal by a closed match on the `input` value — `return` a return flight, every other value one-way — over its two options `flight-type-one-way` (value `one-way`) and `flight-type-return` (value `return`); the two `aria-pressed` mode buttons are retired (examples/seven_guis/src/tasks/flight_booker.rs:66-86)."
    sidecar: "input-validation#1.109: the flight booker's `flight-type` select and its closed match (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (flight_booker.rs:66-86): one `select#flight-type` with the two options, `oninput` a closed match; Retired: the two `aria-pressed` mode buttons; Coverage: `select#flight-type` validation by a closed match on the `input` value."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/data-protection.md
    near: 8
    section: "## Data Protection"
    change: "Add to the entry: since 2026-10-25-cold-agent-bar a second medium — the cold-agent pipe's live server child runs `escher-session mcp --record` into `target/cold-agent/record` (created `0700`) and keeps one owner-only check per accepted `start`, holding the agent session's screens at rest — Cells' contents, the flight-type select's options and value and a lint's findings among them; never printed, copied or committed, removed with the pipe's state by its `cleanup`, the replays run without `--record`; a full recorded Cells session measured 312,431 bytes and the load page's 1,660, under the 1,048,576-byte recording bound."
    sidecar: "data-protection#1.1: the cold-agent pipe's live child as a second kept-check medium (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs (the pipe) and Schema / config: `mcp --record` into `target/cold-agent/record`, owner-only, `cleanup` removing it; Decisions & corrections: a recorded Cells session 312,431 bytes, the load page's 1,660, under the 1,048,576 bound; Outcome (security): no kept check committed."
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 185
    section: "## Data Protection"
    change: "Append to the line: «since 2026-10-25-cold-agent-bar the cold-agent pipe's live child keeps its checks so in `target/cold-agent/record` (`0700`), never printed, copied or committed, removed by the pipe's `cleanup`.»"
    sidecar: "Data Protection, Files (kept checks): the cold-agent pipe's live child's kept checks (2026-10-25-cold-agent-bar)."
    rationale: "The task's Touches: body Data Protection line 185 gains a second kept-check medium; report Schema / config: the pipe's state set gains `target/cold-agent/record/`, owner-only."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 31
    section: "## API Security"
    change: "Add a continuation after the configuration: since 2026-10-25-cold-agent-bar the wait is witnessed on a served stand page — the load page (`stand::boot_load`, `escher-session start load`) issues one fetch of the fixed `http://stand.test/load.css`, answered in process by `LoadNet` after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own with fixed CSS bytes, with no `blitz_net`, socket, listener, print or env read and no `is_noop` override, so the harness counts the load and the page's click waits it out within `LOAD_WAIT` (`stand_load_page`); every other stand boot holds `net_provider: None` (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175)."
    sidecar: "api-security#2.6: the load wait witnessed on the stand's load page through the in-process `LoadNet` (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs: `boot_load` (141-152), `impl NetProvider for LoadNet` (167-175), `LOAD_ANSWER_DELAY` 200 ms, no `is_noop` override, silence census 0; v020-07 witness `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`; unit test `the_stand_options_stay_offline`."
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/secret-management.md
    near: 15
    section: "## Secret Management"
    change: "Before «the committed live evidence of both runs» add «since 2026-10-25-cold-agent-bar the pipe's second live run fired exactly once through the same login (`claude` 2.1.308) — no API key, env read, CI secret, new secret source or auth library — on the founder's «Запускать сейчас (Рекомендую)» within his «Один запуск, до $3 (Рекомендую)», its committed evidence (`live-check.json` and a host-path-masked `live-transcript.jsonl`) read clean by the operator pass's hygiene read with no kept check committed;» and read «of both runs» as «of the two earlier runs»."
    sidecar: "secret-management#1.2: the 2026-10-25 live run through the same login, `claude` 2.1.308, its evidence hygiene-clean (2026-10-25-cold-agent-bar)."
    rationale: "Report Dev-tool versions: `claude` 2.1.308, the live run's `client_version` the same; F15 no-change at secret-management.md:14 (the login unchanged); Outcome (security): hygiene clean, no kept check committed; the task's Touches: no API key, env read, CI secret or new secret source."
note: "F6 and its body line (security-plan.md:27) record the 2026-10-25 live run's carriage of the stand's new screens, a boundary widening on the founder's own word, so the wrap halts on it once (playbook «Boundary widening»); moved citations the report gives no new number for (security-plan.md:301, and those in threat-model-summary#3.1, input-validation#1.11 · #1.53 · #1.57) are left to the P2 citation sweep."
