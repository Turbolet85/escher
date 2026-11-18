@task_prose part=chunk
Adds a seventh served stand app, served by escher-session beside the six, whose controls by ordinary driver steps make a handler panic, return an error the root error boundary takes, and log, so an acting call's answer carries caused.
Adds its stand checks: headless boot in both layout modes, each control's caused items tied to its stable id, owner and event, the app reached through the host and the mcp role.
Adds a goal to the cold-agent stand prompt and a caused count to the cold-agent verdict, read as bools and counts only, with whether the run is required to meet it.
Widens the cold-agent crossing on the founder's F5: a new served app, and its authored error text crossing to the model provider in the live run (which this chunk does not fire).
Moves the pipe's contract tests and the capability statement pins where the served-app count or the verdict's fields are held.
@task_file n=1 path=examples/seven_guis/src/tasks/mod.rs mark=edits
what: the stand's task apps, where the new served error-page app joins the six — edits: the new app's module entry
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot of each task app — edits: the new app's headless boot
@task_file n=3 path=examples/seven_guis/src/session_host.rs mark=edits
what: the session host and its served-task table — edits: the served-task table, one entry added
@task_file n=4 path=examples/seven_guis/src/app.rs mark=edits
what: the windowed stand, if the new app joins it
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — edits: the named stand-check targets
@task_file n=6 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner — edits: the stand-repeat leg's named targets
@task_file n=7 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe, its fixed stand prompt and its verdict — edits: the stand prompt's goal, the verdict's caused count and its requirement
@task_file n=8 path=scripts/cold-agent.ps1 mark=edits
what: the cold-agent pipe on Windows — edits: the stand prompt's goal, the verdict's caused count
@task_file n=9 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests — edits: the served-app count and the verdict's fields
@task_file n=10 path=docs/capabilities.md mark=edits
what: the committed capability statement, its MCP crossing — edits: the served-app counts
@task_file n=11 path=README.md mark=edits
what: the project's status page — edits: the served-app counts
@task_file n=12 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pins — edits: the served-app counts
@task_item n=1 part=needs
The caused text crossing: a handler's panic, its error and its log reach an acting call's answer as caused items; the new app is built so that each control's acting call carries them.
@task_item n=2 part=needs
The root-boundary Err crossing: a handler's returned error is taken by the root error boundary and what crosses from it is the app's authored text.
@task_item n=3 part=needs
The boundary widening rule: the new served app and its authored error text crossing to the model provider are a widening, covered by the founder's F5 option and nothing past it; it supersedes the earlier ruling that caused stays out of the run's reach.
@task_item n=4 part=needs
The cold-agent screen-to-provider crossing: in the live run what the served apps answer, now caused text included, goes to the model provider.
@task_item n=5 part=needs
Authored error text — every panic message, error text, log message and log field the app writes — carries nothing the log sink's scrub would redact: no content-named field of the hidden set, no host path, no secret, no user text, no call-supplied input, so caused crosses the app's own fixed text in full.
@task_item n=6 part=needs
The new app's log sites go through escher's allowlist telemetry sink and its escher_ target content scrub; the chunk adds no log field or span of its own.
@task_item n=7 part=needs
Closed sets stay closed: no new driver verb, argument kind, carrier, refusal cause, session error, exit code, node key, log field or span; only the served-task table grows by one entry.
@task_item n=8 part=needs
The served-task set reached through the escher-session serve and mcp roles, the session socket and the MCP results, as the other served tasks are, including the MCP result and session reply bounds.
@task_item n=9 part=needs
Caused items tied to the control's stable element id, owner and event, as the actions answer and the signal registry's declared writes name them, with no NodeId crossing.
@task_item n=10 part=needs
A handler panic in the served app is met as a caused item on the answer, not as a crash of the session host or the MCP server.
@task_item n=11 part=needs
The cold-agent fixed prompt: the new goal is fixed program text, phrased once, holding no caller-supplied or page text.
@task_item n=12 part=needs
The verdict's caused count is read from the live server child's kept checks or transcript as bools and counts only, never the caused text; checks, recordings and server output stay unprinted and owner-only.
@task_item n=13 part=needs
The cold-agent verdict's required counts and wrong-call bar: whether the run must meet the caused count sits beside the pipe's other required counts.
@task_item n=14 part=needs
The new app's snapshot stays under the snapshot text budget and its boot reads 0 lint findings; its control states cross as closed values in the snapshot, graph and actions answers.
@task_item n=15 part=needs
The committed statement of the MCP crossing names the seventh served app and the caused text crossing to the provider.
@task_item n=16 part=needs
This chunk runs no live agent and reaches no model provider; its stand checks and pipe tests run headless and offline.
@task_standing n=1
convention: each new log line on an escher_ target carries only allowlisted, content-free fields and reaches only the stderr sink
home: logging-monitoring
@task_standing n=2
convention: each new kept check or recording the new stand checks write is owner-only, uncommitted and read for counts only
home: data-protection
@task_standing n=3
convention: each new stand check reaches the app only through the closed driver verb schema and its single validation gate
home: input-validation
@task_item n=1 part=not_needed
Authentication and authorization rows (CI tokens, publish guards and secrets, the cookie store): the chunk adds no credential, token or publish path.
@task_item n=2 part=not_needed
Outbound network rows: HTTP through blitz-net or reqwest, fetch method limits, CORS and same-origin absences, request timeouts and response size limits, per-host concurrency: the new app loads no remote resource.
@task_item n=3 part=not_needed
Browser app storage rows: history in sqlite, the HTTP cache, cookies by default, clipboard, native file dialog, TLS: the stand app stores nothing and opens no dialog.
@task_item n=4 part=not_needed
Remote content and script loading rows: iframe and import nesting limits, stale iframe responses, script fetchers, JavaScript opt-in, file URL rules, external navigation schemes: the served app is a native Dioxus stand screen with no page loading.
@task_item n=5 part=not_needed
WPT runner and workflow rows (post-results trust, PR lookup, quarantine entries, script type detection, its results): the chunk touches no web-platform test.
@task_item n=6 part=not_needed
Coverage role and coverage floor rows (coverage answer line, per-crate floors, the floor checker's exit codes): the chunk changes no coverage reading.
@task_item n=7 part=not_needed
Release and signing rows (release publishing upstream only, signing material removal, signed artifacts): the chunk publishes nothing.
