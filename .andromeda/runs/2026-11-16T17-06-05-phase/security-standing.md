# security · standing conventions

- S1 · Each new test or fixture holds no real credential or secret; a secret-named field used to prove the scrub is a fixture stand-in, and live evidence holds no credentials. · secret-management
  decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
- S2 · Each new test proving a crossing of content asserts the value mask holds on that carrier (a masked control's value never appears) and that no content reaches a log line or span field. · input-validation · logging-monitoring
- S3 · Each new kept check or recorded check produced by a test stays owner-only and uncommitted, and its replay reads it strictly as untrusted. · data-protection
  decision/cold-agent-checks-read-for-counts-only · Cold-agent checks read for bools and counts only — The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
  decision/cold-agent-recordings-owner-only · Cold-agent recordings owner-only — The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
  decision/cold-agent-records-private · Cold-agent recordings stay private — The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
- S4 · Each new CI job or test leg runs locked builds and keeps the per-crate line coverage floors. · dependency-security · input-validation
  decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
  decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
  decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
