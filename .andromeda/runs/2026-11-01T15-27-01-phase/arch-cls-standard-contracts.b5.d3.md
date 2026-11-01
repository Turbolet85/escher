# Classified — arch · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · the Dioxus bridge in dioxus_document.rs, a conflict surface: stable element ids in their four tiers, the accessibility_tree override, data-dioxus-id dispatch and the snapshot's bounds reads, which the resolution keeps and the stand_element_ids, stand_id_persistence and stand_id_edits checks re-read
standard-contracts#27.2 · yes · cells.rs is a conflict surface, and the cells-grid keyboard contract (tabindex, onkeydown, scroll_to, the focus outline) lives in that file
standard-contracts#28.1 · yes · is_masked_input is the predicate the snapshot value reader calls to mask password and file inputs, the password mask the scope keeps through paint changes and the stand checks re-assert in both layout modes
standard-contracts#28.2 · yes · the Input and Change routing lives in dioxus_document.rs, a conflict surface; the select value, closed_select_of and select_list_bounds reads sit on the select model and open list the scope keeps unchanged
standard-contracts#28.3 · no · the absence of an expanded key and the thirteen NODE_FIELDS keys are driver wire and snapshot shape that the take does not reach and the scope excludes as driver surface
standard-contracts#28.4 · yes · a multiple select is a row of the engine-debts merge surface re-read against the take, and its rows' bounds read get_client_bounding_rect, which the scope names as unchanged
standard-contracts#28.5 · yes · multiple_select_values lives in dioxus_document.rs, a conflict surface, and fills the values of Input and Change events
standard-contracts#29.1 · no · the example and app command lines and escher-session's verbs are outside the scope, which names no change to the examples and apps and changes none of the driver session
standard-contracts#29.2 · no · the multi_doc example's modes, env vars and Instant::now() timing sit outside blitz-dom's clock invariant, and the take adds no env read or CLI surface there
standard-contracts#30.1 · no · the browser app and rdme are Not needed: the scope names no change in apps/
standard-contracts#31.1 · no · the WPT runner's outputs, buckets and providers are out of scope and Not needed by the scope
