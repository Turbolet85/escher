@answer n=1 entity=mocking-stubbing-discipline#1.26 v=no
why: its GitHub API dry-run claim lives in wpt_diff_to_pr.py and its test, which the session does not reach and the WPT runner is unchanged
@answer n=2 entity=mocking-stubbing-discipline#2.1 v=no
why: its hover_state_tests claim lives in blitz-dom's document.rs, and blitz-dom is unchanged by the session
@answer n=3 entity=mocking-stubbing-discipline#3.1 v=yes mode=stale
why: the search scope takes in .github/scripts/*.py, which holds the reached test_cold_agent.py whose fake now answers fourteen tools, and the report does not re-search that changed file
@answer n=4 entity=mocking-stubbing-discipline#3.2 v=yes mode=stale
why: the apps search is not shown to exclude the reached examples/seven_guis test files, whose cli, mcp and host-log tests changed
@answer n=5 entity=mocking-stubbing-discipline#3.3 v=yes mode=stale
why: the slice s03 search is not shown to exclude the reached escher-driver and blitz-tests files, which changed whole and gained the new actions.rs and stand_actions.rs
@answer n=6 entity=mocking-stubbing-discipline#3.4 v=yes mode=stale
why: the slice s06 fixture search takes in the reached fixtures module and its readers, whose declarer and reader counts moved from 31 to 32 and 23 to 24
@answer n=7 entity=mocking-stubbing-discipline#3.5 v=yes mode=stale
why: the slice s07 fixture search is not shown to exclude the reached examples/seven_guis test files, which changed whole
@answer n=8 entity=mocking-stubbing-discipline#3.6 v=yes mode=stale
why: the no-match slice s08 search is not shown to hold on the reached docs, README and cold-agent script, which changed whole and gained new text
@answer n=9 entity=mocking-stubbing-discipline#3.7 v=yes mode=stale
why: the slice s09 search is not shown to exclude the reached escher-driver and blitz-tests files, which changed whole and gained new files
@answer n=10 entity=mocking-stubbing-discipline#3.8 v=yes mode=stale
why: the slice s11 search is not shown to exclude the reached driver, blitz-tests and example test files, which changed whole
