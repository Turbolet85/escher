@task_prose part=session
The session added handler coverage: the escher-session binary gains a third command-line role, coverage <app> <check>..., over a new escher_driver::handler_coverage reader that replays kept checks and lists the UI graph handlers no check exercised, as one JSON line.
dioxus-native-dom gains an ungated reached record: record_reached and reached_handlers on DioxusDocument and a pub ReachedHandler, written in DioxusEventHandler::handle_event before runtime.handle_event, beside the unchanged SignalRegistry arm, via a new crate-private ungated reader element_listener_ids (element_facts stays accessibility-gated).
A new stand target stand_handler_coverage (8 tests, both layout modes) and a new stand_replay test (a kept actions step replays) were added; run stand reads 262 passed, 0 failed, 7 ignored over 54 files.
Counts moved: stand_* files 53 to 54, blitz-tests mod common declarers 23 to 24, mod session_common declarers 34 to 35, stand_replay tests 3 to 4, cli_commands tests 17 to 18 on linux, host_log 5 to 6, escher-driver re-exports 59 to 62, dioxus-native-dom re-exports 35 to 36.
Nothing retired; no verb, snapshot field, tree builder read, ARIA mapping or log field moved; the a11y CI leg and its fourteen files are untouched and green.
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
The bridge's handle_event path: a text stating the bridge sets the signal registry's dispatch around runtime.handle_event holds as written, and now gains the reached record written beside it, before runtime.handle_event, for each chain node whose current listeners hold the event name, every document paying one has_context lookup per dispatch.
@task_item n=2 part=touches
The vdom walk of current listeners: a new ungated crate-private reader element_listener_ids gives each element's stable id and current listener events, beside the accessibility-gated element_facts; any text naming the vdom walk's readers or saying the listener read is accessibility-gated can be reached.
@task_item n=3 part=touches
The read-only UI graph (DioxusDocument::graph): it gains a second reader, handler_coverage, which reads it at boot and after each replayed step, mounted left out and deduplicated by id and event; a text listing the graph's readers or consumers is reached.
@task_item n=4 part=touches
The actionable rule's listener leg (current listeners holding an event other than mounted): the coverage reader applies the same mounted exclusion over graph nodes; a text stating who reads that rule may need the new reader beside unkeyed_actionable and actionable_nodes.
@task_item n=5 part=touches
Keyboard activation: a stand check now asserts a keyboard activation of a control counts its click handler as exercised and a refused step exercises nothing, in both layout modes; a text listing what the stand asserts about Enter and Space dispatching a Click is reached.
@task_item n=6 part=touches
Stand target counts: stand_* files 53 to 54 and the run stand reading 244 passed over 52 files to 262 passed, 0 failed, 7 ignored over 54 files; any a11y-plan figure stating the stand file count or reading is moved.
@task_item n=7 part=touches
Shared-module declarer counts: blitz-tests mod common declarers 23 to 24 and mod session_common declarers 34 to 35; a text stating either figure, or the roles and names tables of the shared module and how many checks read them, is moved.
@task_item n=8 part=touches
The stand-repeat leg picks up the new stand_handler_coverage target by its stand_ prefix; a text describing what the stand-repeat leg runs by prefix is reached, while the named eight a11y files it repeats are unchanged.
@task_item n=9 part=touches
Owned limits: the CARRY on the entry Handler coverage for the owed actions replay test is discharged (stand_replay's new test passes); other limits' CARRYs naming an owner generically stand unchanged.
@task_item n=10 part=touches
The escher-session binary's role set moves from serve and mcp to serve, mcp and coverage, with the usage line gaining a coverage role clause; a text naming the binary's roles or usage line is reached.
@task_item n=11 part=touches
Test inventory counts: stand_replay 3 to 4, seven_guis cli_commands 17 to 18 on linux, host_log 5 to 6, escher-driver cli_tests 5 to 6 and json 7 to 8, the cli line table 102 to 109 rows, dioxus-native-dom bridge_tests 8 to 9 and reached::tests 4 new; any a11y-plan inventory stating these is moved.
@task_item n=12 part=touches
The a11y leg reading: green in CI#42530780903 at 54764c41, its fourteen files untouched; a text citing the a11y leg's latest result may be refreshed.
@task_fact n=1 slug=keyboard-navigation
fact: DioxusDocument::record_reached switches on an ungated reached record, written in handle_event before runtime.handle_event for each chain node whose current listeners hold the event, read back by reached_handlers in first-reached order deduplicated by id and event, a handler whose run removes its element still counted by its stable id.
@task_fact n=2 slug=keyboard-navigation
fact: stand_handler_coverage (8 tests, both layout modes) asserts that a keyboard activation exercises its control's click handler and a refused step exercises nothing (a_refused_step_exercises_nothing_and_a_keyboard_activation_its_click), and runs in the stand-repeat leg by prefix, outside the a11y leg.
@task_fact n=3 slug=screen-reader-support
fact: escher_driver::handler_coverage reads the UI graph (DioxusDocument::graph) at boot and after each replayed step, mounted left out, and lists as unexercised the graph handlers no check reached, as ids, owner ids, event names and two counts only, with no second role or name mapping.
@task_fact n=4 slug=a11y-scope-summary
fact: the reached record and its listener reader element_listener_ids compile without the accessibility feature (the wasm build compiles dioxus-native-dom without it), while element_facts stays accessibility-gated.
@task_item n=1 part=untouched
The accessibility tree builder: role derivation, the ARIA state reads (selected, toggled, invalid), the six upstream reads, labels and descriptions, and author_id as the stable id are unchanged; no builder read moved.
@task_item n=2 part=untouched
Contrast: Harness::text_contrast, stand_contrast's tables and rows, the visual lint's contrast reading and the select list's colour constants are unchanged.
@task_item n=3 part=untouched
The a11y CI leg's fourteen files and their list, and the eight of them repeated in stand-repeat, are unchanged; the new target is not in the a11y leg.
@task_item n=4 part=untouched
Focusability, the inert focus fixup, Tab and Shift+Tab orders, select and range keyboard handling and the Enter and Space dispatch rules themselves are unchanged; stand_keyboard and its stated orders did not move.
@task_item n=5 part=untouched
The snapshot model and its fields, including clickable, enabled, checked and the password mask, and the actionable-key rule and unkeyed_actionable's result, are unchanged; no snapshot field or node key was added.
@task_item n=6 part=untouched
The platform adapter residual res-llkgqygv, the Cells grid semantics residual res-wvatzhpe and the windowed AT witness are untouched.
@task_item n=7 part=untouched
Hidden-content exclusion, inert subtree handling, ARIA landmark mapping, cognitive error cues and the log format and allowlist (no new span or log field) are unchanged.
@task_item n=8 part=untouched
The canvas limit (handlers inside a canvas not counted) and the bubbling limit (stopPropagation unread) are capability-statement and residual facts, not accessibility rules.
