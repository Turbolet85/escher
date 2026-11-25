@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the origin of `contract/accessibility-tree/author-id` (every element node of a Dioxus document carries its stable id as AccessKit `author_id`, no TextRun / document-root / Window node does), the name sources (`aria-label`, `<label>`) and the shell building the tree through `Document::accessibility_tree` so a wrapper's override reaches it (`decision/a11y-tree-through-document-trait`). Matters: a virtual node with no DOM behind it is a new kind of tree node; whether and how it carries an author_id (`{owner}///{key}`) must be squared with this contract, and the commented `Widget::accessibility_tree` sits beside the trait path this entry fixed. Trap recorded there: the windowed stand ships no AccessKit adapter.

@history_item n=2 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the founder's actionable-key rule: every element an agent can act on reads an author key, checked by `DioxusDocument::unkeyed_actionable`, role taken from the tree with no second mapping. Matters: the virtual node's author key is the same idea one level down; its trap — never satisfy the check by adding a `tabindex` or role — and the rule that keys are authored, not derived, bind the declaration's key rules.

@history_item n=3 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot consume the accessibility tree's role, name and focus with no second role or name mapping. Matters: the root of `decision/snapshot-model-single-source` and `decision/readers-of-one-snapshot-model`; a closed role set declared on a virtual node must map onto the tree's roles, not become a parallel role/name mapping when «Virtual nodes as elements» later brings the nodes into the tree and snapshot.

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that the driver's schema defines no role or name set of its own; `role` and `name` are field names of its result nodes (row `decision/driver-schema-has-no-role-set`). Matters: the chunk's «role: a closed set» must not land as a driver-side role set; the fork on where the declaration lives (engine `Widget` vs Dioxus renderer) should keep the role set on the engine/tree side.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the tree refresh on change and defined what the engine's changed set marks (mutation, focus/checked, text input; not creation, hover, active, scroll, layout); the diff became a reader of the snapshot model. Matters: the «list at each paint vs callbacks with invalidation» fork decides whether a changed declaration marks the changed set; per `decision/changed-set-marks`, a paint alone marks nothing today, so a paint-time list would never refresh the tree unless the set is marked.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract (were PROVISIONAL). Matters: these are ratified contracts (`decision/a11y-tree-refresh-on-change`, `decision/changed-set-marks`, «one item»); any widening of what marks the set for virtual-node declarations is a change to a ratified row, a founder fork.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom, so both seven_guis binaries compile the tree override, snapshot and diff, neither builds a platform adapter. Matters: background for `decision/accessibility-feature-default-on` / `decision/agent-readers-behind-accessibility` and `absence/assistive-technology-reach` — a virtual-node reader or tree hook placed behind `accessibility` (or `custom-widget`) reaches the stand through this edge; no AT reads it.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP tool results became a reader of the snapshot model, on the crossing the founder ratified 2026-10-10. Matters: the names' crossing fork rests on the same 2026-10-10 rule (authored text on a ratified carrier is a widening); this entry is the precedent for how a snapshot-model reader's crossing was ratified.

@history_item n=9 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named) — retired the unnamed-list gap: every interactive-role node of the four lean tasks is named over nine roles, and the `&dyn Document` tree equals the document's, carrying stable ids only. Matters: origin of `decision/interactive-nodes-named` (guard `stand_accessibility_ids`); the scope's «name: non-empty» rule is the same invariant, and a virtual node with an interactive role must not break it once it enters the tree.

@history_item n=10 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable` reads 0 on Circle Drawer too; the click-without-role count pins Circle Drawer at 1. Matters: Circle Drawer's canvas is the first virtual-node host («Circle Drawer's circles as virtual nodes» is a later entry); its pinned counts are the baseline this chunk must leave unmoved.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role) — mapped `role="switch"` to `Switch` and read `aria-checked` on the five checkable roles alone (ratified 2026-10-18). Matters: the provenance of `contract/aria-role-mapping/role-switch`; a closed role set for virtual nodes that includes checkable roles meets this per-role state read.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes `focus` only where the focused node is among the built nodes, else the window, so the tree never names a node it did not build. Matters: a trap for nodes with no DOM behind them — virtual nodes are not engine nodes, so tree focus on one needs a deliberate rule (owned by the keyboard-path entry), not this fallback by accident.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list; how an open list's labels leave) — a row's own `bounds` read as a closed set inside the ratified crossing by the 2026-10-10 field rule; no `expanded` node key as the operator's call; option and group labels leaving only as pixels and the snapshot's `name`, ratified as a widening. Matters: the nearest precedent for engine-made nodes without their own DOM box (rows with their own bounds) and for authored text crossing as a widening — the shape the names' crossing fork and the region rule will be judged against.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader; the clickable field) — `DioxusDocument::graph` holds one node per snapshot node, names and closed words only; `stand_graph` asserts no served id holds the reserved separator `///` (the `{owner}///{key}` form spelled by `virtual_node_id`, which the walk never calls); `NodeState.clickable` added. Matters: origin of `contract/node-id/reserved-separator` and `decision/ui-graph-one-node-per-snapshot-node` / `decision/ui-graph-carries-no-values`; this chunk is the first to produce `virtual_node_id` ids, so the «no served id holds ///» guard must stay true while nothing serves them, and the graph's reach is owned by «Virtual nodes in every read».

@history_item n=15 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the bridge dispatch's write record) — apps register state through `use_graph_state` with authored `DeclaredWrite` declarations, an element's stable id or a `*`-ended prefix. Matters: precedent for an authored declaration API in dioxus-native-dom keyed on stable ids — relevant to the fork of where the virtual-node declaration lives (engine `Widget` vs Dioxus renderer).

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — one actionable rule `Readings` (focusable, interactive role, or current listeners) read by `unkeyed_actionable` and `actionable_nodes`; `ActionableNode.keyed` true where the id holds no `/`; the `actions` verb writes ids, owner ids, verb and cause words and booleans only; `ACTIONABLE_AT_BOOT` pins Circle Drawer's tasks' counts. Matters: the scope's «author key holding no `/`» mirrors `keyed`'s reading, `owner` is the precedent for `{owner}` in the reserved form, and `contract/actions-answer/fields` plus the boot counts must stay unmoved since this chunk adds no carrier.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler_coverage, a further reader of the UI graph) — `escher_driver::handler_coverage` reads the graph and answers ids, owner ids, event names and two counts only, no role or name. Matters: origin of `contract/handler-coverage/answer`, a row this chunk's declared names must not reach (its reach is «Virtual nodes in every read»).

@history_item n=18 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `decision/keyed-wire-form` ratified (the actions answer's `keyed` field). Matters: the `keyed` reading (id holds no `/`) is a standing wire form, so a virtual node's id `{owner}///{key}` reads unkeyed by today's rule — a conflict the later reads entry inherits and this chunk's id form should foresee.

@history_item n=19 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select. Matters: one link in the changed-set row's chain of widenings; each mark added so far was an operator/founder ruling, the precedent for any mark a callback-with-invalidation declaration would need.

@history_item n=20 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed again in a redraw after style resolve only when the changed set is non-empty, then cleared; a poll pushes no tree. Matters: the current refresh path a declared virtual node would ride — a paint-time declaration made after the resolve that drives the push would not reach the tree until the next marked redraw.

@history_item n=21 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (2. A11y Strategy) — `~ decision/changed-set-marks`: a list's highlight moving rows marks its select. Matters: precedent that engine-internal, non-DOM sub-items (list rows) mark their host element in the changed set — the likely model for virtual nodes marking their canvas.

@history_item n=22 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (2. A11y Strategy) — `~ decision/changed-set-marks` to its current text (post-layout focus loss marks too; «layout alone do not»). Matters: the row as the picked rows hold it; a declaration-driven mark must be added to this ratified list by ruling, not silently.

@history_item n=23 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `+ decision/circle-drawer-canvas-pixels-only`: Circle Drawer's drawing area is one canvas node read by role and name alone, its circles reaching no DOM, snapshot, tree or UI graph. Matters: the row virtual nodes will eventually amend; this chunk declares nodes in process only, so the row's «reach no … accessibility tree or UI graph» must still hold at its wrap.

@history_item n=24 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `decision/circle-drawer-canvas-pixels-only` ratified by the founder, state standing. Matters: a ratified row; changing what leaves the canvas is a founder fork, consistent with the scope sending the names' crossing to P4.

@history_item n=25 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — `+ absence/keyboard-drag-and-drop`: drag and drop is pointer-only with no keyboard or AT route. Matters: an absence in the picked rows' scope area, and the drag-and-drop security gap is carried freight on this chunk's wrap.

@history_item n=26 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `~ decision/circle-drawer-canvas-pixels-only` (labels leave as canvas-verb draw commands, no text), `~ decision/readers-of-one-snapshot-model` and `~ decision/snapshot-model-single-source` (the canvas read is no reader of the snapshot model). Matters: the latest canvas readers; a declared-node reader is a new in-process read that must say whether it is a snapshot-model reader, and «no label's text» is the bar the names' crossing fork meets.

@history_item n=27 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `~ decision/circle-drawer-canvas-pixels-only` to its current text (labels reach the lint as truncated boxes, no text); PROVISIONAL on the escher overseer's delegate confirm, the founder away. Matters: the row stands provisional at this chunk's base; any edit to it here stacks on an unratified operation.

@history_item n=28 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the windowed AT witness owned by residual `res-llkgqygv` (an active assistive technology this version does not reach); `docs/capabilities.md` publishes the accessibility reach by hand. Matters: background of `absence/assistive-technology-reach` — what an AT reads of virtual nodes stays unmeasured, and a new reach is written into `docs/capabilities.md` by hand.
