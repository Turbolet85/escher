@task_prose part=chunk
Mounts one component of an app alone, with props its caller gives and validates first, into a headless document under the stand's boot conditions (800 × 600 viewport, bundled font, default seed), named across the UI graph's component boundary.
Proves for at least one stand component that the isolated mount's snapshot equals that component's subtree inside its app, with ids anchored at the component.
Writes the founder's fork on how the isolated component crosses (in process only, a closed table of named components and prop sets, or open props over the carriers) with each option's price, and builds no carrier before his word.
Joins the layout verb to the cold-agent pipe's verb list and its contract tests so the pipe's isolation check against the MCP server's fifteen tools holds again, with no live run.
Adds a first test of the boundary-marks fallback branch, re-reads the last wrap's CI verdict, and its wrap lands the exception on the no-attribute-text reply row, changing no code for it.
@task_file n=1 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand, its boot at a pinned viewport, font and seed (seven_guis::stand) — edits: the boot path, so it can root a document at one component with given props
@task_file n=2 path=packages/blitz-test-harness/src/lib.rs
what: the test harness that boots a headless document from a Dioxus virtual DOM (Harness::from_vdom), with its virtual clock and run seed
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the Dioxus document's boundary marks reader (boundary_marks and BoundaryMark) — edits: a unit test of its fallback branch, for a document built after its vdom's first build
@task_file n=4 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe's command line and its isolated session over the MCP server — edits: the pipe's verb list, which gains layout
@task_file n=5 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests, including the isolation check that compares the pipe's verb list with the MCP server's tool listing — edits: the verb-list tests, which gain layout
@task_item n=1 part=needs
A component mounted alone boots under the same conditions a stand task reads: the stand's pinned viewport (800 × 600), its bundled typography, offline, and the run's one default seed through the seeded-random hook.
@task_item n=2 part=needs
The isolated document is a headless document and keeps the headless document defaults (light scheme, sequential style, virtual clock, CPU screenshots); the mount adds no engine change that every document would see.
@task_item n=3 part=needs
What is mounted is a component the UI graph names as an owner, and the isolated document's graph names it as its root's component; owners are read through the Dioxus public API and signals reach the graph by registration, with the first graph-state hook providing the root registry.
@task_item n=4 part=needs
The isolated reading's equality with the in-app reading is over ids, roles, names and states, and bounds where the box is the same; ids follow the stable element id tiers (author key, component path, anchored path), so isolated ids differ from in-app ids by the component's prefix and the equality is stated anchored at the component.
@task_item n=5 part=needs
Snapshot readings compared for the equality are the snapshot node keys and node state as the snapshot verb already reads them; no process-local value enters a reading, and the readings stay identical on every CI OS.
@task_item n=6 part=needs
Given props are validated before anything mounts: a props value the component cannot take is refused as a typed value (an error enum, a typed result), never a panic on untrusted input.
@task_item n=7 part=needs
The founder's fork is raised with every option and its price, none taken for him; no carrier of the isolated mount (a verb, an argument, an MCP tool, a kept check, a start option) is built before his answer, since a new carrier widens the session boundary and the validated verb set widens only by ratification.
@task_item n=8 part=needs
The boundary-marks fallback branch (a document built around a vdom already built, so the root boundary is not in the registry) gains a test, by the isolated mount or by a fixture, that never re-runs rebuild on a built vdom; the Dioxus bridge's buffered render and the mounted-after-build-and-poll order bind how that document is built.
@task_item n=9 part=needs
The cold-agent pipe's verb list gains layout as graph and actions joined, so its isolation check against the MCP server's tool listing of fifteen tools holds and the pipe is runnable; the run stays one isolated session reaching only the MCP server's tools.
@task_item n=10 part=needs
The cold-agent boundary widens by the founder's word: on a live run a layout answer (ids, closed words, numbers, no text) can reach the model provider, and an agent-written kept check can hold a layout step; the model provider is still reached only on the live cold-agent run, and no live run happens in this chunk.
@task_item n=11 part=needs
The no-attribute-text reply rule gains its exception at the wrap: an attribute's text does not cross a reply except what an author wrote into a record's message or into the error field of a caused item; the built code already does this and does not change.
@task_item n=12 part=needs
The chunk opens no new socket, port, IPC endpoint or credential path, adds no log field or span content, and lets no NodeId cross anything, the isolated mount and its tests included.
@task_item n=13 part=needs
The last wrap's CI verdict is re-read at P3 and never read as green before it lands.
@task_standing n=1
convention: unit tests sit at the end of their source file, or in a child test module of it, with descriptive names
home: conventions
@task_standing n=2
convention: a test asserts its fixture's own condition first, and fixture cases are numbered and stated once
home: conventions
@task_standing n=3
convention: expect and unwrap in new code carry a justification; fallible new APIs return typed results with error enums
home: conventions · cross-cutting-patterns
@task_standing n=4
convention: new public items and modules carry docs, and rustdoc warnings stay errors
home: conventions
@task_standing n=5
convention: stand checks keep shared helpers in non-target modules and integration test files hold one behavior each
home: conventions
@task_standing n=6
convention: CI helper scripts and their tests are stdlib Python under unittest, and the CI workflow contract stays pinned by tests
home: conventions · standard-contracts
@task_standing n=7
convention: new code holds each crate's line coverage floor and the cognitive complexity ceiling
home: standard-contracts · stack-and-technologies
@task_item n=1 part=not_needed
The WPT runner, its suites, result kinds, statuses, outputs, CI artifacts and the PR-body diff script: no web platform test is run or reported here.
@task_item n=2 part=not_needed
The reference browser app: its history store, about pages, toolbar, favicon cache, persistence and publish job.
@task_item n=3 part=not_needed
CSS layout and paint engine internals: Taffy mapping of tables and grids, writing modes, text decoration, gradients, paint layers, scrollbar geometry.
@task_item n=4 part=not_needed
Script engine rows: Boa, script globals, script fetching, microtask drains and the JS bootstrap.
@task_item n=5 part=not_needed
Networking rows: the net provider, HTTP cache, per-host request caps and URL scheme routing.
@task_item n=6 part=not_needed
Form control and select-list behaviour: select keyboard model, list painting, range input, multiple select, form submission.
@task_item n=7 part=not_needed
Platform shells and targets: winit windowing, Android and iOS entries, wasm builds, the platform accessibility adapter.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and take rules, the release and versioning rules, the Nix flake and the MSRV job.
@task_item n=9 part=not_needed
The lint verb's defect set and reason keys, and the screenshot encoding: the isolated reading compares snapshots, not findings or pixels.
@task_item n=10 part=not_needed
The agent-run script's events, exit codes and state, and the coverage role's exit codes: neither is changed by joining layout to the cold-agent pipe.
