@task_prose part=session
The session gave Circle Drawer's canvas a keyboard path: the canvas is one Tab stop, arrows move a selected circle in declared order, Enter opens the diameter dialog, and a click on a circle selects it and adds none.
A canvas's declared virtual node gains a selected state, written into the accessibility tree and read by the snapshot, its text and a diff as selected or unselected.
The driver's covered refusal now also covers a canvas's virtual node under a node its canvas declares later whose region holds the target's centre; the cause's name, remedy and order are unchanged, its served meaning restated in help and the MCP descriptions.
The engine no longer traps Tab on a focused custom-widget element: the widget still hears the keydown, then the default Tab move runs (PROVISIONAL, flagged upstreamable), placed in escher's own function with one call in upstream's dispatch.
No new verb, argument, key name, cause, session error, exit, carrier, log field or span; no dependency, CI workflow or harness change; the a11y leg reading in the capability statement moved from 20 · 199 · 0 · 0 to 20 · 201 · 0 · 0.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited, the custom-widget virtual node declaration (VirtualNodeDeclaration), now with a selected state
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: edited, the engine's DOM event dispatch, its custom-widget arm now passing Tab on to the default focus move
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: edited, escher's own engine event additions beside the widget click synthesis, now holding the widget Tab pass-through
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited, the document's accessibility tree build with virtual nodes, now writing a declared selected state
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited, the snapshot node read, its virtual-node branch now reading selected from the tree node
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited, the declared-node read of a canvas's virtual nodes and its unit declarations
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited, the driver's command run path and target checks, now reading covered by a later-declared node of the canvas
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: edited, the actions answer (actions), its targets now marked as a canvas's virtual node so it shares the run path's check
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: edited, the refusal cause names and their served meanings, covered's meaning restated
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited, the Circle Drawer sample app, its canvas, graph state registration and declared circle nodes
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: edited, the seven_guis native host binary check
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: edited, the shared stand helpers and the samples' stated drives and tab orders (GRAPH_DRIVES, UNEXERCISED_AFTER_DRIVES)
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited, the shared stand fixture pages, their declared virtual nodes
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited, the stand check of declared virtual nodes
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: edited, the stand check that virtual nodes read as elements in the tree, the snapshot and a diff
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: edited, the stand check of Circle Drawer's circle nodes: covered, disabled, selection keys and focus events
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited, the stand keyboard check walking each sample's stated tab order
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: edited, the stand canvas click check, now with the engine check that Tab leaves a focused widget
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: edited, the stand check of Circle Drawer's canvas and its diameter dialog
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: edited, the stand check of the act error text
@task_file n=21 path=docs/capabilities.md
what: edited, the capability statement, updated by hand, with its accessibility reach reading
@task_file n=22 path=.andromeda/runs/2026-11-26T07-21-03-implement/gate-2026-11-26-canvas-keyboard-path.json
what: edited, the implement run's gate trail, a bookkeeping record, no code
@task_file n=23 path=.andromeda/runs/2026-11-26T07-21-03-implement/rec-2026-11-26-canvas-keyboard-path.json
what: edited, the chunk's land trail, a bookkeeping record, no code
@task_item n=1 part=touches
The refusal cause covered (contract/refusal-cause/covered): its meaning gains the later-node clause, a canvas's virtual node reading covered by a node its canvas declares later whose declared bounding box holds the target's centre, so a later corner covers where its painted shape does not; name, remedy and order unchanged; PROVISIONAL.
@task_item n=2 part=touches
The command span's cause field (contract/command-span/cause and contract/escher-driver-command-span/cause): it now records covered for the later-node case too; the field, its values and the span are unchanged.
@task_item n=3 part=touches
The actions answer's cause (contract/actions-answer/cause): actions lists a circle under a later one as covered, the same check as the run path; the answer's shape unchanged.
@task_item n=4 part=touches
The help command and the MCP tool descriptions (contract/escher-session-commands/help, contract/tool-call-span/help) serve covered's restated meaning text.
@task_item n=5 part=touches
The snapshot node and snapshot text (contract/snapshot-node/clickable, contract/snapshot-text/clickable and their neighbours): a canvas's virtual node now carries selected or unselected, read from the tree as for an element, and a diff names it in differs.
@task_item n=6 part=touches
The declared-node read (decision/declared-node-read-logs-nothing, decision/canvas-read-opens-no-span): the declaration gains a selected field and the driver now reads the canvas's declared nodes for the covered check; no log, event or span is added.
@task_item n=7 part=touches
The driver's no-event rule (decision/driver-emits-no-event, decision/one-span-per-command): the new covered reading inside a command adds no event and no span.
@task_item n=8 part=touches
The graph answer and the state registry (contract/graph-answer/shape, contract/graph-answer/content, decision/state-registry-holds-no-values): Circle Drawer registers one more graph state, selected, and its canvas gains a keydown handler; the registry still holds names, not values.
@task_item n=9 part=touches
The capability statement updated by hand (decision/capability-statement-updated-by-hand): its accessibility reach reading moved 20 · 199 · 0 · 0 to 20 · 201 · 0 · 0 and its covered sentence was restated.
@task_item n=10 part=touches
The handler coverage line (contract/coverage-line/content, contract/coverage-line/shape): Circle Drawer's unexercised handlers after its drives fell from 4 to 2 and its drives rose from 9 to 13; a touch only where a row holds those counts.
@task_item n=11 part=touches
Session logs carry no page content (decision/session-logs-carry-no-page-content): the selected state is a closed boolean declared by the widget; no new content reaches a log.
@task_item n=12 part=touches
Engine log sites (decision/engine-sites-fixed-messages, decision/engine-target-safe-fields): the engine's event dispatch gains the widget Tab pass-through; it writes no log site.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A canvas's declared virtual node carries its declared selected state into the accessibility tree, the snapshot node, the snapshot text and a diff's differs, read as selected or unselected, none stated reading no word.
@task_item n=1 part=untouched
Telemetry strategy: the tracing crate, the engine tracing feature and the escher crates' ungated tracing are unchanged.
@task_item n=2 part=untouched
Metric coverage and metric emission, exporters and telemetry backends: none added.
@task_item n=3 part=untouched
escher's sink log line, its scrub, stream, resource keys and RUST_LOG configuration, and the escher-telemetry events and panic hook.
@task_item n=4 part=untouched
The tool_call span, its tool and ending fields, and the closed span line format.
@task_item n=5 part=untouched
The cold-agent pipe and agent-run harness: their events, verdict fields, counted verbs, transcripts and cleanup.
@task_item n=6 part=untouched
The escher-session exit codes, error object, stderr lines, host wire and session recording files and bounds.
@task_item n=7 part=untouched
Error capture and reporting: script error surfaces, window error events, WPT reports and the panic hook.
@task_item n=8 part=untouched
CI integration: workflows, artifacts, the package-install diagnostic and the build-directory sweep line are unchanged; no dependency moved.
@task_item n=9 part=untouched
The other refusal cause names (disabled, off-screen, stale, not-found, malformed, unknown-verb, fonts-unavailable): unchanged.
