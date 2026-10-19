# security extract

## Relevance
partial. The chunk moves test code only and opens no new surface. Two of the checks it touches are instruments the plan measures by: the log checks `host_log` and `mcp_log`, and escher-driver's `recording.rs` and `cli.rs` unit tests, which pin the recorded-check reader and the argv reader.

## Constraints
- The shared log-check fragment in `examples/seven_guis/tests/common/mod.rs` must keep every hygiene reading that `host_log` and `mcp_log` assert today: 0 occurrences of the typed sentinel, 0 id needles and 0 name needles in the host's and the `mcp` role's stderr at `RUST_LOG=trace`, 0 occurrences of a shot's first 64 base64 characters, and the closed-span line counts (`command`, `tool_call`). Each reading must still be asserted by both checks under both builds (per security-plan §Logging & Monitoring).
- Merging the clones must not narrow a needle set, drop a level or build, or swap an exact count for a looser bound. The plan's no-content-in-stderr readings rest on those checks (per security-plan §Logging & Monitoring).
- `recording.rs`'s moved test modules must keep, unchanged, the strict-reader rows that the plan names for `Check::from_bytes`: 40 refusal rows, 8 read rows and edge rows at both bounds, with every refusal `malformed` and holding nothing of the file (per security-plan §Input Validation, Recorded check row).
- The same moved modules must keep the recording-directory pins unchanged: `0700` directory, a group- or other-open one refused `record-dir-not-private`, `check-{n}.json` `0600` with `create_new`, the 400-record and 1,048,576-byte bounds ending the recording, and `off_unix_tests`' `Unsupported` under its own `cfg` gate (per security-plan §Input Validation, Driver recording directory row).
- `cli.rs`'s moved `tests` module must keep the closed argv reader's pins unchanged: the four endings and statuses, `--record` placement, and nothing the caller typed written back on usage, refusal or session error (per security-plan §Input Validation, CLI arguments row).
- The seven_guis test crate may read only the registered build-time `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")`. Moving the fragment into `tests/common/mod.rs` adds no env var. A new one would need registering first (per security-plan §Secret Management).
- The chunk adds no dependency: `Cargo.toml`, `Cargo.lock` and `deny.toml` stay unchanged (per security-plan §Dependency Security).

## Patterns to follow
- Shared session-check plumbing already lives in `examples/seven_guis/tests/common/mod.rs`, which the plan cites as a site of the registered build-time env reads. The log fragment joins that module rather than a new shared file (per security-plan §Secret Management).
- The log checks follow one census pattern: run the sink-installing child at `RUST_LOG=trace`, capture its stderr whole, and count the sentinel, id, name and shot-prefix needles plus the span lines, under `-p seven_guis` and `--workspace` builds. The fragment states that pattern once and both checks call it (per security-plan §Logging & Monitoring).
- A test that holds a recorded check's bytes or a refusal asserts on closed words and counts, never on the content it read (per security-plan §Error Handling).

## Anti-patterns to avoid
- No assertion message, panic text or helper output in the shared fragment may print the captured stderr, an id, a name or the typed sentinel. Doing so would put content into test output that the hygiene checks exist to keep out of logs (per security-plan §Logging & Monitoring).
- Do not add a `String` or call-supplied text to a `Refusal`, `Fault` or `Cause` while moving tests. Do not loosen an assertion that a refusal holds nothing of the call or the file (per security-plan §Error Handling).

## Contract bindings
- security §Logging & Monitoring ↔ obs-plan: `host_log` and `mcp_log` are the shared measuring instruments for the sink's no-content readings. A change to the merged fragment moves both plans' readings.
- security §Logging & Monitoring and §Secret Management ↔ the wrap's citation cascade: the plan cites `examples/seven_guis/tests/host_log.rs:18`, `host_log.rs:212-225`, `examples/seven_guis/tests/mcp_log.rs:51-333` and `examples/seven_guis/tests/common/mod.rs:14-16` by line, so moving the fragment re-points those citations. The `recording.rs` and `cli.rs` production-line citations (all before line 454) should stay put. Whether they do is research's question.
- security §Input Validation ↔ tests (coverage floor and mutation tier): the recording-reader and argv-reader pins must still count toward escher-driver's coverage and mutant targets after the move. P3 closes this against `.github/scripts/coverage_floor.py`.

## Acceptance criteria contributions
- `host_log` and `mcp_log` pass under `cargo test -p seven_guis` and in the workspace test leg, both calling the one shared fragment. Each still asserts 0 typed-sentinel occurrences, 0 id needles, 0 name needles and 0 shot base64-prefix occurrences at `RUST_LOG=trace`, with the same span-line counts as before (per security-plan §Logging & Monitoring).
- escher-driver's recording and cli unit tests run under the same names and the same total count before and after the split, including every `Check::from_bytes` refusal and bound row and `off_unix_tests` (per security-plan §Input Validation).
- A grep over `examples/seven_guis/tests/` finds no env read beyond `CARGO_BIN_EXE_escher-session` and `CARGO_TARGET_TMPDIR` (per security-plan §Secret Management).
- `Cargo.lock` and `deny.toml` are byte-identical to the pre-chunk tree, and `bash .github/scripts/ci-leg.sh audit` stays green (per security-plan §Dependency Security).
