# obs · standing conventions

- S1 · each test target a take carries or the chunk adds installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the silence census reading 0 env, 0 print and 0 subscriber per file · §3 Observability Harness Contract, Logging stack
  (.andromeda/obs-plan-detail/log-coverage.md:130) logs
  (.andromeda/obs-plan-detail/log-coverage.md:134) logs
  (.andromeda/obs-plan-detail/log-coverage.md:137) logs
  (.andromeda/obs-plan-detail/log-coverage.md:138) logs
- S2 · each engine log site is compiled only under the `tracing` feature, with a no-op path when it is off · §2 Telemetry Strategy and .andromeda/obs-plan-detail/telemetry-strategy.md
- S3 · each log line or span field passes the sink's scrub, holds no page text, id, accessible name or typed value, and stands in the sink's one-line format · §6 Log Coverage, Log format, and §8 PII Scrubbing & Compliance, Scrubbing
- S4 · a CI step, job or gate holds nothing of a user's in its diagnostics, and its output stays in its step log or leg log, with no artifact beyond the fork's two · §9 CI Integration and .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:13) ci
