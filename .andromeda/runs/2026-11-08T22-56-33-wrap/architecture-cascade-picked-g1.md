@account n=1 entity=design-philosophy#1.1 v=not_touched
why: dioxus-native-dom's lib.rs gained lines 28 and 45 only, so its crate description and the modular-crates claim still hold per the report's Changes
@account n=2 entity=design-philosophy#2.1 v=not_touched
why: DioxusDocument still integrates BaseDocument with VirtualDom through handle_ui_event and poll, which the report states untouched; only a citation moved in dioxus_document.rs
@account n=3 entity=established-decisions#11.10 v=not_touched
why: the select list's limits are unchanged; capabilities.md gained the coverage limits elsewhere, a citation shift only
@account n=4 entity=established-decisions#14.3 v=not_touched
why: the IME-on-construct rule and its test stand unchanged; dioxus_document.rs gained the handle_event record only
@account n=5 entity=established-decisions#15.1 v=not_touched
why: disabled, removal and mapping semantics are unmoved; the report's dioxus_document.rs change is the reached record in handle_event alone
@account n=6 entity=established-decisions#24.1 v=not_touched
why: base_url, the ua sheet, hot reload, history and eval are unmoved by the reached record
@account n=7 entity=established-decisions#25.1 v=not_touched
why: the report states initial_build and poll untouched and the record taking no borrow across render_immediate, so the buffered render and its merge surface hold
@account n=8 entity=established-decisions#25.2 v=not_touched
why: the buffered render's same-render limit is unmoved; the record writes no node
@account n=9 entity=established-decisions#25.3 v=not_touched
why: the dioxus-core task-poll order is a dependency fact and Cargo.lock is byte-identical
@account n=10 entity=established-decisions#27.1 v=proposed
why: handle_event's event path now also writes the reached record where a ReachedRecord context holds, every document paying one more has_context lookup
@account n=11 entity=established-decisions#31.1 v=proposed
why: the command line gains the coverage role taking kept-check file paths on argv, a ratified boundary widening (P4 Q1)
@account n=12 entity=established-decisions#31.4 v=proposed
why: F5's screens-spanned decision (P4 Q2, PROVISIONAL) belongs beside the driver session's newest item
@account n=13 entity=established-decisions#32.1 v=proposed
why: a kept check is now also read by the coverage role from a caller's path and replayed by handler_coverage through Check::replay_each
@account n=14 entity=conventions#6.1 v=proposed
why: dioxus-native-dom's new reached module and element_listener_ids are ungated beside the accessibility-gated graph and element_facts
@account n=15 entity=conventions#7.1 v=not_touched
why: the record's DocGuard is a temporary dropped before runtime.handle_event and the buffered render is untouched, so the borrowing convention holds
@account n=16 entity=conventions#8.1 v=proposed
why: F7, the new session check stand_handler_coverage reading common's HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells(), joins the shared-module list
@account n=17 entity=conventions#9.1 v=not_touched
why: cli::tests still sits in cli_tests.rs, which gained a test; cli.rs's moved lines are a citation shift only
