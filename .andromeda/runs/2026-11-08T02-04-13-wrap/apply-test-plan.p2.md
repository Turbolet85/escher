@apply_row n=1 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=1
@apply_row n=3 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=1
@apply_row n=4 ref_file=fanout-test-plan--cascade-g1.md ref_n=4 v=applied edit=E2
@apply_row n=5 ref_file=fanout-test-plan--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=4
@apply_row n=6 ref_file=fanout-test-plan--cascade-g1.md ref_n=6 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=4
@apply_row n=7 ref_file=fanout-test-plan--cascade-g1.md ref_n=7 v=applied edit=E3
@apply_row n=8 ref_file=fanout-test-plan--cascade-g1.md ref_n=8 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=7
@apply_row n=9 ref_file=fanout-test-plan--cascade-g1.md ref_n=9 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=7
@apply_row n=10 ref_file=fanout-test-plan--cascade-g1.md ref_n=10 v=folded_into to_file=fanout-test-plan--cascade-g1.md to_n=7
@apply_row n=11 ref_file=fanout-test-plan--cascade-g1.md ref_n=12 v=applied edit=E4
@apply_row n=12 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E5
@apply_row n=13 ref_file=fanout-test-plan--cascade-g2.md ref_n=2 v=applied edit=E6
@apply_row n=14 ref_file=fanout-test-plan--cascade-g2.md ref_n=3 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=1
@apply_row n=15 ref_file=fanout-test-plan--cascade-g2.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g2.md to_n=1
@apply_row n=16 ref_file=fanout-test-plan--cascade-g2.md ref_n=5 v=applied edit=E7
@apply_row n=17 ref_file=fanout-test-plan--cascade-new-test-scope-summary.md ref_n=1 v=applied edit=E8
@apply_row n=18 ref_file=fanout-test-plan--cascade-g4.md ref_n=1 v=applied edit=E9
@fact id=fct-k5y3m6cu expect_rev=1
old: (15 tests, unix only: every driver command run as a process of the binary — §6 → Drivers per surface;
new: (16 tests, unix only: every driver command run as a process of the binary — §6 → Drivers per surface; a `graph` answering one line of the UI graph (examples/seven_guis/tests/cli_commands.rs:467-492);
old: `tests/mcp_server.rs` (6 tests:
new: `tests/mcp_server.rs` (7 tests:
old: offering the verbs in order, twelve,
new: offering the verbs in order, thirteen, `graph` after `lint`,
old: answers a `lint` tool call with its line of findings,
new: answers a `lint` tool call with its line of findings and a `graph` tool call with its line of the UI graph (examples/seven_guis/tests/mcp_server.rs:339-360),
old: called by the same two (examples/seven_guis/tests/common/mod.rs:454-482)
new: called by the same two (examples/seven_guis/tests/common/mod.rs:454-482), and `graph_needles()`, the component and state names of the UI graph as needles, called by the same two, each making one graph call and reading 0 id, name and graph needles at `trace` under both builds (examples/seven_guis/tests/common/mod.rs:425-452)

@fact id=fct-nks3g3e7 expect_rev=1
old: its count row reads `[12, 13, 12, 3, 8, 4, 15]`
new: its count row reads `[13, 14, 12, 3, 8, 4, 15]`
old: eight instance-level (`screenshot` the seventh, after `scroll`, and `lint` the eighth, after `screenshot`)
new: nine instance-level (`screenshot` the seventh, after `scroll`, `lint` the eighth, after `screenshot`, and `graph` the ninth, after `lint`)
old: `help` at the tool level, twelve;
new: `help` at the tool level, thirteen;
old: the thirteen node keys are named in order
new: the fourteen node keys are named in order, `clickable` after `focused`
old: the listing is the twelve verbs of the table,
new: the listing is the thirteen verbs of the table, `graph` after `lint`,

@fact id=fct-i5a6vv3t expect_rev=2
old: 83 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md)
new: 100 unit tests in eleven files (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
old: the crate-private `mutation_buffer.rs` holding none;
new: the crate-private `mutation_buffer.rs` holding none; nine in `graph::tests` on the UI graph reader `DioxusDocument::graph`, compiled only under the crate's default `accessibility` feature, and four in `signal_registry::tests` on the state registry, under no feature gate (packages/dioxus-native-dom/src/graph.rs:333-671; packages/dioxus-native-dom/src/signal_registry.rs:216-374);
old: thirteen on the stable element id
new: fifteen on the stable element id
old: an edit outside the anchor keeps the anchored id (packages/dioxus-native-dom/src/element_id.rs:410-776)
new: an edit outside the anchor keeps the anchored id, and two on the reserved form `{owner}///{key}` over an adversarial page: no id the walk writes holds the separator, and `virtual_node_id` spells the form and names no element (packages/dioxus-native-dom/src/element_id.rs:410-776)
old: twenty on the snapshot builder
new: twenty-one on the snapshot builder
old: and a select's value and its options as its children (above)
new: and a select's value and its options as its children (above), and an element with an app `click` listener reading `clickable`, true where the element's current listeners hold `click` and it is no disabled form control (packages/dioxus-native-dom/src/snapshot.rs:1038-1059)
old: and before `focused`, since that chunk)
new: and before `focused`, since that chunk, and `clickable` after `focused`, written only when true)
old: seven on the bridge (six until 2026-10-16-fast-failing-mutants)
new: eight on the bridge
old: (packages/dioxus-native-dom/src/dioxus_document_tests.rs:48-240)
new: (packages/dioxus-native-dom/src/dioxus_document_tests.rs:48-240), and a dispatch recorded where a state is registered and acting as before where none is, driven through the test-only `DioxusDocument::dispatch_to_vdom` (packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322)

@fact id=fct-rjsmqegm expect_rev=1
old: and mutation_writer.rs since 2026-10-13-upstream-sync-polish-and-ship;
new: and mutation_writer.rs since 2026-10-13-upstream-sync-polish-and-ship, and graph.rs and signal_registry.rs;

@fact id=fct-z2ynupl3 expect_rev=3
old: — measured and pinned, the snapshot field for it deferred, `NODE_FIELDS` still thirteen)
new: — measured and pinned, the snapshot's click reading `NodeState.clickable`, `NODE_FIELDS` fourteen)
old: `stand_mcp_tools` 6
new: `stand_mcp_tools` 7
old: parenting its `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:709-769);
new: parenting its `command` span (tests/blitz-tests/tests/stand_mcp_tools.rs:709-769); a `graph` tool call answering the reader's graph and parenting one `command` span recording `verb` alone, in both layout modes (tests/blitz-tests/tests/stand_mcp_tools.rs:771-843);
old: eight verbs read back;
new: eight verbs read back; it reads the ninth instance verb, `graph`, too, one call per mode, its `command` line recording `verb` alone (tests/blitz-tests/tests/stand_act_spans.rs:230-247);

@fact after=fct-z2ynupl3 file=test-scope-summary gid=new:non-lean-clickable kind=procedure subject=scope-entities
- **tests/blitz-tests — `stand_non_lean_tasks`' `CLICKABLE` table:** beside its role-less count, the file counts per task the elements reading `clickable` that state no role (`clickable_without_role`) against a stated 7-row table, `CLICKABLE`, in both layout modes — Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676, its data cells, `cells-grid` not among them (tests/blitz-tests/tests/stand_non_lean_tasks.rs:66-75; tests/blitz-tests/tests/stand_non_lean_tasks.rs:117-127) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-tz35iqmq expect_rev=1
old: a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
new: a screen of 53,859 bytes in 740 lines, 6 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-4q3czvm2 file=test-scope-summary gid=new:ui-graph-unconfirmed kind=procedure subject=scope-entities
- **dioxus-native-dom (UI graph) — the `unconfirmed` flag's limit:** `DioxusDocument::graph` reads a declared write `unconfirmed` once a dispatch of the handler's event passed through its element and no run wrote the state, so a write its handler makes only on a condition reads `unconfirmed` after runs that skipped it, a stated limit no check reaches; the stand's `GRAPH_DRIVES` drives each conditional write once, so no stand write reads `unconfirmed` (tests/blitz-tests/tests/common/mod.rs:243-323) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-7m7lxsmy expect_rev=2
old: read through `mod common;` by 21 `stand_*.rs` checks
new: read through `mod common;` by 22 `stand_*.rs` checks
old: by 32 checks, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md (`stand_act_popup.rs`, `stand_cells_served.rs` and `stand_load_page.rs` the latest;
new: by 33 checks, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md (`stand_graph.rs` the latest, then `stand_act_popup.rs`, `stand_cells_served.rs` and `stand_load_page.rs`;
old: 158 `.rs` files, 157 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the latest `dioxus_runtime_probe.rs`, then
new: 159 `.rs` files, 158 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md, the latest `stand_graph.rs`, then `dioxus_runtime_probe.rs`, then

@pattern id=tp2-01
fixed: (15 tests, unix only
@pattern id=tp2-02
fixed: offering the verbs in order, twelve
@pattern id=tp2-03
fixed: [12, 13, 12, 3, 8, 4, 15]
@pattern id=tp2-04
fixed: eight instance-level
@pattern id=tp2-05
fixed: the thirteen node keys are named
@pattern id=tp2-06
fixed: the listing is the twelve verbs
@pattern id=tp2-07
fixed: 83 unit tests in nine files
@pattern id=tp2-08
fixed: thirteen on the stable element id
@pattern id=tp2-09
fixed: twenty on the snapshot builder
@pattern id=tp2-10
fixed: seven on the bridge
@pattern id=tp2-11
fixed: the snapshot field for it deferred
@pattern id=tp2-12
fixed: `stand_mcp_tools` 6
@pattern id=tp2-13
fixed: 47,089 bytes
@pattern id=tp2-14
fixed: 158 `.rs` files, 157 test targets
@pattern id=tp2-15
fixed: by 21 `stand_*.rs` checks
@pattern id=tp2-16
fixed: read through `mod session_common;` by 32 checks
