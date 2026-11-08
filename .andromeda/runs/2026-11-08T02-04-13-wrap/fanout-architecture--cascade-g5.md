@proposal n=1 detector=D-arch-resources severity=escalate file=occupied-resources anchor=fct-crxfkvwh basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qyqo7z6u
section: ## Occupied Resources
change: after the lint sentence, add: Since 2026-11-07-ui-graph both carriers, and a kept check, also take the screen's UI graph out as the `graph` verb's answer — stable ids, component names, handler event names, registered state names, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`. `graph` takes no argument; one is refused `malformed` with nothing run. A snapshot node's `clickable`, a closed bool, crosses in the snapshot and the diff. This is a boundary widening ratified by the founder (2026-11-07, «Новый глагол graph (Рекомендую)», by dialog, relayed). There is still the one listener and no port, the wire's bounds are unchanged, and the Cells answer is 293,869 bytes under the 1,048,576 answer bound (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).
sidecar: occupied-resources network ports and listeners — the graph verb's answer added to what both carriers and a kept check take out, a ratified widening
rationale: the report's Expected amendments name the graph's names crossing the socket, the command line, MCP and kept checks as a ratified widening (inputs#I4), held on one Validate card (inputs#I9); Coverage shows names only and the sentinel absent
@proposal n=2 detector=D-cascade severity=warning file=occupied-resources anchor=fct-drud2scu basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## Occupied Resources
change: after the mcp_log clause, add: since 2026-11-07-ui-graph a `graph` call's `command` span records `verb` alone, its `tool_call` parenting it, and `host_log` and `mcp_log` each make one graph call and read 0 id, name and graph needles (component and state names) at `trace` under both builds; dioxus-native-dom's `SignalRegistry` is a per-document Dioxus root context, not process-wide state, and spawns no thread
sidecar: occupied-resources process-wide state — the graph call's span and its zero graph needles in host_log and mcp_log recorded
rationale: Coverage of new surfaces states the command span records verb alone and host_log and mcp_log read 0 id, name and graph needles at trace in both builds; the registry is a root context (Symbols / APIs)
@proposal n=3 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-dw3hon6z basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-z2vdklsi
section: ## Cross-cutting Patterns
change: the `accessibility` clause reads: gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and, since 2026-11-07-ui-graph, `graph` modules (`DioxusDocument::graph` among them), its `signal_registry` module ungated
sidecar: cross-cutting config management — dioxus-native-dom's graph module named among the accessibility-gated modules, signal_registry ungated
rationale: Crates / modules: dioxus-native-dom gains graph (gated accessibility) and signal_registry (not gated)
@proposal n=4 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-dsg4uyo4 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Cross-cutting Patterns
change: after the dioxus root-context list, add: since 2026-11-07-ui-graph the first `use_graph_state` hook provides a `SignalRegistry` context at `ScopeId::ROOT`, which the bridge's dispatch looks up with `runtime.has_context`; a page that registers no state has none
sidecar: cross-cutting service wiring — the SignalRegistry root context added
rationale: Symbols / APIs: the first registering hook provides a SignalRegistry context at ScopeId::ROOT; the bridge dispatch checks runtime.has_context::<SignalRegistry>(ROOT)
@proposal n=5 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-klon5sda basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Cross-cutting Patterns
change: after the `mounted` clause, add: since 2026-11-07-ui-graph `DioxusEventHandler::handle_event` sets the registry's dispatch (chain head first, its event name) around `runtime.handle_event` and clears it after where a `SignalRegistry` stands at the root, so a signal write is recorded against the dispatch that caused it; a page with no registered state dispatches as before, and the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched
sidecar: cross-cutting event processing — the bridge dispatch bracketed by the signal registry's dispatch
rationale: Symbols / APIs, the bridge dispatch: a change on every Dioxus document's event path, inert without a registry
@proposal n=6 detector=D-new-detail severity=warning file=cross-cutting-patterns anchor=fct-klon5sda fact=F9 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-igaawlyp
section: ## Cross-cutting Patterns
change: add: in Dioxus 0.7.10 an async event handler's spawned task runs inside the dispatch that started it, so its signal writes are recorded against that event; dioxus-signals keeps a reactive context subscribed while `mark_dirty` returns true, so the registry's watcher needs no re-subscribe
sidecar: cross-cutting event processing — async handler writes measured to land inside their dispatch, and no re-subscribe owed
rationale: Decisions & corrections (stand_graph's first red on Circle Drawer's canvas writes) and Spec claims disproved (dioxus-signals 0.7.10 update_subscribers keeps a context whose mark_dirty returns true)
@proposal n=7 detector=D-cascade severity=warning file=project-intent anchor=fct-ldsoar27 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: ## Project Intent
change: «the page saying twelve, the count `VERBS` holds» reads «the page saying thirteen, the count `VERBS` holds»
sidecar: project-intent front page — the README's count of driver commands as tools, twelve to thirteen
rationale: VERBS moved from 12 to 13; the capability pin reads «the thirteen driver commands as tools» in README.md, and Counts moved marks project-intent.md:29 × verbs stale
@remark
F1–F5 (graph, registry and graph verb contracts) belong to standard-contracts, F10 to conventions, and the body's config line (fct-nmnb5xux) names only the snapshot modules as accessibility-gated, so the orchestrator may want to widen it to name graph as well.
