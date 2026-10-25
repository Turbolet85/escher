proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "re-read the citation `tests/blitz-tests/tests/stand_act_ids.rs:55` (the no-area `covered` reading) against the changed file and restate it at the line that now holds that reading — the file gained lines 4-8 · 42 · 60-61 above and around it, its doc comment now 1-16; the claim itself (a box with no width or no height hit at its centre on its holder, reading `covered`) stands, the engine unchanged"
    sidecar: "error-handling#1.1 — `stand_act_ids.rs:55` citation re-pointed after 2026-10-25-cold-agent-bar re-pinned the file (lines 4-8 · 42 added)"
    rationale: "report F6 lists error-handling.md:9 among the `act-ids-census` citation sites (`stand_act_ids.rs:1-15` · `:60`, its doc now 1-16); New text, by line lists stand_act_ids.rs added 4-8 · 42 · 60-61 · 88 · 166-167 · 170-172 · 236-237 · 240-241"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:133
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "replace «while through the `escher-session` host's own sink it is unmeasured, the host booting only the four lean tasks and none paging, its measurement owned by a `.andromeda/residuals.md` line (target: next)» with «and, since 2026-10-25-cold-agent-bar, through the `escher-session` host's own sink — the host serving six tasks, Cells' screen paging — seven_guis `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` (`host_log` 3 → 4 tests) runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and reads `snapshot --part 2`, 0 id and 0 name needles under both builds (examples/seven_guis/tests/host_log.rs:134-277; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md), its residuals line discharged»"
    sidecar: "logging-monitoring#1.1 — the host sink's paged part read measured (Cells, `host_log`, 0 id · 0 name needles, both builds); the unmeasured clause and its residuals owner retired"
    rationale: "report F13 (host_log 3 → 4, `serve cells` at trace, sentinel in a part-2 cell, `snapshot --part 2`, 0 id and 0 name needles, both builds; logging-monitoring.md:14 stale) and Spec claims disproved («the host boots only the four lean tasks and none pages» false since `serve cells`); I4 row 1 discharges residuals.md:38"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 14
    section: "## Logging & Monitoring"
    change: "re-read the recording-session client citation `examples/seven_guis/tests/host_log.rs:42-132` against the changed file and restate it at the lines that now hold that test — lines 15-17 were added above it and 134-278 after it"
    sidecar: "logging-monitoring#1.1 — `host_log.rs:42-132` citation re-pointed after 2026-10-25-cold-agent-bar added host_log.rs 15-17 · 134-278"
    rationale: "host_log.rs is a reached file (report Changes → seven_guis checks); New text, by line lists host_log.rs added 15-17 · 134-278, so the cited block's lines moved (F13 names the file's body shifted)"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:498
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 13
    section: "## Logging & Monitoring"
    change: "re-read the shot-needle citation `examples/seven_guis/tests/host_log.rs:398-411` against the changed file and restate it at the lines that now hold that assertion — 148 lines were added above it (15-17 · 134-278)"
    sidecar: "logging-monitoring#1.1 — `host_log.rs:398-411` citation re-pointed after 2026-10-25-cold-agent-bar"
    rationale: "host_log.rs is reached whole; New text, by line lists host_log.rs added 15-17 · 134-278, both above the cited 398-411"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:499
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 8
    section: "## Logging & Monitoring"
    change: "re-read the telemetry-install citation `examples/seven_guis/src/session_host.rs:30-32` (first thing in `main`) against the changed file and restate it at the lines that now hold the `escher_telemetry::init` call — lines 1-5 · 12-27 (the `Served` enum and the six-row `TASKS` table) were added above it, 43 · 48-50 after"
    sidecar: "logging-monitoring#1.1 — `session_host.rs:30-32` citation re-pointed after 2026-10-25-cold-agent-bar added the `Served` table"
    rationale: "report F1 lists logging-monitoring.md:8 among the `served-tasks` citation sites (`session_host.rs:N`); New text, by line lists session_host.rs added 1-5 · 12-27 · 43 · 48-50"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:104
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 15
    section: "## Logging & Monitoring"
    change: "re-read both `examples/seven_guis/src/session_host.rs:30-32` citations (the binary's one `eprintln!`, the telemetry-install failure) against the changed file and restate them at the lines that now hold it — lines 1-5 · 12-27 added above; the stdout claims (one JSON line per command, usage on stderr with status 2, nothing the caller typed written back) stand, an unknown task still a usage ending"
    sidecar: "logging-monitoring#1.2 — `session_host.rs:30-32` citations re-pointed after 2026-10-25-cold-agent-bar"
    rationale: "report F1 lists logging-monitoring.md:15 among the `served-tasks` citation sites; New text, by line lists session_host.rs added 1-5 · 12-27 · 43 · 48-50; Outcome (arch) an unknown task a usage ending"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:104
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 21
    section: "## Logging & Monitoring"
    change: "after the `check.json` clause add: since 2026-10-25-cold-agent-bar the live server child runs `escher-session mcp --record` into `target/cold-agent/record/` (made `mkdir -m 700`), keeping one owner-only check per accepted `start` — the agent session's screens at rest — which the pipe's `recording` reads only to set `recording_kept` (reason `recording-not-kept`), never printed, copied or committed, removed by `cleanup` with the rest of the state; the replay children run without `--record`"
    sidecar: "logging-monitoring#2.1 — the pipe's at-rest inventory carries `target/cold-agent/record/` (the live child's kept checks, never printed)"
    rationale: "report Symbols / APIs (the live server child runs `mcp --record \"$3\"` into `target/cold-agent/record`, `mkdir -m 700`; replays without `--record`), Schema / config (state set `target/cold-agent/record/`, owner-only, `cleanup` removing it) and Outcome (obs) the kept checks never printed or copied"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:79
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 21
    section: "## Logging & Monitoring"
    change: "extend «counted by span name (`tool_call` lines and their `refused` / `error` endings)» with «and, since 2026-10-25-cold-agent-bar, accepted `tool_call` lines per counted verb (`lint` · `screenshot` · `advance`)», and name the verdict's seven added fields (`wrong_call_bar_pct` · `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` · `recorded_sessions` · `recording_kept`, counts and a boolean) and three added reasons (`capability-unused` · `recording-not-kept` · `wrong-calls-over-bar`, closed words), `wrong_calls` now deciding against the 5 % bar — the metadata-only rule standing"
    sidecar: "logging-monitoring#2.1 — the pipe's server-log counts per verb and its verdict's seven fields and three reasons recorded, still counts, booleans and closed words"
    rationale: "report Symbols / APIs (`span_counts` also returns accepted `tool_call` lines per `COUNTED` verb; verdict fields and reasons added) and Coverage of new surfaces (PII redacted✓: events and verdict carry counts and closed words; planted-marker cases green)"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:76
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 21
    section: "## Logging & Monitoring"
    change: "re-read the citations `scripts/cold-agent.sh:42-536` and `scripts/cold-agent.sh:161-175` against the changed file and restate them at the lines that now hold the events, verdict and transcript-masking code — the pipe gained 127 lines in 25 ranges (among them 3-7 · 10-12 · 170-193 · 219-271 · 483-488 · 571 · 577)"
    sidecar: "logging-monitoring#2.1 — `cold-agent.sh` citations re-pointed after 2026-10-25-cold-agent-bar grew the pipe by 127 lines"
    rationale: "scripts/cold-agent.sh is a reached file; New text, by line lists it added 127 lines in 25 ranges, several above 161 and two past 536 (571 · 577), so both cited spans moved though F14 files this row under the pattern's no-change"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:514
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 403
    section: "## Logging & Monitoring"
    change: "extend «its server log, check and raw transcript are never printed» to «its server log, check, raw transcript and, since 2026-10-25-cold-agent-bar, the live child's kept checks under the owner-only `target/cold-agent/record/` are never printed»"
    sidecar: "security-plan body §Logging & Monitoring — the pipe's never-printed set carries the live child's kept checks"
    rationale: "the broad line the logging-monitoring#2.1 group details; the task's touches say any inventory of the pipe's files at rest or never printed must carry `target/cold-agent/record/` (report Schema / config, Outcome (obs))"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:87
    dependent-of: D-cascade
note: no `## New facts` item is of category error-handling or logging-monitoring, so no D-new-detail proposal; the citation proposals give no new line number because the report lists only added ranges and the orchestrator must read the target line from the changed files
