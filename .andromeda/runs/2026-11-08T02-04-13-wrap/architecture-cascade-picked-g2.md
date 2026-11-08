@account n=1 entity=standard-contracts#1.1 v=not_touched
why: blitz-dom is unchanged (the manifest and blitz-dom diff gate green) and DioxusDocument still implements the Document trait as stated, the reached dioxus_document.rs gaining only the bridge dispatch and a test-only dispatch_to_vdom
@account n=2 entity=standard-contracts#2.6 v=not_touched
why: select_of, closed_select_of and is_select read as stated on the reached snapshot.rs and lib.rs; the graph reading select_of for a node's owner is the bridge's graph fact (F2), not this row's
@account n=3 entity=standard-contracts#17.1 v=proposed
why: the supporting seven_guis API gains tasks::use_graph_signal and the const fn tasks::writes in tasks/mod.rs (F4)
@account n=4 entity=standard-contracts#21.1 v=proposed
why: the driver gains the graph verb, Command::Graph, Outcome::Graphed, two FieldKinds and the clickable node key, so its verb, node-key, field-kind, describe and MCP-listing counts and its JSON, span and record-of-ids statements move
