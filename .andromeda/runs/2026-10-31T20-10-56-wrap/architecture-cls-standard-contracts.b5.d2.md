# Classified — architecture · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · stale · the snapshot's `bounds` reads `select_list_bounds` before the client rect for a row of an open list (F18), so the "for every element" reading moved
standard-contracts#27.2 · no · the Cells grid keyboard lives in examples/seven_guis/src/tasks/cells.rs, which the session did not edit
standard-contracts#28.1 · yes · add-to · F18: dioxus-native-dom's public re-exported `is_select` joins `is_masked_input` beside `MASKED_VALUE`
standard-contracts#28.2 · yes · stale · `closed_select_of` answers `None` for a select whose list is open and an open list's option reads its row box (F18), so the cited reader's meaning moved
standard-contracts#28.3 · yes · stale · the snapshot builder now reads `select_list_bounds` before the client rect for a row (F18), moving the row-bounds code cited at snapshot.rs:245-260
standard-contracts#28.4 · no · `multiple_select_values` lives in dioxus_document.rs, which is not a reached file, and the session changes no multiple select's values
standard-contracts#29.1 · no · the twelve verbs, the six served tasks and the exit statuses are unchanged; contract-clis is re-read and unmoved
standard-contracts#29.2 · no · `multi_doc` is an example the session did not touch
standard-contracts#30.1 · no · the browser app was not edited and no reached file holds its claims
standard-contracts#31.1 · no · the WPT runner was not edited and no reached file holds its claims
