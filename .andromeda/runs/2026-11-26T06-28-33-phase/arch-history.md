@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — (amd-4ls4ci6z) `Document::accessibility_tree` became a defaulted trait method that `DioxusDocument` alone overrides to write each node's `author_id`, and the shell builds the platform tree through `&dyn Document`. This is the crossing (contract/document-trait/accessibility-tree) any write of a virtual node's tree `focus` or `selected` must go through. Trap: a feature-gated item can pass `cargo test --workspace` and still be compiled out under `-p blitz-tests`.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — (amd-glb73szz) the snapshot derives no role or name of its own, so it cannot disagree with the accessibility tree. Its `focused` is the `TreeUpdate`'s `focus` and never reads `get_focussed_node_id`, which falls back to the root (contract/snapshot/focused). The snapshot's `focused` on a circle can only come from the tree's focus naming that virtual node, never from the engine's `focus_node_id`.

@history_item n=3 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — (amd-7zhm5mac) added the changed set, marked by focus/checked writes and never by creation, hover or layout; the shell refreshes on change; `Snapshot::diff` is not derived from the changed set. This is the base of decision/changed-set-write-sites and decision/diff-not-engine-changed-set. A current-circle move or a `selected` write that marks nothing reaches the windowed adapter only at its next rebuild, though the diff still names it.

@history_item n=4 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — (amd-oqwmbksj) a `click` or `type` is refused in the order `disabled` · `off-screen` · `covered`, a refused call runs nothing, and visibility is read from geometry, never a hit (decision/visibility-from-geometry-not-hit). This is the frame for the scope's owed `covered`, `off-screen` and `disabled` readings on a circle.

@history_item n=5 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (amd-fuzh25ns) the seven_guis stand checks state their shared tables and helpers once, in `tests/blitz-tests/tests/common/mod.rs` read by `mod common;` (decision/shared-test-modules). Circle Drawer's Tab order joins `tab_order`, `ENABLED_TAB_ORDER` and the other tables there, not in its own check.

@history_item n=6 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (amd-hbm2vwj6, amd-lqlrwp3b) `NodeState.selected` became the accessibility node's own `is_selected()`, never parsed from the element. The builder writes `selected` from `aria-selected` as a closed bool on the adapter crossing (decision/a11y-aria-states, decision/adapter-crossing-carries-states). A declared `selected` on a circle must be written on its tree node so that the snapshot reads it.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — (amd-r7r6sjir) `DiffNode` gained `differs: Vec<Reading>` in `Reading::ALL` order, crossing as a closed set (decision/differs-closed-set, contract/snapshot-diff/differs). This is the reading v040-11's click clause rests on: a circle click settling with `differs: [selected]`.

@history_item n=8 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — (amd-c5o72f5a) precedent: arrows and Home/End on a focused control became a direct engine change for every document. It was flagged upstreamable, its merge surface owned, one `input` per change and none for a no-change move. It is a nearby precedent for keys moving a current item inside one focused element.

@history_item n=9 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (amd-oyjgj5od, amd-efxr6uek) Enter (pressed) and Space (released) dispatch a queued Click on the focused node's listed control kinds, so app handlers run before the default action (decision/keyboard-activation-enter/space, contract/event-dispatch/enter-click). It added `stand_keyboard` and `tab_order` in the shared module. A focused canvas is outside both key lists, so a circle's keyboard act is the app's own key handler, not engine activation.

@history_item n=10 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (amd-3bta3wci) `stand_keyboard` joined the `a11y` leg, and `ENABLED_TAB_ORDER` and the enabled-state walks were added. Widening Circle Drawer's walk changes an `a11y`-leg target.

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (amd-fq2cbktu) Circle Drawer was keyed: `circle-canvas`, `circle-undo`, `circle-redo`, `circle-diameter` and `circle-close`, with `unkeyed_actionable` empty on all seven tasks. The canvas id the Tab walk and the virtual ids `circle-canvas///{key}` rest on.

@history_item n=12 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — (amd-rvwyylpw) set the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure `stand-repeat` leg (decision/per-crate-coverage-floors, decision/coverage-floors). New tree-focus and `selected` code in dioxus-native-dom and any `press` arm in escher-driver must hold them.

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (amd-qi5jxqm4, amd-mvbqmia6) keyboard-types 0.8 beside Dioxus's 0.7. A module that glob-imports the Dioxus prelude names the workspace crate `::keyboard_types` (decision/keyboard-types-named-by-absolute-path). The driver's key types come through the harness's `Key`, `Modifiers` and `NamedKey` (decision/key-types-through-harness). A trap for a key handler added to `examples/seven_guis/src/tasks/circle_drawer.rs`.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (amd-ha6swk4r, amd-qdhfofev, amd-2p747sk7) `inert` and `disabled` were read on form controls alone through `ElementData::is_disabled`, so a `<div disabled tabindex=0>` takes focus. It added `accessibility_active_descendant_unread`, which pins that the engine reads no page `aria-activedescendant`. A focusable canvas with a tabindex follows these focusability rules. The tree-focus-on-a-virtual-node question meets this standing limit, which the scope keeps unless P4 forks.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (amd-3rlakvkd, amd-wxdpki7i) the double-click window (500 ms, 2 px) was read on the document's clock (decision/repeated-click-window, decision/double-click-window). This bounds how the diameter dialog, reached today by a double click, is reached headlessly, and what a keyboard path to it must replace.

@history_item n=16 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — (amd-a2xisf3l) precedent: a focused element whose keys (arrows, Home/End, typeahead) move a current choice among its children, the choice as the option's `selected`. The changed set is marked through one path (`apply_select_choice`), and a click focuses it. This is the closest engine-side model for a current circle among a canvas's virtual nodes.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (amd-qch7quow) `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else the window (contract/accessibility-tree/focus). Making a virtual node the tree's focus amends this ratified engine change, for every document. (amd-2c27hy5n) Cells' `cells-grid` is the stand precedent: one Tab stop (`tabindex` 0, an `onkeydown`), arrows move `current`, Enter acts, focus returns to the grid. (amd-hkbudmim) `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` joined the shared tables. (amd-t5spowmn) The inert focus fixup fires no blur.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (amd-be23sewu, amd-iqidwpoy) precedent: the tree's focus stays on the select while a current row moves. The highlight was first absent from the snapshot and tree, with no `expanded` node key added. Tab chooses the current row before focus moves. A current item inside a focused element is a crossing question settled by the founder, not a default.

@history_item n=19 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (amd-ogtxzreb, amd-7yesshpe) measured the MountedData hazard of a task a key handler spawns, and found that the bridge's listener mark over-reports, so readers use the vdom walk (decision/listeners-read-from-vdom, decision/listener-mark-outlives-listener). A new `onkeydown` on the canvas is read by the graph, `clickable` and the actionable rule through the vdom walk.

@history_item n=20 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (amd-7nqjoojl, amd-62mpv4th, amd-5ibsrl24) the bridge's buffered render lets a key-handler-spawned task's focus act (decision/dioxus-bridge-buffered-render). Its witness asserts the engine's focus, the snapshot's one `focused` node and the tree's focus agree. Limit: a node created in the same render cannot take focus.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (amd-rb37miab) `clickable` is read from the current `click` listeners; Circle Drawer reads 1. (amd-ol72vjyk) `GRAPH_DRIVES` drives each conditional write once. (amd-kxxabpvh) async handler writes land inside their dispatch. (amd-nkoocfjo) tasks register state through `use_graph_signal` with declared writes. A canvas key handler writing the selection needs declared writes, or `stand_graph` reads a flag, and `GRAPH_DRIVES`' first click selecting under B1 is re-read.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (amd-ji7fe5g2, amd-hybaqglx, amd-gcta5ora) one actionable rule: focusable, an interactive role, or current listeners other than `mounted`. `ACTIONABLE_AT_BOOT` is pinned per task, and the shared per-verb target check `check_target` runs the `disabled`/`off-screen`/`covered` order. Making the canvas focusable and keyed moves no actionable count by itself, but the circles' cause readings go through this one check.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (amd-qch5rru6, amd-puf6ljhv) `handler_coverage` reads the UI graph's handlers at boot and after each replayed step, and the reached record is written on the bridge's dispatch. A new `keydown` handler on `circle-canvas` enters the handler tables (`HANDLERS_AT_BOOT`, `UNEXERCISED_AFTER_DRIVES`) of `stand_handler_coverage`.

@history_item n=24 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — (amd-d2u5nzon) the capability statement's pin also holds the `a11y` leg's file count and `stand-repeat`'s named-target count to the CI leg runner (decision/capability-statement). The scope's sweep of `docs/capabilities.md` sizes and counts must keep those pinned numbers true.

@history_item n=25 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — (amd-sqsx4bdo, amd-cm7pmnn3) a focus change, a programmatic one included, dispatches blur, focusout, focus, focusin (contract/event-dispatch/focus-change). Events are queued on the document (decision/events-queued-on-document). The platform tree is refreshed on a redraw after resolve when the changed set is non-empty (decision/accessibility-tree-refresh-on-change). A current-circle move that sets no focus and marks nothing never reaches a windowed adapter.

@history_item n=26 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — (amd-bbzjg57c, amd-sf34us4u, amd-usx36x6v, amd-eaip5l6g) the closest precedent for agreeing on a current item while focus stays on its owner. The tree names an open select's current option as the select's active descendant, the tree's focus staying on the select (contract/accessibility-tree/active-descendant). The snapshot reads it as the new closed bool `highlighted` (a fifteenth node key, `Reading::Highlighted`). A highlight move to another row marks the changed set (decision/select-list-changed-set). The crossing row was amended for the new node key.

@history_item n=27 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — (amd-c7krfqdn) `press` takes `key` with optional `shift` and `ctrl` and presses on what holds focus (contract/driver-verbs/press). A driver `press` on the focused canvas reaches its key handler this way. (amd-dau4d5qr) The reached record is resolved before dispatch and written after.

@history_item n=28 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — (amd-wypvy775, amd-2ze4iwis, amd-sdo6apzw) added focus fixups under inert and after a layout for a node that no longer shows, and the Tab walk skipping what does not show. These were tallied in decision/hit-focus-form-engine-changes, with focus-change contracts listing the silent paths and the changed set marking a focus given up after layout. Any engine change to the tree's focus joins this tally and these lists.

@history_item n=29 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — (amd-amrxdkmf, amd-skivzijy) the engine-change tally row was amended for the list box, and later restored when a proposer re-worded clauses it did not mean to move. `decision/click-focuses-button`'s except grew. Trap: amend the tally and except rows by one clause, not a rewrite.

@history_item n=30 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — (amd-afucsrvs) a fork addition to an upstream function over the complexity ceiling goes into a function of escher's own, with one call in upstream's body (decision/fork-additions-in-own-functions). This applies to any focus or `selected` addition in `build_accessibility_node` or `handle_key_or_input_event`.

@history_item n=31 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — (amd-j2blpjme) the harness turns on blitz-paint's `custom-widget` feature, so shots paint the widget (decision/harness-paints-custom-widgets). The 7GUIs selection repaint (the chosen circle filled) shows in every harness and driver shot.

@history_item n=32 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** — (amd-bs66iouj, amd-pbnf5rg7) a canvas-only shot is held byte-equal on linux, windows and macOS by one committed digest table (decision/no-screenshot-in-cross-os-byte-check except). A selection fill that changes Circle Drawer's painted canvas moves those committed pins, which the scope says are swept.

@history_item n=33 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — (amd-d7nvxvcb) on a custom-widget target the engine synthesises click and double click with no click walk and no focus clear (decision/widget-click-synthesis). That is decision/click-focuses-button's except, so a click on the canvas does not focus it today. An app reads its click point against the content box (decision/widget-click-point-content-box). (amd-bbwqcfh7) Circle Drawer's boot is in process only and served by no escher-session role (contract/headless-stand/boot).

@history_item n=34 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (amd-5dy5wmfg, amd-ex3tdh4v) a canvas's content crosses as the `canvas` verb's draw commands beside pixels (decision/canvas-crossing). The reading is deterministic and diffable by line (decision/canvas-reading-diffable), and `canvas` is a target verb in actions. A selection fill changes the committed command pins of `stand_canvas_commands` and `stand_canvas_cross_os`.

@history_item n=35 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — (amd-cbkr7gme) added `Widget::virtual_nodes` and `VirtualNodeDeclaration` in `packages/blitz-dom/src/node/custom_widget.rs`, and `DioxusDocument::virtual_nodes` with its refusals. These are the declaration and read the scope extends with `selected`, and its 8 literals in 4 files take the new field. (amd-tsnaceip) A virtual node's role is AccessKit's Role, with no escher role set (decision/virtual-node-role-is-accesskit-role, provisional).

@history_item n=36 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — (amd-vghrwc6q) the tree holds a canvas's taken virtual nodes (contract/accessibility-tree/virtual-nodes). The snapshot reads a virtual node with fixed fields: `focused` false, `selected` none, `clickable` false (contract/snapshot/virtual-node, F12 A). `click` acts at the virtual region in the order disabled, off-screen, covered, not-found, stale. `covered` on a virtual node reads only an element over its region's centre. (amd-mgko66ha) The declaration gained `disabled`, and canvas placement became the click centre or a declared region. (amd-twlyipmu, amd-6g7smx3e) A declaration change marks nothing (decision/declaration-change-marks-nothing). Every row the scope's `selected` and focus readings amend.

@history_item n=37 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** — (amd-urtw5kif, amd-ajm2syrc, amd-bkkbcfqi) the virtual id is served in the snapshot, diff, graph, actions and layout. The verbs taking a virtual id come from one table keyed on the verb (decision/virtual-id-verb-table, provisional); today that is click and layout, other id verbs reading it not-found. The graph owner of a virtual node is its canvas. Any widening of which verbs act on a virtual node (`press` included) goes through this one table.

@history_item n=38 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** — (amd-e5o6c5c5) a virtual node's click is listed in handler coverage under its `{owner}///{key}` id, owned by the canvas (contract/handler-coverage/canvas). `VirtualNodeRole` is re-exported from the blitz-dom crate root. (amd-oi4lfxc3, amd-ev4j4wms) draw commands are grouped under the virtual node holding their centre, each line closing on a node tag (decision/canvas-command-grouping, decision/canvas-reading-diffable). This chunk carries that chunk's CARRY (F1 B1 «выделение полем»; F4 B open); a selection fill re-reads both pins.
