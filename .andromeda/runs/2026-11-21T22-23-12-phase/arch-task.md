@task_prose part=chunk
Closes or states the dependency audit's standing per-ID advisory ignores (libsqlite3-sys via rusqlite, paste via rav1e, memmap2 unsound, the zbus family via accesskit_unix, zip via anyrender_serialize, and reads ttf-parser via winit): each by a reachable version move, a wait on a named release, the founder's keep, or a stated non-closing.
Adds skia's verified prebuilt archive, as the founder ruled: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL, a CI step and a fetch script per platform, no line of upstream code changed.
Acts on the cancelled-runner watch's recurrence (the hosted iOS/macOS job cancelled with no runner) and finds why a CI=true fast leg on the dev host recompiles under new hashes, making it reuse the host's units or stating the cause as a limit.
Re-measures each advisory's reaching chain and the skia jobs at P3; claims capability v040-08 (Codebase health) against the Epoch 2 boundary audit, with the founder's keeps as its kept half; the stale default branch in the workflow doc lands in the operator's slot, never a code step.
Registers the skia fetch and each advisory's standing in the security plan's Dependencies section; founder forks go to founder-forks.md and the phase stops there.
@task_file n=1 path=deny.toml mark=edits
what: the workspace dependency and licence audit's cargo-deny configuration, the advisory gate — edits: the [advisories] ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace root manifest, where shared workspace dependencies are declared once with their version pins — edits: the version pins an advisory's update reaches
@task_file n=3 path=Cargo.lock mark=edits
what: the workspace lock file the locked cargo legs build against
@task_file n=4 path=packages/browser-persistence mark=edits
what: the browser-persistence crate, the browser's best-effort synchronous SQLite history store on rusqlite — edits: its rusqlite pin and what a rusqlite move reaches
@task_file n=5 path=apps/browser mark=edits
what: the browser app, Dioxus chrome over Blitz, built with skia as a default renderer, reaching rusqlite, zip through anyrender_serialize capture and the avif image feature — edits: its manifest where an update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the fork's GitHub Actions CI workflow, its jobs and steps — edits: a skia archive fetch-and-verify step for each job that builds skia, the hosted iOS/macOS job the cancelled-runner watch reads
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the one CI leg runner that CI and the local pre-push gate both call, its legs (fast, audit, msrv and the rest) — edits: the skia checksum check and SKIA_BINARIES_URL wiring, the fast leg's CI-dependent environment
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow contract tests that pin the workflow's pins and invariants — edits: pins for the skia step and checksum and for the iOS job's handling
@task_item n=1 part=needs
The cargo-deny advisory gate and its standing per-ID ignores: each ignore removed by a fix or kept with a stated wait, keep or non-closing reason; the audit leg stays green.
@task_item n=2 part=needs
cargo-deny's unsound-advisory default kept on as a measured limit (memmap2), never turned into an ignore.
@task_item n=3 part=needs
A dependency version move is a manifest and lock change under the locked cargo legs, with the audit, MSRV (build-only MSRV job, advertised rust-version) and lock legs green; dependencies stay declared once in the workspace, with no git dependency.
@task_item n=4 part=needs
A rusqlite move reaches the SQLite browser history: the history store, its history_entries table and HistoryEntry, its append-only migrations and the synchronous initial read must keep their behaviour through any API change.
@task_item n=5 part=needs
A zbus-family close runs through accesskit_unix under the accesskit_xplat adapter: its async features on Unix and its coupling to the pinned AccessKit version bind any move.
@task_item n=6 part=needs
The paste and zip closes run through image raster decoding (rav1e/ravif under the avif feature, anyrender_svg) and anyrender_serialize capture in the browser; a move must keep decoding and capture reading entries by name.
@task_item n=7 part=needs
ttf-parser is reached through winit's exact beta pin, a coupled pin the chunk may not loosen.
@task_item n=8 part=needs
Skia stays a default renderer of the browser app and the WPT runner (renderer chosen at compile time, anyrender over Skia); the verified archive changes how it is obtained, never which renderer is built or any upstream code.
@task_item n=9 part=needs
The skia archive's checksum lives in the repository, is updated by hand at each skia version move, and every job and local gate leg that builds skia checks it and points skia-bindings at the verified file through SKIA_BINARIES_URL, never a silent source build on a failed fetch.
@task_item n=10 part=needs
SKIA_BINARIES_URL and CI as environment names the build and the leg runner read, under the names register of other identifiers.
@task_item n=11 part=needs
One CI leg runner: CI jobs and the local pre-push gate call the same legs, so the skia check and any CI-only switch are in the runner, with its exit status, leg log and unknown-leg behaviour kept.
@task_item n=12 part=needs
The CI=true recompile on the dev host: the fast leg's environment under CI, the one debuginfo level for dev host and CI, line-tables-only dev/test debuginfo and unrewritten build profiles bind what the cause may be and what a fix may change.
@task_item n=13 part=needs
The cancelled-runner watch: acting on the iOS job's no-runner cancel (retry, continue-on-error, removal from the required set, or the founder's keep) within the CI fast/slow split, per-ref concurrency and the required checks.
@task_item n=14 part=needs
The skia fetch is a supply-chain surface registered in the security plan's Dependencies section; no new socket, port, listener, credential path or secret, and the no-served-network-listener absence stays true.
@task_item n=15 part=needs
Read-only CI permissions and the Rust cache on compiling jobs (save refs, restore-only test cache) bound where a fetched archive may be kept between runs.
@task_item n=16 part=needs
Upstream-only workflows and upstream CI resources stay reached only from upstream; the fork's skia step and iOS handling live in the fork's own CI.
@task_item n=17 part=needs
The bounded, retried package-install pattern (repair before retry, one install script) as the model for a bounded fetch of the skia archive.
@task_item n=18 part=needs
The Nix flake dev shell and its single-derivation package build, where it builds skia, keep building with the verified archive or state they do not reach it.
@task_item n=19 part=needs
The capability statement: v040-08 (Codebase health) claimed against the next boundary audit's reading, with the founder's keeps (oversize files, over-ceiling functions under the cognitive complexity ceiling, duplicated test pairs kept as upstream's) as its kept half.
@task_item n=20 part=needs
The workflow doc's default branch lands by the operator's word in the operator's slot, never a hand edit by this chunk.
@task_standing n=1
convention: a new or changed CI workflow test is a stdlib unittest
home: conventions
@task_standing n=2
convention: a new CI helper or fetch script in Python uses the standard library only and runs on the Python 3.11 floor
home: stack-and-technologies
@task_standing n=3
convention: every action a new CI step uses is pinned to a commit SHA
home: stack-and-technologies · infrastructure-patterns
@task_standing n=4
convention: every new CI job, step or pin is held by the CI workflow contract tests
home: standard-contracts
@task_standing n=5
convention: every cargo leg a new step runs is locked
home: standard-contracts · infrastructure-patterns
@task_standing n=6
convention: a new Windows script entry is a pass-through to the shared script
home: standard-contracts
@task_standing n=7
convention: a new CI job uploads its logs on failure
home: infrastructure-patterns
@task_item n=1 part=not_needed
Layout, style, text and paint engine decisions (Taffy, Stylo, Parley, damage, scrolling, select list painting): no engine behaviour changes.
@task_item n=2 part=not_needed
The driver's session, socket wire, MCP server, verbs, refusal causes, snapshot and diff contracts: no driver surface is touched.
@task_item n=3 part=not_needed
Accessibility tree content contracts (ARIA states, names, focus, checked): the adapter's dependency moves, what the tree carries does not.
@task_item n=4 part=not_needed
Telemetry bootstrap, scrub and log-line contracts: no log line is added.
@task_item n=5 part=not_needed
WPT runner result statuses, kinds, outputs, diff JSON and PR-body contracts: skia's provenance changes, not what the runner reports.
@task_item n=6 part=not_needed
Cold-agent, agent-run, kept-check and recording contracts and their verdict fields.
@task_item n=7 part=not_needed
The Dioxus bridge, stable element ids, UI graph and actionable-key rows.
@task_item n=8 part=not_needed
Script runtime rows (Boa, script globals, script fetcher, microtasks).
@task_item n=9 part=not_needed
DOM event dispatch, select-list default actions and form-control readers.
@task_item n=10 part=not_needed
About URLs, toolbar and tab rows: the chrome's behaviour is not changed.
@task_item n=11 part=not_needed
Canvas work and the upstream sync rows (two-parent merge, merge base, upstream take ratification): out of scope, their own entries.
