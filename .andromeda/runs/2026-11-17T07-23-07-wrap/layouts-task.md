@task_prose part=session
The session adds a way to boot one component alone in the headless stand, with given props, at the stand's pinned 800 x 600 viewport and font, reading as it does inside its app.
The props come from a props maker run once in an anonymous root's scope, since a props value cannot carry an event handler made outside a Dioxus runtime; the stand's alone boot puts the timer ticks context on that root.
Measured: the shell given the stand's props reads the stand's Counter boot node for node; the Counter alone reads its in-app subtree with the TaskShell prefix removed from positional ids, its stable ids, roles, names and states unchanged; a click moves counter-value 0 to 1.
Measured: a component alone has no shell CSS, so its root element sizes to its content rather than 600 high; only the document's viewport reads 800 x 600 at scale 1.
The cold-agent pipe's verb list goes from fourteen to fifteen, with layout after actions, as an uncounted read like graph and actions; the counted verbs and the verdict shape are unchanged.
No verb, argument, carrier, MCP tool, session socket call, element id or existing stand boot was added or changed.
@task_file n=1 path=tests/blitz-tests/tests/stand_component_alone.rs
what: new stand check, a component in isolation - the shell and the Counter booted alone against the stand's Counter boot, a click, the pinned viewport and font, the timer reading its ticks from the root context
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs
what: the test harness and its construction - edited to add a constructor and a virtual dom maker that mount one component alone under an anonymous root, plus its first unit test
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs
what: the test harness's public surface - edited to re-export the component-alone virtual dom maker
@task_file n=4 path=packages/blitz-test-harness/Cargo.toml
what: the test harness's manifest - edited to turn on the accessibility feature of the native dom for its test build only
@task_file n=5 path=examples/seven_guis/src/stand.rs
what: the headless stand and its boots - edited to add an alone boot of one component with given props at the stand's options, carrying the timer ticks context
@task_file n=6 path=packages/dioxus-native-dom/src/boundary.rs
what: the error boundary registry of the native dom - edited to add a unit test that a document built after the first build reads the root boundary's error
@task_file n=7 path=scripts/cold-agent.sh
what: the cold-agent pipe script - edited so its verb list, and with it the agent's tool set, holds fifteen verbs with layout after actions
@task_file n=8 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and fake server - edited to hold fifteen verbs, answer layout as a read, and test that a layout call is an uncounted read and no act
@task_file n=9 path=docs/capabilities.md
what: the public capabilities page - edited to add a section on a component alone with its witnesses
@task_item n=1 part=touches
decision/headless-stand-pinned-viewport: a component booted alone keeps the stand's pinned 800 x 600 viewport and font at scale 1, but its root element reads its content height since no shell CSS reaches it - something belongs beside this row.
@task_item n=2 part=touches
decision/session-roles-share-stand-boot: the stand gains a new alone boot of one component beside the existing boots, none of which changed; a row listing the stand's boots is touched.
@task_item n=3 part=touches
decision/fixed-document-skeleton-main-mount: a component alone mounts as the one child of an anonymous root with no shell document or main mount around it - an exception or a note beside the fixed skeleton.
@task_item n=4 part=touches
decision/stand-ids-as-attributes-alone and decision/stand-a11y-by-attributes-alone: a component alone reads the same ids, roles, names and states as in the app, its positional ids without the TaskShell prefix - indirect, through the stand.
@task_item n=5 part=touches
contract/stand-element-ids/counter-value and contract/stand-element-ids/counter-increment: read alone with their ids unchanged and their sizes (60 x 28 and 95.515625 x 39) equal to the app's; touched only if a cell holds the bounds or names where the id is read.
@task_item n=6 part=touches
decision/held-instance-reads-as-fresh-boot: the alone boot is a fresh boot of one component; a row stating what a stand boot reads may need the alone case beside it.
@task_item n=7 part=touches
absence/arbitrary-index-html-templates: a component alone renders with no shell document at all; no template or index.html was added, so the absence stands, but the alone boot is a second document shape beside it.
@task_item n=8 part=touches
decision/cold-agent-uses-driver-mcp: the cold agent's tool set through the driver's MCP server is now fifteen tools, layout included; any cell naming fourteen tools or verbs is stale.
@task_item n=9 part=touches
contract/escher-session-layout/answer: the layout answer is now reachable by the cold agent on the pipe, as an uncounted read; its answer shape is unchanged.
@task_item n=10 part=touches
contract/escher-session-mcp/tools: the tools list itself is unchanged, but the cold-agent pipe now admits all fifteen of its tools rather than fourteen.
@task_item n=11 part=touches
decision/cold-agent-pipe-builds-nothing and decision/cold-agent-prompt-names-no-id: a rule over the cold-agent pipe, whose verb list changed; the pipe still builds nothing and the prompt is unchanged, though the agent can now read ids through layout.
@task_item n=12 part=touches
contract/cold-agent-cli/stdout and contract/cold-agent-cli/usage: the pipe's verdict shape gains no layout field and its counted verbs stay lint, screenshot and advance; touched only where a cell holds the verb count fourteen.
@task_item n=13 part=touches
decision/session-recording-replayable-check: an agent-written check's tool may now be layout, one of the fifteen; layout steps are reads, not acts.
@task_fact n=1 slug=surface-desktop-native
fact: The headless stand can boot one component alone with given props, made by a props maker run once in an anonymous root's scope, at the stand's pinned viewport and font, and it reads as inside its app; its root element sizes to its content because no shell CSS reaches it, so the pinned size is read on the document's viewport, never on the root's box.
@task_fact n=2 slug=surface-desktop-native
fact: A component alone is booted in process only, through the stand and the test harness; no carrier, verb or MCP tool boots a component alone.
@task_fact n=3 slug=surface-cli
fact: The cold-agent pipe gives the agent fifteen tools, layout after actions, and layout is an uncounted read like graph and actions; the counted verbs stay lint, screenshot and advance.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts: no window control, decoration or maximize behaviour changed.
@task_item n=2 part=untouched
Stand element ids other than the Counter's: no id was added, renamed or freed on any task card, circle, crud, flight, temp, timer, cells or load control.
@task_item n=3 part=untouched
decision/stand-accent-colour, decision/stand-lints-clean-at-boot, decision/load-page-only-network, decision/crud-list-keyboard-region, decision/flight-type-select-boot-reading, decision/open-list-rows-own-bounds, decision/scroll-into-view-moves-every-scroller, decision/visible-region-through-clipping-boxes, decision/every-stand-control-keyed: no task's look, keying, scrolling, clipping or network changed.
@task_item n=4 part=untouched
The escher-session verbs and their answers, exit codes, refusals, served tasks, snapshot, screenshot, recording files, host log and command form: no verb, argument, refusal or carrier was added or changed.
@task_item n=5 part=untouched
The agent-run.sh test contract, the multi_doc and paint_bench command lines: none of their arguments, outputs or exits moved.
@task_item n=6 part=untouched
decision/mcp-server-local-stdio, decision/no-caller-input-echoed and the MCP server's protocol revisions, result bound, stdio and end: the server is unchanged.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the Decisions Log: nothing measured or changed reaches them.
