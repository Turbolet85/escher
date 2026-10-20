
## 2026-10-19-visual-lint — the gated part: the `lint` verb's tests and the counts it moved
**Section:** §1 Test Scope Summary · §3 Test Harness Contract · §3 → Session lifecycle (`session-start` · `session-binary` · `session-proof`) · §4 Unit Test Strategy · §5 Integration Test Strategy · §6 E2E Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:**
- The verb table pinned at twelve verbs, eight instance-level, `lint` the eighth (was eleven and seven).
- The MCP listing is twelve, and describe's count row is `[12, 13, 12, 3, 8, 4, 15]`. json.rs's tests pin a finding's written form.
- Test counts:
  - `cli_commands` 13 (was 12): a hosted `lint` answers `{"findings":[]}`.
  - `mcp_server` 6 (was 5) and `stand_mcp_tools` 6 (was 5): a `lint` tool call.
  - `stand_lint` 6 (was 5): the `lint` verb through `Session::run` answers what the reader reads, 7 on the page and 0 on each task. Its «in process only» is retired.
  - `stand_act_spans` 34 calls per mode (was 32), reading a lint's line and a refused no-font one.
  - `host_log` and `mcp_log` each drive one `lint`, 0 needles.
- `session_common` gains `lint()` and `linted()`, and `act` covers a lint. The reader count stays 28.
- The cold-agent pipe: its server, `valid_step`, `isolated` and `FAKE` take twelve tools (was eleven). The 2026-10-15 live run stays dated.
- Session lifecycle: twelve verbs, eight instance-level, `lint`'s answer line. A recording keeps the eight instance verbs' calls. `lint` waits on nothing.
- Readings:
  - `run stand` 192 · 0 · 7 (was 190 · 0 · 7).
  - The workspace tally 204 · 1087 · 0 · 12 (was 1083).
  - CI#39387656399: linux 204 · 1087, macos and coverage 185 · 1084, windows 185 · 1041, stand repeat 165 · 672.
- Count histories («N until chunk») are dropped where a count moved.
**Why:** this chunk's measured counts, CI green 17/17 on ea3153f34 (CI#39387656399). The carriage is the founder's own word, F1 «Новая команда lint (Рекомендую)», relayed verbatim by the overseer (2026-10-20, chunk inputs#I7). His card confirm is inputs#I11.
**Ref:** .andromeda/runs/2026-10-20T07-57-08-wrap/
