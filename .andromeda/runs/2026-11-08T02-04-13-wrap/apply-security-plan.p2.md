@apply_row n=1 ref_file=fanout-security-plan--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g2.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-security-plan--cascade-g2.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-security-plan--cascade-g2.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-security-plan--cascade-g2.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-security-plan--cascade-g2.md ref_n=6 v=applied edit=E6
@apply_row n=7 ref_file=fanout-security-plan--cascade-g2.md ref_n=7 v=applied edit=E7
@apply_row n=8 ref_file=fanout-security-plan--cascade-g2.md ref_n=8 v=applied edit=E8
@apply_row n=9 ref_file=fanout-security-plan--cascade-g2.md ref_n=9 v=applied edit=E9
@apply_row n=10 ref_file=fanout-security-plan--cascade-g2.md ref_n=10 v=applied edit=E10
@apply_row n=11 ref_file=fanout-security-plan--cascade-g2.md ref_n=11 v=applied edit=E11
@apply_row n=12 ref_file=fanout-security-plan--cascade-g2.md ref_n=12 v=applied edit=E13
@apply_row n=13 ref_file=fanout-security-plan--cascade-g2.md ref_n=13 v=applied edit=E12
@apply_row n=14 ref_file=fanout-security-plan--cascade-g3.md ref_n=2 v=applied edit=E14
@apply_row n=15 ref_file=fanout-security-plan--cascade-g3.md ref_n=3 v=applied edit=E15
@apply_row n=16 ref_file=fanout-security-plan--cascade-g4.md ref_n=1 v=applied edit=E16
@apply_row n=17 ref_file=fanout-security-plan--cascade-g4.md ref_n=2 v=applied edit=E17
@apply_row n=18 ref_file=fanout-security-plan--cascade-g4.md ref_n=3 v=applied edit=E18
@apply_row n=19 ref_file=fanout-security-plan--cascade-g4.md ref_n=4 v=applied edit=E19
@apply_row n=20 ref_file=fanout-security-plan--cascade-g4.md ref_n=5 v=applied edit=E20
@fact id=fct-tbmm2cwc expect_rev=1
old: `tool` one of the twelve verbs
new: `tool` one of the thirteen verbs

@fact after=fct-tbmm2cwc as=n1
- **Boundary:** Cold-agent check (cold-agent.sh, the agent-written check)
  - **What to validate:** a `graph` step
  - **How (stack-specific):** `graph` is one of the pipe's thirteen `VERBS`, so `valid_step` admits a `graph` step like any other verb and the replay's server validates it again; no task requires it of a cold agent, so the verdict's `capability-unused` set is unchanged; the test's fake server answers it with an empty graph, `{"nodes":[],"states":[]}` (scripts/cold-agent.sh:53; .github/scripts/test_cold_agent.py:142-143) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-7jixhrbw expect_rev=1
old: the snapshot's text, the diff and a lint's findings, where until that chunk no reply carried any of them
new: the snapshot's text, the diff, a lint's findings and the UI graph — an accepted `graph`'s line `{"nodes":[…],"states":[…]}`, holding ids (an option's or an optgroup's owner among them), component names, registered state names, handler event names, the closed flag words `undeclared` · `unconfirmed` and booleans, never a control value, typed text or `NodeId`, a boundary widening, ratified, under the unchanged 1,048,576-byte bound (the Driver command schema's `graph` row) —, where until that chunk no reply carried any of them
old: `NodeState` holding no field for it and `NODE_FIELDS` thirteen rows
new: `NodeState` holding no field for it and `NODE_FIELDS` fourteen rows, `clickable` after `focused` (the `clickable` row below, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact after=fct-7jixhrbw as=n2
- **Boundary:** Driver session socket (escher-driver)
  - **What to validate:** a node's `clickable` reading
  - **How (stack-specific):** `NodeState.clickable`, a closed `bool` after `focused`, true where the element's current listeners hold `click` and it is no form control `ElementData::is_disabled` reads disabled (the `disabled` row); the snapshot's text writes the bare word `clickable` after `focused` only when true, a diff node writes `clickable` always, beside `focused`, and a changed node's `differs` can name it (`Reading::Clickable`); a closed value inside the ratified carrier, so no widening under the founder's rule (the socket row), never an attribute's value or a listener's code (packages/dioxus-native-dom/src/snapshot_text.rs:128-130) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-ifbk2u2m expect_rev=1
old: a verb outside the eight instance verbs
new: a verb outside the nine instance verbs

@fact id=fct-v4gvn7w2 expect_rev=1
old: `tool` one of the eight instance verbs
new: `tool` one of the nine instance verbs

@fact id=fct-awfxp663 expect_rev=1
old: closed `'static` table of twelve verbs in three levels
new: closed `'static` table of thirteen verbs in three levels
old: a widening of this validated surface onto every ratified carrier, a ratified one) and,
new: a widening of this validated surface onto every ratified carrier, a ratified one), a ninth, `graph` (no argument, running no step; the `graph` row below) and,
old: (a `Vec<Reading>` of a closed eleven-variant enum)
new: (a `Vec<Reading>` of a closed twelve-variant enum, `Reading::Clickable` among them)

@fact after=fct-awfxp663 as=n3
- **Boundary:** Driver command schema (escher-driver)
  - **What to validate:** the `graph` verb
  - **How (stack-specific):** The ninth instance-level verb, `schema::GRAPH` after `LINT`, taking no argument: `validate` reads `("graph", [])` as `Command::Graph` and refuses any argument `malformed` with nothing run, on the socket, the command line, MCP and a kept check alike; its result fields are `nodes` (`FieldKind::GraphNodes`, `graph-nodes`) and `states` (`FieldKind::GraphStates`, `graph-states`), the eleventh and twelfth field kinds; it runs no step, and its `Outcome::Graphed { graph: UiGraph }` feeds the session's record of ids from the graph's node ids and is written by `to_json` alone as `{"nodes":[…],"states":[…]}`; the answer holds ids, owner ids, component names, registered state names, handler event names, the closed flag words `undeclared` · `unconfirmed` and booleans, never a control value, typed text or `NodeId` — a sentinel typed into `crud-name` occurs 0 times in it; its `command` span records `verb` alone, and no log site, print or span field carries the graph — a boundary widening of this validated surface onto every ratified carrier, ratified (packages/escher-driver/src/schema.rs:382-427; tests/blitz-tests/tests/stand_graph.rs:341-379) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-gucmzkph expect_rev=1
old: (examples/seven_guis/tests/cli_commands.rs:375-418); a first word
new: (examples/seven_guis/tests/cli_commands.rs:375-418); `graph` with no other argument than one `--session <dir>` answers the UI graph's one line of JSON, `{"nodes":[…],"states":[…]}`, on stdout, status 0, by the ratified crossing, and an argument to it is refused `malformed`, status 1 (examples/seven_guis/tests/cli_commands.rs:467-492); a first word
old: `screenshot` after `scroll` since 2026-10-12-headless-screenshot)
new: `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `graph` after `lint`)
old: a shot's base64 PNG and a lint's findings by the ratified crossing
new: a shot's base64 PNG, a lint's findings and the UI graph by the ratified crossing

@fact id=fct-f4eyvu3v expect_rev=1
old: `tools/list` the twelve verbs, `help` last (`screenshot` and `lint` among them, a `lint` tool call's result its JSON line `{"findings":[…]}`)
new: `tools/list` the thirteen verbs, `help` last (`screenshot`, `lint` and `graph`, listed after `lint`, among them, a `lint` tool call's result its JSON line `{"findings":[…]}` and a `graph` tool call's the text block of its JSON line `{"nodes":[…],"states":[…]}`, its `tool_call` span `tool` = `graph` parenting one `command` span recording `verb` alone)

@fact id=fct-mbi5iago expect_rev=1
old: `ClickTarget::Disabled` and `handle_click`'s walk gate read it
new: `ClickTarget::Disabled`, `handle_click`'s walk gate and the snapshot's `NodeState.clickable` (true only for an element whose current listeners hold `click` and that is no disabled form control) read it

@fact id=fct-d2yejoup expect_rev=3
old: and `VERBS` holding twelve;
new: and `VERBS` holding thirteen; under the same feature `DioxusDocument::graph` is a further reader of the id — read-only, one `GraphNode` per snapshot node in pre-order holding its id, its `owner` (the stable id of `BaseDocument::select_of`, for an option or an optgroup), its component, its handlers and the states it reads, and no `NodeId` — crossing as the `graph` verb's answer on every ratified carrier, a ratified boundary widening (the Driver command schema's `graph` row);

@fact after=fct-d2yejoup as=n4
- **Boundary:** Markup attributes
  - **What to validate:** the reserved virtual-node id form `{owner}///{key}`
  - **How (stack-specific):** The id grammar reserves a fifth form for a virtual node, `{owner}///{key}` (`VIRTUAL_KEY_SEPARATOR`, `"///"`); the public `virtual_node_id(owner, key)` spells it and returns `None` for an empty or `/`-bearing key; no walk calls it, so no served id holds `///` and the form equals no id of its document, and the anchored path stays the only served id holding `//`; a stable-id contract change, ratified (packages/dioxus-native-dom/src/element_id.rs:42-47; packages/dioxus-native-dom/src/element_id.rs:753-775; tests/blitz-tests/tests/stand_graph.rs:282-339) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-omhuceqe as=n5
- **Boundary:** Dioxus bridge (dioxus-native-dom)
  - **What to validate:** the event dispatch (`DioxusEventHandler::handle_event`)
  - **How (stack-specific):** On every Dioxus document the handler sets the `SignalRegistry`'s dispatch around `runtime.handle_event` and clears it after, only where `runtime.has_context::<SignalRegistry>(ROOT)` holds; a page with no registered state reaches no registry and dispatches as before, the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` untouched; it adds no input class; pinned by `bridge_tests::a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` (packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322) — an engine change for every Dioxus document, inert without a registry — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n5 as=n6
- **Boundary:** Dioxus bridge (dioxus-native-dom)
  - **What to validate:** the signal registry (`dioxus_native_dom::use_graph_state`, `SignalRegistry`, `DeclaredWrite`)
  - **How (stack-specific):** An in-process app API taking dioxus-core types alone, its first registering hook providing the `SignalRegistry` context at `ScopeId::ROOT`; a `DeclaredWrite` holds an element's stable id or a prefix ending in `*` and an event without `on`; each entry leaves with its scope (`use_drop`), the registry's `Debug` prints counts alone, it holds no `NodeId` beyond the dispatch chain, and it has no log site, print, env read or `unwrap` on app input; no input class enters from outside the process (packages/dioxus-native-dom/src/signal_registry.rs:20-29; packages/dioxus-native-dom/src/signal_registry.rs:47-63; packages/dioxus-native-dom/src/signal_registry.rs:112-121) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-cl72c3yo expect_rev=1
old: diffs and a lint's findings, never a pixel
new: diffs, a lint's findings and a `graph` answer's UI graph (ids, owner ids, component names, registered state names, handler event names, the closed flag words and booleans, never a control value or typed text; a boundary widening, ratified), never a pixel

@fact id=fct-svo6bgmr expect_rev=1
old: as MCP tools — twelve:
new: as MCP tools — thirteen:
old: and the twelfth, `lint` — over its own stdin
new: the twelfth, `lint`, and the thirteenth, `graph`, listed after `lint`, taking no argument and answered as one text block — over its own stdin

@fact id=fct-brzrm7cu expect_rev=1
old: none of its eight instance-level verbs
new: none of its nine instance-level verbs
old: the eighth, `lint`, taking no argument and answering `findings`, running no step and settling nothing, so it moves none either;
new: the eighth, `lint`, taking no argument and answering `findings`, running no step and settling nothing, so it moves none either, and the ninth, `graph`, taking no argument and answering `nodes` and `states`, running no step and settling nothing, so it moves none either;
old: and `snapshot`, `screenshot` and `lint` wait on none
new: and `snapshot`, `screenshot`, `lint` and `graph` wait on none

@fact after=fct-t6pddbhw as=n7
- **Control:** The `graph` answer's size (escher-driver)
  - **Configuration:** `graph` takes no argument and its answer is unpaged — no part argument, which would widen the crossing — so a screen's whole graph is one answer under the 1,048,576-byte answer bound: Cells' answer reads 293,869 bytes, held within the bound in both layout modes by `stand_graph`'s `the_whole_cells_answer_is_within_the_answer_bound` (tests/blitz-tests/tests/stand_graph.rs:381-394); its size the stated limit «An unpaged graph» in `docs/capabilities.md`, owned by residual `res-7cc5bi4k` (target: next) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
  - **Stack reference:** escher-driver

@fact id=fct-bzel2phc expect_rev=1
old: name the state words and say the two limits
new: name the state words and `clickable` and say the two limits
old: ; `Display` prints `{name}: {remedy}`
new: ; a `graph` call with an argument is refused `malformed` with nothing run, no cause added, the set staying eight; `Display` prints `{name}: {remedy}`
@pattern id=sec2-01
fixed: one of the twelve verbs
@pattern id=sec2-02
fixed: the eight instance verbs
@pattern id=sec2-03
fixed: table of twelve verbs
@pattern id=sec2-04
fixed: eleven-variant enum
@pattern id=sec2-05
fixed: `tools/list` the twelve verbs
@pattern id=sec2-06
fixed: `VERBS` holding twelve
@pattern id=sec2-07
fixed: `NODE_FIELDS` thirteen rows
@pattern id=sec2-08
fixed: as MCP tools — twelve
@pattern id=sec2-09
fixed: eight instance-level verbs
@pattern id=sec2-10
fixed: `screenshot` and `lint` wait on none
