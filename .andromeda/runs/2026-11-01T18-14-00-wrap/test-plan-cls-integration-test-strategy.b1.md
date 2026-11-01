# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · its test file pseudo_element_flow_classification.rs is unreached, and CI at 45c333f16 is green (run all 733 · 0 · 12) with incremental_oracle unmoved at 7, so the layout-identity claim holds on the changed construct.rs
integration-test-strategy#2.1 · yes · add-to · F8: the new `url_search_params` target, 4 tests of the global URLSearchParams and URL.prototype.searchParams, joins the blitz-vibey-script test targets listed beside dom.rs and selection.rs
integration-test-strategy#3.1 · no · the radio change claim lives in unreached dom.rs and runtime/events.rs, the merge-surface bootstrap files carry URLSearchParams and not the event path, and CI at 45c333f16 is green
integration-test-strategy#4.1 · no · its load claims cover answered and never-answered loads, which the session leaves; the failed-stylesheet change ends only error-answered loads, stand_load_wait and settle_waiting are unedited, and CI at 45c333f16 is green
integration-test-strategy#4.2 · no · the session host and socket paths are unreached and no session-lifecycle file has a diff
integration-test-strategy#4.3 · no · the log sites and escher-telemetry have no diff, stand_act_spans still reads eight fields, and the recorded-session counts are unmoved
integration-test-strategy#5.1 · no · host_log.rs and mcp_log.rs are unreached examples/seven_guis files, and the select_popup change sits in tests/blitz-tests, off the host's log path
integration-test-strategy#6.1 · no · cli_commands.rs is an unreached examples/seven_guis file and its select-list claim holds, with CI at 45c333f16 green
integration-test-strategy#7.1 · no · stand_act_select.rs is unreached and its claims name no bounds number, so the sub-pixel size change keeps each row with its own bounds, and CI at 45c333f16 is green
integration-test-strategy#8.1 · no · stand_act_popup.rs is unreached and its claims (12 tests, lint reading no overlap on a row) still hold, with CI at 45c333f16 green
integration-test-strategy#9.1 · yes · contradict · the claim that one row skips its key press no longer holds: the key-press panic is retired and select_popup.rs:513 is one harness.press for every row
integration-test-strategy#10.1 · yes · contradict · the font-embolden gap this line carries is retired; the header-weight witness is now the `embolden` step of the local `fast` gate
integration-test-strategy#11.1 · no · the absence search covers apps/ only, no file under apps/ was reached, and the new blitz-net tests sit under packages/
