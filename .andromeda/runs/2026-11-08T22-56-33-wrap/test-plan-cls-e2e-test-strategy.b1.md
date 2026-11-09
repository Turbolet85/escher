@answer n=1 entity=e2e-test-strategy#1.1 v=yes mode=add-to
why: F5 adds the coverage role's command-line shape beside this command-line group, and cli_commands' linux count moves 17 to 18 with its tenth usage row
@answer n=2 entity=e2e-test-strategy#1.2 v=yes mode=stale
why: cli_commands.rs is a reached file and its cited lines 425-470 can move with the added coverage test and usage row
@answer n=3 entity=e2e-test-strategy#2.1 v=yes mode=stale
why: cli_commands.rs is a reached file and its cited lines 499-520 can move with the added coverage test and usage row
@answer n=4 entity=e2e-test-strategy#3.1 v=no
why: mcp_server.rs, mcp_log.rs and stand_mcp_tools.rs are not reached and the MCP surface and its verbs are unchanged
@answer n=5 entity=e2e-test-strategy#4.1 v=no
why: the MCP actions tool call and its mcp_server and stand_mcp_tools tests are not reached and the verb surface is unchanged
@answer n=6 entity=e2e-test-strategy#5.1 v=yes mode=stale
why: dioxus_document.rs is reached and this entity cites its lines 318 and 319 for the buffered render, which the reached change can move
@answer n=7 entity=e2e-test-strategy#6.1 v=no
why: dioxus-core 0.7.10 scheduling and the probe evidence sit outside every reached file and no dependency moved
@answer n=8 entity=e2e-test-strategy#7.1 v=no
why: the two pins and their spawned_focus_acts helper live in dioxus_runtime_probe.rs, which the session does not reach
@answer n=9 entity=e2e-test-strategy#8.1 v=no
why: the select list's paint_scene path and its measured headless reading are in no reached file
@answer n=10 entity=e2e-test-strategy#9.1 v=yes mode=stale
why: session_host.rs is a reached file in the example sources this absence search covered, so its zero-match reading can have moved
@answer n=11 entity=e2e-test-strategy#9.2 v=yes mode=stale
why: the new reached stand_handler_coverage.rs is a #[test] file that can join the searched 86 files, moving that count and its no-document reading
