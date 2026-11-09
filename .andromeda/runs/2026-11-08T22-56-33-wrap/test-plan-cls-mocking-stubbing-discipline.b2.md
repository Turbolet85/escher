@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: the wpt_diff_to_pr script and its test are not among the files the session reaches
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: blitz-dom is not reached by the session, so the hover_state_tests claim still holds
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=no
why: no Cargo manifest, CI workflow or CI script moved, so the searched build and CI files are unchanged
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=yes mode=stale
why: the examples/seven_guis test files the session reaches whole sit in the searched apps slice, so its file count and absence reading are not shown to hold
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=yes mode=stale
why: the dioxus-native-dom and blitz-tests files the session reaches whole sit in the searched slice, so its file count and absence reading are not shown to hold
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=yes mode=stale
why: the blitz-tests files the session reaches whole, including the new stand_handler_coverage.rs, sit in the searched slice, so its file count and absence reading are not shown to hold
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=yes mode=stale
why: the escher-driver files the session reaches whole, including the new coverage.rs, sit in the searched slice, so its file count and absence reading are not shown to hold
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=yes mode=stale
why: the escher-driver files the session reaches whole, including the new coverage.rs, sit in the searched slice, so its file count and absence reading are not shown to hold
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=yes mode=stale
why: the dioxus-native-dom files the session reaches whole, including the new reached.rs, sit in the searched slice, so its file count and absence reading are not shown to hold
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=yes mode=stale
why: the session's reached files in the searched slice, including the new reached.rs and stand_handler_coverage.rs, are not shown to leave its file count and absence reading unchanged
