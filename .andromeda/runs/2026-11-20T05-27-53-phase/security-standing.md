# security · standing conventions

- S1 · a new test file or test module takes no tracing and no style dependency · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · a new test's helpers and assertions put no screen content, stable id or accessible name in a log line · logging-monitoring
