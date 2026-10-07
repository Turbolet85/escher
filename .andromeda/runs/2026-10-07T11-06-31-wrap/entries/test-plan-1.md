## 2026-10-07-settle-detection — settle beside pump: the harness's first unit tests, `stand_settle`, re-counts
**Section:** §1 Coverage scope · §2 Harness-driven tests, Directory pattern · §3 blitz-test-harness (Construction, Core, Pump semantics), Crate-local test helpers, Agent-run Proof · §3 → Session lifecycle · §4 What unit tests cover · §5 Harness ↔ event pipeline, Session ↔ held instance · §7 Builders and options · §8 Hand-written fakes and stubs · §9 Local baseline
**Change:**
- §3 Core and Pump semantics state `settle`: pump's pass repeated with poll's answer kept; quiet = poll false, hover node unchanged, no load finished; a load in flight is `NotSettled { busy: Loads }` at once; `Render`, `Loads` or `Layout` after `SETTLE_PASS_LIMIT` (64) passes; no clock read, no time advanced, no load waited on, the changed set neither read nor drained; pump and every input helper unchanged.
- §3 → Session lifecycle: `session-start` gains `act(step) -> Result<Settled, SessionError>` beside `harness()` / `harness_mut()` — not rolled back, in process, nothing of it on the socket; `session-proof` gains `stand_settle` 8 and the fork's CI run on the chunk's pre-CI commit.
- §1 and §4: blitz-test-harness holds its first unit tests, 11 in `settle.rs` (4 counter · 6 loop · 1 messages); escher-driver stays at 13, its message pin now covering `NotSettled`'s three classes; tests/blitz-tests gains `stand_settle` (8, both layout modes).
- §5: a step through `Session::act` returns settled, or `NotSettled(Busy::Render)` at the bound with the instance still readable.
- §7: a fetching `net_provider` is wrapped in the load counter settle reads; a no-op or absent one and a wrapped document carry none.
- §8 Time: was "applied on the next pump"; now applied on the next pass — a pump, an input helper's own pump, or a settle — so a delivery made after an input helper waits for a later pass. §8 Network names `stand_settle`'s file-private `ManualNetProvider`; a new row names the unit tests' scripted `Document` and held-handler `NetProvider`.
- `mod session_common;` readers: four `stand_session_*.rs` checks → five checks, `stand_settle` the fifth (§2, §3 helpers).
- Counts: `run stand` 72 → passed 80 · failed 0 · ignored 3 over 20 files; workspace 142 · 579 · 0 · 8 → 143 result lines · 598 passed · 0 failed · 8 ignored.
- Citations into the harness's and the driver's edited files re-pointed.
**Why:** the chunk added the settled step and its checks. Traps: a delivered tick needs a pass of the check's own only when it follows the input helper — the plan predicted the Timer step green before the fix and it measured red in that order; the changed set is non-empty on a freshly booted harness, so a check reading the flag drains first.
**Kept:** `has_changes` false on a fresh blitz-dom document (§1) — a bare document, not a booted harness.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/
