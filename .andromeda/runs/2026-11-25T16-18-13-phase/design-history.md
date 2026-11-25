@history_item n=1 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `+ decision/circle-drawer-canvas-colours-in-code`: Circle Drawer's circle colours moved out of CSS into widget constants in `CircleCanvas`. The virtual nodes this chunk declares sit in that widget's code. Any region or paint change must keep these colours as widget constants and add no CSS rule or token for them.

@history_item n=2 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (Surface: desktop-native) added `+ decision/canvas-line-form-conventions`, the canvas line's form for colours, blend, cap, dash and `at=none`. This chunk regroups that line under virtual nodes and gives a changed command an identity in the diff (CARRY 1). The grouped form has to keep these conventions, and adding a node id to the canvas answer is a crossing change.

@history_item n=3 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (Motion) added `+ contract/driver-verbs-and-virtual-clock/canvas`: the `canvas` read changes no snapshot, focus, diff or VirtualClock and applies no pending Timer tick. Grouping the draw commands by virtual node must stay a pure read under this promise.

@history_item n=4 marker=2026-11-25-virtual-nodes-as-elements
**2026-11-25-virtual-nodes-as-elements** — changed `~ contract/actions-line/nodes` to «A virtual node is never listed», on F9 A (delegate the escher overseer, PROVISIONAL). This is the chunk that put virtual nodes in the tree, snapshot and diff and let them take click and refusal, the base Circle Drawer's circle nodes rest on. Its PROVISIONAL F-answers (F12 A `covered` by elements only) come back here as CARRY 3.

@history_item n=5 marker=2026-11-25-virtual-nodes-in-every-read
**2026-11-25-virtual-nodes-in-every-read** — changed `~ contract/actions-line/nodes` to «A virtual node is listed», owned by its canvas, on the founder's «A — id во всех чтениях записи». It is the current text of the row this chunk's stand checks hit: each circle node must show up in `actions` with its canvas as owner. It also replaces the earlier entry's «never listed».

@history_item n=6 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line) added the `actions` line's shape, its pre-order listing, its content (ids, owners, verb words, closed causes and booleans, never a value or NodeId) and `decision/actions-share-the-verbs-checks` (causes decided by `validate` and `check_target`). A circle node's listed cause has to equal what `click` returns, so a covered, off-screen, disabled or removed circle is refused with the same cause in both reads.

@history_item n=7 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the unpaged `actions` answer) recorded the unpaged answer as a stated limit owned by `res-7cc5bi4k`. Circle nodes make Circle Drawer's `actions` line grow with the number of circles, and that growth falls under this limit.

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added the cli `graph` answer line and `contract/ui-graph-line/content` (ids, owner ids, component, state and handler names, closed flags and booleans only), plus the unpaged-graph limit (`res-7cc5bi4k`). Circle nodes read in the UI graph as `{owner}///{key}` must fit this closed content list, and their key and name must not carry a value.

@history_item n=9 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the 2026-11-07-ui-graph graph answer line and the 2026-11-08-handler-coverage coverage line, which had been PROVISIONAL. Both rows this chunk's reads extend are now settled by the founder.

@history_item n=10 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — changed `~ contract/escher-session-commands/graph-answer`, `~ contract/escher-session-commands/graph-arguments` and `~ contract/ui-graph-line/content` to add the paged form (`--part`, `part`/`parts`). Circle nodes added to the graph have to page correctly and keep both the whole and the paged forms.

@history_item n=11 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added `values · cli-coverage` (the `coverage` line's shape and content) and `pending · cli-coverage-limits`, whose first limit, «handlers inside a canvas are not counted», is owned by 0.4.0. CARRY 6 (a click on a virtual node counts as its canvas's own `onclick`, under the canvas's id) is that limit coming due on Circle Drawer. Coverage must list no virtual node and must keep the line's content closed.

@history_item n=12 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added `+ contract/escher-session-commands/layout-answer` and `+ contract/escher-session-commands/layout-arguments`. `layout --id` on a circle's `{owner}///{key}` id must answer this shape and refuse a bad id `malformed`.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set the `snapshot [--part]` answer (`{"text"}` or `{"text","part","parts"}`) and the MCP snapshot result for a paged screen. Circle nodes enter Circle Drawer's snapshot text, so pinned snapshot readings and part counts can move. These are among the literal pins to sweep.

@history_item n=14 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the cli output form: one uncoloured JSON line per command, no ESC byte, no terminal test, exit status 0 accepted and 1 refused. CARRY 4's binary-level reading of a circle node's id and name has to read it from this line on stdout.

@history_item n=15 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — set the `mcp` role's stdout: NDJSON JSON-RPC lines only, a tool result's text being the screen text for `snapshot` and the JSON line otherwise. CARRY 4's reading over seven_guis' `mcp_server` process check and the cold-agent pipe rests on this.

@history_item n=16 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set `escher-session screenshot [--id]` as one uncoloured `{"width","height","png"}` line plus the MCP image block. A `screenshot --id` on a circle node, if it is supported, has to keep this form.

@history_item n=17 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — made the hit walk stop at the padding box of a node that clips by `overflow`, the root element excepted. This is where `decision/hit-test-stops-at-paint-clip` comes from. A circle spilling its canvas's content box is clipped, which ties to the `RegionOutside` refusal and to a circle's `covered` and off-screen readings.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — widened the hit stop to a computed `contain` that includes `paint` (F5 A, later ratified). The canvas's ancestors may clip by `contain`, so this bears on whether a circle region's centre is hit or covered.

@history_item n=19 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified at Epoch 2 close) ratified the `contain: paint` hit stop and the one-virtual-clock `advance` rule. Both rows this chunk rests on (hit at paint clip, driver advance) are settled.

@history_item n=20 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured the hit at an element's own box in both layout modes and both builds. It is the precedent for this chunk's stand checks in both layout modes reading a circle node's hit and `covered`.

@history_item n=21 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the hit walk now skips a node dropped since the last resolve. This bears on a removed circle, whose node must read `stale` or be refused with the removed cause rather than be hit.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (non-invertible transform; `visible_region` through transforms) a determinant-0 transform takes no hit, and `visible_region` reads clip boxes through transforms, exact under translate and scale. The off-screen and visible readings for a circle node's region rest on these. Rotated and scaled canvases stay «Pan and zoom»'s.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list answers the hit first) added `decision/open-select-list-hit-first`: `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read the open list first, so a box under it is refused `covered`. A virtual node's `covered` (CARRY 3, element over the region's centre) has to keep this order.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation` in the depth rules) widened the stacking-root list behind `decision/stacking-context-groups-paint-and-hit`. Paint order and hit grouping over the canvas decide which element covers a circle's centre.

@history_item n=25 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — set the rule that the driver's `click`, `type` and `press` leave the clock alone while `advance` moves app time. This is where `contract/driver-verbs-and-virtual-clock/click` comes from. A click on a circle node must not move the VirtualClock.

@history_item n=26 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — made `advance` move the one VirtualClock by exactly its ms together with app time, while click, type and press leave it (`decision/driver-advance-moves-one-clock`, later ratified). Stand checks that click circles and read the canvas must not rely on clock movement.

@history_item n=27 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's colours leave the product surface) moved the stand's accent and other values to layout-templates `values · desktop-native-screens`, leaving no design-system value for them. The Circle Drawer colours row is the recorded exception, kept in widget code. Any circle-node change must not bring stand values back into design-system.

@history_item n=28 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the stand's colours, type and select padding leave design-system) the stand-only lines went to layout-templates on the founder's word. It confirms that stand values such as Circle Drawer's sizes or labels are not design-system rows.

@history_item n=29 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — changed the `contract/escher-session-tasks/*` rows from six to seven served tasks. It is a precedent that served-task counts and lists are literal pins, relevant to sweeping the pins that change when Circle Drawer, a served app, starts declaring nodes.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (decision, contract, absence, overview) and keeps in it only what a planner must respect. Any new rule this chunk adds, such as the canvas reading's grouping by virtual node or command identity, enters as a `+`/`~` row by handle.
