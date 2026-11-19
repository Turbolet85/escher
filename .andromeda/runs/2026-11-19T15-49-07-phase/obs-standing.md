# obs · standing conventions

- S1 · every new or merged engine log line writes a fixed message and safe fields only, under a target the sink allowlist names or judges · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
- S2 · every new log target or call site is checked against the sink target allowlist and the capture's warn-and-above count · log-coverage · 6-log-coverage
- S3 · every new test that asserts empty stderr unsets RUST_LOG and installs no subscriber of its own · 3-observability-harness-contract
- S4 · every CI job added or edited uploads its per-leg log on failure and publishes nothing local · ci-integration · 9-ci-integration
