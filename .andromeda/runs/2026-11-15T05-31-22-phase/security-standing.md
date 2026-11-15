# security · standing conventions

- S1 · each new log line or log target the take brings passes escher's allowlist telemetry sink, its fields withheld unless allowlisted, holding no screen content, accessible name or stable id · logging-monitoring
- S2 · each new or removed dependency the take brings is reviewed by hand, passes the cargo-deny advisory and licence gate, and enters only from the registry, inherited from the workspace · dependency-security
  decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
- S3 · each CI workflow change the take brings keeps actions pinned to commit SHAs and tokens read-only · dependency-security · authentication-authorization
  decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
  decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py
