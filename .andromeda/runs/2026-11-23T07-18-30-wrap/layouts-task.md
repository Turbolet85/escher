@task_prose part=session
The session made a canvas painted by an app from Rust through a custom widget show up in a headless shot, checked on the stand first.
It added one stand check, stand_canvas_shot, a test-only app holding a widget element (id painted-canvas) that fills it red, run in both layout modes, per package and in the workspace build.
The harness shot shows the widget's pixels (red inside, white outside, snapshot bounds 8, 8, 200, 100); the driver's whole shot (800 x 600) decodes equal to the harness raster and its element shot of the widget is red in every pixel, in process only.
The test harness now turns on the paint crate's custom-widget feature, so the widget paint pass is built into the harness, the driver and the escher-session binary in every build selection; blitz-tests took peniko as a dev-dependency (one lock line, no new package).
No verb, argument, result field, exit, cause, session error, log field or span was added; no served task holds a canvas; no product source changed; nothing retired.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: a new stand check (a stand_* target read by stand-repeat and by agent-run's run stand) - a widget painted from Rust shows in a harness shot and in a driver whole and element shot, both layout modes
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: the headless test harness's manifest - its dependencies and the features it turns on, now the paint crate's custom-widget feature
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the manifest of the blitz-tests crate holding the stand checks - its test targets and dev-dependencies, now peniko
@task_file n=4 path=Cargo.lock
what: the workspace lock file - the resolved dependency graph, one line added to the blitz-tests entry
@task_item n=1 part=touches
The escher-session screenshot answer: a shot now carries an app-painted widget's pixels; its fields are unmoved, so the answer's shape holds, only what can appear in the image grew.
@task_item n=2 part=touches
The escher-session MCP server screenshot result: the same crossing, a canvas's content reaching the caller as pixels in the image only, no new field.
@task_item n=3 part=touches
The escher-session snapshot answer and its --part element shot: a widget element reports its bounds and crops to its painted pixels like any element.
@task_item n=4 part=touches
Headless stand pinned viewport: the new check reads the whole shot at the pinned 800 x 600; a row holding that size or the shot size is confirmed, not moved.
@task_item n=5 part=touches
Session roles share the stand boot: the new check starts its own test-only session app (named canvas), not the stand's boot; a row saying every session in the stand checks boots the stand is the one to read against it.
@task_item n=6 part=touches
Stand stable element ids and stand ids as attributes alone: the test app takes the id painted-canvas as an attribute on its widget element; it is not a stand id and no served task holds it.
@task_item n=7 part=touches
Escher-session served tasks: no served task gained a canvas or widget; the list stands, and a row saying no served app paints custom content stays true.
@task_item n=8 part=touches
agent-run.sh test contract (usage, stdout): the run stand selection gains one stand_* target; a row stating the count of stand checks it runs moves by one, a row naming the glob alone does not.
@task_item n=9 part=touches
The escher-session command line and binary: the widget paint pass is now compiled into it in every build selection, with no verb, argument or exit added.
@task_item n=10 part=touches
Replaced or custom content as an object element: the widget paints inside such an element, and that paint now reaches a shot.
@task_item n=1 part=untouched
ShellProvider window chrome controls: the shot runs headless in process, no window chrome is driven.
@task_item n=2 part=untouched
Circle Drawer's circle-canvas, wasm_hello's canvas and the layout answer's canvas key: another sense of canvas, none of them a widget painted from Rust.
@task_item n=3 part=untouched
escher-session exit codes, refusals (fonts-unavailable, off-screen, unknown-verb) and recording (check files, record dir): nothing added or moved.
@task_item n=4 part=untouched
cold-agent.sh run pipe contract and the cold-agent decisions: no cold agent run reads a canvas, no prompt or pipe changed.
@task_item n=5 part=untouched
multi_doc and paint_bench command lines: no argument, env var or output changed.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the absence of arbitrary index.html templates: no layout, page or document skeleton changed.
