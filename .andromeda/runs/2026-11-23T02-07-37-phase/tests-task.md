@task_prose part=chunk
Makes what an app paints from Rust, through blitz-dom's Widget trait and its recorded scene, appear in a headless shot: the harness's screenshot and screenshot_rect on the CPU buffer, the driver's screenshot verb that rides it, and the escher-session binary's screenshot.
Measures on the stand first, before any fix: the stand boots a canvas and a shot of it is taken and read, in both layout modes, under each build selection the project runs it in.
Makes a shot paint a widget in every build selection (harness, driver, seven_guis and the escher-session binary alike, not only a workspace build), so a check's reading does not hang on which packages a cargo invocation selected.
Shows every check it adds red first, on the tree before its fix or with the fix reverted, its red reading kept beside its green.
Re-asserts the screenshot contract and the stand's served-task readings, committed kept checks and stand checks unchanged unless an operator ruling moves one.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget boundary an app paints through — the Widget trait's paint into a render context returning a scene
@task_file n=2 path=packages/blitz-paint/src/lib.rs
what: the scene painter's custom-widget feature and its building of the custom widget scenes behind that feature
@task_file n=3 path=packages/blitz-paint/src/render.rs
what: the scene painter's drawing of a custom widget's recorded scene into the page's scene
@task_file n=4 path=packages/blitz-test-harness/src/screenshot.rs
what: the harness screenshot — screenshot and screenshot_rect rendered to a CPU buffer, the shot the driver's screenshot verb rides
@task_file n=5 path=packages/dioxus-native-dom/src/mutation_writer.rs
what: the Dioxus mutation writer that hands a custom widget to the document from an object element's data attribute; a canvas element's paint source is parsed and read by nothing
@task_item n=1 part=needs
The harness screenshot and screenshot_rect, rendered on the CPU buffer, show the pixels a custom widget paints through the Widget trait's recorded scene.
@task_item n=2 part=needs
The driver session's screenshot verb and the escher-session screenshot command and MCP screenshot tool result, which ride the harness shot, carry the widget's painted pixels in band.
@task_item n=3 part=needs
The scene painter's custom-widget feature is on in every per-package build selection the project runs (harness, driver, seven_guis, escher-session binary, blitz-tests), not only in a workspace build; a per-package green is not the workspace green.
@task_item n=4 part=needs
Measured on the stand first: the seven_guis headless stand boots a canvas and its shot is taken and read before any fix, in both layout modes, under each build selection.
@task_item n=5 part=needs
Every check the chunk adds is controlled red-before-green: run red on the tree before the fix or with the fix reverted, its red reading kept in the chunk's evidence beside the green.
@task_item n=6 part=needs
The screenshot contract stands: an 800 x 600 shot over opaque white, its PNG byte-identical across processes on one host, refused fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=7 part=needs
No new verb, argument, carrier, refusal cause, session error, exit code, log field or span unless an operator fork ruling adds one; no screen content in any log line, stderr line or span field.
@task_item n=8 part=needs
App-painted canvas content leaving through the screenshot answer on the session socket, the command line, MCP and a kept check's shot record is an operator fork (a content class of its own beside a shot's pixels and an open select list's labels); committed recorded checks keep holding no screenshot step.
@task_item n=9 part=needs
The stand's seven served tasks keep their cross-OS snapshot, lint and actions digests, and the committed kept checks and stand checks keep their readings; a ruling that moves one re-records it, never loosens it; cross-OS byte comparisons hold no shot.
@task_item n=10 part=needs
Which element hosts a canvas (an object element's data as upstream's Dioxus path stands, or a canvas element nothing wires) and whether the stand gains a canvas page or task are operator forks, never answered by a default.
@task_item n=11 part=needs
The CPU renderer's custom-paint gap stays out: a widget painting through a texture or a resource is not made to show; no web Canvas 2D context and no wgpu canvas.
@task_item n=12 part=needs
Every build, test and CI cargo leg passes --locked; no new crate or dependency beyond what a taken fork names.
@task_item n=13 part=needs
Stand checks run on the bundled pinned font with no skip, offline, and hold their zero-failure budget across the three stand-repeat rounds.
@task_item n=14 part=needs
Every requirement the chunk adds is witnessed inside stand-repeat, and a capability claim for the canvas names its checks (whether this chunk claims v040-09 is read at P4).
@task_standing n=1
convention: each new integration test file is a target of its own, one behaviour per target, and a shared module it uses is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new test's expected value is pinned as a literal, and its failure prints no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: each new test's data and fixtures are stated in code, nothing seeded, shared fixtures and stand tables in one module
home: 3-test-harness-contract · 7-test-data-fixtures
@task_standing n=4
convention: each new check reads no wall clock and sleeps never; time moves only by the virtual clock
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=5
convention: each new pixel test reads a CPU buffer and decodes its PNG in the check, comparing frames whole
home: test-strategy · 2-test-strategy
@task_standing n=6
convention: each new stand-in is hand-written, no mocking library
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=7
convention: each new code path is held by the per-crate line coverage floors and the zero-failure budget for stand checks
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
The WPT conformance runner, its command line, outputs, status set, pinned commit and fork reading: the chunk runs no WPT suite.
@task_item n=2 part=not_needed
The cold-agent pipe: its events, verbs, exit codes, verdict, isolation, prompt and live run: no cold-agent run is part of this chunk.
@task_item n=3 part=not_needed
The agent-run contract: its events, verbs, selections, state files and exit codes: the chunk adds no agent-run verb or selection.
@task_item n=4 part=not_needed
The apt-install and coverage-floor scripts' exit codes, CI actions pinned to SHAs, CI secrets and read-only permissions, and the CI Python scripts' unittest checks: the chunk edits no CI script or workflow permission.
@task_item n=5 part=not_needed
The chosen select row colour pair, the open pseudo-class, the select list painted by the scene painter, and the password caret placement: the chunk changes no form control's painting.
@task_item n=6 part=not_needed
The MSRV build, the skia matrix archive and the local embolden witness: the chunk touches no platform matrix or skia build.
