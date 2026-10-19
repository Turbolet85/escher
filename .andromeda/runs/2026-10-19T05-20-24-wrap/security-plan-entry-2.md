## 2026-10-19-session-recording — wire v3, fifteen session errors, `call_of`'s home, serde_json and `CARGO_MANIFEST_DIR`
**Section:** §Input Validation (Driver session socket, Driver command schema, MCP server, the `id` row's hosted answer) · §API Security · §Error Handling · §Dependency Security · §Secret Management
**Change:**
- The socket's closed grammar: was wire `v2`; now `v3` — `hello v3`, `stop v3`, `call v3 …`, `ok v3 accepted|refused <json>`, `ok v3 oversize`, and the fixed `ok v3 recording-failed` sent in place of an answer when the host's recording ended, read as `SessionError::RecordingFailed`; the `id` row's hosted answer, §API Security's listener and §Error Handling's refusal line the same.
- §Error Handling: was `SESSION_ERRORS: [ErrorSpec; 13]`; now 15, `record-dir-not-private` and `recording-failed` its new rows, each exit `3`, `help`'s `session_errors` serving 15.
- The schema and MCP rows cite `call_of` at `command.rs:32-60` (was `mcp/result.rs:15-42`).
- §Dependency Security: `serde_json` also reads a kept check's bytes, still writing nothing.
- §Secret Management: `CARGO_MANIFEST_DIR` is also read at build time by escher-driver's unit tests, for the workspace `target/tmp`.
**Why:** this chunk's own facts (the playbook's «Accurate this-chunk addition»); the reply word and the two errors' fixed text sit inside ratified carriers («A field inside a ratified carrier», «The program's own fixed text inside a ratified carrier»); `CARGO_MANIFEST_DIR` is an env read already registered, test-only here.
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
