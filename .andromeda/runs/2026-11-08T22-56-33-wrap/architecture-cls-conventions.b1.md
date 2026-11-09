@answer n=1 entity=conventions#1.1 v=no
why: the session adds no lint allow or gate, and the cited lint sites sit outside the reached files
@answer n=2 entity=conventions#2.1 v=no
why: manifests stay byte-identical, and the cited lib.rs:1 docs.rs cfg is not moved by a re-export and a module line
@answer n=3 entity=conventions#3.1 v=no
why: the new files follow the stated //! and /// rules and bring no new documentation pattern, and the cited files are unreached
@answer n=4 entity=conventions#4.1 v=no
why: the session adds no TODO or FIXME marker, and the cited marker sites are outside the reached files
@answer n=5 entity=conventions#5.1 v=no
why: the new internals are pub(crate) as the convention states, and the cited sites are outside the reached files
@answer n=6 entity=conventions#6.1 v=yes mode=add-to
why: task item 10 makes the new reached module and element_listener_ids ungated, joining the ungated dioxus-native-dom modules listed beside signal_registry
@answer n=7 entity=conventions#7.1 v=yes mode=stale
why: the handle_event edit in reached dioxus_document.rs, beside the render and replay borrow, can move the cited lines 168-173 and 316-321
@answer n=8 entity=conventions#8.1 v=yes mode=add-to
why: F7 adds stand_handler_coverage and HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells() to the blitz-tests common module this entity describes
@answer n=9 entity=conventions#9.1 v=yes mode=stale
why: the coverage role adds lines to reached cli.rs above the cited cli::tests declaration at lines 536-538
@answer n=10 entity=conventions#9.2 v=no
why: the session's shared helpers go to common and not to a fixture page, and the cited fixtures/mod.rs is unreached
@answer n=11 entity=conventions#9.3 v=no
why: the cited mcp_tests.rs and stand_requirement_sweep.rs are unreached, and the session adds no row-walk test
@answer n=12 entity=conventions#10.1 v=no
why: the session adds no app binary or example CSS, and the cited examples are unreached
@answer n=13 entity=conventions#11.1 v=no
why: the session registers no new task signal, and the cited tasks files are unreached
@answer n=14 entity=conventions#12.1 v=no
why: the session adds no HTML fixture, and the cited assets are unreached
