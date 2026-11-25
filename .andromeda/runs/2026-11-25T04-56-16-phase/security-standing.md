# security · standing conventions

- S1 · each new test or fixture canvas keeps the per-crate line coverage floors and reaches the driver through the session as any caller does · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
- S2 · each new test harness or fixture takes no tracing subscriber and no style dependency of its own · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S3 · each new refusal path a test asserts reads its cause from the closed refusal cause set, with no call input in it · error-handling
