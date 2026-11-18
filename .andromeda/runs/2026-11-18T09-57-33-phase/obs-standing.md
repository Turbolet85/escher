# obs · standing conventions

- S1 · each new test check is silent on success and shares its helpers through the common modules · 3-observability-harness-contract
- S2 · each new check that asserts an empty stderr unsets RUST_LOG for the child it runs · 3-observability-harness-contract
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S3 · each new log or output check that fails names a kind and a count, never the content it read · 3-observability-harness-contract
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S4 · each new JSON line the harness writes is encoded by python's json module · 3-observability-harness-contract
  decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
- S5 · each new committed kept check or fixture holds no secret and no private data · 9-ci-integration · ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
