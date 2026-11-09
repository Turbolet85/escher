@account n=1 entity=test-scope-summary#1.1 v=not_touched
why: docs/capabilities.md gained only its three limit bullets while test_capability_statement.py stays byte-identical and CI is green, so the pin and its 110 tests hold as stated
@account n=2 entity=test-scope-summary#2.1 v=proposed
why: cli_commands goes 17 to 18 tests with the coverage role run, host_log 5 to 6 with the coverage run, and common gains coverage_needles() called by host_log alone
@account n=3 entity=test-scope-summary#11.1 v=not_touched
why: the report reads dioxus_runtime_probe green with poll and the buffered render untouched, so its 10 tests, prefix and selection hold
@account n=4 entity=test-scope-summary#12.1 v=not_touched
why: the reached record writes in handle_event and takes no borrow across render_immediate, poll untouched, so the buffered render's limit holds
@account n=5 entity=test-scope-summary#13.1 v=not_touched
why: graph.rs and GRAPH_DRIVES are unchanged and the new common tables sit after them, so the unconfirmed limit holds; no task fact is of this category
@account n=6 entity=test-scope-summary#24.2 v=proposed
why: escher-driver's json.rs goes 7 to 8 tests, cli_tests.rs 5 to 6 and the line table 102 to 109 rows, so the crate's unit count and those files' descriptions moved
@account n=7 entity=test-scope-summary#26.1 v=proposed
why: dioxus-native-dom gains reached::tests 4 in the new reached.rs and bridge_tests 8 to 9, so its 104 in eleven files and its bridge count moved
@account n=8 entity=test-scope-summary#26.2 v=not_touched
why: dioxus_document.rs changed only at its import and handle_event's record lines, so the upstream IME test it names stands as stated
@account n=9 entity=test-scope-summary#26.3 v=not_touched
why: poll, initial_build and the buffered render are untouched and dioxus_runtime_probe is green, so the ported test's reading holds
@account n=10 entity=test-scope-summary#26.4 v=not_touched
why: flush_queued_mounted_events is untouched per the report, so the left-out upstream test and its owner hold
@account n=11 entity=test-scope-summary#27.1 v=proposed
why: the search list of dioxus-native-dom files holding tests gains reached.rs, whose reached::tests module is new
