# security · standing conventions

- S1 · each new test target or test file in the harness crate takes no tracing and no style dependency, and the harness crate stays unpublished · dependency-security
  decision/blitz-tests-not-published · blitz-tests not published — The blitz-tests crate is never published (publish = false).
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each new or changed CI job or step uses actions pinned to commit SHAs and builds locked · dependency-security
  decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
- S3 · each new or changed CI job runs under a read-only token and reaches no secret · authentication-authorization
- S4 · each evidence file or measured reading the chunk commits holds no credentials, no temp or home path and no raw log · secret-management
  decision/agent-run-raw-log-unprinted · Agent-run raw log kept unprinted — The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
  decision/cold-agent-record-unprinted · Cold-agent record kept unprinted — The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
  decision/cold-agent-recordings-owner-only · Cold-agent recordings owner-only — The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
  decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
