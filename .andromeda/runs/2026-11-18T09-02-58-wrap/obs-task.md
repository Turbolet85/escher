@task_prose part=session
The session widened the cold-agent pipe over 0.3.0's carried reads: its counted verbs went from three to six (lint, screenshot and advance, plus graph, actions and layout).
The cold-agent verdict and its run.end event gained five fields: graph_calls, actions_calls, layout_calls, list_opens and refused_by_cause, each a count or an object of closed cause words to counts.
The capability-unused reason now requires eight capabilities at zero (adds graph_calls, actions_calls, layout_calls and list_opens), and the fixed stand prompt asks for the UI graph, what can be done, where the list sits and an opened list, naming no element id or app slug.
The pipe now reads the driver command span's existing cause field from the server log; no driver verb, carrier, argument, cause, session error, log field or span was added, and the wrong-call bar stays 5 percent, confirmed by the founder.
Retired the contract test that pinned layout_calls absent, replaced by one that counts the layout read in the run and replays it in the check; the contract tests went from 38 to 41.
The founder's forks were given after implement: graph and actions answers may cross to the model provider on the live run, and a served error page comes in 0.3.0 before the live run, so the live run must reach the caused ending.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent pipe — runs the fixed stand prompt against escher-session's mcp role, counts the counted verbs from the server log's span lines, and writes the cold-agent verdict, its reasons and the pipe's events (run.start, run.end, status, cleanup)
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests, with a fake escher-session and a claude shim, run on the ci-scripts leg
@task_item n=1 part=touches
The decision that graph, actions and layout are not counted verbs for a cold agent is reversed: all three are now counted and required.
@task_item n=2 part=touches
The cold-agent pipe's counted-verbs contract items (advance, lint, screenshot) gain three siblings: graph, actions and layout.
@task_item n=3 part=touches
The capability-unused reason, in both the verdict's reasons list and the verdict contract, now reads eight fields at zero instead of four: lint_calls, screenshot_calls, advance_calls, graph_calls, actions_calls, layout_calls, part_reads and list_opens.
@task_item n=4 part=touches
The cold-agent verdict's field set grew by five; the closed-verdict-fields rule now holds over graph_calls, actions_calls, layout_calls, list_opens and refused_by_cause.
@task_item n=5 part=touches
The cold-agent pipe events' run.end event carries the five new verdict fields.
@task_item n=6 part=touches
The rule that the cold-agent server log is read by span name only now also covers the command span's lines, from which the pipe counts the cause field per closed word.
@task_item n=7 part=touches
The rule that the cold-agent server log is read only to count now covers counting refusals by cause; a word outside the eight closed cause words is not counted.
@task_item n=8 part=touches
The refusal cause names (covered, disabled, fonts-unavailable, malformed, not-found, off-screen, stale, unknown-verb) are now read by the cold-agent pipe in the schema's order, as the keys of refused_by_cause; renaming or adding a cause now reaches the verdict.
@task_item n=9 part=touches
The driver command span's cause field gains a reader: the cold-agent pipe counts it, without any field being added to the span.
@task_item n=10 part=touches
The rule that the cold-agent verdict holds no content holds over the new fields: each is a count or closed words to counts.
@task_item n=11 part=touches
The rule that decoded screen text stays in the match now covers the list opening count: the pipe decodes accepted click and press answers and keeps only a count of ListBoxOption rows differing in bounds alone.
@task_item n=12 part=touches
The wrong_call_bar_pct value stays 5; the founder confirmed 0.2.0's bar of at most 5 percent wrong calls for 0.3.0.
@task_item n=13 part=touches
The snapshot diff's open list row bounds item is now what the cold-agent pipe reads to count a list opening.
@task_item n=14 part=touches
The cold-agent verdict's reasons order and the wrong-calls-over-bar reason stand next to the widened capability-unused set; the order is unchanged.
@task_fact n=1 slug=log-coverage
fact: The cold-agent pipe counts six verbs from the server log: lint, screenshot, advance, graph, actions and layout.
@task_fact n=2 slug=6-log-coverage
fact: The cold-agent verdict carries graph_calls, actions_calls and layout_calls, each a count of that verb's tool_call lines, written after advance_calls.
@task_fact n=3 slug=6-log-coverage
fact: The cold-agent verdict carries list_opens, a count of accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone; an answer that does not decode as JSON is skipped.
@task_fact n=4 slug=6-log-coverage
fact: The cold-agent verdict carries refused_by_cause, an object of the eight closed refusal cause words to counts, read from the command span lines' cause field; a word outside the eight is not counted.
@task_fact n=5 slug=log-coverage
fact: The cold-agent run fails capability-unused when graph, actions, layout or a list opening is left unused, beside lint, screenshot, advance and part reads.
@task_item n=1 part=untouched
The driver's command and tool_call span contracts, their fields and endings: no span, field, verb or cause was added, and none changed.
@task_item n=2 part=untouched
escher's telemetry sink, its log line format, RUST_LOG handling, scrub set and target allowlist: no scrub-set key was added.
@task_item n=3 part=untouched
Engine tracing, the tracing-crate choice and the absence of a telemetry backend, metric emission or exporters.
@task_item n=4 part=untouched
Error capture and reporting: panic hooks, script error surfaces, window error events and WPT reports.
@task_item n=5 part=untouched
escher-session's commands, exit codes, host wire, coverage role, recording file and session lifecycle.
@task_item n=6 part=untouched
The graph, actions and layout answers' own shape, content and logging rules: the answers did not change, only how often they are counted.
@task_item n=7 part=untouched
The agent-run harness (not the cold-agent pipe): its events, test event and log rules.
@task_item n=8 part=untouched
CI platform, per-leg failure log and coverage report artifacts, and WPT publishing.
