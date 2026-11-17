@task_prose part=chunk
Takes upstream/main at e5305a4c1 (10 commits past the last take ea07ee716) onto this fork's pins by cherry-pick on a fresh trial worktree: external images in SVG, AnyRender 0.15 from the registry, root SVG and canvas background painting, synthetic italic shear direction, scroll offsets clamped at layout rounding.
Re-reads the left-out delta's reasons against e5305a4c1, the AnyRender git pin first, and puts back any commit whose reason no longer holds; Dioxus 0.7.3 and parley 0.12 stay held.
Resolves conflicts keeping both sides' intent and re-reads the carried merge surfaces row by row: the buffered render, the click arm functions, the Request::get ban, the Taffy pin, the anonymous-block fix.
Re-runs every escher proof after the take (fast and doc legs, stand checks in both layout modes, the oracle, the a11y leg, the dependency audit, the screenshot readings), re-reads the per-OS tallies of the take's green CI run, and re-checks the sink allowlist over the taken delta.
Gives the CI disk reds a line: reads both red runs' logs and either adds a pinned free-disk step to the Test [default features] job or states the measured other cause.
@task_file n=1 path=Cargo.toml mark=merge
what: the root manifest — the workspace dependency pins taken from upstream as one set: the AnyRender registry lines and the git Taffy rev
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile — the lock packages of the taken dependency set
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow — its jobs, their steps and the read-only contents permission — edits: the Test [default features] job (a free-disk step, where the logs show disk)
@task_file n=4 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow invariants check, with WorkflowPinTest's 40-hex SHA pin on every action
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one test file and one target per behaviour
@task_file n=6 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs runner — the fast and doc legs re-run after the take
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: escher's telemetry sink line format and its standing capture checks, held at 0 needles
@task_file n=8 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the Request::get ban in blitz-dom outside stamped_request, read against the taken SVG image loads
@task_file n=9 path=packages/blitz-dom/src/events/pointer.rs
what: handle_click's private click_* arm functions — a taken commit that edits a click arm is re-applied by hand to its arm function
@task_file n=10 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the buffered render's merge surface — the two render sites and the buffered_mutations field, re-read at the take
@task_file n=11 path=packages/blitz-dom/src/layout/inline/layout.rs
what: the Taffy merge surface's place_floated_box line, re-read against upstream's Taffy pin
@task_item n=1 part=needs
The sink allowlist over the taken delta: every new tracing or log call site, its target, every print and every env read the take brings is read against the sink target allowlist and the engine-target safe-field rule.
@task_item n=2 part=needs
Engine tracing stays behind the tracing cargo feature in upstream crates: a taken engine log site keeps that gate and writes a fixed message, with no page content (no image URL, no SVG text) as a field.
@task_item n=3 part=needs
The standing capture checks stay at 0 needles after the take: content-named fields scrubbed by field name in the sink and the capture, framework targets counted at warn and above.
@task_item n=4 part=needs
External image loads in inline and top-level SVG are a new sub-resource fetch on every document, remote pages included: routed through stamped_request with the document's URL, and refused for file: loads from non-file: documents.
@task_item n=5 part=needs
The take's boundary widenings (SVG external image fetches, root and canvas background painting, shear direction) are listed for the operator at P4, never settled silently.
@task_item n=6 part=needs
No new crate, port, env var or listener of ours; no telemetry backend, exporter or metric emission arrives with the take; the CDP served listener stays out with a1670ef1b unless a security-plan amendment comes first.
@task_item n=7 part=needs
The workspace takes no git dependency save Taffy's pinned rev: a taken AnyRender comes from the registry, upstream's pins carried as one set or none, MSRV staying 1.91, new or removed dependencies reviewed by hand and through the audit's licence check.
@task_item n=8 part=needs
Scroll offsets clamped after layout at layout rounding change the scrolling surface the driver's into-view and visible-region reads and the scrolled-box bounds stand on; the snapshot, diff and layout answers keep escher's readings, each resolution recorded.
@task_item n=9 part=needs
Paint changes (AnyRender 0.15, canvas backgrounds, shear) may move the shot's pixels: a moved screenshot reading is re-measured and recorded, never loosened.
@task_item n=10 part=needs
dioxus-native-dom keeps escher's behaviour through the take: stable ids, the snapshot, the graph, the reached record and the buffered render whose mutation buffer stays in memory.
@task_item n=11 part=needs
The CI disk reds get a line: both red runs' logs (failing step, error text, runner image) are read, and a free-disk step joins the Test [default features] job where they show disk, or the measured other cause is stated.
@task_item n=12 part=needs
A CI workflow edit keeps every action pinned to a 40-hex SHA and the workflow's read-only contents permission, and keeps the per-leg log artifact on failure and the coverage artifact on success.
@task_item n=13 part=needs
Fork CI green on the pushed commit with every job settled; the route-only commit's CI verdict, still in progress, is re-read before the push and never folded as green.
@task_item n=14 part=needs
Per-OS tallies re-read from the test-bearing job logs of the take's own green CI run, each moved count explained.
@task_item n=15 part=needs
Upstream's new and edited test files run under the blitz-tests rule, one file and one target per behaviour; upstream's mounted-event test stays out until the Dioxus 0.8 move.
@task_item n=16 part=needs
The trial runs on a fresh worktree under target/tmp; its cleanup touches only its own state, and no local state is uploaded.
@task_item n=17 part=needs
Additivity: edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded.
@task_standing n=1
convention: every new or edited engine log site writes a fixed message and prints safe fields only, under the sink target allowlist
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=2
convention: every CI job or step added keeps actions pinned by SHA, uploads only the per-leg log on failure and the coverage report on success, and never uploads local state
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The driver's command and tool_call spans, their fields, endings and refusal cause names: the take changes no driver verb and opens no span.
@task_item n=2 part=not_needed
The cold-agent and agent-run harness rows — events, verdict fields, counted verbs, kept checks, transcripts and their scrubbing: the take touches neither pipe.
@task_item n=3 part=not_needed
escher-session's roles, stdout, exit codes, host wire, error object and coverage line: no session surface changes with the take.
@task_item n=4 part=not_needed
Recording files, the recording and session state directories, and the session lifecycle: the chunk records nothing and changes no session state.
@task_item n=5 part=not_needed
The script error surface, the window error event, onerror arguments and the script diagnostics drain: no taken commit is in script error reporting.
@task_item n=6 part=not_needed
Select, optgroup, checkbox and radio rows, multiple-select form entries and the state registry: the take changes no form control model.
@task_item n=7 part=not_needed
WPT score publishing and WPT report messages: upstream's WPT-only commits and conformance numbers are out of scope.
@task_item n=8 part=not_needed
The WASM console panic hook and the escher-telemetry panic hook's previous-hook chaining: no panic path changes.
