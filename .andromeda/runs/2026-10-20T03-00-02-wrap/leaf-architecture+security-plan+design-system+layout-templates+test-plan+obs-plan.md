group: architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «no driver command waits on a load»
  source: .andromeda/security-plan.md:222 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none»
  replace-with: «an acting driver command waits on a load in flight for at most `LOAD_WAIT` (5 s of wall time), `snapshot` and `screenshot` waiting on none»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «`Session::act` runs one step on the held instance and returns once it has settled;»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5 «the harness is settled, a load in flight waited for up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving on the held instance) since 2026-10-20-load-wait»
  replace-with: «`Session::act` runs one step on the held instance and returns once it has settled, a load in flight waited for up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving on the held instance) and one still loading at the bound `NotSettled(Loads)`;»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «`Session::act` runs one step on it and returns once it has settled (in process; `NotSettled` naming the busy class when it does not go quiet)»
  source: .andromeda/architecture.md:199 «`LOAD_WAIT: Duration`, 5 s of wall time, re-exported at the crate root, is how long an acting step waits for a load in flight before it returns with the load still outstanding.»
  replace-with: «`Session::act` runs one step on it and returns once it has settled (in process; a load in flight waited for up to `LOAD_WAIT`, 5 s of wall time, with no time moving — the five acting verbs waiting so on every carrier, `snapshot` and `screenshot` on nothing; `NotSettled` naming the busy class when it does not go quiet)»

- leaf: CLAUDE.md:83   class: omission
  leaf-text: «`stand_replay.rs` and `stand_settle.rs` (shared»
  source: .andromeda/test-plan.md:185 «since 2026-10-20-load-wait an acting step that meets a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time — `stand_load_wait` 2 through `Session::run`»
  replace-with: «`stand_replay.rs`, `stand_settle.rs` and the load wait's `stand_load_wait.rs` (shared»
