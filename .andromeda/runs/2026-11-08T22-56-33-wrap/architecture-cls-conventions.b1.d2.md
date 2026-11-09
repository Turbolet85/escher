@answer n=1 entity=conventions#1.1 v=no
why: the fmt and clippy gate commands and their CI legs are unchanged, and none of its cited sites is a reached file
@answer n=2 entity=conventions#2.1 v=no
why: the manifests stay byte-identical with no crate, dependency or feature added, so the cited manifest lines still hold
@answer n=3 entity=conventions#3.1 v=no
why: the new modules and public items follow the module and item doc rule, and none of its cited sites is a reached file
@answer n=4 entity=conventions#4.1 v=no
why: the session adds no TODO or FIXME marker, and none of its cited sites is a reached file
@answer n=5 entity=conventions#5.1 v=no
why: the new internals are pub(crate) as the rule asks, and none of its cited sites is a reached file
@answer n=6 entity=conventions#6.1 v=yes mode=add-to
why: the new dioxus-native-dom reached module and element_listener_ids are ungated and join signal_registry, and lib.rs is reached so its cited lines may have moved
@answer n=7 entity=conventions#7.1 v=yes mode=stale
why: dioxus_document.rs is reached whole and handle_event now writes the reached record, so the cited bridge borrow lines 168-173 and 316-321 may have moved
@answer n=8 entity=conventions#8.1 v=yes mode=add-to
why: the blitz-tests common module gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells(), and stand_handler_coverage joins the stand checks per F7
@answer n=9 entity=conventions#9.1 v=yes mode=stale
why: cli.rs is reached whole and gains the coverage role, so its cited test-module lines 536-538 may have moved, and cli_tests.rs grows from 5 to 6 tests
@answer n=10 entity=conventions#9.2 v=no
why: fixtures/mod.rs is not reached and the new check reads served tasks and common tables, not fixture pages, so the one-reader rule holds
@answer n=11 entity=conventions#9.3 v=no
why: the mcp listing test and the requirement sweep row walk sit in files the session does not reach
@answer n=12 entity=conventions#10.1 v=no
why: the binary, Android, Store and example CSS rules sit in main.rs and app files the session does not reach, and it adds no binary
@answer n=13 entity=conventions#11.1 v=no
why: the seven_guis task registration and its writer consts sit in files the session does not reach, and use_graph_state stays unchanged
@answer n=14 entity=conventions#12.1 v=no
why: no examples/assets HTML fixture is reached or added by the session
