
## 2026-10-19-test-splits — the log checks' shared fragment and its silence census
**Section:** §3 Observability Harness Contract → Logging stack (`absence · logs` group)
**Change:** a census item added: escher-driver's `cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs` and seven_guis' shared `tests/common/mod.rs` are silent — 0 print, tracing and subscriber sites over the four; the shared module now states once, for `host_log` and `mcp_log` alone, `Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles`, its assertion messages a kind and a count, never a needle or a captured line; escher-driver still holds its 2 `info_span!` sites, the env-read set unchanged.
**Why:** this chunk's own test-only move (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendment); the log checks the security and obs readings rest on still assert the same zeros.
**Ref:** .andromeda/runs/2026-10-19T21-24-54-wrap/
