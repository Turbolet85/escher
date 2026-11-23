@task_prose part=chunk
Makes what an app paints from Rust through the engine's custom widget appear in a headless harness shot and in the driver's screenshot verb that rides it (v040-09, layer 1: a 2D canvas painted headless).
Measures on the stand first: before any fix the stand boots a canvas and reads a shot of it, in both layout modes, in each build selection the project runs (harness, driver, escher-session binary, workspace).
Makes a shot paint a widget in every build selection, so a check's reading does not hang on which packages a cargo invocation selected.
Runs every new check red first and keeps its red reading beside its green; collects every operator fork (which element hosts a canvas, app-painted pixels leaving through the screenshot answer, a stand canvas page or task) in one forks file, none answered by a default.
Re-asserts the screenshot's contract and the stand's served tasks' readings unchanged.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait an app paints through — paint onto a render context with styles, width, height and scale, returning a recorded scene
@task_file n=2 path=packages/blitz-paint/src/lib.rs
what: the paint crate's scene build, which appends each custom widget's recorded scene behind its custom-widget feature — build of the custom widget scenes
@task_file n=3 path=packages/blitz-paint/src/render.rs
what: the paint crate's render walk, which draws a custom widget's scene in place of its node — draw of a custom widget
@task_file n=4 path=packages/blitz-test-harness/src/screenshot.rs
what: the test harness's screenshot and screenshot-of-a-rect, rendered to a buffer by the CPU renderer, which the driver's screenshot verb rides
@task_file n=5 path=packages/dioxus-native-dom/src/mutation_writer.rs
what: the Dioxus mutation writer, which hands a custom widget only from an object element's data attribute and leaves a canvas element's paint source unread
@task_item n=1 part=needs
The screenshot verb and the harness shot: a widget's painted pixels must appear in the shot, in both layout modes and in every build selection.
@task_item n=2 part=needs
The screenshot's contract stands as it is: an 800 by 600 shot over opaque white, byte-identical across processes on one host, refused with fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=3 part=needs
The screenshot never paints with text skipped and is refused when fonts are unavailable, through the driver screenshot refusals; a canvas shot keeps both.
@task_item n=4 part=needs
The escher-session screenshot command and the MCP screenshot tool result carry the shot unchanged in shape: no new verb, argument, carrier, cause, session error, exit status, log field or span unless a fork's ruling adds one.
@task_item n=5 part=needs
App-painted canvas pixels leaving through the screenshot answer on the session socket, the command line, MCP and a kept check's shot record are a new content class: a fork for the operator, not a default; no content in a log field.
@task_item n=6 part=needs
Which element hosts a canvas — an object element's data attribute as the Dioxus path stands, or a canvas element whose paint source nothing reads — is a choice that turns what the canvas is: a fork for the operator, not a default.
@task_item n=7 part=needs
Whether the stand gains a canvas page or served task is a fork for the operator; until ruled, the escher-session served tasks (counter, crud, cells, flight-booker, timer, order, load) and the unknown-task answer stand.
@task_item n=8 part=needs
The seven served tasks' snapshot, lint and actions digests, the committed kept checks and the stand checks keep their readings; a ruling that moves one re-records it, never loosens it.
@task_item n=9 part=needs
The paint crate's custom-widget feature is on only in a workspace build today and off in every per-package build; the chunk's fix must make the widget painted in every build selection, measured on the stand before the fix.
@task_item n=10 part=needs
No web Canvas 2D context for page scripts and no wgpu canvas: an app paints only through the scene API from Rust.
@task_item n=11 part=needs
The CPU renderer paints custom and resource paints transparent; a widget that paints through a texture or a resource is not made to show.
@task_item n=12 part=needs
Every new check is run red first, on the tree before its fix or with the fix reverted, and its red reading is kept in the chunk's evidence beside its green.
@task_item n=13 part=needs
Every build, test and CI cargo leg passes --locked; no new crate or dependency beyond what a taken fork names.
@task_item n=14 part=needs
The escher-session output is read by an agent, not a person: no colour and no terminal test; exit statuses read from the schema's table.
@task_standing n=1
convention: each new test or stand check is built with the system fonts, as the test engine is
home: typography
@task_standing n=2
convention: each new stand check runs in both layout modes, and its reading is recorded with the served tasks' readings, re-recorded on a ruling, never loosened
home: surface
@task_item n=1 part=not_needed
Color palette rows (engine colours, select list colours, color scheme recascade, base colour): the chunk paints no engine colour and changes no colour scheme.
@task_item n=2 part=not_needed
Spacing rows (body margins, table cell spacing, select list placement): the chunk places nothing; a widget at its content box is a later entry.
@task_item n=3 part=not_needed
Motion rows in both motion areas (virtual clock, scroll, scrollbar fades, select list motion): a shot runs no step and moves no clock, and the chunk animates nothing.
@task_item n=4 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon): the chunk sets no cursor and reads no icon.
@task_item n=5 part=not_needed
The web-spa surface, whose canvas is the WASM page's whole drawing surface: the chunk's canvas is a widget painted headless on the desktop engine, not that surface.
@task_item n=6 part=not_needed
Brand identity, border radius, anti-patterns, self-validation and the design decisions log: no rows bear on a headless canvas shot.
