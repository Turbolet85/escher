@task_prose part=session
The session added an in-process way to render one component alone, with given props, at the headless stand's pinned viewport (800 by 600 at scale 1) and its font, read as it reads inside its app: a harness constructor, a mount function and a stand boot.
Props are given as a maker run once inside the component's anonymous root, since a prop holding an event handler cannot be made outside a Dioxus runtime.
The cold-agent pipe's verb list moved from fourteen to fifteen: the layout verb joins after actions, an uncounted read like graph and actions; its counted verbs (lint, screenshot, advance) and its verdict shape are unchanged.
Measured: a component alone has no shell CSS, so its root element sizes to its content, not to 600; the document's viewport still reads 800 by 600 at scale 1, and the Timer reads the root's ticks context, falling back to wall time without it.
No driver verb, argument, carrier, MCP tool, session socket call, span or log field was added; no escher-session answer or exit status changed; nothing was retired.
A root boundary unit test confirmed that a document rebuilt in place reads one root error mark through the fallback.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: new integration check of a component alone on the headless stand: the task shell given the stand's props reads the stand's Counter node for node, the Counter alone reads its in-app subtree, a click moves it, the document keeps the stand's viewport and font, the Timer reads its root ticks
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: edited, the test harness's construction of a document; gains the constructor that renders one component alone with a props maker, and the mount of a component as the one child of an anonymous root
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: edited, the test harness crate's public surface; re-exports the component-alone mount
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: edited, the test harness's dependencies; its test build turns on the accessibility feature so its unit test reads the UI graph
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: edited, the headless stand of the seven GUIs app (its boots at the pinned viewport and font); gains a boot of one component alone with the stand's timer ticks context
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: edited, the error boundary marks of the native DOM; gains a unit test that a document built after the first build reads the root boundary's error
@task_file n=7 path=scripts/cold-agent.sh
what: edited, the cold-agent pipe that runs a model on the escher-session MCP tools; its verb list now holds fifteen verbs, layout among them
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: edited, the cold-agent pipe's contract test and its fake server; verb list at fifteen, a test that a layout read is an uncounted read and no act
@task_file n=9 path=docs/capabilities.md
what: edited, the public capabilities page; gains a subsection on rendering a component alone with its witnesses
@task_item n=1 part=touches
contract/escher-session-commands/layout-answer and layout-arguments: the layout verb is now one of the cold-agent pipe's tools, read by a cold model agent as an uncounted read; its answer and arguments are unchanged.
@task_item n=2 part=touches
contract/escher-session-mcp/other-results: the layout tool's result now reaches the cold agent through the MCP role; a row that counts or lists the tools a cold agent sees moves from fourteen to fifteen.
@task_item n=3 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: the agent reading the surface now holds fifteen tools, layout among them; check any row naming the verbs an agent is given.
@task_item n=4 part=touches
decision/actions-share-the-verbs-checks: actions stays uncounted beside graph, now with layout as the third uncounted read in the cold-agent pipe; a row naming which verbs are reads and which acts is touched.
@task_item n=5 part=touches
decision/logical-viewport-from-scale: a component alone reads the stand's viewport 800 by 600 at scale 1, while its root element sizes to its content since no shell CSS applies.
@task_item n=6 part=touches
decision/test-engine-with-system-fonts and decision/tests-built-with-system-fonts: the isolated document keeps the stand's font and reads that it can paint text; a rule over the tests' fonts now covers a component rendered alone.
@task_item n=7 part=touches
decision/screenshot-never-skips-text: the isolated document's font check asserts text can be painted, as the stand's boot does.
@task_item n=8 part=touches
contract/escher-session-tasks/counter: the Counter alone reads its four in-app rows with the shell's prefix removed from positional ids, roles, names and states equal, counter-value 60 by 28, counter-increment 95.515625 by 39, and a click moves it 0 to 1, as on the stand.
@task_item n=9 part=touches
contract/escher-session-tasks/timer: the Timer reads its ticks from the root context and falls back to a 100 ms wall-clock delay without it, so elapsed text at boot reads the same with or without the context; alone it reads 0.3s after three ticks.
@task_item n=10 part=touches
contract/ui-graph-line/content: the task shell alone given the stand's props reads a UI graph equal to the stand's Counter boot; the harness's own unit test reads the graph, its count naming the component as sole reader.
@task_item n=11 part=touches
decision/buffered-render-replay and decision/default-ua-sheet-via-default-config: indirectly, a component alone renders through the same document and default user-agent sheet, without the shell's stylesheet.
@task_fact n=1 slug=surface
fact: A component rendered alone in-process (the harness constructor, the mount as an anonymous root's one child, the stand's alone boot) keeps the stand's viewport 800 by 600 at scale 1 and its font, takes its props as a maker run once in the root's scope, and carries no shell CSS, so its root element sizes to its content; it is reached by no carrier, verb or MCP tool.
@task_fact n=2 slug=surface
fact: The cold-agent pipe gives the agent fifteen escher-session tools; layout, graph and actions are uncounted reads, lint, screenshot and advance the counted verbs, and a layout call is no act.
@task_item n=1 part=untouched
Color palette, typography scale, spacing, border radius, depth and iconography rows: no colour, type, spacing, elevation, radius or cursor value changed.
@task_item n=2 part=untouched
Motion rows on the virtual clock, scroll, overlay scrollbars and select list motion: no clock, scroll or animation behaviour changed.
@task_item n=3 part=untouched
Select list rows (its events, accessibility, overlay, painting, hit-testing): no select behaviour changed.
@task_item n=4 part=untouched
escher-session exit statuses and exit codes, stderr session and usage errors, the session error remedy, help, screenshot and snapshot answers: none changed.
@task_item n=5 part=untouched
Coverage and actions line contracts, coverage check files: their content and bounds are unchanged.
@task_item n=6 part=untouched
WPT runner output and result colours, shell provider window rows, JS window object rows and the web-spa canvas: not reached.
@task_item n=7 part=untouched
The escher-session tasks cells, crud, flight-booker, load and the unknown task: no task changed.
