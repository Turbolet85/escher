@apply_row n=1 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g1.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g3.md to_n=1
@apply_row n=3 ref_file=fanout-test-plan.md ref_n=1 v=applied edit=E2
@apply_row n=4 ref_file=fanout-test-plan.md ref_n=2 v=applied edit=E3
@apply_row n=5 ref_file=fanout-test-plan.md ref_n=3 v=refused
why: fct-egnkfdyv stands in the key file contracts/session-lifecycle, which is not among this batch's target files (body, test-scope-summary, test-strategy, unit-test-strategy), so its text and rev were never read here
@apply_row n=6 ref_file=fanout-test-plan.md ref_n=4 v=refused
why: fct-76fzwvj7 stands in the key file contracts/session-lifecycle, which is not among this batch's target files (body, test-scope-summary, test-strategy, unit-test-strategy), so its text and rev were never read here
@apply_row n=7 ref_file=fanout-test-plan.md ref_n=5 v=refused
why: fct-adkwyvky stands in the category file integration-test-strategy, which is not among this batch's target files (body, test-scope-summary, test-strategy, unit-test-strategy), so its text and rev were never read here
@apply_row n=8 ref_file=fanout-test-plan.md ref_n=6 v=refused
why: fct-prdavblg stands in the category file ci-integration, which is not among this batch's target files (body, test-scope-summary, test-strategy, unit-test-strategy), so its text and rev were never read here
@apply_row n=9 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E4
@apply_row n=10 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=applied edit=E5
@apply_row n=11 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=applied edit=E6
@apply_row n=12 ref_file=fanout-test-plan--cascade-g1.md ref_n=5 v=applied edit=E7
@apply_row n=13 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E8
@apply_row n=14 ref_file=fanout-test-plan--cascade-g4.md ref_n=1 v=applied edit=E9
@apply_row n=15 ref_file=fanout-test-plan--cascade-g4.md ref_n=2 v=applied edit=E10
@apply_row n=16 ref_file=fanout-test-plan--cascade-g4.md ref_n=3 v=applied edit=E11
@apply_row n=17 ref_file=fanout-test-plan--cascade-g4.md ref_n=4 v=applied edit=E12
@apply_row n=18 ref_file=fanout-test-plan--cascade-g4.md ref_n=5 v=applied edit=E13
@apply_row n=19 ref_file=fanout-test-plan--cascade-g4.md ref_n=6 v=applied edit=E14
@apply_row n=20 ref_file=fanout-test-plan--cascade-g4.md ref_n=7 v=applied edit=E15

@fact id=fct-jydhmppz expect_rev=4
old: the bridge and its dispatch record, the UI graph reader
new: the bridge, its dispatch record and the reached record, the UI graph reader

@fact id=fct-glytb3w3 expect_rev=3
old: passed 244 · failed 0 · ignored 7 over 52 files, `stand_graph` a file its `stand_` prefix picks up with no script change — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: passed 262 · failed 0 · ignored 7 over 54 files, `stand_handler_coverage` a file its `stand_` prefix picks up with no script change — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-6bs5asmb expect_rev=2
old: `run stand` 244 · 0 · 7 over 52 files, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md
new: `run stand` 262 · 0 · 7 over 54 files, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-k5y3m6cu expect_rev=3
old: (examples/seven_guis/tests/common/mod.rs:481-503); read through `mod common;` by all eight)
new: (examples/seven_guis/tests/common/mod.rs:481-503), and `coverage_needles()`, the CRUD screen's needles read from its UI graph, called by `host_log` alone (examples/seven_guis/tests/common/mod.rs:454-479); read through `mod common;` by all eight)
old: `tests/host_log.rs` (5 tests, unix only, the fourth and the open list's below;
new: `tests/host_log.rs` (6 tests, unix only, the fifth and the open list's below, the fourth a `coverage crud` run over a CRUD session kept through the binary with a typed sentinel and a row click, reading 0 id, name, coverage, sentinel, path and recorded needles at `trace` under both builds (examples/seven_guis/tests/host_log.rs:141-226);
old: `tests/cli_commands.rs` (17 tests, unix only: every driver command run as a process of the binary — §6 → Drivers per surface;
new: `tests/cli_commands.rs` (18 tests, unix only: every driver command run as a process of the binary — §6 → Drivers per surface; a `coverage` role run over a Counter check the binary kept, answering its coverage line with exit 0 (`coverage_replays_a_kept_check_and_lists_the_handlers_it_left`, examples/seven_guis/tests/cli_commands.rs:522-571);

@fact id=fct-nks3g3e7 expect_rev=5
old: 102 unit tests in seventeen files, 66 inline and 36 in four `#[path]` test files
new: 104 unit tests in seventeen files, 67 inline and 37 in four `#[path]` test files
old: json.rs 7 (a string is written with its escapes;
new: json.rs 8 (a string is written with its escapes;
old: since that chunk a changed node with `differs` after `bounds`), cli_tests.rs 5
new: since that chunk a changed node with `differs` after `bounds`; a coverage line keyed per handler, an absent owner left out), cli_tests.rs 6
old: 102 rows, the `mcp` role's three, `graph`'s and `actions`'s among them;
new: 109 rows, the `mcp` role's three, `graph`'s, `actions`'s and the `coverage` role's among them;
old: a line that is no command or is refused runs nothing) client.rs 3
new: a line that is no command or is refused runs nothing; the `coverage` role reads its checks, a missing file, a non-check, one past the bound and another app's check each refused `malformed`, beside a failing boot and an accepted two-check run) client.rs 3
old: by `stand_actions` (8 tests over the six served screens, both layout modes), not by a unit test
new: by `stand_actions` (8 tests over the six served screens, both layout modes), and `handler_coverage` by `stand_handler_coverage` (8 tests over the six served tasks, both layout modes), not by a unit test

@fact id=fct-i5a6vv3t expect_rev=4
old: 104 unit tests in eleven files (as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md)
new: 109 unit tests in twelve files (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md)
old: signal_registry.rs:216-374); a select reading
new: signal_registry.rs:216-374); four in `reached::tests` on the reached record, under no feature gate — a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, and the record's `Debug` printing its count alone (packages/dioxus-native-dom/src/reached.rs:108-210); a select reading
old: and eight on the bridge, in the
new: and nine on the bridge, in the
old: (packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322) — as measured
new: (packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322), and the reached record holding on a page with no registered state (`the_reached_record_holds_on_a_page_with_no_registered_state`, packages/dioxus-native-dom/src/dioxus_document_tests.rs:324-354) — as measured

@fact id=fct-rjsmqegm expect_rev=2
old: and graph.rs and signal_registry.rs; stylo_taffy still holds none
new: and graph.rs, signal_registry.rs and reached.rs; stylo_taffy still holds none

@fact id=fct-z2ynupl3 expect_rev=5
old: the committed checks replayed — `stand_replay` 3, declaring
new: the committed checks replayed — `stand_replay` 4, declaring
old: and neither check holds a `png` key or a shot's first 64 base64 characters (tests/blitz-tests/tests/stand_replay.rs:84-113;
new: neither check holds a `png` key or a shot's first 64 base64 characters, and a recorded Counter session with two `actions` steps replays on a fresh Counter, all 4 steps running, while its copy with one `keyed` flipped stops at step 3, keys exactly `nodes`, in both layout modes (`a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes`, tests/blitz-tests/tests/stand_replay.rs:263-329) (tests/blitz-tests/tests/stand_replay.rs:84-113;

@fact id=fct-5d4ttzku expect_rev=1
old: (tests/blitz-tests/tests/stand_replay.rs:27-39; examples/seven_guis/tests/recording.rs:187-203)
new: (tests/blitz-tests/tests/stand_replay.rs:27-39; examples/seven_guis/tests/recording.rs:187-203); `Check::replay` runs through the crate-private `Check::replay_each` with a no-op hook, and `handler_coverage` is its second caller — it replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step and lists the handlers no check exercised (`escher-session coverage <app> <check>...`), so the recorded checks also measure what they leave unexercised (packages/escher-driver/src/replay.rs:47-55; packages/escher-driver/src/coverage.rs:64-102)

@fact id=fct-krjmr356 expect_rev=1
old: the `select` model's limits in its four bullets (docs/capabilities.md:256-264; docs/capabilities.md:300-303),
new: the `select` model's limits in its four bullets (docs/capabilities.md:256-264; docs/capabilities.md:300-303), handler coverage's limits in three — a handler inside a canvas not counted, a handler on no screen a replay reached not listed, a bubbling dispatch counting every ancestor listener it passed (docs/capabilities.md:273-281) — the handler-coverage claim pinned by `stand_handler_coverage` and `cli_commands`,

@fact id=fct-7m7lxsmy expect_rev=4
old: read through `mod common;` by 23 `stand_*.rs` checks
new: read through `mod common;` by 24 `stand_*.rs` checks
old: by 34 checks, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md (`stand_actions.rs` the latest, then `stand_graph.rs`
new: by 35 checks, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md (`stand_handler_coverage.rs` the latest, then `stand_actions.rs`, then `stand_graph.rs`
old: 160 `.rs` files, 159 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md, the latest `stand_actions.rs`, then
new: 161 `.rs` files, 160 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md, the latest `stand_handler_coverage.rs`, then `stand_actions.rs`, then

@fact id=fct-klxgwgcy expect_rev=4
old: a line read by its rules in order (61 rows)
new: a line read by its rules in order (109 rows)
old: seven_guis' `cli_commands` (17)
new: seven_guis' `cli_commands` (18)

@fact after=fct-qc3s4qp3 file=unit-test-strategy gid=new:escher-driver-coverage kind=procedure subject=unit
- **escher-driver (the coverage line and role):** json.rs's `a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner` pins the coverage line's form, `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent, and cli_tests.rs's `the_coverage_role_reads_its_checks_refusing_any_it_cannot_read` pins a missing file, a non-check, one past the bound and another app's check each refused `malformed`, a failing boot and an accepted two-check run, its scratch files under the workspace `target/tmp` (packages/escher-driver/src/json.rs:1245-1274; packages/escher-driver/src/cli_tests.rs:614-674) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-fwtnmbsl expect_rev=1
old: escher-driver's unit tests read 102 — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
new: escher-driver's unit tests read 104 — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-mi2j5ztx file=unit-test-strategy gid=new:dioxus-native-dom-reached kind=procedure subject=unit
- **dioxus-native-dom (the reached record):** `reached::tests` holds 4 tests pinning the reached record — a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, and the record's `Debug` printing its count alone — and `bridge_tests::the_reached_record_holds_on_a_page_with_no_registered_state` pins the record on a page where no state is registered (packages/dioxus-native-dom/src/reached.rs:108-210; packages/dioxus-native-dom/src/dioxus_document_tests.rs:324-354) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@pattern id=tp1-01
fixed: passed 244 · failed 0 · ignored 7 over 52 files
@pattern id=tp1-02
fixed: 244 · 0 · 7 over 52 files
@pattern id=tp1-03
fixed: (17 tests, unix only
@pattern id=tp1-04
fixed: (5 tests, unix only, the fourth
@pattern id=tp1-05
fixed: 102 unit tests in seventeen files
@pattern id=tp1-06
fixed: cli_tests.rs 5
@pattern id=tp1-07
fixed: json.rs 7
@pattern id=tp1-08
fixed: is read by its rules in order — 102 rows
@pattern id=tp1-09
fixed: 104 unit tests in eleven files
@pattern id=tp1-10
fixed: eight on the bridge
@pattern id=tp1-11
fixed: `stand_replay` 3
@pattern id=tp1-12
fixed: by 23 `stand_*.rs` checks
@pattern id=tp1-13
fixed: read through `mod session_common;` by 34 checks
@pattern id=tp1-14
fixed: 160 `.rs` files, 159 test targets
@pattern id=tp1-15
fixed: in order (61 rows)
@pattern id=tp1-16
fixed: `cli_commands` (17)
@pattern id=tp1-17
fixed: unit tests read 102
