@task_prose part=chunk
The fork's eleventh upstream take: the 16 upstream commits since the last pin, up to upstream/main at ea07ee716, cherry-picked onto the workspace's registry pins, keeping only what builds without git Taffy, git AnyRender or Dioxus 0.8, with a measured reason kept for each commit left out.
Research measures the four Taffy-main commits in full (what the merge breaks, the edits it needs, the dependency audit and MSRV legs with a git Taffy) and whether a crates.io taffy release or an upstream manifest back on one carries them; only a registry Taffy is taken, otherwise they stay out and reversing the no-git-dependency rule becomes one founder question, with no plan step depending on its answer.
The left-out delta's reasons, the earlier takes' merge surfaces and the carried escher arms are re-read against ea07ee716, and every reason that no longer holds puts its commit back in the take.
Every escher proof is re-run after the take: the fast and doc legs, the stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg, the dependency audit, the embolden witness and fork CI green with every job settled.
The per-OS tallies of the take's green CI run are re-read and each moved count explained, and the sink allowlist is re-checked against the taken delta.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the shared workspace dependencies and their coupled registry pins (Taffy, AnyRender), where upstream's git Taffy and git AnyRender bumps would land
@task_file n=2 path=.github/scripts/test_blitz_tests_targets.py
what: the check that pins the blitz-tests target list, one test target per file
@task_file n=3 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner — the fast, doc, a11y, audit, msrv and embolden legs re-run after the take
@task_file n=4 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the escher-telemetry sink's standing capture checks, held at 0 needles
@task_file n=5 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom ban on Request::get outside the stamping constructor, read against the take's font-face loading change
@task_item n=1 part=needs
The take follows the upstream sync rule: upstream changes are taken by cherry-pick onto the registry pins from the last pin's merge base, and each commit left out keeps its measured reason.
@task_item n=2 part=needs
The workspace takes no git dependency: the git Taffy and every git AnyRender bump stay out unless a registry release carries them, and reversing the rule is a founder question the plan never acts on.
@task_item n=3 part=needs
Coupled dependency pins move as one set or not at all. No dependency is bumped past upstream's own registry set. A layout, text or paint number that moves is re-measured and recorded, never loosened to pass. The deferred Stylo, Taffy and Dioxus bumps stay deferred.
@task_item n=4 part=needs
New or removed dependencies are reviewed by hand and run through the dependency and licence audit (cargo-deny). The audit leg is also measured with a git Taffy as research.
@task_item n=5 part=needs
MSRV stays 1.91, and the build-only MSRV leg is measured with a git Taffy as research.
@task_item n=6 part=needs
The engine behaviour changes the take brings land on every document, remote pages included: SVG presentation attributes and inline SVG overflow and transform mapped to CSS, the hr element's color, noshade and size, and the refactored presentational-hint helpers, all at the legacy-attribute presentational-hints level. Each is an upstream behaviour change ratified per take, listed for the operator at P4 and marked PROVISIONAL.
@task_item n=7 part=needs
Inline SVG serialization through svg_source() gains namespace declarations.
@task_item n=8 part=needs
The @font-face font-stretch descriptor is passed to fontique, so font faces are registered by more CSS descriptors. Any upstream sub-resource Request::get in blitz-dom is routed through the stamping constructor with its document's URL, and an edited StylesheetHandler construction or font-face fetch caller takes the new document_url.
@task_item n=9 part=needs
SVG image painting gains clipping and an isolated layer, so it must stay within the paint layer cap and the mask-isolation buffer rule.
@task_item n=10 part=needs
Every conflict resolution keeps both sides' intent: the changed set's contract, the clock invariant (no direct time read outside the document clock), the select model and its open list, the hit stops and the scrolled-box bounds, the stable ids and the snapshot, the range and keyboard models, inert, and the accessibility state reads.
@task_item n=11 part=needs
The Dioxus bridge keeps escher's behaviour through the take: stable ids, snapshot, UI graph, reached record and the buffered render. Each resolution is recorded.
@task_item n=12 part=needs
Additivity: edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires. Each is named, recorded and flagged upstreamable, with nothing sent upstream.
@task_item n=13 part=needs
An upstream commit that edits a handle_click arm does not pick cleanly. It is re-applied by hand to that arm's private click_* function, within the cognitive complexity ceiling.
@task_item n=14 part=needs
If the take touches the hit walk's inner function, the inline-root clipping early return must return content-box coordinates like the final self hit, and gets a check of its own. The non-invertible-transform answer and the clip-padding-box hit stop are kept.
@task_item n=15 part=needs
The earlier takes' merge surfaces are re-read row by row: the deallocated-anonymous-block fix (the dropped block detaches its children and clears their layout parent) and the earlier resolutions.
@task_item n=16 part=needs
The CDP server's served listener stays out with its commit unless the take cannot stand without it. No new crate, port, env var or listener of ours is added, and no served network port is opened beyond the one local Unix-domain listener.
@task_item n=17 part=needs
Upstream's mounted-event test stays out while Dioxus is on 0.7, and the buffered render's merge surface is left as it is until the Dioxus 0.8 move.
@task_item n=18 part=needs
Upstream's new and edited test files run under the take as integration tests in blitz-tests, one file and one target per behaviour, and the pinned target list must match them.
@task_item n=19 part=needs
The WPT-only commits (a skia CPU raster renderer option and runner changes) are taken, and the WPT runner keeps defaulting to CPU rendering.
@task_item n=20 part=needs
After the take, the CI quality gates pass: fmt and clippy, rustdoc warnings as errors, the locked cargo legs, the stand checks in both layout modes, the incremental/non-incremental oracle (both layout modes give identical layouts), the a11y leg, the embolden local witness, and fork CI green with every job settled. The route-only commit's unread verdict is re-read before the push.
@task_item n=21 part=needs
The per-OS tallies are re-read from the take's green CI test-bearing job logs, and the stand readings must stay identical on every CI OS.
@task_item n=22 part=needs
The sink allowlist: the taken delta is read for every new tracing or log call site, target, print and env read. These are held to the telemetry target filter and scrub and to engine tracing gated per call site, and the capture checks stay at 0 needles.
@task_item n=23 part=needs
If the take touches the browser app, the unjustified expect_used allow on the tab webview gets its justification written beside it, as the justified expect/unwrap rule requires. Otherwise that carry rides on to the next entry.
@task_item n=24 part=needs
Upstream AGENTS.md stays out of the tree, and upstream-only workflows and CI resources stay upstream-only through the take.
@task_standing n=1
convention: each upstream test file taken lives in blitz-tests as one behaviour per file with its own test target, and states shared fixtures once
home: conventions
@task_standing n=2
convention: a new check (the inline-root hit coordinate, if owed) gets a descriptive test name, cites the upstream commit it regresses against, asserts its fixture first and runs in both layout modes
home: conventions
@task_standing n=3
convention: any expect or unwrap in new or resolved code carries a justification, and clippy allows stay local and justified
home: conventions
@task_standing n=4
convention: every new log line or tracing call site is gated per call site and passes the telemetry scrub and target filter
home: cross-cutting-patterns
@task_standing n=5
convention: every CI leg the take re-runs goes through the one leg runner, locked
home: standard-contracts
@task_item n=1 part=not_needed
The driver's verbs, session wire, MCP server and escher-session command line and exit-code contracts: the take adds no verb, carrier, argument or exit code.
@task_item n=2 part=not_needed
The cold-agent and agent-run scripts' events, exit codes, verdicts and state directories: the upstream delta does not reach them.
@task_item n=3 part=not_needed
The lint verb, its reason keys and the refusal causes and faults: no upstream commit in the take changes them.
@task_item n=4 part=not_needed
The kept-check file, its reader and the recording directory contracts: the take changes no recording.
@task_item n=5 part=not_needed
The browser's history store, about pages and persistence decisions: the take brings no browser-history or about-page change.
@task_item n=6 part=not_needed
The WPT diff-to-PR tool's command line and output, the PR body markers and the WPT repository dispatch: WPT conformance numbers are out of scope.
@task_item n=7 part=not_needed
The page-script runtime's globals, bootstrap and script fetcher (Boa, document_may_load): no taken commit touches scripting.
@task_item n=8 part=not_needed
The rdme app's Markdown engines and file watching, and the Android, iOS and wasm entry decisions: the upstream delta does not reach them.
