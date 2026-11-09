@account n=1 entity=input-validation#1.72 v=not_touched
why: element_id.rs gains only the crate-private element_listener_ids over placed ids (report Symbols), so no walk spells the virtual form and the claim holds
@account n=2 entity=input-validation#1.73 v=not_touched
why: the coverage line carries ids, owner ids, event names and counts, no accessible name, so the name crossing and host_log readings stand as written
@account n=3 entity=input-validation#1.77 v=not_touched
why: json.rs gains only Coverage::to_json and its test, leaving the state booleans and their carriers as the claim states
@account n=4 entity=input-validation#1.85 v=not_touched
why: dioxus_document.rs gains only the reached-record write in handle_event, the select's input and change dispatch unchanged per the report
@account n=5 entity=input-validation#1.92 v=not_touched
why: multiple_select_values and the set's exits are untouched by the reached-record lines the report lists in dioxus_document.rs
@account n=6 entity=input-validation#1.94 v=not_touched
why: dioxus-native-dom lib.rs gains only the reached module and ReachedHandler re-export, the select readers' re-exports standing
@account n=7 entity=input-validation#1.114 v=not_touched
why: the data-dioxus-id usize parse is not among the dioxus_document.rs additions and still yields no id on failure
@account n=8 entity=input-validation#1.115 v=proposed
why: handle_event now also writes the root-context ReachedRecord before runtime.handle_event, which F2 states beside the registry dispatch
