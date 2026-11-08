@answer n=1 entity=e2e-test-strategy#1.1 v=yes mode=stale
why: the nine instance-level verbs become ten and cli_commands (16) becomes 17 with the new actions tests, so both counts moved
@answer n=2 entity=e2e-test-strategy#1.2 v=yes mode=stale
why: cli_commands.rs is reached whole by the new actions tests, so the cited lines 421-466 may have moved
@answer n=3 entity=e2e-test-strategy#1.3 v=yes mode=stale
why: mcp_server's tools/list now offers fourteen verbs, not thirteen, and its count moves 7 to 8 with the actions tool call (F11)
@answer n=4 entity=e2e-test-strategy#2.1 v=no
why: the buffered render lives in dioxus_document.rs, which the change does not reach, and the session's dioxus-native-dom additions are actionable_nodes and the actionable rule only
@answer n=5 entity=e2e-test-strategy#3.1 v=no
why: dioxus-core's task scheduling is a dependency the session did not change, and no reached file holds it
@answer n=6 entity=e2e-test-strategy#4.1 v=no
why: the two pins and their helper live in dioxus_runtime_probe.rs, which the change does not reach, and the buffered render they pin is untouched
@answer n=7 entity=e2e-test-strategy#5.1 v=no
why: the select list paints through paint_scene, which the session does not change, and its new options reading does not touch the paint path
@answer n=8 entity=e2e-test-strategy#6.1 v=yes mode=stale
why: the example test files the change reaches whole (cli_commands, mcp_server, host_log, mcp_log, common/mod.rs) gain assertions, so this search over the 21 files reads differently
@answer n=9 entity=e2e-test-strategy#6.2 v=yes mode=add-to
why: cli_commands and host_log, named here as UI-driving, gain hosted actions runs, so the list stands and must carry F10 (cli_commands' actions answer as a process, status 1 on an argument)
