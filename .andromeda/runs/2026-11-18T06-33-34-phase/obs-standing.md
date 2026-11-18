# obs · standing conventions

- S1 · each harness JSON line a new test or the pipe writes is encoded by python's json module · 3-observability-harness-contract
  contract/agent-run-events/events-jsonl · agent-run harness events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim.
  decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
- S2 · each new log check's failure names a kind and a count, never the content it read · 3-observability-harness-contract
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S3 · each new check is silent on success and shares the harness's modules · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
- S4 · a new check that expects empty stderr unsets RUST_LOG · 3-observability-harness-contract
  contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
  contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S5 · each new verdict field is a closed word or a count holding no content · log-coverage · 6-log-coverage
- S6 · each committed check, fixture or transcript copy holds no secret or private data · ci-integration · 9-ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
