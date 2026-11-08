@answer n=1 entity=test-strategy#1.1 v=no
why: the harness surface is untouched and the blitz-test-harness files it cites are not in the session's change set
@answer n=2 entity=test-strategy#2.1 v=yes mode=stale
why: the host and mcp-role log checks gained a graph call and seven_guis' shared module gained graph_needles, both reached files whose cited line ranges can have moved
@answer n=3 entity=test-strategy#2.2 v=yes mode=stale
why: the command-line and MCP child checks gained a graph case and the reached seven_guis shared runner, cited at common/mod.rs:57-128, can have moved
@answer n=4 entity=test-strategy#3.1 v=yes mode=stale
why: the two committed Counter checks counter-cli.json and counter-mcp.json were re-recorded, moving their bytes, and both are reached files
@answer n=5 entity=test-strategy#4.1 v=yes mode=stale
why: docs/capabilities.md is reached, its not-built reacts-to-a-click line was retired, and its cited line ranges can have moved
@answer n=6 entity=test-strategy#5.1 v=no
why: the session changes no screenshot, paint or pixel test and adds no dev-dependency, and none of the cited files is reached
@answer n=7 entity=test-strategy#6.1 v=no
why: the painted select-list check and ci-leg.sh are untouched and outside the session's change set
@answer n=8 entity=test-strategy#7.1 v=yes mode=stale
why: stand_mcp_tools.rs is reached and gained a graph tool-call test, so its cited lines 582-595 can have moved
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT runner and its fixtures are untouched and none is in the session's change set
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: the new stand_graph.rs adds one auto-discovered test target, moving the 158 .rs files and 157 targets count to 159 and 158, and reached cfg(test) module files cited by line can have moved
@answer n=11 entity=test-strategy#10.1 v=yes mode=stale
why: tests/common/mod.rs is reached and gained the Drive enum and GRAPH_DRIVES, so its cited lines 93-98 can have moved
