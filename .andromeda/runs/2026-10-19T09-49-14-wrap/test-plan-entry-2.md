## 2026-10-19-recorded-check-replay — test counts, the third flow script and the shot's same-host limit
**Section:** §1 Test Scope Summary (escher-driver; apps: seven_guis) · §2 Test Strategy (Command-line checks and shell flows, Pixel tests, Directory pattern) · §3 Test Harness Contract (blitz-tests session checks) · §3 → session-proof · §4 Unit Test Strategy · §6 E2E Test Strategy (Command line) · §9 CI Integration (Legs → stand-repeat, Engine features by runner) · §10 Quality Gates & Coverage Targets (flakiness budget)
**Change:**
- escher-driver unit tests 87 → 92 in fifteen files, 72 inline, `replay.rs` 5 listed; `recording.rs` rows reworked — refused 35 → 40, read 6 → 8, each record keeping its answer.
- seven_guis `recording` 3 → 4 (+1 off-unix): `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` through the binary and reads both checks byte-equal to the committed copies; `tests/flows/` and §2's shell flows hold three `sh` scripts (was two).
- `session_common`: `replay(session, check)` removed (its callers on `Check::replay`), `shot_prefix` added, readers 24 → 25; blitz-tests 135 `.rs` / 134 targets → 136 / 135.
- stand-repeat and the flakiness budget: 43 → 44 `stand_*.rs` files (+`stand_replay`), the leg's test pin naming it.
- `stand_screenshot_repeat`'s byte-identity (session-proof, §1, §2 Pixel tests, §9 Engine features) restated as on one host: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes — a committed check's digest included — holds no `screenshot` step.
**Why:** this chunk's own counts (the playbook's «Accurate this-chunk addition»); the same-host limit is the operator's resume directive, measured at CI#39225375577 (`Test (macos)`).
**Ref:** .andromeda/runs/2026-10-19T09-49-14-wrap/
