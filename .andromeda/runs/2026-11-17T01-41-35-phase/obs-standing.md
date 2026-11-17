# obs · standing conventions

- S1 · every new check is silent on success and shares its helpers through the shared modules · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
- S2 · every check that asserts an empty stderr unsets RUST_LOG for its child · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · every log check that fails names a kind and a count, never the content it read · 3-observability-harness-contract
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S4 · every committed kept check or fixture holds no secret, page content or local state · 9-ci-integration · ci-integration
