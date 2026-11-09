@answer n=1 entity=standard-contracts#27.1 v=no
why: its launch and re-export claims cite dioxus-native's own files, none reached, and the new dioxus-native-dom re-export passes through the glob without contradicting them
@answer n=2 entity=standard-contracts#28.1 v=yes mode=stale
why: its line citations into reached dioxus_document.rs, element_id.rs and lib.rs can have moved with the new reached record and the ReachedHandler re-export
@answer n=3 entity=standard-contracts#28.2 v=yes mode=stale
why: its citations into reached dioxus_document.rs (new, the constructor and initial_build and poll lines) can have moved with that changed file
@answer n=4 entity=standard-contracts#28.3 v=yes mode=add-to
why: the dispatch carries F4, the reached record written after the event-kind and dioxus-id early returns and before runtime.handle_event, with one has_context lookup on every other document
@answer n=5 entity=standard-contracts#28.4 v=no
why: the Cells keyboard contract cites examples/seven_guis/src/tasks/cells.rs, which the session does not reach, and grid_cells() is test-side only
@answer n=6 entity=standard-contracts#29.1 v=yes mode=stale
why: its citation element_id.rs:253-328 sits in a reached file whose lines can have moved with the new ungated element_listener_ids
@answer n=7 entity=standard-contracts#29.2 v=no
why: the actionable rule lives in actionable.rs, which the session does not reach, and it reads element_facts unchanged
@answer n=8 entity=standard-contracts#29.3 v=no
why: the clickable counts come from stand_non_lean_tasks and the UI graph, neither reached, and the session moves no clickable reading
@answer n=9 entity=standard-contracts#29.4 v=no
why: the dioxus-core listener path lives in mutation_buffer.rs and dioxus-core, neither reached
@answer n=10 entity=standard-contracts#30.1 v=yes mode=stale
why: its claim that is_masked_input is re-exported beside MASKED_VALUE describes reached lib.rs, whose re-export list gains ReachedHandler
@answer n=11 entity=standard-contracts#30.2 v=yes mode=stale
why: its citation dioxus_document.rs:421-426 sits in a reached file whose lines can have moved
@answer n=12 entity=standard-contracts#30.3 v=no
why: the snapshot keeps its fourteen node keys and NODE_FIELDS, the session moving no schema and schema.rs being unreached
@answer n=13 entity=standard-contracts#30.4 v=no
why: the multiple-select bounds and value readings live in snapshot.rs, which the session does not reach
@answer n=14 entity=standard-contracts#30.5 v=yes mode=stale
why: its citation dioxus_document.rs:34-48 sits in a reached file whose lines can have moved
@answer n=15 entity=standard-contracts#30.6 v=yes mode=stale
why: its citation dioxus_document.rs:256-260 and its left-out upstream test sit in reached files that can have moved
@answer n=16 entity=standard-contracts#30.7 v=no
why: the UI graph contract is unchanged, graph.rs is unreached, and the coverage reader only reads DioxusDocument::graph
@answer n=17 entity=standard-contracts#30.8 v=yes mode=stale
why: its citation lib.rs:38 for the actionable_nodes re-export sits in reached lib.rs, whose lines can have moved with ReachedHandler
@answer n=18 entity=standard-contracts#30.9 v=no
why: the graph naming and writer rules live in graph.rs, which the session does not reach or change
@answer n=19 entity=standard-contracts#30.10 v=yes mode=add-to
why: the registry's counts-only Debug and root context gain F3, the ungated reached module's ReachedRecord at ScopeId::ROOT, whose Debug prints its count alone
@answer n=20 entity=standard-contracts#31.1 v=yes mode=add-to
why: the escher-session role set gains F1, the coverage role with its own shape, malformed status 1 and failing-boot status 3
@answer n=21 entity=standard-contracts#31.2 v=no
why: the multi_doc root example is outside the session's reach
@answer n=22 entity=standard-contracts#32.1 v=no
why: the browser app is not reached by the session
@answer n=23 entity=standard-contracts#33.1 v=no
why: the WPT runner outputs are not reached by the session
@answer n=24 entity=standard-contracts#34.1 v=no
why: the CI leg lists and scripts are unchanged and the new stand target is picked up by its prefix alone
@answer n=25 entity=standard-contracts#34.2 v=no
why: the embolden leg is unchanged and no reached file is cited
