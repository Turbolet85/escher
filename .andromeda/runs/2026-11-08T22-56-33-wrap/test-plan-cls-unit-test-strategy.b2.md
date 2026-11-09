@answer n=1 entity=unit-test-strategy#25.1 v=yes mode=stale
why: the session reaches element_id.rs whole, so the cited lines 739-792 and 42-47 may have moved and the earlier measurement does not show the claim holds on the changed file
@answer n=2 entity=unit-test-strategy#26.1 v=yes mode=add-to
why: F7 (reached::tests and bridge_tests' reached-record test) joins the dioxus-native-dom unit-test description of the actionable reader, which the session's item 10 says gains the reached-handler record
@answer n=3 entity=unit-test-strategy#27.1 v=no
why: the session does not reach the WPT runner, and none of its files is in the reached set
@answer n=4 entity=unit-test-strategy#28.1 v=no
why: slice s03 is outside the session's reached files, and the session adds no unit test in that tree
@answer n=5 entity=unit-test-strategy#28.2 v=no
why: slice s07 sits in an untouched tree that the session does not reach
@answer n=6 entity=unit-test-strategy#28.3 v=no
why: blitz-vibey-script is not reached by the session and its source files are untouched
@answer n=7 entity=unit-test-strategy#28.4 v=yes mode=stale
why: the session reaches tests/blitz-tests files whole (common/mod.rs, stand_replay.rs) and adds stand_handler_coverage.rs there, so the 61-file search over that tree no longer holds as written
