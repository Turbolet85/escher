@amendment marker=2026-11-07-ui-graph
title: the «reacts to a click» snapshot field built as NodeState.clickable
section: body §7 Screen Reader Support (the snapshot-model and actionable-key lines); §9 CI Integration (ci-integration.md: the `dioxus_runtime_probe` row); §7 Screen Reader Support (screen-reader-support.md: the snapshot-model, no-`expanded`-key, actionable-key, probe click-reading and listener over-report rows, three new rows)
change:
  - The snapshot model takes `clickable` from the vdom walk's current `click` listeners on an element that is no disabled form control (body snapshot-model line).
  - The actionable-key line: was «no snapshot field says an element reacts to a click», the reading owed to the route entry «The UI graph»; now the snapshot field `clickable` says it, read from the walk's current listeners and not the listener mark.
  - New row: `NodeState.clickable` is a `bool` after `focused` (`ElementData::is_disabled` excludes a disabled form control); `Snapshot::to_text` writes the token `clickable` after `focused` only when true, the JSON node always carries it beside `focused`, and `Snapshot::diff` reads it as `Reading::Clickable`, one of `Reading::ALL`'s twelve.
  - `NODE_FIELDS` was thirteen; now fourteen (the no-`expanded`-key, actionable-key and probe click-reading rows).
  - The actionable-key row and the probe click-reading row: was no snapshot field added, owed to the route entry; now `NodeState.clickable` holds that reading.
  - New row: `stand_non_lean_tasks` asserts a `CLICKABLE` table in both layout modes — Counter, Temperature Converter, Flight Booker, Timer and CRUD 0 each, Circle Drawer 1, Cells 676 — beside the listener mark's Cells 677.
  - New row: on the stand `clickable` reads true on `counter-increment` and on Cells' 676 data cells, false on `counter-value` and `cells-grid`.
  - The listener over-report row: the bridge's vdom walk yields each element's current listeners (`element_listeners`), the reading `clickable` and the graph's handlers take, so neither over-reports as the mark does.
  - The CI row: link 2's reading named as the field `NodeState.clickable` that `stand_graph` and `stand_non_lean_tasks` read; the a11y leg and its job unchanged.
why: the report states `NodeState.clickable` built with its rule, `NODE_FIELDS` 13 → 14 and `Reading::ALL` 11 → 12, the `CLICKABLE` table new, and the stand's readings; the four `click-field` rows it names stale, residual `res-gvelrxja`'s disposition met — the field the founder's C3 (2026-10-14) deferred to the route entry «The UI graph», now built by it.
kept: `DioxusDocument::unkeyed_actionable` still reads the listener mark and over-includes (conservative); the mark's Cells count stays 677.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: the read-only UI graph reader DioxusDocument::graph
section: body §7 Screen Reader Support (the snapshot-model line); §2 A11y Strategy (a11y-strategy.md: the `accessibility` feature's gated set); §7 Screen Reader Support (screen-reader-support.md: two new rows)
change:
  - The snapshot-model line: the read-only UI graph (`DioxusDocument::graph`) holds one node per snapshot node, names and closed words only, with no second role or name mapping.
  - The gated set: was five items its crate doc names; now also the graph reader (`DioxusDocument::graph`, a `UiGraph`, its `graph` module), while the `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated.
  - New row: `DioxusDocument::graph`, behind `accessibility`, a `UiGraph` holding one `GraphNode` per snapshot node in pre-order — id, `owner` (an option's or optgroup's select), component, handlers with their declared writes, the states it reads — and per registered state its declared writes, the writes a run confirmed or flagged (`undeclared`, `unconfirmed`) and its reading components; ids, component, state and handler event names, closed flag words and booleans only, never a value.
  - New row: `stand_graph` asserts in both layout modes that every graph node is a snapshot node, that no served id holds the reserved separator `///` (the reserved `{owner}///{key}` form, spelled by `virtual_node_id`, which the walk never calls), and that a graph read changes nothing and holds no typed text.
why: the report's Symbols / APIs and Crates / modules state the graph reader, its shape and its gating, and its Outcome the names-only answer and no served id holding `///`; a state's readers as components and its writers as confirmed or flagged writes are the operator's two corrections in the implement session (2026-11-08).
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: cited source lines re-pointed after the chunk's edits
section: §2 A11y Strategy (a11y-strategy.md); §8 Cognitive Accessibility (cognitive-accessibility.md); §5 Keyboard Navigation (keyboard-navigation.md); §7 Screen Reader Support (screen-reader-support.md); §6 Visual Design Verification (visual-design-verification.md)
change:
  - Rows citing the chunk's changed files re-pointed to the lines their cited code now holds — the seven_guis tasks, dioxus-native-dom's `lib.rs`, `dioxus_document.rs`, `snapshot.rs` and `snapshot_diff.rs`, escher-driver's `execute.rs`, `json.rs` and `schema.rs`, the stand's test files and `docs/capabilities.md`; no claim's text moved.
why: the report's Changes lists those files edited by the chunk, so the lines they cite shifted.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
@amendment marker=2026-11-07-ui-graph
title: graph state registration and the bridge dispatch's write record
section: §5 Keyboard Navigation (keyboard-navigation.md: a new row beside the keyboard-events row)
change:
  - New row: apps register state for the UI graph through dioxus-native-dom's `use_graph_state` with authored `DeclaredWrite` declarations — an element's stable id or a prefix ending in `*`, and an event named without `on` — wrapped in seven_guis as `use_graph_signal`.
  - Where a `SignalRegistry` is registered, `DioxusEventHandler::handle_event` sets the registry's dispatch (chain and event) around `runtime.handle_event` and clears it after; each signal write is recorded against that dispatch and the writing scope, resolving to the first element of its chain carrying a listener of its event, to nothing where a node of the chain before it no longer resolves.
  - A write reads `undeclared` where no declaration names it, a declared write `unconfirmed` only once a dispatch of its event passed through its element and no run wrote the state; a page with no registered state reaches no registry and dispatches as before.
why: the report's Symbols / APIs states the registry, the dispatch inert without a registry and the resolution rule, its stop at a gone node measured by `stand_graph`; the authored declarations are the founder's P5 answer, given by dialog in the overseer session and relayed by the operator (2026-11-08), and `unconfirmed`'s condition the operator's word at P5.
ref: .andromeda/runs/2026-11-08T02-04-13-wrap/
