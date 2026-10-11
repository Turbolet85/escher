# Live trial — 2026-10-10-mcp-surface

The operator pass's one live agent session through the escher MCP server (plan entry 18, on the founder's word —
fork 3A, `inputs#I3`), fired once on 2026-10-11 at 08:09:58Z through
`gate.py run --plan escher-0.1.0/chunks/2026-10-10-mcp-surface/plan.md --operator 18`, client `claude` 2.1.296.

The entry's line:

```
 18 e2e         green · exit 0 · 10.28s · 697 B → 18.log · history: unmoved · cargo build --locked -p seven_guis --bin escher-session &&… (132 chars)
```

The script's printed line, copied from the entry's log:

```
trial: client_exit 0 · isolated true · tool_calls 7 · refused 0 · errors 0 · end_state reached · protocol_version 2025-11-25 · discover sent
```

Freshness: the four files under `target/mcp-trial/` were written 08:10:00Z–08:10:08Z, after the firing began (the
script removes them before its run). Not committed, and nothing of their content is copied here beyond counts.

Two readings the route pin for the current MCP revision owes (plan §Implementation notes, *Owed at the wrap*): the
host client asked `initialize` for revision `2025-11-25`, which this server does not speak — it answered `2025-06-18`
and the client proceeded — and the client sent `server/discover` (answered `-32601`) before falling back to the
handshake.
