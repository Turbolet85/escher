proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 8
    section: '## API Security'
    change: 'Line 8 opens: No served network API exists — no TCP or UDP port served (two `#[cfg(test)]` tests of blitz-net each bind one, `std::net::TcpListener::bind("127.0.0.1:0")` on loopback, served by no binary, reached by no caller, so no crossing, registered test-only since 2026-11-01-upstream-sync-ui-graph; packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534); the one served listener is local: the driver session''s Unix-domain socket … ; the rest of the line stands, the adopted searches unedited.'
    sidecar: 'api-security: «no TCP or UDP port» now reads as none served, and blitz-net''s two test-only loopback binds are registered as test-only, no crossing.'
    rationale: 'report.md:28 measures two new test-only TCP binds and says session rule 2026-10-15 owes their registration as test-only in the security plan; report.md:47 rules this line changed by a word saying the two tests bind, its searches no-change.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:47
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 8
    section: '## API Security'
    change: 'In the MCP sentence of line 8, «so the session socket stays the one listener» reads «so the session socket stays the one served listener».'
    sidecar: 'api-security: the MCP sentence''s «the one listener» reads the one served listener, beside the two test-only binds.'
    rationale: 'The second occurrence on this line of the unqualified claim the bind registration narrows; report.md:28 gives two TCP binds in blitz-net''s test module, served by no binary.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 32
    section: '## API Security'
    change: 'After line 32 add a control record, three lines in the file''s shape: `- **Control:** Failed loads` | `  - **Configuration:** Since 2026-11-01-upstream-sync-ui-graph a failed fetch reaches its handler: blitz-net''s error arm calls `NetHandler::error`, new on the public trait with a default that does nothing, and a stylesheet''s handler responds `Err(error)`, so a failed, empty or non-CSS stylesheet stops blocking render, and a stylesheet `href` that does not resolve starts no load; blitz-test-harness''s `CountedHandler` forwards the call, so a failed stylesheet no longer reads `NotSettled(Loads)` for good and a failed load ends the driver''s wait where it would otherwise have run to `LOAD_WAIT`, `settle_waiting` unedited; the error text stops in `ResourceLoadResponse` and the `Resource load failed` log line, `url` and `error` in the sink''s scrub set, and no carrier takes it (packages/blitz-net/src/lib.rs:313; packages/blitz-traits/src/net.rs:36-38; packages/blitz-dom/src/net.rs:144-146; packages/blitz-dom/src/mutator.rs:1392-1394; packages/blitz-test-harness/src/settle.rs:121-125)` | `  - **Stack reference:** blitz-net · blitz-traits · blitz-dom · blitz-test-harness`'
    sidecar: 'api-security: new control record Failed loads, a failed load reaching its handler and ending the wait.'
    rationale: 'report.md:14 and report.md:25 bring the behaviour, report.md:31 where the error text stops; this file''s Settle and loads in flight record describes only `LOAD_WAIT` ending a wait, and report.md:25 says the harness and driver contracts owe the sentence. A record of its own since line 30 already runs to the split at line 31.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/api-security.md
    near: 30
    section: '## API Security'
    change: 'In line 30, «`LOAD_WAIT` ending every wait» reads «`LOAD_WAIT` bounding every wait».'
    sidecar: 'api-security: `LOAD_WAIT` bounds every wait, a failed load now ending one earlier.'
    rationale: 'report.md:25, a failed load now ends the wait where it would otherwise have run to `LOAD_WAIT`, so `LOAD_WAIT` is the bound and no longer the one thing ending a wait on a load that fails.'
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:25
    dependent-of: D-new-detail
note: 'No D-security-input, D-security-auth, D-security-deps or D-retired hit in this file; the body''s §API Security broad line repeats «no TCP or UDP port» and its §Error Handling still reads that the handler is not called on a failed fetch, each outside this file and proposed by no one here.'
