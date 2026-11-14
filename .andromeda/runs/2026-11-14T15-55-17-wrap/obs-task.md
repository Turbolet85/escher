@task_prose part=session
The session added auto-exploration to escher-driver: one in-process library call, escher_driver::explore, explores the available actions breadth-first within a step budget (StepBudget, 1 to MAX_EXPLORE_STEPS 10,000), each path on a fresh session, every call through Session::run, the last call under a panic catch.
A path ends in a closed ending (accepted with settled, refused with its cause, panicked, diverged at a step); the report holds closed words, indices, calls and bools only, no panic payload, node id, screen text or digest, and is never logged or printed.
The verbs explored are the action table's target verbs whose result holds no image field, read by the field's kind (today click, type, scroll; screenshot left out); exploration opens no span and emits no event of its own, each driven call leaves its own command span.
Explored paths can be kept as replayable checks into a recording directory, refused as record-dir-not-private before anything boots when the directory is open to group or others; a path deeper than a kept check's 400 records ends its recording and that check is removed.
The kept check's opening record gained a third key, format, written 1 and read strictly (absent read as 1, any other value refused malformed holding nothing of the bytes); the two committed checks were re-recorded and differ by that key alone.
Measured: an unbounded app (the Counter) never exhausts the frontier; exhaustion is witnessed on a finite refusing-control page. No verb, role, tool, cause, exit, env var, span, span field, log site, metric or dependency was added; nothing retired.
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the driver's auto-exploration component, the in-process explore call with its step budget, closed path endings and panic catch, inside the driver library that emits no event and installs no subscriber
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: the unit tests of the exploration's step budget bound and of the verbs explored (the target verbs with no image field)
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: escher-driver's public surface, the driver library's re-exports (explore, StepBudget, Exploration, ExploredPath, PathEnding, CHECK_FORMAT)
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the actions answer (actions) and the driver's verb table lookups, its target verbs and the call naming an action's node
@task_file n=5 path=packages/escher-driver/src/recording.rs
what: the session recording, the kept check file's writer and reader (the recording file, check-{n}.json, its opening record and answer digest)
@task_file n=6 path=packages/escher-driver/src/recording_tests.rs
what: the unit tests of the kept check file, its opening record's format and the malformed refusal
@task_file n=7 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand test target of auto-exploration over the served tasks, both layout modes, kept checks replayed green and the recording directory refusal
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: a committed kept check of the Counter recorded through the escher-session command line
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed kept check of the Counter recorded through the escher-session mcp role
@task_file n=10 path=docs/capabilities.md
what: the capability statement, updated by hand, with its counts and limits
@task_item n=1 part=touches
The kept check file contract (accepted, answer, answer digest, arguments) gains the opening record's key format: the opening record now holds three keys, tool, arguments and format, format written as the whole number 1.
@task_item n=2 part=touches
The recording file's check-{n}.json contract: each check opens with the record carrying format 1; a check without format reads as format 1; any other value (2, 1.0, "1", -1, 1.5, 0, null, true) refuses the whole check.
@task_item n=3 part=touches
The refusal cause malformed gains a new trigger, a kept check whose format is not the whole number 1, and the refusal holds nothing of the check's bytes.
@task_item n=4 part=touches
Committed kept checks hold no secrets and no private data: the two committed Counter checks were re-recorded by the binary and differ only by the format key; no scrub, redaction or violation shape moved.
@task_item n=5 part=touches
Recording directory private and the record-dir-not-private refusal: the exploration's record directory is checked first and refused with SessionError RecordDirNotPrivate before any boot, nothing written.
@task_item n=6 part=touches
Recording bounds: a kept check's 400-record bound now also ends an explored path's recording deeper than it, and that path's check is removed; the bound's value itself unchanged.
@task_item n=7 part=touches
One span per driver command and no call content in the command span: indirect, exploration runs every call through Session::run, so each driven call leaves its own command span and the exploration adds none.
@task_item n=8 part=touches
Driver emits no event and installs no subscriber: indirect, the new explore component inside the driver library has no tracing, print, log site or env read.
@task_item n=9 part=touches
Hosted call runs through the session's run alone: exploration is a second in-process caller that drives every call, re-reach steps included, through Session::run.
@task_item n=10 part=touches
escher-telemetry panic event and panic hook with previous hook: a panic in an explored call is caught by the exploration and recorded as a Panicked ending, while the process's panic hook still runs for it.
@task_item n=11 part=touches
Answer-bearing types never logged and session logs carry no page content: the new exploration report types hold no screen text or payload and are never logged or printed.
@task_item n=12 part=touches
Actions read changes nothing in the document: exploration reads the actions answer on each newly reached screen to expand its children, a new reader of that rule.
@task_item n=13 part=touches
Capability statement updated by hand: an exploration section, its in-process-only absence and the 400-record limit were added; its five counts (verbs fourteen, causes eight, MCP tools fourteen) unchanged.
@task_item n=14 part=touches
Cold-agent kept checks held at rest only and stay local: kept checks of explored paths share the same owner-only medium and are written only into the caller's directory.
@task_fact n=1 slug=span-trace-coverage
fact: Auto-exploration (escher_driver::explore) opens no span and emits no event of its own; each call it drives leaves its own command span, re-reach steps included.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The exploration report (Exploration, ExploredPath, PathEnding) holds closed endings, indices, calls and bools only, no panic payload, node id, screen text or screen digest, and is never logged or printed.
@task_fact n=3 slug=7-error-capture-reporting
fact: A panic in an explored call is caught by the exploration and reported as the closed ending Panicked holding no payload; the process's panic hook still runs, and the session it struck is dropped under a second catch.
@task_fact n=4 slug=3-observability-harness-contract
fact: The kept check's opening record carries format, the whole number 1 (CHECK_FORMAT); absent read as 1, any other value refuses the check malformed holding nothing of its bytes.
@task_fact n=5 slug=ci-integration
fact: An exploration given a recording directory keeps one check per explored path there, refused record-dir-not-private before any boot when the directory is open to group or others.
@task_item n=1 part=untouched
Metric coverage and the metrics absences: no counter, gauge, histogram or exporter was added.
@task_item n=2 part=untouched
The telemetry backend and OTel export absences and the sink's resource keys: no backend, SDK or resource key moved.
@task_item n=3 part=untouched
escher's sink log line, its scrub set, target allowlist, RUST_LOG handling and closed span line format: the sink was not changed.
@task_item n=4 part=untouched
The MCP tool_call span, its tool and ending fields, the escher-session roles, commands, exit codes, host wire and error object: no tool, role, argument, exit or wire version was added.
@task_item n=5 part=untouched
The agent-run harness and cold-agent pipe events, verdict fields, counted verbs and reasons: no harness script or verdict shape changed.
@task_item n=6 part=untouched
The script error surface, window error event, window.onerror, WPT report and engine tracing feature gating: the engine and upstream crates were not reached.
@task_item n=7 part=untouched
CI workflows, CI log and coverage artifacts and the package-install diagnostic line: no CI step, leg or artifact changed.
@task_item n=8 part=untouched
Select, optgroup, open list, snapshot node and form entry privacy rows and the state registry: no snapshot, select or registry behaviour moved.
