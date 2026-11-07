@carried n=1 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the buffered render's replay must still drop a removed node with its descendants and clear both mappings, so a removed node resolves to nothing, as stale_node_mapping re-verifies.
quote: a node a re-render removes is dropped with its descendants by the bridge's crate-private `remove_and_drop_node` / `replace_and_drop_node`, their `element_id_mapping` / `node_id_mapping` entries cleared, so it no longer resolves and reads no id — `get_node` `None`, `element_id` `None`, no pair in `element_ids()` (packages/dioxus-native-dom/src/mutation_writer.rs:110-130; packages/dioxus-native-dom/src/element_id.rs:397-423) — upstream #1185, as measured at escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md;
@carried n=2 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the buffer may live on DioxusState as upstream's does, and try_element_to_node_id, read by stale_node_mapping and by the mounted-event guard, must answer the same after a buffered render.
quote: `DioxusDocument` exposes `vdom`, `inner`, `vdom_state.try_element_to_node_id(ElementId)`, which the guard of `flush_queued_mounted_events` (below) also reads (tests/blitz-tests/tests/stale_node_mapping.rs:66; tests/blitz-tests/tests/stale_node_mapping.rs:76-80)
@carried n=3 label=architecture/standard-contracts fact=fct-kllkxz3d
why: mutation_writer.rs must stay byte-identical to upstream at 434e0661 where the port allows, the state this contract records.
quote: the 27 names before `inert` the module-private `BOOLEAN_ATTRIBUTES` list of escher's own, retired at 2026-10-13-upstream-sync-polish-and-ship for upstream's predicate (#1188), which left `mutation_writer.rs` byte-identical to upstream's at `434e0661`
@carried n=4 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the stable ids the chunk re-verifies through stand_* must read the same after a buffered render, computed on demand and written to neither DOM nor vdom.
quote: **stable element ids** — `DioxusDocument::element_id(NodeId) -> Option<String>` (`None` for a non-element, stale or detached node, never panics) and `element_ids() -> Vec<(NodeId, String)>` (every element reachable from the document root, document pre-order, ids pairwise distinct) compute the id on demand from a vdom walk joined with a DOM walk and write it to neither the DOM nor the vdom
@carried n=4 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: the id **persists** on the stand, in both layout modes and on each lean task: every element present before and after a re-render reads the same id;
@carried n=4 run=3 label=architecture fact=fct-srb7slak
quote: **Dioxus DOM bridge:** DOM events reach the vdom through `data-dioxus-id`; a falsy value of upstream's 28 boolean attributes clears the attribute; `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=5 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the snapshot and its diff are re-verified unchanged after the buffered render, the diff equal across both layout modes.
quote: under the same feature `DioxusDocument::snapshot(&self) -> Snapshot` reads the screen as a tree, read-only: it builds from that `accessibility_tree` override and walks its `children` lists from `TreeInfo.root`;
@carried n=5 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: on the stand each step's diff is equal across the two layout modes (packages/dioxus-native-dom/src/snapshot_diff.rs:108; packages/dioxus-native-dom/src/lib.rs:39-40; tests/blitz-tests/tests/stand_diff.rs:1-12)
@carried n=5 run=3 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=6 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the replayed listener mutations must still write the placeholder and the data-dioxus-id mark that DOM event routing reads.
quote: DOM events route to the vdom via the nearest `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:47-55; packages/dioxus-native-dom/src/dioxus_document.rs:415-421); listener registration sets a `"<rust func>"` placeholder and `data-dioxus-id` (packages/dioxus-native-dom/src/mutation_writer.rs:338-345);
@carried n=7 label=architecture/standard-contracts fact=fct-ie5oemgh
why: the replay must reproduce create_event_listener and remove_event_listener exactly, the over-report included, which a dioxus_runtime_probe test the chunk re-runs witnesses in both layout modes.
quote: the bridge's listener attributes — an attribute named the event, valued `"<rust func>"`, beside the `data-dioxus-id` mark, written by `create_event_listener` — over-report: a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none, in both layout modes (packages/dioxus-native-dom/src/mutation_writer.rs:331-352; packages/dioxus-native-dom/src/mutation_writer.rs:355-360; tests/blitz-tests/tests/dioxus_runtime_probe.rs:387-429).
@carried n=8 label=architecture/standard-contracts fact=fct-67g4co3e
why: the recorder must capture every 0.7 WriteMutations call dioxus-core makes, set_attribute and the listener calls alike, and the replay becomes the workspace's caller of create_event_listener and remove_event_listener.
quote: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener attribute leaves an element's attribute list (a spread); a listener whose value turns to `AttributeValue::None` under the same name is written through `set_attribute` as an `on{event}` attribute, so the bridge's event attribute stays either way (dioxus-core 0.7.10 `diff/node.rs:416-536`, read in the registry copy); `create_event_listener` and `remove_event_listener` have no workspace caller, dioxus-core calling them.
@carried n=9 label=architecture/standard-contracts fact=fct-llnppo2j
why: flush_queued_mounted_events stays unchanged and runs after the replay, so its removed-element skip must keep holding over mappings the replay wrote.
quote: `flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
@carried n=10 label=architecture/standard-contracts fact=fct-2ylcpw3b
why: the Cells grid calls MountedData::scroll_to on handles its onmounted listeners hold, so the mounted events' order and the handles must survive the buffered render.
quote: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles
@carried n=11 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the new mutation_buffer.rs and its ported test land in dioxus-native-dom, whose coverage floor of 83 % the coverage leg must still meet.
quote: the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`;
@carried n=11 run=2 label=architecture fact=fct-scmruv42
quote: `coverage_floor.py` the per-crate coverage floor;
@carried n=12 label=architecture/standard-contracts fact=fct-jelmr2gx
why: the stand checks the chunk re-runs as its witness run three rounds under stand-repeat's zero-failure budget, so the buffered render must not make any of them flaky.
quote: the first red command ends the leg with its status — a zero-failure budget, no retry, no `--ignored`, no env var — and three green rounds end `stand-repeat: 3 of 3 rounds green`;
@binding n=1
**Dioxus DOM bridge ↔ the chunk's buffer and replay**: the buffer replays 0.7's WriteMutations calls in order into the existing MutationWriter, so the node drop with both mappings cleared, the stable ids and the snapshot/diff readings stay as they are (architecture/standard-contracts fct-kllkxz3d), and the mounted-event guard reading try_element_to_node_id after the replay keeps skipping a removed element (architecture/standard-contracts fct-llnppo2j).
@binding n=2
**Listener contract ↔ dioxus-core 0.7 WriteMutations**: the recorder captures set_attribute, create_event_listener and remove_event_listener as dioxus-core issues them (architecture/standard-contracts fct-67g4co3e), and the replay reproduces the bridge's listener attributes and over-report that the probe re-checks in both layout modes (architecture/standard-contracts fct-ie5oemgh).
@binding n=3
**Mounted handles ↔ MountedData calls**: the onmounted handles the stand holds (architecture/standard-contracts fct-2ylcpw3b) are produced by flush_queued_mounted_events after the replay (architecture/standard-contracts fct-llnppo2j), the same handles the ported test and the inverted pins call set_focus on.
@binding n=4
**CI contracts ↔ the new module and the witnesses**: mutation_buffer.rs counts against dioxus-native-dom's coverage floor (architecture/standard-contracts fct-jelmr2gx), and the stand_* witnesses run under stand-repeat's zero-failure budget (architecture/standard-contracts fct-jelmr2gx).
