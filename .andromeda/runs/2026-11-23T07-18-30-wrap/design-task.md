@task_prose part=session
The session made what an app paints from Rust through a custom widget appear in a headless screenshot: the harness shot and the driver's screenshot, whole and cropped to the widget's element, in both layout modes.
It measured it on the stand first: a new stand check paints a red widget and reads red inside its box, white outside, the element shot 200 by 100 all red, the whole driver shot equal to the harness raster.
The test harness now compiles the paint crate's custom-widget pass in every build selection that reaches it (the harness, the driver, the escher-session binary, seven_guis).
No verb, argument, result field, exit status, cause, session error, log field or span was added; the canvas crosses only as pixels in a screenshot, in process only, since no served task holds a canvas yet.
Nothing was retired; CI on the pre-CI commit is green 17 of 17.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: a new stand check, the canvas shot check, witnessing that an app's custom widget painted from Rust shows in a harness screenshot and in a driver screenshot whole and cropped to its element, in both layout modes; it joins the stand checks repeated by their stand_ prefix
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: the headless test harness's manifest; its paint dependency now names the custom-widget feature, so a headless shot paints an app's widget in every build selection reaching the harness, the driver and escher-session
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the manifest of the engine test crate that holds the stand checks; it gains peniko as a dev-dependency for the widget's paint
@task_file n=4 path=Cargo.lock
what: the workspace lockfile; one line added, peniko named under the engine test crate, no package added
@task_item n=1 part=touches
The escher-session screenshot command and the driver screenshot (whole and cropped to an element by id): their shape is unmoved, but the pixels they carry may now hold an app's custom widget content painted from Rust; a row stating what a screenshot paints sits beside this.
@task_item n=2 part=touches
The mcp role's screenshot tool result: no field moved, its image may now carry a custom widget's app-painted pixels.
@task_item n=3 part=touches
The decision that a screenshot never paints with text skipped: beside it now stands that a screenshot never skips a custom widget's paint pass, in any build selection.
@task_item n=4 part=touches
The decisions that the test engine and the tests are built with system fonts: rules over the headless harness's build, whose feature set changed (the custom-widget paint pass compiled in).
@task_item n=5 part=touches
The snapshot command: a custom widget's element reports its bounds like any element (measured at 8, 8, 200 by 100); no part or field added.
@task_item n=6 part=touches
Paint-order and overlay rows (the select list painted after the root render, the select list as an engine overlay, stacking-context grouping of paint): the custom widget's paint pass now runs inside the headless render they order; no order changed.
@task_item n=7 part=touches
The decisions that escher-session and the driver's output are read by an agent: the canvas crosses to that reader only as pixels in a screenshot, no new carrier.
@task_fact n=1 slug=surface
fact: An app's custom widget painted from Rust appears as pixels in a headless screenshot, the harness shot and the driver screenshot whole and cropped to its element, in both layout modes and in every build selection; the canvas crosses only as pixels, with no verb, argument, field, exit, cause or session error of its own.
@task_fact n=2 slug=surface
fact: A canvas in a screenshot is in process only: no served escher-session task holds a canvas, so no carrier of the session carries one yet.
@task_item n=1 part=untouched
Colour, typography, spacing, depth, radius and brand token rows and absences: the test widget's red fill is test content, no token or engine colour moved.
@task_item n=2 part=untouched
Motion and the virtual clock (advance, click, press, type, scroll, animation, settle): no timing or clock behaviour changed.
@task_item n=3 part=untouched
The single select's list events, its accessibility rows and its list colours and placement: untouched.
@task_item n=4 part=untouched
escher-session exit statuses and exit codes, stderr usage and session errors, the remedy field: none added or moved.
@task_item n=5 part=untouched
The escher-session task list (counter, crud, timer, cells, flight-booker, order, load, unknown task): no task added; no served app holds a canvas.
@task_item n=6 part=untouched
The actions, coverage, graph and layout answers and their lines, including the layout answer's canvas key: another sense of canvas, unmoved.
@task_item n=7 part=untouched
The web-spa surface's WASM canvas the page renders into: another sense of canvas, unchanged.
@task_item n=8 part=untouched
The WPT runner output and colours, iconography and cursors, the shell provider's window controls and the JS window object: not reached.
