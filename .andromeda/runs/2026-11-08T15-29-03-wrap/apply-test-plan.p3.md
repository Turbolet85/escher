@apply_row n=1 ref_file=fanout-test-plan--cascade-g4.md ref_n=7 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g4.md ref_n=8 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g5.md ref_n=1 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g5.md ref_n=2 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g5.md ref_n=3 v=applied edit=E5
@fact id=fct-mrjxzany expect_rev=2
old: `tests/blitz-tests/tests/stand_*.rs` — 52 files,
new: `tests/blitz-tests/tests/stand_*.rs` — 53 files,
old: `stand_act_popup` and `stand_graph`
new: `stand_act_popup`, `stand_graph` and `stand_actions`

@fact id=fct-kjlei6et expect_rev=1
old: (`target/ci-logs/test.log`, equal to CI#41333172440's linux job)
new: (`target/ci-logs/test.log`, equal to CI#41333172440's linux job); re-counted with `stand_actions`: 223 result lines, 0 failed (223 `test result: ok`, 0 `FAILED`), the passed and ignored totals not read — the moves from 219 not attributed line by line, `stand_actions` one new result line of 8 passed among them — timings not re-measured — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md (its `ci-leg.sh fast` run on the committed tree)

@fact id=fct-fx4qekph expect_rev=3
old: read in escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md)
new: read in escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/ci.md); with escher-driver's `actions.rs` and `check_target` and dioxus-native-dom's `actionable_nodes` and `Readings` in the tree, CI#42493052318's `Coverage report` check passes under the same floors and `coverage_floor.py`, holding escher-driver ≥ 97 % and dioxus-native-dom ≥ 83 %, the per-crate figures not read — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md

@fact id=fct-3ctmcebn expect_rev=2
old: run the 52 `stand_*.rs` files
new: run the 53 `stand_*.rs` files
old: (§9 Legs; with the seventeen and `stand_act_popup`
new: (§9 Legs; with `stand_actions`, entering through the existing `stand_*.rs` glob, CI#42493052318 green 17/17, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md; with the seventeen and `stand_act_popup`

@fact id=fct-zgelu5mp expect_rev=1
old: mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` 3
new: mcp_tests.rs's `the_listing_is_the_fourteen_verbs_of_the_table` 3, a reading taken under the test's earlier name and not re-read
@pattern id=tp3-01
fixed: stand_*.rs` — 52 files
@pattern id=tp3-02
fixed: run the 52 `stand_*.rs` files
@pattern id=tp3-03
fixed: the_listing_is_the_twelve_verbs_of_the_table
