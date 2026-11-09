@proposal n=1 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Standard Contracts
change: «the crate re-exports 59 names» reads «the crate re-exports 62 names», the list gaining, since 2026-11-08-handler-coverage, the coverage reader's `handler_coverage`, `Coverage` and `HandlerRef` from the private module `coverage`.
sidecar: escher-driver crate-root re-exports 59 → 62 (Coverage, HandlerRef, handler_coverage).
rationale: the report's Counts measure the re-exports 59 → 62 at 8d6c5713 and the tree (pattern drv-reexport, site fct-t7zq5feb) and Symbols name the three re-exports (lib.rs:145).
@proposal n=2 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-t7zq5feb fact=F1 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: Beside the reader's rules: since 2026-11-08-handler-coverage the command line has a coverage role, its one shape `coverage <app> <check>...` with no `--session` or `--record` word (any other shape usage, status 2); each path read bounded (`MAX_CHECK_BYTES` + 1) then `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` (status 1) holding nothing of the path; accepted prints one coverage JSON line, status 0; a failing boot is the session error, status 3.
sidecar: Driver session: the command line's coverage role, its shape, bounded read and four endings.
rationale: the report's Symbols / APIs state the role (cli.rs:45-47, 183-199, 462-473, 495-505), its shape, the bounded read, the malformed refusal and the endings.
@proposal n=3 detector=D-cascade severity=warning file=standard-contracts anchor=fct-t7zq5feb basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: The usage line's role clauses read `; mcp role: mcp [--record <dir>]`, then, since 2026-11-08-handler-coverage, `; coverage role: coverage <app> <check>...;` before `; apps:`, and the `command_line` doc names the coverage role, its endings and its two limits.
sidecar: Driver session: the usage line gains its coverage role clause.
rationale: the report's Symbols / APIs and Schema / config state the usage line gains `; coverage role: coverage <app> <check>...;` before `; apps:` and the doc names the role (cli.rs:79-88).
@proposal n=4 detector=D-cascade severity=warning file=standard-contracts anchor=fct-fltopt6e basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: Add: since 2026-11-08-handler-coverage `Check::replay` runs through the crate-private `Check::replay_each(&self, session, after_step: impl FnMut(&Session)) -> Replay` with a no-op hook, its behaviour unchanged, `handler_coverage` the hook's other caller.
sidecar: Driver session: Check::replay delegates to the crate-private replay_each.
rationale: the report's Symbols / APIs state replay_each (replay.rs:47-55 · 67), crate-private, Check::replay calling it with a no-op hook, handler_coverage the other caller.
@proposal n=5 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-fltopt6e fact=F2 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: - The coverage reader (since 2026-11-08-handler-coverage): public `handler_coverage(checks: &[Check], boot) -> Result<Coverage, SessionError>` boots a fresh session per check, skips a check whose app is not the label, switches on `record_reached`, reads the UI graph at boot and after each replayed step (`mounted` left out, deduplicated by id and event), and returns `Coverage { unexercised: Vec<HandlerRef>, checks, held }`, `HandlerRef { id, owner, event }`, written by hand as `{"unexercised":[...],"checks":n,"held":m}`, `owner` left out when absent.
sidecar: Driver session: handler_coverage, Coverage, HandlerRef and the coverage line.
rationale: the report's Symbols / APIs state handler_coverage (coverage.rs:64-102), Coverage (coverage.rs:33-42), HandlerRef (coverage.rs:21-31) and Coverage::to_json (json.rs:419-442).
@proposal n=6 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-fltopt6e fact=F6 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-z77utipt
section: ## Standard Contracts
change: - Handler coverage's stated limits: handlers inside a canvas are not counted, a handler on no screen a replay reached is not listed, and a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread.
sidecar: Driver session: handler coverage's three stated limits.
rationale: the report's New text lists the three limits at docs/capabilities.md:273-281, and Symbols / APIs state the command_line doc names the role's limits.
@proposal n=7 detector=D-cascade severity=warning file=body anchor=fct-fukxpbvx basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## Standard Contracts
change: «with its `serve` host role and its `mcp` role» reads «with its `serve` host role, its `mcp` role and its `coverage` role».
sidecar: CLIs: escher-session's role set gains coverage.
rationale: the report's roles pattern names fct-fukxpbvx as a site stating the role set, and Symbols / APIs add the coverage role to command_line.
@proposal n=8 detector=D-arch-resources severity=warning file=body anchor=fct-elnqiowc basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: Add after: `handler_coverage` replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
sidecar: Driver session broad line: the coverage reader.
rationale: the report's Symbols / APIs land the public handler_coverage in escher-driver, unregistered in the body's driver lines.
@proposal n=9 detector=D-arch-resources severity=warning file=body anchor=fct-srb7slak basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## Standard Contracts
change: Add after the registry clause: the ungated `DioxusDocument::record_reached` and `reached_handlers` (`ReachedHandler`) keep, once switched on, the handlers `handle_event` reached, written on the same path.
sidecar: Dioxus DOM bridge broad line: the reached record.
rationale: the report's Symbols / APIs land ReachedHandler, record_reached and reached_handlers (reached.rs:20-27, 85-95, 97-105), written in handle_event (dioxus_document.rs:448-449 · 452-460); E2 names the bridge.
@remark
F3 and F4 (dioxus-native-dom reached module, element_listener_ids) have no picked entity here, the Dioxus DOM bridge detail and existing-scopes fct-kaaobbkf (the reached module) are the new-detail pass's; F5 (established-decisions) and F7 (conventions) are other categories'.
