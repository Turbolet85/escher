# Classified — security · error-handling.b1

## Answers
error-handling#1.1 · no · the escher-driver Refusal type and its wire and log forms; the chunk edits no escher-driver code and the scope lists refusals as not needed
error-handling#2.1 · yes · the Stylo hooks' todo!, unimplemented! and panic! sit in stylo.rs, a conflict surface, so a resolution there must add no panic path a page can reach
error-handling#3.1 · no · startup unwraps and expects (event loop, window, JS runtime, tokio, Android) are not page-reached and the scope names only winit's clipboard path, so the chunk's code does not meet them
error-handling#4.1 · yes · the PARALLEL_TRAVERSAL_LOCK poison recovery and the serialised Parallel traversal sit in stylo.rs, a conflict surface, and a resolution must keep them panic-free
error-handling#5.1 · yes · the select list's readers open_select, select_list_bounds, select_list_view, select_row_layout and close_select_list must add no layout_style() panic, and the take must not bring one into them; the select list is named for re-read
error-handling#6.1 · yes · the RefCell panic on MountedData calls from a spawned task sits on the mounted-events path the take edits, and Cells' synchronous handle calls in cells.rs, a conflict surface, must stay
error-handling#7.1 · yes · the key-press panic at position_in_containing_block in node.rs, a conflict surface and the panic the scope names at node.rs:1775, on which select_popup's skipped row waits
error-handling#8.1 · yes · the select list's close on removal, disable, hide or inert with the list open, and its tests in select_popup.rs, which the chunk edits, must keep closing with no panic
