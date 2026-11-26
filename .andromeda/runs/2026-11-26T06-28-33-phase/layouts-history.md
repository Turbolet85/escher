@history_item n=1 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` took `tabindex="0"` and an `onkeydown` (ArrowDown/Up, Home, End move the selected row, which fills the form as a click does; a key at the end changes nothing), CRUD's Tab sequence grew to six stops, and the tree's focus stays the list, never the row, with no `aria-activedescendant` read. This is the direct precedent for making `circle-canvas` a Tab stop with keys that move a current circle. The tree-focus gap was stated plainly on the operator's F5. This chunk's open question, how tree and snapshot focus name a virtual node, sits on the same gap.

@history_item n=2 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named `People` by `aria-label` alone, and the current-option gap (no `aria-activedescendant` read) stayed stated. Two things follow for this chunk: a canvas Tab stop with an interactive role needs a name given by attribute alone, and the gap stays a stated limit unless P4 forks on it (per the scope's boundary).

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` took an id, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`. Arrows move `current`, clamped, with a synchronous `scroll_to`; Enter opens the editor and its Enter returns focus to the grid. The grid became one Tab stop (founder's F13) with its sequence named by `CELLS_TAB_ORDER`. The reacting-element count moved (676 to 677), and the async handler form panicked (`RefCell already borrowed`). This is the closest precedent for a canvas with a keyboard-moved current item, a dialog reached by key and the shared Tab table. Its traps are the count shift in `stand_non_lean_tasks` and async handlers. A pointer path also moved in that chunk, and the operator ruled on it at P2.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence and its button's keys) — the load page's Tab sequence is stated by naming `LOAD_TAB_ORDER` in `tests/blitz-tests/tests/common/mod.rs`, not spelled out, and walked by `stand_keyboard` in both layout modes. This is the form Circle Drawer's new Tab-order constant follows when it enters the shared table.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the tab-order citations after the actionable-at-boot table) — the Tab-order citations into `tests/blitz-tests/tests/common/mod.rs` were re-pointed after `ACTIONABLE_AT_BOOT` was inserted above them. Adding a Circle Drawer Tab order to that file moves the same citations again.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — in an open select list, focus stays on the select while the current row is the highlight, which is in neither the snapshot nor the tree. The Tab sequence holds with the list open and after each close, and Tab chooses the current row before focus moves. This is a precedent for engine focus on one element while a sub-item is current. It also bears on the scope's decision of what the tree's `focus` and the snapshot's `focused` name for a current circle.

@history_item n=7 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — added Circle Drawer's author ids (`circle-undo`, `circle-redo`, `circle-canvas`, and `circle-diameter`/`circle-close` on the open dialog) as `id:` lines alone, with `unkeyed_actionable()` reading 0 on every task. These are the origin of the picked `contract/stand-element-ids/circle-*` rows. The diameter dialog this chunk reaches by keyboard is where `circle-diameter`/`circle-close` appear.

@history_item n=8 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the founder's rule: every element an agent can act on reads an author key, given as an attribute only. A circle's virtual-node key and any new focus or selected reading must keep to it.

@history_item n=9 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs were named by `for`/`aria-label` alone, with no element, class, style or order changed. This is the origin of `decision/stand-a11y-by-attributes-alone`, which binds the canvas's `tabindex`, role and name when they are added.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — states were given by attributes alone (`aria-invalid`, `role="listbox"`/`"option"`, `aria-selected`), and the classes that style those states stay. This is the precedent for a stated `selected`. The founder's B1 instead carries `selected` on `VirtualNodeDeclaration` and reads it as the snapshot's existing `selected`.

@history_item n=11 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `decision/circle-drawer-circles-painted-on-canvas` (on a delegate's word, PROVISIONAL) and added Circle Drawer to `decision/stand-lints-clean-at-boot`. Both picked rows start here, and this chunk's selection repaint and key handling must keep them true.

@history_item n=12 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — the founder ratified `decision/circle-drawer-circles-painted-on-canvas`, so its state moved from provisional to standing. Any change to the row (a selected or focus reading, the 7GUIs filled repaint) is an amendment to a standing, founder-ratified row.

@history_item n=13 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — the row `decision/circle-drawer-circles-painted-on-canvas` changed: the circles now cross as shot pixels and canvas draw commands. The 7GUIs selection repaints the chosen circle filled, which moves the draw-command pins (`stand_canvas_commands`, `stand_canvas_cross_os`) that the operator's direction says to sweep.

@history_item n=14 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `decision/circle-drawer-circles-painted-on-canvas` now says that lint reads label ink boxes. `decision/stand-lints-clean-at-boot` gained the promise that Circle Drawer reads no canvas finding after a click or at 12 and 100 circles. That rested on a delegate, PROVISIONAL. Under B1 a click now selects rather than adds, and selection and focus repaints must keep these lint readings clean.

@history_item n=15 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** (Surface: desktop-native) — the row's latest rule: `data-circles` reads x,y,d,id, and each circle is also a canvas virtual node crossing as its node. This chunk extends that node with `selected` and keyboard focus. The prior chunk's stand click still adds a circle (F1 A), and the founder's B1 carried here reverses that.

@history_item n=16 marker=2026-11-25-circle-drawer-virtual-nodes
**2026-11-25-circle-drawer-virtual-nodes** (Surface: cli) — `contract/escher-session-coverage/answer` gained a canvas virtual node's click under its `{owner}///{key}` id, canvas-owned and exercised where a replayed click named it. `GRAPH_DRIVES`/`stand_handler_coverage` must be re-read now that the first click selects, and a key path on the canvas may need the same coverage reading.

@history_item n=17 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native) — `decision/stand-lints-clean-at-boot` grew by one served page, and new stand ids entered as one row each. This is the pattern for how this chunk's row changes land: a new stand id or a changed lint promise is one `~`/`+` row each.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — `stand-lints-clean-at-boot` reached `at_boot` `[0, 0, 0, 0]` because `overlaps` skips a closed select's options (F9). This shows that a new sub-element reading (here, virtual nodes) can make a clean-lint row false unless lint excludes it. Virtual nodes in lint remain out of scope («Node-graph stand app»).

@history_item n=19 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand mounts one task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with DejaVu Sans. This is the origin of `decision/headless-stand-pinned-viewport`. The keyboard walk and the `covered`/`off-screen` circle readings are measured against it.

@history_item n=20 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the origin of `decision/held-instance-reads-as-fresh-boot`, which keyboard-driven circle selection must not break.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount; escher-session's coverage role) — the coverage role replays each check on a fresh boot through the same mount, and `held-instance-reads-as-fresh-boot` holds in both layout modes. This is the coverage answer's origin. Both parts were ratified at 2026-11-15T05-21-42-wrap.

@history_item n=22 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll moves every scrolling box that holds its target, then the viewport, and `visible_region` reads what an element can be seen through. The founder ratified this for every document. These are the origins of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`. They bind a circle's `off-screen` reading, which v040-11 still owes on the stand.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the picked rows' `except:` clause: a row of an open select list is seen through, and scrolled within, the list's scrollport alone. This is the precedent for a sub-element exception to both rules, the same question a virtual node's `off-screen`/`covered` reading would raise if it needed one.

@history_item n=24 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot --id`, with an element shot that crops to no pixel refused `off-screen`, exit 1. This is the origin of `contract/escher-session-refusals/off-screen`, which a circle's off-screen reading meets.

@history_item n=25 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — gave `escher-session` its endings: refused means exit 1 with the refusal's JSON. This is the origin of `contract/escher-session-exit-codes/1`, which a `press`/`click` refusal on a covered, off-screen or disabled circle uses.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `decision/no-caller-input-echoed` gained its one exception: the layout answer's id field. Any new answer that names a focused or selected virtual node by the caller's id would need the same kind of ruling. The 2026-11-17-a-component-in-isolation wrap ratified that exception.

@history_item n=27 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — scoped «no ending writes back anything the caller typed» to the commands. This is the history of `decision/no-caller-input-echoed`.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — added the `actions` answer `{"nodes"}` of id, owner, keyed and verbs with cause, which never carries a value, typed text or `NodeId`. This is `contract/escher-session-actions/answer`. A focusable canvas (and any `press` on it) changes what `actions` lists for `circle-canvas` and its circles.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — added the `graph` answer: ids, component names, handler event names, state names, flags and booleans, never a value. A new `onkeydown` on the canvas adds a handler event to Circle Drawer's graph. The founder ratified it at 2026-11-15T05-21-42-wrap.

@history_item n=30 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `contract/escher-session-graph/answer` gained `--part` paging (part and parts, with states on part 1 alone). This is the row's current form, which `stand_graph` re-reads.

@history_item n=31 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — added the `lint` verb answering `{"findings"}` on every carrier. This is `contract/escher-session-lint/answer`, through which Circle Drawer's clean-lint readings are taken.

@history_item n=32 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Surface: cli) — added the `canvas` verb and its MCP tool (PROVISIONAL on a delegate). A selected circle's filled repaint changes what `canvas` answers for Circle Drawer, which is the pins the operator's direction says to sweep.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows, each verified against the code. This explains the row handles the picked set uses and why this chunk's changes land as `~`/`+` row entries.
