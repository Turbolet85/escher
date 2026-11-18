@task_prose part=session
The session widened the cold-agent pipe over escher 0.3.0's carried reads: its counted verbs go from 3 to 6 (lint, screenshot, advance, plus graph, actions and layout), and the capability-unused requirement set goes from 4 to 8 (adding graph_calls, actions_calls, layout_calls and list_opens).
The cold-agent verdict and its run.end event gain five fields, each a count or an object of closed words to counts: graph_calls, actions_calls, layout_calls, list_opens and refused_by_cause (the eight closed refusal cause words, read from the existing cause field of the command span lines in the server log; a word outside the eight is not counted).
list_opens counts accepted click or press answers in the raw transcript whose decoded changed list holds a ListBoxOption differing only in bounds; an answer that does not decode is skipped, and only the count leaves.
The fixed stand prompt gains four sentences: read the counter's UI graph, ask what can be done on the flight booker's screen and open its flight type list, ask where CRUD's people list sits and why it has its size, and ask Cells for neither; it still names no element id or app slug. The wrong-call bar stays 5 %.
No driver verb, carrier, argument, cause, session error, log field or span was added, and no scrub-set key; no live run fired, so no answer reached the model provider.
After implement the founder answered: F1 keeps the 5 % bar, F3 allows one live run under a 3 USD budget, F4 approves graph and actions answers crossing to the model provider on the live run, and F5 places a seventh served app (an error page whose authored error text would cross to the provider) in 0.3.0 before that run, superseding the ruling that caused text is out of the run's reach.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent pipe (cold-agent.sh) — its fixed stand prompt, its counted verbs, its server-log and transcript readers, its verdict and run.end event, its capability-unused reason and its wrong-call bar; edited
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: the cold-agent contract tests and their fake escher-session, which now writes a command span line per instance-verb call with its cause when refused; edited, one test retired and replaced
@task_item n=1 part=touches
decision/cold-agent-screen-to-provider: the founder's F4 approves graph and actions answers crossing to the model provider on the live run, and the widened prompt now asks for graph, actions and layout reads; the row is to be restated at this wrap. No live run fired in this session.
@task_item n=2 part=touches
decision/graph-answer-crossing and decision/actions-answer-crossing: the cold-agent run now asks the agent for these answers, so on the live run they cross to the model provider (approved by F4).
@task_item n=3 part=touches
decision/caused-text-crossing: the ruling that caused text is out of the cold-agent run's reach is superseded by F5; a served error page whose authored error text crosses to the model provider is placed before the live run, a boundary widening its text names (not built in this session).
@task_item n=4 part=touches
decision/cold-agent-fixed-prompt: the fixed prompt gains four sentences asking for the 0.3.0 reads and the flight type list; it stays fixed and names no element id or app slug.
@task_item n=5 part=touches
decision/cold-agent-wrong-call-bar: its value stays 5 %, now on the founder's F1 (0.2.0's rule kept for 0.3.0); the pipe's comment still says it holds 0.2.0's bar until the founder's number.
@task_item n=6 part=touches
decision/cold-agent-logs-metadata-only and decision/cold-agent-events-metadata-only: the verdict and its run.end event gain five fields, each a count or an object of closed words to counts; no content enters them.
@task_item n=7 part=touches
decision/cold-agent-server-output-unprinted: the pipe now reads every server log line to count the command span's cause per closed word; it prints none of it.
@task_item n=8 part=touches
decision/cold-agent-transcript-raw and decision/cold-agent-check-reader-counts-only: the pipe now decodes acting answers from the raw transcript to count list openings, keeping only the count.
@task_item n=9 part=touches
decision/model-reply-untrusted-input: the new list-opening reader treats the transcript's answers as untrusted, decoding JSON and skipping what does not decode.
@task_item n=10 part=touches
contract/command-span/cause and contract/driver-command-span/cause: the cold-agent pipe is a new reader of the command span's cause field; the field itself is unchanged.
@task_item n=11 part=touches
decision/closed-refusal-cause-set, decision/refusal-closed-cause-set and contract/refusal-causes/*: the pipe holds its own copy of the eight closed cause words in the schema's order for refused_by_cause; the set is unchanged.
@task_item n=12 part=touches
decision/select-list-crossing and decision/open-list-shot-pixels: the prompt now asks the agent to open the flight type select list and click its option, and list_opens reads a ListBoxOption bounds diff; only the count leaves the pipe.
@task_item n=13 part=touches
contract/diff-node/differs: the pipe now reads the differs key of changed diff nodes (differs equal to bounds alone) to count a list opening.
@task_item n=14 part=touches
decision/kept-check-graph-answer, decision/kept-check-actions-answer and decision/kept-check-layout-answer: the cold-agent run now asks for these reads, so on the live run its kept checks will hold those answers.
@task_item n=15 part=touches
decision/closed-value-field-no-widening and decision/boundary-widening-rule: the five new verdict fields are closed-value fields (counts, closed words to counts), no widening; F5's error page names a widening still owed.
@task_item n=16 part=touches
decision/cold-agent-call-validation: the pipe's counted verbs move from 3 to 6 (graph, actions and layout added) and the capability-unused requirement set from 4 to 8.
@task_item n=17 part=touches
decision/live-runs-use-operator-login and decision/no-screen-content-in-logs: F3 allows one live run with a 3 USD budget limit, after the served error page exists; the run's evidence and logs stay content-free.
@task_fact n=1 slug=logging-monitoring
fact: The cold-agent verdict and its run.end event carry refused_by_cause, an object of the eight closed refusal cause words to counts read from the command span lines in the server log, a word outside the eight not counted, and list_opens, a count; neither carries call input or screen content.
@task_fact n=2 slug=input-validation
fact: The cold-agent pipe decodes the transcript's acting answers only to count list openings (a ListBoxOption changed in bounds alone); an answer that does not decode is skipped, and the decoded answer leaves the pipe only as that count.
@task_fact n=3 slug=threat-model-summary
fact: The founder approved (F4) graph and actions answers crossing to the model provider on the one live cold-agent run, and placed (F5) a served error page before that run, so caused text is in the run's reach and no longer out of it.
@task_item n=1 part=untouched
Driver verbs, argument kinds, refusal JSON, session errors and the session socket wire contracts: no verb, carrier, argument, cause, session error, log field or span was added or changed.
@task_item n=2 part=untouched
The MCP server's transport, protocol errors, request and result bounds, and the escher-session flags, roles and exit codes.
@task_item n=3 part=untouched
Authentication and authorization, CI tokens, publish secrets and signing material.
@task_item n=4 part=untouched
Dependency security: no crate, dependency or dev-tool version changed.
@task_item n=5 part=untouched
The engine's own input validation and error handling: HTML, CSS and SVG parsing, script fetch and URL scheme limits, select list keys, painting and closing, stable element id derivation.
@task_item n=6 part=untouched
Browser data protection: history database, cookies, HTTP cache, TLS, clipboard and file dialog.
@task_item n=7 part=untouched
API security's outbound HTTP, load waits, idle expiry and the absence rows of CORS, CSP and response limits.
