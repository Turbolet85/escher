@task_prose part=chunk
The fork's tenth upstream take: Blitz upstream/main pinned at 563d3db73, taken in the shape the founder picks at P4 (cherry-pick on the registry pins, that plus Stylo 0.24, Parley 0.13 and winit beta.4, the whole merge with git pins, or nothing); 128 commits are left to take or leave, the 21 left out last time among them.
It folds two external CI reds into the take: the clippy needless_bool lint under rustc 1.100 at three engine sites, and the dependency-audit advisory on libsqlite3-sys under rusqlite in browser persistence, which gets a P4 call between a bump and an ignore by ID.
It pins every action of the three upstream-only workflows to a 40-hex SHA and widens the pinning check past the main CI workflow to every workflow. It also adds two headless frame tests (mix-blend-mode/isolation, and an inline box's own decorations), each in both layout modes.
It re-reads the merge surfaces (the anonymous-block deallocation fix, the non-invertible hit hunk, the bridge's buffered render), the per-OS test tallies of the take's green CI run and the telemetry sink allowlist over the taken delta. Boundary widenings it carries in go to the operator at P4.
It re-proves every escher proof after the take: the fast, doc, a11y and audit legs, the stand checks in both layout modes, the incremental oracle, the embolden witness and fork CI green.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the blitz-dom document core, BaseDocument, with the anonymous-block deallocation and the changed set — edits: the needless_bool clippy site, deallocate_anonymous_block where the take's anonymous-block transform fix meets it
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: blitz-dom layout damage propagation and damage-scoped cache clearing — edits: the two needless_bool clippy sites
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document and DOM bridge: element ids, snapshot, UI graph, reached record and the bridge's buffered render (the two render sites, the buffered_mutations field)
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs mark=merge
what: the Dioxus bridge's mutation buffer behind the buffered render, which upstream's Dioxus 0.8 file replaces whole if that move is taken
@task_file n=5 path=packages/dioxus-native-dom/src/lib.rs mark=merge
what: the dioxus-native-dom crate root and its module list (the buffered-render mod line)
@task_file n=6 path=Cargo.toml mark=merge
what: the root workspace manifest: workspace members and the shared dependency pins (stylo, taffy, anyrender, parley, Dioxus, winit, vello) and rust-version
@task_file n=7 path=Cargo.lock mark=merge
what: the workspace lockfile the audit and the locked cargo legs read, which upstream's coupled pin set moves
@task_file n=8 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT CI workflow — edits: every action reference pinned to a commit SHA with its ref as a trailing comment
@task_file n=9 path=.github/workflows/publish-browser.yml mark=edits
what: the upstream-only browser publish workflow — edits: every action reference pinned to a commit SHA with its ref as a trailing comment
@task_file n=10 path=.github/workflows/wpt-post-results.yml mark=edits
what: the upstream-only WPT results posting workflow — edits: every action reference pinned to a commit SHA with its ref as a trailing comment
@task_file n=11 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow contract tests that pin the workflows — edits: the action-pinning check, widened from the main CI workflow to every workflow
@task_file n=12 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning blitz-tests' one-target-per-file test target list — edits: the pinned target list, for the new frame tests and upstream's new test files
@task_file n=13 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner whose fast, doc, a11y, audit and embolden legs re-prove the take
@task_file n=14 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the escher-telemetry line format and scrub, home of the standing capture checks that must stay at 0 needles
@task_item n=1 part=needs
The coupled dependency pins: a take carries upstream's Stylo, Taffy, Parley, AnyRender/vello and winit pins as one set or none, and a pin set that does not resolve is a halt, never a one-sided fix.
@task_item n=2 part=needs
The no-git-dependencies rule: every crate is a crates.io registry version, so a whole merge bringing git Taffy and git AnyRender reverses a standing decision and is the founder's fork. The deferred Stylo, Taffy and Dioxus bumps are also in play.
@task_item n=3 part=needs
How upstream is taken: the merge base and its cherry-pick trailers, cherry-pick as the precedent shape, and each upstream behaviour change ratified per take as an engine behaviour change.
@task_item n=4 part=needs
Additivity and direct engine changes: any edit of upstream-owned code made for a conflict, a gate failure or a ruled fork is minimal, named, recorded as non-additive and flagged upstreamable or not.
@task_item n=5 part=needs
Conflict resolutions keep escher's engine surfaces: the document clock and no wall-time reads, the select model and open select list, hit stops at clips and the non-invertible-transform no-hit answer, scrolled-box bounds, text_extent, range and keyboard activation, inert and the password mask.
@task_item n=6 part=needs
The changed set and where it is written, including the open-select-list qualifier that lands as a wrap amendment; the take's AccessKit refresh and mutation changes are read against those write sites and against the accessibility-tree rebuild on a non-empty changed set.
@task_item n=7 part=needs
Anonymous blocks: a deallocated anonymous block detaches its children, a dropped one clears their layout parent, and anonymous blocks live only in layout children. The upstream fix resolving transforms through anonymous block boxes is read against these.
@task_item n=8 part=needs
The Dioxus bridge's buffered render and mutation-buffer replay, stable element ids, the snapshot, its text and diff, the UI graph and the reached record: the stand checks for ids, persistence, snapshot, graph and handler coverage must still pass.
@task_item n=9 part=needs
The mounted-event ordering (mounted listeners fire after build and poll, a stale mounted event is skipped): upstream's mounted-event test returns only if the chosen shape takes Dioxus 0.8.
@task_item n=10 part=needs
Actions pinned to commit SHAs in every workflow, and the CI workflow contract pinned by tests, now widened to every workflow while the WPT and publish workflows stay upstream-only.
@task_item n=11 part=needs
The fmt and clippy gate under the latest stable toolchain CI runs (clippy -D warnings), green once the needless_bool lint is taken or ported.
@task_item n=12 part=needs
The cargo-deny advisory and licence audit gate: the libsqlite3-sys advisory is settled by a semver-compatible update or an ignore by ID with a written reason, and every crate that enters is reviewed by hand and licence-checked.
@task_item n=13 part=needs
MSRV 1.91 is advertised and built by the MSRV leg; any dependency the take brings that declares a higher rust-version is read against it, and the flake Rust pin follows rust-version.
@task_item n=14 part=needs
The telemetry sink: escher-telemetry scrubbing, the target prefix drop and engine tracing gated per call site. Every new tracing or log call site, target, print and environment read in the taken delta is read against them.
@task_item n=15 part=needs
Occupied resources: no served network port, one local Unix-domain listener, and no new crate, port, env var or listener of escher's. DevTools/CDP and upstream's new crates are read for membership and for any socket they open, which would need a security-plan amendment first.
@task_item n=16 part=needs
Boundary widenings carried by the take (lossy page decoding, new legacy attribute reads such as font face and size, valign, an empty alt as decorative, text-shadow and mask paint, the AccessKit refresh, new env reads) are listed for the operator, and new script engine APIs are read as an untrusted-input surface.
@task_item n=17 part=needs
Paint: mix-blend-mode and isolation, mask isolation forcing a buffer, and the skip of an inline box's own text decorations each get a headless frame test, screenshotted on the Vello CPU renderer over white.
@task_item n=18 part=needs
Both layout modes give identical layouts, and the incremental and non-incremental oracle still agrees after the take.
@task_item n=19 part=needs
Moved numbers: where the take moves Taffy, Stylo, Parley or vello, the exact bounds, snapshot byte sizes, lint findings and screenshot pixels in the stand checks are re-measured and recorded, never loosened. This includes the font-embolden local witness.
@task_item n=20 part=needs
Stylo prefs are set once by Stylo 0.22 names, and the HTML parser and parser crates match Stylo's atoms: a Stylo 0.24 move with selectors 0.43 and html5ever 0.40 must keep them coupled.
@task_item n=21 part=needs
The accessibility target, re-proven through the accessibility tree's ARIA and native state reads and the a11y leg, which widens in place.
@task_item n=22 part=needs
The fork's CI: one CI leg runner, cargo legs run locked, CI logs uploaded on failure, and fork CI green on the pushed commit with every job settled. The per-OS tallies are re-read from the test-bearing job logs.
@task_item n=23 part=needs
Workspace membership and the crate name register: upstream's new crates (the code editor, devtools) are checked against the workspace members and the published-crates lockstep versioning.
@task_standing n=1
convention: each new integration test lives in blitz-tests as its own file and its own test target, one behaviour per file
home: conventions
@task_standing n=2
convention: each new scenario or frame test runs in both layout modes
home: conventions
@task_standing n=3
convention: each new test has a descriptive name, asserts its fixture's condition first, and cites upstream where it guards an upstream regression
home: conventions
@task_standing n=4
convention: shared blitz-tests fixtures are stated once, with relative asset URLs and numbered cases
home: conventions
@task_standing n=5
convention: each workflow change stays pinned by the CI workflow contract tests, run as stdlib unittest
home: standard-contracts
@task_item n=1 part=not_needed
The driver session socket wire, its hello, call and stop requests, reply bounds and session errors: the take adds no verb and no carrier.
@task_item n=2 part=not_needed
The MCP server and mcp role contracts (handshake, tools/list, tools/call, result bounds): no driver-facing protocol changes in a sync.
@task_item n=3 part=not_needed
The cold-agent and agent-run tooling, their events, exit codes, verdicts and state directories: they are host tools this take does not touch.
@task_item n=4 part=not_needed
Kept checks and session recording (check files, digests, replay, the recording directory): no recording format changes.
@task_item n=5 part=not_needed
The driver's verbs and their answers (actions, graph, lint, snapshot paging, refusal causes and faults, argument kinds, key names): adopting new upstream APIs in escher's own code is out of scope.
@task_item n=6 part=not_needed
WPT runner results and diffs (result kinds, statuses, flags, harness and subtest codes, the diff-to-PR output): upstream's WPT conformance numbers are out of scope.
@task_item n=7 part=not_needed
Browser chrome features (about pages, toolbar, URL suggestions, tabs, the history service and its fold rules): the take changes no browser UI behaviour of ours.
@task_item n=8 part=not_needed
Platform entry points and shells (Android NativeActivity, iOS redraw, macOS safe area, the Windows subsystem, wasm resize and font setup): no platform work in this sync.
@task_item n=9 part=not_needed
The rdme app's Markdown engines, file watching and wikilinks, and the example CLIs' arguments.
@task_item n=10 part=not_needed
The Nix flake's package derivation and checks, the apt install script's arguments and exit codes, and the mutation tier's nextest profile.
@task_item n=11 part=not_needed
The capability statement's sections and the root README's tool count: no driver capability changes.
