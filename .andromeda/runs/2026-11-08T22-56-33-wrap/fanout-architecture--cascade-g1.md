@proposal n=1 detector=D-cascade severity=warning file=established-decisions anchor=fct-6ytdsegs basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Established Decisions
change: Add beside the registry arm: since 2026-11-08-handler-coverage `handle_event` also writes the reached record, only where `runtime.has_context::<ReachedRecord>(ScopeId::ROOT)` holds, after the event-kind and dioxus-id early returns and before `runtime.handle_event`, inserting each chain node's stable id whose current listeners hold the event's name, its `DocGuard` a temporary dropped before `runtime.handle_event`; the `SignalRegistry` arm is unchanged, every other document pays one `has_context` lookup per dispatch, and `initial_build`, `poll` and `flush_queued_mounted_events` stay untouched (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md).
sidecar: established-decisions — the bridge's dispatch gains the reached record beside the SignalRegistry arm (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs place the record write in DioxusEventHandler::handle_event (dioxus_document.rs:448-449 · 452-460) under its own has_context gate, the registry arm unchanged; the entity's «does nothing without a registry» no longer describes the whole event-path change.
@proposal n=2 detector=D-cascade severity=warning file=established-decisions anchor=fct-kukffiv3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-s3yolsvv
section: ## Established Decisions
change: Add: since 2026-11-08-handler-coverage the command line also takes kept-check file paths as arguments — the `coverage` role, `coverage <app> <check>...`, each path read bounded and refused `malformed` holding nothing of the path, its answer one line of ids, owner ids, event names and two counts on stdout — with no new socket message, MCP tool, verb, cause, span field or dependency: a boundary widening ratified by the founder («A. Роль командной строки (Рекомендую)», P4 Q1, relayed verbatim by the overseer, inputs#I4).
sidecar: established-decisions — the driver session boundary gains the coverage role's kept-check path arguments, ratified (2026-11-08-handler-coverage).
rationale: Expected amendment E3 (fct-v4dqoq5r) and the founder's word on the carrier (fct-s3yolsvv) name kept-check files on the role's argv a ratified boundary widening, and the entity's own rule is that a new path argument widens.
@proposal n=3 detector=D-new-detail severity=warning file=established-decisions anchor=fct-wojfpqrs fact=F5 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-rhs2cert
section: ## Established Decisions
change: New item after it: Handler coverage reads every screen the replays reach — the UI graph at boot and after each replayed step — the overseer's delegate call (P4 Q2, inputs#I4), held PROVISIONAL until the epoch boundary; the canvas handler count is deferred to 0.4.0 and the stopPropagation limit is opened as a residual row (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md).
sidecar: established-decisions — handler coverage's screens-spanned decision, PROVISIONAL (2026-11-08-handler-coverage).
rationale: F5; the report's Decisions & corrections (fct-rhs2cert, fct-qr6xem6l) and E10 carry the overseer's delegate call and the P5 card's deferral and residual row.
@proposal n=4 detector=D-cascade severity=warning file=established-decisions anchor=fct-7p2g4lnp basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Established Decisions
change: Add: since 2026-11-08-handler-coverage a kept check is also read by the command line's `coverage` role from a path its caller names (bounded at MAX_CHECK_BYTES + 1, then `Check::from_bytes`) and replayed by `handler_coverage` through the crate-private `Check::replay_each`, still step by step through `Session::run`, `Check::replay` unchanged.
sidecar: established-decisions — a kept check gains the coverage role as a second reader and replay_each as its replay hook (2026-11-08-handler-coverage).
rationale: The report's Symbols / APIs: replay_each (replay.rs:47-55 · 67) with Check::replay calling it with a no-op hook, and read_check (cli.rs:495-505); the entity names Check::replay as the one replay path.
@proposal n=5 detector=D-cascade severity=warning file=body anchor=fct-nb3g6bqn basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-yj6ebn3k
section: ## Established Decisions
change: Add to what crosses: a coverage line's ids, owner ids, event names and two counts; and to the carriers: the command line's `coverage` role takes kept-check paths on argv, a ratified widening.
sidecar: body — the driver session's broad line gains the coverage line and the role's path arguments (2026-11-08-handler-coverage).
rationale: The report's Schema / config and Coverage of new surfaces state the coverage line holds ids, owner ids, event names and two counts; E3 ratifies the path arguments.
@proposal n=6 detector=D-cascade severity=warning file=conventions anchor=fct-bhxvo7z2 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-ufvb4gnc
section: ## Conventions
change: Add after the signal_registry clause: and its `reached` module (`ReachedHandler`, `record_reached`, `reached_handlers`) is not gated either, reading the crate-private ungated `element_listener_ids` rather than the `accessibility`-gated `element_facts`, so the record compiles on the wasm build that leaves `accessibility` off.
sidecar: conventions — feature gating names dioxus-native-dom's ungated reached module and element_listener_ids (2026-11-08-handler-coverage).
rationale: Deviation 1 (fct-ufvb4gnc) and Crates / modules (fct-z4cdnfit): the reached module is ungated (lib.rs:28) and element_listener_ids (element_id.rs:304-320) replaces the accessibility-gated element_facts.
@proposal n=7 detector=D-new-detail severity=warning file=conventions anchor=fct-zd5gb466 fact=F7 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-tsl356di
section: ## Conventions
change: Add to the session checks reading `common`: the handler coverage check `stand_handler_coverage` (8 tests, both layout modes), which also reads `common`'s `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` over the six served tasks, Cells' 676 grid-cell clicks generated by `grid_cells()` rather than stated literally.
sidecar: conventions — the tests convention names stand_handler_coverage among the checks sharing common and session_common (2026-11-08-handler-coverage).
rationale: F7; deviation 2 (fct-tsl356di) and the counts (fct-xgydedcz: mod common 23 → 24, mod session_common 34 → 35, stand files 53 → 54).
@remark
F1-F4 and F6 (the coverage role, handler_coverage, the reached record, the coverage limits) and the D-arch-resources registration of the coverage role, Coverage/HandlerRef/ReachedHandler and re-export counts 59→62 and 35→36 (fct-t7zq5feb, fct-kaaobbkf) are standard-contracts/existing-scopes facts, not group 1's; architecture fct-o6vmxcuj «by 23» is the cm-declarers count site outside these categories.
