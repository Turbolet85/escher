@task_prose part=session
The cold-agent pipe now counts six verbs, adding graph, actions and layout to lint, screenshot and advance.
Its capability-unused requirement set widens from four to eight: graph_calls, actions_calls, layout_calls and list_opens join lint_calls, screenshot_calls, advance_calls and part_reads, each failing at 0.
The verdict and its run.end event gain five fields: graph_calls, actions_calls, layout_calls, list_opens and refused_by_cause, each a count or an object of closed words to counts; no scrub key is added.
The fixed stand prompt gains four asks (counter: read its UI graph; flight booker: ask what can be done and open the flight type list; CRUD: ask where its people list sits; Cells: ask for neither), still naming no element id or app slug.
The wrong-call bar stays 5 %; the founder kept it for 0.3.0. No live run fired, and no driver verb, carrier, cause, session error, log field or span was added.
The offline contract tests grow from 38 to 41 cases; the test pinning layout as an uncounted read is retired and replaced by one pinning it counted and replayed in the check.
@task_file n=1 path=scripts/cold-agent.sh
what: edited — the cold-agent pipe: it runs the cold agent over the stand with the fixed prompt, counts its calls from the server log, and writes the cold-agent verdict and its run events
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: edited — the cold-agent pipe's offline contract tests under unittest, with the hand-written fake escher-session and claude stand-ins, run on the ci-scripts leg
@task_item n=1 part=touches
decision/cold-agent-passes-on-positive-evidence: the positive evidence a pass needs widens; capability-unused now requires graph, actions and layout calls and a list opening besides lint, screenshot, advance and part reads (4 to 8).
@task_item n=2 part=touches
contract/cold-agent-verdict/capability-unused: its requirement set moves from four fields to eight (adds graph_calls, actions_calls, layout_calls, list_opens).
@task_item n=3 part=touches
decision/cold-agent-prompt-names-no-ids: the prompt is widened with four app sentences asking for the 0.3.0 reads and the painted list; the rule holds, pinned by the no-stand-id test.
@task_item n=4 part=touches
contract/cold-agent-tasks/stand: the stand task the prompt gives now asks the counter for its UI graph, the flight booker for what can be done and its flight type list opened, CRUD for where its list sits, and Cells for neither.
@task_item n=5 part=touches
contract/cold-agent-events/run-end: the run.end event gains the five new verdict fields.
@task_item n=6 part=touches
contract/cold-agent-verdict/reasons and contract/cold-agent-verdict/counts-agree: the verdict they read over gains five fields and six counted verbs; the reason names themselves are unchanged.
@task_item n=7 part=touches
decision/cold-agent-events-content-free and decision/cold-agent-output-scrubbed: the new fields are counts or closed words to counts; no scrub-set key was added and both scrub cases read green.
@task_item n=8 part=touches
decision/cold-agent-server-log-counted-only: the pipe now reads every server log line and counts the command span's existing cause field per closed word; still counted, never shown.
@task_item n=9 part=touches
decision/cold-agent-wrong-call-bar: the bar stays 5 %, now held for 0.3.0 by the founder's word; the pipe's comment says it holds 0.2.0's bar until that number.
@task_item n=10 part=touches
decision/cold-agent-tests-offline: the offline fake host now writes a command span line per instance-verb call with its cause when refused, takes causes and lists knobs, and answers acting calls in the driver's added/removed/changed shape.
@task_item n=11 part=touches
decision/cold-agent-replayed-check: a layout read is now counted in the run and replayed in the check; the case pinning it uncounted is retired, any row guard naming that test is stale.
@task_item n=12 part=touches
Any row stating graph, actions or layout are uncounted reads of the cold-agent run, or naming three counted verbs: now six.
@task_item n=13 part=touches
decision/mcp-capture-counts-by-span: the pipe's tool_call-span counts now cover six verbs, and it also reads the command span lines.
@task_item n=14 part=touches
contract/driver-command-span/one-line-per-call, contract/stand-log/command-span and contract/session-host-stderr/command-span: indirect, the cold-agent pipe is a new reader of the command span's cause field; the span itself is unchanged.
@task_item n=15 part=touches
decision/eight-refusal-causes-in-order: indirect, the pipe now holds the eight closed cause words in the schema's order to count refusals by.
@task_item n=16 part=touches
decision/mutation-controlled-tests: controls (a) to (d) ran red-before-green; a deletion control on a prompt goal was vacuous where the goal's words stood twice, and the ci-scripts leg's unittest controls were read by hand.
@task_fact n=1 slug=3-test-harness-contract
fact: The cold-agent verdict counts six verbs: lint, screenshot, advance, graph, actions and layout, each as a calls field.
@task_fact n=2 slug=3-test-harness-contract
fact: The cold-agent verdict field refused_by_cause is an object of the eight closed refusal cause words, each a count of the command span lines refused with that cause; a word outside the eight is not counted.
@task_fact n=3 slug=3-test-harness-contract
fact: The cold-agent verdict field list_opens counts accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone; an undecodable answer is skipped.
@task_fact n=4 slug=3-test-harness-contract
fact: The cold-agent stand prompt asks the counter for its UI graph, the flight booker for what can be done and its return flight chosen by opening the flight type list, CRUD for where its people list sits and why, and tells Cells to ask for neither.
@task_fact n=5 slug=unit-test-strategy
fact: The cold-agent pipe's fake escher-session writes a command span line per instance-verb call after its tool_call line, with its cause word when refused, and none for start, status, stop, help or an error ending.
@task_fact n=6 slug=unit-test-strategy
fact: A deletion control on a prompt goal pins the goal by a phrase that stands once in the prompt; a phrase that also stands elsewhere makes the control vacuous.
@task_fact n=7 slug=unit-test-strategy
fact: Controls on the ci-scripts leg are read by hand from their removed log, since the gate's control reader reads only nextest, libtest and pytest case lines and reads unittest INDETERMINATE.
@task_item n=1 part=untouched
Driver verbs, session wire, session errors, run carriers and the driver session contracts: no verb, carrier, argument, cause, error, log field or span was added or changed.
@task_item n=2 part=untouched
The refusal cause set and its printed form: the eight words are only read, not changed.
@task_item n=3 part=untouched
Harness construction, settle, input, inspection, contrast and screenshot contracts, and the virtual clock decisions: no engine or harness change.
@task_item n=4 part=untouched
WPT runner, status and conformance rows, and the CI integration, coverage floor and mutation baseline gates: no Rust delta and no CI job change.
@task_item n=5 part=untouched
The escher-session CLI and MCP role contracts, help and tools/list: no command or tool changed.
@task_item n=6 part=untouched
decision/cold-agent-live-run-off-ci and decision/real-mcp-client-outside-ci: no live run fired and none moved into CI.
@task_item n=7 part=untouched
The capability statement rows and the agent-run events and verbs: the docs and the agent-run driver were not touched.
@task_item n=8 part=untouched
absence/mocking-library and absence/windowed-end-to-end-tests: stand-ins stay hand-written and no window is launched.
