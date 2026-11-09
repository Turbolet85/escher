@answer n=1 entity=unit-test-strategy#25.1 v=yes mode=stale
why: element_id.rs is reached whole by the session, so its cited line ranges (739-792, 42-47) and the reserved-form test reading may have moved
@answer n=2 entity=unit-test-strategy#26.1 v=yes mode=add-to
why: task item 10 gives the actionable reader description the reached-handler record (F7, reached::tests and bridge_tests)
@answer n=3 entity=unit-test-strategy#27.1 v=no
why: the WPT runner is not reached by the session (task item 7 lists it as untouched)
@answer n=4 entity=unit-test-strategy#28.1 v=yes mode=stale
why: the reached whole file examples/seven_guis/src/session_host.rs may sit in slice s03, so its no-test search over the 32 files must be re-run
@answer n=5 entity=unit-test-strategy#28.2 v=no
why: slice s07 is outside the session's reached files (blitz-side surface, untouched per task item 7)
@answer n=6 entity=unit-test-strategy#28.3 v=no
why: blitz-vibey-script is not reached by the session (task item 7)
@answer n=7 entity=unit-test-strategy#28.4 v=yes mode=stale
why: the session adds stand_handler_coverage.rs under tests/blitz-tests and reaches stand_replay.rs and common/mod.rs, so the 61-file search over that tree moves
