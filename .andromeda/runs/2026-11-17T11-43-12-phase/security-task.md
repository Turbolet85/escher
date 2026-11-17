@task_prose part=chunk
Takes upstream/main at e5305a4c1 into this fork as its twelfth upstream sync: the ten commits since ea07ee716, cherry-picked onto this fork's pins on a fresh trial worktree, each commit left out keeping its measured reason.
The ten bring external images in inline and top-level SVG, AnyRender 0.15 back on the crates.io registry, root SVG and canvas-only propagated backgrounds, the synthetic italic shear direction, and scroll offsets clamped at layout rounding; the left-out table's reasons are re-read, the AnyRender git pin first.
Re-reads the Taffy pin against upstream's, the folded merge surfaces row by row, the Request::get ban against the SVG image loads, and the sink allowlist over every new log call site, env read and print in the taken delta.
Re-runs every escher proof (fast and doc legs, stand checks in both layout modes, the oracle, the a11y leg, the dependency audit, the embolden witness, screenshot readings) and re-reads the per-OS tallies of the take's own green CI run.
Gives the CI disk reds a line: reads both red runs' logs and, where disk caused the red, adds a pinned free-disk step to the Test [default features] job; otherwise states the measured cause.
@task_file n=1 path=Cargo.toml mark=merge
what: the root manifest's engine dependency pins — the coupled AnyRender set, the git Taffy line on its pinned rev, the held Dioxus and parley pins
@task_file n=2 path=Cargo.lock mark=merge
what: the lockfile the locked builds and the dependency audit read — its AnyRender and Taffy packages
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow — its jobs, its pinned actions and its read-only contents permission — edits: the Test [default features] job's free-disk step, the Stand checks repeated job's free-disk step read as the model
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=conflict
what: the engine's pointer event handling — handle_click and its private click_* arm functions, where an upstream edit to a handle_click arm is re-applied by hand
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=re-reads
what: the Taffy merge surface's place_floated_box line, kept as upstream wrote it, re-read against upstream's Taffy pin
@task_file n=6 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the clippy ban on Request::get in blitz-dom outside the one stamping sub-resource request constructor
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the telemetry sink's standing capture checks, held at 0 needles
@task_file n=8 path=.github/scripts/ci-leg.sh mark=re-reads
what: the fast and doc CI legs re-run after the take
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule — one test file and one target per behaviour — over upstream's new and edited test files
@task_file n=10 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow invariants and WorkflowPinTest's 40-hex SHA pin check over every action
@task_item n=1 part=needs
The upstream take itself: ten upstream commits cherry-picked onto the fork's pins, each left-out commit keeping its measured reason; what each taken commit brings is a change of this chunk.
@task_item n=2 part=needs
External images in inline and top-level SVG: a new sub-resource fetch on every document, remote pages included — a boundary widening listed for the operator at P4 and checked against the escalate patterns, never settled silently.
@task_item n=3 part=needs
The SVG image load against the sub-resource request rules: any new Request::get in blitz-dom goes through the one stamping constructor with its document's URL, sub-resource requests stay GETs, and file URLs load only for file documents (blitz-net's FileForbidden refusal).
@task_item n=4 part=needs
AnyRender 0.15 from the crates.io registry with anyrender_serialize 0.9 and anyrender_skia 0.13: the coupled set taken whole or not at all, no git dependency added, the left-out git-pin chain re-read for what it releases.
@task_item n=5 part=needs
The no-git-dependency rule and its one carve-out: the workspace takes no git dependency save Taffy's pinned rev, re-read against upstream's Taffy pin a3997d55 with no Taffy move expected.
@task_item n=6 part=needs
Still-held engine pins: Dioxus 0.7.3 and parley 0.12 stay; a commit needing Dioxus 0.8 or parley 0.13 stays out with its measured reason, the mounted-event test and the buffered render's merge surface riding on untouched.
@task_item n=7 part=needs
New and removed dependencies reviewed by hand and through the dependency audit's licence gate and advisory read on the locked graph; MSRV stays 1.91; no dependency bumped beyond upstream's own registry set.
@task_item n=8 part=needs
Paint changes for every document — root SVG background, propagated root and body backgrounds painted only on the canvas, synthetic italic and oblique shear direction — engine changes listed for the operator at P4; a moved screenshot reading re-measured and recorded, never loosened.
@task_item n=9 part=needs
Scroll offsets clamped after layout at layout rounding: resolutions keep escher's programmatic scroll clamping, the scrolled-box bounds, the reversed scroll origins, the into-view readings and the visible region reads intact.
@task_item n=10 part=needs
Conflict resolutions keep both sides' intent over the engine surfaces escher changed: the changed set's contract, the virtual clock invariant, the select model and its open list, hit stops, stable ids and the snapshot, the range and keyboard models, inert, and the accessibility state reads; each resolution recorded.
@task_item n=11 part=needs
The folded merge surfaces re-read row by row: the dropped anonymous block's parent-clearing fix and the earlier takes' resolutions, each still standing after the take.
@task_item n=12 part=needs
Sink allowlist over the taken delta: every new tracing or log call site, log target, print and env read is read; outside targets stay dropped, the standing capture checks stay at 0 needles, and nothing new reads an env var beyond what the rules allow.
@task_item n=13 part=needs
The CDP server's served listener stays out: its commit is not taken unless the take cannot stand without it, and any take of it is a security-plan amendment first; the chunk adds no crate, port, env var or listener of its own.
@task_item n=14 part=needs
Additivity: edits of the fork's to upstream-owned code limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded; a handle_click arm edit re-applied by hand to its click_* function.
@task_item n=15 part=needs
The CI disk reds: both red runs' logs read for the failing step, its error and the runner image; a free-disk step added to Test [default features] only where the logs show disk, otherwise the measured other cause stated — the hypothesis kept as the operator gave it.
@task_item n=16 part=needs
The runner disk-freeing decision that today frees disk only for the repeated stand job: a free-disk step on Test [default features] changes it, and the new step stays pinned to a 40-hex SHA under the workflow invariants.
@task_item n=17 part=needs
CI workflow edits keep the workflow's read-only contents token permission and give no job a new secret.
@task_item n=18 part=needs
Every escher proof passes after the take and fork CI is green on the pushed commit with every job settled; the route-only commit's unread CI verdict is re-read, never folded as green.
@task_standing n=1
convention: every action a CI step uses is pinned to a 40-hex commit SHA, checked by the workflow pin test
home: dependency-security
@task_standing n=2
convention: every CI job runs with a read-only contents token and reaches no secret it does not need
home: authentication-authorization
@task_standing n=3
convention: every log line or log target a taken change adds passes escher's allowlist telemetry sink, outside targets dropped whole and content-named fields withheld
home: logging-monitoring
@task_standing n=4
convention: every dependency a take adds or moves is a registry dependency under locked builds, taken as upstream's whole set and passing the audit's licence and advisory gate
home: dependency-security
@task_item n=1 part=not_needed
MCP server rows (stdio transport, request and result bounds, protocol error codes, tool listing): the take brings no change to escher's MCP role.
@task_item n=2 part=not_needed
Driver session socket rows (closed request grammar, reply forms, connection bounds, owner-only socket, idle expiry): the take adds no verb and touches no session wire.
@task_item n=3 part=not_needed
Recorded and kept check file rows (their fields, owner-only directories, lifecycle, strict reader): no recording format moves.
@task_item n=4 part=not_needed
Cold-agent pipe and agent-run harness rows (fixed prompt, call allowlist, replay isolation, metadata-only events): no harness script changes.
@task_item n=5 part=not_needed
escher-session CLI rows (flags, roles, exit codes, stdout and stderr lines, session error table): the command line is untouched.
@task_item n=6 part=not_needed
Browser app storage rows (history in sqlite, cookie store, on-disk HTTP cache, clear history): the take brings no storage change.
@task_item n=7 part=not_needed
Release publishing and signing rows (publish job repository guard, upstream-only secrets, signing material removal): the chunk publishes nothing.
@task_item n=8 part=not_needed
WPT post-results, PR lookup and quarantine rows: the WPT-only upstream commits carry no fork workflow change and upstream's WPT numbers are out of scope.
@task_item n=9 part=not_needed
Refusal cause and refusal JSON contract rows of the driver: no driver refusal is added or reworded.
