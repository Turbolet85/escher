@task_prose part=chunk
The twelfth upstream take: the 10 commits from ea07ee716 to upstream/main at e5305a4c1 cherry-picked onto this fork's pins on a fresh trial worktree, each commit left out keeping its measured reason, and the left-out table's reasons re-read now that AnyRender is back on the registry at 0.15.
Every conflict resolution keeps both sides' intent, escher's engine and dioxus-native-dom behaviour included; the carried merge surfaces (buffered render, click arm functions, Taffy line, earlier takes' resolutions) are re-read row by row.
It re-runs every escher proof after the take: the fast and doc legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, the embolden witness and the screenshot readings, with fork CI green on the pushed commit.
It re-reads the per-OS test tallies from the take's own green run and re-checks the sink allowlist against the taken delta.
It gives the CI disk reds a line: the two red runs' logs are read, and either a pinned free-disk step is added to the Test [default features] job or the measured other cause is stated.

@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the coupled dependency pins (AnyRender, Taffy's git rev, Dioxus, parley) a take carries as one set or none

@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock file — the locked packages of the coupled dependency pins the take carries

@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow — the Stand checks repeated job with its pinned free-disk step, and the Test [default features] job, under read-only contents permission — edits: the Test [default features] job (a free-disk step, where the logs show disk)

@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=conflict
what: the engine's pointer event handling — handle_click and its private click arm functions, where an upstream edit to a click arm is re-applied by hand

@task_file n=5 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing log capture checks that stay at 0 needles over the sink's target allowlist

@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one file and one target per behaviour for upstream's new and edited test files

@task_file n=7 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs re-run after the take

@task_file n=8 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: the buffered render's merge surface — the two render sites and the buffered mutations field, re-read at the take

@task_file n=9 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom request ban — a sub-resource Request::get only through stamped_request, read against the take's new SVG external image loads

@task_item n=1 part=needs
The a11y CI leg runs green after the take, as one of the escher proofs every take must pass; a red there is fixed, never skipped.

@task_item n=2 part=needs
The accessibility state reads (checked, pressed, selected, invalid, expanded, disabled) and the accessibility tree built through the document's own method keep escher's behaviour through every conflict resolution in the engine.

@task_item n=3 part=needs
The single-choice select model and its engine overlay list, the multiple select, and the select list's keyboard model keep their behaviour through the take's resolutions.

@task_item n=4 part=needs
The range input model and the keyboard activation and Tab order models keep their behaviour through the take; an upstream edit to a click arm is re-applied by hand to its click function so the click, focus and disabled-walk rules hold.

@task_item n=5 part=needs
Inert subtrees stay excluded from focus, hit-testing and the accessibility tree, and hidden content stays out of the tree, through the take.

@task_item n=6 part=needs
The stable node ids and the snapshot read from the accessibility tree keep their behaviour; the dioxus-native-dom document (ids, snapshot, UI graph, reached-handler record, buffered render) keeps escher's behaviour, each resolution recorded.

@task_item n=7 part=needs
Upstream's scroll-offset clamping at layout rounding reaches the scrolling surface escher's scroll into view, visible region and scrolled-box bounds read; the snapshot's bounds and the reversed scroll origins keep their readings or a moved reading is re-measured and recorded.

@task_item n=8 part=needs
The take's paint changes (AnyRender 0.15, root and body backgrounds painted on the canvas, root SVG background, synthetic italic shear direction) may move pixels and computed background readings; a moved screenshot or stand reading is re-measured and recorded, never loosened to pass.

@task_item n=9 part=needs
The stand checks in both layout modes and the Stand checks repeated three-round job stay green after the take.

@task_item n=10 part=needs
The engine's changed set and the clock invariant keep their contract through the resolutions, so the accessibility tree refresh on change and the double-click reading on the document's clock hold.

@task_item n=11 part=needs
The Dioxus pin stays at 0.7.3 and parley at 0.12; a commit needing either stays out with its measured reason, so the Dioxus accessibility feature stays taken only by name and the buffered render stays on the 0.7 bridge.

@task_item n=12 part=needs
Coupled dependency pins move as one set or none, from the registry only (no git dependency save Taffy's pinned rev), MSRV staying 1.91, each new or removed dependency reviewed by hand and through the audit's licence check; the accessibility feature's default stays as it is.

@task_item n=13 part=needs
The CI workflow edit for the disk reds keeps every action pinned to a 40-hex SHA and the workflow's read-only contents permission, and leaves the a11y leg and the Stand checks repeated job as they are.

@task_item n=14 part=needs
The sink allowlist is re-checked over the taken delta for every new tracing or log call site, target, print and env read; the capture checks stay at 0 needles, so blitz and accesskit_xplat targets print only their allowed fields.

@task_item n=15 part=needs
Boundary widenings the take carries (external image loads in SVG as a new fetch on every document, the background and shear engine changes) are listed for the operator at P4, never settled silently; no new crate, port, env var or listener of ours.

@task_standing n=1
convention: any accessibility assertion the take brings or re-homes is a Rust test over the AccessKit tree, one file and one target per behaviour
home: 3-a11y-assertion-harness-contract

@task_standing n=2
convention: any CI step added keeps the accessibility checks gating CI and the a11y leg unchanged
home: 9-ci-integration · ci-integration

@task_item n=1 part=not_needed
Cognitive accessibility rows (the invalid-state cues beyond colour or class, the shared stand Tab order table) are not reached: the take changes no stand page or form state presentation.

@task_item n=2 part=not_needed
The actions answer, the MCP tool results and the driver's answer fields are not reached: the answer writer split is the next entry, and this chunk changes no driver answer.

@task_item n=3 part=not_needed
The platform adapter and assistive technology reach rows (no adapter on the stand binaries, accessibility reach tier 0, the capability statement edited by hand) are not reached: the take adds no adapter, binary or capability claim.

