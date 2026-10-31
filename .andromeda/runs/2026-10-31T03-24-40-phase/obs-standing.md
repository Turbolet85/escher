# obs · standing conventions

- S1 · Each new or extended check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, in both layout modes · `## 3. Observability Harness Contract`
- S2 · Each new check is covered by a silence census reading 0 over it and over any shared module it reads, recorded as a line with the chunk's report as its measure · `## 3. Observability Harness Contract`
- S3 · A new check that declares `tests/blitz-tests/tests/common/mod.rs`, `session_common/mod.rs` or `fixtures/mod.rs` joins that module's recorded reader count · `## 3. Observability Harness Contract`
- S4 · A new fixture page or network stand-in holds no `blitz_net`, print, `tracing` call, env read, file, socket or listener · `## 3. Observability Harness Contract`
- S5 · Each engine change adds no log site, span, field, log field or env read, and the driver keeps its 2 `info_span!` sites and 0 events · `## 3. Observability Harness Contract`
  (.andromeda/obs-plan-detail/log-coverage.md:125) logs
  (.andromeda/obs-plan-detail/metric-coverage.md:34) metrics
- S6 · A blitz-dom log site, should a fix add one, compiles only with `#[cfg(feature = "tracing")]` and has a no-op path with the feature off · `## 2. Telemetry Strategy`
- S7 · A line under an engine target prints only the safe fields, the rest redacted; a driver line holds closed words, bools and counts, never screen text, an argument or an `Outcome` · `## 8. PII Scrubbing & Compliance`
  (.andromeda/obs-plan-detail/ci-integration.md:12) ci
  (.andromeda/obs-plan-detail/ci-integration.md:13) ci
  (.andromeda/obs-plan-detail/ci-integration.md:14) ci
  (.andromeda/obs-plan-detail/ci-integration.md:17) ci
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
- S8 · Each engine or driver change leaves `host_log` and `mcp_log` passing under both builds, reading 0 id, name and typed-text needles at `trace` · `## 3. Observability Harness Contract`
  (.andromeda/obs-plan-detail/span-trace-coverage.md:30) spans
