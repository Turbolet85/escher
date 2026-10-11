
## 2026-10-10-mcp-surface — the MCP checks, the `tool_call` log line, the counts
**Section:** §1 Test Scope Summary (escher-driver · examples) · §2 Test Strategy → Process-lifecycle checks · Integration suites · §3 Test Harness Contract → Stand log format · §3 → Session lifecycle (`session-start`, `session-binary`, `session-checks`, `session-proof`) · §3 → agent-run counts · §5 → Session host ↔ what it writes · §6 → Drivers per surface (MCP) · §9 → the workspace count
**Change:**
- escher-driver: 50 → 59 unit tests in eleven files (`mcp.rs` 9); `cli.rs`'s reader table 61 → 64 rows.
- seven_guis: five → seven integration targets (`mcp_server` 2, `mcp_log` 1), read through `mod common;` by all seven; the shared module gains `string_at`, `Speaker`, `Spoke`, `REPLY_BOUND`. blitz-tests: `stand_mcp_tools` 2.
- Process-lifecycle: was "such a check is `cfg(unix)`-gated", now a check holding a session over the socket is; the MCP stdio-child checks through `Speaker` are not but `mcp_server`'s comparison, and run on windows (CI#38124028789 attempt 2). Same in `session-checks`.
- Log format (§2, §3): the `mcp` role writes one `tool_call` line per `tools/call`, `tool` and `ending` only, the parent of `command`; 9 / 10 lines over one CRUD flow at `info`. §5: `mcp_log` covers the role at `trace` under both builds.
- §6: a new MCP driver row, and the one live trial's figures (`client_exit 0 · isolated true · tool_calls 7 · end_state reached · protocol_version 2025-11-25 · discover sent`).
- Counts: `run stand` 127 · 0 · 6 over 33 files; workspace 805 · 0 · 11 over 170 result lines; `session-proof` gains the three MCP checks and the crate's 59; `session-binary` the `mcp` role; `session-start` the MCP server as `run`'s third caller.
**Why:** three checks and nine unit tests landed with the MCP server; the `tool_call` line is the founder's ratified span (2026-10-10, relayed by the overseer), bound to obs-plan §3. Standing trap: `tracing`'s callsite interest is process-global — a `with_default` check needs its own recorder per test.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
