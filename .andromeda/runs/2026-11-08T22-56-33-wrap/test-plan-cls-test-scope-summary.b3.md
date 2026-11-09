@answer n=1 entity=test-scope-summary#41.1 v=no
why: the session does not reach stand_keyboard.rs, and its Cells and load-page keyboard sequences are unchanged
@answer n=2 entity=test-scope-summary#42.1 v=no
why: the session does not reach stand_contrast.rs, and the load page and Cells ratio tests are unchanged
@answer n=3 entity=test-scope-summary#43.1 v=no
why: the session does not reach stand_requirement_sweep.rs, and its sweep helpers and path-prefix constants are unchanged
@answer n=4 entity=test-scope-summary#44.1 v=no
why: the session does not reach ci-leg.sh or test_ci_workflows.py, and the stand-repeat leg lists are unchanged
@answer n=5 entity=test-scope-summary#45.1 v=yes mode=stale
why: stand_handler_coverage adds a third check on served pages declaring mod session_common, so the two-check count moves
@answer n=6 entity=test-scope-summary#45.2 v=yes mode=stale
why: host_log.rs is reached whole and gains a coverage test (its tests move 5 to 6), so the cited lines 228-371 can move
@answer n=7 entity=test-scope-summary#45.3 v=no
why: the session does not reach examples/seven_guis/src/stand.rs, and its unit tests are unchanged
@answer n=8 entity=test-scope-summary#45.4 v=no
why: the session does not reach lint.rs or stand_lint, and the select-option lint reading is unchanged
@answer n=9 entity=test-scope-summary#46.1 v=yes mode=stale
why: cli_commands.rs and host_log.rs are reached whole and gain tests, so the cited lines 425-470 and 373-484 can move
@answer n=10 entity=test-scope-summary#47.1 v=no
why: the session does not reach stand_cells_served.rs, and its keyboard walk and key names are unchanged
@answer n=11 entity=test-scope-summary#48.1 v=no
why: the session does not reach the wpt runner, and its unit-test counts are unchanged
