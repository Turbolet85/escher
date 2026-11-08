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
@apply_row n=12 ref_file=fanout-security-plan--cascade-g2.md ref_n=12 v=applied edit=E12
@apply_row n=13 ref_file=fanout-security-plan--cascade-g2.md ref_n=13 v=applied edit=E13
@apply_row n=14 ref_file=fanout-security-plan--cascade-g2.md ref_n=14 v=folded_into to_file=fanout-security-plan--cascade-g2.md to_n=13
@apply_row n=15 ref_file=fanout-security-plan--cascade-g3.md ref_n=1 v=applied edit=E14
@apply_row n=16 ref_file=fanout-security-plan--cascade-g4.md ref_n=1 v=applied edit=E15
@apply_row n=17 ref_file=fanout-security-plan--cascade-g4.md ref_n=2 v=applied edit=E16
@apply_row n=18 ref_file=fanout-security-plan--cascade-g4.md ref_n=3 v=applied edit=E17
@apply_row n=19 ref_file=fanout-security-plan--cascade-g4.md ref_n=4 v=applied edit=E18
@apply_row n=20 ref_file=fanout-security-plan--cascade-g4.md ref_n=5 v=applied edit=E19
@fact id=fct-tbmm2cwc expect_rev=2
old: `tool` one of the thirteen verbs;
new: `tool` one of the fourteen verbs;

@fact id=fct-pbvi5fsv expect_rev=1
old: `graph` is one of the pipe's thirteen `VERBS`, so `valid_step` admits a `graph` step like any other verb
new: `graph` and, after it, `actions` are among the pipe's fourteen `VERBS`, so `valid_step` admits a `graph` or an `actions` step like any other verb
old: no task requires it of a cold agent
new: no task requires either of a cold agent
old: `{"nodes":[],"states":[]}` (scripts/cold-agent.sh:53; .github/scripts/test_cold_agent.py:142-143) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: `{"nodes":[],"states":[]}`, and answers an `actions` step too (scripts/cold-agent.sh:53; .github/scripts/test_cold_agent.py:142-143; .github/scripts/test_cold_agent.py:144-145) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the `actions` step at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-7jixhrbw expect_rev=2
old: a lint's findings and the UI graph — an accepted `graph`'s line
new: a lint's findings, the UI graph — an accepted `graph`'s line
old: under the unchanged 1,048,576-byte bound (the Driver command schema's `graph` row) —,
new: under the unchanged 1,048,576-byte bound (the Driver command schema's `graph` row) — and the actions answer — an accepted `actions`'s line `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, holding ids, owner ids, `keyed` booleans, the table's verb words and closed cause words, never a control value, typed text or `NodeId`, a boundary widening, ratified, its `keyed` field PROVISIONAL, under the same 1,048,576-byte bound (the Driver command schema's `actions` row) —,

@fact id=fct-ifbk2u2m expect_rev=2
old: a verb outside the nine instance verbs, a `lint` step kept like every other with its answer whole
new: a verb outside the ten instance verbs, a `lint` or `actions` step kept like every other with its answer whole

@fact id=fct-v4gvn7w2 expect_rev=2
old: `tool` one of the nine instance verbs
new: `tool` one of the ten instance verbs

@fact id=fct-awfxp663 expect_rev=2
old: a closed `'static` table of thirteen verbs
new: a closed `'static` table of fourteen verbs
old: a ninth, `graph` (no argument, running no step; the `graph` row below) and,
new: a ninth, `graph` (no argument, running no step; the `graph` row below), a tenth, `actions` (no argument, running no step; the `actions` row below) and,

@fact after=fct-2njwkumb as=n1
- **Boundary:** Driver command schema (escher-driver)
  - **What to validate:** the `actions` verb
  - **How (stack-specific):** The tenth instance-level verb, `schema::ACTIONS` after `GRAPH`, taking no argument: `validate` reads `("actions", [])` as `Command::Actions` and refuses any argument `malformed` (unnamed) with nothing run — in process, on the command line over the socket (status 1) and over MCP (`isError`); its one result field is `nodes` (`FieldKind::ActionNodes`, `action-nodes`), the thirteenth field kind; it runs no step, and its `Outcome::Actions { nodes }` is written by `json.rs`'s `action_node` alone as `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, `owner` and `cause` left out when absent, a cause by `Cause::name`; the answer holds ids, owner ids, `keyed` booleans, the table's verb words and closed cause words, never a control value, typed text or `NodeId` — a typed sentinel and `MASKED_VALUE` occur 0 times in it; its `command` span records `verb` alone and, refused, `cause`, and `host_log` and `mcp_log` read 0 of its needles at `trace` — a boundary widening of this validated surface onto every ratified carrier, ratified, the `keyed` field PROVISIONAL (packages/escher-driver/src/schema.rs:429-452; packages/escher-driver/src/json.rs:251-272; tests/blitz-tests/tests/stand_actions.rs:486-544) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n1 as=n2
- **Boundary:** Driver command schema (escher-driver)
  - **What to validate:** a call's target, before the call acts
  - **How (stack-specific):** One crate-private check, `check_target`, answering `Cleared` (a point, a rect or free) or a `Refusal`: `click` and `type` meet `action_point` (`disabled`, then `off-screen`, then `covered`), `scroll` meets none, and `screenshot` with an id meets `in_view`, then an empty crop, then `BaseDocument::can_paint_text` false → `fonts-unavailable`, all before painting; its callers are the four id-taking arms of `Session::execute`, each calling it before it acts, and `Session::actions`, and no arm keeps a check of its own (packages/escher-driver/src/execute.rs:478-526) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n2 as=n3
- **Boundary:** Driver command schema (escher-driver)
  - **What to validate:** the `actions` read's built calls (`Session::actions`)
  - **How (stack-specific):** `Session::actions` reads one snapshot and one `element_ids` map per call; for each listed node and each id-taking instance verb — `target_verbs()`, the table's `Level::Instance` rows with an `ArgKind::Id` argument in table order, no verb name written in the read's code — it builds the call `call_naming` spells, a required non-target argument given a fixed in-bound value by its `ArgKind` (the empty text for `type`'s `text`), and passes it through `validate` and then `check_target` (the target check row above), running nothing; a node that resolves to no element reads `stale`; the read changes nothing — after a drain no `has_changes()`, the snapshot, both focus readings and the diff unchanged, an open list staying open — and adds no `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call (packages/escher-driver/src/actions.rs:40-136) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-gucmzkph expect_rev=2
old: (examples/seven_guis/tests/cli_commands.rs:468-493);
new: (examples/seven_guis/tests/cli_commands.rs:468-493); `actions` with no other argument than one `--session <dir>` answers the actions answer's one line of JSON, `{"nodes":[…]}`, on stdout, status 0, by the ratified crossing, and an argument to it is refused `malformed`, status 1 (examples/seven_guis/tests/cli_commands.rs:495-516);

@fact id=fct-f4eyvu3v expect_rev=2
old: `tools/list` the thirteen verbs, `help` last (`screenshot`, `lint` and `graph`, listed after `lint`, among them, a `lint` tool call's result its JSON line `{"findings":[…]}` and a `graph` tool call's
new: `tools/list` the fourteen verbs, `help` last (`screenshot`, `lint`, `graph`, listed after `lint`, and `actions`, after `graph`, among them, a `lint` tool call's result its JSON line `{"findings":[…]}`, a `graph` tool call's
old: parenting one `command` span recording `verb` alone), each `description`
new: parenting one `command` span recording `verb` alone, and an `actions` tool call's the text block of its JSON line `{"nodes":[…]}`, its `tool_call` span `tool` = `actions` parenting one `command` span recording `verb` alone, an argument to it refused with `isError`), each `description`

@fact after=fct-d2yejoup as=n4
- **Boundary:** Markup attributes
  - **What to validate:** the actionable rule (`Readings`) and `DioxusDocument::actionable_nodes`
  - **How (stack-specific):** Under the `accessibility` feature the actionable rule is one crate-private reading, `Readings`: focusable (`Node::is_focussable()`), an interactive role (the 27-role `is_interactive`), or current listeners holding an event other than `mounted`, read from `element_facts`, the vdom walk's exact reading; `unkeyed_actionable` and the read-only `DioxusDocument::actionable_nodes` both read it; `actionable_nodes` returns, in snapshot pre-order over the UI graph's nodes, an `ActionableNode { id, owner, keyed }` for each node the rule holds for or whose graph `owner` is `Some` — the stable id, the owner's stable id and `keyed` true where the id holds no `/` — ids, owner and keyed only, no `NodeId`; a listener removed on a re-render is read by neither (packages/dioxus-native-dom/src/actionable.rs:49-81; packages/dioxus-native-dom/src/actionable.rs:118-146; packages/dioxus-native-dom/src/actionable.rs:324-374) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-d2yejoup expect_rev=4
old: `VERBS` holding thirteen
new: `VERBS` holding fourteen
old: (the Driver command schema's `graph` row);
new: (the Driver command schema's `graph` row); escher-driver's `Session::actions` is a further reader of the id — it takes no argument, reads `actionable_nodes` (the actionable rule row below) and returns `ActionNode`s holding ids, owner ids, `keyed` booleans and the table's verb and closed cause words, no `NodeId` — crossing as the `actions` verb's answer on every ratified carrier, a ratified boundary widening, the `keyed` field PROVISIONAL (the Driver command schema's `actions` row);
old: no driver, CLI or MCP command yet (whether it becomes one is owned by a route entry)
new: no driver, CLI or MCP command of its own, the actionable reading crossing through `Session::actions` (below)
old: on `Session::lint`, which takes no argument (the operator
new: on `Session::lint`, which takes no argument, and on `Session::actions`, which takes no argument and records every id it reads (the operator

@fact id=fct-pnrbnsjg expect_rev=1
old: `is_select`'s `execute.rs` `locate` alone
new: `is_select`'s caller `execute.rs`'s crate-private `target_of` alone (packages/escher-driver/src/execute.rs:443), which `locate` ends in and `Session::actions` also calls

@fact id=fct-cl72c3yo expect_rev=2
old: a lint's findings and a `graph` answer's UI graph
new: a lint's findings, a `graph` answer's UI graph
old: a boundary widening, ratified), never a pixel
new: a boundary widening, ratified) and an `actions` answer (ids, owner ids, `keyed` booleans, the table's verb words and closed cause words, never a control value, typed text or `NodeId`; a boundary widening, ratified, the `keyed` field PROVISIONAL; the replay of a kept `actions` step exercised by no test yet, owed on «Handler coverage»), never a pixel

@fact id=fct-svo6bgmr expect_rev=2
old: MCP tools — thirteen:
new: MCP tools — fourteen:
old: the twelfth, `lint`, and the thirteenth, `graph`, listed after `lint`, taking no argument and answered as one text block
new: the twelfth, `lint`, the thirteenth, `graph`, listed after `lint`, and the fourteenth, `actions`, listed after `graph`, each of the two taking no argument and answered as one text block

@fact id=fct-brzrm7cu expect_rev=2
old: none of its nine instance-level verbs
new: none of its ten instance-level verbs
old: answering `nodes` and `states`, running no step and settling nothing, so it moves none either;
new: answering `nodes` and `states`, running no step and settling nothing, so it moves none either, and the tenth, `actions`, taking no argument and answering `nodes`, running no step and settling nothing, so it moves none either;
old: `snapshot`, `screenshot`, `lint` and `graph` wait on none
new: `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on none

@fact after=fct-s6ul72yw as=n5
- **Control:** The `actions` answer's size (escher-driver)
  - **Configuration:** `actions` takes no argument and its answer is unpaged — no part argument — so a screen's whole answer is one answer under the 1,048,576-byte answer bound: Cells' answer reads 106,088 bytes for 678 nodes through the binary, held within the bound in both layout modes by `stand_actions`' `the_whole_cells_answer_is_within_the_answer_bound` (tests/blitz-tests/tests/stand_actions.rs:546-560); its size the stated limit «An unpaged actions answer» in `docs/capabilities.md`, beside «An unpaged graph», owned by residual `res-7cc5bi4k` (target: next) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
  - **Stack reference:** escher-driver

@fact id=fct-bzel2phc expect_rev=2
old: a `graph` call with an argument is refused `malformed` with nothing run, no cause added, the set staying eight;
new: a `graph` call with an argument is refused `malformed` with nothing run, no cause added, the set staying eight, and so is an `actions` call with an argument, in process, on the command line (status 1) and over MCP (`isError`); `actions` reads each listed node's cause through the executor's own per-verb check, running nothing, so it adds no cause;
@pattern id=sec2-01
fixed: one of the thirteen verbs
@pattern id=sec2-02
fixed: one of the pipe's thirteen `VERBS`
@pattern id=sec2-03
fixed: outside the nine instance verbs
@pattern id=sec2-04
fixed: one of the nine instance verbs
@pattern id=sec2-05
fixed: table of thirteen verbs
@pattern id=sec2-06
fixed: `tools/list` the thirteen verbs
@pattern id=sec2-07
fixed: `VERBS` holding thirteen
@pattern id=sec2-08
fixed: no driver, CLI or MCP command yet
@pattern id=sec2-09
fixed: `execute.rs` `locate` alone
@pattern id=sec2-10
fixed: MCP tools — thirteen
@pattern id=sec2-11
fixed: none of its nine instance-level verbs
@pattern id=sec2-12
fixed: `lint` and `graph` wait on none
