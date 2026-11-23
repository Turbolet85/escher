# obs · standing conventions

- S1 · each new test is a silent check over shared modules, printing nothing of its own and unsetting RUST_LOG where it asserts empty stderr · 3-observability-harness-contract
- S2 · each new headless stand test runs with no telemetry sink installed · 3-observability-harness-contract
- S3 · each new committed test, fixture or kept log holds no secrets or private data, local state staying local · 9-ci-integration · ci-integration
  decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
  decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
  decision/committed-transcript-copies-masked · Committed transcript copies masked — Each committed copy of a cold-agent transcript masks the session dir and the home prefix.
  decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.
