@task_prose part=session
The session made what an app paints from Rust through a custom widget appear in a headless shot, measured on the stand: a harness screenshot and a driver screenshot (whole and cropped to the element) carry the widget's pixels, in both layout modes, in process only.
blitz-test-harness now names blitz-paint's custom-widget feature, so blitz-paint's widget pass is compiled into every per-package build selection reaching the harness (the harness, escher-driver, seven_guis and so the escher-session binary), not only the workspace build.
blitz-tests gains one stand check target, stand_canvas_shot, which joins stand-repeat and agent-run's run stand by the existing stand_ prefix, declares the shared session_common module, and takes peniko as a dev-dependency from the workspace (one lock line, no new package).
No public symbol, verb, argument, result field, refusal cause, session error, exit code, log field, span, carrier, crate or source file under packages changed; the driver graph still holds no platform adapter; CI went green 17 of 17.
The canvas content crosses only as pixels in a screenshot, ratified by the founder for every carrier; no served app holds a canvas yet, so no carrier carries one today.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_shot.rs
what: a new blitz-tests stand check (a stand_ target, one target per file) that mounts a Dioxus component holding a custom widget painted from Rust and reads its pixels in a harness screenshot and in a driver session's whole and element screenshot, in both layout modes; it declares the shared session_common test module
@task_file n=2 path=packages/blitz-test-harness/Cargo.toml
what: blitz-test-harness's manifest, the headless test harness's dependencies and the cargo features it turns on in them; its blitz-paint dependency now names the custom-widget feature
@task_file n=3 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests integration-test crate's manifest, its dev-dependencies declared from the workspace; it gains peniko
@task_file n=4 path=Cargo.lock
what: the workspace lockfile; the blitz-tests entry gains peniko, no package added
@task_item n=1 part=touches
Custom content as an object element and the custom widget embedding contract: an app's widget now paints in a headless harness and driver shot; the row may need to say the widget's paint reaches the shot.
@task_item n=2 part=touches
The Widget trait contract of blitz-dom nodes and widgets: a test-private widget implements it and its paint output is now witnessed in a headless shot.
@task_item n=3 part=touches
Widgets and hit-testing feature-gated, and optional capabilities as cargo features: blitz-paint's custom-widget feature is now turned on by blitz-test-harness, so the widget pass is compiled into every selection reaching the harness.
@task_item n=4 part=touches
Default features fixed per crate and in-repo crates without default features: the harness now names one more feature on an in-repo dependency explicitly; check the rows still hold.
@task_item n=5 part=touches
Driver names only the accessibility feature: the driver's own feature names are unchanged, but its build graph now carries blitz-paint's custom-widget through the harness; check the row's wording.
@task_item n=6 part=touches
Driver graph holds no platform adapter: re-measured under the moved feature set, still none (the probe reads 0).
@task_item n=7 part=touches
Screenshot pixels cross in band, screen pixels in band and the screenshot image carried in band: app-painted canvas content now rides these as pixels, with no new carrier, field or verb.
@task_item n=8 part=touches
New carriers widen the session boundary, and the select list crossing only as bounds and pixels: the precedent beside which the canvas crossing lands; no new carrier was added.
@task_item n=9 part=touches
The test harness screenshot and screenshot_rect contract, harness screenshots over opaque white and the Vello CPU renderer for headless screenshots: the new stand check reads a widget's red fill over the white background through them.
@task_item n=10 part=touches
The driver verbs screenshot contract, the per-verb target check for screenshot with an id and the session socket's screenshot answer: the element shot of the widget's element is read whole and cropped, shape unchanged.
@task_item n=11 part=touches
Paint emits commands to a paint scene and the blitz-paint paint_scene contract: blitz-paint's widget scene pass is now compiled into harness, driver and session builds.
@task_item n=12 part=touches
Kept check keeps masking, kept check never logged and the model provider reached only on the live cold-agent run: a canvas's pixels may cross on every ratified carrier including the model provider, while a kept check keeps no pixels.
@task_item n=13 part=touches
Integration tests live in blitz-tests, one test target per file in blitz-tests, and the blitz-tests manifest declaring one test table: one stand_ target added by file.
@task_item n=14 part=touches
Stand-check helpers in non-target modules and shared test modules in subdirectories: the new target declares the shared session_common module; any row stating the count of its declarers moves by one.
@task_item n=15 part=touches
Scenarios in both layout modes: the new check runs each case with incremental layout off and on.
@task_item n=16 part=touches
Stand-repeat zero-failure budget, the ci-leg stand-repeat leg and committed stand checks: one more stand_ target is read by the repeated stand run; any row holding the count of stand_ readers moves by one.
@task_item n=17 part=touches
Dependencies declared once in the workspace, the dependency and licence audit and locked cargo legs: peniko joins blitz-tests as a workspace dev-dependency, lock gains one line, audit green.
@task_item n=18 part=touches
Fixed program text on the carriers and the cold-agent run carrying the stand's screen: the canvas crossing adds app-painted pixels beside them, not text; check the rows do not imply only program text or page text crosses.
@task_fact n=1 slug=established-decisions
fact: A canvas's app-painted content crosses the driver's carriers only as pixels in a screenshot, on every ratified carrier, with no new carrier, verb, field, exit, refusal cause, session error, log field or span; ratified by the founder.
@task_fact n=2 slug=established-decisions
fact: blitz-test-harness names blitz-paint's custom-widget feature so a harness or driver shot paints an app's widget in every build selection reaching the harness, guard stand_canvas_shot.
@task_item n=1 part=untouched
Other senses of canvas: wasm_hello's host-sized canvas, Circle Drawer's canvas handlers and the handler coverage canvas item, the layout answer's canvas key and the replaced-element list are not reached.
@task_item n=2 part=untouched
The session wire, refusal causes, session errors, exit codes, MCP server and role contracts, and the command line: no verb, argument, field or cause changed.
@task_item n=3 part=untouched
Telemetry, tracing spans, log lines and scrubbing: no log field or span added.
@task_item n=4 part=untouched
Occupied names: no crate, binary, environment variable, port or listener taken or freed.
@task_item n=5 part=untouched
Engine layout, style, select list, scroll, hit testing, events, accessibility tree and form controls in blitz-dom: no engine source changed.
@task_item n=6 part=untouched
The browser app, WPT runner, script runtime, networking, upstream sync and coverage floors: none reached.
