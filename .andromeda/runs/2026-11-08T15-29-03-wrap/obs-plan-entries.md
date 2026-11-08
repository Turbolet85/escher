@amendment marker=2026-11-08-what-can-i-do-here
title: the stand_actions check is silent, the shared module's thirty-fourth reader
section: §3 Observability Harness Contract (body: the harness's silent checks and their shared modules); §6 Log Coverage (log-coverage: the observed-absent rows)
change:
  - The blitz-tests module `session_common` is read by thirty-four checks, `stand_actions` after `stand_graph` (was thirty-three, `stand_graph` the last).
  - A new observed-absent row: no print, env read, subscriber or log site in the check `stand_actions` (a `session_common` and `fixtures` reader, both layout modes), escher-driver's private module `actions` or dioxus-native-dom's read-only `actionable_nodes`; the silence census reads 0, no span, field, log field or env read is added, and escher-driver keeps its 2 `info_span!` sites.
why: the report's Counts move the `session_common` declarers 33 to 34 with `stand_actions` the thirty-fourth, and its gate entries read the silence census green (last line 0) and `info_span!` at 2.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: cited line ranges re-pointed after the chunk's added lines
section: §3 Observability Harness Contract (body); §9 CI Integration (ci-integration: the Session state directory row); §6 Log Coverage (log-coverage: the escher-driver emitter lines, the select-list absent rows, the `cc-popup` row); §4 Span / Trace Coverage (span-trace-coverage: the `command` and `tool_call` spans, the hosted part read, the select list)
change:
  - Each cited range in `stand_mcp_tools.rs`, `json.rs`, `execute.rs`, `command.rs`, `stand_act_spans.rs`, `session_common/mod.rs`, `host_log.rs`, `mcp_log.rs`, the seven_guis `common/mod.rs` and `cli_commands.rs` now names the lines where its code stands; no claim, count or name in those lines changed.
why: the report's New text by line lists the chunk's added lines in each of those files, which moved the cited code below them.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions answer's carried set, at rest and in the PII boundary
section: §9 CI Integration (ci-integration: the Recording directory row); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance: a new row after the graph answer)
change:
  - The recording directory holds at rest, beside a `graph` answer's words, an `actions` answer's ids, owner ids, the table's verb words, closed cause words and booleans.
  - A new row: the actions answer (`actions`, `Outcome::Actions`) carries ids, owner ids, the table's verb words, closed cause words and booleans only, written by `to_json` alone as `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` with `owner` and `cause` left out when absent, never a value, typed text, mask or `NodeId`; a typed sentinel and `MASKED_VALUE` occur 0 times in it; a boundary widening, `ratified` in the body; a command's answer, kept whole in a recorded check, never logged or fielded.
why: the crossing is a boundary widening on pattern `rul-izni57qv`, ratified by the founder's word «Новый глагол actions (Рекомендую)», given by dialog in the overseer session and relayed verbatim by the overseer (inputs#I4, relayed again as inputs#I6), and by his own confirm at this wrap's halt, «Подтвердить все 24 (Рекомендую)», relayed by the operator (answer-card.md). The report's security outcome reads the sentinel and `MASKED_VALUE` 0 times and no `NodeId` written, and `recording_tests` reads a kept `actions` step's answer back whole.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the cold-agent pipe's VERBS fourteen, actions uncounted
section: §6 Log Coverage (log-coverage: the cold-agent pipe's verbs)
change:
  - The pipe's `VERBS` (`scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`) hold fourteen words, `graph` and `actions` among them, and the fake server answers `actions` too (was thirteen words, `graph` among them).
  - Neither `graph` nor `actions` is a counted verb or required of a cold agent, so neither adds a verdict field or a `capability-unused` reason.
why: the report's Counts: the cold-agent pipe's `VERBS` joins `actions` after `graph`, uncounted [cold-tools].
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: actions joins the command span's verbs and the tool_call span's tools
section: §6 Log Coverage (log-coverage: escher-driver's `command` and `tool_call` lines); §4 Span / Trace Coverage (span-trace-coverage: the `command` span's verb-alone list, the `tool_call` span)
change:
  - The `command` line's verb table: was thirteen words, nine instance-level; now fourteen, ten instance-level. An actions line is `span="command" verb="actions"` (`cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
  - The `tool_call` span's `tool`: was one of thirteen, ending `lint` and `graph`; now one of fourteen, `lint`, `graph` and `actions`; no field added, the span keeps its two.
  - An accepted `actions` (`Outcome::Actions`, no step run; `stand_act_spans` one call per layout mode) records `verb` alone.
  - An `actions` tool call ends `accepted` (one text block, `{"nodes":[…]}`), an argument refused `malformed`, and parents exactly one `command` child recording `verb` alone, in process in both layout modes (`stand_mcp_tools`, through the helper the `graph` test runs).
why: the report's Schema / config (`VERBS` 14, `actions` the tenth instance verb, the span's eight fields unchanged), its Counts [instance], and its obs and security outcomes (`stand_act_spans`, `stand_mcp_tools`, an argument refused `malformed` over MCP).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions read opens no span and adds no site
section: §6 Log Coverage (log-coverage: a new **escher-driver (actions)** entry); §4 Span / Trace Coverage (span-trace-coverage: the observed-absent list)
change:
  - New entry: `Session::actions` reads what each actionable node can take without acting, one snapshot and one `element_ids` map per call, each named call validated then put through the shared `check_target`, adding no `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call; the read changes nothing in the document.
  - The absent list adds `Session::actions` with `check_target` and `target_of`, which the `actions` verb and the four id-taking verbs reach inside the call's `command` span, opening none of their own, and dioxus-native-dom's read-only `actionable_nodes`, which opens no span, emits no event and records no field.
why: the report's arch outcome (the read changes nothing; the diff adds none of those calls in `actions.rs`), its Symbols / APIs (`check_target`, `target_of`) and its Coverage of new surfaces (a read inside `Session::run`'s span; `actionable_nodes` read-only, no log site).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions verb read at 0 needles in process and in the host and mcp logs
section: §6 Log Coverage (log-coverage: the seven_guis shared module's needle helpers); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance: the typed-text and recording-client paragraphs)
change:
  - The seven_guis shared module holds `actions_needles` beside `option_needles`: the CRUD screen's listed author-key ids holding a `-`, for `host_log`'s and `mcp_log`'s `actions` calls.
  - `stand_act_spans`' capture: was 36 calls per mode; now 38, and ten verbs with `actions`, one call per layout mode, its `command` line recording `verb` alone.
  - `host_log` and `mcp_log` each make one `actions` call and read 0 id, name and actions needles at `trace` under both builds.
why: the report's Symbols / APIs (`actions_needles`, its callers `host_log` and `mcp_log`), its Counts (`stand_act_spans`' driven calls to 38 at `run.calls == 38`; the needle helpers [needles]) and its obs outcome (0 needles at `trace` under the package and the workspace build).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
