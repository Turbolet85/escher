@apply_row n=1 ref_file=fanout-test-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g1.md ref_n=11 v=folded_into to_file=fanout-test-plan.md to_n=5
@apply_row n=7 ref_file=fanout-test-plan--cascade-g2.md ref_n=6 v=folded_into to_file=fanout-test-plan.md to_n=5
@apply_row n=8 ref_file=fanout-test-plan.md ref_n=6 v=applied edit=E6
@apply_row n=9 ref_file=fanout-test-plan.md ref_n=7 v=applied edit=E7
@apply_row n=10 ref_file=fanout-test-plan.md ref_n=8 v=applied edit=E8
@apply_row n=11 ref_file=fanout-test-plan.md ref_n=9 v=applied edit=E9
@apply_row n=12 ref_file=fanout-test-plan.md ref_n=14 v=refused
why: its place, fct-nks3g3e7, stands in the category file test-scope-summary, not among this batch's targets (the doc:test-plan body alone); the body holds no «64 rows» of `cli_tests.rs`, and the report states no row count of it
@apply_row n=13 ref_file=fanout-test-plan.md ref_n=15 v=refused
why: its places, fct-klxgwgcy, fct-csryo2zo, fct-gtzxj2kr and fct-egnkfdyv, stand in the category files unit-test-strategy, e2e-test-strategy and contracts/session-lifecycle, not among this batch's targets (the doc:test-plan body alone); the body states no `cli_commands` or `mcp_server` test count
@fact id=fct-63656bwh expect_rev=1
old: `escher-session mcp` (the twelve verbs)
new: `escher-session mcp` (the thirteen verbs)
old: admits only `tool` ∈ the twelve verbs
new: admits only `tool` ∈ the thirteen verbs

@fact id=fct-szqqqnrt expect_rev=1
old: the init tools exactly the twelve `mcp__escher__{verb}` tools
new: the init tools exactly the thirteen `mcp__escher__{verb}` tools

@fact id=fct-glytb3w3 expect_rev=2
old: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s
new: `run stand`'s `run.end` reads passed 244 · failed 0 · ignored 7 over 52 files, `stand_graph` a file its `stand_` prefix picks up with no script change — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md; `run all`'s

@fact id=fct-xxpbx2ay expect_rev=1
old: examples/seven_guis/tests/mcp_log.rs:194-207)
new: examples/seven_guis/tests/mcp_log.rs:194-207), and `graph` a word of the same two closed sets, no span field added — a graph call's `command` line records `verb` alone under its `tool_call` parent, and the host's and the `mcp` role's stderr at `trace` hold 0 id, name and graph needles (the component and state names a graph answers) under both builds (`host_log`, `mcp_log`; examples/seven_guis/tests/host_log.rs:543-548; examples/seven_guis/tests/mcp_log.rs:218-226)

@fact id=fct-jydhmppz expect_rev=2
old: the stable element id, the actionable-key check, the bridge, the mutation writer
new: the stable element id and its reserved virtual form, the actionable-key check, the bridge and its dispatch record, the UI graph reader, the signal registry, the mutation writer

@fact id=fct-i47d76zr expect_rev=1
old: no function under test gained a constructor, a `pub` or a `cfg(test)` shim (packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-6; packages/dioxus-native-dom/src/dioxus_document_tests.rs:185-240)
new: no function under test gained a constructor or a `pub`, and the one `cfg(test)` shim, `DioxusDocument::dispatch_to_vdom`, dispatches a chain into the vdom for the bridge test `a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` (packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-6; packages/dioxus-native-dom/src/dioxus_document_tests.rs:185-240; packages/dioxus-native-dom/src/dioxus_document.rs:240-250; packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322)

@fact id=fct-og7dxjok expect_rev=1
old: (tests/blitz-tests/tests/common/mod.rs:65-91; tests/blitz-tests/tests/common/mod.rs:131-134)
new: (tests/blitz-tests/tests/common/mod.rs:65-91; tests/blitz-tests/tests/common/mod.rs:131-134), and `GRAPH_DRIVES` with its `Drive` enum (each stand task's drives, seven rows, every conditional write driven once so no stand write reads `unconfirmed`, read by `stand_graph`) (tests/blitz-tests/tests/common/mod.rs:233-323)

@fact id=fct-prxcxiqp expect_rev=1
old: `lint()`, `snapshot_part(n)`
new: `lint()`, `graph()`, `snapshot_part(n)`
old: `linted(session, call)` (a lint's findings, `None` for a refused call or any other outcome),
new: `linted(session, call)` (a lint's findings, `None` for a refused call or any other outcome), `graphed(session, call)` (a graph's `UiGraph`, `None` for a refused call or any other outcome; tests/blitz-tests/tests/session_common/mod.rs:134-147),

@fact id=fct-gq2zcc44 expect_rev=1
old: a password typed through the driver never leaves in an answer or as pixels
new: a password typed through the driver never leaves in an answer or as pixels; the `graph` verb reads as `DioxusDocument::graph` on every served screen in both layout modes and changes nothing (`stand_graph`)

@pattern id=tp1-01 ignore_case=true
fixed: the twelve verbs
@pattern id=tp1-02
fixed: exactly the twelve `mcp__escher__
@pattern id=tp1-03
regex: 237 · (failed )?0 · (ignored )?7 over 51 files
@pattern id=tp1-04
fixed: a `pub` or a `cfg(test)` shim
