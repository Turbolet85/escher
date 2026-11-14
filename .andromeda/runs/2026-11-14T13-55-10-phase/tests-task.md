@task_prose part=chunk
Adds one exploration command over a held app: from boot it reads the actions available on the screen and drives them breadth-first, each path re-reached from a fresh session, the frontier ordered by path length, until a step budget is spent, then returns a closed report.
Each step is a validated call through the driver session's one run path; a refusal is caught and reported by its closed cause, a panic inside a step is caught and reported as a closed word with its path kept, and neither stops the run.
Each explored path is left as an owner-only recorded check with no screenshot step that replays green; the recorded-check step format gains a stated format version so a later two-target step is an addition.
Exploration reads the default seed alone and does not follow navigation (provisional); it is proven on the stand's served tasks in both layout modes and on a panicking-handler fixture and a refusing-control fixture.
@task_item n=1 part=needs
The new exploration command over a held driver session: breadth-first over available actions from boot, each path re-reached from a fresh session started anew, frontier ordered by path length, stopping when the step budget is spent, returning a closed report.
@task_item n=2 part=needs
The actions offered come from the driver's own readers alone: the actions verb's answer on a node and the schema's verb table, never a closed verb list in exploration's code, so a later verb is driven with no reshaping.
@task_item n=3 part=needs
Every explored step is a validated call through the driver session's run, as the start-in-a-state reach path does: no second executor, the run's validation and refusal order applying to each step.
@task_item n=4 part=needs
A refused step is caught and reported by its closed driver refusal cause (the fixed cause set and its printed form) and exploration carries on; a refusal changes nothing in the instance and echoes no input.
@task_item n=5 part=needs
A panic raised inside a step is caught and reported as a closed fixed word, the path that led to it kept, and exploration carries on; no panic text reaches any report or log.
@task_item n=6 part=needs
Each explored path is kept as a recorded check in the session recording's check-file format, readable by the check reader and replaying green through check replay on the ordinary call path.
@task_item n=7 part=needs
Kept checks are owner-only: a 0600 file in a 0700 launcher-named directory, and the same refusal of a group- or other-open recording directory as the session recording gives.
@task_item n=8 part=needs
No screenshot step in any explored path and no pixel at rest: exploration holds no shot.
@task_item n=9 part=needs
The recorded-check step format is versioned: a kept check states its format version, and the format can carry a verb with more than one target as an addition, not a redefinition; existing checks still read and replay.
@task_item n=10 part=needs
Exploration reads the default seed alone, as every driver session does: no seed argument on the command, and the command's arguments are shaped so a later seed argument is an addition.
@task_item n=11 part=needs
Navigation is not followed (provisional, the founder's fork): an action that would navigate the document, a link or a form submission to another URL, is not driven or its step not taken, and nothing new is loaded.
@task_item n=12 part=needs
Carriers held to the most conservative provisional reading: the command reaches no new carrier, adding no verb to the schema's verb table, no MCP tool, no socket verb and no binary role.
@task_item n=13 part=needs
Untrusted input to the command, the step budget and the kept-check directory, is refused as typed values with no panic path.
@task_item n=14 part=needs
No new log field carrying content: a span, if exploration adds one, holds fixed words and counts only, nothing of the screen, ids, names or typed text.
@task_item n=15 part=needs
Proven on the seven_guis stand's served tasks booted through the stand's boot functions, in both layout modes, on the bundled font with no skip.
@task_item n=16 part=needs
Two new fixtures: an app whose handler panics and one holding a refusing control (disabled, covered or off-screen), stated in code beside the shared fixtures, booted offline.
@task_item n=17 part=needs
Steps run on the virtual clock: no sleep or wall-clock read in exploration or its checks; a path re-reached from a fresh session reads the same as the path driven once.
@task_item n=18 part=needs
Driving the type verb on a password field never returns or records the typed password; a kept check holds no private data.
@task_standing n=1
convention: each new behaviour gets its own integration test target, one target per test file, shared helpers in a module that is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new stand check runs in both layout modes and on the bundled font, never skipping
home: test-scope-summary
@task_standing n=3
convention: each new fixture is stated in code in the one shared fixtures module, nothing seeded
home: 3-test-harness-contract
@task_standing n=4
convention: each new test is controlled red-before-green, a table-driven test asserts its row count, and a failure prints no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=5
convention: each new check holds no sleep or wall-clock read and owns the host it starts
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=6
convention: each new test file joining a CI leg keeps its legs, and stand checks repeat three rounds under a zero-failure budget
home: ci-integration · 9-ci-integration
@task_standing n=7
convention: each new unit sits under its crate's line coverage floor
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_standing n=8
convention: each new recording or socket check is unix-gated
home: session-lifecycle
@task_item n=1 part=not_needed
WPT conformance rows: the runner, its CLI, statuses, outputs, pinned commit and reftest fonts; exploration touches no CSS conformance.
@task_item n=2 part=not_needed
Cold-agent pipe rows: its events, verdict, exit codes, invocation, isolation and wrong-call bar; exploration is no cold-agent run.
@task_item n=3 part=not_needed
Agent-run five-command rows: its verbs, events, exit codes, selections and state files.
@task_item n=4 part=not_needed
Harness text contrast and lint defect rows: the contrast formula, required ratio and lint findings are not read by exploration.
@task_item n=5 part=not_needed
Pixel and screenshot rendering rows: CPU-buffer pixel tests, PNG determinism and decoding, frames compared whole; exploration takes no shot.
@task_item n=6 part=not_needed
Handler coverage rows: the coverage line, its limits on bubbling, canvas and unreached screens; exploration measures no handler coverage.
@task_item n=7 part=not_needed
Engine rendering regression rows: select list painting and colours, scrollbar fade and thumbs, box-shadow, font embolden, password caret, open pseudo-class, buffered render on the Dioxus bridge.
@task_item n=8 part=not_needed
CI supply rows: action pinning to SHAs, no secrets, read-only permissions, package install retries and their exit codes, cache save branches, upstream-only publishing jobs.
@task_item n=9 part=not_needed
Canvas virtual node and two-target verb production rows: drag and canvas nodes are out of scope; only the step format's room for them is built.
@task_item n=10 part=not_needed
UI graph rows: task state registration for the graph, the reserved graph id form and the graph answer; exploration reads actions, not the graph.
