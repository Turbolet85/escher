@answer n=1 entity=standard-contracts#17.1 v=no
why: the seven_guis stand module it cites (stand.rs, app.rs, lib.rs, tasks/timer.rs) is not among the session's changed files
@answer n=2 entity=standard-contracts#18.1 v=no
why: the typography cites seven_guis stand.rs, lib.rs and Cargo.toml, none of which the session changed
@answer n=3 entity=standard-contracts#19.1 v=no
why: boot_cells cites seven_guis stand.rs:131-139, which the session did not change
@answer n=4 entity=standard-contracts#19.2 v=no
why: boot_load cites seven_guis stand.rs:141-244, which the session did not change
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet cites seven_guis stand.rs:154-175, which the session did not change
@answer n=6 entity=standard-contracts#19.4 v=no
why: the graph hooks cite seven_guis tasks/mod.rs, which the session did not change, and the use_graph_state contract is unchanged
@answer n=7 entity=standard-contracts#20.1 v=no
why: escher-telemetry is outside the session, which adds no span, event or log field
@answer n=8 entity=standard-contracts#21.1 v=yes mode=stale
why: the driver crate-root re-export count moves from 59 to 62 names (Coverage, HandlerRef, handler_coverage), and the command line gains the coverage role (F1) read from lib.rs, cli.rs and json.rs, with its own usage clause
@answer n=9 entity=standard-contracts#22.1 v=no
why: the recording cites recording.rs and session.rs, neither changed, and the kept-check and answer writers keep their form since Coverage::to_json only joins json.rs
@answer n=10 entity=standard-contracts#22.2 v=yes mode=stale
why: Check::replay now runs through the crate-private replay_each in replay.rs, moving the cited replay.rs lines 21-149 while its behaviour stays unchanged
@answer n=11 entity=standard-contracts#22.3 v=no
why: the lint verb, its schema and Session::lint sit outside the session's reached files, and the verb count stays fourteen
@answer n=12 entity=standard-contracts#22.4 v=no
why: the load wait lives in session.rs and execute.rs, neither changed, and the five acting verbs are unchanged
@answer n=13 entity=standard-contracts#23.1 v=no
why: the select-option overlap reading lives in lint.rs and the stand_lint tests, none of which the session changed
@answer n=14 entity=standard-contracts#23.2 v=no
why: the load-page witness cites stand_load_page.rs, which the session did not change
@answer n=15 entity=standard-contracts#23.3 v=yes mode=stale
why: the graph writer cited at json.rs:242-332 sits in a reached file that gains Coverage::to_json, so its cited lines can have moved
@answer n=16 entity=standard-contracts#23.4 v=yes mode=stale
why: the cited stated limit sits at docs/capabilities.md:293-296, a reached file that gains three stated limits (F6), so the cited lines can have moved
@answer n=17 entity=standard-contracts#23.5 v=yes mode=stale
why: GRAPH_DRIVES is cited at tests/blitz-tests/tests/common/mod.rs:254-334, a reached file that gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells(), so the cited lines can have moved
@answer n=18 entity=standard-contracts#23.6 v=no
why: the actions read cites actions.rs and the schema's verb table, neither of which the session reached
@answer n=19 entity=standard-contracts#23.7 v=no
why: check_target and locate sit in execute.rs, which the session did not change
@answer n=20 entity=standard-contracts#23.8 v=no
why: the actions read's non-mutation is held in actions.rs and stand_actions.rs, neither of which the session changed
@answer n=21 entity=standard-contracts#23.9 v=yes mode=stale
why: the cited stated limit sits at docs/capabilities.md:282-284 and 297-299, a reached file that gains three stated limits (F6), so the cited lines can have moved
@answer n=22 entity=standard-contracts#24.1 v=no
why: blitz-vibey-script is outside the session's changes
@answer n=23 entity=standard-contracts#25.1 v=no
why: document_may_load and the fetch path live in blitz-vibey-script, which the session did not change
@answer n=24 entity=standard-contracts#26.1 v=no
why: the CSS property accessors live in blitz-vibey-script, which the session did not change
@answer n=25 entity=standard-contracts#26.2 v=no
why: URLSearchParams and its WPT quarantine lines sit in blitz-vibey-script and wpt, which the session did not change
