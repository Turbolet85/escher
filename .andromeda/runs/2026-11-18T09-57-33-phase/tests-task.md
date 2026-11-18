@task_prose part=chunk
Builds a seventh served stand app, the served error page, whose controls make a handler panic, return an error the root error boundary takes, and log, so an acting call's answer on it carries `caused`.
Adds its stand checks (headless boot, both layout modes, each control's `caused` items tied to its stable id, owner and event; listed and reached through the host and the mcp role).
Adds a goal for it to the cold-agent pipe's stand prompt and a count of the calls whose answer carried `caused` to the cold-agent verdict, with whether the run is required to meet it.
Moves the pipe's contract tests and the capability statement pins where the served-app count or the verdict's fields are held; runs no live agent and reaches no model provider.
@task_file n=1 path=examples/seven_guis/src/tasks/
what: the seven_guis stand's task apps — the new seventh served app, the served error page, joins them
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand, its boot functions — edits: the headless boot of the new app
@task_file n=3 path=examples/seven_guis/src/session_host.rs mark=edits
what: the session host's served-task table of the escher-session binary — edits: the served-task table, one entry added
@task_file n=4 path=examples/seven_guis/src/app.rs mark=edits
what: the windowed stand, its list of task apps — edits: the new app's entry, if it joins the windowed stand
@task_file n=5 path=tests/blitz-tests/tests/
what: the stand checks, one integration test target per file — a new stand check file for the served error page
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the CI script test holding the manifest's test targets — edits: the pinned target list
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script — edits: the stand-repeat leg's named targets
@task_file n=8 path=examples/seven_guis/tests/ mark=edits
what: the escher-session binary's served-task checks (session status served, host and mcp role reaching each served task)
@task_file n=9 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe — edits: the stand prompt's goals, the verdict's counts and required counts
@task_file n=10 path=scripts/cold-agent.ps1 mark=edits
what: the cold-agent pipe's windows entry
@task_file n=11 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's offline contract tests — edits: the prompt goal and verdict count pins
@task_file n=12 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the served-app count
@task_file n=13 path=README.md mark=edits
what: the README's 0.3.0 status — edits: the served-app count
@task_file n=14 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pins, the CI script test holding its section names and counts — edits: the served-app count pin
@task_item n=1 part=needs
The new served app's controls, each reached by an ordinary driver step (an acting verb on a control), cause a handler panic, a handler `Err` taken by the root error boundary, and a handler log, so the acting call's answer carries `caused` items naming the control's stable id, owner and event.
@task_item n=2 part=needs
The served-task table grows by exactly one entry; the new app is listed in the session status's served tasks and reached through the session host and the mcp role as the six other served tasks are.
@task_item n=3 part=needs
The new app's stand checks boot it headlessly through the seven_guis headless stand, run in both layout modes, on the bundled font with no skip, and are witnessed inside stand-repeat at the zero-failure budget.
@task_item n=4 part=needs
Indirect, through the engine's `caused` record class: engine caused items are asserted on a tracing build only, so the new checks of `caused` items stand where that build runs.
@task_item n=5 part=needs
Indirect, through the root error boundary: a root-boundary `Err` is checked on its own session, so the control returning an error is driven on a session of its own apart from the panic and log controls.
@task_item n=6 part=needs
Every authored panic message, `Err` text and log message and field the new app writes is the app's own fixed text, carrying no field of the scrub's hidden set, no host path, no secret and no user text, so what crosses in `caused` to the model provider is read in full; driver, stand and session log lines keep fixed words and counts only, with no ids or names.
@task_item n=7 part=needs
Indirect, through agent-run and cold-agent outputs: a handler panic's text never reaches agent-run output or cold-agent events, which stay content-free.
@task_item n=8 part=needs
The new app's snapshot stays within the snapshot text budget and its boot reads 0 lint findings, as the lean stand screens do.
@task_item n=9 part=needs
The six existing served apps' answers stay byte for byte as they are, held by their kept checks and the cross-OS readings of the served tasks by answer digest.
@task_item n=10 part=needs
No new driver verb, argument kind, run carrier, refusal cause, session error, exit code, snapshot node key, log field or span is added; the new app is reached through the existing verb table and acting verbs only.
@task_item n=11 part=needs
The stand prompt gains a goal asking the cold agent to meet the error page and make sense of what its controls cause, phrased once (the prompt goal control pins a phrase standing once), naming no element ids or app slugs.
@task_item n=12 part=needs
The cold-agent verdict gains a count of the calls whose answer carried `caused`, read from the live server child's kept checks or transcript as bools and counts only, never the text, and a field saying whether the run is required to meet it beside the pipe's other required counts; the run still passes only on positive evidence and an empty run is never a pass.
@task_item n=13 part=needs
The cold-agent pipe's contract tests stay offline under unittest and pin the new prompt goal and verdict count; the live cold-agent run stays off CI and is not fired by this chunk.
@task_item n=14 part=needs
The capability statement's served-app count and its pins move together, the claim section names and counts held to the schema, each claim naming its checks.
@task_item n=15 part=needs
Indirect, through the stand's task state: the new app's state is registered for the UI graph and the shared graph drives exercise its writes as they do every stand write.
@task_item n=16 part=needs
The new stand check file is a blitz-tests target of its own, joins the pinned target list and the stand-repeat leg's named targets, and keeps any leg it joins.
@task_standing n=1
convention: each new Rust integration test file is one test target, its shared helpers in shared modules that are no target
home: test-strategy
@task_standing n=2
convention: each new session or socket check owns its host behind a guard, sends child streams to null or drains them, and is unix-gated
home: test-strategy · 2-test-strategy
@task_standing n=3
convention: each new test is controlled red-before-green, pins its expected value as a literal, prints no argument value on failure, and table-driven tests assert their row count
home: unit-test-strategy
@task_standing n=4
convention: each new check uses no sleep or wall-clock read and holds no screen content in what it logs
home: integration-test-strategy
@task_standing n=5
convention: each new fixture is test data stated in code, the stand booting offline, nothing seeded
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=6
convention: each new Python script test runs under unittest, offline
home: 3-test-harness-contract
@task_item n=1 part=not_needed
WPT conformance runner rows (its CLI, environment, status set, outputs, pinned commit, Thai-font reftests): the chunk reaches no engine conformance suite.
@task_item n=2 part=not_needed
Engine paint and text rows (pixel tests on a CPU buffer, box-shadow expectations from Chromium, font-embolden witness, scrollbar fade, select list painting, password caret): the error page changes no painting or shaping.
@task_item n=3 part=not_needed
Layout engine test rows (incremental differential oracle, randomized template-swap stress, pure-restyle by hover, buffered render on the Dioxus bridge and its upstream tests): the chunk adds an app, not engine layout.
@task_item n=4 part=not_needed
Virtual clock and advance rows (advance moves and reports exact ms, timer driven by delivered ticks, timer step remainder): the new app holds no timer.
@task_item n=5 part=not_needed
Network stand-in and load rows (load page stand-in delay, counted fetching providers, file: URLs, failed-load settle): the new app loads nothing.
@task_item n=6 part=not_needed
Typed password rows (typed password never leaves or is returned, masked value): the new app has no password field.
@task_item n=7 part=not_needed
Mutation testing runs on the dev host (baseline, cargo-mutants under nextest) and the cognitive-complexity ceiling read on the host: no gate of this chunk's CI run.
@task_item n=8 part=not_needed
CI package install rows (apt-install script exit codes, bounded retried installs) and CI action pinning, cache saving, secrets and upstream-only publishing jobs: the chunk adds no CI job or install step.
@task_item n=9 part=not_needed
Session wire handshake rows (hello, protocol version, refused version, connection): the chunk changes no wire protocol.
