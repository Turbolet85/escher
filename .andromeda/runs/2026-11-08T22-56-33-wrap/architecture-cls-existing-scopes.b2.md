@answer n=1 entity=existing-scopes#1.26 v=yes mode=stale
why: the session adds a coverage module, making the eighteen private modules nineteen, and adds the coverage role to command_line and Coverage::to_json to json, while the cited lib.rs lines sit in a reached file
@answer n=2 entity=existing-scopes#1.27 v=no
why: the apps/browser modules are outside the session's reached files
@answer n=3 entity=existing-scopes#1.28 v=no
why: the browser persistence crate is outside the session's reached files
@answer n=4 entity=existing-scopes#1.29 v=no
why: the readme app is outside the session's reached files
@answer n=5 entity=existing-scopes#1.30 v=no
why: the wpt runner is outside the session's reached files and the engine decisions it names are not reached
@answer n=6 entity=existing-scopes#1.31 v=yes mode=stale
why: the stand checks gain stand_handler_coverage, so the mod common declarers move from 23 to 24, stand_replay moves from 3 to 4 tests, and the common module gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells()
@answer n=7 entity=existing-scopes#1.32 v=yes mode=add-to
why: the escher-session role set gains the coverage role, and tests/common/mod.rs gains coverage_needles, called by host_log alone, so its list of needle readers is incomplete
@answer n=8 entity=existing-scopes#1.33 v=no
why: the example crates are outside the session's reached files
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root example files are outside the session's reached files
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures are outside the session's reached files and none is changed
@answer n=11 entity=existing-scopes#2.1 v=no
why: the two named served-page checks keep their counts and their session_common reader, and the session names no stand count or reading here
@answer n=12 entity=existing-scopes#3.1 v=yes mode=stale
why: cli.rs and cli_tests.rs are reached files, so the cited cli.rs:536-538 lines can move, and cli_tests.rs grows from 5 to 6 tests with the coverage role
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: the stand checks reading mod common rise from 23 to 24 with stand_handler_coverage, and tests/common/mod.rs gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells()
@answer n=14 entity=existing-scopes#3.3 v=no
why: the predicate reads the stand_ prefix and its two path constants, which the session does not change, and the new check falls inside that prefix
@answer n=15 entity=existing-scopes#3.4 v=no
why: the blitz-vibey-script runtime modules are outside the session's reached files
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification is not a reached file and its three tests are unchanged
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact test files are not reached, and the reached record writes no event, so their event claims hold
@answer n=18 entity=existing-scopes#4.2 v=no
why: the buffered-render witness still holds since the reached record takes no borrow across render_immediate, and the probe's own file is unchanged
@answer n=19 entity=existing-scopes#4.3 v=yes mode=stale
why: the DioxusDocument tests cite dioxus_document.rs:878-916, a reached file that gained record_reached, so those cited lines can move
