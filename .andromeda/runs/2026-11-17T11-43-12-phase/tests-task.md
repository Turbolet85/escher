@task_prose part=chunk
Takes upstream/main at e5305a4c1 (10 commits past ea07ee716) as a cherry-pick onto this fork's pins, measured on a fresh trial worktree; each commit left out keeps its measured reason.
Brings external images in SVG, AnyRender 0.15 from the registry, root SVG and canvas-only propagated backgrounds, synthetic oblique shear direction, scroll offsets clamped at layout rounding, and WPT-only changes.
Re-reads the left-out delta's reasons (the AnyRender git pin first), the Taffy pin, the merge surfaces row by row, the Request::get ban against SVG image loads, and the sink allowlist over the taken delta.
Re-runs every escher proof (fast and doc legs, stand checks in both layout modes, the incremental oracle, a11y leg, dependency audit, embolden witness, screenshot readings) and re-reads the per-OS tallies of the take's green CI run.
Gives the CI disk reds a line: measures from the two runs' logs whether disk caused each red, and adds a free-disk step to the Test [default features] job where it did.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest — the coupled dependency pins (anyrender, anyrender_serialize, anyrender_skia, the git Taffy line) the take carries as one set
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock file — the lock packages of the taken dependency set, AnyRender from the registry
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the CI workflow — the CI jobs, the Stand checks repeated job's free-disk step, read-only permissions, SHA-pinned actions — edits: the Test [default features] job (a free-disk step, where the logs show disk)
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=conflict
what: the engine's pointer event handling — handle_click and its private click_* arm functions, where an upstream edit to an arm is re-applied by hand
@task_file n=5 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow invariants check, WorkflowPinTest's SHA pin among them
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests one-target-per-file check
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing sink capture checks, held at 0 needles
@task_file n=8 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom Request::get ban outside the stamping constructor, read by the clippy leg
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=re-reads
what: the Taffy merge surface's place_floated_box line, re-read against upstream's Taffy pin
@task_file n=10 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script, its fast and doc legs re-run after the take
@task_item n=1 part=needs
The take's coupled dependency pins: AnyRender 0.15 with its serialize and skia crates taken from the registry as one set or none, the workspace taking no git dependency save Taffy's pinned rev, MSRV 1.91 held by build, and new or removed dependencies passing the dependency audit's advisory and licence check.
@task_item n=2 part=needs
A paint change the take brings (AnyRender 0.15, canvas-only propagated backgrounds, root SVG backgrounds, oblique shear direction) may move pixel and screenshot readings: a moved reading is re-measured and recorded, never loosened; pixel tests stay on the CPU buffer.
@task_item n=3 part=needs
Scroll offsets clamped after layout at layout rounding reach the scrolling surface escher reads: scroll_into_view, the visible region, scrolled-box bounds read where the box stands, reversed scroll origins and the into-view readings keep escher's behaviour, each resolution recorded.
@task_item n=4 part=needs
External image loads in inline and top-level SVG are a new sub-resource fetch on every document: in blitz-dom any such request goes through the one stamping constructor with its document's URL, file: URLs stay loadable only from a file: document, and the widening is listed for the operator.
@task_item n=5 part=needs
Every conflict resolution keeps both sides' intent: the changed set's contract, the one virtual clock, the select model and its painted list, hit stops, stable ids and the snapshot, range and keyboard models, inert, and the accessibility state reads; edits to upstream-owned code limited to what a conflict, gate failure or ruled fork requires, each named.
@task_item n=6 part=needs
An upstream commit that edits a handle_click arm is re-applied by hand to its private click_* arm function, the price the host-only cognitive-complexity ceiling's exception names.
@task_item n=7 part=needs
Upstream's new and edited tests run under the take under the blitz-tests rule: one file and one target per behaviour, the blitz-tests manifest's difference from upstream kept.
@task_item n=8 part=needs
Upstream tests left out keep their measured reason: the mounted-flush test stays out until the Dioxus 0.8 move; the buffered-render test on the Dioxus 0.7 bridge kept as written; a commit needing Dioxus 0.8 or parley 0.13 stays out.
@task_item n=9 part=needs
Every escher proof passes after the take: the fast and doc CI legs, stand checks in both layout modes on the bundled font, the incremental/non-incremental differential oracle, the a11y leg, the dependency audit, the local embolden witness, clippy over the workspace and upstream blitz-dom, and fork CI green with every job settled.
@task_item n=10 part=needs
Per-OS tallies re-read from the test-bearing job logs of the take's own green CI run by the plan's method: ANSI stripped, each moved count attributed by test name and explained.
@task_item n=11 part=needs
Sink allowlist re-checked over the taken delta: every new tracing or log call site, target, print and env read; the sink drops third-party targets and the standing capture checks stay at 0 needles.
@task_item n=12 part=needs
The CI disk reds given a line: the Test [default features] job gains a free-disk step only where the two runs' logs show disk; otherwise the measured other cause is stated. This changes which jobs free disk, today only the stand-repeat job.
@task_item n=13 part=needs
A CI workflow edit keeps every action pinned to a 40-hex commit SHA, the workflow's read-only contents permission, no secrets, and the workflow invariants check green.
@task_item n=14 part=needs
The CI verdict of the route-only commit c74c7f3e6 is unread and never folded as green; it is re-read before this chunk's push.
@task_standing n=1
convention: every test file joining the workspace is one integration test target of its own, pinned by the blitz-tests targets check
home: test-strategy · 1-test-scope-summary
@task_standing n=2
convention: every CI step added uses actions pinned to a commit SHA, under read-only permissions and no secrets
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: every CI job or step added keeps the leg script, cache, locked cargo and failure-log upload conventions of the existing jobs
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
The cold-agent pipe and agent-run contracts (events, exit codes, verbs, selections, verdict): the take changes no harness tooling.
@task_item n=2 part=not_needed
The session wire, session host, session errors and session recording contracts of the session lifecycle: no protocol, socket or recording change.
@task_item n=3 part=not_needed
The escher-session command line, CLI exit codes, help and MCP role contracts: no new verb, port, env var or listener of ours.
@task_item n=4 part=not_needed
Driver refusal causes, driver JSON answers and the driver verb table: the answer writer split is the next entry, out of scope.
@task_item n=5 part=not_needed
Lint defects and lint findings order, and handler coverage limits: no lint or coverage reader changes.
@task_item n=6 part=not_needed
Harness construction for a component alone and its stand boot: built by the previous chunk, untouched here.
@task_item n=7 part=not_needed
Mutation testing on the dev host and its baseline: the chunk runs no mutation pass.
@task_item n=8 part=not_needed
Upstream's WPT conformance numbers and the WPT runner CLI and outputs: out of scope; WPT stays report-only and upstream-only.
