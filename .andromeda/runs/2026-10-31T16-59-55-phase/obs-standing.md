# obs · standing conventions

- S1 · Each new or extended check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, the silence census reading 0 over it · §3 Observability Harness Contract in the body
  (.andromeda/obs-plan-detail/log-coverage.md:127) logs
  (.andromeda/obs-plan-detail/log-coverage.md:128) logs
  (.andromeda/obs-plan-detail/log-coverage.md:131) logs
  (.andromeda/obs-plan-detail/span-trace-coverage.md:17) spans
  (.andromeda/obs-plan-detail/telemetry-strategy.md:22) strategy
- S2 · Each new check joining a shared module (`tests/blitz-tests/tests/common/mod.rs`, `session_common/mod.rs`, `fixtures/mod.rs`) joins that module's recorded reader count · §3 Observability Harness Contract in the body
- S3 · Each new check reading a sink's output fails with a kind and a count, never a needle or a captured line · §3 Observability Harness Contract in the body
- S4 · Each new engine log line compiles only under `#[cfg(feature = "tracing")]`, with a no-op path when the feature is off · §2 Telemetry Strategy in the body
- S5 · Each new log line or span field passes the sink's allowlist and scrub, holding closed words, bools and counts and no page text, id or value · §8 PII Scrubbing & Compliance in the body and .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) pii
- S6 · Each new silence or absence row carries its file:line citation or its as-measured chunk report · the `absence · logs` detail in §3 of the body
