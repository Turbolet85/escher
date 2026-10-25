# design · cascade read · group 3 of 3

## Carried
- (.andromeda/design-system-detail/depth-strategy.md:10) The flight booker's card shadow must keep its value while the chunk edits the one-way / return choice in the same file, and a driver shot of the restyled task paints it.
  > seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; the circle dialog uses 0.10 (examples/seven_guis/src/tasks/counter.rs:43; examples/seven_guis/src/tasks/timer.rs:76; examples/seven_guis/src/tasks/flight_booker.rs:140; examples/seven_guis/src/tasks/circle_drawer.rs:210)
- (.andromeda/design-system-detail/depth-strategy.md:19) The new select and Cells' scrolling grid must stack over the task frame through the stacking contexts the engine builds after layout, no stand change reaching into them.
  > Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:121-125; packages/blitz-dom/src/lib.rs:89)
- (.andromeda/design-system-detail/depth-strategy.md:20) Which of the select, Cells' grid and the flight booker's clipped toggle group roots a stacking context decides how each paints and is hit, so any row the plan adds to the re-pinned stand checks must hold to this engine rule rather than restyle around it.
  > A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1332-1375); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1377-1406)
- (.andromeda/design-system-detail/depth-strategy.md:26) Cells' `overflow: auto` grid and the flight booker's `overflow: hidden` group must stop a hit at their padding box, the reading that sets the driver's `covered` and `off-screen` refusals an agent meets there.
  > since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box
  (.andromeda/design-system.md:87)
  > Hit-testing stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, and the node itself is still answered inside its border box.
- (.andromeda/design-system-detail/depth-strategy.md:32) Positioned descendants of a served task must paint in tree order, the order a driver shot and the lint's `overlap` read.
  > Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)
- (.andromeda/design-system-detail/depth-strategy.md:33) The flight booker card's outset shadow must paint by the element's shape in a shot of the restyled task.
  > Outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82); an outset shadow takes the element's shape corner for corner
- (.andromeda/design-system-detail/border-radius.md:12) A restyled flight-booker choice and the Cells grid must keep the stand's untokenized radii, the flight booker's 8px card and 6px controls.
  > seven_guis (untokenized) | cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px
- (.andromeda/design-system-detail/border-radius.md:22) The radii of the restyled choice and its card must resolve per corner and scale to device pixels in every shot the agent takes.
  > Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1395-1412)
- (.andromeda/design-system-detail/motion.md:8) The served tasks, Cells and the new select included, must keep seven_guis styles free of transitions and animations, so an acting step's diff is the step's alone.
  > observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
  (.andromeda/design-system-detail/motion.md:47)
  > Observed absent — transitions or animation · searched: `transition|animation` over the 8 s07 slice files
- (.andromeda/design-system-detail/motion.md:11) An agent's `scroll` into Cells' grid must be instant in every box and the viewport, made inside one settled step.
  > the driver's `scroll` is instant in every box and in the viewport, made inside one settled step
  (.andromeda/design-system.md:123)
  > A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, whatever behaviour was asked, and the requested behaviour, `Smooth` included, applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport.
- (.andromeda/design-system-detail/motion.md:13) Cells' scrolling grid must show overlay scrollbars whose fade runs on the document's clock, the fade a shot after a `scroll` paints.
  > Overlay scrollbars show at full opacity on scroll, stay opaque for `FADE_DELAY` = 500 ms after last activity, then fade linearly over `FADE_DURATION` = 200 ms
- (.andromeda/design-system-detail/motion.md:19) Any animation a served task holds must move only by the clock during style resolution, the time `advance` moves.
  > CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:112-130)
- (.andromeda/design-system-detail/motion.md:20) Settle must answer a served task's animating set the same way, a scrollbar fade after an agent's `scroll` in Cells included, holding nothing open.
  > a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass
  (.andromeda/design-system.md:127)
  > A harness settle answers every member of the document's animating set the same way — none holds it open, none is waited on or advanced: it returns `Settled`, `animating` reading the document's animating flag.
- (.andromeda/design-system-detail/motion.md:26) The new goals exercise `advance` on the one `VirtualClock`, which must move by exactly its `ms` with the Timer's step while `click`, `type` and `press` leave it.
  > the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was
  (.andromeda/design-system.md:129)
  > The test harness drives animation from the `VirtualClock` its documents read, and settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.
- (.andromeda/design-system-detail/motion.md:32) A load in flight on the stand must reach the load wait with no clock moving, so no timer or animation runs during it.
  > The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait (packages/escher-driver/src/session.rs:16-53; as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md)
  (.andromeda/design-system.md:131)
  > The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT`, the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait.
- (.andromeda/design-system-detail/iconography.md:8) The select and Cells' inputs must take their cursor from the CSS `cursor` keyword, else the engine's fallbacks.
  > The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise
  (.andromeda/design-system.md:151)
  > The cursor is the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there.
- (.andromeda/design-system-detail/iconography.md:9) The stand's `cursor: pointer`, `default` and `not-allowed` keywords must map one-to-one to cursor icons in the windowed stand.
  > CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49)
- (.andromeda/design-system-detail/surface.md:8) The new `select` must paint from the one default stylesheet every document gets, never one the stand adds itself.
  > Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93)
- (.andromeda/design-system-detail/surface.md:9) The flight booker's `select` must lay out with the user-agent select defaults, its chosen option's label alone, unless the stand styles it.
  > In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token
- (.andromeda/design-system-detail/surface.md:11) The edited `stand::options` must pin the viewport over the engine default, the screen Cells and the select must fit for the shot and lint.
  > The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom
- (.andromeda/design-system-detail/surface.md:122) The cold-agent pipe's client reads `escher-session mcp`'s stdout, which must stay JSON-RPC lines only, uncoloured, with no ESC byte, `snapshot`'s paged line among them.
  > `escher-session`, the driver's command line (since 2026-10-10-driver-cli), is a surface an agent reads, not a person: a command prints one line of JSON on stdout — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there
  (.andromeda/design-system.md:208)
  > `escher-session`, the driver's command line, and its `mcp` role are surfaces an agent reads, not a person: stdout holds only JSON — one line per command (the answer, the refusal or a session error's object), or newline-delimited JSON-RPC 2.0 lines — never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not; the command line's exit status carries the class (`0` accepted · `1` refused · `2` usage · `3` session error), and an `mcp` `Err` ending exits `3`.

## Bindings
- **depth ↔ motion**: Cells' scrolling grid binds both — a hit stops at its padding box (.andromeda/design-system-detail/depth-strategy.md:26) while an agent's `scroll` into it is instant (.andromeda/design-system-detail/motion.md:11) and its scrollbar fade is read off the document's clock by settle (.andromeda/design-system-detail/motion.md:20).
- **surface ↔ border-radius ↔ depth**: a `select` in place of the flight booker's two buttons paints the user-agent defaults (.andromeda/design-system-detail/surface.md:9) unless styled with the stand's radii (.andromeda/design-system-detail/border-radius.md:12), inside the card whose shadow stays (.andromeda/design-system-detail/depth-strategy.md:10) and whose stacking follows the root rule (.andromeda/design-system-detail/depth-strategy.md:20).
- **surface ↔ depth**: the viewport `stand::options` pins (.andromeda/design-system-detail/surface.md:11) is the screen against which a clipped box's hit (.andromeda/design-system-detail/depth-strategy.md:26) and the lint's readings are taken.
- **motion ↔ surface (cli)**: the clock the new goals move by `advance` and the load wait leaves still (.andromeda/design-system-detail/motion.md:26; .andromeda/design-system-detail/motion.md:32) is read by the agent only through the MCP role's JSON output (.andromeda/design-system-detail/surface.md:122).
- **iconography ↔ surface**: the select's cursor falls to the engine fallback (.andromeda/design-system-detail/iconography.md:8) as its look falls to the default stylesheet (.andromeda/design-system-detail/surface.md:8).
- **surface ↔ color-palette (group 1)**: the select's user-agent colours (.andromeda/design-system-detail/surface.md:9) are the ones the re-measured contrast rows read.
