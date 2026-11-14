@task_prose part=chunk
Builds a seed for a run of the driver: a held Session, and the harness it holds, is given one seed, and every source of randomness the run's app and page can read draws from it.
The app-facing source: a Dioxus app hosted by the driver reads its randomness from the run's seed through one engine-provided source (a root context or hook), not from the host.
The page-facing source: a page script's Math.random in a seeded run draws from the run's seed.
The proof: an integration check, in both layout modes, that two runs given one seed read the same state (snapshot text and diffs) at every step of a scripted sequence on an app that reads randomness, and that two runs given different seeds read a difference.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the capability statement pinned to the driver schema — edits: its self-description of the driver and its pin, where a seed joins Session::start or the verb table
@task_item n=1 part=needs
Driver session start: Session::start, or a builder beside with_time, takes one seed for the run; two runs given one seed read the same held state, snapshot text and diffs at every step, and two runs given different seeds read a difference.
@task_item n=2 part=needs
Harness construction and its options (HarnessOptions): the harness a session holds carries the run's seed, and every document it builds (from a component, from html, from a vdom, wrapped) draws its randomness from that seed.
@task_item n=3 part=needs
The app-facing randomness source: a Dioxus app on the bridge reads randomness through one engine-provided source seeded by the run, never the host's thread RNG.
@task_item n=4 part=needs
The page-facing randomness source: in a seeded run the script engine's bootstrap replaces Math.random so a page script draws from the run's seed.
@task_item n=5 part=needs
The integration proof must not pass vacuously: the different-seed pair must read a difference, and an empty run or a sequence on an app that reads no randomness is never a pass.
@task_item n=6 part=needs
A stand surface or fixture that reads randomness: the seven_guis headless stand reads none today, so a stand boot, stand task or shared fixture that reads randomness is added for the proof, booting offline, its data stated in code.
@task_item n=7 part=needs
Driver verbs and argument kinds: if start takes a seed, the verb table gains a new argument kind on a ratified carrier, and its lookup, level check and refusal of an out-of-grammar or malformed seed hold as for every argument.
@task_item n=8 part=needs
The escher-session command line and MCP role: a seed argument on start reaches the command form, the start command, help's self-description and tools/list with parity between the command line and MCP.
@task_item n=9 part=needs
The session host and session wire: a seed handed to start over the socket travels within the existing call and lifecycle messages and their bounds, with no new listener, port or socket.
@task_item n=10 part=needs
The kept check, if the seed is recorded: the session recording's record fields, the check file and Check::from_bytes carry the seed, and replay through the ordinary call path restarts with it so a kept check replays the run it recorded.
@task_item n=11 part=needs
Committed recorded checks stay byte-equal to the binary's recording and kept fresh where a recorded seed changes what a recording holds.
@task_item n=12 part=needs
Capability statement pinned to the driver schema: help's self-description and the capability document are read from the schema, and claim section names and counts stay held to it when start gains a seed.
@task_item n=13 part=needs
Indirect, through the content rule over logs: a seed's value is carried by no log line, command span, tool_call span, stderr line or event field, and no env read carries it; driver log lines keep fixed words and counts only.
@task_item n=14 part=needs
Indirect, through the virtual clock: a seed sits beside the run's one virtual clock as a second determinism input; the new checks read no wall time and use no sleep, and timers advance only by advance.
@task_item n=15 part=needs
Carve-out: the windowed shell and the browser app are not seeded runs; only the headless driver and harness runs are.
@task_item n=16 part=needs
The new checks run in both layout modes, on the bundled pinned font with no skip, and hold the zero-failure stand repeat across rounds.
@task_standing n=1
convention: each new integration test file is its own test target, one behaviour per target, and shared helpers live in a shared module that is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new stand or layout check runs in both layout modes on the bundled font, never skipping
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=3
convention: each new check reads no wall time and never sleeps; time moves only through the virtual clock
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=4
convention: each new session check owns its host process, sends child streams to null and is unix-gated where it uses the socket
home: session-lifecycle
@task_standing n=5
convention: each new test is controlled red-before-green and its failures print no argument value; table-driven tests assert their row count
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=6
convention: each new fixture is stated in code, shared fixture pages stated once, and the stand boots offline
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=7
convention: each new stand check joins the stand repeat rounds under the zero-failure budget
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
WPT conformance runner, its suites, statuses, CLI, outputs and pinned commit: no page conformance reading changes.
@task_item n=2 part=not_needed
Cold-agent pipe and agent-run harness: their verbs, events, exit codes, verdicts and isolation are untouched.
@task_item n=3 part=not_needed
Lint defects, text contrast and the WCAG formula: no lint or contrast reading changes.
@task_item n=4 part=not_needed
Handler coverage and its limits: coverage counting of listeners is not reached by a seed.
@task_item n=5 part=not_needed
Typed-password masking and caret placement: no masked value changes.
@task_item n=6 part=not_needed
Painting rows — select list painting, chosen row colours, scrollbar thumbs and fade, box-shadow, pixel tests on a CPU buffer: no paint path changes.
@task_item n=7 part=not_needed
CI supply chain — actions pinned to commit SHAs, no secrets, read-only permissions, package installs bounded and retried, caches, upstream-only publishing jobs: no workflow plumbing changes.
@task_item n=8 part=not_needed
Upstream buffered-render and mounted-flush tests and the buffered render bridge: kept as they stand, no seed reaches them.
@task_item n=9 part=not_needed
The graph and actions verbs' answers and their reserved id form: their content is not changed by a seed.
@task_item n=10 part=not_needed
Embolden witness, keyboard compatibility coverage exclusion and mutation tooling on the dev host: unrelated to randomness.
