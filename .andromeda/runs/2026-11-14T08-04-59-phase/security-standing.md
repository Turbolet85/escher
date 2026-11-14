# security · standing conventions

- S1 · each new or changed CI job or step pins its actions to commit SHAs and builds locked · dependency-security
  decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
- S2 · each new CI job runs with a read-only token and reaches no secret · authentication-authorization
  decision/ci-secrets-in-github-actions · CI secrets in GitHub Actions — Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
  decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py
  decision/publish-job-repository-guard · Publish job repository guard — The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
  decision/publish-secrets-upstream-main-only · Publish secrets reachable only upstream — The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.
- S3 · each new log line goes through escher's allowlist stderr sink and carries no screen content or caller input · logging-monitoring
  decision/escher-telemetry-allowlist-sink · escher's allowlist telemetry sink — Both seven_guis binaries log through escher's own stderr-only allowlist subscriber. · except: The scrubbing reaches that sink only: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
  decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
  decision/select-list-label-never-logged · Select list row labels never logged — The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
  decision/stderr-only-telemetry-sink · Escher's own stderr telemetry sink — Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.
- S4 · each new committed test fixture or check file holds readings only, no user data and no credentials · data-protection
- S5 · each new test counts toward its crate's line coverage floor, which the change must keep · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
