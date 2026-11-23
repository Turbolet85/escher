@task_prose part=chunk
Makes what an app paints from Rust through a custom widget appear in a headless shot: blitz-paint appends the widget's recorded scene, and the harness screenshot and the driver's screenshot verb that rides it show those pixels.
Makes a shot paint a widget in every build selection the project runs (the harness, the driver and the escher-session binary alike), where today blitz-paint's custom-widget feature is on only in a workspace build.
Measures on the stand first: before any fix the stand boots a canvas and a shot of it is taken and read, in both layout modes, in each build selection; every check it adds is run red first and that red reading is kept beside its green.
Collects every fork that is the operator's (which element hosts a canvas, app-painted pixels leaving through the screenshot carriers, whether the stand gains a canvas page or task) into one founder-forks file, none answered by a default.
Re-asserts the screenshot's contract and re-records, never loosens, the stand's served-task readings and kept checks only where a ruling moves one; re-reads the last wrap commit's CI verdict before the push.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: the paint crate's scene builder that appends each custom widget's recorded scene behind its custom-widget feature — build_custom_widget_scenes
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: the paint crate's render path that draws a custom widget's scene into the page — draw_custom_widget
@task_file n=3 path=packages/blitz-test-harness/src/screenshot.rs
what: the test harness's screenshot and screenshot_rect, rendering the page to a buffer with the CPU renderer — the shot the driver's screenshot verb rides
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait an app paints through, its paint call recording a scene from a render context, styles, size and scale
@task_file n=5 path=packages/dioxus-native-dom/src/mutation_writer.rs
what: the Dioxus native mutation writer that sets a custom widget from an object element's data attribute and leaves a canvas element's source unread
@task_item n=1 part=needs
The custom widget paint path: a widget's recorded scene appended to the page scene behind blitz-paint's custom-widget feature, and that feature reaching every build selection the project runs (harness, driver, escher-session binary, per-package and workspace builds alike).
@task_item n=2 part=needs
The screenshot's contract re-asserted with widget pixels in it: an 800 x 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable refused before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=3 part=needs
The screenshot answer as it crosses: carried in band as a png on the session socket, the command line and the MCP screenshot tool result under its size bound; screen pixels in band as the ratified class.
@task_item n=4 part=needs
App-painted canvas pixels leaving through the screenshot answer, the MCP result and a kept check's shot record are a content class of their own beyond a shot's pixels and an open select list's labels as pixels: a boundary widening under the boundary widening rule, written as an operator fork, not taken by default.
@task_item n=5 part=needs
Which element hosts a canvas (an object element's data attribute carrying a written-once widget, as the Dioxus path stands, or a canvas element's source, which nothing reads): a fork that turns what the canvas is, including how the custom widget's initial attribute replay reaches it.
@task_item n=6 part=needs
No new driver verb, argument, argument kind, carrier, refusal cause, session error, exit code, log field or span unless a fork's ruling adds one; the closed verb schema and the single validation gate stand.
@task_item n=7 part=needs
No content in a log field: what a canvas paints stays out of logs, the command span and the telemetry sink's allowlisted fields, as screen content does.
@task_item n=8 part=needs
Every build, test and CI cargo leg passes --locked, and no new crate or dependency beyond what a taken fork names; turning on a feature of an engine crate stays within the workspace-inherited dependency versions and the engine dependency pins.
@task_item n=9 part=needs
A feature change on the test harness keeps it taking no tracing and no style.
@task_item n=10 part=needs
The shot's encoder stays png, with no base64 crate added for a canvas shot.
@task_item n=11 part=needs
The stand's seven served tasks keep their snapshot, lint and actions digests and the committed kept checks and stand checks keep their readings unless a ruling moves one, then re-recorded, never loosened.
@task_item n=12 part=needs
Each new control is shown with its red reading: every check this chunk adds is run red first, on the tree before its fix or with the fix reverted, and that red reading is kept in the chunk's evidence beside its green, in both layout modes.
@task_item n=13 part=needs
No web Canvas 2D context for page scripts and no wgpu canvas: the canvas is painted from Rust through the scene API only; the CPU renderer's transparent custom and resource paints stay a stated limit, not made to show.
@task_item n=14 part=needs
Whether the stand gains a canvas page or task: a fork for the operator, and any served task it adds meets the stand's committed-check and kept-check rules.
@task_item n=15 part=needs
The last wrap commit's CI verdict is unread and never folded as green; it is re-read before this chunk's push, with the CI matrix running every job.
@task_standing n=1
convention: each new test, stand check or CI leg builds and runs with --locked under the locked-builds rule
home: dependency-security
@task_standing n=2
convention: each new test counts toward the per-crate line coverage floors and the coverage floor check stays met
home: input-validation
@task_standing n=3
convention: each new evidence file, red or green reading kept for a check holds no credentials and no screen content in a log line
home: secret-management · logging-monitoring
@task_standing n=4
convention: each new committed kept check or stand check keeps each answer whole, holds closed content and stays owner-only where kept
home: data-protection
@task_item n=1 part=not_needed
Outbound network requests, fetch methods, per-host limits, CORS, same-origin, TLS, request timeouts and response size limits: a canvas painted from Rust fetches nothing.
@task_item n=2 part=not_needed
Authentication and authorization, CI tokens, publish jobs, signing material and release publishing: the chunk touches no credential or release path.
@task_item n=3 part=not_needed
Browser history, cookies, the HTTP cache, clipboard and native file dialogs: no stored browser data is reached.
@task_item n=4 part=not_needed
Page scripting: JavaScript opt-in, the CSSOM script API, JS module loading, URLSearchParams and bootstrap helpers, since the canvas has no script context.
@task_item n=5 part=not_needed
Select list keyboard, pointer, wheel, typeahead, row choice and choice exits, and range input actions: the chunk changes no form control behaviour.
@task_item n=6 part=not_needed
The cold-agent, agent-run, apt-install and build-directory sweep scripts and their argument allowlists and exit codes.
@task_item n=7 part=not_needed
The WPT runner and its post-results workflow, quarantine entries and result kinds, and rdme's raw HTML passthrough.
@task_item n=8 part=not_needed
Stable element id derivation, its virtual-node form and keyed list segments: virtual nodes and a canvas read as data are later epochs.
@task_item n=9 part=not_needed
The CDP devtools server and its environment variables, and the skia build fetch.
@task_item n=10 part=not_needed
The virtual clock, advance, scrollbar fade and load-wait bounds: a shot runs no step and moves no time.
@task_item n=11 part=not_needed
Iframe nesting, CSS import depth, stylesheet loading and presentational hint parsing.
