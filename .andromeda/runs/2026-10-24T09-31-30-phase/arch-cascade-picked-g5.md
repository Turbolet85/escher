# arch · picked accounting · group 5 of 7

## Picked
standard-contracts#21.1 · carried · `286874cc1` syncs `dioxus_document.rs`, `events.rs`, `lib.rs` and `mutation_writer.rs` and adds `stale_node_mapping.rs`, so the bridge's event routing, boolean-attribute clearing, stable ids, node drop, accessibility override, snapshot, diff and actionable check must survive the take
standard-contracts#22.1 · carried · the synced `lib.rs` must keep `is_masked_input` re-exported beside `MASKED_VALUE`, the test the snapshot's mask and the recorder read
standard-contracts#22.2 · carried · the select's `Input | Change` conversion sits in `dioxus_document.rs`, replaced on upstream's side by `286874cc1`, inside CARRY 5's select surface
standard-contracts#26.1 · carried · the take's gates are these legs: fast's clippy, msrv, doc, audit (the tokio advisory), a11y, coverage with its `keyboard_compat` exclusion and floors, stand-repeat, and the blitz-tests target contract that upstream's new test files enter
