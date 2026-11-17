@task_prose part=session
The session split the driver's answer writer by answer family into seven crate-private child modules: snapshot, diff, caused, layout, graph, actions and lint.
The writer's entry points stay in the parent module: strings, lists, the object builder, the screenshot without its image, coverage, the outcome dispatch, refusals, the session error and the session lines.
Every answer escher-session writes is byte for byte unchanged: no key, field order, word or value moved, and 49 literal JSON pins and the stand replay of both Counter checks read green.
The writer's unit tests moved out of line into one test file with their names unchanged; no public symbol, dependency, verb, exit status or schema changed.
CI read green 17/17 and the driver's coverage read 97.68 % against its 97 % floor.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: edited; the driver's hand-written answer writer for escher-session, now its parent module: string and number writing, the object builder, the screenshot answer without its image, the coverage line, the outcome dispatch per verb, refusals, the session error with its remedy, and the session lines
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: added; the snapshot answer's node writer: each node with the schema fields it has, its reading word, its bounds and its differs mark
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: added; the acted answer a verb returns: settled, busy and the snapshot diff of added, removed and changed nodes
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: added; the caused and caused-past lists an answer carries: panics, errors and logs with source, target, file, line, owner and event
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: added; the layout answer: placer, shift and size-cause words, lengths, placement, shift and size reasons, box and offset
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: added; the UI graph line's node and state writer: owner, component, handlers, writes and their declared or confirmed readings
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: added; the actions line's node writer: each actionable node with its owner
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: added; the lint finding writer: each finding with its defect, bounds and reason
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: added; the answer writer's unit tests: the 49 literal JSON pins for every outcome, refusal, session error, session answer, layout word and coverage line
@task_item n=1 part=touches
escher-session commands answer contracts (snapshot, screenshot, actions answer, graph answer, layout answer, coverage answer, help): their writer was split by family; every written key, order and byte is unchanged, so a row stating those answers should still hold.
@task_item n=2 part=touches
The actions line contracts (content, nodes, stale): its node writer moved into its own family module with output unchanged.
@task_item n=3 part=touches
The UI graph line content contract: its node and state writers moved into their own family module with output unchanged.
@task_item n=4 part=touches
The coverage line content contract: its writer stays in the parent module, unchanged.
@task_item n=5 part=touches
The escher-session session error object remedy contract and the stderr session-error and usage-error contracts: the session error writer stays in the parent, unchanged.
@task_item n=6 part=touches
The escher-session mcp role results (help, screenshot, snapshot, other tool results, Err ending) and the stdout command line and mcp line contracts: the answers they carry come from the split writer, bytes unchanged.
@task_item n=7 part=touches
The driver screenshot refusals contract (fonts-unavailable) and the driver screenshot fonts-unavailable contract: refusals are still written from their causes' fixed strings in the parent module.
@task_item n=8 part=touches
Driver command line output read by agents, and escher-session is read by an agent, not a person: the decisions over the output format the split writer produces; no output changed.
@task_item n=9 part=touches
No colour and no terminal test on escher-session: the split adds no colour, print or log site (census read empty).
@task_item n=10 part=touches
The snapshot select-list row bounds and the select accessibility snapshot contracts (chosen option, highlighted row, open attribute, open select): the node writer they rely on moved to the snapshot family module, output unchanged.
@task_item n=11 part=touches
actions decided by the verbs' own checks: the actions answer writer moved, the deciding checks did not.
@task_item n=12 part=touches
Exit statuses read from the schema's table, and the escher-session exit status and exit code contracts 0 to 3: unchanged, the schema and its table were not touched.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth, border radius, iconography and brand rows: the session reaches no engine styling, font, paint, hit-testing or cursor behaviour.
@task_item n=2 part=untouched
Motion rows and the driver verbs and virtual clock contracts (advance, click, press, type): no verb behaviour, clock or scroll changed.
@task_item n=3 part=untouched
Select list behaviour and select-list events (open, close, choose, highlight move): no engine overlay or event changed.
@task_item n=4 part=untouched
escher-session tasks (counter, timer, crud, cells, flight-booker, load, unknown task): no task added, removed or reshaped.
@task_item n=5 part=untouched
Shell provider, dioxus-native configuration, JS window and MountedData contracts, coverage check files read bound and refusal, WPT runner output and colours, and the web-spa surface: none reached.
