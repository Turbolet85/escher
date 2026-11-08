@apply_row n=1 ref_file=fanout-test-plan--cascade-g1.md ref_n=11 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g1.md ref_n=12 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g3.md ref_n=16 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=1
@apply_row n=6 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=1
@apply_row n=7 ref_file=fanout-test-plan--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=12
@apply_row n=8 ref_file=fanout-test-plan--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=12
@apply_row n=9 ref_file=fanout-test-plan--cascade-g1.md ref_n=6 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=12
@apply_row n=10 ref_file=fanout-test-plan--cascade-g1.md ref_n=7 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=12
@apply_row n=11 ref_file=fanout-test-plan--cascade-g1.md ref_n=8 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=12
@apply_row n=12 ref_file=fanout-test-plan--cascade-g1.md ref_n=9 v=applied edit=E5
@apply_row n=13 ref_file=fanout-test-plan--cascade-g1.md ref_n=10 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=9
@apply_row n=14 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E6
@apply_row n=15 ref_file=fanout-test-plan--cascade-g2.md ref_n=2 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=1
@apply_row n=16 ref_file=fanout-test-plan--cascade-g2.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=1
@apply_row n=17 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E7
@apply_row n=18 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E8
@fact id=fct-jydhmppz expect_rev=3
old: the actionable-key check, the bridge
new: the actionable-key check and the actionable nodes reader over their one rule, the bridge

@fact id=fct-nks3g3e7 expect_rev=4
old: 101 rows, the `mcp` role's three and `graph`'s among them
new: 102 rows, the `mcp` role's three, `graph`'s and `actions`'s among them
old: 100 unit tests in sixteen files, 64 inline and 36
new: 102 unit tests in seventeen files, 66 inline and 36
old: packages/escher-driver/src/lint.rs:556-634); host.rs
new: packages/escher-driver/src/lint.rs:556-634) and actions.rs 2 (every call named for a target verb validates; an id past its bound reads malformed — packages/escher-driver/src/actions.rs:138-158); host.rs
old: nine instance-level (`screenshot` the seventh
new: ten instance-level (`screenshot` the seventh
old: and `graph` the ninth, after `lint`)
new: `graph` the ninth, after `lint`, and `actions` the tenth, after `graph`)
old: `help` at the tool level, thirteen;
new: `help` at the tool level, fourteen;
old: the listing is the thirteen verbs of the table, `graph` after `lint`,
new: the listing is the fourteen verbs of the table, `graph` after `lint` and `actions` after `graph`,
old: its count row reads `[13, 14, 12, 3, 8, 4, 15]`
new: its count row reads `[14, 14, 12, 3, 8, 4, 15]`
old: over a held instance by `stand_lint`, not by a unit test
new: over a held instance by `stand_lint`, and `Session::actions` and the shared per-verb target check `check_target` by `stand_actions` (8 tests over the six served screens, both layout modes), not by a unit test

@fact id=fct-gq2zcc44 expect_rev=2
old: changes nothing (`stand_graph`)
new: changes nothing (`stand_graph`); the `actions` verb lists each served screen's actionable nodes with the table's verbs, every listed cause what the call returns, in both layout modes, and changes nothing (`stand_actions`)

@fact id=fct-k5y3m6cu expect_rev=2
old: (examples/seven_guis/tests/common/mod.rs:425-452); read through `mod common;` by all eight)
new: (examples/seven_guis/tests/common/mod.rs:425-452), and `actions_needles()`, the CRUD screen's listed author-key ids holding a `-` as needles, called by the same two, each making one `actions` call and reading 0 id, name and actions needles at `trace` under both builds (examples/seven_guis/tests/common/mod.rs:454-476); read through `mod common;` by all eight)
old: 16 tests, unix only: every driver command
new: 17 tests, unix only: every driver command
old: (examples/seven_guis/tests/cli_commands.rs:468-493);
new: (examples/seven_guis/tests/cli_commands.rs:468-493); an `actions` answering one line of the actionable nodes, an argument to it refused with status 1 (examples/seven_guis/tests/cli_commands.rs:495-516);
old: 7 tests: the binary's
new: 8 tests: the binary's
old: `tools/list` offering the verbs in order, thirteen, `graph` after `lint`,
new: `tools/list` offering the verbs in order, fourteen, `graph` after `lint` and `actions` after `graph`,
old: (examples/seven_guis/tests/mcp_server.rs:341-362)
new: (examples/seven_guis/tests/mcp_server.rs:341-362) and an `actions` tool call with its line of the actionable nodes, an argument to it answered `isError` (examples/seven_guis/tests/mcp_server.rs:363-388)

@fact id=fct-i5a6vv3t expect_rev=3
old: 100 unit tests in eleven files (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
new: 104 unit tests in eleven files (as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md)
old: four on the actionable-key check `DioxusDocument::unkeyed_actionable`
new: eight in actionable.rs, four on the actionable-key check `DioxusDocument::unkeyed_actionable`
old: the report naming structure never content — compiled only under
new: the report naming structure never content — and four on the one actionable rule read by it and by the actionable nodes reader `DioxusDocument::actionable_nodes`: a listener removed on a re-render read by neither reader, a select's options and group listed with the select as owner, a positional id reading unkeyed and an author key keyed, an inert or hidden element not listed (packages/dioxus-native-dom/src/actionable.rs:317-433) — compiled only under

@fact id=fct-z2ynupl3 expect_rev=4
old: since 2026-10-10-mcp-surface `stand_mcp_tools` 7
new: since 2026-10-10-mcp-surface `stand_mcp_tools` 8
old: (tests/blitz-tests/tests/stand_mcp_tools.rs:772-867);
new: (tests/blitz-tests/tests/stand_mcp_tools.rs:772-867); an `actions` tool call answering the reader's nodes and parenting one `command` span recording `verb` alone, in both layout modes (`an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone`), the `graph` and `actions` tests running one shared helper, `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone` (tests/blitz-tests/tests/stand_mcp_tools.rs:774-792);
old: 36 calls per mode
new: 38 calls per mode (`run.calls == 38`, tests/blitz-tests/tests/stand_act_spans.rs:767)
old: its `command` line recording `verb` alone (tests/blitz-tests/tests/stand_act_spans.rs:232-249)
new: its `command` line recording `verb` alone, and the tenth, `actions`, one call per mode, its `command` line recording `verb` alone (tests/blitz-tests/tests/stand_act_spans.rs:232-249)
old: tests/blitz-tests/tests/stand_mcp_tools.rs:923-946);
new: tests/blitz-tests/tests/stand_mcp_tools.rs:923-946); `stand_actions` 8, declaring `mod common;`, `mod fixtures;` and `mod session_common;`, through `Session::actions` and `Session::run` on the six served screens in both layout modes — each screen's listed actionable nodes with the table's verbs and counts (`common::ACTIONABLE_AT_BOOT`), every listed cause on the four lean tasks equal to the call's, 3 `disabled` controls among them, the stated Cells and load-page causes, the flight booker's options read `covered` until the list opens, `crud-person-14` read `off-screen` until a `scroll`, the fixtures read covered, unkeyed and fonts-unavailable, the read changing nothing and holding no typed text or mask, and the whole Cells answer within the answer bound — picked up by `run stand` and the stand-repeat leg by its prefix (tests/blitz-tests/tests/stand_actions.rs:164-560);

@fact id=fct-2vwamopj expect_rev=1
old: and by blitz-dom's `font_reading_tests` (tests/blitz-tests/tests/stand_act_screenshot.rs:199-222;
new: and by blitz-dom's `font_reading_tests`, and the `actions` read names the same `fonts-unavailable` for `screenshot` with nothing painted — the shared target check `check_target` reading `can_paint_text` after the crop and before painting — read by `stand_actions`' `the_fixtures_read_covered_unkeyed_and_fonts_unavailable` in both layout modes (tests/blitz-tests/tests/stand_actions.rs:420-484; tests/blitz-tests/tests/stand_act_screenshot.rs:199-222;

@fact id=fct-7m7lxsmy expect_rev=3
old: read through `mod common;` by 22 `stand_*.rs` checks
new: read through `mod common;` by 23 `stand_*.rs` checks
old: read through `mod session_common;` by 33 checks, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md (`stand_graph.rs` the latest, then
new: read through `mod session_common;` by 34 checks, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md (`stand_actions.rs` the latest, then `stand_graph.rs`, then
old: read through `mod fixtures;` by 31 checks, counted anchored at line start, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md — 23 `stand_*.rs` (`stand_act_popup`,
new: read through `mod fixtures;` by 32 checks, counted anchored at line start, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md — 24 `stand_*.rs` (`stand_actions`, `stand_act_popup`,
old: 159 `.rs` files, 158 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the latest `stand_graph.rs`, then
new: 160 `.rs` files, 159 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md, the latest `stand_actions.rs`, then `stand_graph.rs`, then

@pattern id=tp1-01
fixed: 101 rows, the `mcp` role's three
@pattern id=tp1-02
fixed: 16 tests, unix only
@pattern id=tp1-03
fixed: offering the verbs in order, thirteen
@pattern id=tp1-04
fixed: 100 unit tests in sixteen files
@pattern id=tp1-05
fixed: nine instance-level
@pattern id=tp1-06
fixed: the listing is the thirteen verbs
@pattern id=tp1-07
fixed: [13, 14, 12, 3, 8, 4, 15]
@pattern id=tp1-08
fixed: 100 unit tests in eleven files
@pattern id=tp1-09
fixed: `stand_mcp_tools` 7
@pattern id=tp1-10
fixed: 36 calls per mode
@pattern id=tp1-11
fixed: by 33 checks
@pattern id=tp1-12
fixed: by 31 checks
@pattern id=tp1-13
fixed: 159 `.rs` files, 158 test targets
