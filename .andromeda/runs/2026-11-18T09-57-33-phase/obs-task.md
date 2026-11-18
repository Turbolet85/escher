@task_prose part=chunk
Adds a seventh served stand app, a served error page, whose controls make a handler panic, make a handler return an error the root error boundary takes, and make a handler log, so an acting call's answer on that app carries `caused`.
Adds its stand checks: headless boot in both layout modes, each control's acting call answering the `caused` items it is built to cause, tied to the control's stable id, owner and event, and the app listed and reached through the host and the mcp role.
Adds a goal to the cold-agent pipe's stand prompt, asking the agent to meet the error page, and a count in the cold-agent verdict of the calls whose answer carried `caused`, kept as counts and bools only, plus whether the run must meet it.
Moves the pipe's contract tests and the capability statement pins where the served-app count or the verdict's fields are held. Runs no live agent and reaches no model provider.
@task_file n=1 path=examples/seven_guis/src/tasks/error_page.rs mark=edits
what: the new served stand app, the error page, whose controls panic, return an error and log (new file)
@task_file n=2 path=examples/seven_guis/src/tasks/mod.rs mark=edits
what: the stand's task modules — edits: the module list, which gains the new app
@task_file n=3 path=examples/seven_guis/src/stand.rs mark=edits
what: the headless stand boot of the served apps — edits: the new app's headless boot
@task_file n=4 path=examples/seven_guis/src/session_host.rs mark=edits
what: escher-session's served-task table and host — edits: the served-task table, TASKS, which gains one entry
@task_file n=5 path=examples/seven_guis/src/app.rs mark=edits
what: the windowed stand in seven_guis_native — edits: the stand's app list, if the new app joins it
@task_file n=6 path=tests/blitz-tests/tests/stand_error_page.rs mark=edits
what: the new app's stand checks: headless boot in both layout modes and each control's `caused` items (new file)
@task_file n=7 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — edits: the pinned list of test targets
@task_file n=8 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner — edits: the stand-repeat leg's named targets
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: the escher-session binary's served-task checks — edits: the served-task list and the usage line's apps count
@task_file n=10 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe (POSIX) — edits: the stand prompt's goal, the verdict's `caused` count and its required reading
@task_file n=11 path=scripts/cold-agent.ps1 mark=edits
what: the cold-agent pipe (Windows) — edits: the stand prompt's goal, the verdict's `caused` count and its required reading
@task_file n=12 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests — edits: the checks on the prompt goal and the verdict's fields
@task_file n=13 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the served-app count
@task_file n=14 path=README.md mark=edits
what: the project README's 0.3.0 status — edits: the served-app count
@task_file n=15 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pins — edits: the pinned served-app count
@task_item n=1 part=needs
The actions answer's cause content on the new app: each control's acting call answers the `caused` items it is built to cause (a handler panic, an error taken by the root error boundary, a handler log), tied to the control's stable id, owner and event.
@task_item n=2 part=needs
The served-task table grows by exactly one entry; the new app is listed and reached through the escher-session host and mcp role as the six other served tasks are, and adds no driver verb, argument, carrier, cause, session error, exit code, node key, log field or span.
@task_item n=3 part=needs
Indirect, through escher-telemetry's scrub set: every panic message, error text and log message and field the new app writes carries no field named in the hidden set (url, href, src, html, text, value, attrs, path, request, error, password, panic.payload), no host path, no secret and no user text, so what crosses in `caused` is the app's own fixed text, read in full.
@task_item n=4 part=needs
Indirect, through the fixed-diagnostics and fixed-message rules: the new app's log sites and panic and error texts are fixed strings, echoing nothing the caller typed.
@task_item n=5 part=needs
The widening on the founder's F5 option B: a new served app and its authored error text crossing to the model provider in the live run, and nothing past that; the security-plan's cold-agent crossings gain exactly that.
@task_item n=6 part=needs
The new app's handler panic, error and log reach the answer only as `caused` items; the answer-bearing types and a part's answer are never logged, and the driver's command and tool_call spans gain no field and carry no content from the error text.
@task_item n=7 part=needs
The headless stand boots the new app through seven_guis::stand with no telemetry sink installed, in both layout modes.
@task_item n=8 part=needs
The new app's snapshot stays under SNAPSHOT_TEXT_BUDGET and its boot reads 0 lint findings, as the lean stand screens do.
@task_item n=9 part=needs
The six served apps' answers stay byte for byte as they are, held by their kept checks and the cross-OS stand readings.
@task_item n=10 part=needs
The cold-agent stand prompt gains one goal asking the agent to meet the error page and make sense of what its controls cause, phrased once.
@task_item n=11 part=needs
The cold-agent verdict gains a count of the calls whose answer carried `caused`, read from the live server child's kept checks or transcript as bools and counts only, never the text; its fields stay a closed set and hold no content.
@task_item n=12 part=needs
The verdict says, beside the pipe's other required counts, whether the run is required to meet the `caused` count, and where it is required a run that misses it is named among the verdict's reasons in their fixed order.
@task_item n=13 part=needs
Indirect, through the cold-agent pipe's reading rules: the server log, kept checks and transcript are read only to count, never printed or copied, and the pipe's events carry no user content and no `caused` text.
@task_item n=14 part=needs
The capability statement and README served-app counts are updated by hand, and their pins and the pipe's contract tests move with the new served-app count and the verdict's new field.
@task_item n=15 part=needs
The chunk runs no live agent and reaches no model provider; the live run that proves v030-15 is a later chunk's.
@task_item n=16 part=needs
The P5 card says, for each control of the new app, what an agent reads in `caused`.
@task_standing n=1
convention: each new test check is silent on success and shares its helpers through the common modules
home: 3-observability-harness-contract
@task_standing n=2
convention: each new check that asserts an empty stderr unsets RUST_LOG for the child it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: each new log or output check that fails names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_standing n=4
convention: each new JSON line the harness writes is encoded by python's json module
home: 3-observability-harness-contract
@task_standing n=5
convention: each new committed kept check or fixture holds no secret and no private data
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Metric emission, metrics exporters and telemetry backends: the chunk emits no counter, gauge or histogram and adds no exporter or SDK.
@task_item n=2 part=not_needed
WPT scores, WPT report messages and their upstream publishing: the chunk runs no WPT suite.
@task_item n=3 part=not_needed
The page-script error surface (the window error event and its fields, window.onerror, script error source labels, the embedder drain of script diagnostics): the new app's errors come from its own handlers, not from page script.
@task_item n=4 part=not_needed
Select, optgroup and multiple-select rows (chosen sets, list row labels, open list shots, size attribute): the new app's controls are not select lists.
@task_item n=5 part=not_needed
The package-install diagnostic line: the chunk changes no CI package installation.
@task_item n=6 part=not_needed
Stylesheet loading and file: loads from non-file documents: the new app loads no external resource.
@task_item n=7 part=not_needed
The WASM console panic hook: the served app runs in the native escher-session binary, not a WASM build.
@task_item n=8 part=not_needed
The agent-run harness events and its raw log: the chunk changes the cold-agent pipe, not the agent-run harness.
@task_item n=9 part=not_needed
CI artifact uploads (per-leg logs on failure, the coverage report on success): the chunk adds no artifact.
