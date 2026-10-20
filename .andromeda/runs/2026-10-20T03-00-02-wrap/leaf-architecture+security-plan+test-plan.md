group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:12   class: omission
  leaf-text: «the file modes and the host's idle clock.»
  source: .andromeda/architecture.md:199 «It reads `std::time::Instant` and sleeps, and nothing else: no `tick`, `advance_time` or time step, so no time moves on the held instance while it waits»
  replace-with: «the file modes, the host's idle clock and, since 2026-10-20-load-wait, the session's load wait (`std::time::Instant` and a sleep).»

- leaf: .claude/docs/services/escher-driver.md:12   class: stale
  leaf-text: «The executor, the session, the schema, the validation, the JSON writer and, since 2026-10-12-headless-screenshot, the private `image` encoder read no clock or environment and print nothing; the host reads one for its idle expiry, and the MCP server one when a request arrives with a session held.»
  source: .andromeda/architecture.md:192 «print nothing and read no environment, and none of them reads a clock but the session's load wait (below)»
  replace-with: «The executor, the session, the schema, the validation, the JSON writer and, since 2026-10-12-headless-screenshot, the private `image` encoder read no environment and print nothing, and none of them reads a clock but the session's load wait (since 2026-10-20-load-wait, `session.rs` the one clock reader among them); the host reads one for its idle expiry, and the MCP server one when a request arrives with a session held.»

- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «then settles it (`Harness::settle`): on `Ok` everything the step made due is on the screen; an instance that does not go quiet is `NotSettled(Busy)`. The step is not rolled back, and no time is advanced.»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5 «a load in flight waited for up to `LOAD_WAIT` (5 s of wall time, settled again every 10 ms, no time moving on the held instance) since 2026-10-20-load-wait»
  replace-with: «then settles it (`Harness::settle`) and, since 2026-10-20-load-wait, through the load wait (crate-private `settle_waiting`): while the answer is `NotSettled { busy: Busy::Loads }` it settles again every `LOAD_POLL` (10 ms, private) until the load is answered or `LOAD_WAIT` (5 s of wall time, re-exported at the crate root) has passed, its last sleep the time left, then one more settle, while a `render` or `layout` answer returns at once; on `Ok` everything the step made due is on the screen; an instance that does not go quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`. The step is not rolled back, and no time is advanced — the wait reads `std::time::Instant` and sleeps, with no `tick`, `advance_time` or time step, no changed-set drain, no print and no `tracing` call.»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «Each acting verb is one settled step: snapshot, step, `Harness::settle`, snapshot, diff.»
  source: .andromeda/architecture.md:199 «wait on every carrier — in process, the socket, the command line and MCP — and `snapshot` and `screenshot`, which run no step, wait on nothing»
  replace-with: «Each acting verb is one settled step: snapshot, step, the load wait over `Harness::settle`, snapshot, diff — so since 2026-10-20-load-wait the five acting verbs wait on a load in flight, for at most `LOAD_WAIT`, on every carrier (in process, the socket, the command line and MCP), while `snapshot` and `screenshot`, which run no step, wait on nothing.»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «The acting verbs' result: `settled`, `busy` (one of `BUSY_CLASSES`, only when not settled)»
  source: .andromeda/architecture.md:199 «The `settled` field's help states the bound («a load in flight waited for up to 5 s»)»
  replace-with: «The acting verbs' result: `settled` (its help stating the load wait's bound, «a load in flight waited for up to 5 s», since 2026-10-20-load-wait), `busy` (one of `BUSY_CLASSES`, only when not settled)»

- leaf: .claude/docs/services/escher-driver.md:38   class: stale
  leaf-text: «Settling moves no time and waits on no load: time on a held instance moves only by `advance`, which moves the session's clock on every session and, through the caller's step where one was handed, the app's own time (ratified by the founder, 2026-10-18); no driver command waits on a load (the operator, 2026-10-07).»
  source: .andromeda/security-plan.md:222 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none»
  replace-with: «Settling moves no time: time on a held instance moves only by `advance`, which moves the session's clock on every session and, through the caller's step where one was handed, the app's own time (ratified by the founder, 2026-10-18); since 2026-10-20-load-wait an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none — so a kept check whose app meets a load can read `settled` differently on a replay whose load lands at another time (recorded, not established); the two committed Counter checks meet no load.»

- leaf: .claude/docs/services/escher-driver.md:49   class: omission
  leaf-text: «`settled`, `busy` (the schema's busy-class word), `passes`, and»
  source: .andromeda/architecture.md:199 «and on a waited step the `command` span's `passes` is the passes of the settle that went quiet»
  replace-with: «`settled`, `busy` (the schema's busy-class word), `passes` (on a step the load wait settled again, the passes of the settle that went quiet), and»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «94 unit tests in sixteen files (`wire.rs` 12 · `session.rs` 6 · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 6»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»
  replace-with: «99 unit tests in sixteen files (`wire.rs` 12 · `session.rs` 10, 6 until 2026-10-20-load-wait added the load wait's four over a scripted settle — a load answered inside the bound ends the wait settled, one never answered ends it at the bound, work other than a load returns at once, a quiet settle returns after one settle · `error.rs` 5 · `refusal.rs` 3 · `schema_tests.rs` 7, 6 until 2026-10-20-load-wait added that the `settled` help states `LOAD_WAIT` in whole seconds, that the five acting verbs carry that field and that the wait ends before an answer read»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «92 in fifteen until 2026-10-19-visual-lint»
  source: .andromeda/test-plan.md:22 «(94 in sixteen, 60 inline and 34 in four `#[path]` files, until 2026-10-20-load-wait, 92 in fifteen until 2026-10-19-visual-lint»
  replace-with: «94 in sixteen until 2026-10-20-load-wait, 92 in fifteen until 2026-10-19-visual-lint»

- leaf: .claude/docs/services/escher-driver.md:60   class: stale
  leaf-text: «60 tests inline and 34 in those four files»
  source: .andromeda/test-plan.md:22 «- **escher-driver:** 99 unit tests in sixteen files, 64 inline and 35 in four `#[path]` test files»
  replace-with: «64 tests inline and 35 in those four files (60 and 34 until 2026-10-20-load-wait)»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «`stand_session_quiet`'s host installs no log sink»
  source: .andromeda/test-plan.md:27 «`stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a body stylesheet»
  replace-with: «Since 2026-10-20-load-wait `stand_load_wait` (2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a body stylesheet: a load its in-file `LateNet` answers after 200 ms ends the `click` settled with its content on screen, in under `LOAD_WAIT`; a load `HeldNet` never answers ends it at the bound, `settled` false with `busy` `Loads`; `harness().time()` unmoved in both; picked up by `run stand` and the stand-repeat leg by its prefix). `stand_session_quiet`'s host installs no log sink»
