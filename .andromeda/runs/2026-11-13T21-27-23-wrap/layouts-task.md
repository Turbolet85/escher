@task_prose part=session
The session restructured three functions to lower their cognitive load, with behaviour byte for byte unchanged: the engine's click handling, the escher-session driver's command execution, and the Dioxus document's state graph.
Each was split into private functions, one per arm or step; no public symbol, verb, refusal cause, span field, log field, key name or carrier was added, removed or re-signed.
Measured cognitive load: click handling 97 to 2, command execution 32 to 1, state graph 50 to 4; functions over 15 in the three files 6 to 3.
It added one silent check of click default actions (link navigation, label walk-on, focus cleared on an unmatched click, disabled submit, multiple-select label, double click), run in both layout modes.
Coverage floors held (escher-driver 98.01, dioxus-native-dom 90.09, blitz-test-harness 95.74); nothing retired, no dependency or config changed.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer events: a click's default actions (checkbox, radio, summary, select, option, label re-dispatch, link navigation, file input), focus clearing and the double click
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the escher-session driver's command execution: one arm per verb (snapshot, click, type, press, advance, scroll, screenshot, graph, lint, actions) and each verb's answer
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the Dioxus document's state graph: states, handlers, writes and reads behind the escher-session graph answer
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a check of a click's default actions on a plain HTML page and a double click on a Dioxus page, in both layout modes
@task_item n=1 part=touches
The escher-session graph answer: the state graph it reports was restructured into private steps, its content unchanged.
@task_item n=2 part=touches
The escher-session snapshot answer and its --part paging: the snapshot verb now runs in its own function, its text budget and parts unchanged.
@task_item n=3 part=touches
The escher-session screenshot answer: the verb's crop and its error map moved into their own function, unchanged.
@task_item n=4 part=touches
The escher-session command form and its actions answer: every verb is now one call from a single dispatch; lint and actions stay one-line arms.
@task_item n=5 part=touches
The escher-session exit codes and refusals (off-screen, unknown-verb, fonts-unavailable): the verb dispatch that leads to them was restructured; no refusal cause was added or removed.
@task_item n=6 part=touches
Decisions on what a click does on the stand: the crud list as one keyboard region, the flight type select read at boot, open list rows reading their own bounds; the engine's click default actions were split per element kind, behaviour unchanged.
@task_item n=7 part=touches
Decisions on a type step on a select or a text field: the type verb's acting order (select focus or click, select-all and backspace on an empty text, typing skipping a leading space on a select) is now a named step, unchanged.
@task_item n=8 part=touches
Decisions on scroll and advance verbs (into-view scroll moving every holding scroller, visible region through clipping boxes): the scroll and advance verbs now run in their own functions, unchanged.
@task_item n=9 part=touches
The load page as the only one with a network provider and link navigation: the new check records navigations through its own provider, and the link default action moved into its own function.
@task_item n=10 part=touches
The session recording as a replayable check and the stand ids as attributes: the graph verb still feeds the session's record of ids, now from its own function.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts (drag, maximise, minimise, close, decorations): no window chrome was reached.
@task_item n=2 part=untouched
The stand stable element ids and served tasks: no id was added, renamed or freed and no stand page changed.
@task_item n=3 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines: no script, argument, exit code or output of theirs changed.
@task_item n=4 part=untouched
The escher-session MCP server, coverage role, host log and recording files: no transport, tool list, bound, log field or recording artefact changed.
@task_item n=5 part=untouched
The stand accent colour, document skeleton, headless pinned viewport and the absent index.html templates: no layout, style or template changed; the mobile-native and web-spa surfaces are not reached.
