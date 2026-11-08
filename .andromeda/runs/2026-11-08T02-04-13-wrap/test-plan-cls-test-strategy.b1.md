@answer n=1 entity=test-strategy#1.1 v=no
why: the harness surface is unchanged and still wraps DioxusDocument, which gained only methods, so the wrapping claim holds on the changed file
@answer n=2 entity=test-strategy#2.1 v=yes mode=stale
why: the graph calls and helpers in the reached shared modules and mcp_server.rs can move the cited lines of common/mod.rs, session_common/mod.rs and mcp_server.rs
@answer n=3 entity=test-strategy#2.2 v=yes mode=stale
why: cli_commands gains a graph command-line case and common/mod.rs gains graph_needles, so the cited runner lines 57-128 of seven_guis' common/mod.rs may move
@answer n=4 entity=test-strategy#3.1 v=no
why: the re-recorded counter-cli.json and counter-mcp.json stay committed data that stand_replay replays green, so the entity's claim still holds on the changed files
@answer n=5 entity=test-strategy#4.1 v=yes mode=stale
why: docs/capabilities.md changes its counts to thirteen verbs and fourteen node keys and retires the not-built click line, so its cited line ranges and limit list move
@answer n=6 entity=test-strategy#5.1 v=no
why: its cited paint, screenshot and layer files are unreached and the session changes no pixel path, since the bridge dispatch is inert without a registry
@answer n=7 entity=test-strategy#6.1 v=no
why: the painted select list and select_popup_painted are unreached and the session changes no select paint
@answer n=8 entity=test-strategy#7.1 v=yes mode=stale
why: stand_mcp_tools gained a graph tool-call test, so its cited fonts-unavailable reading at lines 582-595 may move
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT conformance readings and runner are unreached by the session
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: the new stand_graph.rs adds one .rs file and one auto-discovered test target to tests/blitz-tests/tests, moving the file and target counts
@answer n=11 entity=test-strategy#10.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs gained the Drive enum and GRAPH_DRIVES, so the cited LOAD_TAB_ORDER and CELLS_TAB_ORDER lines 93-98 may move
