# obs · standing conventions

- S1 · a check added to cover a click, executor or graph arm before it is moved runs silently, with no sink installed in a headless stand, and unsets RUST_LOG where it asserts empty stderr · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
  decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
- S2 · a check added for a moved arm that reads a span or a log line asserts only on the closed span's line and its fixed fields, never on content · log-coverage · 6-log-coverage
