@task_prose part=session
A codebase-health pass: our source files over 800 code lines and our functions over cognitive complexity 15 were split to come under the audit's bars, with no behaviour, output, verb, field, exit status or test name changed.
The escher-session command line's role reading and the MCP role's serving loop were each split into private helpers; the driver's key-name list moved whole into a private child module, re-exported under the same name.
The snapshot's unit tests moved out of line into their own test file, and the driver JSON output tests' fixture builders moved into a test-only child module.
Four stand checks (the UI graph, the actions line, accessibility names, the dioxus runtime probe) had their long test bodies split into helper assertions, each check's assertions and messages unchanged.
Measured after: files of ours over 800 code lines 10 to 7, functions of ours over cognitive 15 7 to 0; coverage escher-driver 97.92 percent, dioxus-native-dom 89.91 percent, blitz-test-harness 96.07 percent, each at or above its floor.
The founder ruled that upstream's oversize files, complex functions and duplicated test pairs stay as upstream's, and that escher's seven oversize integration-test targets stay as they are with no route entry.
@task_file n=1 path=packages/escher-driver/src/cli.rs
what: the escher-session command line reading — the role words (host, mcp, coverage), the task and the address arguments, and the usage error on a bad line
@task_file n=2 path=packages/escher-driver/src/mcp.rs
what: the escher-session mcp role — its serving loop over stdin and stdout, the tool results and the Err ending
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command verbs (click, press, type, advance and the rest) and their arguments
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: the named keys the press verb accepts, each key's name and the reading of a key from its name
@task_file n=5 path=packages/escher-driver/src/json_tests.rs
what: the unit tests of the driver's JSON output lines — snapshot, actions, coverage, layout, UI graph and session error answers
@task_file n=6 path=packages/escher-driver/src/json_tests/fixtures.rs
what: the test fixtures building sample answers for the driver's JSON output tests — findings, action nodes, a layout explanation, a UI graph
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot — the accessibility-tree read of the document (roles, names, values, checked, pressed, selected, enabled, masked password and file values, select options, clickable)
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: the unit tests of the snapshot — names, values, masking, disabled, aria states and the select's chosen option and options
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the dioxus runtime probe check — walks the runtime's roots, listeners and dynamic nodes
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand check of the UI graph line — every graph node a snapshot node, no reserved id form, each task's declarations agreeing with its code
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions line — each served screen lists its actionable nodes with the verbs table's verbs
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: the stand check of accessibility names — every interactive-role element and every region reads a name
@task_item n=1 part=touches
The escher-session command line rows (commands and their arguments, the usage error, the exit codes and statuses): the role and address reading was split into helpers; the accepted lines and every answer stay the same, so these rows should still hold.
@task_item n=2 part=touches
The escher-session mcp role rows (stdout, help, snapshot, screenshot and other tool results, the Err ending): the serving loop was split into an instance part and a session part; the results stay the same.
@task_item n=3 part=touches
The press verb of the driver verbs and the virtual clock: the named-key list moved into its own private module under the same public name; the key names and the verb stay the same.
@task_item n=4 part=touches
The snapshot and select accessibility rows (chosen option, highlighted row, open attribute, open select, a row of an open select list's bounds): the snapshot's unit tests that pin these moved out of line, unchanged in name and body.
@task_item n=5 part=touches
The actions line rows and the decision that actions are decided by the verbs' own checks: the stand check of the actions line was split into helper assertions with the same checks and messages.
@task_item n=6 part=touches
The UI graph line row: its stand check was split into helper assertions (no flag, readers agree, no reserved separator), same checks.
@task_item n=7 part=touches
The decision that escher-session is read by an agent, and the driver command line output read by agents: the output tests' fixtures moved into a test-only child module, the output lines unchanged.
@task_item n=8 part=touches
The escher-session tasks rows (counter, timer, crud, cells, flight-booker, order, load, unknown task): the stand checks driving those tasks were split into helpers, the tasks unchanged.
@task_item n=9 part=touches
The coverage line and coverage check files rows: per-crate coverage readings moved (escher-driver 97.77 to 97.92 percent, dioxus-native-dom 90.72 to 89.91 percent), each above its floor; a row holding a floor or a reading should be checked against those numbers.
@task_item n=10 part=touches
The decisions on masked password and file values and disabled reading as not enabled: their unit tests moved whole to the snapshot's out-of-line test file, the behaviour unchanged.
@task_item n=11 part=untouched
Colour palette, typography, spacing, depth strategy and border radius rows: no engine colour, font, spacing, stacking or paint rule moved.
@task_item n=12 part=untouched
Motion rows on animation, smooth and instant scroll, overlay scrollbar fade timings and the select list opening without motion: no timing or scroll behaviour changed.
@task_item n=13 part=untouched
Iconography rows (cursor icons, cursor resolution, favicon) and the shell provider rows (set cursor, maximize, minimize, window drag, decorations): untouched.
@task_item n=14 part=untouched
WPT runner output and result colour rows, and the web-spa surface: no runner or WASM page changed.
@task_item n=15 part=untouched
The select list as an engine overlay, its events (open, close, choose, highlight move) and its gestures: no select list behaviour changed; only tests over the snapshot's read of a select moved.
@task_item n=16 part=untouched
The JS window object rows, the dioxus-native configuration rows (base colour, font context, alpha mode) and the MountedData calls during a render: no configuration or script runtime surface changed.
