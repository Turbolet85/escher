# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · the wasm font-context citations at document.rs:439-442 sit past the session's insertions in document.rs (343-344 moved), and no feature, manifest or config moved
cross-cutting-patterns#2.1 · yes · stale · the debug_timer! citation at resolve.rs:172 sits past the new reconcile_select_list step at resolve.rs:133-135, and execute.rs:97-112 is an edited driver file; no span or log field was added
cross-cutting-patterns#3.1 · yes · add-to · opening, moving, scrolling and closing a list each ask a redraw (indirectly through opening_moving_scrolling_and_closing_each_ask_a_redraw), joining the list of what requests a redraw
cross-cutting-patterns#4.1 · no · the text-input editor and IME route in layout/construct.rs is not reached, and the session's select focus changes do not touch a text input's editor
cross-cutting-patterns#5.1 · no · the default-action queue and handler order in events/driver.rs and events/mod.rs are not reached, and select Enter and Space ride the same queue unchanged
cross-cutting-patterns#6.1 · yes · contradict · the changed-set sentence says scroll writes nothing, false where a list stands (F26: open, close and a page scroll closing a list mark the select), and the list is new interaction state cleared in clear_interaction_state_for_removed_node
