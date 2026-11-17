# obs · standing conventions

- S1 · each new check or test is silent, printing nothing on success and sharing modules rather than echoing content · 3-observability-harness-contract
- S2 · each new test that asserts an empty stderr unsets RUST_LOG for the process it runs · 3-observability-harness-contract
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · each new log-reading check that fails names a kind and a count, never the content it read · 3-observability-harness-contract
- S4 · each new fixture or committed check holds no secrets and no private data · 9-ci-integration · ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
