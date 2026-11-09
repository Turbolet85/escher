@apply_row n=1 ref_file=fanout-architecture--cascade-g4.md ref_n=7 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g3.md ref_n=7 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=7
@apply_row n=3 ref_file=fanout-architecture.md ref_n=1 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=7
@apply_row n=4 ref_file=fanout-architecture--cascade-g4.md ref_n=8 v=applied edit=E2
@apply_row n=5 ref_file=fanout-architecture--cascade-g3.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=8
@apply_row n=6 ref_file=fanout-architecture.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=8
@apply_row n=7 ref_file=fanout-architecture--cascade-g4.md ref_n=9 v=applied edit=E3
@apply_row n=8 ref_file=fanout-architecture--cascade-g3.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=9
@apply_row n=9 ref_file=fanout-architecture.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=9
@apply_row n=10 ref_file=fanout-architecture.md ref_n=4 v=applied edit=E4
@apply_row n=11 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-architecture.md to_n=4
@apply_row n=12 ref_file=fanout-architecture.md ref_n=5 v=applied edit=E5
@apply_row n=13 ref_file=fanout-architecture--cascade-g5.md ref_n=2 v=folded_into to_file=fanout-architecture.md to_n=5
@apply_row n=14 ref_file=fanout-architecture--cascade-g1.md ref_n=1 v=applied edit=E6,E7
@apply_row n=15 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=applied edit=E8
@apply_row n=16 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=applied edit=E9
@apply_row n=17 ref_file=fanout-architecture--cascade-g1.md ref_n=6 v=applied edit=E10
@apply_row n=18 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E11
@apply_row n=19 ref_file=fanout-architecture--cascade-g3.md ref_n=1 v=applied edit=E12
@apply_row n=20 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=applied edit=E16
@apply_row n=21 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=applied edit=E17
@apply_row n=22 ref_file=fanout-architecture--cascade-g3.md ref_n=4 v=applied edit=E13
@apply_row n=23 ref_file=fanout-architecture--cascade-g3.md ref_n=6 v=applied edit=E15
@apply_row n=24 ref_file=fanout-architecture--cascade-g4.md ref_n=1 v=applied edit=E14
@apply_row n=25 ref_file=fanout-architecture--cascade-g3.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=1
@apply_row n=26 ref_file=fanout-architecture--cascade-g4.md ref_n=3 v=applied edit=E18
@apply_row n=27 ref_file=fanout-architecture--cascade-g4.md ref_n=4 v=applied edit=E19
@apply_row n=28 ref_file=fanout-architecture--cascade-g4.md ref_n=5 v=applied edit=E20
@fact id=fct-fukxpbvx expect_rev=1
old: with its `serve` host role and its `mcp` role,
new: with its `serve` host role, its `mcp` role and its `coverage` role,

@fact id=fct-srb7slak expect_rev=2
old: only where a registry stands.
new: only where a registry stands, and writes the ungated reached record (`DioxusDocument::record_reached`, `reached_handlers`, `ReachedHandler`) only where it was switched on, inert elsewhere at one `has_context` lookup per dispatch.

@fact id=fct-elnqiowc expect_rev=1
old: `Check::replay` runs them again through `Session::run`;
new: `Check::replay` runs them again through `Session::run`, and `handler_coverage` replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised;

@fact id=fct-nb3g6bqn expect_rev=3
old: and an actions answer's owner ids, verb words, closed cause words and booleans cross, never a `NodeId`;
new: an actions answer's owner ids, verb words, closed cause words and booleans, and a coverage line's ids, owner ids, event names and two counts cross, never a `NodeId`; the command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly by `Check::from_bytes`, a ratified boundary widening;

@fact id=fct-k6jtbppj expect_rev=1
old: its `mcp` role at most one session in process;
new: its `mcp` role at most one session in process and its `coverage` role one fresh session in process per check it is given, with no listener;

@fact id=fct-6ytdsegs expect_rev=1
old: event path that does nothing without a registry:
new: event path that does nothing without a registry or a switched-on reached record (below):

@fact after=fct-6ytdsegs as=n1
- The reached record rides the same dispatch, inert where it was not switched on: only where `runtime.has_context::<ReachedRecord>(ScopeId::ROOT)` holds, `DioxusEventHandler::handle_event` writes it after the event-kind and dioxus-id early returns and before `runtime.handle_event`, inserting each chain node's stable id whose current listeners hold the event's name, its `DocGuard` a temporary dropped before `runtime.handle_event`; the `SignalRegistry` arm is unchanged, every other document pays one `has_context` lookup per dispatch, and `initial_build`, `poll` and `flush_queued_mounted_events` stay untouched (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md).

@fact after=fct-wojfpqrs as=n2
- The command line's `coverage` role, `coverage <app> <check>...`, takes kept-check file paths its caller names as arguments: each path is read bounded and through `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` holding nothing of the path, and its answer is one line of ids, owner ids, event names and two counts on stdout; no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md.

@fact id=fct-7p2g4lnp expect_rev=3
old: a kept check replays through `Session::run` alone (`Check::replay`),
new: a kept check replays through `Session::run` alone (`Check::replay`, and `handler_coverage` through the crate-private `Check::replay_each` they share), the command line's `coverage` role also reading one from a path its caller names, bounded at `MAX_CHECK_BYTES` + 1 bytes and then `Check::from_bytes`,

@fact id=fct-bhxvo7z2 expect_rev=2
old: is not, so an app registers state whatever its features;
new: is not, so an app registers state whatever its features, and its `reached` module (`ReachedHandler`, `record_reached`, `reached_handlers`) is not either, reading the crate-private ungated `element_listener_ids` rather than the `accessibility`-gated `element_facts`, so the record compiles on the wasm build that leaves `accessibility` off;

@fact id=fct-zd5gb466 expect_rev=3
old: and the actions check `stand_actions`, which also reads `common`'s `ACTIONABLE_AT_BOOT` —
new: the actions check `stand_actions`, which also reads `common`'s `ACTIONABLE_AT_BOOT`, and the handler coverage check `stand_handler_coverage`, which also reads `common`'s `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES`, Cells' grid-cell clicks generated by `grid_cells()` rather than stated literally —

@fact id=fct-t7zq5feb expect_rev=4
old: the crate re-exports 59 names under
new: the crate re-exports 62 names (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md) under
old: the actions read's `ActionNode` and `VerbReading` from private modules, the last from the private module `actions`,
new: the actions read's `ActionNode` and `VerbReading` and the coverage reader's `Coverage`, `HandlerRef` and `handler_coverage` from private modules, the last from the private module `coverage`,

@fact after=fct-fltopt6e as=n3
- `Check::replay` runs through the crate-private `Check::replay_each(&self, session, after_step: impl FnMut(&Session)) -> Replay` with a no-op hook, its behaviour unchanged; `handler_coverage` is the hook's other caller, reading the screen after each replayed step (packages/escher-driver/src/replay.rs:47-55; packages/escher-driver/src/replay.rs:67).

@fact after=fct-2udtsa6z as=n4
- The coverage reader: the public `handler_coverage(checks: &[Check], boot: impl FnMut() -> Result<Session, SessionError>) -> Result<Coverage, SessionError>` boots a fresh session for each check; a check whose app is not the label runs nothing and contributes nothing, and for each other it calls `record_reached()`, reads the UI graph at boot and after each replayed step through `Check::replay_each` (`mounted` left out, deduplicated by id and event) and adds `reached_handlers()` after the replay; it returns `Coverage { unexercised: Vec<HandlerRef>, checks: usize, held: usize }` with `HandlerRef { id, owner: Option<String>, event }`, both re-exported, written by hand by `Coverage::to_json` as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent and `serde_json` writing nothing (packages/escher-driver/src/coverage.rs:21-102; packages/escher-driver/src/json.rs:419-442).

@fact after=@n4 as=n5
- Handler coverage's stated limits, each with its owner: handlers inside a canvas are not counted, deferred to 0.4.0 (docs/capabilities.md:273-275); a handler on no screen a replay reached is not listed, by construction, so no owner (docs/capabilities.md:276-278); and a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread, owned by an open `residual` row (target: next) (docs/capabilities.md:279-281).

@fact after=@n5 as=n6
- The command line's `coverage` role: its reader takes the first word `coverage` after `mcp` and before the verb table, in one shape, `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them — any other shape usage, status `2`; each path is read through `File::open(..).take(MAX_CHECK_BYTES + 1)`, then `Check::from_bytes`, and a file that cannot be read, one past the bound, one that is no check and a check whose `app()` is not `<app>` are refused `malformed`, status `1`, holding nothing of the path or the file; accepted, it prints the coverage line on stdout, status `0`, and a boot that fails is the session error, status `3`; the role calls the binary's boot once per check through `handler_coverage` (packages/escher-driver/src/cli.rs:45-47; packages/escher-driver/src/cli.rs:183-199; packages/escher-driver/src/cli.rs:462-473; packages/escher-driver/src/cli.rs:495-505).

@fact after=@n6 as=n7
- The usage line's role clauses read `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`, and the `command_line` doc names the coverage role, its endings and its two limits (packages/escher-driver/src/cli.rs:79-88).

@fact after=fct-uzcbujjs as=n8
- The reached record's write: where `runtime.has_context::<ReachedRecord>(ScopeId::ROOT)` holds, `DioxusEventHandler::handle_event` writes the record after the event-kind and dioxus-id early returns and before `runtime.handle_event`: for each node of the dispatch's chain whose current listeners hold the event's name it inserts that node's stable id with the event name, resolved at dispatch, so a handler whose own run removes its element still counts; it reads the crate-private ungated `element_listener_ids`, its `DocGuard` a temporary dropped before `runtime.handle_event`, so no borrow is taken across `render_immediate`; the `SignalRegistry` arm beside it is unchanged, and every other document pays one `has_context` lookup per dispatch (packages/dioxus-native-dom/src/dioxus_document.rs:448-449; packages/dioxus-native-dom/src/dioxus_document.rs:452-460).

@fact id=fct-ie5oemgh expect_rev=3
old: beside `element_facts`, each element's owning scope and component name;
new: beside `element_facts` (gated under `accessibility`), each element's owning scope and component name, and the ungated crate-private `element_listener_ids`, one vdom walk giving each element's stable id and its current listener events, whose one caller is the reached record;

@fact after=fct-b7zqmrdo as=n9
- The reached record: dioxus-native-dom's ungated `reached` module exposes `ReachedHandler { id, event }`, re-exported from the crate root; `DioxusDocument::record_reached(&mut self)` provides a crate-private `ReachedRecord` context at `ScopeId::ROOT` through `runtime.provide_context`, a second call keeping the one record; `DioxusDocument::reached_handlers(&self) -> Vec<ReachedHandler>` returns the handlers in first-reached order, deduplicated by (id, event), and is empty when the record was never switched on; the record's `Debug` prints its count alone (packages/dioxus-native-dom/src/reached.rs:20-27; packages/dioxus-native-dom/src/reached.rs:29-41; packages/dioxus-native-dom/src/reached.rs:85-105; packages/dioxus-native-dom/src/lib.rs:28; packages/dioxus-native-dom/src/lib.rs:45).
@pattern id=arch1-01
fixed: re-exports 59 names
@pattern id=arch1-02
regex: (does nothing|inert) without a registry
@pattern id=arch1-03
fixed: host role and its `mcp` role
