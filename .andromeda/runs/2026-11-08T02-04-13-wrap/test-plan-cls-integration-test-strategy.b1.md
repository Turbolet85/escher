@answer n=1 entity=integration-test-strategy#1.1 v=no
why: pseudo_element_flow_classification.rs is outside the change's reach and the pipeline tests stay byte-unchanged
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script's dom.rs and selection.rs are outside the change's reach and unchanged
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: the radio change test in blitz-vibey-script's dom.rs is outside the change's reach
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: url_search_params.rs in blitz-vibey-script is outside the change's reach
@answer n=5 entity=integration-test-strategy#5.1 v=yes mode=add-to
why: the session-over-held-instance boundary gains the read-only graph verb, which F1's six stand_graph tests cover in both layout modes
@answer n=6 entity=integration-test-strategy#5.2 v=yes mode=stale
why: cli_commands.rs, cited here at lines 135-217, is reached by the new graph case, so that citation can move
@answer n=7 entity=integration-test-strategy#5.3 v=yes mode=stale
why: host_log and mcp_log each gain one graph call, moving their seven and ten call counts and their command-span line counts
@answer n=8 entity=integration-test-strategy#6.1 v=yes mode=stale
why: host_log.rs and mcp_log.rs, cited here at lines 283-394 and 70-115, are reached by the new graph calls, so those citations can move
@answer n=9 entity=integration-test-strategy#7.1 v=yes mode=stale
why: cli_commands.rs, cited here at lines 420-465, is reached by the new graph case, so that citation can move
@answer n=10 entity=integration-test-strategy#8.1 v=no
why: stand_act_select.rs is outside the reached set and its multiple-select claims are unaffected by the shared-module additions
@answer n=11 entity=integration-test-strategy#9.1 v=no
why: stand_act_popup.rs is outside the reached set and its popup claims are unaffected by the change
@answer n=12 entity=integration-test-strategy#10.1 v=no
why: select_popup.rs is outside the reached set and the harness input claims stand unchanged
@answer n=13 entity=integration-test-strategy#11.1 v=no
why: select_popup_painted.rs is outside the reached set and its frame claims stand unchanged
@answer n=14 entity=integration-test-strategy#12.1 v=no
why: render_blocking_stylesheet.rs is outside the reached set and its stylesheet claims are unaffected
@answer n=15 entity=integration-test-strategy#13.1 v=no
why: mutation_writer.rs and dioxus_runtime_probe.rs are unreached and unchanged, and the bridge dispatch is inert without a registry, so the listener claim holds
@answer n=16 entity=integration-test-strategy#14.1 v=yes mode=stale
why: the seven_guis task signals now register through use_graph_signal, moving the reading that the stand keeps all its state in hook-local use_signal
@answer n=17 entity=integration-test-strategy#15.1 v=yes mode=stale
why: the new graph tests in seven_guis' cli_commands.rs and mcp_server.rs are integration tests outside src, so this absence reading's since-list moves
