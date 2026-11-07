@answer n=1 entity=e2e-test-strategy#1.1 v=no
why: its CLI verb, status and snapshot-size claims rest on examples/seven_guis tests and reports the session does not reach, and the seven_guis stand is untouched.
@answer n=2 entity=e2e-test-strategy#1.2 v=no
why: the select-popup CLI test at cli_commands.rs 419-464 is a seven_guis test the session does not reach.
@answer n=3 entity=e2e-test-strategy#1.3 v=no
why: its MCP claims rest on seven_guis and stand_mcp_tools tests the session does not reach, so the claim still holds on the changed dioxus-native-dom files.
@answer n=4 entity=e2e-test-strategy#2.1 v=yes mode=contradict
why: the buffered render replaces the borrow held across render_immediate that made a key-handler MountedData call panic, so the hazard is closed and the cited poll lines and panic pin no longer hold.
@answer n=5 entity=e2e-test-strategy#3.1 v=yes mode=contradict
why: the two should_panic pins are renamed and inverted to mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental over spawned_focus_acts, so the panic pins no longer exist.
@answer n=6 entity=e2e-test-strategy#4.1 v=no
why: the select list's paint_scene path and its headless reading live in files the session did not change, and the claim does not reach the dioxus-native-dom bridge.
@answer n=7 entity=e2e-test-strategy#5.1 v=no
why: its assertion search covers the example fixtures of slice s02, which the session did not touch.
@answer n=8 entity=e2e-test-strategy#5.2 v=no
why: its search covers the apps' test files, and the session's ported test is a dioxus-native-dom unit test outside the apps, so no app test launches a window or document.
