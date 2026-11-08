@amendment marker=2026-11-07-ui-graph
title: the UI graph joins what the carriers take out
section: Threat Model Summary (body: the session socket and MCP stdio vectors; threat-model-summary.md: boundary-session-socket, boundary-mcp-stdio, the capability statement line); Input Validation (body: the driver session socket line)
change:
  - The socket vector's, the MCP stdio vector's and the socket line's carried sets gain a UI graph's ids, component, state and handler event names, closed flag words and booleans (was: ending at a lint's findings).
  - boundary-session-socket gains a line: an answer's `graph` takes the screen's UI graph out on the socket, the command line, MCP and a kept check — ids, owner ids, component names, registered state names, handler event names (without `on`), the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or `NodeId` — a boundary widening, ratified.
  - boundary-mcp-stdio gains a line: a `graph` result carries the UI graph to the client as the text block of its JSON line; what a client does with it, a model provider included, is the client's.
  - The `docs/capabilities.md` line's carried-onward set gains a UI graph (was: ending at a lint's findings).
why: the report's security outcome — the answer carries ids, component, handler event and state names, closed flag words and booleans only, a typed sentinel 0 times. A boundary widening ratified by the founder's P4 fork 1 (by question dialog in the overseer session, relayed by the operator, 2026-11-07) and his P5 answer on the authored write declarations (relayed by the operator, 2026-11-08); that these lines lie inside his words is the overseer's confirm of the answer card, provisional for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the thirteenth verb, graph, in the schema, MCP, the settle and the refusals
section: Input Validation (body: the driver command schema line); API Security (api-security.md: the no-served-API row, the settle row); Error Handling (error-handling.md: the refusal row)
change:
  - The command schema's closed table holds thirteen verbs (was twelve).
  - The `escher-session mcp` role serves thirteen MCP tools (was twelve), the thirteenth `graph`, listed after `lint`, taking no argument and answered as one text block.
  - The command schema has nine instance-level verbs (was eight), the ninth `graph`, taking no argument, answering `nodes` and `states`, running no step and settling nothing, so it moves no time; `snapshot`, `screenshot`, `lint` and `graph` wait on no load (was: the first three).
  - A `graph` call with an argument is refused `malformed` with nothing run, no cause added, the set staying eight.
why: the report's Schema / config (`VERBS` 12 to 13, instance verbs 8 to 9) and Symbols / APIs (`schema::GRAPH` after `LINT`, no argument, `Outcome::Graphed` running no step; the MCP listing routing `graph` after `lint`, its answer one text block), and its Coverage of new surfaces (an argument refused `malformed` with nothing run).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the Dioxus bridge's dispatch and the signal registry
section: Input Validation (body: a Dioxus bridge line; input-validation.md: two Dioxus bridge rows, the event dispatch and the signal registry)
change:
  - The body gains a Dioxus bridge line: on every Dioxus document the dispatch sets a registered `SignalRegistry`'s dispatch around `runtime.handle_event` and clears it after, inert where no state is registered; `use_graph_state` takes dioxus-core types alone, holds no value of a state, has no log site, print, env read or `unwrap` on app input; no input class enters from outside the process.
  - The event dispatch row (`DioxusEventHandler::handle_event`): set only where `runtime.has_context::<SignalRegistry>(ROOT)` holds; a page with no registered state dispatches as before, the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` untouched; an engine change for every Dioxus document, inert without a registry.
  - The signal registry row (`use_graph_state`, `SignalRegistry`, `DeclaredWrite`): its first registering hook provides the context at `ScopeId::ROOT`; a `DeclaredWrite` holds a stable id or a prefix ending in `*` and an event without `on`; each entry leaves with its scope (`use_drop`); its `Debug` prints counts alone; it holds no `NodeId` beyond the dispatch chain.
why: the report's Symbols / APIs (the registry and the bridge dispatch, inert without a registry) and Coverage of new surfaces (an in-process app API, no value held; the dispatch pinned by its bridge test).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the cited lines the chunk's edits moved
section: Data Protection (body: the file-input path line); Secret Management (body: the two env lines); Error Handling (error-handling.md: the mounted-handle, removed-element and refusal rows); Input Validation (input-validation.md: the rows citing the moved files); Logging & Monitoring (logging-monitoring.md: the sink-by-level and select-list rows)
change:
  - Each citation into a file this chunk edited is re-read to the lines it now names — dioxus-native-dom (`dioxus_document.rs`, `snapshot.rs`, `snapshot_diff.rs`, `element_id.rs`, `lib.rs`), escher-driver (`schema.rs`, `json.rs`, `command.rs`, `execute.rs`, `mcp_tests.rs`), seven_guis (the tasks `flight_booker`, `cells`, `circle_drawer`, `crud`; the tests `cli_commands`, `host_log`, `mcp_log`, `common`) and blitz-tests (`session_common`, `stand_act_spans`); no claim those rows make moved.
why: the report's Files — the chunk's edits to those files moved the lines the rows cite.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph answer's size under the answer bound
section: API Security (api-security.md: a control, the `graph` answer's size)
change:
  - A new control: `graph` takes no argument and its answer is unpaged — no part argument, which would widen the crossing — so a screen's whole graph is one answer under the 1,048,576-byte bound; Cells' answer reads 293,869 bytes, held within the bound in both layout modes by `stand_graph`; its size the stated limit «An unpaged graph» in `docs/capabilities.md`, owned by residual `res-7cc5bi4k` (target: next).
why: the report's Insufficient fixes — the readers narrowing kept but not the remedy, the writers narrowing bringing the answer to 293,869 bytes, the remainder owned by `res-7cc5bi4k`; the operator's corrections in the implement session (no argument and no new field; the remaining size a stated limit and a residual row, target next), and the operator's relay of the overseer at this resume that the size stays a stated limit with its residual row.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: a recorded graph answer at rest
section: Data Protection (data-protection.md: the kept checks row)
change:
  - The screens' readings a kept check holds at rest gain a `graph` answer's UI graph — ids, owner ids, component names, registered state names, handler event names, the closed flag words and booleans, never a control value or typed text; a boundary widening, ratified (was: ending at a lint's findings).
why: the report's Schema / config (`escher-session` answers `graph` in a kept check) and its security outcome (names, flag words and booleans only). A boundary widening ratified by the founder's P4 fork 1 (by question dialog in the overseer session, relayed by the operator, 2026-11-07); that it lies inside his words is the overseer's confirm of the answer card, provisional for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the snapshot's clickable reading
section: Error Handling (error-handling.md: the refusal row); Input Validation (input-validation.md: a new socket row, the socket row, the command schema row, the disabled row); Threat Model Summary (threat-model-summary.md: the select-list line)
change:
  - The help of `changed` and of `snapshot`'s `text` name `clickable`, each still a fixed literal.
  - A new socket row: `NodeState.clickable`, a closed `bool` after `focused`, true where the element's current listeners hold `click` and it is no form control `ElementData::is_disabled` reads disabled; the snapshot's text writes the bare word `clickable` only when true, a diff node writes it always, and `differs` can name it (`Reading::Clickable`); never an attribute's value or a listener's code.
  - `NODE_FIELDS` reads fourteen rows, `clickable` after `focused` (was thirteen), in the socket row and the select-list line.
  - A changed node's `differs` is a closed twelve-variant enum, `Reading::Clickable` among them (was eleven).
  - The disabled row: the snapshot's `NodeState.clickable` is a further reader of the presence reading.
why: the report's Symbols / APIs (`NodeState.clickable` and its writing) and Schema / config (`NODE_FIELDS` 13 to 14, `Reading::ALL` 11 to 12). A closed value inside the ratified carrier, so no widening under the founder's rule of 2026-10-10 (by question dialog in the overseer session, relayed by the overseer).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph verb on the validated surface
section: Input Validation (input-validation.md: the command schema row, a new `graph` row, the session socket row, the command line row, the MCP server row, the stable id row)
change:
  - The schema's table holds thirteen verbs in three levels (was twelve), a ninth instance verb `graph`.
  - A new row: `schema::GRAPH` after `LINT`, `("graph", [])` read as `Command::Graph`, any argument refused `malformed` with nothing run on every carrier; fields `nodes` and `states`, the eleventh and twelfth field kinds; it runs no step, feeds the session's record of ids and is written by `to_json` alone; the answer holds names, the flag words `undeclared` · `unconfirmed` and booleans, never a value, typed text or `NodeId`; its span records `verb` alone.
  - The socket row's answer carries the UI graph under the unchanged 1,048,576-byte bound.
  - The command line answers `graph` as one line of JSON, status 0, an argument refused `malformed`, status 1; the usage line lists `graph` after `lint`.
  - `tools/list` holds thirteen verbs (was twelve), `graph` after `lint`, its result one text block, its `tool_call` span parenting one `command` span.
  - `VERBS` holds thirteen (was twelve); `DioxusDocument::graph` is a further, read-only reader of the id, holding no `NodeId`.
why: the report's Symbols / APIs and Coverage of new surfaces. A boundary widening ratified by the founder's P4 fork 1 (by question dialog in the overseer session, relayed by the operator, 2026-11-07); that these rows lie inside his words is the overseer's confirm of the answer card, provisional for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the verb counts the checks read
section: Input Validation (input-validation.md: the cold-agent check row and a new cold-agent row, the recording directory row, the recorded check reader row)
change:
  - The cold-agent check's `valid_step` takes `tool` one of the thirteen verbs (was twelve).
  - A new cold-agent row: `graph` is one of the pipe's thirteen `VERBS`, admitted like any verb and validated again by the server; no task requires it, so the `capability-unused` set is unchanged; the test's fake server answers an empty graph.
  - A call no record can spell is a verb outside the nine instance verbs (was eight).
  - A kept check's record has `tool` one of the nine instance verbs (was eight).
why: the report's Harness / gate surface (the pipe's `VERBS` gain `graph`, the fake server's empty graph, `capability-unused` unchanged) and Counts / qualifiers moved (instance verbs 8 to 9).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the reserved virtual-node id form
section: Input Validation (input-validation.md: a new markup attributes row)
change:
  - A new row: the id grammar reserves a fifth form, `{owner}///{key}` (`VIRTUAL_KEY_SEPARATOR`); the public `virtual_node_id(owner, key)` spells it and returns `None` for an empty or `/`-bearing key; no walk calls it, so no served id holds `///`, and the anchored path stays the only served id holding `//`; a stable-id contract change, ratified.
why: the report's Symbols / APIs and its arch outcome (no served id holds `///`); the form ratified by the founder's P4 fork 2 (by question dialog in the overseer session, relayed by the operator, 2026-11-07).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph verb's silence in the logs
section: Logging & Monitoring (logging-monitoring.md: a new line)
change:
  - A new line: `graph` records `verb` alone on its `command` span, an MCP call of it one `tool_call` line parenting that span; escher-driver holds two `info_span!`, and dioxus-native-dom's `graph` and `signal_registry` modules hold no log site; `host_log` and `mcp_log` each drive one `graph` call with 0 id, name and graph needles at `RUST_LOG=trace` under both builds.
why: the report's obs outcome (0 needles with one graph call each, `tool_call` parenting one `command` recording `verb` alone, 2 `info_span!` sites, no log site in the new files).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
