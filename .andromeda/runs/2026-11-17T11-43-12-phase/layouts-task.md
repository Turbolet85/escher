@task_prose part=chunk
Takes this fork's twelfth upstream sync: the 10 upstream commits ea07ee716..e5305a4c1, cherry-picked onto this fork's pins on a fresh trial worktree, each commit left out keeping its measured reason.
The take brings external images in SVG, AnyRender 0.15 from the registry, root SVG and canvas-only root/body backgrounds, the synthetic italic shear direction, and scroll offsets clamped after layout during layout rounding.
Every conflict resolution keeps both sides' intent: the stable ids and snapshot, the select model, the hit stops and scrolled-box bounds, the reversed scroll origins and into-view readings, the range and keyboard models, inert, and the accessibility state reads.
Re-runs every escher proof after the take: the fast and doc legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, the screenshot readings, and fork CI green; re-reads the per-OS tallies, the sink allowlist, the Taffy and earlier merge surfaces, and the left-out delta's reasons.
Gives the CI disk reds a line: measures from the two failing runs' logs whether disk caused each red, and adds a free-disk step to the default-features test job where the logs show disk, or states the measured other cause.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest and its coupled dependency pins, the AnyRender set and the git Taffy rev
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock, its AnyRender and Taffy lock packages
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow, its jobs and read-only contents permission — edits: the `Test [default features]` job, a free-disk step where the logs show disk
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's click handling, handle_click's arms as private click functions — edits: the click arm functions a taken commit's handle_click edit is re-applied to by hand
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check, one test file and one target per behaviour
@task_file n=6 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner, the fast and doc legs re-run after the take
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing sink capture checks, held at 0 needles
@task_file n=8 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom clippy ban on a bare sub-resource request outside the stamped request
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=re-reads
what: the Taffy merge surface's floated-box placement line, re-read against upstream's Taffy pin
@task_file n=10 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow invariants check a workflow edit must hold
@task_item n=1 part=needs
Scroll offsets clamped after layout during layout rounding change the scrolling surface the into-view scroll, the visible region and the scrolled-box bounds read; each resolution keeps the into-view scroll moving every holding scroller and the visible region read through clipping boxes.
@task_item n=2 part=needs
Open list rows keep reading their own bounds and the hit stops and scrolled-box bounds hold over the clamped scroll offsets.
@task_item n=3 part=needs
Reversed scroll origins and their into-view readings stay as escher had them after the take.
@task_item n=4 part=needs
The stand's stable element ids and the snapshot stay as they are; the ids remain attributes alone through every resolution.
@task_item n=5 part=needs
The select model and its list hold, the flight type select still reading its two options at boot.
@task_item n=6 part=needs
The range and keyboard models, inert, and the accessibility state reads are kept; the stand's names, roles and states stay read by attributes alone and the a11y leg stays green.
@task_item n=7 part=needs
The stand checks pass in both layout modes on the headless stand's pinned viewport, and the incremental/non-incremental oracle agrees.
@task_item n=8 part=needs
The paint changes taken (AnyRender 0.15, root and body backgrounds painted only on the canvas, root SVG background, synthetic italic shear direction) may move pixels: each moved screenshot reading is re-measured and recorded, never loosened; the embolden witness still passes.
@task_item n=9 part=needs
A moved layout, text or paint number from the take is re-measured and recorded, never loosened to pass, including what the layout verb and the screenshot answer report.
@task_item n=10 part=needs
External images in inline and top-level SVG are a new sub-resource fetch on every document, remote pages included: routed through the stamped request with its document's URL, read against the file refusal and against the load page alone having a network provider, and listed for the operator at P4 as a boundary widening.
@task_item n=11 part=needs
The background and shear changes are engine changes for every document; each one taken is listed for the operator at P4, never settled silently.
@task_item n=12 part=needs
AnyRender comes from the registry, its pins carried as upstream's one set or none; the workspace takes no git dependency save Taffy's pinned rev, which stays unmoved.
@task_item n=13 part=needs
Dioxus 0.7.3 and parley 0.12 stay held; a commit that needs either stays out with its measured reason, and the mounted-event test stays out.
@task_item n=14 part=needs
The left-out delta's reasons are re-read against the new pin; a reason that no longer holds, the AnyRender git pin first, puts that commit back in the take.
@task_item n=15 part=needs
Upstream's new and edited test files run under the take, one file and one target per behaviour.
@task_item n=16 part=needs
The taken delta is read for every new tracing or log call site, target, print and env read; the sink allowlist holds and caller input is never written back.
@task_item n=17 part=needs
No new crate, port, env var or listener of ours; the CDP served listener stays out unless the take cannot stand without it, and any take of it is a security-plan amendment first.
@task_item n=18 part=needs
A free-disk step added to the default-features test job is pinned to a 40-hex SHA and keeps the workflow's read-only contents permission.
@task_item n=19 part=needs
Per-OS test tallies are re-read from the take's own green CI run logs, each moved count explained.
@task_item n=20 part=needs
Edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded; MSRV stays 1.91 and new or removed dependencies pass the audit's licence check.
@task_standing n=1
convention: each moved layout, text, paint or screenshot reading is re-measured and recorded, never loosened to pass
home: surface-desktop-native · surface-cli
@task_standing n=2
convention: each new log line or call site the take brings stays inside the sink allowlist and writes no caller input back
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls: the take moves no window drag, close, maximize, minimize or decorations control.
@task_item n=2 part=not_needed
The agent-run and cold-agent run pipe contracts and the cold-agent prompt and pipe decisions: the sync changes no agent-facing script.
@task_item n=3 part=not_needed
The multi_doc command line: its arguments, env knobs and stdout are untouched by the take.
@task_item n=4 part=not_needed
The escher-session MCP server's protocol revisions, stdio, tools list, result bound and end, and the session exit codes and refusals: the take adds no verb, tool or refusal.
@task_item n=5 part=not_needed
The escher-session coverage role and recording contracts: no recording or coverage file format changes.
@task_item n=6 part=not_needed
The CRUD list keyboard region and the stand accent colour decisions: no stand component is restyled or re-keyed by the take.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces and the decisions log: they hold no rows the take reaches.
@task_item n=8 part=not_needed
Arbitrary index.html templates for a document: the take adds no template path.
