@answer n=1 entity=unit-test-strategy#25.1 v=no
why: the absence of #[test] in slice s03 names no file the chunk edits; escher-driver, which the chunk extends, carries its own unit tests per the needs list, so s03 is not that crate
@answer n=2 entity=unit-test-strategy#25.2 v=no
why: the test-less files of slice s07 are not named in the scope or the file marks; dioxus-native-dom carries a coverage floor, so its reached files are not the test-less slice
@answer n=3 entity=unit-test-strategy#25.3 v=no
why: blitz-vibey-script's source has no test module; the chunk does not reach apps/ or any script engine, which the not_needed list names
@answer n=4 entity=unit-test-strategy#25.4 v=no
why: no in-file cfg(test) module in tests/blitz-tests; the chunk's new checks are stand_*.rs integration files and its unit tests sit in escher-driver source, so the absence stands
