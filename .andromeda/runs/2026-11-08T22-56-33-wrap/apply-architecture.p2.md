@apply_row n=1 ref_file=fanout-architecture--cascade-g5.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g5.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g5.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g5.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture--cascade-g5.md ref_n=6 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g5.md ref_n=7 v=applied edit=E6
@apply_row n=7 ref_file=fanout-architecture--cascade-g6.md ref_n=1 v=applied edit=E7
@apply_row n=8 ref_file=fanout-architecture--cascade-g6.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=1
@apply_row n=9 ref_file=fanout-architecture--cascade-g6.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=1
@apply_row n=10 ref_file=fanout-architecture--cascade-g6.md ref_n=4 v=applied edit=E8
@apply_row n=11 ref_file=fanout-architecture--cascade-g6.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=4
@apply_row n=12 ref_file=fanout-architecture--cascade-g6.md ref_n=6 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=4
@apply_row n=13 ref_file=fanout-architecture--cascade-g6.md ref_n=7 v=applied edit=E9
@apply_row n=14 ref_file=fanout-architecture--cascade-g6.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=7
@apply_row n=15 ref_file=fanout-architecture--cascade-g6.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=7
@apply_row n=16 ref_file=fanout-architecture--cascade-g6.md ref_n=10 v=applied edit=E10
@apply_row n=17 ref_file=fanout-architecture--cascade-g6.md ref_n=11 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=10
@apply_row n=18 ref_file=fanout-architecture--cascade-g6.md ref_n=12 v=applied edit=E11
@fact after=fct-3eiirvbg as=n1
- The binary's `coverage` role (`coverage <app> <check>...`) runs in one process with no listener, socket or port. It boots one fresh session in process per named check, through the same boot closure the host and the `mcp` role use, and each replayed step runs through `Session::run` under the binary's `escher_telemetry::init` sink, leaving its own `command` span; the role opens no span of its own, a further role reaching that one install site (examples/seven_guis/src/session_host.rs:5-7; packages/escher-driver/src/coverage.rs:64-102; packages/escher-driver/src/cli.rs:462-473).

@fact id=fct-qz4rtwlm expect_rev=1
old: serving `load` and in the `mcp` role under a `start load`
new: serving `load`, in the `mcp` role under a `start load` and in the `coverage` role over a `load` check

@fact id=fct-jzxi3spj expect_rev=3
old: ) and `actions_needles` (
new: ), `actions_needles` (
old: for the one `actions` call each log check makes) (examples
new: for the one `actions` call each log check makes) and `coverage_needles` (needles read from the in-process CRUD screen's UI graph, for `host_log`'s one `coverage` run over a kept CRUD check, `host_log` alone calling it) (examples

@fact id=fct-dw3hon6z expect_rev=2
old: its `signal_registry` module ungated
new: its `signal_registry` and `reached` modules ungated

@fact after=fct-biqhin7y as=n2
- `DioxusDocument::record_reached` provides a crate-private `ReachedRecord` context at `ScopeId::ROOT` through `runtime.provide_context`, a second call keeping the one record, which the bridge's dispatch looks up with `runtime.has_context`; a document never switched on has none (packages/dioxus-native-dom/src/reached.rs:85-95)

@fact after=fct-56u6r4qr as=n3
- Where a `ReachedRecord` stands at the root, `DioxusEventHandler::handle_event` writes the record after the event-kind and dioxus-id early returns and before `runtime.handle_event`: each node of the chain whose current listeners hold the event's name is inserted with that name by its stable id, resolved there, so a handler whose own run removes its element still counts; the `DocGuard` is a temporary dropped before `runtime.handle_event`, the `SignalRegistry` arm beside it is unchanged, `poll`, `initial_build` and `flush_queued_mounted_events` are untouched, and every other document pays one `has_context` lookup per dispatch (packages/dioxus-native-dom/src/dioxus_document.rs:448-449; packages/dioxus-native-dom/src/dioxus_document.rs:452-460)

@fact id=fct-kaaobbkf expect_rev=4
old: through one `MutationWriter`), snapshot (
new: through one `MutationWriter`), reached (under no `accessibility` gate: the reached record — `DioxusDocument::record_reached`, which provides the crate-private `ReachedRecord` at `ScopeId::ROOT`, a second call keeping the one record, and `DioxusDocument::reached_handlers`, the handlers in first-reached order deduplicated by (id, event), empty when the record was never switched on; written in `DioxusEventHandler::handle_event` before `runtime.handle_event`; its `ReachedHandler` type re-exported), snapshot (
old: the walk's `element_facts` and `element_listeners`, and the reserved
new: the walk's `element_facts` and `element_listeners`, the ungated `element_listener_ids` (one vdom walk giving each element's stable id and its current listener events, called by the reached record alone), and the reserved
old: eight unit tests of the bridge
new: nine unit tests of the bridge, among them the reached record holding on a page with no registered state

@fact id=fct-mzrxx2a3 expect_rev=3
old: eighteen private modules
new: nineteen private modules
old: json the action node's hand-written form) — re-exported at the crate root
new: json the action node's hand-written form) and coverage (`handler_coverage`, `Coverage` and `HandlerRef`: a fresh session booted per check, a check of another app running nothing and contributing nothing, the reached record switched on, and the UI graph read at boot and after each replayed step, `mounted` left out, deduplicated by (id, event), the handlers it reached taken after the replay; json the coverage line's hand-written form `Coverage::to_json`) — re-exported at the crate root
old: its `help` ending since 2026-10-12-self-description)
new: its `help` ending since 2026-10-12-self-description; its `coverage` role, `coverage <app> <check>...`, reading each named kept check through a bounded read and `Check::from_bytes` in its crate-private `read_check` and answering one coverage line)
old: acceptance or kept answer differs)
new: acceptance or kept answer differs; the crate-private `Check::replay_each`, which runs a hook after each step, `Check::replay` calling it with a no-op hook and `handler_coverage` its other caller)

@fact id=fct-4zmppehn expect_rev=4
old: `stand_replay`, 3 tests in both layout modes
new: `stand_replay`, 4 tests in both layout modes
old: its failure messages holding the mode, the file and a step index alone (tests/blitz-tests/tests/stand_replay.rs:84-113; tests/blitz-tests/tests/stand_replay.rs:201-237; tests/blitz-tests/tests/stand_replay.rs:239-261)
new: its failure messages holding the mode, the file and a step index alone, and a recorded Counter session with two `actions` steps holding on a fresh Counter, all 4 steps run, while its copy with one `keyed` flipped stops at step 3, accepted then and now, its keys exactly `["nodes"]` (tests/blitz-tests/tests/stand_replay.rs:84-113; tests/blitz-tests/tests/stand_replay.rs:201-237; tests/blitz-tests/tests/stand_replay.rs:239-261; tests/blitz-tests/tests/stand_replay.rs:263-329)
old: as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md) — and of seven helpers
new: as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md), `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES` — the handlers each served task shows at boot and those each `GRAPH_DRIVES` task leaves unexercised after its drives, Cells' 676 grid cells generated by `grid_cells()` (tests/blitz-tests/tests/common/mod.rs:336-412) — and of seven helpers
old: read through `mod common;` by 23 stand checks
new: read through `mod common;` by 24 stand checks
old: `stand_graph` the twenty-second and `stand_actions` the twenty-third)
new: `stand_graph` the twenty-second, `stand_actions` the twenty-third and `stand_handler_coverage` the twenty-fourth)
old: (tests/blitz-tests/tests/stand_actions.rs:164-560); and what
new: (tests/blitz-tests/tests/stand_actions.rs:164-560); `stand_handler_coverage`, 8 tests in both layout modes — handler coverage on the six served tasks: a snapshot check listing exactly `HANDLERS_AT_BOOT`, equal to the graph's reading at boot, a check of each task's drives listing its `UNEXERCISED_AFTER_DRIVES` row, two checks listing only what neither exercised, the load page registering no state and reading its click exercised, a refused step exercising nothing and a keyboard activation its click, a handler whose step removes its element read exercised, a typed sentinel never on the coverage line, and a check of another app contributing nothing and not holding — in `stand-repeat` by its `stand_` prefix (tests/blitz-tests/tests/stand_handler_coverage.rs:212-423); and what
old: `stand_graph` and `stand_actions` — the readers counted below
new: `stand_graph`, `stand_actions` and `stand_handler_coverage` — the readers counted below

@fact id=fct-qkp2micb expect_rev=3
old: in its `mcp` role an MCP server over its standard streams:
new: in its `mcp` role an MCP server over its standard streams and, in its `coverage` role, `coverage <app> <check>...`, the handlers its named kept checks left unexercised, the same boot closure called once per check:
old: `graph_needles()` and `actions_needles()` among its readers (examples/seven_guis/tests/common/mod.rs:535-564
new: `graph_needles()`, `actions_needles()` and `coverage_needles()` — `host_log` alone calling it, over a CRUD screen — among its readers (examples/seven_guis/tests/common/mod.rs:454-479; examples/seven_guis/tests/common/mod.rs:535-564

@fact id=fct-o6vmxcuj expect_rev=3
old: by 23 checks, `tests/session_common/mod.rs`, by 34,
new: by 24 checks, `tests/session_common/mod.rs`, by 35,
old: the 34 and the 32 at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
new: the 24 and the 35 at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md, the 32 at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
@pattern id=arch2-01
fixed: and in the `mcp` role under a `start load`
@pattern id=arch2-02
fixed: eight unit tests of the bridge
@pattern id=arch2-03
fixed: eighteen private modules
@pattern id=arch2-04
fixed: `stand_replay`, 3 tests
@pattern id=arch2-05
fixed: `mod common;` by 23
@pattern id=arch2-06
fixed: `tests/session_common/mod.rs`, by 34
