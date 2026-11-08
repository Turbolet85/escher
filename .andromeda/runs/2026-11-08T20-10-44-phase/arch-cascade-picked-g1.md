@account n=1 entity=design-philosophy#1.1 v=not_in_scope
why: the chunk adds no crate, moves no crate boundary and paints nothing; its record stays inside the existing Dioxus wrapper, a crate-modularity re-check every change owes
@account n=2 entity=design-philosophy#2.1 v=carried
why: its DioxusDocument line states the event path (handle_ui_event, poll) in dioxus_document.rs where the chunk records which (element, event) handlers a run reached
@account n=3 entity=design-philosophy#4.1 v=carried
why: a dropped node's id stops resolving and every public API takes NodeId, so a record spanning a run's screens must key on the graph's stable id, never a NodeId
@account n=4 entity=design-philosophy#5.1 v=not_in_scope standing=S2
why: integration tests living in blitz-tests' tests/ directory, one target per file, is the test-home convention
@account n=5 entity=stack-and-technologies#1.1 v=not_in_scope
why: no manifest, edition or MSRV moves; compiling at the workspace's edition and MSRV is a re-check every change owes
@account n=6 entity=stack-and-technologies#1.9 v=not_in_scope
why: windowing, keyboard-types and winit are not reached by a read-only coverage command or a replay test
@account n=7 entity=stack-and-technologies#1.13 v=carried
why: the exercised record is written against the Dioxus 0.7.3 vdom's event dispatch in dioxus-native-dom, with no Dioxus version moved
@account n=8 entity=stack-and-technologies#1.14 v=carried
why: serde_json is escher-driver's reader of kept checks and of the live answer line a replay compares, which the owed replay test reads key by key
@account n=9 entity=stack-and-technologies#1.20 v=carried
why: the chunk's stand tests reach the headless stand and the driver session through blitz-tests' existing seven_guis and escher-driver dev-dependencies
@account n=10 entity=stack-and-technologies#1.22 v=carried
why: the chunk may edit test_capability_statement.py, a CI Python helper held to stdlib with no package added
@account n=11 entity=stack-and-technologies#2.1 v=not_in_scope
why: the dev-host cognitive-complexity ceiling is a re-check every new function owes, not this chunk's obligation
