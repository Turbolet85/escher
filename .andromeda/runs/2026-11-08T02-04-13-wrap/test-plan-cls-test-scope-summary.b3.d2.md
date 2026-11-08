@answer n=1 entity=test-scope-summary#41.1 v=yes mode=stale
why: stand_requirement_sweep.rs, where these helpers live, is reached whole by the session, so its cited line ranges can have moved
@answer n=2 entity=test-scope-summary#42.1 v=yes mode=stale
why: the sweep test's leg check sits in stand_requirement_sweep.rs, which is reached whole, though ci-leg.sh and test_ci_workflows.py are unchanged and the stand_ glob takes stand_graph with no script change
@answer n=3 entity=test-scope-summary#43.1 v=yes mode=stale
why: the Cells screen moved from 47,089 to 53,859 bytes and from 5 to 6 parts, its lines unmoved at 740
@answer n=4 entity=test-scope-summary#43.2 v=yes mode=stale
why: host_log.rs, which holds the cells-part log check cited here, is reached whole and gained a graph call with its needles
@answer n=5 entity=test-scope-summary#43.3 v=no
why: examples/seven_guis/src/stand.rs and its stand:: unit tests are not reached, the stand options stay pinned, and the session added no stand unit test
@answer n=6 entity=test-scope-summary#43.4 v=yes mode=stale
why: the snapshot's select-option client-rect clause lives in snapshot.rs, which is reached whole with NodeState.clickable added, so the clause can have moved
@answer n=7 entity=test-scope-summary#44.1 v=yes mode=stale
why: cli_commands.rs, host_log.rs and mcp_log.rs, which hold the cited open-list checks, are all reached whole and gained graph calls
@answer n=8 entity=test-scope-summary#45.1 v=yes mode=stale
why: stand_cells_served.rs, which holds the keyboard walk cited at lines 179-214, is reached whole
@answer n=9 entity=test-scope-summary#46.1 v=no
why: the WPT runner and its tests are unreached by the session's change
