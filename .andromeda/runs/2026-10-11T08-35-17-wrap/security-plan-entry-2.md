
## 2026-10-10-mcp-surface — `serde_json` reviewed; the `tool_call` span; the live trial's login
**Section:** §Dependency Security → Pinning · §Logging & Monitoring (escher's sink · Stdout output) · §Secret Management → Storage (Development) · What counts as secret
**Change:**
- Pinning: `serde_json = "1"` (workspace, no `[features]`, so no `preserve_order`), the reader of the MCP server's untrusted stdin, writing nothing — reviewed by hand: no package enters or moves, one lock line, already in both graphs, no adapter, audit green, no ignore.
- The sink prints a second `escher_driver` span, `tool_call` (`tool`, `ending`, neither in `CONTENT_FIELDS`), the parent of `command`; the `mcp` role reads 9 lines built alone and 10 with the workspace over one CRUD flow at `info`, 0 typed sentinels, 0 ids, 0 names at `trace` under both builds. Stdout: the role writes protocol lines only, no ESC byte, the admitted `id` its one echo.
- Development: was "the cold-agent live run only", now also the 2026-10-10-mcp-surface live trial — the same login, no new secret source; What counts as secret names both.
**Why:** `serde_json` as a reader only is the founder's answer (2026-10-10, relayed by the overseer). The span is a boundary widening (a new log field) ratified by him with the same confirm at this wrap's Validate. The trial's use of the login rests on his word for the one live trial, recorded at this wrap on his answer relayed by the overseer as the operator's.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
