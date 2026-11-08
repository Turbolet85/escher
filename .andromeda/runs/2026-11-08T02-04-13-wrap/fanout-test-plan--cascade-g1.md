@proposal n=1 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-wgoi5qhk
section: ## 1. Test Scope Summary
change: `tests/cli_commands.rs` reads 16 tests, 15 until 2026-11-07-ui-graph added `graph_answers_one_line_of_the_ui_graph`, the `graph` verb run as a process answering one line of the UI graph
sidecar: test-scope-summary: cli_commands 15 → 16 tests with the graph verb's process test (2026-11-07-ui-graph)
rationale: the report's New text lists cli_commands.rs 467-492 `fn graph_answers_one_line_of_the_ui_graph()`, and its Outcome names it as the command-line leg of the acceptance
@proposal n=2 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-e44heebj
section: ## 1. Test Scope Summary
change: `tests/mcp_server.rs` reads 7 tests, `tools/list` offering the verbs in order, thirteen (`graph` after `lint`), and since 2026-11-07-ui-graph `a_graph_tool_call_answers_its_line_of_the_ui_graph`; 6 tests and twelve verbs until then
sidecar: test-scope-summary: mcp_server 6 → 7 tests and its listing twelve → thirteen verbs (2026-11-07-ui-graph)
rationale: the report's Schema / config moves VERBS 12 → 13 with `graph` listed after `lint`, and its New text lists mcp_server.rs 339-360 holding the graph tool-call test its Outcome names
@proposal n=3 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-vzxdnwb7
section: ## 1. Test Scope Summary
change: the shared `tests/common/mod.rs` holds since 2026-11-07-ui-graph `graph_needles()`, the component and state names of the UI graph as needles, called by `host_log` and `mcp_log`, each of which makes one graph call and reads 0 id, name and graph needles at `trace` under both builds
sidecar: test-scope-summary: seven_guis' common module gains graph_needles, read by host_log and mcp_log (2026-11-07-ui-graph)
rationale: the report's New text lists common/mod.rs 425-452 `pub fn graph_needles()`, and Coverage of new surfaces and Outcome (obs) state host_log and mcp_log read 0 id, name and graph needles with one graph call each
@proposal n=4 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 1. Test Scope Summary
change: describe.rs 2 — its count row reads `[13, 14, 12, 3, 8, 4, 15]` (`[12, 13, 12, 3, 8, 4, 15]` until 2026-11-07-ui-graph)
sidecar: test-scope-summary: describe's count row [12, 13, …] → [13, 14, 12, 3, 8, 4, 15] (2026-11-07-ui-graph)
rationale: the report's Schema / config moves describe's count row to `[13, 14, 12, 3, 8, 4, 15]`, and Counts / qualifiers moved names this row (pattern describe-row) stale
@proposal n=5 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 1. Test Scope Summary
change: schema_tests.rs 7 — the verbs are named in order, each at its level: nine instance-level (`screenshot` the seventh, `lint` the eighth, since 2026-11-07-ui-graph `graph` the ninth, after `lint`), then `start` · `status` · `stop`, then `help` at the tool level, thirteen (twelve until then); the fourteen node keys are named in order (thirteen until then, `clickable` after `focused`)
sidecar: test-scope-summary: schema_tests' verbs twelve → thirteen, instance verbs eight → nine, node keys thirteen → fourteen (2026-11-07-ui-graph)
rationale: the report's Schema / config moves VERBS 12 → 13, instance verbs 8 → 9 and NODE_FIELDS 13 → 14 with `clickable` after `focused`; the escher-driver count stays 100 because the new cases extend existing tests' rows, among them the renamed verb-order test
@proposal n=6 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-brwfx6hx
section: ## 1. Test Scope Summary
change: mcp_tests.rs 15 — the listing is the thirteen verbs of the table (twelve until 2026-11-07-ui-graph, `graph` after `lint`)
sidecar: test-scope-summary: mcp_tests' listing twelve → thirteen verbs (2026-11-07-ui-graph)
rationale: the report's Outcome (tests) reads the MCP listing 13 tools with `graph` after `lint`, mcp_tests.rs edited at 399, 408 and 413-416, and escher-driver still 100 passed
@proposal n=7 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 1. Test Scope Summary
change: dioxus-native-dom: 100 unit tests in eleven files (83 in nine until 2026-11-07-ui-graph, which added 17) — among them since that chunk nine in the new `graph::tests`, the UI graph reader compiled under `accessibility`, and four in the new `signal_registry::tests`, under no feature gate
sidecar: test-scope-summary: dioxus-native-dom unit tests 83 → 100, graph::tests 9 and signal_registry::tests 4 new (2026-11-07-ui-graph)
rationale: the report's Counts / qualifiers moved reads dnd-units 83 → 100 (entry 1, «100 passed») and names this row stale; its New text lists graph.rs tests 401-670 (nine test functions) and signal_registry.rs tests 272-373 (four)
@proposal n=8 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rg7py446
section: ## 1. Test Scope Summary
change: fifteen on the stable element id (thirteen until 2026-11-07-ui-graph) — beside the six and the seven, two on the reserved form `{owner}///{key}` over an adversarial page: no id the walk writes holds the separator, and `virtual_node_id` spells the form and names no element
sidecar: test-scope-summary: element_id tests thirteen → fifteen with the reserved-form pair (2026-11-07-ui-graph)
rationale: the report's New text lists element_id.rs 753-761 `no_id_the_walk_writes_holds_the_virtual_separator` and 763-775 `a_virtual_node_id_spells_the_reserved_form_and_names_no_element`, and Outcome (arch) reads them met
@proposal n=9 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-lhgd3uie
section: ## 1. Test Scope Summary
change: twenty-one on the snapshot builder (twenty until 2026-11-07-ui-graph) — since that chunk `an_element_with_an_app_click_listener_reads_clickable`, `clickable` true where the element's current listeners hold `click` and it is no disabled form control; the text form's state tokens end ` focused clickable`, `clickable` written only when true
sidecar: test-scope-summary: snapshot builder tests twenty → twenty-one with clickable, and the clickable text token (2026-11-07-ui-graph)
rationale: the report's New text lists snapshot.rs 1038-1059 `fn an_element_with_an_app_click_listener_reads_clickable()` and snapshot_text.rs 128-130 `if node.state.clickable`; Symbols / APIs states the token ` clickable` after `focused`, written only when true
@proposal n=10 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-twghj2ei
section: ## 1. Test Scope Summary
change: eight on the bridge (seven until 2026-11-07-ui-graph) — since that chunk a dispatch recorded where a state is registered and acting as before where none is, driven through the `cfg(test)` `DioxusDocument::dispatch_to_vdom`
sidecar: test-scope-summary: bridge tests seven → eight with the dispatch-record test (2026-11-07-ui-graph)
rationale: the report's New text lists dioxus_document_tests.rs 269-322 `a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` and dioxus_document.rs 240-250 `dispatch_to_vdom`, test-only per Symbols / APIs
@proposal n=11 detector=D-cascade severity=warning file=body anchor=fct-jydhmppz basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## 1. Test Scope Summary
change: dioxus-native-dom: unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id, the actionable-key check, the bridge, the mutation writer, the document's events and, since 2026-11-07-ui-graph, the UI graph reader and the signal registry — the rest of the line as it stands
sidecar: test-plan §1: dioxus-native-dom's unit-test line names the UI graph reader and the signal registry (2026-11-07-ui-graph)
rationale: the report's Crates / modules adds `graph` and `signal_registry` to dioxus-native-dom, and Coverage of new surfaces reads them tested at unit level (graph::tests, signal_registry::tests)
@proposal n=12 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-rjsmqegm basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zkn5zjmx
section: ## 1. Test Scope Summary
change: matches only in dioxus-native-dom: dioxus_document.rs and events.rs at that search, element_id.rs, snapshot.rs, actionable.rs, snapshot_text.rs, snapshot_diff.rs and dioxus_document_tests.rs since, mutation_writer.rs since 2026-10-13-upstream-sync-polish-and-ship, and graph.rs and signal_registry.rs since 2026-11-07-ui-graph; dioxus-native and stylo_taffy still hold none
sidecar: test-scope-summary: the dioxus-native-dom test matches add graph.rs and signal_registry.rs (2026-11-07-ui-graph)
rationale: the report's Changes lists graph.rs and signal_registry.rs as new dioxus-native-dom files, each holding a `mod tests`, and Dependencies states dioxus-native unchanged by its gate
@remark
F8 (the CLICKABLE table) and F13 (the unconfirmed flag's stated limit) no picked entity carries — the new-detail pass places them; §3's body still reads «the twelve verbs» and «exactly the twelve mcp__escher__ tools» for the cold-agent pipe (fct-63656bwh, fct-szqqqnrt) and run stand 237 · 0 · 7 over 51 files (fct-glytb3w3, fct-6bs5asmb), which move to thirteen and 244 · 0 · 7 over 52 — not this group's, for the body or obs-plan pass (D-tests-obs-harness)
