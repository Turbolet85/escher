@task_prose part=chunk
Four engine debts left as residuals by 0.1.0 to 0.3.0, each put to the founder as a fork, then fixed or kept by his word.
1: a focused control hidden by the hidden attribute, inline display:none or CSS alone keeps focus and takes keys; fork: a focus fixup that gives focus up after the next layout, or keep.
2: a task polled during a Dioxus buffered render cannot focus a node made in that render; fork: a deferred or replayed act, or keep.
3: a page script's Math.random is unseeded; fork: seed Boa's Math.random from a document-level seed. 4: the error-boundary registry grows with each mount; fork: prune by a reachable signal, wait on a dioxus-core release, or keep.
A fixed debt is proven headless on the stand or by a fixture in both layout modes; a kept one stays a residual with the founder's reason.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the document mutator, whose flush applies DOM mutations and clears focus from a control that turns inert — edits: DocumentMutator::flush's focus clear
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document, holding focus state and the inert test — edits: clear_focus, focus, is_inert
@task_file n=3 path=packages/dioxus-native-dom/src/
what: the Dioxus native DOM bridge: the buffered render on the Dioxus 0.7 bridge, MountedData's set_focus, the seeded source SeededRandom and DEFAULT_SEED as a root context, the error-boundary registry and root_error_mark
@task_file n=4 path=packages/blitz-vibey-script/
what: the page-script runtime on Boa: the runtime's set-up and Boa's Math.random host hook
@task_item n=1 part=needs
Focus on a control that no longer renders: a focused control hidden by the hidden attribute, inline display:none or a CSS rule alone must lose focus after the next layout and take no keys, where today only an inert subtree clears focus; read through the harness's focused inspection and key dispatch.
@task_item n=2 part=needs
Inert-subtree focus clearing stays as it is beside the new fixup; new checks for the hidden control sit beside the existing inert-subtree checks of blitz-tests.
@task_item n=3 part=needs
The buffered render on the Dioxus 0.7 bridge: a task polled during a render acts on the document the previous render left; a fix makes MountedData's set_focus on a node created in that same render take effect (set_focus_to no longer false) by a deferred or replayed act, measured against the spawned-acts probe pins.
@task_item n=4 part=needs
The upstream buffered-render test stays as written and the buffered render keeps the document unborrowed while a task runs; a deferred act must not borrow it mid-render.
@task_item n=5 part=needs
Seeded page-script randomness: Boa's Math.random seeded from a document-level seed tied to the run's seeded source (SeededRandom, DEFAULT_SEED), so the same seed gives the same page-script sequence; reached through the harness's from_vdom construction and documents built from HTML.
@task_item n=6 part=needs
The seeded source reads no environment variable, file or carrier, and the script runtime's security rules for its fetcher and runtime set-up hold for any seed handed to Boa.
@task_item n=7 part=needs
The error-boundary registry: boundaries built under a document's factory are pruned by a reachable signal (a weak handle or a generation) so a remounting app no longer grows it, while root_error_mark and the root boundary's Err reading stay correct, the root-boundary Err checked on its own session.
@task_item n=8 part=needs
A fix to items 1 to 3 changes the engine for every document, remote pages included: checks reach it through documents built from HTML as well as from Dioxus components.
@task_item n=9 part=needs
A fix adds no carrier, verb, argument, field, cause, log field or span unless the founder rules one; what the driver and session already answer stays byte-for-byte the same.
@task_item n=10 part=needs
Each fixed debt is proven headless: a stand check or a fixture whose layout assertions run in both layout modes.
@task_item n=11 part=needs
A debt kept by the founder stays a residual with his reason, and a wait on a dioxus-core release is named as such; no test is added for a kept limit beyond what pins the limit as it stands.
@task_item n=12 part=needs
The focus fixup acts after the next layout, read on harness pump, tick and settle under the virtual clock; settle never advances time.
@task_item n=13 part=needs
blitz-dom unit fixtures are built without the HTML parser; where a check depends on parsed markup (the hidden attribute, a stylesheet rule) it is pinned where the real parser runs.
@task_item n=14 part=needs
Bridge tests of the Dioxus native DOM reach the buffered render, set_focus and the boundary registry as a child module of the dioxus document, without widening visibility.
@task_item n=15 part=needs
Vibey-script tests of the seeded Math.random run on the script runtime as built, with the seed stated in code.
@task_item n=16 part=needs
Component-alone harness construction provides its root context, the seeded source among them, before the build.
@task_standing n=1
convention: each new integration test file is its own test target, one target per file, the blitz-tests manifest keeping its one-target form
home: 1-test-scope-summary · test-scope-summary
@task_standing n=2
convention: each new unit test sits in an inline test module of its source file
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: each new regression test is named for its bug and documents it
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: each new test's data is stated in code, nothing seeded from outside, shared fixtures kept in one module
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: each new check uses no sleep or wall-clock read; time moves only through the virtual clock
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=6
convention: each new test is controlled red-before-green, pins its expected value as a literal, counts the rows of its table and prints no argument value on failure
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=7
convention: each new stand check runs on the bundled font with no skip and holds inside the three-round zero-failure stand repeat
home: 9-ci-integration · ci-integration
@task_standing n=8
convention: each new stand-in is hand-written; no mocking library
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=9
convention: each touched crate keeps its line coverage floor
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_item n=1 part=not_needed
The session host, the session wire, session errors and state directory: the chunk changes no session lifecycle or protocol.
@task_item n=2 part=not_needed
The escher-session command line, its exit codes, help, snapshot budget and the MCP role's tools and results: no command or tool changes.
@task_item n=3 part=not_needed
Agent-run verbs, selections, events and exit codes, and the cold-agent pipe, its verdict and events: the chunk touches no test runner script.
@task_item n=4 part=not_needed
Session recording, committed recorded checks and check replay: no recording format or replayed check changes.
@task_item n=5 part=not_needed
Handler coverage, lint defects and text contrast readings: the chunk changes no reader of these.
@task_item n=6 part=not_needed
The WPT runner, its CLI, outputs, status set and pinned commit: no conformance work.
@task_item n=7 part=not_needed
CI action pinning, cache, package install scripts and their exit codes, secrets and permissions: dependency and CI debts are out of scope.
@task_item n=8 part=not_needed
Select list painting and the chosen select row colours, the single select sized as a list box: out of scope.
@task_item n=9 part=not_needed
Screenshot and pixel tests on a CPU buffer, PNG decoding and box-shadow expectations: the chunk makes no paint change and no canvas work.
@task_item n=10 part=not_needed
Driver refusal causes, their order and remedies, and the driver's advance command: a fix adds no cause and moves no clock surface.
@task_item n=11 part=not_needed
Password masking and typed-password privacy rules: no text-entry change.
@task_item n=12 part=not_needed
The UI graph's owner keys, task-state registration and snapshot node fields: the chunk changes no graph or snapshot answer.
