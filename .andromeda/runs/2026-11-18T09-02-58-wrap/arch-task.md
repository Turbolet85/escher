@task_prose part=session
The session widened the cold-agent pipe over escher 0.3.0's three carried reads: graph, actions and layout are now counted verbs beside lint, screenshot and advance (3 to 6).
The capability-unused verdict reason now requires eight capabilities, not four: lint, screenshot, advance, graph, actions, layout, part reads and a list opening.
The verdict and its run.end event gain five fields: graph_calls, actions_calls, layout_calls, list_opens and refused_by_cause (the refusals counted per each of the eight closed refusal cause words, read from the existing command span's cause field).
The fixed stand prompt gains four goal sentences: read the counter's UI graph; ask what can be done on the flight booker's screen and choose the return flight by opening its flight type list; ask where CRUD's people list sits and why it has its size; for Cells, ask neither for the UI graph nor for what can be done. It still names no element id or app slug.
The wrong-call bar stays at 5 percent; the founder later answered keep 5 percent, allow graph and actions answers to cross to the model provider on the live run, and place a seventh served error-page app in 0.3.0 before the live run, which was not fired.
No driver verb, carrier, argument, refusal cause, session error, log field, span or scrub key was added.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent pipe, the host-run cold-agent client and its stdlib Python judge: its counted verbs, the verdict and its reasons, the run.end event, the fixed stand prompt and the wrong-call bar
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests on the ci-scripts leg, with their fake escher-session that writes tool_call and command span lines and answers acting calls in the driver's added, removed and changed shape
@task_item n=1 part=touches
contract/cold-agent-verdict-reasons/capability-unused: the required set grows from four to eight, adding graph_calls, actions_calls, layout_calls and list_opens, each failing the run at 0.
@task_item n=2 part=touches
decision/cold-agent-six-apps: the six apps' goals in the fixed prompt now ask for 0.3.0's reads (the counter's UI graph, the flight booker's actions and its flight type list opening, CRUD's layout) and tell the agent not to ask Cells for graph or actions.
@task_item n=3 part=touches
The cold-agent verdict.json contract items (lint_calls, screenshot_calls, advance_calls, part_reads, wrong_call_bar_pct, passed): the verdict's field set grows by five beside them, with graph_calls, actions_calls and layout_calls after advance_calls and list_opens after part_reads.
@task_item n=4 part=touches
contract/cold-agent-events/run-end: the run.end event carries the five new verdict fields.
@task_item n=5 part=touches
decision/cold-agent-wrong-call-bar and contract/cold-agent-verdict/wrong-call-bar-pct: the bar's value stays 5 percent, now held as 0.3.0's bar on the founder's answer; a row that says it awaits a 0.3.0 number is stale.
@task_item n=6 part=touches
decision/cold-agent-run-carries-stand-screen, decision/model-provider-only-on-live-cold-agent-run and decision/model-provider-operator-only: the founder allowed graph and actions answers to cross to the model provider on the live run; a row naming only the snapshot, screenshot or lint content as what crosses is stale.
@task_item n=7 part=touches
Rows over the cold-agent prompt naming no ids (the fixed prompt, no element id or app slug): the prompt widened by four sentences and still names none; the rule holds over a larger prompt.
@task_item n=8 part=touches
decision/refusal-causes-closed-at-eight and the refusal-causes contract items: the pipe now reads and counts these eight closed words from the command span; a word outside them is not counted.
@task_item n=9 part=touches
contract/command-span/cause and contract/command-span/verb: the cold-agent pipe is now a reader of the command span's existing cause field; no field is added.
@task_item n=10 part=touches
The driver's acting-verb result and snapshot diff items (changed, differs with bounds, ListBoxOption rows): the pipe now decodes accepted click and press answers to count a list opening as a changed ListBoxOption whose differs is exactly bounds.
@task_item n=11 part=touches
decision/cold-agent-judge-stdlib-python and decision/cold-agent-client-host-only: the judge grows (cause counting, list-opening reading) and stays stdlib Python on the host.
@task_item n=12 part=touches
contract/ci-leg-legs/ci-scripts and decision/ci-python-unittest: the cold-agent contract tests grow from 38 to 41 cases under unittest; a row holding either count is stale.
@task_item n=13 part=touches
contract/cold-agent-cli/run-task and contract/cold-agent-exit-codes: the live run stand was not fired; the live run now waits behind a seventh served error-page app in 0.3.0.
@task_item n=14 part=touches
decision/capability-statement and contract/capability-statement/sections: the statement's cold-agent part is unchanged; the founder ruled it states the run as measured, naming its reach, once the live run fires.
@task_fact n=1 slug=standard-contracts
fact: The cold-agent verdict and its run.end event carry graph_calls, actions_calls and layout_calls, each a count of the agent's calls to that verb read from the tool_call span.
@task_fact n=2 slug=standard-contracts
fact: The cold-agent verdict carries list_opens, the count of accepted click or press answers whose changed list holds a ListBoxOption row whose differs is exactly bounds; an answer that does not decode as JSON is skipped.
@task_fact n=3 slug=standard-contracts
fact: The cold-agent verdict carries refused_by_cause, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
@task_fact n=4 slug=standard-contracts
fact: The cold-agent pipe counts six verbs (lint, screenshot, advance, graph, actions, layout), and a run that leaves any of them, part reads or a list opening unused fails as capability-unused.
@task_fact n=5 slug=occupied-resources
fact: On the live cold-agent run, graph and actions answers may cross to the model provider, by the founder's word; the authored error text of a planned served error-page app is a further crossing not yet ratified.
@task_item n=1 part=untouched
The engine: layout, style, paint, DOM, select list, hit testing, scrolling, accessibility tree and script rows. Nothing in the engine moved.
@task_item n=2 part=untouched
The driver's verbs, argument kinds, refusal faults, session errors, session wire, socket host and verb table. No verb, carrier, argument, cause or error was added.
@task_item n=3 part=untouched
The MCP server and mcp role rows: tool listing, handshake, bounds and tool results are unchanged.
@task_item n=4 part=untouched
Telemetry rows: the scrub sets, line format, targets and spans. No log field, span or scrub key was added.
@task_item n=5 part=untouched
Kept checks, recording and replay rows: the recording directory and check files are unchanged.
@task_item n=6 part=untouched
The seven_guis stand and its apps: no app, stand check, element id or served page changed; the planned error-page app is not built.
@task_item n=7 part=untouched
WPT runner, browser app, examples, rdme and upstream sync rows.
@task_item n=8 part=untouched
Stack, toolchain, CI workflow, coverage floors, mutation tier and dependency audit rows: no dependency, dev tool or CI job changed.
@task_item n=9 part=untouched
The agent-run.sh contracts and the root README's MCP tool count: neither moved.
