@task_prose part=session
The session widened the cold-agent pipe, the harness that runs a cold model agent over the served stand apps through escher-session and writes a verdict.
The pipe now counts six verbs instead of three: lint, screenshot and advance, plus graph, actions and layout.
The verdict and its run.end event gain five count fields: graph_calls, actions_calls, layout_calls, list_opens (accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone) and refused_by_cause (the command span's closed refusal cause words, each counted).
The capability-unused reason now requires eight capabilities used, not four; the stand prompt now asks the counter for its UI graph, the flight booker for what can be done and to open the flight type list and click its option, CRUD for where its people list sits and why it has its size, and tells the Cells task not to ask for the graph or for actions.
No driver verb, carrier, argument, cause, session error, log field or span was added; the wrong-call bar stays 5 percent; the live run was not fired.
A measurement carried from research disproves a standing figure: the Cells actions answer measures 118,292 B at HEAD, not 106,088 B.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent pipe: the stand prompt, the counted verbs, the span and refusal-cause counts, the list-opening count and the verdict with its reasons (capability-unused, the wrong-call bar)
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and their fake escher-session, which writes command span lines with refusal causes and answers acting calls in the driver's added/removed/changed diff shape
@task_item n=1 part=touches
contract/escher-session-tasks/counter, flight-booker, crud and cells: the cold-agent stand's goals for these apps widened; the counter is asked to read its UI graph, the flight booker to ask what can be done and to choose the return flight by opening the flight type list, CRUD to ask where its people list sits, Cells told not to ask for the graph or actions.
@task_item n=2 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: the agent reader of escher-session is now counted on graph, actions and layout reads and required to use each.
@task_item n=3 part=touches
contract/escher-session-commands/graph-answer, actions-answer and layout-answer: these answers are now counted verbs of the cold-agent run and required at least once by capability-unused.
@task_item n=4 part=touches
contract/escher-session-commands/actions-answer and contract/actions-line/content: where a row holds the Cells actions answer size as 106,088 B, it is stale; it measures 118,292 B at HEAD, each node now listing layout.
@task_item n=5 part=touches
contract/ui-graph-line/content and contract/actions-line/nodes: the lines an agent now must read in the stand run; any row saying graph, actions or layout are uncounted or optional reads in the cold-agent run is stale.
@task_item n=6 part=touches
contract/select-list-events/open, decision/single-select-open-gestures and contract/snapshot/select-list-row-bounds: a select list opening is now counted by the cold-agent run as an accepted click or press whose diff changes ListBoxOption rows in bounds alone; the stand requires the flight type list opened.
@task_item n=7 part=touches
contract/escher-session-mcp/other-results: graph and actions tool results now cross to the model provider on the live run, by the founder's F4 approval; the run itself was not fired.
@task_item n=8 part=touches
contract/escher-session-session-error/remedy and the refusal rows: refusals are now counted by their closed cause word from the command span's existing cause field; no cause word was added or renamed.
@task_item n=9 part=touches
decision/actions-share-the-verbs-checks: the cold-agent run now uses actions as a required read beside the verbs it advises.
@task_fact n=1 slug=surface
fact: The cold-agent run counts a select list opening as an accepted click or press whose changed list holds a ListBoxOption row differing in bounds alone, and fails capability-unused when no list is opened.
@task_fact n=2 slug=surface
fact: The cold-agent stand requires graph, actions and layout each read at least once (counter graph, flight booker actions, CRUD layout); the Cells task is told to ask for neither the UI graph nor what can be done.
@task_item n=1 part=untouched
Colour, typography, spacing, depth, border radius and iconography rows: no engine paint, font, colour token or cursor changed.
@task_item n=2 part=untouched
Motion rows (virtual clock, advance, scroll, overlay scrollbar, select list motion): no clock or animation behaviour changed; advance stays a counted verb as before.
@task_item n=3 part=untouched
escher-session exit status and exit code rows, stdout, stderr and help rows: no command, argument, exit status, output line or help text changed.
@task_item n=4 part=untouched
Select list engine overlay, paint order, hit-testing and accessibility rows: the select list's behaviour and its accessibility tree are unchanged; only its opening is now counted by the harness.
@task_item n=5 part=untouched
WPT runner output and colours, dioxus-native config, shell provider, JS window and MountedData rows: none of these surfaces moved.
@task_item n=6 part=untouched
surface-web-spa and brand-identity: no WASM page or brand surface changed.
