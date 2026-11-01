# Classified — arch · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · the Dioxus bridge's stable ids, snapshot, unkeyed_actionable and accessibility_tree override live in dioxus_document.rs, a predicted conflict surface the Needs require a resolution there to keep
standard-contracts#27.2 · yes · Cells' keyboard on cells-grid lives in examples/seven_guis/src/tasks/cells.rs, a predicted conflict surface the resolution keeps
standard-contracts#28.1 · yes · the password and file masking that is_masked_input decides is the masking the scope names through paint changes, carried by the snapshot value reader
standard-contracts#28.2 · yes · Input and Change routing in dioxus_document.rs (a conflict surface), and select_value, closed_select_of and select_list_bounds, the select model the scope keeps unchanged
standard-contracts#28.3 · no · the snapshot key set and NodeState fields in snapshot.rs are not reached and the take adds no snapshot field, so this is a re-check every change owes
standard-contracts#28.4 · yes · the bounds and value of a multiple select's rows and groups read the multiple select the scope names in the engine-debts merge surface and the stylo.rs height seam
standard-contracts#28.5 · yes · multiple_select_values in dioxus_document.rs, a predicted conflict surface, fills evt.values() on Input and Change
standard-contracts#29.1 · no · the CLI lines of examples and apps, escher-session included, are not reached, the driver surface is unchanged by the chunk, and the scope names no change to examples or apps
standard-contracts#29.2 · no · examples/multi_doc.rs is not reached, and its Instant::now() sits outside blitz-dom, so the clock invariant does not reach it
standard-contracts#30.1 · no · the browser app's history store, about pages and loader are not reached, and the scope names no change in apps/
standard-contracts#31.1 · no · the WPT runner and its outputs are out of scope, and its workflows run upstream only
