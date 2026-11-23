# obs · standing conventions

- S1 · each new check and shared test module is silent, printing nothing on a pass, and a check that reads stderr as empty unsets RUST_LOG · 3-observability-harness-contract
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
  contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S2 · each committed check artifact, such as a digest table row, holds no private data or secret · ci-integration · 9-ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
- S3 · each new engine log line writes a fixed message with safe fields, compiled in only behind the engine's tracing feature · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
