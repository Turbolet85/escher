
## 2026-10-10-mcp-surface — the `tool_call` span, ratified; the `mcp` role on the sink
**Section:** §2 Telemetry Strategy (the mechanism bullet) · §3 Observability Harness Contract → Logging stack (the `escher-session` and headless-stand bullets) · §4 Span / Trace Coverage (escher-driver) · §6 Log Coverage (examples · escher-driver) · §8 PII Scrubbing → Scrubbing
**Change:**
- §4: a second span, `tool_call` — `info_span!(target: "escher_driver", "tool_call", tool, ending)`, one per MCP `tools/call`, entered before its params are read and held until its reply is written; `tool` the table's word, `ending` `accepted` · `refused` · `error`; `command` its child by nesting, the executor unchanged; session verbs leave a `tool_call` with no child. Was "the span is `run`'s" alone; `serve_mcp` outside a tool call writes no record.
- §2: was "one span macro", now two; the `mcp` role reaches both. §8: was "`escher_driver` now has an emitter", now two, neither of `tool_call`'s fields content-named.
- §6: the `tool_call` closed-span line and its emitters; the role's 9 / 10 lines over one CRUD flow at `info` (5 `tool_call`, 3 `command`); the role's endings and usage.
- §3: the `mcp` role (protocol lines on stdout, the sink stderr-only, still two install sites); `serve_mcp`'s replies a further stdout write, never a log line; `stand_mcp_tools`' in-process recorder a second exception to "installs no subscriber"; was "three more seven_guis targets … the five share", now two more, the seven share, and `mcp_log` reads the role at `trace` — `host_log` no longer the one such check.
**Why:** a boundary widening (a new log field) ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer; his confirm at this wrap's Validate, relayed by the overseer, as the operator's answer) — recorded ratified. Both fields are closed words; no content field was added.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
