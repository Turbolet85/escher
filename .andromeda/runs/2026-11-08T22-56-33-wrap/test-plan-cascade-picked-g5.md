@account n=1 entity=integration-test-strategy#7.1 v=proposed
why: the UI graph's integration entity is where stand_handler_coverage, its screens decision and its stated limits (F1, F2, F3) sit beside
@account n=2 entity=integration-test-strategy#10.1 v=proposed
why: the owed actions-step replay test now exists and passes (stand_replay, matrix#v030-06 MET), so «owed» is contradicted and F4 sits beside it
@account n=3 entity=integration-test-strategy#12.1 v=not_touched
why: the report adds a coverage test to cli_commands and leaves the graph answer and its refusal unchanged, so the claim still holds
@account n=4 entity=integration-test-strategy#13.1 v=not_touched
why: the actions answer and its refusal are unchanged by the report's Changes, cli_commands gaining only the coverage test and a usage row
@account n=5 entity=integration-test-strategy#14.1 v=not_touched
why: start, status and stop on the six served tasks still hold on the changed cli_commands, which only gained the coverage test and usage fragments
@account n=6 entity=integration-test-strategy#14.2 v=proposed
why: host_log's stated total moves 5 to 6 with the coverage run, which is F6's new fact beside it
@account n=7 entity=integration-test-strategy#15.1 v=not_touched
why: host_log's select-list test is unchanged, the report adding only the coverage-run test to that file
@account n=8 entity=integration-test-strategy#16.1 v=not_touched
why: cli_commands' select-list test is unchanged, the report adding only the coverage test, usage row and fragments
@account n=9 entity=integration-test-strategy#24.1 v=not_touched
why: no new integration test file outside src in the apps, the seven_guis tests already named
@account n=10 entity=e2e-test-strategy#1.1 v=proposed
why: cli_commands moves 17 to 18 on linux, its usage line gains the coverage role, and F5 sits beside it
@account n=11 entity=e2e-test-strategy#1.2 v=not_touched
why: the hosted select-list shot test is unchanged by the report's Changes
@account n=12 entity=e2e-test-strategy#2.1 v=not_touched
why: the actions command test is unchanged, the coverage test added after it
@account n=13 entity=e2e-test-strategy#5.1 v=not_touched
why: the report states poll, initial_build and flush_queued_mounted_events untouched in dioxus_document.rs, the record sitting in handle_event alone
@account n=14 entity=e2e-test-strategy#6.1 v=not_touched
why: dioxus-core's task scheduling reading is not moved by the record, which takes no borrow across render_immediate
@account n=15 entity=e2e-test-strategy#7.1 v=not_touched
why: dioxus_runtime_probe's pins are unchanged and green (entry 6), the record's guard dropped before runtime.handle_event
@account n=16 entity=e2e-test-strategy#9.1 v=not_touched
why: the example fixtures gained no assertion
@account n=17 entity=e2e-test-strategy#9.2 v=not_touched
why: no test launches a window, and the coverage run still drives the UI through the escher-session binary
@account n=18 entity=test-data-fixtures#2.1 v=not_touched
why: the committed checks are byte-identical (entry 8) and still embedded by stand_replay and recording
@account n=19 entity=test-data-fixtures#3.7 v=not_touched
why: the Cells screen's size and its part readers are unchanged, Cells' handler tables sitting beside the actionable-count row instead
@account n=20 entity=test-data-fixtures#3.9 v=not_touched
why: option_needles is unchanged in the changed common module, coverage_needles added beside it
@account n=21 entity=test-data-fixtures#3.17 v=not_touched
why: test_capability_statement.py is byte-identical (entry 8) and still reads docs/capabilities.md by path
@account n=22 entity=test-data-fixtures#3.30 v=proposed
why: GRAPH_DRIVES is now read by stand_handler_coverage too, whose drives check covers the 7 GRAPH_DRIVES tasks
@account n=23 entity=test-data-fixtures#3.31 v=proposed
why: F9's tables HANDLERS_AT_BOOT and UNEXERCISED_AFTER_DRIVES sit in the same shared module beside ACTIONABLE_AT_BOOT, cells reading 678
@account n=24 entity=test-data-fixtures#4.1 v=not_touched
why: slice s08's search for fixture files is not moved by the change
