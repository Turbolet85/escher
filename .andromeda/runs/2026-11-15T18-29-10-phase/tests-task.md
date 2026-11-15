@task_prose part=chunk
Moves the workspace's taffy dependency from the crates.io registry 0.15.1 to upstream's git pin a3997d55 of dioxuslabs/taffy, so the tree carries Taffy #1303 (the caller-defined float id) and the Taffy content of upstream's left-out chain.
Takes the blitz side of that chain where it touches escher's tree: the inline layout call that passes the node's taffy id as place_floated_box's new first argument, and the code edits of nine "Bump Taffy" commits; no other upstream pick.
The lock moves with it: one taffy package from a git source in place of the registry one, every other package only as the move forces.
Before any plan, research re-measures the move in a trial worktree: whether the workspace builds, which gates and tests change (fast leg, clippy, audit, MSRV 1.91, doc, build, the incremental layout oracle, the stand checks, the WPT runner's build), the lock's package moves, and the standalone wasm example's own taffy resolution.
A red gate with no in-scope fix, an MSRV above 1.91, a broken wasm leg or a lock move a rule refuses stops the phase with the measurement; a fork that widens a boundary or needs a rule lifted beyond the Taffy-only exception goes to the forks file and stops.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace root manifest, which declares the workspace's shared dependencies and their sources — edits: the taffy dependency entry (registry version to git source at a pinned rev)
@task_file n=2 path=Cargo.lock mark=edits
what: the workspace lock that every cargo leg runs locked against — edits: the taffy package entry (registry source to git source), and only the packages the move forces
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=edits
what: blitz-dom's inline layout, which places floated boxes in the block formatting context during inline layout — edits: the place_floated_box call (the node's taffy id as its new first argument)
@task_file n=4 path=examples/wasm_hello/Cargo.toml mark=re-reads
what: the wasm example's standalone workspace with its own lock, outside the root's locked legs; its taffy resolution is read on its own
@task_file n=5 path=deny.toml mark=re-reads
what: the dependency audit's advisories and licences checks, which a git taffy must still pass
@task_item n=1 part=needs
The cargo legs run locked: every build, test and CI leg keeps its locked flag, and the moved lock must satisfy it with taffy from a git source.
@task_item n=2 part=needs
The minimum supported Rust version 1.91, verified by build only: the git taffy and anything it pulls must build at 1.91, or the move stops.
@task_item n=3 part=needs
The dependency audit over advisories and licences: a git taffy still passes it; any second git source the move would bring is a fork, not a fix.
@task_item n=4 part=needs
Clippy over the workspace and upstream blitz-dom: the taffy move and the inline-layout call change must leave both clippy legs clean.
@task_item n=5 part=needs
The workspace tested in the fast CI leg, and per-package green is not workspace green: the measurement reads the whole workspace's tests, not one crate's.
@task_item n=6 part=needs
The incremental layout differential oracle: its invariant holds after the move; the move is re-measured against it, since the new taffy changes layout.
@task_item n=7 part=needs
Layout assertions and stand checks run in both layout modes: a layout change brought by the new taffy is read in both modes, not one.
@task_item n=8 part=needs
The stand checks on the bundled real font with no skip, and their zero-failure repeat budget: the move must leave them green under the same budget.
@task_item n=9 part=needs
Floated box placement: Taffy #1303's caller-defined float id changes how inline layout hands a float to the block context; a test that pins float placement reads that the floats still place where they did, or names what moved.
@task_item n=10 part=needs
Layout behaviour changes the chain brings change layout for every document, remote pages included; each one the measurement finds is named as a change (and as a fork on whether it needs its own ratification), and a test proving a changed layout documents what moved.
@task_item n=11 part=needs
The WPT conformance runner must still build against the git taffy; its reading stays report-only, so a moved WPT score is measured and named, never a gate to fix.
@task_item n=12 part=needs
The blitz-tests manifest's difference from upstream: the taffy source the tests build against follows the workspace's, and that difference is re-read rather than widened.
@task_item n=13 part=needs
The local pre-push gate and host-reproducible CI legs: the measurement runs the same legs on the host in the trial worktree that CI runs.
@task_item n=14 part=needs
Regression tests document and are named for their bug: any test the chunk adds for a layout the move breaks or fixes follows that rule.
@task_item n=15 part=needs
Font-metric tests never skip and pixel tests stay on a CPU buffer: a layout move the new taffy brings is read through them as they stand, not by loosening them.
@task_standing n=1
convention: each new integration test of a layout behaviour sits in its own test target, one target per behaviour and per file
home: 1-test-scope-summary · test-scope-summary
@task_standing n=2
convention: each new unit test sits in an inline test module in its source file
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: each new test's data is stated in code, its fixtures built without seeding, shared fixture pages stated once
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: each new test is shown red before green, and a table-driven test asserts its row count
home: unit-test-strategy
@task_item n=1 part=not_needed
The session lifecycle, its socket wire, host, state directory, errors and recording rows: the chunk changes no session code and no wire.
@task_item n=2 part=not_needed
The MCP server, the escher-session command line, its exit codes, stderr and stdout lines: no command-line or MCP surface moves.
@task_item n=3 part=not_needed
The cold-agent pipe, its events, verdict, exit codes and isolation: no cold-agent run is part of this chunk.
@task_item n=4 part=not_needed
The agent-run verbs, selections, events and exit codes: the agent-run contract is untouched.
@task_item n=5 part=not_needed
Driver refusal causes, driver verbs, snapshot nodes, element ids and the driver's log line: the driver's answers and refusals do not change.
@task_item n=6 part=not_needed
Harness contrast, lint defects and the contrast formula: no contrast reading or lint check is changed.
@task_item n=7 part=not_needed
Handler coverage and its limits, listener reading from the vdom walk: event handling is not touched.
@task_item n=8 part=not_needed
Committed recorded checks, their recording script and byte-equality to the recording: no recorded check is added or re-recorded.
@task_item n=9 part=not_needed
Virtual clock, advance, settle and timer-tick rows: the chunk changes no time or settle behaviour.
@task_item n=10 part=not_needed
Mocking discipline and network stand-ins: the chunk adds no stand-in and touches no provider.
@task_item n=11 part=not_needed
CI action pinning, secrets, read-only permissions, cache saving and trigger branches: the chunk edits no CI workflow.
@task_item n=12 part=not_needed
Mutation testing and the cognitive-complexity ceiling on the dev host: not part of the move's measurement.
@task_item n=13 part=not_needed
Password masking, typed-password and select-row colour rows: no form-control behaviour is in scope.
