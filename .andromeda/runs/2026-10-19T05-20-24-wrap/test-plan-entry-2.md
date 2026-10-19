## 2026-10-19-session-recording — the recording's tests, the counts they move and the stand-repeat list
**Section:** §1 Test Scope Summary (apps, blitz-dom (clock), escher-driver, dioxus-native-dom, tests/blitz-tests) · §2 → Directory pattern · §3 → Crate-local test helpers (session checks), Stand log format · §4 → escher-driver · §5 → Session host ↔ what it writes · §6 → Command line, MCP · §9 → Legs · §10 → Flakiness budget
**Change:**
- §1 apps: seven_guis holds eight integration-test targets (was seven) with `tests/recording.rs` (3, unix only, +1 off-unix), `mod common;` read by all eight; `host_log` and `mcp_log` 2 tests each (were 1); the host role `[--record <dir>]`.
- §1 escher-driver: was 74 unit tests in thirteen files; now 87 in fourteen, 67 inline and 20 in two `#[path]` files, three off-unix besides — error.rs over fifteen variants (was thirteen), session.rs 6 (was 4), wire.rs's `v3` grammar (was `v2`) with `recording-failed`, mcp_tests.rs 14 (was 12), recording.rs 9, describe.rs's count row `[11, 13, 12, 3, 8, 4, 15]`.
- §1 dioxus-native-dom: was 72 unit tests; now 73 (`is_masked_input`'s seven rows). §1 blitz-dom (clock): pan and the fling among the reads, and `iframe`'s carried-clock test. §1 tests/blitz-tests: `stand_session_recording` 3 among the session checks; `virtual_clock_engine` 5 (was 4).
- §2: was 134 `.rs` files and 133 targets; now 135 and 134; `session_common` read by 24 checks (was 23). §3 helpers: `replay`, `kept` and the unix-only `mode` (24 readers). §3 Stand log format: the host role `[--record <dir>]`, spans the same with a recording held.
- §4 escher-driver: wire.rs's grammar at `v3` with `ok v3 recording-failed` (was `v2`); error.rs over the fifteen errors; the recorder's, the reader's and the recording-failed unit tests.
- §5: the recorded `host_log` / `mcp_log` runs read 0 recording needles, span counts unchanged. §6: `start --record` and `cli_commands`' usage line; `mcp_log` 2 and `recording`'s `mcp --record` runs.
- §9: stand-repeat runs 43 `stand_*` files (was 42) and seven_guis' `recording` too, both words pinned; §10: 43 files and eight process checks (were 42 and seven).
**Why:** this chunk's own shipped tests and the counts they move (the playbook's «Accurate this-chunk addition»).
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
