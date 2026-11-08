@account n=1 entity=conventions#4.1 v=not_in_scope
why: TODO/FIXME and DO-NOT-REORDER markers sit in files the chunk does not edit (mutation_writer.rs and engine crates), a re-check every change owes
@account n=2 entity=conventions#6.1 v=carried
why: the graph module is gated by accessibility and the signal registry is not, which decides where the new exercised record, which must hold registry or not, can live; escher-driver's hand-built ungated spans govern any new verb in execute.rs and mcp.rs
@account n=3 entity=conventions#7.1 v=carried
why: the bridge renders the vdom into a MutationBuffer with the document's RefCell unborrowed, and the new handler record is written in dioxus_document.rs's event path, so it must keep that borrow discipline
@account n=4 entity=conventions#8.1 v=carried
why: the conventions list of Session-holding stand checks that share session_common must record the new handler-coverage stand check the chunk adds
@account n=5 entity=conventions#9.1 v=carried
why: the owed replay test lands in recording_tests.rs, cfg(all(test, unix)), so which OS jobs witness v030-06's verified state is this rule's
@account n=6 entity=conventions#9.3 v=carried
why: a new verb adds a row to the MCP listing test and the scope reaches stand_requirement_sweep.rs, whose rows are asserted through track_caller helpers naming row and index only
@account n=7 entity=conventions#11.1 v=carried
why: the stand's tasks register every signal through use_graph_signal, so a stand-only test cannot witness the exercised record holding on an app with no registry, which the scope requires
