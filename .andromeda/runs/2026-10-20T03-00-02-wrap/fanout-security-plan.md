proposals:
  - detector: D-security-input
    severity: escalate
    section: "§API Security → Settle and loads in flight (row)"
    change: "Split the row's claim: `Harness::settle` still reports a load in flight at once — `NotSettled` naming `Loads` — and never waits on one, waiting on and moving no clock; since 2026-10-20-load-wait the driver's acting step (`Session::act` and the executor's `settled_step`, so `click` · `type` · `press` · `advance` · `scroll` on every carrier — in process, the socket, the command line, MCP) settles through `settle_waiting`, which settles again every 10 ms (`LOAD_POLL`) while the result is `Loads`, up to `LOAD_WAIT` (5 s of wall time, no time moving on the held instance), then returns the step `settled: false` with `busy` loads — so the absent request timeout above still cannot hang a command, because `LOAD_WAIT` ends every wait (under the 30 s `ANSWER_BOUND`); `snapshot` and `screenshot` run no step and do not wait."
    sidecar: "security-plan §API Security, Settle and loads in flight: the driver's acting step now waits on a load in flight up to LOAD_WAIT (5 s wall, no virtual time); Harness::settle itself still never waits; the absent request timeout still cannot hang a command (2026-10-20-load-wait)."
    rationale: "Report Changes → Symbols / APIs: `settle_waiting` / `settle_within` (session.rs:24-29, 31-52), `Session::act` and `execute::settled_step` (execute.rs:462) now wait on `Busy::Loads` up to `LOAD_WAIT`; Counts / qualifiers moved, fact `wait`, marks security-plan.md:222@c187 «changed — the driver's `act` now waits, bounded by `LOAD_WAIT`»; Expected amendments name this row. The row's «and the driver session's `act` over it … never waits on one» is no longer true. No new external input (a constant, not an argument) — filed under this detector as the row guarding the absent request timeout at the load boundary."
    basis: "packages/escher-driver/src/session.rs:16-53"
  - detector: D-security-input
    severity: escalate
    section: "§API Security → Settle and loads in flight (row)"
    change: "Replace «no driver command waits on a load (the operator, 2026-10-07)» with: an acting driver command waits on a load in flight up to `LOAD_WAIT`, 5 s, stated in the `settled` result field's help, and no command waits longer — still no argument or result field of a wait (since 2026-10-20-load-wait); the command line's own 30 s bounds still bound a transport and wait on no load."
    sidecar: "security-plan §API Security, Settle and loads in flight: «no driver command waits on a load» retired — an acting command waits up to LOAD_WAIT (5 s), stated in the settled help (2026-10-20-load-wait)."
    rationale: "Report Retired bullet's note and fact `wait`: security-plan.md:222@c2234 «no driver command waits on a load (the operator, 2026-10-07)» changed; Schema / config: the `settled` help (schema.rs:121) now states the 5 s bound, no verb, argument or result field added."
    basis: "packages/escher-driver/src/schema.rs:121"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Driver command schema (escher-driver) (row)"
    change: "Where the row says the executor «reads no clock and keeps no `NodeId` between calls», say instead that the executor reads no clock itself — its settled step settles through the session's `settle_waiting`, whose load wait reads the wall clock (`Instant::now`) and sleeps, bounded by `LOAD_WAIT` (5 s), since 2026-10-20-load-wait — and keeps no `NodeId` between calls."
    sidecar: "security-plan §Input Validation, Driver command schema: the executor's settled step now reads the wall clock through the session's load wait (2026-10-20-load-wait)."
    rationale: "Report fact `clockrd`: security-plan.md:83@c4819 «changed — the executor's settle now reads the wall clock through `settle_waiting`»; probe `grep -rln -E 'Instant::now|thread::sleep' packages/escher-driver/src` reads `client.rs host.rs mcp.rs session.rs`; Expected amendments name this row (session.rs reads a wall clock for the wait). Validation's own «reads no clock» (83@c2692) stays true."
    basis: "packages/escher-driver/src/execute.rs:462"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Driver command schema (escher-driver) (row)"
    change: "Where the row says «its executor, session, schema, validation and JSON writer and … its `image` encoder read no clock and print nothing — the clocks are the host's … and … the MCP server's», take the session out of the no-clock list: its executor, schema, validation, JSON writer and `image` encoder read no clock, the session reads the wall clock (`Instant::now`, `std::thread::sleep`) for the load wait and nothing else, and none prints — the clocks are the host's (idle expiry), the MCP server's (a request arriving with a session held) and, since 2026-10-20-load-wait, the session's load wait, bounded by `LOAD_WAIT`."
    sidecar: "security-plan §Input Validation, Driver command schema: the session joins the driver's clock readers, for the load wait alone (2026-10-20-load-wait)."
    rationale: "Report fact `clockrd`: security-plan.md:83@c8549 «changed — the session's wait joins them»; Symbols / APIs: `settle_within` reads `std::time::Instant` and calls `std::thread::sleep` and nothing else — no print, no `tracing` call."
    basis: "packages/escher-driver/src/session.rs:31-52"
    dependent-of: D-security-input
note: "D-security-auth and D-security-deps do not fire (no identity, token, key or secret touched; Dependencies: none, Cargo.toml/Cargo.lock diff 0). D-security-input finds no new external-input surface (LOAD_WAIT is a constant, not an argument); the four proposals are the report's Expected amendments for security-plan (facts `wait` 222@c187/c2234 and `clockrd` 83@c4819/c8549), filed under D-security-input as the nearest detector, so its escalate severity is the detector's, not a measured unvalidated boundary. The second and fourth proposals depend on the first and third respectively (two distinct retired claims: «the driver waits on no load» and «the driver session reads no clock»). Unchanged on purpose: 222@c2 row label, 222@c909 (no argument or result field of a wait — still true), 222@c2006 (the command line's 30 s bounds wait on no load — still true), 83@c2692 and 222@c1861 (validation reads no clock)."
