# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · yes · the three pseudo_element_flow_classification tests are a conflict surface run in both layout modes, and incremental_oracle is their layout-identity witness, which a layout resolution must keep
integration-test-strategy#2.1 · yes · dom.rs and selection.rs run on the script engine, and bootstrap.rs and events.rs are conflict surfaces where the take's URLSearchParams and the resolved conflicts land
integration-test-strategy#3.1 · yes · the one-change-per-radio-click rule lives in runtime/events.rs, a conflict surface, so the resolution must still synthesize no change
integration-test-strategy#4.1 · yes · the held-instance stand checks (settle, load wait, scrolled-box bounds in scrolling.rs, the select and cells paths) are re-proven in both layout modes on the synced engine, the proofs the layout, node, scrolling and mutator resolutions must pass
integration-test-strategy#4.2 · no · the session host and socket lifecycle are escher-owned code outside every reached file, named not needed by the task; the rule states the lifecycle, not the cells task that the host serves
integration-test-strategy#4.3 · yes · the 0-needle capture checks (host_log, mcp_log, telemetry_drop, stand_act_spans, the session quiet checks) are named by the Needs, and the sink allowlist is re-read against the taken delta
integration-test-strategy#5.1 · yes · host_log's select-list test at trace is one of the standing 0-needle capture checks the Needs name, and the select list is a merge surface of sixteen files
integration-test-strategy#6.1 · yes · the hosted select list, its shot equal to the in-process one and a row chosen by click, is select-list behaviour through the host, on a merge surface the chunk re-proves as every escher check
integration-test-strategy#7.1 · yes · stand_act_select's multiple-select rules in both layout modes are named by the Needs as the select model and its list
integration-test-strategy#8.1 · yes · stand_act_popup is named by the Needs as the select list's standing proof, in both layout modes, over the merge-surface select list
integration-test-strategy#9.1 · yes · select_popup is named by the Needs, and its one key-press row is the edited unit the founder's fork restores, guarding the node.rs:1775 panic
integration-test-strategy#10.1 · yes · select_popup_painted is named by the Needs and is the font-embolden witness the local gate joins, read in both layout modes
integration-test-strategy#11.1 · no · a search reading of where the apps keep their tests; the chunk adds no app integration test, and upstream's files it brings go under tests/blitz-tests (S1), so nothing here binds
