@answer n=1 entity=unit-test-strategy#1.1 v=yes mode=stale
why: cli.rs and dioxus_document.rs are reached whole, so the cited line ranges cli.rs 536-538 and dioxus_document.rs 919-921 can have moved
@answer n=2 entity=unit-test-strategy#2.1 v=no
why: blitz-paint is not reached by the session and layers.rs is unchanged
@answer n=3 entity=unit-test-strategy#3.1 v=no
why: blitz-shell is not reached by the session and its one test is unchanged
@answer n=4 entity=unit-test-strategy#4.1 v=no
why: blitz-test-harness is not reached by the session and its pinned claims hold
@answer n=5 entity=unit-test-strategy#4.2 v=yes mode=add-to
why: the escher-driver unit-test list gains the coverage line form and the coverage role reading (F8, json.rs and cli_tests.rs), and its seven_guis cli_commands count moves 17 to 18
@answer n=6 entity=unit-test-strategy#5.1 v=yes mode=stale
why: json.rs is reached whole and gains a test, so the cited json.rs lines 1175-1176 can have moved
@answer n=7 entity=unit-test-strategy#6.1 v=no
why: mcp_tests.rs and mcp.rs are not reached and the fourteen verbs of the table are unchanged, the coverage role being command-line only
@answer n=8 entity=unit-test-strategy#7.1 v=no
why: schema.rs and schema_tests.rs are not reached and the help of type is unchanged
@answer n=9 entity=unit-test-strategy#8.1 v=yes mode=stale
why: the escher-driver figure of 102 moves to 109 with the command-line line table, and cli_tests.rs grows by one test
@answer n=10 entity=unit-test-strategy#9.1 v=no
why: schema_tests.rs and schema.rs are not reached and the two nodes fields are unchanged
@answer n=11 entity=unit-test-strategy#10.1 v=no
why: the CI workflows, ci-leg.sh and the CI Python tests are unchanged and the stand-repeat named targets stay seventeen
@answer n=12 entity=unit-test-strategy#11.1 v=no
why: the blitz-tests manifest is unchanged and its one test table still holds
@answer n=13 entity=unit-test-strategy#12.1 v=no
why: the cold-agent pipe, its fake binary and the MCP surface are unchanged by the session
@answer n=14 entity=unit-test-strategy#13.1 v=yes mode=stale
why: docs capabilities.md is reached whole, and the code spans and counts this test reads come from that file
@answer n=15 entity=unit-test-strategy#14.1 v=no
why: blitz-dom is not reached by the session
@answer n=16 entity=unit-test-strategy#15.1 v=no
why: blitz-dom is not reached by the session
@answer n=17 entity=unit-test-strategy#16.1 v=no
why: blitz-dom is not reached by the session
@answer n=18 entity=unit-test-strategy#17.1 v=no
why: blitz-dom is not reached by the session
@answer n=19 entity=unit-test-strategy#18.1 v=no
why: blitz-dom is not reached by the session
@answer n=20 entity=unit-test-strategy#19.1 v=no
why: the blitz-dom document module is not reached, the session's document changes being in dioxus-native-dom
@answer n=21 entity=unit-test-strategy#20.1 v=no
why: blitz-net is not reached by the session
@answer n=22 entity=unit-test-strategy#21.1 v=yes mode=add-to
why: the reached-handler record joins the graph readers' unit-test description per task touch 10, graph.rs itself being unchanged
@answer n=23 entity=unit-test-strategy#22.1 v=no
why: signal_registry.rs is not reached and its dispatch record is unchanged
@answer n=24 entity=unit-test-strategy#23.1 v=yes mode=add-to
why: F7, bridge_tests' the_reached_record_holds_on_a_page_with_no_registered_state, joins this bridge dispatch record, bridge_tests moving 8 to 9
@answer n=25 entity=unit-test-strategy#24.1 v=no
why: snapshot.rs is not reached and clickable reads the element's current listeners, which the reached record leaves alone
