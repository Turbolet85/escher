@task_prose part=chunk
Makes what an app paints from Rust, through blitz-dom's Widget trait, appear in a headless shot: the harness's screenshot and screenshot_rect, the driver's screenshot verb that rides it, and the escher-session binary.
Measures on the stand first, before any fix: the stand boots a canvas and a shot of it is taken and read, in both layout modes, in each build selection the project runs it under (blitz-paint's custom-widget feature is on only in a workspace build today).
Makes a shot paint a widget in every build selection, so a check's reading does not hang on which packages a cargo invocation selected.
Collects every fork that is the operator's (which element hosts a canvas, app-painted pixels leaving on the carriers, a stand canvas page or task) in one founder-forks evidence file, answering none by a default.
Runs each new control red first and keeps its red reading beside its green; re-reads the last wrap's CI verdict at P3 and before the push.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: blitz-paint, the paint scene builder that appends a custom widget's recorded scene behind its custom-widget feature — build_custom_widget_scenes
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: blitz-paint rendering, the widget drawing step behind the custom-widget feature — draw_custom_widget
@task_file n=3 path=packages/blitz-test-harness/src/screenshot.rs
what: the test harness's screenshot and screenshot_rect, rendered on the Vello CPU renderer over opaque white
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: blitz-dom nodes and widgets, the Widget trait an app paints through into a scene
@task_file n=5 path=packages/dioxus-native-dom/src/mutation_writer.rs
what: the Dioxus DOM bridge's mutation writer, which sets a custom widget from an object element's data attribute
@task_item n=1 part=needs
The Widget trait's paint into a recorded scene, and blitz-paint appending that scene into the paint scene behind its custom-widget feature: a widget's pixels must show in the painted scene a shot renders.
@task_item n=2 part=needs
The harness's screenshot and screenshot_rect on the Vello CPU renderer, and the driver's screenshot verb and the escher-session screenshot command that ride it, must show a widget's painted pixels.
@task_item n=3 part=needs
Which features a build selection gives blitz-paint: custom-widget is off in every per-package build (harness, driver, seven_guis, the escher-session binary) and on only in a workspace build by feature unification; a shot must paint a widget in every selection the project runs, within the rules on feature gating, per-crate default features and in-repo crates without default features.
@task_item n=4 part=needs
Measured on the stand first: the seven_guis headless stand boots a canvas and a shot of it is taken and read before any fix, in both layout modes and in each build selection.
@task_item n=5 part=needs
Which element hosts a canvas is an operator's fork, not a default: the object element carrying a widget in its data attribute (the custom widget embedding as it stands, with object excluded from the replaced elements) against the canvas element's special element data, which nothing reads today.
@task_item n=6 part=needs
App-painted pixels leaving through the screenshot answer is an operator's fork: the shot crosses in band on the session socket, the command line, MCP and a kept check's shot record, and a canvas's painted content is a content class of its own beside the ratified shot pixels and select list labels.
@task_item n=7 part=needs
Whether the stand gains a canvas page or task is an operator's fork, read against the session apps table and the stand's served tasks.
@task_item n=8 part=needs
Each operator's fork goes into the one founder-forks file with what crosses, the rule it meets, its options with measured cost and its dependences, checked against the escalate patterns; technical forks go to the operator as dialogs.
@task_item n=9 part=needs
The screenshot's contract stands: an 800 by 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=10 part=needs
No new verb, argument, carrier, refusal cause, session error, exit status, log field or span unless a fork's ruling adds one; no content in a log field; the validated verb set and the session boundary widen only by ratification.
@task_item n=11 part=needs
The stand's served tasks keep their readings: the seven tasks' snapshot, lint and actions digests across OSes, the committed kept checks and the stand checks hold unless a ruling moves one, then re-recorded, never loosened.
@task_item n=12 part=needs
Every build, test and CI cargo leg runs locked; no new crate or dependency beyond what a taken fork names.
@task_item n=13 part=needs
Each new control is run red first, on the tree before its fix or with the fix reverted, and its red reading is kept in the chunk's evidence beside its green.
@task_item n=14 part=needs
Every stand check and new scenario runs in both layout modes, incremental and non-incremental.
@task_item n=15 part=needs
Carve-out: the CPU renderer paints custom and resource paints transparent with no warning; a widget painting through a texture or a resource is not made to show.
@task_item n=16 part=needs
No web Canvas 2D context for page scripts and no wgpu canvas: the canvas is painted from Rust through the scene API alone.
@task_item n=17 part=needs
A change inside an upstream body (blitz-paint, the harness, the Dioxus bridge) is written as an addition in a function of escher's own and flagged as a direct engine change that may go upstream.
@task_item n=18 part=needs
The last wrap's CI verdict on ac8e6d58a is unread and never folded as green: re-read at P3 and before this chunk's push.
@task_standing n=1
convention: each new integration test lives in blitz-tests, one test target per file, one behaviour per file, with a descriptive name
home: design-philosophy · conventions
@task_standing n=2
convention: each new scenario runs in both layout modes and asserts its fixture's condition first
home: conventions
@task_standing n=3
convention: each new stand check keeps its shared helpers in non-target modules and holds the stand's zero-failure repeat budget
home: conventions · standard-contracts
@task_standing n=4
convention: each new public item and module carries docs, with rustdoc warnings as errors
home: conventions
@task_item n=1 part=not_needed
The reference browser app: chrome, tabs, about pages, history storage, persistence and its publish workflow; no canvas reaches it.
@task_item n=2 part=not_needed
The WPT runner, its outputs, result kinds, statuses, CI artifacts and PR-body tooling.
@task_item n=3 part=not_needed
The JS runtime: Boa, script globals, script fetching, CSSOM from scripts and the script document; no script reaches a canvas in this version.
@task_item n=4 part=not_needed
The select model and open select list machinery: its default actions, keyboard model, rows and readers.
@task_item n=5 part=not_needed
The accessibility tree, AccessKit adapters and ARIA state readings.
@task_item n=6 part=not_needed
The layout reason readers and the layout verb's answer words; widget placement at its content box is a later entry.
@task_item n=7 part=not_needed
The cold-agent runs, their events, exit codes and verdict fields, and the agent-run script.
@task_item n=8 part=not_needed
Exploration, the UI graph and its registry, and stable element id tiers; a canvas read as data, virtual nodes, drag, pan and zoom are later epochs.
@task_item n=9 part=not_needed
Networking: blitz-net, the HTTP cache, per-host request caps, URL scheme routing and the net provider.
@task_item n=10 part=not_needed
Platform shells and targets: winit windows, Android and iOS entries, wasm and Trunk builds, the Nix flake.
@task_item n=11 part=not_needed
Upstream sync by two-parent merge and its merge base; this chunk takes nothing from another tree.
@task_item n=12 part=not_needed
Text and font layout through Parley, text decoration and canvas text from the bundled fonts; canvas text is a later entry.
