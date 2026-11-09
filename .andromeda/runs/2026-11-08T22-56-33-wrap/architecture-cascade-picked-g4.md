@account n=1 entity=standard-contracts#23.4 v=not_touched
why: the report leaves graph.rs and stand_graph unchanged, so the unpaged graph, its Cells size and its limit still hold
@account n=2 entity=standard-contracts#23.5 v=not_touched
why: common/mod.rs gained only lines 335-412 after GRAPH_DRIVES and stand_graph stayed green, so its claim on stand_graph and the drive table still holds
@account n=3 entity=standard-contracts#23.9 v=proposed
why: its claim on actions holds, but handler_coverage (F2) and its stated limits (F6) are new Driver session rows that read the same UI graph and go beside it
@account n=4 entity=standard-contracts#28.1 v=not_touched
why: the report says the record is written after the data-dioxus-id early return and leaves routing, snapshot, diff and unkeyed_actionable as they were, so the claims on dioxus_document.rs and element_id.rs still hold
@account n=5 entity=standard-contracts#28.2 v=not_touched
why: the report says poll, initial_build and flush_queued_mounted_events are untouched and no borrow is held across render_immediate
@account n=6 entity=standard-contracts#28.3 v=proposed
why: handle_event now also writes the reached record beside the registry's dispatch where a ReachedRecord stands (F4)
@account n=7 entity=standard-contracts#29.1 v=proposed
why: its list of exact listener readers leaves out the new ungated crate-private element_listener_ids, which the reached record reads
@account n=8 entity=standard-contracts#30.1 v=not_touched
why: is_masked_input and the snapshot reader are not among the report's Changes, and snapshot.rs is not a reached file
@account n=9 entity=standard-contracts#30.2 v=not_touched
why: the report leaves the select arm of dioxus_document.rs unchanged; the file only gained line 10 and the handle_event lines 448-460
@account n=10 entity=standard-contracts#30.5 v=not_touched
why: multiple_select_values is not among the report's Changes; dioxus_document.rs gained only line 10 and lines 448-460
@account n=11 entity=standard-contracts#30.6 v=not_touched
why: the report states flush_queued_mounted_events is untouched
@account n=12 entity=standard-contracts#30.8 v=not_touched
why: the report leaves actionable_nodes and actionable.rs unchanged
@account n=13 entity=standard-contracts#30.10 v=proposed
why: the reached record is a second root context like the registry's, provided at ScopeId::ROOT with a Debug that prints a count alone (F3)
@account n=14 entity=standard-contracts#31.1 v=proposed
why: escher-session gains a third role, coverage, beside serve and mcp (F1)
