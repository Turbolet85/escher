# a11y · standing conventions

- S1 · every new accessibility assertion is a Rust test over the AccessKit tree, run in the a11y leg · 3-a11y-assertion-harness-contract
- S2 · every new stand check runs in the slow-tier accessibility leg, gates CI, and is repeated three rounds · ci-integration · 9-ci-integration
- S3 · every new log line passes the sink's target allowlist, printing only allowed fields and redacting values · 3-a11y-assertion-harness-contract
