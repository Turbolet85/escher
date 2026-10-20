- line: .andromeda/security-plan.md:222
old: «A harness settle (`Harness::settle`, and the driver session's `act` over it) reports a load in flight at once — `NotSettled` naming the class `Loads` — and never waits on one: it waits on no clock and moves none — its `animating` reading reads the overlay scrollbar fade off the document's clock — so the absent request timeout above cannot hang it, and its pass loop is bounded at 64 passes;»
new: «A harness settle (`Harness::settle`) reports a load in flight at once — `NotSettled` naming the class `Loads` — and never waits on one: it waits on no clock and moves none — its `animating` reading reads the overlay scrollbar fade off the document's clock — so the absent request timeout above cannot hang it, and its pass loop is bounded at 64 passes; the driver's acting step over it (`Session::act` and the executor's settled step, so `click` · `type` · `press` · `advance` · `scroll` on every carrier) settles again every 10 ms while the answer is `Loads`, for at most `LOAD_WAIT`, 5 s of wall time under the 30 s answer read below, moving no time on the held instance, then returns the load still in flight as `settled: false` with its busy class — so the absent timeout cannot hang a command either, `LOAD_WAIT` ending every wait (since 2026-10-20-load-wait);»

- line: .andromeda/security-plan.md:222
old: «1 to 60000 ms a call — no driver command waits on a load (the operator, 2026-10-07) —»
new: «1 to 60000 ms a call — an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none —»

- line: .andromeda/security-plan.md:83
old: «reads no clock and keeps no `NodeId` between calls»
new: «reads no clock itself — its settled step's load wait, the session's, reads the wall clock (below) — and keeps no `NodeId` between calls»

- line: .andromeda/security-plan.md:83
old: «and its executor, session, schema, validation and JSON writer and, since 2026-10-12-headless-screenshot, its `image` encoder read no clock and print nothing — the clocks are the host's, read on its accept loop for the idle expiry, and since 2026-10-10-mcp-surface the MCP server's, read when a request arrives with a session held,»
new: «and its executor, schema, validation and JSON writer and, since 2026-10-12-headless-screenshot, its `image` encoder read no clock, and none of them nor the session prints — the clocks are the host's, read on its accept loop for the idle expiry, since 2026-10-10-mcp-surface the MCP server's, read when a request arrives with a session held, and since 2026-10-20-load-wait the session's, read for the load wait alone (`Instant::now` and a sleep, bounded by `LOAD_WAIT`),»
