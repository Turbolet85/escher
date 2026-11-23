# obs · standing conventions

- S1 · each new test or check is silent on success, unsets RUST_LOG where it reads empty stderr, and a failure names a kind and a count · 3-observability-harness-contract
  decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
- S2 · each new headless stand fixture runs with no sink and logs no answer, part or shot content · 3-observability-harness-contract
- S3 · each new committed check, fixture or expected table holds no secret or private data, and local state is never uploaded from CI · 9-ci-integration · ci-integration
  decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
  decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.
- S4 · a shot image leaves only in its answer and is never logged, stored in a span or kept in a recording · pii-scrubbing-compliance · 8-pii-scrubbing-compliance
