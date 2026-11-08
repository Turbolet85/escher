@proposal n=1 detector=D-new-detail severity=warning file=span-trace-coverage anchor=fct-zuu6oxcq fact=F2 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## 4. Span / Trace Coverage
change: beside «an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too» add: since 2026-11-07-ui-graph an accepted `graph` (`Outcome::Graphed`, no step run) records `verb` alone, its nodes, states, ids, component, state and event names never fielded; a `graph` tools/call opens a `tool_call` with `tool` `graph` that parents exactly one `command` child recording `verb` alone, measured in process in both layout modes (`stand_mcp_tools`; `stand_act_spans` one `graph` call per layout mode), the span keeping its eight fields.
sidecar: span-trace-coverage — the command span's verb-alone readings gain `graph` (Outcome::Graphed) and its tool_call parenting (2026-11-07-ui-graph).
rationale: the report's Coverage of new surfaces states the graph verb's command span records verb alone and its tool_call parents it (stand_mcp_tools; stand_act_spans one call per mode), and Symbols / APIs states Outcome::Graphed's command span records verb alone.
@proposal n=2 detector=D-cascade severity=warning file=span-trace-coverage anchor=fct-mp6exhtv basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 4. Span / Trace Coverage
change: `tool`, the verb table's word — one of its thirteen, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, `lint`, and `graph` since 2026-11-07-ui-graph — unrecorded for an unknown tool.
sidecar: span-trace-coverage — tool_call's tool word: one of the verb table's thirteen, graph added (2026-11-07-ui-graph).
rationale: the report's Schema / config moves VERBS 12 → 13 with graph after lint, and Counts / qualifiers moved classes every row stating the table's count today stale (pattern verbs).
@proposal n=3 detector=D-cascade severity=warning file=log-coverage anchor=fct-gwvelgmp basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 6. Log Coverage
change: in the command line's sub-bullet, «the verb table's twelve words (eight instance-level; …)» reads «the verb table's thirteen words (nine instance-level; …)», and after the part read's clause add: a graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name — `host_log` and `mcp_log` each drive one `graph` call at `trace`, 0 id, name and graph needles under both builds.
sidecar: log-coverage — escher-driver's command line: thirteen verbs (nine instance-level), the graph line and its 0-needle reading added (2026-11-07-ui-graph).
rationale: VERBS 12 → 13 and instance verbs 8 → 9 (Schema / config; patterns verbs and inst-verbs stale), and the report's (obs) outcome states host_log and mcp_log read 0 needles with one graph call each, the command span recording verb alone.
@proposal n=4 detector=D-cascade severity=warning file=log-coverage anchor=fct-gwvelgmp dependent_of=D-cascade basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 6. Log Coverage
change: in the tool_call line's sub-bullet, «`tool` (the table's word, one of its twelve, … and `lint`)» reads «`tool` (the table's word, one of its thirteen, … `lint`, and `graph` since 2026-11-07-ui-graph)».
sidecar: log-coverage — escher-driver's tool_call line: one of the verb table's thirteen words, graph added (2026-11-07-ui-graph).
rationale: the same VERBS 12 → 13 move as the command line's, a second occurrence in the same entity (pattern verbs, stale).
@proposal n=5 detector=D-new-detail severity=warning file=log-coverage anchor=fct-iekb43j4 fact=F7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: ## 6. Log Coverage
change: add after the verdict fields: since 2026-11-07-ui-graph the pipe's VERBS hold `graph` (thirteen) and its fake server answers `graph` with `{"nodes":[],"states":[]}`; `graph` is not required of a cold agent, so the `capability-unused` set and the counted verbs (`lint` · `screenshot` · `advance`) and the verdict fields are unchanged.
sidecar: log-coverage — cold-agent pipe: graph joins its VERBS and fake server, the verdict fields and counted verbs unchanged (2026-11-07-ui-graph).
rationale: the report's Harness / gate surface states scripts/cold-agent.sh and test_cold_agent.py VERBS gain graph, the fake server answers {"nodes":[],"states":[]}, and the capability-unused set is unchanged.
@proposal n=6 detector=D-new-detail severity=warning file=log-coverage anchor=fct-x7v6upkc fact=F1 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ctk4yyh6
section: ## 6. Log Coverage
change: add the group **dioxus-native-dom (UI graph)** — no site added: dioxus-native-dom's `graph` (gated `accessibility`) and `signal_registry` modules and the `stand_graph` check hold no print, log site, subscriber or env read, the silence census reading 0 over them; `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}`, never logged or fielded, escher-driver keeping its 2 `info_span!` sites (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).
sidecar: log-coverage — dioxus-native-dom's graph and signal_registry and stand_graph: no log site, Outcome::Graphed never logged (2026-11-07-ui-graph).
rationale: the report's (obs) outcome states 2 info_span! sites and the new files hold no log site, Coverage of new surfaces states the registry's silence census entry 11 green, and Symbols / APIs the JSON form of the answer.
@proposal n=7 detector=D-new-detail severity=warning file=log-coverage anchor=fct-x7v6upkc fact=F5 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 6. Log Coverage
change: add under **dioxus-native-dom (UI graph)**: no site added on the bridge dispatch — `DioxusEventHandler::handle_event` sets the `SignalRegistry`'s dispatch around `runtime.handle_event` and clears it after only when a registry context is present at the root scope, and adds no log site; a page with no registered state reaches no registry and dispatches as before (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).
sidecar: log-coverage — dioxus-native-dom's bridge dispatch: the registry's dispatch set around handle_event, no log site added (2026-11-07-ui-graph).
rationale: the report's Symbols / APIs states the bridge dispatch change, inert without a registry, and Coverage of new surfaces marks its instrumentation n/a.
@remark
F3, F4 and F6 (slug pii-scrubbing-compliance) are not this group's; no detector invariant is violated (graph instrumented, no dependency, no logged user data); the option_needles citation moved by graph_needles' insert is the citation sweep's; body broad lines fct-pzih6tk6 and fct-wa4xq2qh state no verb count and stay true.
