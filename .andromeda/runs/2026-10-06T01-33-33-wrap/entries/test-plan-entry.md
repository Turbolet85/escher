
## 2026-10-06-telemetry-bootstrap — stand log format measured; telemetry tests; workspace count re-counted
**Section:** §1 Coverage scope (escher-telemetry · tests/blitz-tests) · §3 Test Harness Contract (Stand log format · NOT YET MEASURED marker) · §9 CI Integration → Local baseline · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- §1: escher-telemetry holds 5 inline unit tests (three scrub branches, a bridged `log` record, the identity macro); tests/blitz-tests coverage adds process telemetry through `telemetry_stdout_silent`, `telemetry_scrub`, `telemetry_panic_hook`, `telemetry_init_idempotent`.
- §3: was "no … log format … was gathered"; now the stand's stderr line `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…` with the allowlist scrub (bound to obs-plan §3), exercised by those four files and the boot smoke `RUST_LOG=info timeout 10 target/debug/seven_guis_native` (exit 124, `service.name=seven_guis`); the marker keeps boot / status / cleanup / logs commands, status shape, PID file and test-data bootstrap.
- §9 Local baseline: `cargo test --workspace` re-counted 416 passed · 0 failed · 4 ignored, 114 result lines (was 407 · 0 · 3, 108 lines — its timings kept as the 2026-10-05 run's); the blitz-tests 255 · 0 · 3 and coverage-leg 404 · 0 · 3 readings marked as before this chunk's +9 tests / +1 ignored.
- 2 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk added the crate's unit tests, four one-process-per-behaviour integration files and the stand's log line.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/
