@answer n=1 entity=e2e-test-strategy#1.1 v=yes mode=stale
why: the instance verbs move from eight to nine with graph, and cli_commands gains a graph test moving its count of 15
@answer n=2 entity=e2e-test-strategy#1.2 v=yes mode=stale
why: cli_commands.rs is reached whole by the graph test additions, so its cited lines 420-465 can move
@answer n=3 entity=e2e-test-strategy#1.3 v=yes mode=stale
why: the MCP listing moves from twelve to thirteen tools with graph, and mcp_server and stand_mcp_tools gain graph tests moving their counts of 6
@answer n=4 entity=e2e-test-strategy#2.1 v=yes mode=stale
why: dioxus_document.rs is reached whole by the dispatch_to_vdom shim and the event-path registry mark, so its cited lines 317 and 318 can move
@answer n=5 entity=e2e-test-strategy#3.1 v=no
why: the registry dispatch mark sits on the event path and leaves dioxus-core's task scheduling and the probe's pins as they stood, the buffered render being untouched
@answer n=6 entity=e2e-test-strategy#4.1 v=no
why: the probe file it cites is not among the session's reached files and its focus reads are unchanged, the probe's 10 tests staying green
@answer n=7 entity=e2e-test-strategy#5.1 v=no
why: the select list's headless paint path through paint_scene is not reached by the graph, registry or snapshot changes
@answer n=8 entity=e2e-test-strategy#6.1 v=yes mode=stale
why: examples/seven_guis/src/tasks/mod.rs and the seven_guis test common module are reached whole, so the assertion search reading over the 21 files cannot be shown to hold
@answer n=9 entity=e2e-test-strategy#6.2 v=yes mode=stale
why: the seven_guis test files gain graph tests and a graph_needles helper, so the search reading over the 86 files of slice s04 cannot be shown to hold
