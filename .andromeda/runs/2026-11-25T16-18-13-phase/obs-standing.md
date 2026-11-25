# obs · standing conventions

- S1 · each new process check is silent, unsets RUST_LOG where it asserts an empty stderr, and its failure names a kind and a count, never an answer · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
- S2 · each new test or kept check reads answers in the match only and never logs or prints a graph, actions, layout, canvas or coverage answer · log-coverage · 6-log-coverage
- S3 · each new committed check, fixture or evidence file holds no private data, no secret and no temp or home path · ci-integration · 9-ci-integration
- S4 · each new test's names, labels and decoded screen text stay in the match and are never stored in a span field or log line · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
