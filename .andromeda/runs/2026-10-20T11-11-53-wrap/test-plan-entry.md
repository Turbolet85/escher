
## 2026-10-20-large-screen-in-parts — the parts check, `--part` and the counts it moved
**Section:** §1 Test Scope Summary · §2 Test Strategy · §3 Test Harness Contract (crate-local helpers · agent-run Proof · test-data bootstrap) · §3 → Session lifecycle (`session-start` · `session-binary` · `session-proof`) · §4 Unit Test Strategy · §5 Integration Test Strategy · §6 E2E Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:**
- New check `stand_snapshot_parts` 3 (§1, `session-proof`): a fixture screen over `SNAPSHOT_TEXT_BUDGET` walked in parts through `Session::run` and `serve_mcp` in both layout modes, and a screen inside the budget read whole.
- Unit counts: dioxus-native-dom 81 (was 75), six on `Snapshot::text_parts`; escher-driver 100 (was 99), 36 in four `#[path]` files (was 35), `mcp_tests.rs` 15 (was 14), the `snapshot` tool's optional `part`. §4: seven argument kinds (was six), `ArgKind::Part` 1 to `MAX_PART` = 1000.
- `session-start`: `snapshot` returns `Outcome::Screen`, or `Outcome::Part` for a screen over the budget or a `part` asked (was `Outcome::Screen` alone).
- `cli_commands` 14 (was 13): `snapshot --part` by number, and 0, 1001 and `x` refused, exit 1. `session-binary`: `--part` with one value, the paged answer line, the usage line opening `snapshot [--part <part>]`.
- `stand_act_spans` 35 calls per mode (was 34). `host_log` and `mcp_log` each drive one part read, 0 needles.
- `session_common` 29 readers (was 28), gaining `snapshot_part` and `part`; `fixtures` 24 readers (was 23), 22 `stand_*` (was 21). Stand-repeat and the flakiness budget: 48 files (was 47).
- Readings, each the newest only: `run stand` 195 · 0 · 7 over 48 files (was 192 · 0 · 7 over 47); the workspace tally 205 · 1098 · 0 · 12 (was 204 · 1087); CI#39410865604: linux 205 · 1098, macos and coverage 186 · 1095, windows 186 · 1051, stand repeat 168 · 684 (was CI#39387656399's 204 · 1087, 185 · 1084, 185 · 1041, 165 · 672).
**Why:** this chunk's measured counts, CI green 17/17 on 191e66fdb (CI#39410865604). The paged carriage is the founder's own word, F1 «Аргумент part, части по умолчанию (Рекомендую)», relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm is inputs#I5.
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/
