# Classified — architecture · standard-contracts.b5

## Answers
standard-contracts#27.1 · yes · add-to · F22: flush_queued_mounted_events skips an element removed since its event was queued, and try_element_to_node_id gains that guard as a reader (line 266); dioxus_document.rs is a reached merge surface, so its line citations move too
standard-contracts#27.2 · no · the Cells keyboard and cells-grid live in examples/seven_guis/src/tasks/cells.rs, which is not reached, and no hunk touches the stand's Cells
standard-contracts#28.1 · no · is_masked_input lives in packages/dioxus-native-dom/src/snapshot.rs, not reached, and no reading or mask changed
standard-contracts#28.2 · yes · stale · dioxus_document.rs is a merge surface reached whole, so the citation dioxus_document.rs:397-402 for the Input and Change route moves
standard-contracts#28.3 · no · the snapshot's node keys and NodeState live in snapshot.rs, not reached, and the session adds no field
standard-contracts#28.4 · no · multiple-select rows' client-rect bounds are read in snapshot.rs, not reached, and no hunk moves that reader
standard-contracts#28.5 · yes · stale · dioxus_document.rs is a merge surface reached whole, so the citation dioxus_document.rs:31-45 for multiple_select_values moves
standard-contracts#29.1 · no · the escher-session CLI and the example and WPT binaries are not reached; the driver and examples hold no diff
standard-contracts#29.2 · no · examples/multi_doc.rs is not reached and no env read was added
standard-contracts#30.1 · no · nothing under apps/ is reached
standard-contracts#31.1 · no · wpt/runner/src is not reached, and only the quarantine list moved, which this entity does not name
