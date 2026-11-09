@account n=1 entity=test-strategy#2.1 v=not_touched
why: seven_guis tests/common/mod.rs gains only coverage_needles at 454-480 and host_log's coverage run holds its host behind a guard, so the lifecycle rule still holds
@account n=2 entity=test-strategy#2.2 v=not_touched
why: cli_commands still runs the binary per command and reads the same four exit statuses; its new coverage test is F5, an e2e-test-strategy fact
@account n=3 entity=test-strategy#3.1 v=proposed
why: Check::replay now runs through replay_each and handler_coverage is a second caller, replaying kept checks to list the handlers none exercised
@account n=4 entity=test-strategy#4.1 v=proposed
why: docs/capabilities.md gains the handler-coverage claim and its three stated limits at 273-281
@account n=5 entity=test-strategy#9.1 v=proposed
why: stand_handler_coverage.rs moves the mod common declarers 23 to 24, the session_common readers 34 to 35 and the blitz-tests file and target counts by one
@account n=6 entity=test-strategy#10.1 v=not_touched
why: tests/common/mod.rs gains its tables at 335-412 only, leaving LOAD_TAB_ORDER and CELLS_TAB_ORDER as stated; the new tables are F9, a test-data-fixtures fact
@account n=7 entity=unit-test-strategy#1.1 v=not_touched
why: reached.rs holds its tests in an inline mod tests and no out-of-line module is added, so the location rule still holds
@account n=8 entity=unit-test-strategy#4.2 v=proposed
why: cli_commands moves 17 to 18, the line table to 109 rows, and json.rs and cli_tests.rs gain the coverage tests (F8)
@account n=9 entity=unit-test-strategy#5.1 v=not_touched
why: VERBS stays 14, the causes 8 and the schema unchanged, so the help and count row stand
@account n=10 entity=unit-test-strategy#8.1 v=proposed
why: escher-driver gains two unit tests, cli_tests.rs 5 to 6 and json.rs 7 to 8, moving the stated 102
@account n=11 entity=unit-test-strategy#13.1 v=not_touched
why: test_capability_statement.py is byte-identical and its counted tables unchanged, and CI is green
@account n=12 entity=unit-test-strategy#21.1 v=not_touched
why: graph.rs is unchanged; the reached record's tests (F7) are placed beside the bridge record entity
@account n=13 entity=unit-test-strategy#23.1 v=proposed
why: reached::tests and bridge_tests' new reached-record test (F7) sit beside the bridge's dispatch record
@account n=14 entity=unit-test-strategy#24.1 v=not_touched
why: snapshot.rs is unchanged and its clickable pin holds
@account n=15 entity=unit-test-strategy#25.1 v=not_touched
why: element_id.rs gains only the crate-private element_listener_ids, its reserved-form tests and the walk's rule unchanged
@account n=16 entity=unit-test-strategy#26.1 v=not_touched
why: actionable.rs is not reached and its eight tests stand
@account n=17 entity=unit-test-strategy#28.1 v=not_touched
why: slice s03 holds none of the reached files
@account n=18 entity=unit-test-strategy#28.4 v=not_touched
why: the new stand_handler_coverage.rs holds test functions only, no in-file cfg module, per the report's New text
