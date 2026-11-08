@answer n=1 entity=test-scope-summary#1.1 v=yes mode=stale
why: the session reaches .github/scripts/test_cold_agent.py, whose 33 tests this entity counts inside its 110 total, and no report shows that count holds on the changed file
@answer n=2 entity=test-scope-summary#2.1 v=yes mode=stale
why: the session adds a graph test to cli_commands.rs and mcp_server.rs, a graph_needles helper to common/mod.rs, a graph call to host_log and mcp_log, and the MCP listing moves from twelve tools to thirteen, so its counts and line ranges are stale
@answer n=3 entity=test-scope-summary#2.2 v=no
why: apps/browser/src/tab.rs is outside everything the session changed
@answer n=4 entity=test-scope-summary#2.3 v=no
why: blitz-dom is byte-unchanged and its 83 #[test] functions are not the dioxus-native-dom count the session moved
@answer n=5 entity=test-scope-summary#2.4 v=no
why: the session changes no blitz-dom behaviour or blitz-tests check named in this entity
@answer n=6 entity=test-scope-summary#3.1 v=no
why: the inline-block-with-block-children paint skip in blitz-dom is untouched by the session
@answer n=7 entity=test-scope-summary#4.1 v=no
why: the four upstream blitz-dom and blitz-paint commits with no test of their own are outside the session's reach
@answer n=8 entity=test-scope-summary#5.1 v=no
why: blitz-dom hit testing and hit_non_invertible_transform.rs are not in the session's reach
@answer n=9 entity=test-scope-summary#6.1 v=no
why: the blitz-dom multiple-select rules and select_interaction.rs are not in the session's reach
@answer n=10 entity=test-scope-summary#7.1 v=no
why: the engine default chosen-row colour pair in default.css and its check are not in the session's reach
@answer n=11 entity=test-scope-summary#8.1 v=yes mode=stale
why: the session reaches packages/escher-driver/src/schema.rs, whose lines 273-275 this entity cites, and the added graph verb and clickable field can move those lines
@answer n=12 entity=test-scope-summary#9.1 v=no
why: select_popup.rs and blitz-dom's document.rs deallocate path are not in the session's reach
@answer n=13 entity=test-scope-summary#10.1 v=no
why: key_press_after_container_lost_its_box.rs and blitz-dom's anonymous-block handling are not in the session's reach
@answer n=14 entity=test-scope-summary#11.1 v=no
why: dioxus_runtime_probe.rs is not reached, its 10 tests stay unchanged, and the buffered render its two pins read is untouched
@answer n=15 entity=test-scope-summary#12.1 v=no
why: the buffered render's limit is unchanged, since the session leaves the buffered render, initial_build, poll and flush_queued_mounted_events untouched
@answer n=16 entity=test-scope-summary#13.1 v=no
why: the windowed select list's stated limits belong to a select popup the session does not change
@answer n=17 entity=test-scope-summary#14.1 v=no
why: the blitz-dom layout bounds reader and scrolled_box_client_rect.rs are not in the session's reach
@answer n=18 entity=test-scope-summary#14.2 v=no
why: pseudo_element_flow_classification.rs and blitz-dom's flow classification are not in the session's reach
@answer n=19 entity=test-scope-summary#14.3 v=no
why: the blitz-dom node files and their in-file tests are not in the session's reach
@answer n=20 entity=test-scope-summary#14.4 v=no
why: blitz-dom's :open state flush and open_state.rs are not in the session's reach
@answer n=21 entity=test-scope-summary#15.1 v=no
why: blitz-dom's node/select_list.rs and the checks reading it are not in the session's reach
@answer n=22 entity=test-scope-summary#16.1 v=no
why: open_state.rs is not in the session's reach
@answer n=23 entity=test-scope-summary#17.1 v=no
why: blitz-dom's clock.rs, its iframe test and virtual_clock_engine are not in the session's reach
@answer n=24 entity=test-scope-summary#18.1 v=no
why: blitz-net's inline tests and their listeners are not in the session's reach
@answer n=25 entity=test-scope-summary#19.1 v=no
why: blitz-paint's password mask and password_caret_click.rs are not in the session's reach
