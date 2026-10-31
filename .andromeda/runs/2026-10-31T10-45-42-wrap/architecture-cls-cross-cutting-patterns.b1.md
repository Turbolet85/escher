# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · document.rs is reached and its edits re-point the wasm32 citation at :436-439
cross-cutting-patterns#2.1 · yes · stale · execute.rs is reached and its edits re-point the command-span citation at :97-112
cross-cutting-patterns#3.1 · yes · add-to · DocumentMutator::flush calls clear_focus on an inert focused node (F3), so a redraw on a focus cleared from the mutation side now has a caller
cross-cutting-patterns#4.1 · no · its IME path sits in layout/construct.rs, which the session does not reach, and its redraw-route pointer to the Paint and shell's contract stands
cross-cutting-patterns#5.1 · yes · stale · dioxus_document.rs gains multiple_select_values and runtime/events.rs drops the change synthesis, so the cited lines at dioxus_document.rs :160-168, :232-252, :342-348 and runtime/events.rs :171-173 re-point
cross-cutting-patterns#6.1 · yes · add-to · record_select_of_multiple_write (F9) and maybe_record_node's optgroup arm (F12) are new records the changed set's four kinds of site must be read against, and the select statement stands
