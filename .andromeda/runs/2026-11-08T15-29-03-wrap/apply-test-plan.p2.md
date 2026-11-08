@apply_row n=1 ref_file=fanout-test-plan--cascade-g3.md ref_n=3 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g3.md ref_n=5 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g3.md ref_n=6 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g3.md ref_n=7 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g3.md ref_n=8 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=1 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-g3.md ref_n=9 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan--cascade-g3.md ref_n=10 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan--cascade-g3.md ref_n=11 v=applied edit=E10
@apply_row n=11 ref_file=fanout-test-plan--cascade-g3.md ref_n=12 v=applied edit=E11
@apply_row n=12 ref_file=fanout-test-plan--cascade-g3.md ref_n=13 v=applied edit=E12
@apply_row n=13 ref_file=fanout-test-plan--cascade-g3.md ref_n=14 v=applied edit=E13
@apply_row n=14 ref_file=fanout-test-plan--cascade-g3.md ref_n=15 v=applied edit=E14
@apply_row n=15 ref_file=fanout-test-plan--cascade-g4.md ref_n=1 v=applied edit=E15
@apply_row n=16 ref_file=fanout-test-plan--cascade-g4.md ref_n=2 v=applied edit=E16
@apply_row n=17 ref_file=fanout-test-plan--cascade-g4.md ref_n=3 v=applied edit=E17
@apply_row n=18 ref_file=fanout-test-plan--cascade-g4.md ref_n=4 v=applied edit=E18
@apply_row n=19 ref_file=fanout-test-plan--cascade-g4.md ref_n=5 v=applied edit=E19
@apply_row n=20 ref_file=fanout-test-plan--cascade-g4.md ref_n=6 v=applied edit=E20

@fact id=fct-klxgwgcy expect_rev=3
old: thirteen verbs in a stated order
new: fourteen verbs in a stated order
old: the nine instance-level verbs
new: the ten instance-level verbs
old: `graph` after `lint` (no argument; fields `nodes` and `states`), then `start`
new: `graph` after `lint` (no argument; fields `nodes` and `states`), `actions` after `graph` (no argument; one field `nodes` of `FieldKind::ActionNodes`), then `start`
old: the field kinds, twelve (
new: the field kinds, thirteen (
old: `GraphStates` (`graph-states`))
new: `GraphStates` (`graph-states`), then `ActionNodes` (`action-nodes`))
old: `cli_commands` (16)
new: `cli_commands` (17)
old: the verb table's thirteen verbs, twelve field kinds and fourteen node keys at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: the verb table's fourteen verbs, thirteen field kinds and fourteen node keys at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-5zhbhdnd file=unit-test-strategy gid=new:escher-driver-actions-read kind=procedure subject=unit as=n1
- **escher-driver (the `actions` read):** actions.rs holds two unit tests in its inline `mod tests` — `every_call_named_for_a_target_verb_validates`, every call `call_naming` names for a target verb validating, and `an_id_past_its_bound_reads_malformed`, an id past its bound reading `malformed` (packages/escher-driver/src/actions.rs:142-149; packages/escher-driver/src/actions.rs:151-157); escher-driver's unit tests read 102 — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n1 file=unit-test-strategy gid=new:escher-driver-two-nodes-fields kind=procedure subject=unit as=n2
- **escher-driver (the two `nodes` fields):** schema_tests.rs keys its field-kind table per verb (`graph.nodes`, `actions.nodes`), the two verbs' `nodes` fields differing in kind, and pins the `actions` help word for word among its 12 `amended` rows — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-fghufsse expect_rev=2
old: with `VERBS` at 13, causes at 8 and `describe`'s count row `[13, 14, 12, 3, 8, 4, 15]`
new: with `VERBS` at 14, causes at 8 and `describe`'s count row `[14, 14, 12, 3, 8, 4, 15]`
old: `VERBS` and the count row at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: `VERBS` and the count row at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-zot44d7e expect_rev=2
old: `the_listing_is_the_twelve_verbs_of_the_table` asserts the thirteen verbs of the table in `VERBS`' order, `graph` after `lint`,
new: `the_listing_is_the_fourteen_verbs_of_the_table` asserts the fourteen verbs of the table in `VERBS`' order, `graph` after `lint` and `actions` after `graph`,
old: the thirteen verbs at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: the fourteen verbs and the test's name at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-3ferhyj7 expect_rev=2
old: which speaks the thirteen tools, answering
new: which speaks the fourteen tools, `actions` after `graph`, answering

@fact after=fct-3cpa7acl file=unit-test-strategy gid=new:dioxus-native-dom-actionable-reading kind=procedure subject=unit
- **dioxus-native-dom (the actionable reading):** `actionable::tests` holds 8 tests, four of them pinning the one actionable rule both readers read — `a_listener_removed_on_a_re_render_is_read_by_neither_reader`, a listener removed on a re-render read by neither `unkeyed_actionable` nor `actionable_nodes`, which also asserts that `data-dioxus-id` outlives the listener, the in-tree witness that a reading of the mark would still count it; `a_selects_options_and_group_are_listed_with_the_select_as_owner`; `a_positional_id_reads_unkeyed_and_an_author_key_keyed`; and `an_inert_or_hidden_element_is_not_listed` (packages/dioxus-native-dom/src/actionable.rs:324-374; packages/dioxus-native-dom/src/actionable.rs:376-398; packages/dioxus-native-dom/src/actionable.rs:400-418; packages/dioxus-native-dom/src/actionable.rs:420-432) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-t7rg232i file=integration-test-strategy gid=new:session-load-in-flight-without-wait kind=procedure subject=integration
- **Session ↔ held instance, a load in flight without the wait:** `stand_actions` puts a fetch in flight with `harness.click("#load-fetch")`, which waits on nothing, where a driver `click` waits out `LOAD_WAIT` — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=fct-csattoyz file=integration-test-strategy gid=new:session-what-can-be-done-here kind=procedure subject=integration as=n3
- **Session ↔ held instance, what can be done here:** `stand_actions` (8) holds, over the six served screens in both layout modes, each screen's listed actionable nodes with the table's verbs and their counts; every listed cause on the four lean tasks equal to what `Session::run` returns, 3 controls reading `disabled`; the stated Cells and load-page causes; the flight booker's two options reading `covered` with owner `flight-type` until the list opens; `crud-person-14` reading `off-screen` until a `scroll`; the fixtures reading `covered`, unkeyed and `fonts-unavailable`; a read changing nothing and holding no typed text or mask; and the whole Cells answer within the answer bound (tests/blitz-tests/tests/stand_actions.rs:164-560) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n3 file=integration-test-strategy gid=new:cells-actions-answer-size kind=procedure subject=integration as=n4
- **The Cells actions answer's size:** the Cells `actions` answer measures 106,088 bytes for 678 nodes through the binary (`start cells` · `actions` · `stop`), 461 cells reading `off-screen` for `click`, `type` and `screenshot` and all 678 taking `scroll`; `stand_actions`' `the_whole_cells_answer_is_within_the_answer_bound` asserts the answer exceeds 100,000 bytes and stays within 1,048,576, in both layout modes (tests/blitz-tests/tests/stand_actions.rs:546-560) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact after=@n4 file=integration-test-strategy gid=new:kept-actions-step-replay-owed kind=procedure subject=integration
- **A kept actions step's replay, owed:** no test exercises the replay of a kept `actions` step — `Check::replay` runs every instance-verb step through `Session::run`, so it holds by construction, unmeasured; the replay test is owed, a CARRY on «Handler coverage» — as stated at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md, Addendum 1

@fact after=fct-eja3jr5q file=integration-test-strategy gid=new:actions-answer-holds-no-value kind=procedure subject=integration
- **The actions answer holds no value:** an `actions` answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value — a sentinel typed into `fixtures::password_labelled` occurs 0 times in it, `MASKED_VALUE` 0 times, and no `NodeId` field is written (`stand_actions`) — and an argument to `actions` is refused `malformed` (`stand_actions`, `cli_commands`, `mcp_server`, `stand_mcp_tools`) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-5t64ong4 expect_rev=1
old: records `verb` alone (tests/blitz-tests/tests/stand_mcp_tools.rs:772-867) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: records `verb` alone (tests/blitz-tests/tests/stand_mcp_tools.rs:772-867); its body is the shared helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`, which `an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone` runs for an `actions` tool call (tests/blitz-tests/tests/stand_mcp_tools.rs:783-792) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the shared helper at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-adkwyvky expect_rev=2
old: and each also drives one `lint`, one `snapshot` part read and one `graph` call — `host_log` eight calls and 8 command-span lines, `mcp_log` eleven tool calls and 9 `command` lines — still reading 0 needles on stderr at `trace`, the graph's needles (each served component and registered state name, `common::graph_needles`; examples/seven_guis/tests/common/mod.rs:425-452) among them, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: and each also drives one `lint`, one `snapshot` part read, one `graph` call and one `actions` call — `host_log` nine calls and 9 command-span lines, `mcp_log` twelve tool calls and 10 `command` lines — still reading 0 needles on stderr at `trace`, the graph's needles (each served component and registered state name, `common::graph_needles`; examples/seven_guis/tests/common/mod.rs:425-452) and the actions needles (the CRUD screen's listed author-key ids, `common::actions_needles`; examples/seven_guis/tests/common/mod.rs:454-476) among them, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-csryo2zo expect_rev=3
old: `cli_commands` (16)
new: `cli_commands` (17)
old: each of the nine instance-level verbs is accepted
new: each of the ten instance-level verbs is accepted

@fact after=fct-2oltbokg file=e2e-test-strategy gid=new:command-line-actionable-nodes kind=procedure subject=e2e
- **Command line, the actionable nodes:** `cli_commands`' `actions_answers_one_line_of_the_actionable_nodes` runs `actions` as a process against a started host and reads one JSON line, and an argument to `actions` is refused, status `1` (examples/seven_guis/tests/cli_commands.rs:495-516) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-gtzxj2kr expect_rev=2
old: `mcp_server` (7)
new: `mcp_server` (8)
old: thirteen, `graph` after `lint` —
new: fourteen, `graph` after `lint` and `actions` after `graph` —
old: `stand_mcp_tools` (6, its sixth
new: `stand_mcp_tools` (8, its sixth

@fact after=fct-gtzxj2kr file=e2e-test-strategy gid=new:mcp-actionable-nodes kind=procedure subject=e2e
- **MCP, the actionable nodes:** `mcp_server`'s `actions` tool call answers its line of the actionable nodes, and an argument to `actions` reads `isError` (examples/seven_guis/tests/mcp_server.rs:363-388); in process, `stand_mcp_tools`' `actions` tool call parents one `command` span that records `verb` alone (§5) — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-qbzx34mx expect_rev=1
old: and `stand_contrast`'s `the_load_page_meets_its_ratios`
new: and `stand_contrast`'s `the_load_page_meets_its_ratios`, and read by `stand_actions`, its fetch put in flight through the harness (§5) to read `load-fetch`'s stated cause

@fact after=fct-a24rqe7v as=n5
| Actionable counts at boot (stand tasks) | `ACTIONABLE_AT_BOOT: [(&str, usize); 6]` in the stand checks' shared module, the actionable node counts at boot — counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md; read by `stand_actions` | (tests/blitz-tests/tests/common/mod.rs:93-102) |

@pattern id=tp2-01
fixed: thirteen verbs in a stated order
@pattern id=tp2-02
fixed: the nine instance-level verbs
@pattern id=tp2-03
fixed: the field kinds, twelve
@pattern id=tp2-04
fixed: `cli_commands` (16)
@pattern id=tp2-05
fixed: `VERBS` at 13
@pattern id=tp2-06
fixed: `[13, 14, 12, 3, 8, 4, 15]`
@pattern id=tp2-07
fixed: the_listing_is_the_twelve_verbs_of_the_table
@pattern id=tp2-08
fixed: speaks the thirteen tools
@pattern id=tp2-09
fixed: `host_log` eight calls and 8 command-span lines
@pattern id=tp2-10
fixed: `mcp_log` eleven tool calls and 9 `command` lines
@pattern id=tp2-11
fixed: `mcp_server` (7)
@pattern id=tp2-12
fixed: thirteen, `graph` after `lint`
@pattern id=tp2-13
fixed: `stand_mcp_tools` (6, its sixth
