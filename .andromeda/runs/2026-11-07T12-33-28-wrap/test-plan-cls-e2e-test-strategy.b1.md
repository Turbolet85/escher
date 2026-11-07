@answer n=1 entity=e2e-test-strategy#1.1 v=no
why: the CLI, MCP and seven_guis test files it names are outside the session's four reached files, and the change keeps the public initial_build and poll signatures the callers use.
@answer n=2 entity=e2e-test-strategy#1.2 v=no
why: the select-list CLI test lives in examples/seven_guis/tests/cli_commands.rs, which the session does not reach.
@answer n=3 entity=e2e-test-strategy#1.3 v=no
why: the MCP tests (mcp_server, mcp_log, stand_mcp_tools) are not in the session's reached files, and the harness callers keep their signatures.
@answer n=4 entity=e2e-test-strategy#2.1 v=yes mode=contradict
why: the buffered render replays mutations with the document unborrowed, so a MountedData call in a key-handler task no longer panics with RefCell already borrowed, which this row still states.
@answer n=5 entity=e2e-test-strategy#3.1 v=yes mode=contradict
why: the probe's two should_panic pins are renamed and inverted to mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental over spawned_focus_acts, so they no longer pin a panic, and the probe's line citations moved with the 574-613 insertion.
@answer n=6 entity=e2e-test-strategy#4.1 v=no
why: the select list is painted by paint_scene in blitz-paint, which the session does not reach, and the list's headless path is unchanged.
@answer n=7 entity=e2e-test-strategy#5.1 v=no
why: the session changes no example fixture, and the assertion search over the slice s02 fixtures still holds.
@answer n=8 entity=e2e-test-strategy#5.2 v=no
why: the absence search covers the apps slice s04, which the session does not reach, and the new unit test sits in dioxus-native-dom, not in apps.
