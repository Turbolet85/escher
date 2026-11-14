# security · standing conventions

- S1 · each new test file reaches the driver through the test harness, which takes no tracing and no style · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each new fixture or fixture state a test serves is named in the closed fixture table, its name checked against it · input-validation
- S3 · each new line of driver code stays under its crate's line coverage floor · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/coverage-floor-input-validation · Coverage floor checker validates and prints no paths — coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path. · guard: CoverageFloorTest
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
- S4 · each new log line is one sink line per closed span on an escher target, content-named fields withheld · logging-monitoring
  decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
  decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but eleven content-named ones.
  decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which print as [redacted].
  decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
  decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed
