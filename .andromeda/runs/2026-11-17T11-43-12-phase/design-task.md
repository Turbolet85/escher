@task_prose part=chunk
Takes upstream/main at e5305a4c1 (ten commits past the last take) as a cherry-pick onto this fork's pins, measured on a fresh trial worktree; AnyRender moves to the 0.15 registry set and left-out commits whose reasons no longer hold return.
The take brings external images in SVG, root SVG background painting, root and body backgrounds painted only on the canvas, the synthetic italic/oblique shear direction fix and scroll offsets clamped after layout rounding.
Every conflict is resolved keeping both sides' intent; every escher proof is re-run (fast and doc legs, stand checks in both layout modes, the incremental oracle, a11y leg, audit, embolden witness, screenshot readings re-measured) and fork CI must be green.
Per-OS test tallies and the sink allowlist are re-read; the carried merge surfaces and the left-out table are re-read against the new pin.
The CI disk reds get a line: both runs' logs are read and either a pinned free-disk step is added to the default-features test job or the measured other cause is stated.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest — its dependency pins (AnyRender set, Taffy git rev, Dioxus, parley)
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock file — the locked packages of the coupled dependency pins
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow — its jobs and their pinned actions, read-only contents permission — edits: the Test [default features] job (a free-disk step)
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — handle_click's arms as private click_* functions, an upstream arm edit re-applied by hand
@task_file n=5 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the ban on Request::get in blitz-dom outside stamped_request, checked against the new SVG image loads
@task_file n=6 path=packages/blitz-dom/src/layout/inline/layout.rs mark=re-reads
what: the Taffy merge surface — the place_floated_box line as upstream wrote it, re-read against upstream's Taffy pin
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one test file and one target per behaviour
@task_file n=8 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs re-run after the take
@task_file n=9 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing log capture checks — zero needles after the take
@task_file n=10 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow invariants check, including the 40-hex SHA pin of every action
@task_item n=1 part=needs
The paint changes the take brings — root SVG background painting and propagated root and body backgrounds painted only on the canvas — change what every document's background paints; engine colours stay as the engine sets them, and each is a boundary widening listed for the operator.
@task_item n=2 part=needs
The synthetic italic/oblique shear direction change alters how faux-oblique text paints; the screenshot never paints with text skipped and the embolden witness is re-run.
@task_item n=3 part=needs
Screenshot readings may move with AnyRender 0.15, canvas backgrounds and shear; a moved reading is re-measured and recorded, never loosened, and tests stay built with system fonts.
@task_item n=4 part=needs
Scroll offsets clamped after layout during layout rounding reach the scrolling surface: driver scroll stays instant, into-view scroll in nested boxes stays instant, and the scrolled-box bounds, reversed scroll origins and into-view readings keep escher's behaviour.
@task_item n=5 part=needs
Hit-testing still stops where paint clips, stacking-context roots still group paint and hit-testing, and positioned auto z-index keeps its paint level through every resolution.
@task_item n=6 part=needs
The select model and its list keep escher's behaviour through the take: the list as an engine overlay painted after the root render, answering hits first, placed inside the viewport, opening with no motion, its events and its snapshot and accessibility readings.
@task_item n=7 part=needs
The clock invariant holds: animation driven by the one virtual clock, driver advance moving that clock, settle not waiting on animation, load wait moving no clock, and the driver verbs' clock contracts.
@task_item n=8 part=needs
The stable ids, the snapshot and the accessibility state reads (including inert and the range and keyboard models) keep escher's behaviour; each conflict resolution is recorded.
@task_item n=9 part=needs
dioxus-native-dom keeps escher's buffered render — rendered buffered then replayed under one borrow — and MountedData calls during a render keep their document-state and focus contracts; the mounted-event test stays out until the Dioxus 0.8 move.
@task_item n=10 part=needs
External images in inline and top-level SVG are a new sub-resource fetch on every document, remote pages included; it is read against the Request::get ban and the file: refusal and listed for the operator, never settled silently.
@task_item n=11 part=needs
Coupled dependency pins: upstream's AnyRender registry set is taken as one set or none, Dioxus 0.7.3 and parley 0.12 stay held, and a moved layout, text or paint number is re-measured, never loosened.
@task_item n=12 part=needs
The workspace takes no git dependency save Taffy's pinned rev; AnyRender comes from the registry; MSRV stays 1.91; new and removed dependencies are reviewed by hand and by the audit's licence check.
@task_item n=13 part=needs
Additivity: edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded; no new crate, port, env var or listener of ours, and the CDP listener commit stays out.
@task_item n=14 part=needs
Upstream's new and edited test files run under the fork's blitz-tests rule, one file and one target per behaviour.
@task_item n=15 part=needs
The CI disk reds get a measured line: a free-disk step added to the default-features test job is pinned to a 40-hex SHA and keeps the workflow's read-only permission; otherwise the measured other cause is stated.
@task_item n=16 part=needs
The sink allowlist is re-checked over the taken delta for every new tracing or log call site, target, print and env read, and per-OS test tallies are re-read with each moved count explained.
@task_standing n=1
convention: every test the take brings or edits is built and run with system fonts, never a bundled font context
home: typography
@task_item n=1 part=not_needed
Brand identity and border radius — no recorded intent, and the take brings no brand or radius change.
@task_item n=2 part=not_needed
Spacing presentational hints — body right and bottom margins, table cellspacing, cellpadding and valign — the take changes no spacing hint.
@task_item n=3 part=not_needed
Iconography — cursor icons, cursor resolution order and favicon reading — untouched by the ten commits.
@task_item n=4 part=not_needed
WPT runner output and result colours — the WPT-only commits change test expectations, not the runner's output, and WPT numbers are out of scope.
@task_item n=5 part=not_needed
The shell provider's window controls (decorations toggle, maximize, minimize, window drag) and the window compositing alpha mode — no commit of the take reaches them.
@task_item n=6 part=not_needed
The JS window object's size and devicePixelRatio contracts and the logical viewport from scale — the take changes no viewport or script window reading.
@task_item n=7 part=not_needed
The web-spa surface, WASM bundled font context and mobile targets built not tested — the take reaches no WASM or mobile path.
@task_item n=8 part=not_needed
Color-scheme change recascade and the viewport's system color scheme — the background changes paint, they do not change scheme handling.
@task_item n=9 part=not_needed
escher-session's colour and terminal rules and the driver command line output for agents — the take adds no driver verb, answer or output.
@task_item n=10 part=not_needed
Overlay scrollbar fade timings and scrollbar thumb drag — the scroll clamp changes offsets at layout rounding, not the scrollbar's motion or drag.
