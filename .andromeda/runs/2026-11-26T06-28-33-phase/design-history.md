@history_item n=1 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added decision/circle-drawer-canvas-colours-in-code: Circle Drawer's colours (stand accent stroke, dark label, accent fill at alpha 38/255) left CSS for constants in the widget code. This chunk repaints a selected circle filled under the founder's B1, so any new selected-fill or focus colour belongs among those widget constants, recorded as an exception to that row, and not in a stylesheet.

@history_item n=2 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's colours leave the product surface) — the stand accent and its uses are held in layout-templates §Surface: desktop-native → Primary screens, and design-system's `seven_guis stand` row states no value. A selected-circle fill or a canvas focus ring drawn in the accent is a new use and not a new token; the value stays in layout-templates. The founder ratified this on 2026-10-27 (2026-10-27-upstream-sync-engine-debts).

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand accent's use in the Cells grid; the Cells grid's into-view move) — the precedent for a stand widget's keyboard path. The founder's F13 gave `cells-grid` one Tab stop, with arrow keys moving a current cell and a `.cells-grid:focus .cell.current` outline in the accent, recorded as a new use of the accent with no token. Trap: the into-view move had to call `MountedData::scroll_to(Instant)` synchronously, because a task spawned from a key handler panicked with `RefCell already borrowed`. The canvas's one Tab stop with keys moving a current circle mirrors this pattern.

@history_item n=4 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `MountedData` calls (`set_focus` and the rest) made during a render are buffered and act on the document as the previous render left it, so `set_focus_to` is false for a node created in that render. This bears on focusing the canvas or moving focus from a key handler or task.

@history_item n=5 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — changed contract/mounteddata-during-render/document-state: a task awaiting a shown node's handle focuses it once mounted. It also retired decision/boxless-focused-select-keeps-focus, because the post-layout focus fixup now gives up focus on an unshown control (a11y-plan's decision/focus-fixup-gives-up-unshown-focus). Both constrain how the canvas holds engine focus while a virtual circle is current.

@history_item n=6 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — changed contract/select-accessibility/open-select: with focus kept on the select, the tree names the highlighted option as its active descendant, and a page's aria-activedescendant is not read. It also made the highlighted row the snapshot's closed `highlighted` bool. This is the one precedent for the tree and snapshot naming a current sub-item while engine focus stays on its host. It is the model, and the stated limit, for how the tree's `focus` and the snapshot's `focused` could name a virtual circle while the engine holds the canvas.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list answers the hit first; what the tree, snapshot and a shot read of an open list; how the list opens, moves and chooses) — sets decision/open-select-list-hit-first: `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read the open list first, and a button under it is refused `covered`. This chunk's `covered` reading on a circle (F4) goes through the same driver check. Its key model (keys move a highlight and choose nothing; Enter or Space chooses) is the nearby precedent for keys moving a current circle and acting on it.

@history_item n=8 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — set decision/animation-driven-by-virtual-clock and decision/driver-advance-moves-one-clock: the harness drives animation from the documents' VirtualClock, `advance` moves it by exactly its ms together with app time, and `click`, `type` and `press` leave it where it was. The founder ratified this on 2026-10-18 (2026-10-18-engine-debts, the Epoch 2 close batch). A new `press` on a focused canvas, and any repaint after selection, must leave the clock unmoved.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the origin of contract/driver-verbs-and-virtual-clock/click and /press: the driver's click and press leave the animation clock where it was, and a click beside a running animation returns settled without waiting. These contracts bind the keyboard press and the circle click this chunk adds.

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — the origin of decision/settle-does-not-wait-on-animation: no member of the animating set holds settle open, and a canvas reads animating for as long as it exists. A canvas click or press settles with `animating` read and not waited on, which bears on the selection diff (`differs: [selected]`).

@history_item n=11 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — added decision/canvas-line-form-conventions and contract/driver-verbs-and-virtual-clock/canvas (the canvas read changes nothing: no snapshot, focus, diff or clock change, and no pending Timer tick applied). A selected circle repainted filled moves the canvas lines, which must keep the form conventions. A focused canvas must not let a `canvas` read alter focus.

@history_item n=12 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** — added decision/canvas-line-node-tag: each line closes with `node=` and the circle's 1-based declared place, or `node=-`, and no line holds a key twice (guard no_canvas_line_holds_a_key_twice). This is the chunk this one carries from (F1 A: a stand click still adds a circle). Under B1 the selected fill moves the committed canvas pins, which must keep the tag and the no-key-twice rule.

@history_item n=13 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — changed contract/actions-line/nodes to «A virtual node is never listed» (PROVISIONAL, delegate the escher overseer, F9 A). A virtual node became a snapshot and tree element taking click and refusals. Later superseded by item 14.

@history_item n=14 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** — changed contract/actions-line/nodes to «A virtual node is listed», owned by its canvas, on the founder's «A — id во всех чтениях записи». A circle's `covered`, `off-screen` and `disabled` causes, and the canvas's own verbs once it is focusable, come through this answer.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the origin of decision/actions-share-the-verbs-checks, contract/actions-line/content, contract/actions-line/stale and contract/escher-session-commands/actions-answer. A node's verbs are decided by the same `validate` and `check_target` every id-taking verb meets, so a listed cause is the cause the call returns. The line carries no value, typed text or NodeId. This chunk's refusals on a circle (`covered`, `off-screen`, `disabled`) and `press` on the canvas must agree with what actions lists.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the origin of contract/escher-session-commands/coverage-answer and contract/coverage-line/content, plus a stated limit: handlers inside a canvas are not counted (owner 0.4.0, the canvas plans). A key handler on `circle-canvas`, and the re-read `GRAPH_DRIVES` in `stand_handler_coverage`, meet this answer and its canvas limit. The founder ratified the coverage line (2026-11-15T05-21-42-wrap).

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the origin of contract/escher-session-commands/graph-answer and contract/ui-graph-line/content: ids, owner ids, component and state names, and handler event names only. A new keydown handler and focusability on the canvas move what `stand_graph` reads. The founder ratified this (2026-11-15T05-21-42-wrap).

@history_item n=18 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — changed contract/escher-session-commands/graph-answer and contract/ui-graph-line/content to add a graph read in parts (`part`/`parts`). These are the current wordings of two picked rows that the stand's graph pins read.

@history_item n=19 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (decision, contract, absence and overview), each checked against the code. The picked rows not otherwise amended, such as decision/logical-viewport-from-scale, come from this conversion and hold as the code showed then.
