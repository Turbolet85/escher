
## 2026-10-15-cold-agent-test — the cold-agent pipe on the escher server: its registered resources
**Section:** §Occupied Resources (Names · Process-wide state and threads · Filesystem · Outbound hosts) · §Infrastructure Patterns (directory tree)
**Change:**
- Names: `scripts/cold_agent_stub.py` and its MCP server name `stub` are retired; the pipe's per-run `mcp.json` names the server `escher`, allowed as `mcp__escher` — the name the `escher-session mcp` role already holds.
- Process-wide state: the `claude` client spawns `sh` → `escher-session mcp` (`RUST_LOG=info` on that child only, its stderr to `target/cold-agent/server.log`) in place of `python3 cold_agent_stub.py`; the judge spawns two replay `escher-session mcp` children directly (stderr to `/dev/null`, 30 s per reply, each killed and reaped); no listener, port or socket is added.
- Filesystem: `target/cold-agent/` gains `server.log` (never printed or copied) and `check.json` (never printed) and loses `calls.jsonl` and `stub-state.json`; the session dir holds `mcp.json` alone (was "the stub copy and `mcp.json`"); the 2026-10-15 live copies (`live-run.md`, `live-verdict.json`, `live-events.jsonl`, `live-check.json`, `live-transcript.jsonl`) are registered beside the 2026-10-06 ones (was "the one recorded live run").
- Outbound hosts: the model-provider row adds that the run now carries the stand's screen — element ids, accessible names and control values — from the driver's own MCP server, one run.
- Directory tree: `scripts/` reads `cold-agent.{sh,ps1} (cold-agent pipe)`.
**Why:** the process crossings and the provider row record a boundary widening, ratified by the founder: his «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)» (2026-10-15, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels), confirmed by his «Подтвердить (Рекомендую)» at this wrap's answer card. No env var is new: `RUST_LOG` stays escher-telemetry's registered filter, set on a child.
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/
