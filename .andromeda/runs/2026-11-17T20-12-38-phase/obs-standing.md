# obs · standing conventions

- S1 · every new test or fixture check is silent — it prints nothing of the page, the answer or the captured stream on success · 3-observability-harness-contract
  decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
- S2 · every new log or stderr check's failure message names a kind and a count, never the content it found · 3-observability-harness-contract
- S3 · every new committed fixture or kept check holds no secrets or private data · 9-ci-integration · ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
- S4 · every new check that spawns a sink-installing binary and expects an empty stderr unsets RUST_LOG unless it reads the sink at trace · 3-observability-harness-contract
