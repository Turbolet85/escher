@apply_row n=1 ref_file=fanout-test-plan--cascade-g5.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g5.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g5.md ref_n=7 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g5.md ref_n=8 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g5.md ref_n=9 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g5.md ref_n=10 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan.md ref_n=10 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan.md ref_n=11 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan.md ref_n=12 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan.md ref_n=13 v=applied edit=E10
@fact id=fct-6hjdlvz3 expect_rev=1
old: `counter-cli.json` (8 lines, 2,929 B) and `counter-mcp.json` (11 lines, 3,720 B)
new: `counter-cli.json` (8 lines) and `counter-mcp.json` (11 lines)

@fact id=fct-iee65fcu expect_rev=1
old: 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/counts.md
new: 53,859 bytes in 740 lines and 6 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-iee65fcu as=n1
| Stand graph answer (Cells) | `graph` on a fresh `start cells`: 293,869 bytes, the whole screen in one answer, `graph` taking no argument — the stated limit «An unpaged graph», owned by residual `res-7cc5bi4k`, target next; held within the answer bound in both layout modes by `stand_graph`'s `the_whole_cells_answer_is_within_the_answer_bound`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md | (tests/blitz-tests/tests/stand_graph.rs:381-394) |

@fact after=fct-qbzx34mx as=n2
| Graph drives (stand tasks) | `GRAPH_DRIVES` in the stand checks' shared module, seven rows of `Drive` steps, drives each stand task's conditional writes once: Cells clicks A0, C0, A0 and leaves its keyed editor once by Enter and once by Tab, so no stand write reads a flag (`undeclared` or `unconfirmed`); read by `stand_graph` | (tests/blitz-tests/tests/common/mod.rs:234-241; tests/blitz-tests/tests/common/mod.rs:243-323) |

@fact id=fct-mrjxzany expect_rev=1
old: — 51 files, `stand_session_recording`
new: — 52 files, `stand_session_recording`
old: `stand_load_page` and `stand_act_popup`
new: `stand_load_page`, `stand_act_popup` and `stand_graph`

@fact id=fct-3ctmcebn expect_rev=1
old: run the 51 `stand_*.rs` files
new: run the 52 `stand_*.rs` files

@fact id=fct-76fzwvj7 expect_rev=1
old: twelve verbs — the eight instance-level ones
new: thirteen verbs — the nine instance-level ones
old: `lint` the eighth, no argument, its accepted answer one line `{"findings":[…]}`, status `0`)
new: `lint` the eighth, no argument, its accepted answer one line `{"findings":[…]}`, status `0`; `graph` the ninth, no argument (an argument refused `malformed`), its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0`)
old: the usage line lists `help` after `stop`
new: the usage line lists `graph` after `lint` and `help` after `stop`

@fact id=fct-3ylhu62k expect_rev=1
old: while `snapshot`, `screenshot` and `lint` run no step
new: while `snapshot`, `screenshot`, `lint` and `graph` run no step
old: the counts `part` and `parts`, and an instance
new: the counts `part` and `parts`, `graph` returns `Outcome::Graphed { graph }` with the screen's `UiGraph`, and an instance
old: the eight instance verbs, a `lint` among them
new: the nine instance verbs, a `lint` and a `graph` among them

@fact id=fct-egnkfdyv expect_rev=1
old: seven_guis `cli_commands` 15
new: seven_guis `cli_commands` 16
old: and parenting its `command` span
new: and parenting its `command` span; the `graph` verb held over the command line by `cli_commands::graph_answers_one_line_of_the_ui_graph` and over MCP by `mcp_server::a_graph_tool_call_answers_its_line_of_the_ui_graph` and `stand_mcp_tools::a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone`
old: · seven_guis' library unit tests 3
new: · `stand_graph` 6 (`Session::run` in both layout modes, the `graph` verb's answer equal to `DioxusDocument::graph` on every read) · seven_guis' library unit tests 3

@fact id=fct-2k7iskpv expect_rev=1
old: ; no `NodeId` crosses
new: ; a `graph` call's answer also carries the screen's UI graph out — ids, owner ids, component names, registered state names, handler event names, the declared writes, the closed flag words `undeclared` · `unconfirmed` and booleans, never a value or typed text; no `NodeId` crosses
@pattern id=tp4-01
fixed: 2,929 B
@pattern id=tp4-02
fixed: 47,089 bytes
@pattern id=tp4-03
fixed: 51 files
@pattern id=tp4-04
fixed: 51 `stand_*.rs` files
@pattern id=tp4-05 ignore_case=true
fixed: twelve verbs
@pattern id=tp4-06 ignore_case=true
regex: eight instance(-level)? (verbs|ones)
@pattern id=tp4-07
fixed: `screenshot` and `lint` run no step
@pattern id=tp4-08
fixed: `cli_commands` 15
