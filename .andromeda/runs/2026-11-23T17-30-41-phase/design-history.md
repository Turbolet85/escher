@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the seven_guis headless stand registers the bundled DejaVu Sans for every generic with system fonts off (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`); Circle Drawer's in-process stand boot must take that same font context, since the canvas text is painted through `Widget::fonts` from the document's own font context and the cross-OS digest rests on one bundled face.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle answers every member of the keeps-animating set alike, none holding it open, `animating` reading the document flag; it names a canvas as reading animating for as long as it exists. The Circle Drawer canvas will keep `animating` true on every step, and settle must still return `Settled` without waiting on it (row decision/animation-driven-by-virtual-clock).

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time while `click`, `type` and `press` leave the animation clock where it was; a click beside a running animation returns settled. Precedent for the motion rows the canvas clicks rest on (contract/driver-verbs-and-virtual-clock/click).

@history_item n=4 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read; `advance` moves it by exactly its ms and the app's own time together, `click` leaves it. Source of decision/animation-driven-by-virtual-clock, decision/driver-advance-moves-one-clock and contract/driver-verbs-and-virtual-clock/advance · click; F4 A's double click (a second centre click with no `advance`) reads against this clock.

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — ratifies the hit stop at a `contain: paint` box and `advance` moving the clock and app time together; both the depth and motion rows this chunk is bound by stand ratified, not provisional. The earlier same-marker entry widened the hit stop from `overflow` alone to a computed `contain` including `paint` (decision/hit-test-stops-at-paint-clip).

@history_item n=6 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node that clips by overflow, the node itself answered in its border box, root excepted: origin of the hit-at-paint-clip rows; a pointer on the canvas `<object>` reaching the widget target is decided by this walk.

@history_item n=7 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured hits at an element's own box: a raster `img` answers the `img`, an `iframe`'s sub-document answers the `iframe`, a filled `input` the `input`; precedent that a replaced element answers as itself — the canvas `<object>` is the target the click synthesis (F2 A) and driver `click --id circle-canvas` reach.

@history_item n=8 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the hit walk skips a node dropped since the last resolve, as paint does; a depth-strategy rule beside the stacking and hit rows this chunk's events change rests on.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (non-invertible transform; Cells into-view) — a transform with determinant 0 or non-finite takes no hit for the subtree, a depth rule beside the picked hit rows; and the Cells grid's `scroll_to` is called synchronously because an async form spawned from a key handler panicked `RefCell already borrowed` — a trap for any widget or handler code calling back into the document.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (hit before the page walk; paints last) — a row of an open select list is answered before the page's hit walk by `select_list_hit` in `hit_with_scrollbar`, the except clause of decision/hitting-stops-at-paint-clip; the open list paints last in `paint_scene`. Precedent for an engine overlay or special target in the hit and paint path that `events/mod.rs` dispatch must not disturb.

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — `mix-blend-mode` other than `normal` joins the stacking-root list (decision/stacking-context-groups-paint-and-hit), and citations into `custom_widget.rs` moved +2: the custom-widget code this chunk's widget and click synthesis stand on was last re-pointed here.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a driver `screenshot` whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; `screenshot [--id]` answers `{"width","height","png"}`, MCP an image block then text. Source of the picked typography refusal rows and contract/escher-session-commands/screenshot; Circle Drawer's canvas shot (with widget-painted text) must pass this gate.

@history_item n=13 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography) — + decision/unavailable-families-use-default-font: an unavailable family falls back to the default font; beside decision/no-generic-family-lays-out-sans, it bears on which face the canvas text and the `fonts-unavailable` check resolve.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the bundled DejaVu Sans for every stand boot (`boot_cells`, `boot_load`), `escher-session` serving six tasks, and a driver `click` in a shown overlay scrollbar's band taken as a thumb drag until an `advance` passes its fade; precedent for adding a stand boot (Circle Drawer's in-process boot) with the stand font, and for click traps on the stand.

@history_item n=15 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (stand radii, shadows, padding; stand font registration) — the stand's card and circle-dialog shadows, radii and padding left design-system for layout-templates `values · desktop-native-screens`, and the stand's font registration left for architecture §Standard Contracts → Headless stand: Circle Drawer's canvas styling and its stand boot's font belong there, not as design-system values.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (ratified move; stand colours and type leave) — ratifies the stand-row move and removes the remaining seven_guis colour, type and spacing rows to layout-templates; any canvas colours or circle-text sizes this chunk adds are stand values held there, the engine holding no tokens (absence/engine-color-tokens, absence/project-type-scale).

@history_item n=17 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once through `DocumentConfig::default()`: source of decision/default-ua-sheet-via-default-config, which the new Circle Drawer stand boot must honour.

@history_item n=18 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document a render is recorded into a mutation buffer unborrowed, then replayed under one borrow (decision/buffered-render-replay); the `data-circles` attribute write reaching `attribute_changed` arrives through that replay.

@history_item n=19 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (MountedData document-state) — a MountedData call during a render acts on the document as the previous render left it; re-states the buffered-render limit the canvas sample's handlers live under.

@history_item n=20 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (three stated limits) — the `coverage` answer does not count handlers inside a canvas (owner 0.4.0, the canvas plans); with Circle Drawer's circles moved onto a canvas this limit becomes live for the sample, while `onclick`/`ondoubleclick` on the `<object>` itself remain DOM handlers.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` line: every actionable node in pre-order (focusable, interactive role, a current listener, or owned) with verbs decided by `validate` and `check_target`; source of decision/actions-share-the-verbs-checks and contract/actions-line/*. The canvas `<object>` (clickable, takes focus) is listed and must be keyed (`unkeyed_actionable` 0).

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph answer line) — the UI graph's line content (ids, component names, state names, handler event names, flags, booleans only); with circles off the DOM the graph no longer reads them (contract/ui-graph-line/content).

@history_item n=23 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the cli surface's graph answer line) — the graph answer line stands ratified by the founder, no longer PROVISIONAL.

@history_item n=24 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — ~ contract/escher-session-commands/graph-answer and ~ contract/ui-graph-line/content: the graph answers a paged form with `part`/`parts`; the picked rows' current wording.

@history_item n=25 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text"}` or `{"text","part","parts"}`: source of contract/escher-session-commands/snapshot; circles leave the snapshot once on the canvas.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — + contract/escher-session-commands/layout-answer: `layout --id` answers bounds, laid_out, placed, shifts; the canvas `<object>`'s placement the earlier canvas chunks proved reads through it.

@history_item n=27 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the served-task set moved from six to seven (`order` added) with every task row's count re-worded; F1 A keeps Circle Drawer unserved, so no served count or task row moves in this chunk.

@history_item n=28 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` prints one uncoloured JSON line per command, no ESC byte, exit classes 0/1/2/3: the form every picked escher-session-commands row extends.

@history_item n=29 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: JSON-RPC lines only, uncoloured, no ESC byte; the carrier over which the picked command rows' answers also travel.

@history_item n=30 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock (`VirtualClock` stands still for at most `LOAD_WAIT`); beside the clock rows a canvas click step reads.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (137 rows), each verified against the code; the handles in the picked rows file date from this conversion.
