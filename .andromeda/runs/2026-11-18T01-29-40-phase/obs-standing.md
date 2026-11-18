# obs · standing conventions

- S1 · each new test is silent, printing nothing beyond the harness's lines, and its shared code lives as shared modules · 3-observability-harness-contract
- S2 · each new test joins a CI leg whose per-leg log is uploaded on failure · ci-integration
- S3 · each new committed fixture or check holds no secrets and no private data · 9-ci-integration · ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
- S4 · each new test that reads a failure from logs names a kind and a count, never the logged content · 3-observability-harness-contract
  decision/agent-run-no-captured-output · Agent-run events hold no captured output — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
