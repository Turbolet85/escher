@history_item n=1 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `+ decision/circle-drawer-canvas-pixels-only`: Circle Drawer's circle nodes were replaced by one canvas node (role group, named Drawing area, keyed, clickable), read by role and name alone, on the delegate's provisional word. This chunk turns the circles back into nodes, now virtual ones, so it amends this row at its root.

@history_item n=2 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** (amd-pfrr546l and amd-rjjvoc3s) — `~ decision/circle-drawer-canvas-pixels-only`: state went from provisional to standing on the founder's own «Подтвердить (Recommended)». Because the row is ratified founder word, its clause that the circles reach no snapshot, tree or UI graph cannot be edited quietly when the circles become virtual nodes. The change has to be a rule change with its own ratification path.

@history_item n=3 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `~ decision/circle-drawer-canvas-pixels-only` widened «only as shot pixels» to include the canvas verb's draw commands, no label's text among them. `~ decision/readers-of-one-snapshot-model` now says the canvas read is no reader of the snapshot model. This is the precedent for re-stating the Circle Drawer row whenever a new reading of its circles appears, and the canvas reading this chunk groups by virtual node (CARRY 1) is the read these rows exempt from the snapshot model.

@history_item n=4 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (amd-4746wvru) — `~ decision/snapshot-model-single-source` now says the canvas read, which reads one canvas element's draw commands, is no reader of the snapshot model. If grouping commands under a virtual node's id puts snapshot-derived ids into the canvas answer, this twin row must be re-checked together with `readers-of-one-snapshot-model`.

@history_item n=5 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `~ decision/circle-drawer-canvas-pixels-only`: Circle Drawer's labels now reach the lint as truncated ink boxes with no text (F1 A), the confirm PROVISIONAL as the delegate's. This is the row's current text. The trap is that the circles and labels still «reach no DOM, snapshot, accessibility tree or UI graph», which this chunk makes false. CARRY 2 (label region) and CARRY 7 (the lint reads no virtual node) build on this lint reading.

@history_item n=6 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (amd-vjpeddx7) — `~ decision/snapshot-model-single-source` names the virtual-node read, which reads one canvas's declared nodes, as no reader of the snapshot model. Circle Drawer's `CircleCanvas::virtual_nodes` feeds that read.

@history_item n=7 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** (amd-726lrkbr) — `~ decision/readers-of-one-snapshot-model` likewise names the declared-node read as no reader of the snapshot model, kept in step with its twin. Any change this chunk makes to one twin has to be made to the other.

@history_item n=8 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-mmqj546n) — `~ contract/snapshot/enabled`: a virtual node's enabled reads the declaration's disabled flag (F8 A with F2 A). This is the reading a disabled circle node's `disabled` refusal rests on. The same entry's `~ decision/ui-graph-one-node-per-snapshot-node` (F9 A, PROVISIONAL) excepted virtual nodes from the graph. A later entry reversed that (n=12).

@history_item n=9 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-cjywvajx) — `~ contract/node-id/reserved-separator`: the `{owner}///{key}` form became served as a virtual node's author id and snapshot id, no element id holding `///`. Circle keys must satisfy this form, and a circle has no id of its own, which is an open design choice.

@history_item n=10 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** (amd-dy2qbl6d) — `~ contract/driver-click-refusals/covered`: a virtual node reads `covered` only when an element lies over its region's centre (F12 A, delegate, PROVISIONAL). This is CARRY 3. Overlapping circles are exactly the case where another virtual node over the centre is not read as covering.

@history_item n=11 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** (amd-rn7vbwds) — `~ contract/node-id/reserved-separator`: the reserved form is now also served in the UI graph, actions answer and layout answer, these three never carrying the node's declared name. A circle's name must not leak into those three answers. The binary-level reading of id and name is owed here by CARRY 4.

@history_item n=12 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** (amd-oww7ws4g) — `~ decision/ui-graph-one-node-per-snapshot-node`: the graph holds a canvas's virtual nodes, each owned by its canvas with no handlers or reads, so actions and exploration hold them. Handler coverage lists none, which is CARRY 6: a click on a circle counts as the canvas's own `onclick` under the canvas id.

@history_item n=13 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — every stand task keyed. `unkeyed_actionable` went to 0 on Circle Drawer, its dialog open included, from 3, and the elements that react to a click with no role were pinned at Circle Drawer 1. Circle Drawer's stand pins (`stand_circle_drawer_canvas`, `stand_non_lean_tasks`) descend from this, and the scope's literal pin sweep must re-read them once circles become nodes.

@history_item n=14 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — introduced `decision/actionable-key-rule`: every actionable element reads an author key. Trap: never satisfy the check by adding a `tabindex` or role, which changes the pinned Tab order; key the element instead. This applies to circle nodes, whose keyboard reach is the next entry's and not this chunk's.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-wpjidzuh) — `decision/one-actionable-rule`: `unkeyed_actionable` and `actionable_nodes` share one crate-private rule whose listener leg is the vdom walk's exact current listeners. A virtual circle node carries no listener of its own, so whether it is actionable rests on its role and on its graph owner.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-itr5bp6q) — `contract/actionable-nodes/listing`: lists the UI graph's nodes the rule holds for or whose `owner` is set, each with id, owner and keyed. `keyed` is true where the id holds no `/`. Trap: a circle's `{owner}///{key}` id holds `/`, so its keyed reading must be checked against `decision/keyed-wire-form`.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-fpx233pi) — added the `keyed` wire form on the actions answer as PROVISIONAL (`decision/keyed-wire-form`, true where the id holds no slash). This is the row circle nodes in the actions answer will read against.

@history_item n=18 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form`: state went from provisional to standing on the founder's word. Any change to how a virtual id reads `keyed` is a ratified-row change.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-drbh3mkf) — `decision/actions-verb-changes-nothing`, `contract/actions-answer/causes`, `contract/actions-answer/fields`: the actions verb is read-only, answers each verb with the refusal cause (`covered`, `off-screen`, `disabled`, `stale`), and writes ids and closed words only. These are the causes v040-11 asks circle nodes to read on the stand.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-bk6sddie) — `ACTIONABLE_AT_BOOT` pins actionable counts at boot per task, asserted by `stand_actions` in both modes. These pinned counts may move when Circle Drawer's circles join the graph and the actions answer.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-on55r22y) — built `NodeState.clickable` (`contract/snapshot/clickable`, `contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`). `stand_non_lean_tasks` asserts a `CLICKABLE` table with Circle Drawer at 1, a pin the chunk's literal sweep must re-read when circle nodes join the snapshot.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-fdnkydq5) — the read-only graph reader `DioxusDocument::graph` (`decision/ui-graph-carries-no-values`). `stand_graph` asserted that no served id holds `///`, the origin of the reserved-separator contract that virtual nodes later opened.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-3rvybnoq) — graph state registration through `use_graph_state` and `DeclaredWrite`, with a page with no registered state reaching no registry (`contract/ui-graph-state/no-registered-state`). Circle Drawer's state registration and its canvas `onclick` writes go through this path.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-72ctxs7v) — `contract/handler-coverage/answer`: `handler_coverage` reads the graph at boot and after each step and lists unexercised handlers, with ids, event names and counts only. CARRY 6 lands here: a virtual node lists no handler, and a click on a circle counts under the canvas's id.

@history_item n=25 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-e3ybktlf) — the ungated reached record (`contract/dioxus-document/record-reached`, `contract/dioxus-document/reached-handlers`) on `handle_event`'s path, resolved before handlers run. A circle click reaches the canvas's `onclick` through it.

@history_item n=26 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/dioxus-document/reached-handlers`: a stopped, unwound or non-bubbling dispatch counts the innermost listener alone. This is the current promise CARRY 6's counting of a circle click under the canvas reads against.

@history_item n=27 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal is detected on the snapshot's `enabled` only (`contract/driver-actions/disabled`, `contract/driver-refusal/disabled`). Rule kept: never read it from an attribute value, focusability or a role list. A disabled circle node's refusal must come through the declaration's flag into `enabled`.

@history_item n=28 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — worded the `disabled` cause on the presence reading and set the rule that the detecting entry reads the snapshot's `enabled`, never the attribute's value. This is the precedent behind `contract/snapshot/enabled`'s virtual-node clause.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-patznspx) — origin of `contract/driver-click-refusals/covered`: a driver click on an element under an open list is refused `covered`. This is the row CARRY 3's virtual-node clause extends, and overlapping circles test its limit.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-6otkgvws) — the lint's closed-select overlap ended, and the stand at boot reads no finding of any kind (`decision/stand-lint-clean-at-boot`, guard `the_stand_at_boot_reads_its_measured_findings`). Overlapping circles must not bring an overlap finding back at boot. CARRY 7 says the lint reads no virtual node.

@history_item n=31 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the accessibility tree with no second mapping (`decision/snapshot-reads-accessibility-tree`, `decision/snapshot-reads-the-tree`). A circle's role and name must therefore reach the snapshot through the tree node its declaration writes.

@history_item n=32 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (amd-komqr2qi and amd-6rwtnurh) — `~ decision/snapshot-reads-accessibility-tree` and `~ decision/snapshot-reads-the-tree` add `highlighted` to the readings taken from the tree. This is the current text of the two rows a circle node's snapshot readings follow.

@history_item n=33 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — every Dioxus element node carries its stable id as `author_id`, and no TextRun, document-root or Window node does (`contract/accessibility-tree/author-id`). Virtual nodes carry `{owner}///{key}` as author id beside this. The entry also set `decision/accessibility-feature-default-on` and the document-method tree build (`decision/accessibility-tree-through-document-method`).

@history_item n=34 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (amd-57nw4lid) — every interactive-role node of the four lean tasks is named over `REGION_NAMES`, and the `&dyn Document` tree equals the document's (`decision/interactive-nodes-named`, `decision/adapter-tree-equals-harness-tree`, `decision/adapter-tree-is-document-tree`). Circle Drawer is non-lean, but a circle node with an interactive role must still carry a non-empty name.

@history_item n=35 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand's role and name tables are stated once in `tests/blitz-tests/tests/common/mod.rs` (`decision/role-name-tables-stated-once`, `decision/stand-expectation-tables`). Expected circle roles and names for the new stand checks belong in those shared tables, never in a second mapping.

@history_item n=36 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-emc4tzfx) — the bridge's listener mark over-reports, and the vdom walk reads current listeners exactly (`decision/listeners-from-vdom-walk`). This is the basis for counting the canvas's `onclick`, not a circle, as the listener.

@history_item n=37 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the diff reads the snapshot model, and a text change is reported on the element whose name it changes, never as a text-run entry (`contract/snapshot-diff/entries`). It also set the changed set and the refresh on change. «Its diff names it» for a circle click rests on this.

@history_item n=38 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (amd-jbgzgzmv) — `~ decision/changed-set-marks`: the current marking rule, where a node's mutation marks and node creation or layout alone does not. A circle added or moved by a canvas click must mark the set through its canvas so the tree refresh and the diff see it.

@history_item n=39 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (amd-e3ygung6 and amd-tcemxx6t) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed again in a redraw after style resolve when the changed set is non-empty, with no push on poll. Virtual nodes reach the tree only through this refresh.

@history_item n=40 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (`decision/harness-settle-leaves-changed-set`). Trap for the new stand checks: drain the set first when reading it as what one step changed.

@history_item n=41 marker=2026-10-12-self-description
**2026-10-12-self-description** — `DiffNode::differs` names the readings that differ in node-key order (`contract/snapshot-diff/differs`), and the node keys' help reads the model. A circle node moved or resized by a click reads `[bounds]` there.

@history_item n=42 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP tool results read the snapshot model alone (`contract/mcp-result/snapshot`). CARRY 4's binary-level reading of a virtual node's id and name goes through these carriers.

@history_item n=43 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the snapshot's text parts, with every node id on exactly one line (`contract/snapshot-text/parts`). Circle nodes add lines that the parts must still hold whole.

@history_item n=44 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` is one more reader of the model's bounds, cropped to what shows, and a shot changes nothing. The circles still leave as shot pixels, and a shot of a circle node crops to its region's bounds.

@history_item n=45 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility`, so both seven_guis binaries compile the tree override but neither builds an adapter (`decision/stand-builds-no-platform-adapter`, `decision/agent-readers-behind-accessibility`). Circle Drawer's virtual nodes reach the stand binaries through this edge, and no assistive technology reads them.

@history_item n=46 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (amd-dtn2btox) — the latest stand-graph re-read: no platform adapter in the stand graph, owned by residual `res-llkgqygv`. This must be re-read if the chunk moves a manifest or the lock.

@history_item n=47 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach, and its pin reads none of the claims, so changes are made by hand (`decision/capability-statement-reach-by-hand`). The scope's sweep of `docs/capabilities.md` sizes and counts must hand-edit the reach if circle nodes change it.

@history_item n=48 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand` why: the pin now reads only the a11y leg's file count among the section's claims. Any leg file-count change trips it, and the rest stays hand-kept.

@history_item n=49 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (amd-6ec4g27i) — `inert` takes focusability, hits and the tree away, and a hit passes through an inert element (`decision/inert-subtree-excluded`, `decision/inert-element-takes-no-hit`, `contract/hit-testing/inert`). This bears on a virtual node whose canvas sits under inert or hidden content.

@history_item n=50 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — a framework-written `inert: false` is removed, the 28-name falsy predicate (`decision/inert-subtree-excluded`'s second clause).

@history_item n=51 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (amd-e65yobek and amd-tm2qhe37) — `~ contract/accessibility-tree/hidden-and-inert-nodes` and `~ decision/hidden-content-excluded-from-a11y-tree` add `visibility: collapse` to the exclusions. Both were ratified later by 2026-11-21-dependency-and-ci-debts (amd-owhcuhy3, amd-hzdh42am). Virtual nodes of an excluded canvas leave the tree and the snapshot with it.

@history_item n=52 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — `+ absence/keyboard-drag-and-drop`: no keyboard or assistive-technology route to drag and drop. This is part of the scope summary's absences beside the circles' keyboard reach, which is left to «Canvas keyboard path».
