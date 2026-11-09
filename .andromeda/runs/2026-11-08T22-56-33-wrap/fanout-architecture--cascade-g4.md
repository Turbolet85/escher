@proposal n=1 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-2udtsa6z fact=F2 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add a row after it: escher-driver's public `handler_coverage(checks, boot)` boots a fresh session for each check and skips any check whose app is not the label (that check contributes nothing). For the others it calls `record_reached()` and reads the UI graph at boot and after each replayed step through the crate-private `Check::replay_each`, leaving `mounted` out and deduplicating by (id, event). It returns `Coverage { unexercised: Vec<HandlerRef>, checks, held }` with `HandlerRef { id, owner, event }`, each re-exported, written by hand as `{"unexercised":[...],"checks":n,"held":m}` with `owner` left out when absent.
sidecar: standard-contracts: Driver session gains the handler_coverage row (Coverage, HandlerRef, the coverage line)
rationale: the report's Symbols / APIs add handler_coverage, Coverage, HandlerRef, Coverage::to_json and replay_each (coverage.rs:21-102, json.rs:419-442, replay.rs:47-55); E1 carries them to this contract
@proposal n=2 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-2udtsa6z fact=F6 basis_doc=architecture basis_fact=fct-2udtsa6z
section: ## Standard Contracts
change: add a row beside the handler_coverage row: handler coverage's stated limits are that handlers inside a canvas are not counted (deferred to 0.4.0), a handler on no screen a replay reached is not listed (by construction, no owner), and a bubbling dispatch counts every ancestor listener it passed with a stopPropagation unread (a new residual row, target next).
sidecar: standard-contracts: Driver session gains handler coverage's three stated limits
rationale: the report's Outcome and inputs I5 state the three limits (docs/capabilities.md:273-281) and their owners, in the shape of the actions row's stated limit
@proposal n=3 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-uzcbujjs fact=F4 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add a row after it: where `runtime.has_context::<ReachedRecord>(ScopeId::ROOT)` holds, `DioxusEventHandler::handle_event` writes the reached record after the event-kind and dioxus-id early returns and before `runtime.handle_event`. For each node of the dispatch's chain whose current listeners hold the event's name, it inserts that node's stable id, resolved at dispatch, so a handler whose own run removes its element still counts. It reads the crate-private ungated `element_listener_ids`, the `DocGuard` is a temporary dropped before `runtime.handle_event`, no borrow is taken across `render_immediate`, and every other document pays one `has_context` lookup per dispatch.
sidecar: standard-contracts: the bridge's dispatch gains the reached record beside the registry's dispatch
rationale: the report's Symbols / APIs place the record's write on handle_event (dioxus_document.rs:448-449, 452-460) next to the unchanged SignalRegistry arm; E2
@proposal n=4 detector=D-cascade severity=warning file=standard-contracts anchor=fct-ie5oemgh basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-ufvb4gnc
section: ## Standard Contracts
change: next to `element_listeners` and `element_facts` (the latter gated under `accessibility`), name the ungated crate-private `element_listener_ids`: one vdom walk that gives each element's stable id and its current listener events. The reached record is its one caller, and it reads the exact listeners, never the listener mark.
sidecar: standard-contracts: the exact listener readers gain the ungated element_listener_ids, the reached record's reader
rationale: deviation 1 and Symbols / APIs: the record reads element_listener_ids (element_id.rs:304-320) because element_facts is gated under accessibility and the record must compile without it
@proposal n=5 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-b7zqmrdo fact=F3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add a row after it: dioxus-native-dom's ungated `reached` module exposes `ReachedHandler { id, event }`, re-exported from the crate root. `DioxusDocument::record_reached` provides a crate-private `ReachedRecord` context at `ScopeId::ROOT`, and a second call keeps the one record. `DioxusDocument::reached_handlers` returns the handlers in first-reached order, deduplicated by (id, event), and empty when the record was never switched on. The record's `Debug` prints its count alone.
sidecar: standard-contracts: Dioxus DOM bridge gains the reached module (ReachedHandler, record_reached, reached_handlers)
rationale: the report's Symbols / APIs (reached.rs:20-105, lib.rs:28, lib.rs:45); E2
@proposal n=6 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-c5e4c7ao fact=F1 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add after the mcp role: the `coverage` role has one shape, `coverage <app> <check>...`, with no `--session` or `--record` word (any other shape is usage, status 2). Each path is read bounded to MAX_CHECK_BYTES + 1 and then through `Check::from_bytes`. An unreadable, over-bound, non-check or other-app file is refused `malformed` (status 1), holding nothing of the path. An accepted run prints one coverage JSON line with status 0, and a failing boot is the session error, status 3. The role calls the same boot closure once per check, and the usage line gains its coverage clause before `; apps:`.
sidecar: standard-contracts: CLIs gain escher-session's coverage role
rationale: the report's Symbols / APIs add the command_line coverage role (cli.rs:45-47, 183-199, 462-473, 495-505; session_host.rs:5-7); E1
@proposal n=7 detector=D-arch-resources severity=warning file=body anchor=fct-fukxpbvx basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: the escher-session clause reads «with its `serve` host role, its `mcp` role and its `coverage` role» in place of «with its `serve` host role and its `mcp` role»
sidecar: body: the CLIs line names the coverage role beside serve and mcp
rationale: the report's roles pattern names fct-fukxpbvx as a site that states the role set, and the coverage role is a new command-line surface
@proposal n=8 detector=D-arch-resources severity=warning file=body anchor=fct-srb7slak basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add at the end: «; the ungated reached record (`record_reached`, `reached_handlers`, `ReachedHandler`) is written on `handle_event`'s path only where it was switched on»
sidecar: body: the Dioxus DOM bridge line names the reached record
rationale: the reached sweep keeps fct-srb7slak as written and adds the record beside it (E2); the record is a new public API of dioxus-native-dom
@proposal n=9 detector=D-arch-resources severity=warning file=body anchor=fct-elnqiowc basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: add: «`handler_coverage` replays kept checks on fresh sessions and lists the UI graph's handlers no check exercised»
sidecar: body: the Driver session recording line names handler_coverage beside Check::replay
rationale: handler_coverage is a new public escher-driver API and the second caller of the replay path (replay.rs:47-55)
@remark
Not placed here: the escher-driver re-export count 59 to 62 (fct-t7zq5feb) and the dioxus-native-dom module list missing `reached` (fct-kaaobbkf) belong to the existing-scopes group; F5 (established-decisions) and F7 (conventions) belong to other groups. The founder's carrier ruling (E3) is an established-decisions edit.
