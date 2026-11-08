@apply_row n=1 ref_file=fanout-obs-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-obs-plan--cascade-g1.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-obs-plan--cascade-g1.md ref_n=2 v=applied edit=E3
@apply_row n=4 ref_file=fanout-obs-plan--cascade-g1.md ref_n=3 v=applied edit=E4
@apply_row n=5 ref_file=fanout-obs-plan--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-obs-plan--cascade-g1.md to_n=3
@apply_row n=6 ref_file=fanout-obs-plan--cascade-g1.md ref_n=5 v=applied edit=E5
@apply_row n=7 ref_file=fanout-obs-plan--cascade-g1.md ref_n=6 v=applied edit=E6
@apply_row n=8 ref_file=fanout-obs-plan--cascade-g1.md ref_n=7 v=applied edit=E7
@apply_row n=9 ref_file=fanout-obs-plan--cascade-g2.md ref_n=1 v=applied edit=E8
@apply_row n=10 ref_file=fanout-obs-plan--cascade-g2.md ref_n=2 v=applied edit=E9
@apply_row n=11 ref_file=fanout-obs-plan--cascade-g2.md ref_n=3 v=applied edit=E10
@apply_row n=12 ref_file=fanout-obs-plan--cascade-g2.md ref_n=4 v=applied edit=E11
@apply_row n=13 ref_file=fanout-obs-plan--cascade-g2.md ref_n=5 v=applied edit=E12
@fact id=fct-icom2cq7 expect_rev=1
old: its eighth, `lint`, and `Session::with_time` with them
new: its eighth, `lint`, its ninth, `graph`, and `Session::with_time` with them
old: never logged or fielded (packages/escher-driver/src/json.rs:450-454)
new: never logged or fielded (packages/escher-driver/src/json.rs:450-454), and `Outcome::Graphed { graph }` is written by `to_json` alone as `{"nodes":[…],"states":[…]}` — ids, component names, registered state and handler event names, closed flag words and booleans, never a value; a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded

@fact id=fct-zuu6oxcq expect_rev=1
old: an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too
new: an accepted `lint`, an accepted `graph` (`Outcome::Graphed`, no step run; `stand_act_spans` one `graph` call per layout mode) and a part read (`Outcome::Part`) record `verb` alone too
old: a lint's findings or their count, the session's label
new: a lint's findings or their count, a graph's nodes, states, ids or names, the session's label

@fact id=fct-mp6exhtv expect_rev=1
old: one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint` —
new: one of its thirteen, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, `lint` and `graph` —
old: tests/blitz-tests/tests/stand_mcp_tools.rs:709-769); no field added
new: tests/blitz-tests/tests/stand_mcp_tools.rs:709-769); a `graph` tool call ends `accepted` (its reply the JSON line `{"nodes":[…],"states":[…]}`, one text block), an argument to it refused `malformed`, and parents exactly one `command` child recording `verb` alone, measured in process in both layout modes (`stand_mcp_tools::a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone`; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md); no field added

@fact id=fct-gwvelgmp expect_rev=1
old: the verb table's twelve words (eight instance-level;
new: the verb table's thirteen words (nine instance-level;
old: (`serve cells`, `snapshot --part 2`), 0 id and 0 name needles under both builds;
new: (`serve cells`, `snapshot --part 2`), 0 id and 0 name needles under both builds; a graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name — `host_log` and `mcp_log` each drive one `graph` call at `trace` with 0 id, name and graph needles under both builds (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md);
old: (the table's word, one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint`)
new: (the table's word, one of its thirteen, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, `lint` and `graph`)

@fact after=fct-iekb43j4 as=n1
- The pipe's `VERBS` (`scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`) hold thirteen words, `graph` among them, and the contract tests' fake server answers `graph` with `{"nodes":[],"states":[]}`; `graph` is no counted verb and is not required of a cold agent, so it adds no verdict field and no `capability-unused` reason (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-x7v6upkc as=n2
- **dioxus-native-dom (UI graph)**
  - no site added: dioxus-native-dom's modules `graph` (gated `accessibility`) and `signal_registry` and the check `stand_graph` hold no print, log site, subscriber or env read, the silence census reading 0 over them; `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}`, never logged or fielded, escher-driver keeping its 2 `info_span!` sites (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact after=@n2 as=n3
- **dioxus-native-dom (bridge dispatch)**
  - no site added: `DioxusEventHandler::handle_event` sets the `SignalRegistry`'s dispatch around `runtime.handle_event` and clears it after only where a registry context stands at the root scope, adding no log site; a page with no registered state reaches no registry and dispatches as before (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact id=fct-hlttx6z4 expect_rev=1
old: each line `verb` alone). A shot's image
new: each line `verb` alone), and nine with the `graph` verb, one `graph` call per layout mode, its `command` line recording `verb` alone. A shot's image
old: the part read at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).
new: the part read at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md). `host_log` and `mcp_log` each make one `graph` call and read 0 id, name and graph needles (`graph_needles`, the stand's component and registered state names) at `trace` under both builds (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-hlttx6z4 as=n4
- The UI graph's answer (`graph`, `Outcome::Graphed`) carries stable ids, component names, registered state names, handler event names, the closed flag words `undeclared` and `unconfirmed`, and booleans only, never a state's value — a sentinel typed into `crud-name` occurs 0 times in it — a boundary widening, ratified; its log reading stands in the row above (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-pkczt72m as=n5
- The snapshot's `clickable` reading (`NodeState.clickable`, the text token `clickable` after `focused`, a written node's `"clickable"` key) is a closed bool — true where an element's current listeners hold `click` and it is no `ElementData::is_disabled` form control — carrying no page text (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-myhl4hwq as=n6
- The state registry `SignalRegistry` (dioxus-native-dom, behind `use_graph_state`) holds in process memory each registered state's name, its declarations, its write records and the standing dispatch chain, and no value of a state and no `NodeId` beyond that chain; its `Debug` prints counts alone, and it holds no log site, print, env read or `unwrap` on app input (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-g4yyfogl expect_rev=1
old: diffs and a lint's findings at rest, and no log
new: diffs, a lint's findings and a `graph` answer's ids, component names, registered state names, handler event names, closed flag words and booleans at rest, and no log

@pattern id=obs-01
fixed: one of its twelve
@pattern id=obs-02
fixed: the verb table's twelve words
@pattern id=obs-03
fixed: eight instance-level
