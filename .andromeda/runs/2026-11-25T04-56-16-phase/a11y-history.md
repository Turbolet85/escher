@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — (name sources, carried stable id) every Dioxus element node carries its stable id as AccessKit `author_id`, no TextRun / document-root / Window node does (contract/accessibility-tree/author-id), and the shell builds and refreshes the tree through `Document::accessibility_tree` so a wrapper's override reaches it. A virtual tree node is a fourth kind of node the author-id loop must recognise (F7 A: the loop checks the virtual map first), and it must enter through that same override.

@history_item n=2 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot became a consumer of the accessibility tree, taking role, name and focus from it with no second role or name mapping. This is the basis of decision/snapshot-reads-accessibility-tree and decision/snapshot-reads-the-tree, and so of reading a virtual node from the tree rather than from its declaration.

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule: every actionable element (focusable, interactive role or listener) reads an author key, checked by `unkeyed_actionable`, which takes the role from the tree. Trap: never satisfy it by adding a tabindex or a role. A virtual node with an interactive role in the tree must not trip the check or be held as unkeyed, and its `{owner}///{key}` id is not an element key.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` (the presence of `disabled`), `checked` and `value` from the element because the tree gives an element none of them. A virtual node has no element, so F8 A's declared `disabled`, written on the virtual tree node and read as `enabled`, is a new source for this reading that has to be stated beside the element rule (contract/snapshot/enabled).

@history_item n=5 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` is a reader of the snapshot model alone, one line per snapshot node, role and name the node's. Virtual nodes put in the model reach the text form, and through it every carrier that F3 A widens, by this rule.

@history_item n=6 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree is refreshed on change from the engine's changed set, and `Snapshot::diff` became a reader of the snapshot model: a text change is reported on the element whose name it changes, never as a text-run entry. This is the origin of decision/changed-set-marks and contract/snapshot-diff/entries, which the virtual-node diff (added, removed or changed by id) extends. F11 A keeps declarations unmarked.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract (decision/a11y-tree-refresh-on-change, decision/changed-set-marks). F11 A is the founder's word not to amend these ratified rows for declaration changes; the row stays as ratified.

@history_item n=8 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a fresh harness document holds a non-empty set, so a check reading the set as one step's change drains it first (decision/harness-settle-leaves-changed-set). This bears on any check that a declaration change marks nothing, and on the virtual click's settle-then-diff.

@history_item n=9 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` cause is worded on the snapshot's presence reading. Rule: the entry that detects `disabled` reads the snapshot's `enabled`, never an attribute value. The virtual node's `disabled` refusal must come from its snapshot `enabled`, fed by the declared bool, and not from a separate check.

@history_item n=10 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `disabled` is detected in `Session::run` on `enabled == Some(false)`, the first screen-level check, with nothing dispatched and focus as it was (contract/driver-refusal/disabled, contract/driver-actions/disabled). This fixes the order and no-dispatch rule that the virtual node's refusals (stale / not-found, then disabled, off-screen, covered) must keep.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (the tree states selected, toggled and invalid) the builder writes the states on the node and the snapshot takes them from the tree, never from the element; one parser per attribute keeps the snapshot and the tree in agreement. This is the precedent for writing the virtual node's state (declared disabled) on its tree node and reading it from there.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result reads the snapshot model and nothing else, and a `snapshot` block equals `Session::run`'s screen text (contract/mcp-result/snapshot). Under F3 A virtual nodes reach MCP only through this path, with no new carrier.

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** — the node keys' help and `DiffNode::differs` are readers of the model, closed words only. `differs` names readings in node-key order (contract/snapshot-diff/differs), and that order applies to a changed virtual node. No node key is added.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` reads the model's bounds: an element shot by stable id is its bounds rounded outward and cropped, and a shot changes nothing (contract/driver-screenshot/element-shot). F10 B makes `screenshot --id` on a virtual id read `not-found`, a stated limit against this contract.

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (`disabled` by presence) disabled-ness is keyed one way, by the attribute's presence (founder's word). The sibling entry has Enter and Space dispatch a synthetic Click (decision/synthetic-click-at-node-centre). These are the origins of decision/disabled-by-attribute-presence, decision/disabled-by-presence and decision/enabled-by-disabled-presence, which the declared `disabled` bool on a virtual node sits beside without an attribute.

@history_item n=16 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — `aria-disabled` writes a tree node's disabled, but the snapshot's `enabled` never reads it (decision/enabled-by-disabled-presence: aria-disabled moves no snapshot reading). Trap: F8 A writes the declared `disabled` on the virtual tree node and has the snapshot read it as `enabled`. That is a tree-sourced enabled reading which this row currently excludes for elements, so the row's clause must be qualified without letting `aria-disabled` in.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (§5, §6: `inert` and `disabled`) a truthy `inert` takes focusability, hits and the tree from its subtree, a hit passes through an inert element, and `disabled` is narrowed to form controls (decision/inert-subtree-excluded, decision/inert-element-takes-no-hit, contract/hit-testing/inert; ratified 2026-10-18). A canvas under `inert` or hidden must take its virtual children out of the tree, and a click on one must not land.

@history_item n=18 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` reads the model through the text form, every node id on exactly one line (contract/snapshot-text/parts). Virtual node lines and ids are bound by this.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the tree's focus falls back to the window) `build_accessibility_tree` writes focus only where the focused node is among the built nodes, else `u64::MAX` (contract/accessibility-tree/focus, contract/accessibility-tree/root). F7 A's focus writer has to check the virtual map first, and the tree must never name a node it did not build.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the snapshot reads of an open list; covered; the changed set) the snapshot builder reads `select_list_bounds(node)` before the client rect, so a row's bounds come from something other than its DOM box. That is the nearest precedent for virtual bounds from a region moved by the canvas origin. No `expanded` node key was added. A driver click on an element under an open list is refused `covered` (contract/driver-click-refusals/covered). A single select's list open/close marks the changed set. Precedents for the virtual bounds arm, the `covered` reading and the no-new-key rule.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `NodeState.clickable` is read from the vdom walk's current click listeners (contract/snapshot/clickable, contract/snapshot-node/clickable, contract/snapshot-text/clickable). The read-only UI graph holds one node per snapshot node (decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values). `stand_graph` asserts that no served id holds the reserved separator `///`, the `{owner}///{key}` form spelled by `virtual_node_id` (contract/node-id/reserved-separator). This chunk serves `///` ids for the first time: the separator contract and the stand_graph assertion are what change. F12 A has a virtual node read `clickable` false, and F9 A excludes virtual nodes from the graph to keep one node per snapshot node.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actionable_nodes` lists the UI graph's nodes, `keyed` true where the id holds no `/`. The `actions` verb answers each id-taking verb with the cause `check_target` would refuse, a node with no element reading `stale`. A virtual id holds `///` (keyed false) and resolves to no element, so the actions reader and the refusal causes (`stale`, `not-found`) need care. Exploration is held out under F9 A.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `handler_coverage` is one more reader of `DioxusDocument::graph`. With virtual nodes kept out of the graph (F9 A), it must not see them through the snapshot.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks` took the select-list scroll qualifier. This is part of the row history that F11 A leaves unamended (a declaration change writes no mark).

@history_item n=25 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed in a redraw after style resolve only when the changed set is non-empty, through the document trait's method. `~ contract/accessibility-tree/hidden-and-inert-nodes`, `~ decision/hidden-content-excluded-from-a11y-tree` and `~ decision/hidden-content-excluded` added `visibility: collapse`. Under F11 A a declaration change alone never pushes a new platform tree, and a hidden canvas's virtual nodes fall under these exclusion rows.

@history_item n=26 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ decision/changed-set-marks` (highlight moves mark), `~ decision/snapshot-reads-accessibility-tree` and `~ decision/snapshot-reads-the-tree` (highlighted taken from the tree), and the tree naming the highlighted option as active descendant. This is the latest precedent for a new tree-sourced snapshot reading and its row edits, which a virtual node's tree-written state follows.

@history_item n=27 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — `~ decision/changed-set-marks` reached its current wording (post-layout focus loss too) and `~ decision/tab-and-shift-tab-move-focus` changed. This is the row text F11 A leaves as is, and the scope's CARRY `cry-t4af3d26` names this row as left unamended.

@history_item n=28 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — `~ decision/single-select-engine-overlay-list` (a drop-down alone opens the overlay) and `~ contract/accessibility-tree/select-expanded`. These are the current texts of the picked select rows behind the `covered` refusal and the expanded reading that a virtual node does not touch.

@history_item n=29 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** — ratified the PROVISIONAL hidden-content rows (`decision/hidden-content-excluded`, `contract/accessibility-tree/hidden-and-inert-nodes`, `decision/hidden-content-excluded-from-a11y-tree`) in the founder's own word. Those rows now stand ratified, and any change to them for virtual nodes needs the founder.

@history_item n=30 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0) and is edited by hand, since its pin reads none of the claims (decision/capability-statement-reach-by-hand, decision/accessibility-reach-tier-0). F3 A lists `docs/capabilities.md` among the surfaces now carrying virtual ids and names, and its stated limits (F10 B, F12 A) go there by hand.

@history_item n=31 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand` · why: the pin reads only the a11y leg's file count and no check reads the rest. Any reach or limit line this chunk adds is unchecked.

@history_item n=32 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — `~ decision/removed-focused-node-drops-focus`: focus drops to the root element and the tree's focus names the window. A virtual click focuses the canvas, and the focus writer's virtual-map check must keep the window fallback.

@history_item n=33 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (the enabled-state walks) the `&dyn Document` tree is shown equal to the document's, carrying stable ids only (decision/adapter-tree-is-document-tree). The platform adapter is an F3 A exit, so the adapter tree must hold the same virtual nodes and ids.

@history_item n=34 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the `lint` verb) a contrast finding carries the node's id, with ids, closed words and numbers only (contract/lint-contrast-finding/id). F9 A holds virtual nodes out of the lint, so no finding carries a `///` id.

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the lint's closed-select overlap ended) `overlaps` skips a closed select's options by reading the element, not the role. This is a precedent for the lint excluding snapshot nodes by kind, as F9 A's exclusion in `lint.rs` must.

@history_item n=36 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `+ decision/circle-drawer-canvas-pixels-only`: Circle Drawer's drawing area is one named canvas node, and its circles reach no snapshot or tree. Making Circle Drawer's circles virtual nodes is outside this chunk, so its circles keep reading as this row states.

@history_item n=37 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `~ decision/circle-drawer-canvas-pixels-only` state provisional → standing, ratified by the founder. The canvas-node row this chunk's fixture canvas sits beside now stands.

@history_item n=38 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `~ decision/readers-of-one-snapshot-model` and `~ decision/snapshot-model-single-source`: the canvas read is no reader of the snapshot model. This is the first split of these rows between snapshot readers and canvas reads, which this chunk changes by moving virtual nodes into the snapshot model.

@history_item n=39 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `~ decision/circle-drawer-canvas-pixels-only`: canvas labels reach the lint as truncated boxes with no text, PROVISIONAL on the delegate's confirm. This is the nearest canvas-to-lint path, and it must stay separate from the F9 A lint exclusion of virtual nodes.

@history_item n=40 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — `~ decision/snapshot-model-single-source` and `~ decision/readers-of-one-snapshot-model`: the virtual-node read of one canvas's declared nodes is no reader of the snapshot model and feeds no carrier. This chunk inverts that, putting virtual nodes in the tree, the snapshot and every carrier (F3 A), so both rows' virtual-node clauses must be re-said. This is the CARRY `cry-qprhfz5u` inversion.

@history_item n=41 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — `+ absence/keyboard-drag-and-drop`: no keyboard or assistive-technology route exists for drag and drop. It is a picked absence of the scope area. Keyboard reach to virtual nodes likewise stays out of scope («Canvas keyboard path»).
