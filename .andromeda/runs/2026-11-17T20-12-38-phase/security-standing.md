# security · standing conventions

- S1 · each new test file or fixture check joins the unpublished test crate and its harness, which takes no tracing and no style dependency · dependency-security
  decision/blitz-tests-not-published · blitz-tests not published — The blitz-tests crate is never published (publish = false).
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each new log line or measured stderr line goes through escher's allowlist stderr telemetry sink and holds no screen content, id, name or caller input · logging-monitoring
  decision/agent-run-events-metadata-only · Agent-run events hold harness metadata only — The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
  decision/cold-agent-events-metadata-only · Cold-agent events hold run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check.
  decision/cold-agent-logs-metadata-only · Cold-agent pipe logs run metadata only — The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check. · guard: The scrub-set key check and four planted markers, under the shim
- S3 · each CI leg change keeps locked builds, SHA-pinned actions and read-only tokens · dependency-security · authentication-authorization
  decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
  decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py
  decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
