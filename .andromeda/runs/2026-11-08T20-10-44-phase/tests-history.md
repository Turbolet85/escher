@history_item n=1 marker=amd-i4iccah5
**amd-i4iccah5** — (2026-11-08-what-can-i-do-here) added the integration detail that no test exercises a kept `actions` step's replay: `Check::replay` holds by construction, unmeasured, and the replay test is owed as a CARRY on «Handler coverage». This chunk writes that test, and it discharges this entry's detail. test-plan states no matrix status, so v030-06's `verified` stays the ledger's to record.

@history_item n=2 marker=amd-7qex4fkt
**amd-7qex4fkt** — (2026-10-19-recorded-check-replay) stated the kept record (`tool` · `arguments` · `masked` · `accepted` · `digest` · `answer`) and `Check::replay`: each step runs through `Session::run`, the replay stops at the first acceptance or kept answer that differs, and nothing runs on another app's session. It also added `stand_replay` (red at step 1 and step 3 on a test-local Counter variant) and the committed checks under `tests/blitz-tests/checks/` as fixtures. This is the precedent the owed actions-step replay test follows, its red-path shape included.

@history_item n=3 marker=amd-siqngma2
**amd-siqngma2** — (2026-10-19-recorded-check-replay) added replay.rs's 5 unit tests and removed `session_common::replay` in favour of `Check::replay`. It made `recording` compare the committed checks byte for byte against what `record_checks.sh` records, and `stand_replay` joined stand-repeat. It also set the same-host limit: shot PNG bytes differ across OSes, so a check compared byte for byte holds no `screenshot` step. That limit binds any committed check with an `actions` step.

@history_item n=4 marker=amd-xliprnr6
**amd-xliprnr6** — (2026-10-19-session-recording) added `Session::recording_into` (unix only, `0700` directory, `check-{n}.json` `0600`), the 400-record / 1 MiB bound and `Check::from_bytes`. A check that records is `cfg(unix)`-gated item by item, beside an off-unix `Unsupported` check. The owed record-then-replay test must be gated this way or must read a committed check.

@history_item n=5 marker=amd-3unj565m
**amd-3unj565m** — (2026-10-19-session-recording) added `stand_session_recording` 3 and seven_guis `recording`, and gave `session_common` `kept` and the unix-only `mode` readers. Recording runs joined stand-repeat (files 42 → 43) and the flakiness budget. This sets the pattern for where a recording test lives and which shared readers it uses.

@history_item n=6 marker=amd-d7nbibtj
**amd-d7nbibtj** — (2026-11-07-ui-graph) re-recorded the two committed checks through `record_checks.sh` and dropped their byte sizes. Their line counts stay stated, `counter-cli.json` 8 and `counter-mcp.json` 11. A new committed check with an `actions` step would move this list and must stay byte-equal to what the binary records.

@history_item n=7 marker=amd-n3udqhi7
**amd-n3udqhi7** — (2026-11-08-what-can-i-do-here) added `stand_actions` 8 over the six served screens in both layout modes. The read changes nothing and holds no typed text, mask or value, and an argument is refused `malformed`. `Session::actions` and `check_target` are covered by this check, not by a unit test. This is the actions answer the replay test keeps and compares, and the read-only pattern the coverage read must match.

@history_item n=8 marker=amd-jjah2vzy
**amd-jjah2vzy** — (2026-11-08-what-can-i-do-here) added `ACTIONABLE_AT_BOOT` in `tests/common/mod.rs` (counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2). The Cells actions answer measures 106,088 B, and `stand_actions` holds it within the answer bound. A kept actions step on Cells would carry an answer of that size inside the 1 MiB recording bound.

@history_item n=9 marker=amd-ldelf34f
**amd-ldelf34f** — (2026-11-08-what-can-i-do-here) brought `actions` to the command line and MCP: `cli_commands` 17, `mcp_server` 8 (fourteen tools, `actions` after `graph`), `stand_mcp_tools` 8. `stand_mcp_tools` holds the shared helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`. This is the precedent for every pin a new coverage verb would move.

@history_item n=10 marker=amd-epp6lr5n
**amd-epp6lr5n** — (2026-11-08-what-can-i-do-here) made `host_log` (9 calls, 9 command lines) and `mcp_log` (12 tool calls, 10 command lines) each drive one `actions` call, 0 needles at `trace`. It added `actions_needles()` and raised `stand_act_spans` to 38 calls per mode, `actions` the tenth instance verb. These are the counts a new coverage verb moves next.

@history_item n=11 marker=amd-dkb4oe4y
**amd-dkb4oe4y** — (2026-11-08-what-can-i-do-here) raised `VERBS` to 14 (ten instance-level) and `describe`'s row to `[14, 14, 12, 3, 8, 4, 15]`. Field kinds read thirteen, cli_tests 102 rows, and the listing test is renamed `the_listing_is_the_fourteen_verbs_of_the_table`. schema_tests keys field kinds per verb, with 12 `amended` rows. A fifteenth verb moves all of these together, the renamed test and the complexity-ceiling note included.

@history_item n=12 marker=amd-7otlq3ax
**amd-7otlq3ax** — (2026-11-08-what-can-i-do-here) put escher-driver at 102 unit tests in seventeen files, 66 inline, with actions.rs 2. This is the base count to restate if the chunk adds driver unit tests.

@history_item n=13 marker=amd-lfaykhfb
**amd-lfaykhfb** — (2026-11-08-what-can-i-do-here) raised stand-repeat and the flakiness budget to 53 `stand_*.rs` files and the local `fast` baseline to 223 result lines, 0 failed. Coverage passed under the floors with the per-crate figures not read; the last stated figures stay those of 2026-11-07-mounteddata-buffered-render. These are the bases a new `stand_` file and new crate code move.

@history_item n=14 marker=amd-sid6zlb7
**amd-sid6zlb7** — (2026-11-08-what-can-i-do-here) set the shared-module reader counts: `mod common;` 23, `session_common` 34, `fixtures` 32 (24 `stand_*`). blitz-tests reads 160 `.rs` files and 159 targets. A new check file moves these counts.

@history_item n=15 marker=amd-q5lvjmtv
**amd-q5lvjmtv** — (2026-11-08-what-can-i-do-here) moved the cold-agent pipe, `valid_step`, `isolated` and the test fake to fourteen tools, the test count staying 37. On the new-verb fork that count moves again. Per scope, it moves under «Cold-agent bar», not in this chunk.

@history_item n=16 marker=amd-zsfczxyi
**amd-zsfczxyi** — (2026-11-07-ui-graph) set the precedent for adding a read verb to the cold-agent pipe: it joined the pipe's `VERBS` and the fake answers it empty, with no verdict requiring it of a cold agent. A coverage verb would follow that form if it is ever added there.

@history_item n=17 marker=amd-x5hh3g7d
**amd-x5hh3g7d** — (2026-11-08-what-can-i-do-here) put dioxus-native-dom at 104 unit tests in eleven files and actionable.rs at eight. One rule serves both readers, and its listener leg moved from the `data-dioxus-id` mark to the exact current listeners. A removed listener is read by neither reader, the mark outliving it. A handler-coverage count must key on current listeners in the same way, and a test the chunk adds restates the 104.

@history_item n=18 marker=amd-2ho3wh7t
**amd-2ho3wh7t** — (2026-11-04-dioxus-runtime-probe) measured that the bridge's listener attributes over-report: a listener removed on a re-render keeps its attribute, and `mounted` writes none. The vdom walk's dynamic attributes read current listeners exactly, and `GlobalSignal` subscribers give the readers. This is the trap a handler-coverage count must avoid: never count listener attributes.

@history_item n=19 marker=amd-poiocitr
**amd-poiocitr** — (2026-11-07-ui-graph) added `stand_graph` 6 in both modes. A graph read changes nothing and holds no typed text, the `graph` verb equals `DioxusDocument::graph`, and the Cells answer stays within the bound. The answer holds names and flags only, a sentinel 0 times, and an argument is refused `malformed`. The coverage read must meet the same read-only and no-value rules, and `stand_graph` must stay green.

@history_item n=20 marker=amd-z23dex63
**amd-z23dex63** — (2026-11-07-ui-graph) added `GRAPH_DRIVES` with its `Drive` enum to `tests/common/mod.rs` (seven rows, each conditional write driven once) and the `graph()` builder and `graphed()` reader to `session_common`. This is the ready-made "checks that exercised" fixture. A new reader or builder goes into the shared module.

@history_item n=21 marker=amd-z6dzej35
**amd-z6dzej35** — (2026-11-07-ui-graph) raised `run stand` to 244 · 0 · 7 over 52 files with `stand_graph` picked up by its prefix, and made `run all`'s 743 · 0 · 12 a stated non-re-count. This is the base reading the chunk's stand recount restates.

@history_item n=22 marker=amd-xep3xm3z
**amd-xep3xm3z** — (2026-11-07-ui-graph) gave dioxus-native-dom's unit scope the bridge's dispatch record, the graph reader and the signal registry. It added the one `cfg(test)` shim `DioxusDocument::dispatch_to_vdom`, with no function under test gaining a constructor or `pub`. A unit test of the new (element, event) exercised record must reuse that shim under the same rule.

@history_item n=23 marker=amd-a2uwkadt
**amd-a2uwkadt** — (2026-11-07-ui-graph) added the unit groups for the graph. `graph::tests` covers current listeners (a removed one gone, confirmed after a dispatch). `signal_registry::tests` covers a page with no registered state holding no registry, and the bridge test covers a dispatch recorded where a state is registered and acting as before where none is. The exercised record must keep all of these holding and must work on an app with no registry.

@history_item n=24 marker=amd-ckioxq2y
**amd-ckioxq2y** — (2026-11-07-ui-graph) stated the `unconfirmed` limit: a write a handler makes only on a condition reads `unconfirmed` after runs that skipped it, and no check reaches this. The one existing "reached" reading is a dispatch through the element, so the new exercised record must not move this limit.

@history_item n=25 marker=amd-jiyzcs5d
**amd-jiyzcs5d** — (2026-11-07-ui-graph) recorded that stand state reaches the UI graph only through `tasks::use_graph_signal` / `SignalRegistry`. Hook-local `use_signal` exposes no link. This is why «exercised» needs a record that holds with no registry.

@history_item n=26 marker=amd-halqoyxj
**amd-halqoyxj** — (2026-11-07-ui-graph) measured the Cells `graph` answer at 293,869 B, unpaged, the stated limit «An unpaged graph» owned by `res-7cc5bi4k`. A coverage answer over the same graph is held to the same answer bound in both modes.

@history_item n=27 marker=amd-tvsd3b6b
**amd-tvsd3b6b** — (2026-11-07-ui-graph) made `graph` a word of the closed `tool_call`/`command` sets with no span field added, and gave `host_log`/`mcp_log` one graph call each, 0 needles at `trace` under both builds. It added `graph_needles()`. This is the precedent a new coverage verb follows in the log checks.

@history_item n=28 marker=amd-o5bpvnvg
**amd-o5bpvnvg** — (2026-11-07-ui-graph) added `cli_commands` `graph_answers_one_line_of_the_ui_graph` and `mcp_server`'s graph tool call, `tools/list` ordered by `VERBS`. These are the e2e pins a new verb extends.

@history_item n=29 marker=amd-afxaahjb
**amd-afxaahjb** — (2026-11-07-ui-graph) moved the verb-table pins from twelve to thirteen: schema_tests, field kinds, `describe`'s row, the mcp listing, json.rs and cli_tests rows. This is the precedent for moving every count pin together.

@history_item n=30 marker=amd-7efxs2hi
**amd-7efxs2hi** — (2026-11-07-ui-graph) added `graph` to the session contract as an instance verb that runs no step, `Outcome::Graphed`. A recording keeps every instance verb's call. A read verb for coverage would enter session-start, session-binary and session-proof the same way.

@history_item n=31 marker=amd-5zqg4udh
**amd-5zqg4udh** — (2026-11-07-ui-graph) recorded the graph answer crossing the socket as a boundary widening, ratified provisionally by the founder (ids, names, event names, flags, never a value or `NodeId`). A new coverage verb or carrier content is likewise a widening, the founder's, per the scope's P4 fork.

@history_item n=32 marker=amd-472linzq
**amd-472linzq** — (2026-10-25-capability-statement) added `test_capability_statement.py`, which pins `docs/capabilities.md`'s verbs and five counts in words, and README's tool count. `## Where each claim is proven` names each claim's check. A fifteenth verb, the coverage claim and its canvas limit all go through this pin.

@history_item n=33 marker=amd-kmiudnmn
**amd-kmiudnmn** — (2026-11-07-mounteddata-buffered-render) inverted the probe's two pins to `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, the buffered render's discriminating witness on Dioxus 0.7. It also stated dioxus-core's `Work::PollTask` path. Any exercised record added in `dioxus_document.rs` must keep these pins green.

@history_item n=34 marker=amd-jmojotdw
**amd-jmojotdw** — (2026-11-07-mounteddata-buffered-render) made `initial_build` and `poll` render into `MutationBuffer` with the document unborrowed, `poll` borrowing once after `render_immediate`. An exercised record in the bridge must take no borrow across `render_immediate`.

@history_item n=35 marker=amd-f6w3v2eb
**amd-f6w3v2eb** — (2026-11-07-mounteddata-buffered-render) stated the buffered render's limit: a task polled during render sees the previous render's document, owned by an open residual. The new record must leave this limit as it stands.

@history_item n=36 marker=amd-6kqtriic
**amd-6kqtriic** — (2026-11-07-mounteddata-buffered-render) kept upstream's `element_removed_before_its_mounted_event_is_flushed` out, owned by `res-6ssktw6x` / `cry-mjwr45xr`, so the `6ef4b532d` guard has no test of its own. The chunk reaches `dioxus_document.rs` but must not move this.

@history_item n=37 marker=amd-x4m5gjdg
**amd-x4m5gjdg** — (2026-11-07-mounteddata-buffered-render) put dioxus-native-dom at 83 unit tests with upstream's ported `task_polled_during_render_can_access_document` in `dioxus_document.rs`'s module, `mutation_buffer.rs` holding none. That ported test must stay green beside the new record.

@history_item n=38 marker=amd-g4rjfxtm
**amd-g4rjfxtm** — (2026-11-07-mounteddata-buffered-render) put dioxus-native-dom at 2937/3387 = 86.71 % against its 83 % floor, the last stated per-crate figure. New bridge code is read against this margin.

@history_item n=39 marker=amd-jy34nxjf
**amd-jy34nxjf** — (2026-11-07-mounteddata-buffered-render) re-read the per-OS tallies on CI#42371051720 and set the rule that a tally move is attributed by grepping the test's own name. The chunk's new tests move every job's tally under this rule.

@history_item n=40 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — (2026-10-14-quality-gates) set the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure stand-repeat flakiness budget, on the founder's word. New stand checks and new crate code are bound by both.

@history_item n=41 marker=amd-h4ip3qlm
**amd-h4ip3qlm** — (2026-10-19-test-splits) recorded that a `*_tests.rs` `#[path]` file is no record of the per-crate reading. A unit test placed in `dioxus_document_tests.rs` (`bridge_tests`) counts toward no coverage, so the exercised record's coverage must come from tests the reading counts.

@history_item n=42 marker=amd-xpqvmmzd
**amd-xpqvmmzd** — (2026-10-17-upstream-sync-engine-debts) excluded dioxus-native-dom's `keyboard_compat.rs` from both llvm-cov lines by filename, the floors unchanged. That is the basis of the dioxus-native-dom reading the new record must keep above 83 %.

@history_item n=43 marker=amd-ur7tfp4h
**amd-ur7tfp4h** — (2026-10-07-audit-corrections) placed the bridge tests in the `cfg(test)` child module `dioxus_document::bridge_tests` (`dioxus_document_tests.rs`), a child because `DioxusEventHandler`'s fields are private. It also opened the shared stand module. This is where a dispatch-record unit test is pinned.

@history_item n=44 marker=amd-nlavi4cc
**amd-nlavi4cc** — (2026-10-19-shared-fixtures) added `tests/fixtures/mod.rs` with `boot` and `hold`, and set the rule that a page two checks share goes in `fixtures`. The test-data bootstrap was measured: nothing is seeded, and there is no `run fixtures` selection. New session checks hold sessions through it or through `session_common`.

@history_item n=45 marker=amd-t5mve4e6
**amd-t5mve4e6** — (2026-10-07-act-by-id) set the rule that a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads. The new coverage and replay checks inherit this rule.

@history_item n=46 marker=amd-2gzx73ga
**amd-2gzx73ga** — (2026-10-06-headless-stand) made stand checks assert unconditionally under the bundled DejaVu Sans, with no skip path. This is the anti-pattern the new stand checks must not break.

@history_item n=47 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — (2026-10-18-one-virtual-clock) made `advance` accepted on every session, moving the clock by exactly `ms`. `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep in a session check. A replay or coverage check moves time only through `advance`.

@history_item n=48 marker=amd-tem72y7f
**amd-tem72y7f** — (2026-10-20-load-wait) added the second exception to «no sleep and no wall-clock read» (`stand_load_wait`, whose subject is wall time) and the load wait inside acting verbs. A check that is not about wall time takes neither.

@history_item n=49 marker=amd-u7jdserc
**amd-u7jdserc** — (2026-10-10-mcp-surface) added the `tool_call` span and `stand_mcp_tools`, with a standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test. This applies if a coverage verb joins `stand_mcp_tools`/`stand_act_spans`.

@history_item n=50 marker=amd-s53lzeiq
**amd-s53lzeiq** — (2026-10-07-driver-command-spans) set the rule that a check reading a sink's capture counts lines by target, never in total, and that a green per-package run is not the workspace leg's green. This applies to any span or log check extended for a new verb.

@history_item n=51 marker=amd-cixe5kae
**amd-cixe5kae** — (2026-10-24-stand-requirement-sweep) made stand-repeat run named non-`stand_` targets as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds`, while `run stand` keeps its prefix rule. A coverage or replay check without the `stand_` prefix would need this list and its pin moved to enter the flakiness budget.
