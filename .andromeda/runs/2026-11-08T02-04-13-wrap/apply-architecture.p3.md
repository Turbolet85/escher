@apply_row n=1 ref_file=fanout-architecture--cascade-g5.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g5.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g5.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g5.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture--cascade-g5.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g5.md ref_n=6 v=applied edit=E6
@apply_row n=7 ref_file=fanout-architecture--cascade-g5.md ref_n=7 v=applied edit=E7
@apply_row n=8 ref_file=fanout-architecture--cascade-g6.md ref_n=1 v=applied edit=E8
@apply_row n=9 ref_file=fanout-architecture--cascade-g6.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=1
@apply_row n=10 ref_file=fanout-architecture--cascade-g6.md ref_n=3 v=applied edit=E9
@apply_row n=11 ref_file=fanout-architecture--cascade-g6.md ref_n=4 v=applied edit=E10
@apply_row n=12 ref_file=fanout-architecture--cascade-g6.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=4
@apply_row n=13 ref_file=fanout-architecture--cascade-g6.md ref_n=6 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=4
@apply_row n=14 ref_file=fanout-architecture--cascade-g6.md ref_n=7 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=4
@apply_row n=15 ref_file=fanout-architecture--cascade-g6.md ref_n=8 v=applied edit=E11
@apply_row n=16 ref_file=fanout-architecture--cascade-g6.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g6.md to_n=8
@apply_row n=17 ref_file=fanout-architecture--cascade-g6.md ref_n=10 v=applied edit=E12
@apply_row n=18 ref_file=fanout-architecture--cascade-g6.md ref_n=11 v=applied edit=E13
@fact after=fct-sa5lklrp as=n1
- Both carriers, and a kept check, also take the screen's UI graph out as the `graph` verb's answer — stable ids, owner ids, component names, handler event names, registered state names, the declared writes the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`; `graph` takes no argument, and a call that gives one is refused `malformed` with nothing run; a snapshot node's `clickable`, a closed bool, crosses in the snapshot and the diff; a boundary widening ratified (§Established Decisions → [Driver session]); still the one listener and no port, the wire's bounds unchanged, the whole Cells answer 293,869 bytes under the 1,048,576-byte answer bound (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-cffzp2kh as=n2
- A `graph` call's `command` span records `verb` alone, its `tool_call` parenting it under the `mcp` role, and `host_log` and `mcp_log` each make one graph call and read 0 id, name and graph needles (component and state names) at `trace` under both builds; dioxus-native-dom's `SignalRegistry` is a Dioxus root context of one document's virtual DOM, not process-wide state, with no log site, print or env read (packages/dioxus-native-dom/src/signal_registry.rs:106-110; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-dw3hon6z expect_rev=1
old: `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules
new: `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules, its `signal_registry` module ungated

@fact after=fct-akoisoy3 as=n3
- The first `use_graph_state` hook of a page provides a `SignalRegistry` context at `ScopeId::ROOT`, which the bridge's dispatch looks up with `runtime.has_context`; a page that registers no state has none (packages/dioxus-native-dom/src/signal_registry.rs:47-63)

@fact after=fct-3kximnte as=n4
- Where a `SignalRegistry` stands at the root, `DioxusEventHandler::handle_event` sets the registry's dispatch — the chain head first, and its event name — around `runtime.handle_event` and clears it after, so a signal write is recorded against the dispatch that caused it; a page with no registered state dispatches as before, and the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched (packages/dioxus-native-dom/src/signal_registry.rs:168-184)

@fact after=@n4 as=n5
- In Dioxus 0.7.10 an async event handler's spawned task runs inside the dispatch that started it, so its signal writes are recorded against that event; dioxus-signals 0.7.10 keeps a reactive context subscribed while its `mark_dirty` returns true, so the registry's watcher owes no re-subscribe (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact id=fct-ldsoar27 expect_rev=1
old: the page saying twelve, the count
new: the page saying thirteen, the count

@fact id=fct-kaaobbkf expect_rev=2
old: `DioxusDocument::element_id` / `element_ids`), events,
new: `DioxusDocument::element_id` / `element_ids`, the walk's `element_facts` and `element_listeners`, and the reserved `{owner}///{key}` form's `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id`, both re-exported), events, graph (crate-private, compiled only under `accessibility`: the read-only reader `DioxusDocument::graph`, its `UiGraph`, `GraphNode`, `GraphHandler`, `GraphWrite`, `GraphState`, `GraphWriter` and `WriteFlag` types re-exported),
old: `TextPart` type re-exported), write_once_attr,
new: `TextPart` type re-exported), signal_registry (crate-private, under no `accessibility` gate: the `use_graph_state` hook, `DeclaredWrite` and `SignalRegistry`, re-exported), write_once_attr,
old: seven unit tests of the bridge (six until 2026-10-16-fast-failing-mutants, which added a created text node's mapping and stack read)
new: eight unit tests of the bridge, beside the `cfg(test)` helper `DioxusDocument::dispatch_to_vdom` in `dioxus_document.rs`

@fact id=fct-mzrxx2a3 expect_rev=1
old: json the finding's written form) — re-exported
new: json the finding's written form) and the `graph` verb — schema's `GRAPH` and its two `FieldKind`s, command's `Command::Graph`, execute holding `Outcome::Graphed`, json the graph's hand-written node and state forms — re-exported

@fact id=fct-4zmppehn expect_rev=1
old: (tests/blitz-tests/tests/password_caret_click.rs:83-121); and what
new: (tests/blitz-tests/tests/password_caret_click.rs:83-121); `stand_graph`, 6 tests in both layout modes — the verb's answer equal to `DioxusDocument::graph`, Counter's declared write read at boot and confirmed after a click, every stand task's declarations agreeing with its code once its handlers have run, the served screens' handlers, owners and clicks, every graph node a snapshot node with no served id holding `///`, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound — in `stand-repeat` by its `stand_` prefix (tests/blitz-tests/tests/stand_graph.rs:106-394); and what
old: tests/blitz-tests/tests/common/mod.rs:131-134) — and of seven helpers
new: tests/blitz-tests/tests/common/mod.rs:131-134), the stand's drive table `GRAPH_DRIVES` and its `Drive` steps, which drive each conditional write once (tests/blitz-tests/tests/common/mod.rs:234-323) — and of seven helpers
old: read through `mod common;` by 21 stand checks
new: read through `mod common;` by 22 stand checks
old: the twentieth and twenty-first, since 2026-10-14-stand-requirement-sweep)
new: the twentieth and twenty-first, since 2026-10-14-stand-requirement-sweep, and `stand_graph` the twenty-second)
old: with its `onkeydown`, measured and pinned),
new: with its `onkeydown`, measured and pinned), the per-task count of those role-less elements whose snapshot reads `clickable` (`CLICKABLE`: Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676),
old: parenting no `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:411-580; tests/blitz-tests/tests/stand_mcp_tools.rs:845-897)
new: parenting no `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:411-580; tests/blitz-tests/tests/stand_mcp_tools.rs:845-897), and a `graph` tool call answering the readers' graph and parenting one `command` span that records its verb alone (tests/blitz-tests/tests/stand_mcp_tools.rs:771-843)
old: `stand_cells_served` and `stand_load_page` — the readers counted below
new: `stand_cells_served`, `stand_load_page` and `stand_graph` — the readers counted below
old: as the driver writes it (tests/blitz-tests/tests/session_common/mod.rs:1-7;
new: as the driver writes it, and the `graph` builder and the `graphed` reader (tests/blitz-tests/tests/session_common/mod.rs:134-146; tests/blitz-tests/tests/session_common/mod.rs:1-7;

@fact id=fct-qkp2micb expect_rev=1
old: timer (examples/seven_guis/src/tasks/mod.rs:22-28);
new: timer (examples/seven_guis/src/tasks/mod.rs:22-28), the module also holding `use_graph_signal(name, writers, init) -> Signal<T>` (`use_signal` with `use_graph_state`) and the `const fn` `writes(id, event)`, through which every task's signals register with declarations read off the code, `use_signal` staying the stand shell's `active` and `stand.rs`'s `linked` (examples/seven_guis/src/tasks/mod.rs:4-20);
old: holds `cells_needles(part)` among its readers (examples/seven_guis/tests/common/mod.rs:484-513)
new: holds `cells_needles(part)` and `graph_needles()` among its readers (examples/seven_guis/tests/common/mod.rs:484-513; examples/seven_guis/tests/common/mod.rs:425-452)

@fact id=fct-okim3jaw expect_rev=1
old: its screen 47,089 bytes in 740 lines walked in 5 parts
new: its screen 53,859 bytes in 740 lines walked in 6 parts
old: the 47,089 at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
new: the 53,859 and the 6 parts at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-o6vmxcuj expect_rev=1
old: read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 32,
new: read through `mod common;` by 22 checks, `tests/session_common/mod.rs`, by 33,
old: the 32 and the 31 at
new: the 31 at

@pattern id=arch3-01
fixed: `snapshot_text` and `actionable` modules
@pattern id=arch3-02
fixed: the page saying twelve
@pattern id=arch3-03
fixed: seven unit tests of the bridge
@pattern id=arch3-04
fixed: by 21 stand checks
@pattern id=arch3-05
fixed: 47,089 bytes
@pattern id=arch3-06
fixed: by 21 checks, `tests/session_common/mod.rs`, by 32
