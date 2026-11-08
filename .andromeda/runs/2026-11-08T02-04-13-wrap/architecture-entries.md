@amendment marker=2026-11-07-ui-graph
title: the UI graph crosses the ratified carriers
section: §Established Decisions → Driver session (body; established-decisions: the Driver session widenings, kept checks) · §Occupied Resources (occupied-resources: the carriers)
change:
  - The body's Driver session line: what crosses adds a UI graph's names, closed flag words and booleans (was ids, names, masked values, snapshot text, diffs, shots and lint findings), never a `NodeId`.
  - A new row: the UI graph crosses the session socket's `call`, the command line and MCP (a text block) and rests in a kept check as the `graph` verb's answer — stable ids, owner ids, component names, registered state names, handler event names, the declared writes the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`; `graph` takes no argument, one refused `malformed` with nothing run; each snapshot node carries the closed bool `clickable`; no new carrier, exit, cause, session error, span field or dependency.
  - Kept checks: what a kept check holds at rest adds a UI graph's names, closed flag words and booleans.
  - Occupied Resources: both carriers and a kept check take the graph out; still the one listener and no port, the wire's bounds unchanged, the whole Cells answer 293,869 bytes under the 1,048,576-byte answer bound.
why: a ratified boundary widening — the founder's answers at P4 fork 1 (the new `graph` verb) and at the P5 review (the authored write declarations), each by dialog in the overseer session, relayed by the operator; that these proposals lie inside those answers is the overseer's confirm at this wrap's halt, by question dialog — provisional, superseded by the founder's own word in his batch at the epoch boundary. The report's Coverage of new surfaces measures names only: a typed sentinel occurs 0 times in the answer.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the `graph` verb in the Driver session contract
section: §Standard Contracts → Driver session (body; standard-contracts: the verb table, the UI graph verb) · §Existing Scopes (existing-scopes: escher-driver)
change:
  - `VERBS` lists thirteen verbs (was twelve), the body's line too, nine asked of the held instance (was eight), `graph` the ninth after `lint`: no argument, an argument refused `malformed` with nothing run; result `nodes` (`FieldKind::GraphNodes`) and `states` (`FieldKind::GraphStates`); a further widening of the validated verb set.
  - `FieldKind` twelve kinds (was ten); `describe()` 13 verbs and 14 node keys (was 12 and 13).
  - `Command` gains `Graph`, validated from `("graph", [])`; `Outcome` gains `Graphed { graph: UiGraph }`, read by the read-only `DioxusDocument::graph`, so a graph read dispatches nothing, runs no step and settles nothing.
  - The record of ids is fed also from an accepted `graph`'s node ids; an accepted `graph` records `verb` alone on its `command` span, and a graph, its ids and its names are never fielded.
  - MCP `tools/list` answers thirteen verbs (was twelve), `graph` after `lint`; the `tool_call` span's `tool` reads `graph`, no field added.
  - A new row: `graph`'s hand-written form `{"nodes":[…],"states":[…]}` — its node, handler, write, state, declaration and writer keys — holding ids, component, state and event names, closed flag words and booleans only.
  - escher-driver's scope names the verb's homes: schema's `GRAPH` and its two `FieldKind`s, `Command::Graph`, `Outcome::Graphed`, json's node and state forms.
why: the report's Schema / config (`VERBS` 12 → 13, instance verbs 8 → 9, `FieldKind` 10 → 12, describe's row) and its Symbols / APIs for escher-driver; the widening is the founder's, as «the UI graph crosses the ratified carriers» records.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the reserved virtual-node id form
section: §Standard Contracts → Dioxus DOM bridge (body; standard-contracts: stable element ids) · §Existing Scopes (existing-scopes: dioxus-native-dom's element_id)
change:
  - Beside the four tiers the id grammar states a fifth, reserved form `{owner}///{key}`, which no tier can hold and no caller produces, the walk not calling it; the public, re-exported `VIRTUAL_KEY_SEPARATOR` (`"///"`) and `virtual_node_id(owner, key) -> Option<String>` spell it, answering `None` for an empty or `/`-bearing key.
  - The body's bridge line names the form beside the four tiers; element_id's scope names `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id`.
why: a stable-id contract change on the founder's answer at P4 fork 2, by dialog in the overseer session, relayed by the operator; the report's (arch) outcome measures no served id holding `///`.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the UI graph reader
section: §Standard Contracts → Dioxus DOM bridge (body; standard-contracts: the UI graph, the graph's readings) · §Existing Scopes (existing-scopes: dioxus-native-dom's modules)
change:
  - The body's bridge line: under `accessibility` the UI graph (`DioxusDocument::graph`, a `UiGraph` of nodes and registered states) reads the screen read-only beside the snapshot, its text form and its diff (was those three alone).
  - A new row: `DioxusDocument::graph(&self) -> UiGraph`, read-only, one `GraphNode { id, owner, component, handlers, reads }` per snapshot node in pre-order, with `GraphHandler`, `GraphWrite`, `GraphState`, `GraphWriter` and `WriteFlag::{Undeclared, Unconfirmed}` (`word()`).
  - A new row, its readings: `owner` the stable id of `select_of` for an option or an optgroup; a repeated (name, component) registration reads `{name}:{k}`; `readers` names each reading component once; `writers` lists only writes a run confirmed or flagged, `declared` keeping the authored declaration; a write resolves to the first chain element listening for its event, or to nothing past a gone node; `unconfirmed` fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
  - dioxus-native-dom's module list adds `graph` (crate-private, under `accessibility`, its seven types re-exported); element_id's scope adds the walk's `element_facts` and `element_listeners`.
why: the report's Symbols / APIs (the graph reader) and Crates / modules; `readers` naming components and `writers` listing confirmed or flagged writes are the operator's two fixes in the implement session, after implement's first report; the stop at a gone node is the report's disproved resolution rule, measured by `stand_graph`.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the snapshot's `clickable` reading
section: §Established Decisions → DOM semantics (established-decisions) · §Standard Contracts → Driver session and Dioxus DOM bridge (body; standard-contracts: node keys, the snapshot model, listener readings, `clickable` on the stand, selects) · §Existing Scopes (existing-scopes: `stand_non_lean_tasks`)
change:
  - `NodeState` carries eight fields (was seven), `clickable: bool` after `focused`: true where the vdom walk's current listeners hold `click` and the element is no `ElementData::is_disabled` form control, so a disabled `button` with a `click` listener reads not clickable and a `<div disabled>` with one reads clickable; DOM semantics names it among the readers of the one `disabled` reading.
  - The text form writes `clickable` after `focused`, only when true; `Reading` twelve variants (was eleven), `Clickable` after `Focused`, `differing` comparing it.
  - `NODE_FIELDS` fourteen keys (was thirteen), `[NodeFieldSpec; 14]`; a written node always carries `clickable` beside `focused`; the `changed` and `text` helps name it.
  - The exact listener reading is the crate-private `element_listeners`, `mounted` included, beside `element_facts`; the graph's handlers and `clickable` read these, never the listener mark.
  - A new row and `stand_non_lean_tasks`' scope: `CLICKABLE` reads Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676, in both layout modes, the listener mark's count staying 677.
why: the report's Symbols / APIs (the snapshot), Schema / config (`NODE_FIELDS` 13 → 14, `Reading::ALL` 11 → 12) and its new `CLICKABLE` table; a closed bool inside the ratified carrier (playbook rule `rul-cepujoy6`).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the graph registry and the bridge's dispatch
section: §Standard Contracts → Dioxus DOM bridge, High-level launch (body; standard-contracts) · §Conventions → Feature gating (conventions) · §Cross-cutting Patterns → Config management, Service wiring, Event processing (cross-cutting-patterns) · §Established Decisions → Dioxus integration, held signals (established-decisions) · §Existing Scopes (existing-scopes: dioxus-native-dom)
change:
  - `use_graph_state(name, writers, subscribers)` takes dioxus-core types alone; the first registering hook provides a `SignalRegistry` context at `ScopeId::ROOT`; each entry's `ReactiveContext::new_with_callback` records one write against the standing dispatch and the writing scope and leaves with its scope through `use_drop`; `DeclaredWrite { id, event }` names a stable id or a `*` prefix; the registry's `Debug` prints counts alone; dioxus_native re-exports the three.
  - `DioxusEventHandler::handle_event` sets the registry's dispatch (chain head first, its event name) around `runtime.handle_event` and clears it after, only where a `SignalRegistry` stands at the root — a change on every Dioxus document's event path, inert without a registry; the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` untouched, every manifest unchanged, `mutation_writer.rs` byte-equal to upstream `434e0661`.
  - Held signals: hook-local `use_signal` state an app does not register stays unreachable without a fork-level change (was: any hook-local state); a state registered through `use_graph_state` is reachable through public API, `DioxusDocument::graph` reading its readers and writes.
  - `graph` is gated by `accessibility`, `signal_registry` ungated (Feature gating, Config management, the module list).
  - The bridge tests read eight (was seven, its 2026-10-16 history clause retired), beside the `cfg(test)` helper `DioxusDocument::dispatch_to_vdom`.
why: the report's Symbols / APIs (the registry, the bridge dispatch), Crates / modules and Dependencies (none; the manifest and `mutation_writer.rs` gates green); a new bridge test pins a dispatch recorded where a state is registered and acting as before where none is.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: citation ranges re-read past the chunk's line shifts
section: body (Occupied Resources → URL schemes) · conventions (Feature gating, Unsafe and borrowing, Tests, the MCP listing test, the stylesheet constants) · cross-cutting-patterns (logging, Service wiring, Event processing, the mounted skip) · design-philosophy (`DioxusDocument`) · established-decisions (select limits, IME, DOM semantics, Dioxus integration, buffered render, Driver session) · existing-scopes (seven_guis, the upstream render test) · occupied-resources (state directories, spawned processes) · standard-contracts (Document core, select readers, seven_guis API, Driver session, the bridge, recording, load wait, Cells keyboard, masked input, selects, mounted flush, the cold-agent pipe)
change:
  - Each `file:line` range into a file this chunk lengthened — dioxus-native-dom's `dioxus_document.rs`, `snapshot.rs`, `snapshot_diff.rs`, `element_id.rs` and `lib.rs`; escher-driver's `execute.rs`, `json.rs`, `schema.rs` and `lib.rs`; seven_guis' tasks and tests; blitz-tests' `session_common`, `stand_mcp_tools`, `stand_act_spans` and `stand_non_lean_tasks`; `docs/capabilities.md`; `test_cold_agent.py` — reads where the code now stands, the claim around it unmoved.
why: the report's New text, by line: the chunk added lines ahead of each cited range.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: `stand_graph` and the blitz-tests shared modules
section: §Conventions → Tests (conventions) · §Existing Scopes (existing-scopes: blitz-tests' stand checks and shared modules) · §Standard Contracts (standard-contracts: `stand_graph`)
change:
  - `stand_graph`, 6 tests in both layout modes, in `stand-repeat` by its `stand_` prefix: the verb's answer equal to `DioxusDocument::graph`, Counter's declared write at boot and confirmed after a click, every stand task's declarations agreeing with its code once its handlers have run, the served screens' handlers, owners and clicks, every graph node a snapshot node with no served id holding `///`, a graph read changing nothing and holding no typed text, the whole Cells answer within the answer bound.
  - `tests/common/mod.rs` holds `Drive` and `GRAPH_DRIVES`, which drive each conditional write once so no stand write reads a flag; it is read by 22 stand checks (was 21), `stand_graph` the twenty-second.
  - `tests/session_common/mod.rs` gains the `graph` builder and the `graphed` reader and is read by 33 checks (was 32); the Tests convention lists `stand_graph` among the checks that hold a `Session`.
  - `stand_mcp_tools` gains a `graph` tool call answering the readers' graph and parenting one `command` span that records its verb alone.
why: the report's Changes (the new `stand_graph.rs`), Gates (6 passed) and its decision on `GRAPH_DRIVES`; the reader counts 22 and 33 measured on the work tree at this wrap's Validate.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: seven_guis registers its state through `use_graph_signal`
section: §Conventions → Apps and examples (conventions) · §Existing Scopes (existing-scopes: seven_guis' tasks) · §Standard Contracts (standard-contracts: the stand's graph hooks)
change:
  - `tasks::use_graph_signal(name, writers, init) -> Signal<T>` wraps `use_signal` and `use_graph_state` in `tasks/mod.rs`, and the crate-private `const fn` `tasks::writes(id, event)` builds a `DeclaredWrite`; every task's signals register through it with declarations read off the code, `use_signal` left to the stand shell's `active` and `stand.rs`'s `linked`.
  - A declaration slice is written `const { &[…] }` or as a `const` item, since a bare `&[writes(..)]` is a temporary not promoted to `'static` and errors `E0716` at the parameter.
why: the report's Reverted / negative API facts and Deviation 1: the plan's `Signal` form could not ship in dioxus-native-dom, which names no crate that defines `Signal`, and the manifest freeze forbids adding one; its Rust gotcha (five `E0716` on `cells.rs`).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: async handler writes land inside their dispatch
section: §Cross-cutting Patterns → Event processing (cross-cutting-patterns)
change:
  - A new row: in Dioxus 0.7.10 an async event handler's spawned task runs inside the dispatch that started it, so its signal writes are recorded against that event; dioxus-signals 0.7.10 keeps a reactive context subscribed while its `mark_dirty` returns true, so the registry's watcher owes no re-subscribe.
why: measured — `stand_graph`'s first red on Circle Drawer's canvas writes, and the report's disproved plan claim that the watcher re-subscribes (dioxus-signals' `update_subscribers` keeps such a context).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the log checks read the graph's names
section: §Existing Scopes (existing-scopes: seven_guis' `tests/common/mod.rs`) · §Occupied Resources (occupied-resources: the log checks' readers, the graph call's spans)
change:
  - seven_guis' shared `tests/common/mod.rs` holds `graph_needles` beside `cells_needles(part)`: each served component and registered state name, for the one `graph` call `host_log` and `mcp_log` each make.
  - A new row: a `graph` call's `command` span records `verb` alone, its `tool_call` parenting it under the `mcp` role, and both log checks read 0 id, name and graph needles at `trace` under both builds; `SignalRegistry` is a root context of one document's virtual DOM, not process-wide state, with no log site, print or env read.
why: the report's Coverage of new surfaces (instrumentation, PII) and its (obs) outcome; the `graph_needles` line was raised at Validate, where the log checks' module list read false.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the Cells screen in six parts
section: §Existing Scopes (existing-scopes: the served-page checks)
change:
  - `stand_cells_served`'s Cells screen reads 53,859 bytes in 740 lines, walked in 6 parts of `SNAPSHOT_TEXT_BUDGET` (was 47,089 bytes in 5 parts), its measurement re-dated to this chunk.
why: the `clickable` token the snapshot text now writes; the report's `cells-parts` reading (a fresh `start cells` read whole by parts; `stand_cells_served` `PARTS` 6), the plan's prediction confirmed.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: thirteen verbs where a surface counts them
section: §Project Intent → Front page (project-intent) · §Standard Contracts → the recording, the visual lint, the load wait, the served command line, CI contracts (standard-contracts)
change:
  - The README's count of the driver commands as tools reads thirteen (was twelve).
  - The recording keeps a `graph` step like every other, its answer whole, and reads the nine instance verbs (was eight), in its keeping and in `Check::from_bytes`.
  - The visual lint row: `VERBS` holds thirteen, `graph` listed after `lint` (was twelve).
  - The load wait: `graph`, which runs no step, waits on nothing, beside `snapshot`, `screenshot` and `lint`.
  - The served command line: the thirteen of `VERBS` (was twelve), `graph` after `lint`.
  - The cold-agent pipe runs with only the escher server's thirteen tools, its isolation atom reading thirteen `mcp__escher__{verb}` tools (was twelve each).
why: the report's Schema / config (`VERBS` 12 → 13, instance verbs 8 → 9) and Harness / gate surface (`cold-agent.sh` and `test_cold_agent.py` `VERBS` gain `graph`).
kept: the rows naming `lint` the eighth instance verb stand, `graph` being the ninth; the 2026-10-25 live run's twelve MCP tools stays as that run read them.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the unpaged graph and its Cells size
section: §Standard Contracts (standard-contracts: the graph is unpaged)
change:
  - A new row: `graph` answers the whole screen at once; Cells' answer reads 293,869 bytes, under the 1,048,576-byte answer bound `stand_graph` holds in both layout modes; the remainder, each cell node's declared `click` writes, is the stated limit «An unpaged graph», owned by residual `res-7cc5bi4k` (target: next).
why: measured — 547,685 bytes at the first build, 522,692 after the readers narrowing, 293,869 after the writers narrowing, the plan's under-200 KB forecast disproved; the stated limit and its residual row are the operator's word in the implement session, kept so by the overseer's word the operator relayed at this resume.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
