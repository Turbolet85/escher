@amendment marker=2026-11-07-ui-graph
title: the harness contract takes the graph verb and its never-logged answer
section: §3 Observability Harness Contract (body: the session library's instrumented verbs and `Outcome` writings; the session shared module's readers)
change:
  - `Session::run`'s instrumented verbs run on past the eighth, `lint`, to a ninth, `graph`, under the same one `command` span (was: the list stopped at `lint`).
  - `Outcome::Graphed { graph }` joins the never-logged answers: written by `to_json` alone as `{"nodes":[…],"states":[…]}` (ids, component names, registered state and handler event names, closed flag words and booleans, never a value), a command's answer, over MCP its JSON line, kept whole in a recorded check, never logged or fielded.
  - The checks reading `tests/blitz-tests/tests/session_common/mod.rs` were thirty-two; now thirty-three, `stand_graph` among them.
why: the report adds `Command::Graph` and `Outcome::Graphed`, whose `command` span records `verb` alone, and states the graph verb instrumented with the new files holding no log site and escher-driver keeping its 2 `info_span!` sites; `stand_graph` is a new check declaring the shared module.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph answer's carried set, ratified
section: §9 CI Integration (ci-integration.md: the Recording directory row); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: a new item on the graph answer)
change:
  - The Recording directory row's content at rest was the screens' ids, names, values, snapshot text, diffs and a lint's findings; now also a `graph` answer's ids, component names, registered state names, handler event names, closed flag words and booleans.
  - A new §8 item: the UI graph's answer (`graph`, `Outcome::Graphed`) carries stable ids, component names, registered state names, handler event names, the closed flag words `undeclared` and `unconfirmed`, and booleans only, never a state's value (a sentinel typed into `crud-name` occurs 0 times in it), a boundary widening, ratified; its log reading stands in the row above.
why: the report states the answer carries names, closed flag words and booleans only, the sentinel 0 times, and that the graph answers in a kept check, the recording routing every verb through `Outcome::to_json`. A boundary widening, ratified by the founder's P4 fork 1 answer (a new `graph` verb), by dialog in the overseer session, relayed by the operator through the P4 dialog (inputs#I4), with his P5 answer on the authored write declarations, relayed by the operator (inputs#I5); the operator relayed at this resume that the widening has those two answers (inputs#I9). The confirm that these proposals lie inside them is the overseer's, by question dialog, PROVISIONAL (delegate the overseer), for the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the cold-agent pipe's verbs take graph
section: §6 Log Coverage (log-coverage.md: a new item after the verdict fields)
change:
  - A new item: the pipe's `VERBS` (`scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`) hold thirteen words, `graph` among them, and the contract tests' fake server answers `graph` with `{"nodes":[],"states":[]}`; `graph` is no counted verb and is not required of a cold agent, so it adds no verdict field and no `capability-unused` reason.
why: the report's harness surface states both `VERBS` gain `graph`, the fake server's empty answer, and the `capability-unused` set unchanged, `graph` not required of a cold agent.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph verb's command and tool_call lines
section: §6 Log Coverage (log-coverage.md: escher-driver's `command` and `tool_call` lines); §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the in-process and standing-check readings); §4 Span / Trace Coverage (span-trace-coverage.md: the `command` span's verb-alone readings, the `tool_call` span)
change:
  - The `command` line's verb table was twelve words, eight instance-level; now thirteen, nine instance-level. A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name; `host_log` and `mcp_log` each drive one `graph` call at `trace`, 0 id, name and graph needles under both builds.
  - The `tool_call` span's `tool` word, in §6 and §4, was one of the table's twelve; now one of its thirteen, `lint` and `graph`.
  - §8: the verb table `stand_act_spans` reads was eight; now nine with `graph`, one call per layout mode, its `command` line `verb` alone; `host_log` and `mcp_log` each make one `graph` call, 0 id, name and graph needles (`graph_needles`) at `trace` under both builds.
  - §4: an accepted `graph` (`Outcome::Graphed`, no step run) records `verb` alone, a graph's nodes, states, ids or names never fielded; a `graph` tool call ends `accepted` (one text block), an argument refused `malformed`, and parents exactly one `command` child recording `verb` alone, in process in both layout modes; no field added.
why: the report moves `VERBS` 12 to 13 and instance verbs 8 to 9, and its obs outcome states `host_log` and `mcp_log` read 0 needles with one graph call each and the `graph` `tool_call` parents one `command` recording `verb` alone (`stand_mcp_tools`; `stand_act_spans` one call per mode).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: dioxus-native-dom's graph modules and bridge dispatch add no log site
section: §6 Log Coverage (log-coverage.md: new groups **dioxus-native-dom (UI graph)** and **dioxus-native-dom (bridge dispatch)**)
change:
  - New group (UI graph): the modules `graph` (gated `accessibility`) and `signal_registry` and the check `stand_graph` hold no print, log site, subscriber or env read, the silence census reading 0 over them; `Outcome::Graphed` is written by `to_json` alone, never logged or fielded, escher-driver keeping its 2 `info_span!` sites.
  - New group (bridge dispatch): `DioxusEventHandler::handle_event` sets the `SignalRegistry`'s dispatch around `runtime.handle_event` and clears it after only where a registry context stands at the root scope, adding no log site; a page with no registered state reaches no registry and dispatches as before.
why: the report adds the two modules and the check, states the new files hold no log site with the registry's silence census green, and states the bridge dispatch change inert without a registry, its instrumentation n/a.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the snapshot's clickable reading carries no page text
section: §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: a new item)
change:
  - A new item: the snapshot's `clickable` reading (`NodeState.clickable`, the text token `clickable` after `focused`, a written node's `"clickable"` key) is a closed bool, true where an element's current listeners hold `click` and it is no `ElementData::is_disabled` form control, carrying no page text.
why: the report adds `NodeState.clickable` after `focused`, its text token and its written key, and states its PII n/a as a closed bool.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the state registry holds no state value
section: §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: a new item)
change:
  - A new item: `SignalRegistry` (dioxus-native-dom, behind `use_graph_state`) holds in process memory each registered state's name, its declarations, its write records and the standing dispatch chain, and no value of a state and no `NodeId` beyond that chain; its `Debug` prints counts alone, and it holds no log site, print, env read or `unwrap` on app input.
why: the report states the registry's `Debug` prints counts alone, that it holds no `NodeId` beyond the dispatch chain and no log site, print, env read or `unwrap` on app input, and its PII n/a, no value held.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
