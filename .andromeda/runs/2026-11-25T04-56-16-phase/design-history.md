@history_item n=1 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every member of the animating set alike, none waited on or advanced; a canvas reads animating for as long as it exists (`decision/settle-does-not-wait-on-animation`). A virtual node's `click` must return settled with its diff beside an always-animating canvas, by this same reading.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's act-by-id verbs: `click`, `type` and `press` leave the animation clock where it was and a click beside a running animation returns settled without waiting (`contract/driver-verbs-and-virtual-clock/click`). A click resolved to a virtual id keeps this clock promise.

@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the refusal-detection chunk: the driver's `scroll` is instant in every box and the viewport inside one settled step (`decision/driver-scroll-instant`), founder-ratified. The `off-screen`/`covered`/`disabled` order this chunk extends to virtual nodes was set there; `scroll` on a virtual id reads `not-found` (F10 B).

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` prints one uncoloured JSON line per command, keyed by the driver schema's words, exit status carrying the class (`decision/escher-session-agent-read-surface`, `contract/escher-session-exit-codes/*`). A virtual node's id and name ride this form unchanged; no new exit or cause (F3 A).

@history_item n=5 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node clipping by `overflow` (`decision/hit-test-stops-at-paint-clip`). `covered` for a virtual node reads the hit at its region's centre, which stops at the canvas element, so a canvas clipped by an ancestor bounds what the virtual region can answer.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: JSON-RPC lines only, a `snapshot` result the screen text (`contract/escher-session-mcp/stdout`, `.../snapshot-result`, `.../other-results`). Virtual nodes reach the MCP carrier through the snapshot text unchanged in form.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table that `help` serves (`decision/exit-statuses-from-schema-table`). The chunk adds no exit, cause or session error; the table stands.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answer line, MCP image block, `fonts-unavailable` refusal with nothing painted (the typography rows picked). `screenshot --id` on a virtual id reads `not-found` (F10 B), a stated limit beside these refusals.

@history_item n=9 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default user-agent sheet arrives once through `DocumentConfig::default()` (`decision/default-ua-sheet-via-default-config`); the fixture canvas document a test declares takes its sheet this way.

@history_item n=10 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the hit measured at an element's own box (an `img`, an `iframe` sub-document answering the `iframe`, a filled `input`); a replaced box answers itself, not its inside. The precedent for a canvas answering the hit while the widget, not the walk, reads which virtual node was meant.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the hit stops at a `contain: paint` box) widens the clip stop to a computed `contain` including `paint`, PROVISIONAL then ratified by the founder at the Epoch 2 close in a later entry under the same marker; the current `decision/hit-testing-stops-at-paint-clip` row `covered` and the canvas-centre hit rest on.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`; `advance` moves it by exactly its ms, `click`/`type`/`press` leave it (`decision/animation-driven-by-virtual-clock`, `decision/driver-advance-moves-one-clock`), ratified 2026-10-18. A virtual click must not move the clock.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` sit in the exit-3 class with no new exit (`contract/escher-session-exit-status/3`). The session recording and kept-check replay now carry virtual ids and names (F3 A), adding no session error.

@history_item n=14 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — an acting step's load wait moves no clock (`decision/load-wait-moves-no-clock`); a virtual `click` is an acting step under the same wait.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot [--part]` answer line and its MCP form (`contract/escher-session-commands/snapshot`, `contract/escher-session-mcp/snapshot-result`). Virtual nodes enter the snapshot text and so its paging; the form stays.

@history_item n=16 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (the hit walk skips a dropped node) the hit walk skips a node dropped since the last resolve, as paint does. A stale virtual id and a canvas dropped under it meet this rule on the `stale`/`covered` path.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — right after a scroll the shown overlay scrollbar takes a pointerdown in its band as a thumb drag, so a driver click there needs an `advance` past the fade (`decision/shown-scrollbar-takes-thumb-drag`). A virtual click lands as a pointer event at its region's centre and is caught by the same trap in a scrolled canvas box; also the cold-agent bar this chunk's carriers reach.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`visible_region` through a clipping box's transforms; a non-invertible transform takes no hit) `visible_region` reads clipping boxes through their transforms, a rotation giving its bounding box; a non-finite/zero-determinant transform answers no hit for the subtree. `off-screen` reads a virtual region's centre against the canvas's `visible_region`, and its bounds move by the canvas's transforms.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list answers the hit first; `visible_region` on a row; what the tree, snapshot and a shot read of an open list) a non-element row answered before the page's walk and refused `covered` under it (`decision/open-select-list-hit-first`, `decision/hit-testing-stops-at-paint-clip` except clause); `visible_region` answers a row's list scrollport; a row carries its own `bounds` in the snapshot ahead of its client rect. The nearest precedent for a non-DOM node with its own bounds, region and refusals in the snapshot builder.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation` in the depth rules) the stacking-root list gains `mix-blend-mode` and `isolation`; paint and hit group under the same root (`decision/stacking-context-groups-paint-and-hit`), which `covered` at a canvas centre reads.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the graph answer line, carrying ids, component and state names and closed flags only (`contract/escher-session-commands/graph-answer`, `contract/ui-graph-line/content`), ratified 2026-11-15. `graph()` reads the snapshot, so a virtual node put there must be excluded from the graph (F9 A) to keep this row true.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` line, decided by the verbs' own `validate` and `check_target`, a node resolving to no element reading `stale` (`decision/actions-share-the-verbs-checks`, `contract/actions-line/*`). Actions excludes virtual nodes until «Virtual nodes in every read», while `click`'s refusals for a virtual node must still be the cause the same checks return.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage` answer line (`contract/escher-session-commands/coverage-answer`, ratified 2026-11-15) and its stated limit «handlers inside a canvas are not counted», owner 0.4.0; coverage stays out of this chunk, the limit standing.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows, each verified against the code; every picked row's text dates from this conversion unless amended after it.

@history_item n=25 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer`, `+ .../layout-arguments`: `layout --id` answer and its `malformed` refusals. `layout --id` on a virtual id reads `not-found` (F10 B), a stated limit; these rows stand unamended.

@history_item n=26 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — the highlighted row of an open list gained a closed bool in the snapshot and the tree names it as active descendant: a precedent for a non-element item entering the tree and snapshot as a closed reading inside the ratified carriers, as a virtual node's role, bounds and state do here.

@history_item n=27 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/escher-session-commands/graph-answer`, `~ contract/ui-graph-line/content` (paged graph). The current text of two picked rows; graph content stays free of virtual nodes under F9 A.

@history_item n=28 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — Circle Drawer's circles drawn by a widget with colours as constants; its circles as virtual nodes are a later entry, outside this chunk's boundary.

@history_item n=29 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `+ decision/canvas-line-form-conventions`, `+ contract/driver-verbs-and-virtual-clock/canvas`: the canvas read changes nothing and moves no clock. `canvas --id` on a virtual id reads `not-found` (F10 B); the canvas line's conventions stand.
