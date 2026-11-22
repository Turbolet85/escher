@task_prose part=session
A behaviour-preserving restructuring: no public symbol, verb, refusal, role, state, key handling or layout hint changed, and every named test and stand target stayed green.
The driver's command-table unit tests moved whole out of the command module into a child test file; the seven tests keep their paths and the command module shrank from 848 to 249 code lines.
Escher's own additions inside three upstream bodies (the accessibility node builder, the keyboard event handler, the presentational hints) moved into private functions of escher's own, each one call in upstream's body.
Each ARIA attribute (expanded, selected, label, toggled/checked, invalid) is still read once; the five key arms (range, single list box, drop-down, multiple select, Enter activation) keep their order and gates; the select size list-box height, overflow and rows are unchanged.
The lock moved url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3, closing RUSTSEC-2026-0592; driver coverage read 97.80 percent against its floor 97; CI green on the pushed head.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: the escher-session command table: the verbs a driver call names, their argument rules, the session-level and instance-level checks, and the refusals (unknown-verb, a malformed call with the rule it broke)
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: the command table's unit tests: every verb admitted to its command, an unknown verb refused by name, a malformed call refused with its rule, each verb checked at its own level
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder: each node's role, name and states (expanded, selected, toggled/checked, invalid, label) that the snapshot and agents read from attributes
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: keyboard event handling: Tab closing an open list, Enter activation, range keys, single list box, drop-down and multiple select keys
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: presentational hints from attributes: a select's size giving the list box its height, its overflow-y and the open list rows
@task_file n=6 path=Cargo.lock
what: the workspace dependency lock: the resolved versions of url and form_urlencoded and the syn edges
@task_item n=1 part=touches
Stand names, roles and states read by attributes alone: the builder that reads them was restructured; the reads and their results are unchanged.
@task_item n=2 part=touches
CRUD list as one keyboard region: the list box key handling moved into an escher-own function, same arms and order.
@task_item n=3 part=touches
Open list rows read their own bounds: the list-box rows a select's size gives are now returned by an escher-own hints function, value unchanged.
@task_item n=4 part=touches
Flight type select reads its two options at boot: the drop-down select's key and state handling was restructured, not changed.
@task_item n=5 part=touches
escher-session command line form and actions answer: the command table they are checked against was split from its tests; verbs and arguments unchanged.
@task_item n=6 part=touches
escher-session refusals (unknown-verb) and exit codes: the refusal rules stay as they were; their unit tests moved whole.
@task_item n=7 part=touches
escher-session snapshot answer and its part: the accessibility states it carries come from the restructured builder, unchanged.
@task_item n=8 part=touches
Every stand control keyed: the Tab, Enter and select key paths moved into escher-own functions, behaviour kept.
@task_item n=9 part=touches
MCP server tools list and served tasks: they reach the command table whose module was split; no tool, task or bound moved.
@task_item n=10 part=touches
The escher-session coverage role check bound: no bound moved; the coverage reading changed only against the per-crate floor 97, which holds.
@task_item n=1 part=untouched
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): no window chrome changed.
@task_item n=2 part=untouched
Stand stable element ids: no id was added, renamed or removed on any stand page.
@task_item n=3 part=untouched
The agent-run and cold-agent test pipes, their exit codes, usage and run logs: no script or pipe changed.
@task_item n=4 part=untouched
The multi_doc and paint_bench command lines, their arguments, environment knobs and backends: untouched.
@task_item n=5 part=untouched
Stand page build decisions: fixed document skeleton, accent colour, pinned headless viewport, network on the load page alone, order page controls, held instance as a fresh boot.
@task_item n=6 part=untouched
Scrolling and clipping decisions (into-view scroll, visible region through clipping boxes): no scroll or clip code moved.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates and the unmeasured mobile-native, web-spa and cli surface overviews: no layout surface was added.
