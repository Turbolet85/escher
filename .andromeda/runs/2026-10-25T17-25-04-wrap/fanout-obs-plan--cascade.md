proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "Keep the field list and the reasons as written; mark its two citations `scripts/cold-agent.sh:398-498` and `scripts/cold-agent.sh:161-175` moved and re-read them at apply, because the file grew by 127 lines in 25 ranges, among them 3-7, 10-12, 170-193, 219-271 and 483-488. The fields and reasons added at 2026-10-25-cold-agent-bar follow on the next lines (F3, F4)."
    sidecar: "log-coverage#2.1: the verdict row's cold-agent.sh citations re-read after the pipe grew by 127 lines (2026-10-25-cold-agent-bar)"
    rationale: "The report's New text, by line lists scripts/cold-agent.sh as added 127 line(s) in 25 range(s), with ranges before and inside both cited spans. F14 lists log-coverage.md:14 as stale."
    basis: "scripts/cold-agent.sh:483-488"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "- Since 2026-10-25-cold-agent-bar the verdict adds seven fields: `wrong_call_bar_pct` (5), `lint_calls`, `screenshot_calls`, `advance_calls`, `part_reads`, `recorded_sessions` and `recording_kept`. Each is a count, a boolean or a number, and none holds transcript text, a tool argument, a tool answer, a reply or a check (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "log-coverage: the cold-agent verdict's seven new fields, each a count, boolean or number (2026-10-25-cold-agent-bar, F3)"
    rationale: "The report's Symbols / APIs list the pipe's added verdict fields, and its Coverage of new surfaces says the events and verdict carry counts and closed words only."
    basis: "scripts/cold-agent.sh:483-488"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "- Since 2026-10-25-cold-agent-bar the reasons add `capability-unused`, `recording-not-kept` and `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`), after `task-unmet` and before the check's reasons. `wrong_calls` now decides the verdict by the founder's 5 % bar (scripts/cold-agent.sh:483-488)."
    sidecar: "log-coverage: the cold-agent verdict's three new reasons and wrong_calls deciding by the 5 % bar (2026-10-25-cold-agent-bar, F4)"
    rationale: "The report's Symbols / APIs list the three reasons added after task-unmet, and state that wrong_calls now decides. Its New text lists cold-agent.sh:483-488, the capability, recording and bar checks."
    basis: "scripts/cold-agent.sh:483-488"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "- Since 2026-10-25-cold-agent-bar `span_counts` also returns the accepted `tool_call` span lines of each counted verb (`lint`, `screenshot` and `advance`) from `target/cold-agent/server.log`. The pipe reads that file by span name only and never prints or copies it."
    sidecar: "log-coverage: span_counts counts accepted tool_call lines per counted verb from server.log (2026-10-25-cold-agent-bar, F5)"
    rationale: "The report's Symbols / APIs say span_counts also returns accepted tool_call lines per COUNTED verb. Its obs acceptance says server.log is never printed or copied."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "- Since 2026-10-25-cold-agent-bar the live server child runs `mcp --record` into `target/cold-agent/record/`, created with `mkdir -m 700`, and keeps one check per accepted `start`. `recording` reads that directory to set `recorded_sessions` and `recording_kept`. The pipe never prints or copies the kept checks, `cleanup` removes them with the state, and the replay children run without `--record` (scripts/cold-agent.sh:257-271)."
    sidecar: "log-coverage: the live child's kept checks under target/cold-agent/record/, never printed, removed by cleanup (2026-10-25-cold-agent-bar, F6)"
    rationale: "The report's Symbols / APIs give the live child's --record into target/cold-agent/record (mkdir -m 700) and say replays run without it. Its Schema / config gives the owner-only state set that cleanup removes, and its obs acceptance says the kept checks are never printed or copied."
    basis: "scripts/cold-agent.sh:257-271"
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 14
    section: "## 6. Log Coverage"
    change: "- Since 2026-10-25-cold-agent-bar `tasks_met` reads accepted snapshot answers only. It resets an app on its accepted `start` and joins the latest answer of each part of a paged screen. The replay's expect match also decodes a paged answer's JSON-escaped text (`paged`, `screen_text`; scripts/cold-agent.sh:178-193). The decoded text goes into the match only and never into an event or the verdict."
    sidecar: "log-coverage: tasks_met and the replay's expect match decode paged answers (2026-10-25-cold-agent-bar, F7)"
    rationale: "The report's Symbols / APIs describe tasks_met and the replay's decoding of a paged answer. New text lists paged at 178-187 and screen_text at 190-193."
    basis: "scripts/cold-agent.sh:178-193"
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 169
    section: "## 6. Log Coverage"
    change: "Keep the line, and add at its end: since 2026-10-25-cold-agent-bar `wrong_calls` also decides the verdict against the founder's 5 % bar (`wrong-calls-over-bar`), beside the seven new count and boolean fields."
    sidecar: "obs-plan §6: the verdict's broad line says wrong_calls decides by the 5 % bar (2026-10-25-cold-agent-bar)"
    rationale: "The report's F14 lists obs-plan.md:169 as stale (the pipe: the verdict's seven new fields and three reasons, wrong_calls deciding by the founder's 5 % bar)."
    basis: ".andromeda/obs-plan.md:169"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: "## 6. Log Coverage"
    change: "Keep the endings as written; the usage line's task names are now six (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, the private `Served` table), and the usage line's `apps:` and the MCP `start` tool's `enum` follow them, both still the program's own fixed text with no new diagnostic. Re-read the citation `examples/seven_guis/src/session_host.rs:31` at apply, because the file gained lines 1-5 and 12-27 before it."
    sidecar: "log-coverage#4.1: escher-session's usage line names six tasks; its session_host.rs citation re-read (2026-10-25-cold-agent-bar)"
    rationale: "The report's Symbols / APIs give session_host.rs:12-27, the Served enum and TASKS [(&str, Served); 6], whose usage line and MCP start enum follow as fixed text. F1's citation row lists log-coverage.md:34."
    basis: "examples/seven_guis/src/session_host.rs:12-27"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: "## 6. Log Coverage"
    change: "In the part-read clause, after the mention of `host_log` and `mcp_log` each driving one part read at `trace` with 0 needles, add: since 2026-10-25-cold-agent-bar `host_log`'s fourth test also reads a paged part through the `escher-session` host's own sink. It runs `serve cells` and `snapshot --part 2`, and the host's `command` line reads `verb=snapshot` with 0 id and 0 name needles under both builds. Re-read the citation `examples/seven_guis/tests/host_log.rs:398-411` at apply, because the body shifted by lines 134-278."
    sidecar: "log-coverage#5.1: the host's own-sink part read on served Cells, 0 needles; host_log.rs citation re-read (2026-10-25-cold-agent-bar)"
    rationale: "The report's F13: host_log 3 → 4 tests, `serve cells` at RUST_LOG=trace and snapshot --part 2, 0 id and 0 name needles under both builds. New text lists host_log.rs added at 15-17 and 134-278, before the cited 398-411."
    basis: "examples/seven_guis/tests/host_log.rs:134-277"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 113
    section: "## 6. Log Coverage"
    change: "- observed absent — a print, a log site, an env read, a socket, a listener or `blitz_net` in the stand's load-page boot (`seven_guis::stand::boot_load` and its in-process stand-in `LoadNet`, whose `impl NetProvider` answers each fetch with fixed CSS from its own thread after `LOAD_ANSWER_DELAY`, 200 ms) · searched: the silence census over `examples/seven_guis/src/stand.rs`, reading 0 (examples/seven_guis/src/stand.rs:167-175; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its entry 10)"
    sidecar: "log-coverage: the load page's LoadNet stand-in is silent, census 0 (2026-10-25-cold-agent-bar, F2)"
    rationale: "The report's Symbols / APIs: LoadNet (impl NetProvider at 167-175) uses no blitz_net, print, env read, socket or listener, and silence census entry 10 reads 0. The gate line `cat examples/seven_guis/src/stand.rs … | grep -c` ends green with a last line of 0."
    basis: "examples/seven_guis/src/stand.rs:167-175"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 114
    section: "## 6. Log Coverage"
    change: "- observed absent — a print, a log site, a subscriber or an env read in seven_guis' shared module's `cells_needles(part)`. It returns the in-process Cells screen's author keys for one part and that part's middle column-A cell, and feeds `host_log`'s fourth test, whose messages name a kind and a count and never a needle · searched: the chunk's silence census, reading 0 (examples/seven_guis/tests/common/mod.rs:425-454; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its obs acceptance)"
    sidecar: "log-coverage: seven_guis' cells_needles is silent and names no needle (2026-10-25-cold-agent-bar, F8)"
    rationale: "The report's Symbols / APIs give common::cells_needles at common/mod.rs:425-454. Its obs acceptance (silence census 0; host_log's Cells part read 0 needles) is MET."
    basis: "examples/seven_guis/tests/common/mod.rs:425-454"
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 11
    section: "## 8. PII Scrubbing & Compliance"
    change: "Replace the clause «a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)» with: a paged part read through the host's own sink, unmeasured at that chunk, is measured since 2026-10-25-cold-agent-bar on `serve cells` (below). That discharges its residual line."
    sidecar: "pii-scrubbing-compliance#1.1: the host-sink paged part read is no longer unmeasured, and the four-lean-tasks premise is gone (2026-10-25-cold-agent-bar)"
    rationale: "The report's Spec claims disproved by measurement: «the host boots only the four lean tasks and none pages» is false since `serve cells`, and pii-scrubbing-compliance.md:11 is named. inputs#I4 row 1 discharges residuals.md:38."
    basis: ".andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 11
    section: "## 8. PII Scrubbing & Compliance"
    change: " A paged part read through the `escher-session` host's own sink is measured since 2026-10-25-cold-agent-bar. `host_log`'s fourth test (`a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`, unix only) runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and calls `snapshot --part 2`. It reads 0 id and 0 name needles under both builds, and its failures name a kind and a count only (`host_log` 4). The residual for the read is discharged (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, F13)."
    sidecar: "pii-scrubbing-compliance: the host-sink paged part read on served Cells, 0 id and name needles under both builds (2026-10-25-cold-agent-bar, F1)"
    rationale: "The report's F13 and obs acceptance: host_log's Cells part read reads 0 id and name needles under both builds. Its Deviations say the read commits a sentinel first."
    basis: "examples/seven_guis/tests/host_log.rs:134-277"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 32
    section: "## 8. PII Scrubbing & Compliance"
    change: " Since 2026-10-25-cold-agent-bar the founder's one live run of that day (6 tasks, 66 calls, 0 wrong) carried the stand's new screens to the model provider in `transcript.jsonl`: Cells' contents, the select's options and value, a lint's findings and a shot. He ruled it on «Запускать сейчас (Рекомендую)», within «Один запуск, до $3 (Рекомендую)». `transcript.jsonl` is still never printed, and the committed transcript is host-path-masked (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, F15)."
    sidecar: "pii-scrubbing-compliance: the 2026-10-25 live run's carriage of the new screens to the model provider (2026-10-25-cold-agent-bar, F10)"
    rationale: "The report's F15 lists pii-scrubbing-compliance.md:32 as stale for live-run: 6 tasks, 66 calls, 0 wrong, on inputs#I6 within inputs#I3. Its obs acceptance says the transcript is never printed, and the evidence copies a masked live-transcript.jsonl."
    basis: ".andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32"
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "Keep the row; re-read its three citations at apply, because each cited file gained lines before the cited span: `tests/blitz-tests/tests/session_common/mod.rs:333-341` (305-306, 309-313, 316 and 318 added), `examples/seven_guis/tests/common/mod.rs:17-25` (6-7 added) and `examples/seven_guis/tests/host_log.rs:303` (15-17 and 134-278 added). Add to the seven_guis directory list the state directory `host_log`'s fourth test names, as read at apply."
    sidecar: "ci-integration#1.6: the session-state row's three citations re-read in their shifted files (2026-10-25-cold-agent-bar)"
    rationale: "The report's F13 lists obs ci-integration.md:15 as a host_log.rs:N citation. New text lists the added ranges in session_common/mod.rs, seven_guis common/mod.rs and host_log.rs."
    basis: "examples/seven_guis/tests/host_log.rs:134-277"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "Change «the checks write theirs under `target/tmp/`» to «the checks write theirs under `target/tmp/`. Since 2026-10-25-cold-agent-bar the cold-agent pipe's live `escher-session mcp --record` child writes its own under `target/cold-agent/record/` (Cold-agent pipe state, below)»."
    sidecar: "ci-integration#1.7: the recording directory's second writer, the cold-agent live child under target/cold-agent/record/ (2026-10-25-cold-agent-bar)"
    rationale: "The report's Symbols / APIs: the live server child runs mcp --record into target/cold-agent/record (mkdir -m 700). Its Schema / config adds that directory to the pipe's owner-only state set."
    basis: "scripts/cold-agent.sh:257-271"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 17
    section: "## 9. CI Integration"
    change: "Add `record/` to the state set, making it `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` (the next row). Re-read its citations `scripts/cold-agent.sh:17`, `:555-588` and `:593-596` at apply, because the file grew by 127 lines in 25 ranges, 3-7 and 10-12 before line 17."
    sidecar: "ci-integration#1.8: the pipe state set gains record/; cold-agent.sh citations re-read (2026-10-25-cold-agent-bar)"
    rationale: "The report's Schema / config gives the pipe's state set (target/cold-agent/record/, owner-only, cleanup removing it with the state). New text lists cold-agent.sh as added 127 line(s) in 25 range(s)."
    basis: "scripts/cold-agent.sh:257-271"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 17
    section: "## 9. CI Integration"
    change: "| Cold-agent recording | `target/cold-agent/record/`: made `mkdir -m 700` by the pipe's live run, whose `escher-session mcp --record` child keeps one check per accepted `start` there (a recording directory, above); the replay children run without `--record`. The kept checks are never printed or copied and are removed by `cold-agent.sh cleanup` with the rest of the state. Local only, uploaded by no CI leg. No kept check of the live run is committed, and the two committed Counter checks (below) stay the only committed ones | (scripts/cold-agent.sh:257-271; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md) |"
    sidecar: "ci-integration: target/cold-agent/record/ is local state no leg uploads, with no live-run check committed (2026-10-25-cold-agent-bar, F9)"
    rationale: "The report's security acceptance: no kept check committed, and target/cold-agent/record/ is uncommitted. Its Harness / gate surface says no CI step changes. Its Schema / config gives the owner-only record directory that cleanup removes."
    basis: "scripts/cold-agent.sh:257-271"
    fact: F9
note: "No pii fact covers the live run's kept checks under target/cold-agent/record/, which hold the screens' readings at rest (each answer kept whole). The new-detail pass may want a line for them beside pii-scrubbing-compliance#4.1."
