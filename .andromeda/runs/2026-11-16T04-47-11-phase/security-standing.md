# security · standing conventions

- S1 · each crate's line coverage holds its per-crate floor; new driver and telemetry code is covered by its tests · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/coverage-floor-input-validation · Coverage floor checker validates and prints no paths — coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path. · guard: CoverageFloorTest
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
- S2 · a new test target takes no tracing and no style in its harness unless its own measure needs it; dependencies stay workspace-inherited and builds locked · dependency-security
  decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
  decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
- S3 · every new log line goes through escher's own allowlist sink, by an escher_ target, with content-named fields scrubbed · logging-monitoring
- S4 · a new CI job or test target runs across the CI matrix with every job run · error-handling
  decision/ci-matrix-runs-every-job · CI matrix runs every job — CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
