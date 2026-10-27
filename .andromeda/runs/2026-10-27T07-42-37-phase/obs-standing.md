# obs · standing conventions

- S1 · each test target the take carries installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, confirmed by the silence census per file · §3 Observability Harness Contract (Logging stack)
  (.andromeda/obs-plan-detail/log-coverage.md:123) logs
  (.andromeda/obs-plan-detail/log-coverage.md:124) logs
  (.andromeda/obs-plan-detail/span-trace-coverage.md:17) spans
- S2 · an engine log site is compiled only behind its crate's `tracing` feature with a no-op fallback · §2 Telemetry Strategy · .andromeda/obs-plan-detail/telemetry-strategy.md
- S3 · no new user-content log field; a value logged as-is is recorded, and only the sink's allowlist and scrub decide what reaches stderr · §8 PII Scrubbing & Compliance · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
- S4 · a measured fact cites the chunk report or evidence that measured it · §3 Observability Harness Contract
