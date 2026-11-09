@task_prose part=session
The session added a third role to the escher-session command line: coverage <app> <check>... replays one or more kept check files and prints one JSON line listing the UI graph's handlers no check exercised.
The line's form is {"unexercised":[{"id","owner","event"}],"checks":n,"held":m}, owner left out when absent, hand-written, uncoloured; endings stay the four statuses (0 accepted, 1 refused malformed, 2 usage, 3 session error), none added.
In-process it adds escher_driver::handler_coverage with Coverage and HandlerRef, and in the DOM bridge a reached-handler record (ReachedHandler, record_reached, reached_handlers) written on the handle_event path.
It adds the stand_handler_coverage target (8 tests) and a stand_replay test replaying a kept actions step; run stand reads 262 passed, 0 failed, 7 ignored over 54 files.
docs/capabilities.md gains three limits: handlers inside a canvas are not counted, a handler on no screen a replay reached is not listed, a bubbling dispatch counts every ancestor listener it passed.
No verb, schema, MCP, socket, dependency or spec-master change; nothing retired.
@task_file n=1 path=packages/dioxus-native-dom/src/reached.rs
@task_file n=2 path=packages/escher-driver/src/coverage.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_handler_coverage.rs
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=5 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=7 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=8 path=packages/escher-driver/src/replay.rs
@task_file n=9 path=packages/escher-driver/src/lib.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/cli.rs
@task_file n=12 path=packages/escher-driver/src/cli_tests.rs
@task_file n=13 path=examples/seven_guis/src/session_host.rs
@task_file n=14 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=15 path=examples/seven_guis/tests/common/mod.rs
@task_file n=16 path=examples/seven_guis/tests/host_log.rs
@task_file n=17 path=tests/blitz-tests/tests/stand_replay.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=docs/capabilities.md
@task_item n=1 part=touches
The subjects list of design-system's items (the comment naming palette through web-spa-canvas) must gain a cli-coverage item for the new coverage role's output.
@task_item n=2 part=touches
The cli Component Patterns rule that escher-session, the driver's command line and its mcp role are agent-read surfaces now covers a third role, coverage; any text naming the role set as the command line plus mcp alone is stale.
@task_item n=3 part=touches
The cli-output rule (stdout holds only JSON, one line per command, never colour or an ESC byte, no terminal test) holds for the coverage role as well: one uncoloured JSON line on stdout.
@task_item n=4 part=touches
The exit-status classes 0 accepted, 1 refused, 2 usage, 3 session error are reused by the coverage role unchanged and none is added; a malformed check file is refused with status 1, a wrong argv shape is usage 2, a failing boot is 3.
@task_item n=5 part=touches
The command line's usage line gains the clause coverage role: coverage <app> <check>... before the apps clause; any cli item quoting the usage line or its role clauses is reached.
@task_item n=6 part=touches
The cli Navigation Pattern NOT YET MEASURED note (no slice recorded argument parsing or command structure for the CLI binaries) is reached: the session defines a closed argv shape for the coverage role, an app then one or more check paths, no --session and no --record word.
@task_item n=7 part=touches
The cli-graph item is reached indirectly: the coverage reader reads the UI graph at boot and after each replayed step (mounted left out, deduplicated by id and event) to find each handler a screen shows.
@task_item n=8 part=touches
The cli-actions item is reached indirectly: a kept check's actions steps now replay under test, and the coverage role replays kept checks of any step kind.
@task_item n=9 part=touches
The web-spa canvas surface is reached by a stated limit: handlers dispatched inside a canvas element's one listener are not counted by handler coverage, deferred to 0.4.0.
@task_item n=10 part=touches
Any count of escher-driver crate-root re-exports moves 59 to 62 (Coverage, HandlerRef, handler_coverage), and of dioxus-native-dom re-exports 35 to 36 (ReachedHandler).
@task_item n=11 part=touches
Any stand inventory or reading moves: stand files 53 to 54, run stand 244 passed over 52 files to 262 passed, 0 failed, 7 ignored over 54 files; stand_replay tests 3 to 4.
@task_item n=12 part=touches
Any CLI test inventory moves: seven_guis cli_commands 17 to 18 on linux, escher-driver cli_tests 5 to 6, json tests 7 to 8, the line table 102 to 109 rows, host_log 5 to 6.
@task_item n=13 part=touches
The coverage line holds ids, owner ids, event names and two counts only; a typed value never reaches it and a refusal holds nothing of the check file's path or bytes, which bears on any cli rule about what output carries.
@task_fact n=1 slug=surface
fact: The escher-session coverage role (coverage <app> <check>...) prints one uncoloured JSON line {"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}, owner left out when absent, listing the UI graph handlers no given kept check exercised; it ends 0 accepted, 1 malformed, 2 usage or 3 session error, adding no exit status.
@task_fact n=2 slug=surface
fact: The coverage role reads each named check file bounded at 1,048,576 + 1 bytes and refuses malformed, status 1, any file it cannot read, past the bound, not a check, or a check of another app, the refusal holding nothing of the path or the file.
@task_fact n=3 slug=surface
fact: Handler coverage has three stated limits: handlers inside a canvas are not counted, a handler on no screen a replay reached is not listed, and a bubbling dispatch counts every ancestor listener it passed with stopPropagation unread.
@task_item n=1 part=untouched
Color palette, semantic colors, the engine-painted select list colours and the absent colour token set: the session adds no colour and the CLI output stays uncoloured.
@task_item n=2 part=untouched
Typography, font loading and the fonts-unavailable screenshot refusal: no font, text or screenshot behaviour moved.
@task_item n=3 part=untouched
Spacing, depth strategy, hit-testing through paint clips, and border radius: no layout, paint or hit-test rule moved.
@task_item n=4 part=untouched
Motion, the VirtualClock, settle and the load wait: the coverage replay advances no clock beyond each replayed step's own behaviour, and no animation rule moved.
@task_item n=5 part=untouched
Iconography, cursors and favicons: no icon, cursor or favicon changed.
@task_item n=6 part=untouched
Surface desktop-native and mobile-native items (tokens, titlebar, custom widget, devtools overlay, window, urlbar, back button, capture, stack): no native window or mobile surface moved.
@task_item n=7 part=untouched
The cli-tokens item (owo-colors in the WPT runner) and the paint_bench and screenshot example outputs: unchanged.
@task_item n=8 part=untouched
The pending cli-unpaged-graph and cli-unpaged-actions items: the graph and actions reads stay unpaged in this session; the paging residual's ruled form is a later edit, not this session's change.
@task_item n=9 part=untouched
The mcp role's JSON-RPC 2.0 framing and its Err ending exit 3, and the fourteen verbs: no MCP surface or verb moved.
@task_item n=10 part=untouched
Brand identity, anti-patterns, self-validation protocol and the design decisions log: still no recorded intent; the session records none.
