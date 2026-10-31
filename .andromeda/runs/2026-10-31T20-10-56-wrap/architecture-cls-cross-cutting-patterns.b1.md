# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · the wasm32 font-discovery citations at document.rs:439-442 and lib.rs:137-142 sit in reached files whose lines moved past the new insertions (document.rs at 343-344, lib.rs re-exports); the config claim itself holds, with no feature, manifest or DocumentConfig change
cross-cutting-patterns#2.1 · yes · stale · the debug_timer! citation resolve.rs:172 lies past the reconcile_select_list step inserted at resolve.rs:133-135, so its line moved; the span and sink claims hold (no span, log field or sink added or moved, escher-telemetry unedited)
cross-cutting-patterns#3.1 · yes · add-to · opening, moving, scrolling and closing a list each ask a redraw, indirectly through opening_moving_scrolling_and_closing_each_ask_a_redraw, so the list joins the redraw-request list; the pointer.rs and document.rs citations in it moved too
cross-cutting-patterns#4.1 · no · the IME and text-editor construction at construct.rs:860-872 is not in the reached files, and the session touches no text input, editor or set_ime_enabled path
cross-cutting-patterns#5.1 · no · the event queue, default-action ordering and dispatch sit in events/driver.rs, events/mod.rs, dioxus_document.rs and mutation_writer.rs, none reached, and the session changes no queue or ordering (the list's choice reuses input then change)
cross-cutting-patterns#6.1 · yes · contradict · the changed set is no longer unwritten by scroll or layout where a list stands (open and close are a fifth kind of site, and a page scroll or move marks through reconcile_select_list at resolve); F26 belongs here; the list is new interaction state cleared in clear_interaction_state_for_removed_node
