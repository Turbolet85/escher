@answer n=1 entity=standard-contracts#17.1 v=no
why: the headless stand boot, options and time step are unchanged and examples/seven_guis/src/stand.rs is not in the session's reached files
@answer n=2 entity=standard-contracts#18.1 v=no
why: the stand's DejaVu Sans typography is unchanged and its stand.rs and seven_guis lib.rs are not reached
@answer n=3 entity=standard-contracts#19.1 v=no
why: boot_cells and its cells_root are unchanged and stand.rs is not reached
@answer n=4 entity=standard-contracts#19.2 v=no
why: boot_load and the load page are unchanged and stand.rs is not reached
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet and LOAD_ANSWER_DELAY are unchanged and stand.rs is not reached
@answer n=6 entity=standard-contracts#19.4 v=no
why: the graph hooks in tasks/mod.rs are unchanged and that file is not reached
@answer n=7 entity=standard-contracts#20.1 v=no
why: the session adds no span, event or log field and the telemetry bootstrap is untouched, per the task's untouched items
@answer n=8 entity=standard-contracts#21.1 v=yes mode=stale
why: the escher-driver crate-root re-export count moves from 59 to 62 (Coverage, HandlerRef, handler_coverage, F2), the command line gains the coverage role with its own usage clause (F1), and Coverage::to_json joins the json writer
@answer n=9 entity=standard-contracts#22.1 v=yes mode=add-to
why: F1, the escher-session coverage role, reads each path bounded at MAX_CHECK_BYTES + 1 then through Check::from_bytes and refuses an unreadable, over-bound or non-check file malformed holding nothing of the path
@answer n=10 entity=standard-contracts#22.2 v=yes mode=stale
why: replay.rs is reached and Check::replay now runs through the crate-private replay_each with a no-op hook (behaviour unchanged), with handler_coverage (F2) as its second caller, so the cited replay.rs lines moved
@answer n=11 entity=standard-contracts#22.3 v=no
why: the lint verb, its Finding types and lint.rs are unchanged, and the command line change touches only the role set
@answer n=12 entity=standard-contracts#22.4 v=no
why: LOAD_WAIT stays re-exported at the crate root, since the 59 to 62 additions are Coverage, HandlerRef and handler_coverage, and its settle rules in session.rs are unchanged
@answer n=13 entity=standard-contracts#23.1 v=no
why: the select-option overlap reading lives in lint.rs and stand_lint.rs, neither reached, and the session changes no lint rule
@answer n=14 entity=standard-contracts#23.2 v=no
why: the load wait's served witness in stand_load_page.rs is unchanged and that file is not reached
@answer n=15 entity=standard-contracts#23.3 v=yes mode=stale
why: json.rs is reached (Coverage::to_json joins it) so the graph's written form cited at json.rs:242-332 may have moved, though the graph writer itself is unchanged
@answer n=16 entity=standard-contracts#23.4 v=yes mode=stale
why: docs/capabilities.md is reached and gains three stated limits, so the cited limit lines 293-296 may have moved
@answer n=17 entity=standard-contracts#23.5 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached and gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells(), so the cited GRAPH_DRIVES lines 254-334 may have moved
@answer n=18 entity=standard-contracts#23.6 v=no
why: the actions read in actions.rs and its stand_actions test are unchanged and no reached file is cited
@answer n=19 entity=standard-contracts#23.7 v=no
why: check_target and the shared target check in execute.rs are unchanged and that file is not reached
@answer n=20 entity=standard-contracts#23.8 v=no
why: the actions read's no-change guarantee in actions.rs and stand_actions.rs is unchanged and neither file is reached
@answer n=21 entity=standard-contracts#23.9 v=yes mode=stale
why: docs/capabilities.md is reached and gains three coverage limits, so the cited limit lines 282-284 and 297-299 may have moved
@answer n=22 entity=standard-contracts#24.1 v=no
why: the JavaScript blitz-vibey-script contracts are not reached, per the task's untouched items
@answer n=23 entity=standard-contracts#25.1 v=no
why: document_may_load and the JavaScript fetch rules are not reached by the session
@answer n=24 entity=standard-contracts#26.1 v=no
why: the CSS accessor bootstrap in blitz-vibey-script is not reached by the session
@answer n=25 entity=standard-contracts#26.2 v=no
why: URLSearchParams and its WPT quarantine rows are not reached by the session
