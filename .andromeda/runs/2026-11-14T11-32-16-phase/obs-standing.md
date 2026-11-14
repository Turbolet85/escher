# obs · standing conventions

- S1 · each new check runs silent and shares its helpers through the shared modules, printing nothing on success · 3-observability-harness-contract
- S2 · a check that asserts empty stderr unsets RUST_LOG for the child it runs · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · a check failure names a kind and a count, never an answer, a snapshot text or a typed value · 3-observability-harness-contract
  decision/runtime-probe-leaks-no-read-value · Runtime probe leaks no read value — The dioxus runtime probe lets no read value leave it: its failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id.
- S4 · driver span fields carry no call content; only closed-value fields print · 4-span-trace-coverage · span-trace-coverage
