@proposal n=1 detector=D-arch-resources severity=warning file=occupied-resources anchor=fct-drud2scu basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Occupied Resources
change: append — since 2026-11-08-handler-coverage the binary's `coverage` role (`coverage <app> <check>...`) is one process with no listener and no thread of its own, booting one fresh session in process per named check through the same boot closure the host uses, each replayed step running through `Session::run` under the binary's `escher_telemetry::init` sink, another role reaching that one install site.
sidecar: occupied-resources#8.1 gains the escher-session coverage role's process shape (2026-11-08-handler-coverage).
rationale: the report's Symbols / APIs add the coverage role, its boot closure called once per check, and Coverage of new surfaces states each replayed step leaves its own command span through Session::run.
@proposal n=2 detector=D-cascade severity=warning file=body anchor=fct-k6jtbppj dependent_of=D-arch-resources basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Occupied Resources
change: «…its `mcp` role at most one session in process» reads «…its `mcp` role at most one session in process, its `coverage` role one fresh session in process per named check».
sidecar: body Process-wide state line names the coverage role's one session per check (2026-11-08-handler-coverage).
rationale: the broad line the group details lists the binary's roles; the report adds the coverage role booting the same closure once per check.
@proposal n=3 detector=D-cascade severity=warning file=occupied-resources anchor=fct-qz4rtwlm basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Occupied Resources
change: «…in an `escher-session` host serving `load` and in the `mcp` role under a `start load`» reads «…in an `escher-session` host serving `load`, in the `mcp` role under a `start load` and in the `coverage` role over a `load` check».
sidecar: occupied-resources#9.1 names the coverage role among LoadNet's hosts (2026-11-08-handler-coverage).
rationale: the report's coverage role takes an app the binary boots and boots a fresh session per check, and stand_handler_coverage's load-page test reads the load page's click exercised.
@proposal n=4 detector=D-cascade severity=warning file=occupied-resources anchor=fct-jzxi3spj basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-trrk4ltd
section: ## Occupied Resources
change: after «…and `actions_needles` (…for the one `actions` call each log check makes)» add «and `coverage_needles` (needles read from the in-process CRUD screen's UI graph, for `host_log`'s one `coverage` run over a kept CRUD check, called by `host_log` alone)».
sidecar: occupied-resources#10.1 gains coverage_needles in the log checks' needle readers (2026-11-08-handler-coverage).
rationale: the report's Counts name fct-jzxi3spj a site of the needles pattern, coverage_needles called by host_log alone, and deviation 3 states it reads the CRUD screen.
@proposal n=5 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-dw3hon6z basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-z4cdnfit
section: ## Cross-cutting Patterns
change: «…its `signal_registry` module ungated» reads «…its `signal_registry` and `reached` modules ungated».
sidecar: cross-cutting-patterns#1.1 names the ungated reached module (2026-11-08-handler-coverage).
rationale: the report's Crates / modules add dioxus_native_dom::reached ungated, compiled with and without accessibility (deviation 1, the wasm build).
@proposal n=6 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-biqhin7y basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Cross-cutting Patterns
change: append — likewise `DioxusDocument::record_reached` provides a crate-private `ReachedRecord` context at `ScopeId::ROOT` through `runtime.provide_context`, a second call keeping the one record, which the bridge's dispatch looks up with `runtime.has_context`; a document never switched on has none.
sidecar: cross-cutting-patterns#4.3 gains the reached record's root context (2026-11-08-handler-coverage).
rationale: the report's Symbols / APIs state record_reached provides ReachedRecord at ScopeId::ROOT and handle_event writes it where has_context holds.
@proposal n=7 detector=D-cascade severity=warning file=cross-cutting-patterns anchor=fct-56u6r4qr basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Cross-cutting Patterns
change: append — beside it, where a `ReachedRecord` stands at the root, `handle_event` writes the record after the event-kind and dioxus-id early returns and before `runtime.handle_event`, inserting each chain node whose current listeners hold the event's name by its stable id resolved there, its `DocGuard` a temporary dropped before `runtime.handle_event`; every other document pays one `has_context` lookup per dispatch.
sidecar: cross-cutting-patterns#6.2 gains the reached record written on handle_event's path (2026-11-08-handler-coverage).
rationale: the report states the handle_event sites hold as written and gain the record beside them (reached pattern, E2), with the write's place and the per-dispatch lookup in Symbols / APIs.
@remark
the report names no state directory of host_log's or cli_commands' new coverage tests, so occupied-resources#5.1's enumerated hl-*/cc-* list may lack one; the coverage role, handler_coverage and the reached API belong to Standard Contracts (F1-F4), not this group.
