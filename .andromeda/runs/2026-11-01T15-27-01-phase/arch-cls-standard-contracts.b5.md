# Classified — arch · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · the bridge's data-dioxus-id dispatch, stable-id tiers, snapshot, accessibility_tree override and unkeyed_actionable live in dioxus_document.rs, a predicted conflict surface the resolution keeps
standard-contracts#27.2 · yes · the cells-grid keyboard and focus rules sit in examples/seven_guis/src/tasks/cells.rs, a predicted conflict surface
standard-contracts#28.1 · yes · the scope names a password input staying masked through paint changes, and is_masked_input is the test the snapshot value reader calls that carries it
standard-contracts#28.2 · yes · Input and Change reach the app as NativeFormData at dioxus_document.rs:393-398, a predicted conflict surface, so the onchange rule lives in a conflict file
standard-contracts#28.3 · no · the absence of an expanded key and the NODE_FIELDS list sit in snapshot.rs and the driver surface, neither reached nor named by the scope
standard-contracts#28.4 · no · the multiple-select row and group bounds sit in snapshot.rs, unreached; the scope names the multiple select only as an engine merge row and the size seam
standard-contracts#28.5 · yes · multiple_select_values in dioxus_document.rs:31-45 is in a predicted conflict surface
standard-contracts#29.1 · no · the example and app command lines and the escher-session surface are not needed; the chunk changes none of them
standard-contracts#29.2 · no · multi_doc is an example outside the reached files; its command lines and Instant::now() are not in the scope
standard-contracts#30.1 · no · the browser app and rdme history, about pages and suggester are not needed; the scope names no change in apps/
standard-contracts#31.1 · no · the WPT runner's execution, outputs and report formats are not needed; the scope names no WPT change
