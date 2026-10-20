# tests extract

## Relevance
relevant — the chunk changes what an acting driver command returns when a load is in flight. Its proof is a new held-session fixture check beside `stand_settle`, with a network stand-in that answers on its own (test-plan §3, §5, §8, §10).

## Constraints
- Both layout modes: test-plan §5 Integration Test Strategy (Session ↔ held instance) requires every held-session check, `stand_settle` included, to run in both layout modes. Each arm of the load-wait check must too: answered inside the bound, and never answered (per test-plan §5 Integration Test Strategy).
- Wall time in session checks: test-plan §5 Integration Test Strategy requires the held-session checks to read no sleep and no wall clock. Its one recorded exception is `stand_virtual_clock`'s single 800 ms sleep. A check that measures "returns within the bound" against wall time, or a stand-in that answers after a wall delay, is a second exception. It must be stated as narrowly as that one: which check, which read, and why (per test-plan §5 Integration Test Strategy).
- Harness time and settle: test-plan §3 Test Harness Contract (Core) requires that `settle` never moves the harness's `VirtualClock`, a rule ratified by the founder on 2026-10-18. §3 Test Harness Contract (Pump semantics) requires `settle` to read no clock but the harness's, advance no time and wait on no load. A wait loop placed over `settle` must leave both readings true. Whether the harness's own contract changes is P4's, and the settle unit tests pin its current form ("a request in flight after a quiet pass reads `Loads` after one poll") (per test-plan §3 Test Harness Contract).
- Network stand-in: test-plan §8 Mocking & Stubbing Discipline makes `HeldNet` escher's one held network stand-in. It does not override `is_noop`, so the harness counts its loads, and it answers `.test` URLs with no real fetch. A stand-in that answers on its own after a delay must keep both properties: loads counted, and no real fetch (per test-plan §8 Mocking & Stubbing Discipline).
- The fixtures module: test-plan §3 Test Harness Contract (blitz-tests (fixtures)) requires `tests/fixtures/mod.rs` to hold no print, `tracing` call, env read, file, socket, listener or `blitz_net`. It also requires that a page with one reader stays in its own check file. A delayed answerer added there inherits all of these (per test-plan §3 Test Harness Contract).
- Flakiness budget: test-plan §10 Quality Gates & Coverage Targets puts every `stand_*.rs` file in the `stand-repeat` leg. That leg runs three rounds with no retry under a zero-failure budget. Two consequences follow:
  - the bound assertions (inside the bound, past the bound) must hold on a loaded CI runner;
  - the never-answered arm costs the full bound for each mode in each round, which weighs on the bound's value (per test-plan §10 Quality Gates & Coverage Targets).
- Coverage floor: test-plan §10 Quality Gates & Coverage Targets holds `packages/escher-driver` at a 97 % line floor. It last read 97.93 %. The wait loop and its bound path in escher-driver need tests that reach them, or the floor falls (per test-plan §10 Quality Gates & Coverage Targets).

## Patterns to follow
- The new check is a `tests/blitz-tests/tests/stand_*.rs` file that declares `mod session_common;` and `mod fixtures;`, as `stand_settle` does. `run stand` then picks it up by its prefix, with no script change (per test-plan §3 Test Harness Contract — blitz-tests (session checks), blitz-tests (fixtures), ``§3 `run` ``).
- Boot the fixture through `fixtures::boot(app, incremental)` / `fixtures::hold(label, app, incremental)` with the stand's pinned options. Make driver calls through `Session::run`, using the call builders in `tests/session_common/mod.rs` (per test-plan §3 Test Harness Contract — blitz-tests (fixtures), blitz-tests (session checks)).
- Follow `stand_settle`'s load arm: a head stylesheet, then its `@import`, on `.test` URLs, read as `NotSettled(Loads)` until it is answered. This is the shape the answered-inside-the-bound and never-answered arms extend (per test-plan §1 Test Scope Summary, tests/blitz-tests; test-plan §8 Mocking & Stubbing Discipline).
- The stated bound belongs in the schema's fixed help text. escher-driver's `describe.rs` unit tests require every text in `help`'s document to be its tables' and pin its count row `[11, 13, 12, 3, 8, 4, 15]`. A bound stated through the tables, rather than inline, keeps those tests true or moves them deliberately (per test-plan §4 Unit Test Strategy, escher-driver).

## Anti-patterns to avoid
- Advancing the session's `VirtualClock` during a wait, or calling a pump or tick in the check outside the driver. Settle and its checks move no virtual time; `advance` is the one mover (per test-plan §3 Test Harness Contract, Core; test-plan §5 Integration Test Strategy).
- Reaching a real network or `blitz_net` from the fixture. Keep the pages offline on `.test` URLs, answered by an in-process stand-in (per test-plan §8 Mocking & Stubbing Discipline; test-plan §3 Test Harness Contract, blitz-tests (fixtures)).
- An unrecorded sleep or wall-clock read in a session check, beyond the narrowly stated exception (per test-plan §5 Integration Test Strategy).

## Contract bindings
- tests ↔ obs (command span): escher-driver's unit tests pin the `command` span at exactly eight fields, none an argument's name, with each busy class reading as its schema word. `stand_act_spans` asserts the span lines on the stand. A wait-time field, if obs-plan §4 rules one, moves both. With no field ruled, they stay as they are (per test-plan §4 Unit Test Strategy, escher-driver; test-plan §1 Test Scope Summary, tests/blitz-tests).
- tests ↔ security/arch (session wire): ``§3 `session-wire` `` pins a call's answer read at 30 s per read (`wire::ANSWER_BOUND`), through the unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout`. The stated load bound must stay below it, or a hosted command that waits ends as `Timeout` (per test-plan §3 Test Harness Contract, `session-wire`).
- tests ↔ MCP carriers: seven_guis' `Speaker` (`mcp_server`, `mcp_log`) reads each reply under the 30 s `REPLY_BOUND`, and the cold-agent replay reads each reply under `REPLY_BOUND = 30` s. The stated bound must also stay below both (per test-plan §2 Test Strategy, Process-lifecycle checks; test-plan §3 Test Harness Contract, Cold-agent run pipe).
- tests ↔ harness contract: whether `Harness::settle`'s documented semantics, and its settle unit tests, change depends on where P4 places the wall read. With the wait in escher-driver they stay as they are (per test-plan §3 Test Harness Contract, Pump semantics; test-plan §4 Unit Test Strategy, blitz-test-harness).

## Acceptance criteria contributions
- A new `stand_*` held-session fixture check passes in both layout modes on both arms:
  - a load answered inside the bound returns `settled` true, with the loaded content in the returned diff and the next snapshot;
  - a load never answered returns `settled` false and `busy: "loads"`, within the bound, measured against it.

  `bash scripts/agent-run.sh run stand` reads it with 0 failed (per test-plan §5 Integration Test Strategy; ``§3 `run` ``).
- `bash .github/scripts/ci-leg.sh stand-repeat` reads 3 of 3 rounds green with the new check included, with no retry (per test-plan §10 Quality Gates & Coverage Targets).
- The `coverage` leg ends `coverage floor: 3 of 3 crates at or above their floors`, with escher-driver at or above 97 % (per test-plan §10 Quality Gates & Coverage Targets).
- escher-driver's unit tests stay green, or move deliberately with the stated bound:
  - the `describe.rs` count row and "every text is its tables'";
  - the eight-field `command` span;
  - `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` at 30 s, with the stated bound below it.

  (per test-plan §4 Unit Test Strategy, escher-driver; test-plan §3 Test Harness Contract, `session-wire`)
