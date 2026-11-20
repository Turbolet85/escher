# obs · standing conventions

- S1 · every check and shared test module is silent — it prints nothing to stdout or stderr and installs no subscriber · 3-observability-harness-contract
- S2 · every check's failure message names a kind and a count and echoes nothing the caller typed or the answer held · 3-observability-harness-contract
  decision/runtime-probe-leaks-no-read-value · Runtime probe leaks no read value — The dioxus runtime probe lets no read value leave it: its failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id.
- S3 · committed checks and their helpers hold no private data or secret · ci-integration · 9-ci-integration
  decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
  decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
