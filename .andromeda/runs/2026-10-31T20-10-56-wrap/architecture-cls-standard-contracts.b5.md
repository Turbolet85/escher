# Classified — architecture · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · contradict · F18: the snapshot builder reads select_list_bounds before the client rect for a row, so `bounds` is no longer get_client_bounding_rect for every element (F19 adds no NodeState field)
standard-contracts#27.2 · no · the cells grid and examples/seven_guis/src/tasks/cells.rs are not reached; that source was not edited
standard-contracts#28.1 · yes · add-to · F18: is_select joins as a public re-exported predicate beside is_masked_input and MASKED_VALUE
standard-contracts#28.2 · yes · stale · closed_select_of now answers None for a select whose list is open (Touches :279, F18 row-bounds read), while the closed-select claim itself holds
standard-contracts#28.3 · yes · stale · F18 changes the snapshot row-bounds read this cites (snapshot.rs:245-260); the list is single-select only, but the report does not show the multiple select's rows unmoved
standard-contracts#28.4 · no · dioxus_document.rs is not reached, and multiple_select_values fills values only for a multiple select, which opens no list
standard-contracts#29.1 · no · the CLI contract was re-read and unmoved: twelve verbs, the six served tasks, exit statuses and boot unchanged, no verb or entry point added
standard-contracts#29.2 · no · examples/multi_doc.rs is not reached and no line it cites moved
standard-contracts#30.1 · no · the browser app is not reached and was not edited
standard-contracts#31.1 · no · the WPT runner is not reached and was not edited
