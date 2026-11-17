@task_prose part=chunk
The twelfth upstream take: the 10 commits from the last take's read to upstream/main at e5305a4c1 are cherry-picked onto this fork's pins on a fresh trial worktree, each commit left out keeping its measured reason, and each left-out reason re-read against the new pin.
The take brings external images in inline and top-level SVG, AnyRender 0.15 back from crates.io (with its serialize and skia companions), root SVG background painting, root and body backgrounds painted only on the canvas, the synthetic italic/oblique shear direction fix, scroll offsets clamped after layout during layout rounding, and WPT-only changes; the AnyRender registry move may release the git-pinned delta every earlier take left out.
Taffy stays on its pinned git rev and its merge surface is re-read against upstream's pin; Dioxus 0.7.3 and parley 0.12 stay held; the earlier takes' merge surfaces are re-read row by row.
Every escher proof is re-run after the take (fast and doc legs, stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg, the dependency audit, the embolden witness, screenshot readings re-measured where paint moves pixels), the per-OS test tallies of the take's green CI run re-read, and the telemetry sink allowlist re-checked against the taken delta.
The CI disk reds get a line: both failing runs' logs are read for disk as the cause, and either the default-features test job gains a pinned free-disk step or the measured other cause is stated.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest, the shared workspace dependency pins (anyrender, anyrender_serialize, anyrender_skia, the git Taffy line) the take carries as one set
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile, its anyrender and Taffy lock packages moved with the manifest's pin set
@task_file n=3 path=.github/workflows/ci.yml mark=edits
what: the fork CI workflow, its jobs, SHA-pinned actions and read-only contents permission — edits: the `Test [default features]` job, which may gain a `Free Disk Space (Ubuntu)` step where the logs show disk
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=conflict
what: blitz-dom pointer event handling, `handle_click`'s arms as private `click_*` arm functions, where an upstream edit to a `handle_click` arm is re-applied by hand to its arm function
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=re-reads
what: the Taffy merge surface's `place_floated_box` line as upstream wrote it, re-read against upstream's Taffy pin
@task_file n=6 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom `Request::get` ban outside the stamping constructor, read against the SVG external-image load through the clippy leg
@task_file n=7 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing telemetry capture checks, kept at 0 needles after the take
@task_file n=8 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check, one file and one test target per behaviour, run over upstream's new and edited test files
@task_file n=9 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner, its fast, doc, a11y, audit and embolden legs re-run after the take
@task_file n=10 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow contract tests (WorkflowPinTest's 40-hex SHA pin and the workflow invariants) a free-disk step must pass
@task_item n=1 part=needs
The take itself: upstream changes taken by cherry-pick onto this fork's pins, measured on a trial worktree with conflicts read by merge-tree first, each commit left out keeping its measured reason, and the merge base and left-out table recorded as the earlier takes did.
@task_item n=2 part=needs
AnyRender moves from 0.14 to the registry's 0.15 (with anyrender_serialize 0.9 and anyrender_skia 0.13): the rendering abstraction over Vello and Skia, the window renderer chosen at compile time, the headless screenshots on the Vello CPU renderer, all on a version the workspace has not run; the hybrid backend's rename to gpu reaches any feature or renderer name escher selects.
@task_item n=3 part=needs
The no-git-dependency rule: the workspace takes no git dependency save Taffy's pinned rev, so a taken AnyRender comes from the registry, and the earlier git-pinned AnyRender chain may now be taken only as registry versions.
@task_item n=4 part=needs
Coupled dependency pins: upstream's pin set is carried as one set or none; companion crates whose versions follow the renderers (skrifa with parley and vello, svgtypes with usvg, font deps matching the renderers) stay matched; Dioxus 0.7.3 and parley 0.12 stay held and a commit needing either stays out.
@task_item n=5 part=needs
The Taffy pin: the root manifest's git Taffy rev and lock package, ratified as the one allowed git dependency, are re-read against upstream's pin at e5305a4c1 and expected unmoved.
@task_item n=6 part=needs
External images in inline and top-level SVG: a new sub-resource load on every document, remote pages included; it must go through the stamping request constructor carrying its document's URL (the engine's sub-resource requests carry their initiator), be read against blitz-net's file: refusal and the file-URL load rule, and go through the image cache by URL; the widening is listed for the operator.
@task_item n=7 part=needs
Paint changes the take carries for every document: root SVG background painting, propagated root and body backgrounds painted only on the canvas, and the synthetic italic/oblique shear direction; paint emits into the paint scene from resolved style, and a moved screenshot or embolden reading is re-measured and recorded, never loosened.
@task_item n=8 part=needs
Scroll offsets clamped after layout during layout rounding: the one scrolling primitive, programmatic scrolls clamped to one box, reversed boxes scrolling to negative offsets, scroll_into_view's whole-chain and nested-instant writes, visible_region and the scrolled-box bounds all keep escher's readings through the resolution.
@task_item n=9 part=needs
Resolutions keep both sides' intent over the engine surfaces escher changed: the changed set's contract and write sites, the document clock invariant, the select model and its open list, hit stops at clipping and paint clips, stable element ids and the snapshot, the range and keyboard models, inert, and the accessibility state reads; each resolution recorded.
@task_item n=10 part=needs
Upstream behaviour changes ratified per take: each engine behaviour change the take carries (SVG images, canvas backgrounds, shear, scroll clamping) is listed for the operator at P4 as a boundary widening, never settled silently.
@task_item n=11 part=needs
Additivity: edits of ours to upstream-owned code only where a conflict, a gate failure or a ruled fork requires, each named and recorded and flagged upstreamable; a hand re-application into a click arm function is such an edit.
@task_item n=12 part=needs
The CDP server's served listener stays out: its commit is not taken unless the take cannot stand without it, and any take of it is a security-plan amendment first; no served network port, no new listener, port, env var or crate of ours.
@task_item n=13 part=needs
The sink allowlist: every new tracing or log call site, target, print and env read in the taken delta is read against the telemetry scrub, target filter and engine targets, with engine tracing gated per call site, and the standing capture checks stay at 0 needles.
@task_item n=14 part=needs
The CI disk reds: disk is freed today only in the stand-repeat job; where the failing runs' logs show disk, the default-features test job gains a free-disk step on a SHA-pinned action under the workflow contract tests and the read-only permission, widening that rule by ratification; where they do not, the measured other cause is stated.
@task_item n=15 part=needs
The proofs re-run on the synced tree: the fast and doc legs, the stand checks in both layout modes with both layout modes identical, the incremental/non-incremental oracle, the a11y leg, the dependency and licence audit over new and removed dependencies, the embolden witness, and fork CI green with every job settled; the stand-repeat zero-failure budget holds.
@task_item n=16 part=needs
Per-OS tallies: the test-bearing job logs of the take's green CI run are re-read, every moved count explained, and stand readings stay identical on every CI OS.
@task_item n=17 part=needs
The MSRV stays 1.91: the taken crates must build on the advertised MSRV in the build-only MSRV job, and the flake's Rust pin stays matched to rust-version.
@task_item n=18 part=needs
The left-out delta's carried reasons: the mounted-event test stays out until the Dioxus 0.8 move, the buffered render's bridge stays as ported on 0.7, and the anonymous-block deallocation fix and earlier resolutions are re-read row by row as merge surfaces.
@task_standing n=1
convention: every taken upstream integration test stands in blitz-tests as one behaviour per file and one test target per file
home: conventions
@task_standing n=2
convention: every new or taken regression test names its behaviour descriptively and cites the upstream change it guards
home: conventions
@task_standing n=3
convention: every action a CI job step names is pinned to a 40-hex commit SHA and the workflow keeps read-only permissions
home: stack-and-technologies · infrastructure-patterns
@task_standing n=4
convention: every CI workflow change is held by the CI workflow contract tests, and cargo legs run locked
home: standard-contracts
@task_standing n=5
convention: every dependency line is declared once in the workspace and inherited by member crates
home: conventions
@task_item n=1 part=not_needed
The browser application's history store, about pages, bookmarks, toolbar and persistence: no taken commit and no proof of this chunk reaches the reference browser's state.
@task_item n=2 part=not_needed
The cold-agent run, its events, exit codes, verdict and wrong-call bar: the chunk runs no cold-agent pass and changes nothing it reads.
@task_item n=3 part=not_needed
The driver's verbs, answer JSON, refusal causes and the MCP server's protocol: the take adds no verb, carrier or answer word, and the answer writer's split belongs to the next entry.
@task_item n=4 part=not_needed
The session host, session socket wire, recording directory and kept check file: no part of the take touches the session process model or its on-disk forms.
@task_item n=5 part=not_needed
The JavaScript runtime, Boa, script globals and script fetching: none of the 10 commits touch script execution.
@task_item n=6 part=not_needed
Platform entry points and shells for Android, iOS, wasm and Windows subsystems: the take changes no platform entry or shell quirk.
@task_item n=7 part=not_needed
The capability statement's sections and the root README's tool count: no driver command or capability is added or removed.
