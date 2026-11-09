@amendment marker=2026-11-08-handler-coverage
title: the cited line ranges follow the moved code
section: §1 Obs Scope Summary (body); §3 Observability Harness Contract (body); §4 Span / Trace Coverage (span-trace-coverage.md); §6 Log Coverage (log-coverage.md); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md); §9 CI Integration (ci-integration.md: the session state directory row)
change:
  - The rows citing `escher-session`'s module (`session_host.rs`), escher-driver's `cli.rs` and `json.rs`, seven_guis' `host_log.rs`, `cli_commands.rs` and shared `common` module, and dioxus-native-dom's `dioxus_document.rs` now cite the line ranges those files hold after the chunk's additions; no claim of those rows moved.
why: the report's Changes and New text, by line: the chunk added lines to each of those files, moving the code the rows cite.
kept: §9's session state directory row keeps its enumerated `host_log` directories (`hl-trace`, `hl-client`, `hl-client-rec`, `hl-cells`, `hl-popup`): the report names no state directory of the coverage run, so a claim that the list leaves one out would be inferred, not measured.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: escher-session's coverage role, a further reach of the same sink install
section: §3 Observability Harness Contract (body: the `escher-session` install row)
change:
  - The `escher-session` row now states the reader taking the first word `coverage` as a further role, `coverage <app> <check>...` with no `--session` and no `--record`: `handler_coverage` and dioxus-native-dom's reached record install no subscriber, read no environment variable or clock and add no log site, span or field; each replayed step of a kept check leaves its own `command` span through `Session::run`, escher-driver keeping its 2 `info_span!` sites; the coverage line (ids, owner ids, event names and two counts) is written on stdout by the crate's hand writer as an answer, never a log line; a `malformed` refusal holds nothing of the path; the role a further reach of the same install in the same `main`, measured at the chunk's report.
why: the report's Symbols / APIs (the `coverage` role on `command_line`) and Coverage of new surfaces (instrumentation n/a, `info_span!` at 2, the line's content). The role is a boundary widening ratified by the founder's own word at P4 Q1, relayed verbatim by the overseer through the operator (inputs#I4); this edit, among the 29 escalated at this wrap's Q1, was confirmed by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the session_common module's readers gain stand_handler_coverage
section: §3 Observability Harness Contract (body: the silent-checks row)
change:
  - The `session_common` module's readers: was thirty-four; now thirty-five, the list gaining `stand_handler_coverage` after `stand_graph` and `stand_actions`.
why: the report's Counts: the `^mod session_common;` declarers moved 34 to 35 with the new check `stand_handler_coverage`.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: host_log's paged-part test is its fifth
section: §3 Observability Harness Contract (body: the session-check spawns row); §4 Span / Trace Coverage (span-trace-coverage.md: the hosted part read); §6 Log Coverage (log-coverage.md: the `command` span line); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the recording-session paragraph)
change:
  - `host_log`'s test reading a paged part of a served Cells screen through the host's own sink (`serve cells`, `snapshot --part 2`): was its fourth test (`host_log` 4); now its fifth (`host_log` 5), in each of the four sections.
why: the report's Counts: seven_guis `host_log` tests moved 5 to 6, the coverage run's test added ahead of the paged-part test, so the per-test ordinals the sections state moved by one.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: escher-session's endings gain the coverage role
section: §6 Log Coverage (log-coverage.md: a new item, examples (`escher-session`'s `coverage` role))
change:
  - New item: `escher-session coverage <app> <check>...` — an app the binary boots and one or more paths, no `--session` or `--record` word among them, any other shape usage (status 2), the usage line holding `; coverage role: coverage <app> <check>...;` before `; apps:` — reads each named kept check through a bounded read of 1,048,576 + 1 bytes and `Check::from_bytes`: accepted, one uncoloured JSON coverage line on stdout (status 0); a boot that fails, the session error (status 3); a file that cannot be read, one past the bound, one that is no check or a check of another app, refused `malformed` (status 1), holding nothing of the path or the file.
why: the report's Symbols / APIs (the role's shape, bounded read, refusals, endings and usage clause). The role is a boundary widening ratified by the founder's own word at P4 Q1, relayed verbatim by the overseer through the operator (inputs#I4); this edit, among the 29 escalated at this wrap's Q1, was confirmed by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the command span's reach gains the coverage role
section: §6 Log Coverage (log-coverage.md: the `command` span line's emitters); §4 Span / Trace Coverage (span-trace-coverage.md: a new handler-coverage entity); §2 Telemetry Strategy (telemetry-strategy.md: the command span's reach)
change:
  - §6: the `command` span line's emitters now include the `escher-session coverage` role, each replayed step of a kept check run through `Session::run` writing its own `command` line, the coverage line itself never logged or fielded.
  - §4: new entity — handler coverage adds no span, field, event or log line: the `coverage` role, `handler_coverage`, `Coverage::to_json`, the crate-private `Check::replay_each` and dioxus-native-dom's reached record (`record_reached`, `reached_handlers`, the crate-private `ReachedRecord`, read through the crate-private `element_listener_ids`) open no span; escher-driver keeps its 2 `info_span!` sites; `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` reads the binary at `trace` under both builds with 0 needles.
  - §2: the command span is now also reached in `escher-session` by its `coverage` role, each replayed step writing its own closed `command` span, the role adding no span.
why: the report's Coverage of new surfaces (instrumentation n/a; each replayed step leaves its own `command` span) and its (obs) Outcome (`info_span!` last line 2). The role is the founder's ratified boundary widening at P4 Q1, relayed verbatim by the overseer through the operator (inputs#I4); the §2 and §6 edits, among the 29 escalated at this wrap's Q1, were confirmed by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the coverage module, the reached record and their check log nothing
section: §6 Log Coverage (log-coverage.md: new items escher-driver (handler coverage) and dioxus-native-dom (reached record); a new observed-absent row)
change:
  - New item: escher-driver's module `coverage` (`handler_coverage`, `Coverage`, `HandlerRef`) holds no span, event, env read, clock read or print; `Coverage::to_json` writes `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}` by the crate's hand writer as a command line answer, `owner` left out when absent and `serde_json` writing nothing, never logged or fielded.
  - New item: dioxus-native-dom's ungated module `reached` (`ReachedHandler`, `record_reached`, `reached_handlers`, the crate-private `ReachedRecord`, a root-scope context), written in `DioxusEventHandler::handle_event` beside the `SignalRegistry` arm and read through the crate-private `element_listener_ids`, holds no log site, print or env read; the record holds stable ids and event names only, no `NodeId`, its `Debug` printing its count alone.
  - New observed-absent row: no print, env read, subscriber or log site in `stand_handler_coverage` (both layout modes), the `coverage` module or the `reached` module — the silence census over the three files, last line 0.
why: the report's Symbols / APIs and Coverage of new surfaces (instrumentation n/a for `handler_coverage` and the record; no `NodeId`) and its (obs) Outcome (the silence census last line 0, `info_span!` last line 2).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: host_log's coverage run reads 0 needles at trace
section: §6 Log Coverage (log-coverage.md: a new observed-absent row; the shared module's needle readers); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the recording-session paragraph)
change:
  - §6: new observed-absent row — no handler id, name, coverage line, typed sentinel, path or kept check's contents in the `escher-session` binary's log: `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` runs `coverage crud` over a CRUD session kept through the binary, with a typed sentinel and a row click, reading 0 id, name, coverage, sentinel, path and recorded needles at `trace`, green under the package and workspace builds.
  - §6: the seven_guis shared module's needle readers now include `coverage_needles`, the CRUD screen's UI-graph ids, for `host_log`'s coverage run alone.
  - §8: the paragraph now states `host_log`'s one `coverage crud <check>` run and its 0-needle reading at `trace` under both builds.
why: the report's (obs) Outcome (the test green under the package and workspace builds), Coverage of new surfaces (the needles read) and Deviations (`coverage_needles` reads the CRUD screen, a screen holding names and a typed field).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the coverage line's carried fields
section: §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: a new row after the actions answer)
change:
  - New row: the coverage line (`escher-session coverage <app> <check>...`, `Coverage::to_json`) carries stable ids, owner ids, handler event names and two counts only, written by the crate's hand writer alone, `owner` left out when absent and `serde_json` writing nothing, never a value or typed text — a typed sentinel occurs 0 times in it (`stand_handler_coverage`) — a boundary widening, ratified; a command line answer, never logged or fielded.
why: the report's Symbols / APIs (`Coverage::to_json`'s form), Coverage of new surfaces (PII redacted) and (security) Outcome (sentinel 0). The widening is ratified by the founder's own word at P4 Q1, relayed verbatim by the overseer through the operator (inputs#I4); this row, among the 29 escalated at this wrap's Q1, was confirmed by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
