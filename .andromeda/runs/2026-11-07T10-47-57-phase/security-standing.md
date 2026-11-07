# security · standing conventions

- S1 · a log line or span a change adds in escher's own code holds fixed words, bools and counts only, goes through escher_telemetry's target allowlist, and never carries document content, attribute text or a host path · logging-monitoring
  (security-plan/error-handling fct-bzel2phc) errors
- S2 · every cargo build and test the change runs, its new tests included, passes --locked and the cargo-deny audit leg, and no package enters Cargo.lock without being read name by name · dependency-security
  (security-plan/dependency-security fct-a4qh6ioi) deps
  (security-plan/dependency-security fct-6a266lkz) deps
  (security-plan/dependency-security fct-zrg6urnw) deps
  (security-plan/dependency-security fct-nyplqb5c) deps
  (security-plan/dependency-security fct-3t4xl422) dep-serde-json
  (security-plan/dependency-security fct-q4iqo3xv) dep-png
  (security-plan/dependency-security fct-bf3tngmb) dep-color
  (security-plan/dependency-security fct-n5tq3u6e) deps-supply-chain
- S3 · an env var read by a new test is registered as test-only and non-secret under Environment values read · Secret Management
- S4 · a panic, unwrap or expect path a change adds or removes in engine or bridge code is recorded under the Panic paths or Graceful degradation entries of error handling · error-handling
