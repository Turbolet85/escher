# obs · standing conventions

- S1 · each new test is a silent check — it prints nothing on success and any shared helper it adds stays test-only · 3-observability-harness-contract
- S2 · each new test runs in a CI leg whose log is uploaded only on that leg's failure, and the tests keep no local state for upload · 9-ci-integration · ci-integration
  decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
  decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
