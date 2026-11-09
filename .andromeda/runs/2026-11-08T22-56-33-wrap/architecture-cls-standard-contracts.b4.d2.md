@answer n=1 entity=standard-contracts#27.1 v=no
why: the launch surface lives in dioxus-native, which the session does not reach, and its glob re-export of dioxus_native_dom still names the same items
@answer n=2 entity=standard-contracts#28.1 v=yes mode=add-to
why: F3 adds ReachedHandler, record_reached and reached_handlers to DioxusDocument's public readers, and dioxus_document.rs, element_id.rs and lib.rs are reached whole so their cited lines can move
@answer n=3 entity=standard-contracts#28.2 v=yes mode=stale
why: dioxus_document.rs is reached whole and this entity cites its constructor and poll lines, which the change can move
@answer n=4 entity=standard-contracts#28.3 v=yes mode=add-to
why: F4 places the reached record in handle_event after the early returns and before runtime.handle_event, beside the registry dispatch
@answer n=5 entity=standard-contracts#28.4 v=no
why: the Cells keyboard lives in cells.rs, which the session does not reach
@answer n=6 entity=standard-contracts#29.1 v=yes mode=add-to
why: F4 adds the ungated element_listener_ids beside the crate-private element_listeners reading in element_id.rs, which is reached
@answer n=7 entity=standard-contracts#29.2 v=no
why: the Readings actionable rule lives in actionable.rs, which the session does not reach, and the new listener reader does not change it
@answer n=8 entity=standard-contracts#29.3 v=no
why: the session adds no listener and changes no clickable reading, so the CLICKABLE table counts stand
@answer n=9 entity=standard-contracts#29.4 v=no
why: dioxus-core listener diffing and mutation_buffer.rs are not reached by the session
@answer n=10 entity=standard-contracts#30.1 v=no
why: is_masked_input lives in snapshot.rs, which the session does not reach
@answer n=11 entity=standard-contracts#30.2 v=yes mode=stale
why: dioxus_document.rs is reached whole and this entity cites its Input and Change lines, which the change can move
@answer n=12 entity=standard-contracts#30.3 v=no
why: no node key moved, so NODE_FIELDS keeps its fourteen keys and the snapshot still holds no list field
@answer n=13 entity=standard-contracts#30.4 v=no
why: the multiple-select bounds and value readings live in snapshot.rs, which the session does not reach
@answer n=14 entity=standard-contracts#30.5 v=yes mode=stale
why: dioxus_document.rs is reached whole and this entity cites the multiple_select_values lines, which the change can move
@answer n=15 entity=standard-contracts#30.6 v=yes mode=stale
why: dioxus_document.rs is reached whole and this entity cites the flush_queued_mounted_events lines, which the change can move
@answer n=16 entity=standard-contracts#30.7 v=no
why: the UI graph contract is unchanged, and the coverage reader only reads DioxusDocument::graph from graph.rs, which the session does not reach
@answer n=17 entity=standard-contracts#30.8 v=yes mode=stale
why: lib.rs is reached whole, this entity cites its crate-root re-export line, and the session moves the root re-export count from 35 to 36
@answer n=18 entity=standard-contracts#30.9 v=no
why: the graph readings live in graph.rs, which the session does not reach
@answer n=19 entity=standard-contracts#30.10 v=no
why: the SignalRegistry and use_graph_state contracts are unchanged and signal_registry.rs is not reached
@answer n=20 entity=standard-contracts#31.1 v=yes mode=add-to
why: F1 gives escher-session a coverage role with its own usage clause and the same exit statuses, and session_host.rs, which this entity cites, is reached
@answer n=21 entity=standard-contracts#31.2 v=no
why: multi_doc is a root example the session does not reach
@answer n=22 entity=standard-contracts#32.1 v=no
why: the browser app is not reached by the session
@answer n=23 entity=standard-contracts#33.1 v=no
why: the WPT runner outputs are not reached by the session
@answer n=24 entity=standard-contracts#34.1 v=no
why: the new stand target is picked up by its prefix alone, and the CI legs, named targets and capability-statement counts stay unchanged
@answer n=25 entity=standard-contracts#34.2 v=no
why: the embolden leg and its witness are not reached by the session
