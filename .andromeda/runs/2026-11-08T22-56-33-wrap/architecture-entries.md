@amendment marker=2026-11-08-handler-coverage
title: the coverage role's kept-check files on argv, a ratified boundary widening
section: §Established Decisions → Driver session (body: the session's crossing; established-decisions: the coverage role's path arguments, a kept check's readers)
change:
  - What crosses gains a coverage line's ids, owner ids, event names and two counts, still never a `NodeId`.
  - The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly by `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` holding nothing of the path; its answer is one line on stdout; no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening.
  - A kept check was replayed by `Check::replay` alone; now also by `handler_coverage` through the crate-private `Check::replay_each` they share, and read by the `coverage` role from a named path, bounded at `MAX_CHECK_BYTES` + 1 bytes and then `Check::from_bytes`.
why: the decision's own rule, a new carrier or exit widens the boundary. Ratified by the founder's own word at P4 Q1 on the carrier, relayed verbatim by the overseer (inputs#I4), a boundary widening (rul-izni57qv); confirmed at this wrap's halt Q1 by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the coverage reader handler_coverage and its stated limits
section: §Standard Contracts → Driver session (body; standard-contracts: the re-export list, the replay, the coverage reader, the limits) · §Existing Scopes (existing-scopes: escher-driver's modules)
change:
  - The public `handler_coverage(checks, boot)` boots a fresh session per check; a check of another app runs nothing and contributes nothing; for each other it switches the reached record on, reads the UI graph at boot and after each replayed step (`mounted` left out, deduplicated by id and event) and adds the reached handlers after the replay, answering `Coverage { unexercised, checks, held }` with `HandlerRef { id, owner, event }`.
  - `Coverage::to_json` writes `{"unexercised":[…],"checks":n,"held":m}` by hand, `owner` left out when absent, `serde_json` writing nothing.
  - `Check::replay` runs through the crate-private `Check::replay_each` with a no-op hook, its behaviour unchanged; `handler_coverage` is the hook's other caller.
  - The crate re-exports 62 names (was 59), gaining `Coverage`, `HandlerRef` and `handler_coverage` from the private module `coverage`; escher-driver holds nineteen private modules (was eighteen).
  - Three stated limits, each with its owner: handlers inside a canvas not counted, deferred to 0.4.0; a handler on no screen a replay reached not listed, by construction, no owner; a bubbling dispatch counting every ancestor listener it passed, `stopPropagation` unread, owned by an open residual row (target: next).
why: the report's new reader and its measured re-export count (59 to 62). The limits' owners are the operator's own word on P5 card 2 (inputs#I5), approving its deferral, its residual row and its limit with no owner.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the reached record on the bridge's dispatch
section: §Standard Contracts → Dioxus DOM bridge (body; standard-contracts: the reached record, its write, the listener readers) · §Conventions (feature gating) · §Cross-cutting Patterns (config management, the record's context and write) · §Established Decisions (the bridge's dispatch) · §Existing Scopes (dioxus-native-dom's modules)
change:
  - dioxus-native-dom's ungated `reached` module: `ReachedHandler { id, event }`, re-exported; `DioxusDocument::record_reached` provides a crate-private `ReachedRecord` at `ScopeId::ROOT`, a second call keeping the one record; `reached_handlers` returns the handlers in first-reached order, deduplicated by (id, event), empty when never switched on; the record's `Debug` prints its count alone.
  - `handle_event` writes the record only where it was switched on, after the event-kind and dioxus-id early returns and before `runtime.handle_event`, inserting each chain node's stable id whose current listeners hold the event's name, so a handler whose run removes its element still counts; its `DocGuard` a temporary, no borrow across `render_immediate`; every other document pays one `has_context` lookup per dispatch.
  - The dispatch change was inert without a registry; now without a registry or a switched-on reached record, the `SignalRegistry` arm unchanged.
  - The record reads the crate-private ungated `element_listener_ids`, not the `accessibility`-gated `element_facts`; the ungated modules are `signal_registry` and `reached` (was `signal_registry` alone).
  - `bridge_tests` hold nine unit tests (was eight), the ninth the record holding on a page with no registered state.
why: the report's new record and its counts (bridge_tests 8 to 9); its deviation from the plan, the record compiling on the wasm build that leaves `accessibility` off.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the escher-session coverage role, its shape, endings and process
section: §Standard Contracts → CLIs (body; standard-contracts: the role's reader, the usage line) · §Occupied Resources → Process-wide state and threads (body; occupied-resources: the role's process, `LoadNet`'s hosts) · §Existing Scopes (existing-scopes: escher-driver's cli, seven_guis' escher-session)
change:
  - `escher-session` has a `serve` host role, an `mcp` role and a `coverage` role (was the first two).
  - The reader takes the first word `coverage` after `mcp` and before the verb table, in one shape, `coverage <app> <check>...`, no `--session` and no `--record` word; any other shape usage, status 2; a refused file `malformed`, status 1; accepted, the coverage line on stdout, status 0; a failed boot the session error, status 3; the binary's boot called once per check.
  - The usage line gains `; coverage role: coverage <app> <check>...;` before `; apps:`, and the `command_line` doc names the role, its endings and its two limits.
  - The role is one process, one fresh session in process per check, no listener, socket or port, each replayed step leaving its own `command` span, the role opening none; `LoadNet` also answers in the role over a `load` check.
why: the report's new role. Its carrier is the founder's own word at P4 Q1, relayed verbatim by the overseer (inputs#I4), ratified; confirmed at this wrap's halt Q1 by the overseer as delegate, PROVISIONAL (delegate overseer), for the founder's own word at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: citations re-pinned to the lines the chunk moved
section: §Occupied Resources (body, occupied-resources) · §Conventions · §Cross-cutting Patterns · §Design Philosophy · §Established Decisions · §Existing Scopes · §Standard Contracts
change:
  - The line ranges of the citations into the files the chunk edited, among them `dioxus_document.rs`, both crates' `lib.rs`, `element_id.rs`, escher-driver's `cli.rs`, `json.rs` and `replay.rs`, `session_host.rs`, `host_log.rs`, `cli_commands.rs` and `docs/capabilities.md`, read where the cited text now stands; no claim's words moved.
why: the report's changed files, the chunk's added lines shifting the text those citations name.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: stand_handler_coverage, the handler tables and the shared modules' readers
section: §Conventions → Tests · §Existing Scopes (existing-scopes: blitz-tests' shared modules, `stand_replay`, seven_guis' test module) · §Occupied Resources (occupied-resources: the log checks' needles)
change:
  - `stand_handler_coverage`, 8 tests in both layout modes over the six served tasks, joins the session checks, reading `common`'s `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES`, Cells' 676 grid cells generated by `grid_cells()`; in `stand-repeat` by its prefix.
  - `mod common;` is read by 24 checks (was 23), `mod session_common;` by 35 (was 34).
  - `stand_replay` holds 4 tests (was 3), the fourth a recorded Counter session with two `actions` steps holding, its copy with one `keyed` flipped stopping at step 3, keys exactly `["nodes"]`.
  - seven_guis' shared test module gains `coverage_needles()`, read from the CRUD screen's UI graph, `host_log` alone calling it.
why: the report's measured counts (23 to 24, 34 to 35, 3 to 4), its deviations (the tables partly generated; the needles read on CRUD) and its outcome for v030-06.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
