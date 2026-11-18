@task_prose part=chunk
Adds a seventh served stand app, the served error page, beside counter, flight-booker, timer, crud, cells and load, whose controls, each by an ordinary driver step, make a handler panic, make a handler return an error the root error boundary takes, and make a handler log, so an acting call's answer on that app carries `caused`.
Adds its stand checks: the app booted headlessly in both layout modes, each control's acting call answering the `caused` items it is built to cause, the app listed and reached through the host and the mcp role as the other served tasks are.
Adds a goal to the cold-agent pipe's stand prompt and a count to its verdict of the calls whose answer carried `caused`, as bools and counts only, and moves the pipe's contract tests and the capability statement's served-app counts to match.
Runs no live agent and reaches no model provider; which controls, how many, which levels and which acting verbs reach them are forks for the operator.
@task_file n=1 path=examples/seven_guis/src/tasks/ mark=edits
what: the stand's task screens, the seven_guis apps — edits: a new served error page app
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot of its apps — edits: the new app's headless boot
@task_file n=3 path=examples/seven_guis/src/session_host.rs mark=edits
what: the session host's served-task table of the escher-session binary — edits: the served-task table, one entry added
@task_file n=4 path=examples/seven_guis/src/app.rs mark=edits
what: the windowed stand — edits: the new app's place in it, if it joins
@task_file n=5 path=tests/blitz-tests/tests/ mark=edits
what: the stand checks of the blitz-tests target — edits: the new app's stand checks
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — edits: the pinned target list
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI legs — edits: the stand-repeat leg's named targets
@task_file n=8 path=examples/seven_guis/tests/ mark=edits
what: the binary's served-task checks through the host and the mcp role — edits: the new served task's checks
@task_file n=9 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe — edits: the stand prompt's goal, the verdict's count
@task_file n=10 path=scripts/cold-agent.ps1 mark=edits
what: the cold-agent pipe on Windows — edits: the stand prompt's goal, the verdict's count
@task_file n=11 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests — edits: the served-app count and the verdict's fields
@task_file n=12 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the served-app counts
@task_file n=13 path=README.md mark=edits
what: the project README's 0.3.0 status — edits: the served-app counts
@task_file n=14 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pins — edits: the served-app counts
@task_item n=1 part=needs
The new app's controls: each one an ordinary driver step on an actionable, named control whose handler panics, returns an error the root error boundary takes, or logs.
@task_item n=2 part=needs
The actions answer and an acting call's answer on the new app: every `caused` item tied to the control's stable id, owner and event, with no new cause, verb, argument, carrier, session error, exit, node key, log field or span.
@task_item n=3 part=needs
The new app's interactive nodes each carry an accessible name and are read as actionable by the one actionable rule, so the snapshot, the UI graph and the actions listing show them the way they show the six apps' controls.
@task_item n=4 part=needs
The new app's screen at boot reads 0 lint findings and its text and controls meet the stand's SC 1.4.3 contrast thresholds, as the lean stand screens do; its snapshot stays under SNAPSHOT_TEXT_BUDGET.
@task_item n=5 part=needs
The new app's expected roles, names and focus order stated once in the stand's shared expectation tables, read by its headless boot in both layout modes.
@task_item n=6 part=needs
The authored error text, every panic message, Err text and log message and field the app writes, carries no field of the scrub's hidden set (url, href, src, html, text, value, attrs, path, request, error, password, panic.payload), no host path, no secret and no user text, so the log sink's allowlist and redaction for escher's own targets leave it whole and it crosses in `caused` in full.
@task_item n=7 part=needs
The served-task table grows by one entry; the six served apps' answers stay byte for byte as they are, held by their kept checks and the stand's cross-OS readings.
@task_item n=8 part=needs
The new stand checks join the stand-repeat leg's three rounds by name and the blitz-tests target pin.
@task_item n=9 part=needs
The stand binaries still build no platform adapter and the stand stays at accessibility tier 0 with the new app on it; the capability statement's reach is edited by hand where its served-app counts move.
@task_item n=10 part=needs
The stand prompt's new goal, to meet the error page and make sense of what its controls cause, phrased once; the verdict's new count read from the live server child's kept checks or transcript as bools and counts only, never the text.
@task_item n=11 part=needs
The P5 card says, per control, what an agent reads in `caused`.
@task_standing n=1
convention: Each new accessibility assertion is a Rust test over the AccessKit tree, booted through the stand.
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: Each new stand check is a named target of the stand-repeat leg, repeated three rounds, and gates CI.
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: A new stand screen's expected roles and names are stated once, in the stand's expectation tables, not repeated per test.
home: screen-reader-support · 7-screen-reader-support
@task_item n=1 part=not_needed
Range input rows, its keys, pointerdown, click and value: the new app's controls panic, return an error or log and are no range input.
@task_item n=2 part=not_needed
Single-choice and multiple select rows, open and closed lists, optgroups, typeahead and their colours, row heights and motion: the error page opens no select list.
@task_item n=3 part=not_needed
Cells grid and CRUD list keyboard contracts: those apps stay as they are.
@task_item n=4 part=not_needed
Password value masking, caret and reveal rules: the new app has no password field.
@task_item n=5 part=not_needed
Driver screenshot contracts, element and row shots: the chunk adds no screenshot behaviour.
@task_item n=6 part=not_needed
Scrollbar thumb contrast, scrollbar drag, hidpi or zoom reshaping and scheme-change rows: the chunk changes no engine painting or viewport behaviour.
@task_item n=7 part=not_needed
Engine ARIA property reads and role mapping rules (aria-describedby, aria-level, aria-expanded, HTML-AAM): the chunk changes no engine reading of the tree, it only adds a screen read by them.
@task_item n=8 part=not_needed
Invalid date state rules: the new app carries no date field.
