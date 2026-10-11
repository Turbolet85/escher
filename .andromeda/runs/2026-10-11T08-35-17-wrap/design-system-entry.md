
## 2026-10-10-mcp-surface — the `mcp` role's stdout on the cli surface
**Section:** §Surface: cli → Component Patterns (the `escher-session` bullet)
**Change:** the `escher-session` bullet gains its `mcp` role — a surface an agent's client reads: newline-delimited JSON-RPC 2.0 protocol lines only on stdout (a reply per request that is not a notification; a tool result's text the screen text for `snapshot`, the JSON line otherwise), uncoloured, no ESC byte in any reply and still no terminal test; its end writes nothing there, an `Err` ending's fixed message on stderr with status `3`.
**Why:** a new mode of the same cli surface; the no-ESC reading is asserted on every reply by the shared `Speaker` and the no-terminal-test search counts 0. No token, colour or layout entered.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/
