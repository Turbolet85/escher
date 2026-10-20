# P5 review — 2026-10-20-load-wait · Load wait

**Plan:** `escher-0.2.0/chunks/2026-10-20-load-wait/plan.md` (399 lines, 262 of them prose · 20 gate entries ·
10 acceptance criteria) · **Research:** `escher-0.2.0/chunks/2026-10-20-load-wait/research.md` · **Scope:**
`escher-0.2.0/chunks/2026-10-20-load-wait/scope.md`

## Goal
An acting driver command (`click` · `type` · `press` · `advance` · `scroll`) that meets a load in flight now waits
it out, on every carrier, for up to **`LOAD_WAIT` = 5 s**. 5 s is your answer at P4.
- If the load lands first, the command returns `settled: true`.
- If the bound passes first, it returns today's `settled: false, busy: "loads"`.
- The wait moves no virtual time, and `Harness::settle` itself is untouched.
- Nothing new crosses: no verb, argument, field, cause, exit, span field, env read or dependency. The bound is
  stated in the `settled` field's help, which is the program's own fixed text.

## Top acceptance criteria
- **v020-07, proven on an in-file fixture held in a session in both layout modes:**
  - a body stylesheet answered after 200 ms by a stand-in on its own thread → `settled: true`, with `fx-box`
    changed (`bounds`) in the diff and the next snapshot;
  - the request never answered (`HeldNet`) → `settled: false`, `busy: "loads"`, at ≥ 5 s and < 7 s.
- The harness's `time()` is the same before and after each waited call.
- Nothing else moves:
  - `Harness::settle`, blitz-dom, every manifest and the lock are unchanged;
  - the only new wall read is in `session.rs`;
  - escher-driver still has 2 span sites and 0 events;
  - the new check is silent.
- `agent-run run stand` is predicted to move from 183 · 0 · 7 to 185 · 0 · 7. The CI run on the pushed sha must be
  green, with stand-repeat at three rounds and the coverage floor held.

## Touchpoints
- `packages/escher-driver/src/session.rs`: `LOAD_WAIT`, the waiting settle and its 4 unit tests, and
  `Session::act` through it.
- `packages/escher-driver/src/execute.rs`: `settled_step` through it, with its doc moved.
- `packages/escher-driver/src/schema.rs` and `packages/escher-driver/src/schema_tests.rs`: the help text stating
  5 s, pinned against the constant and kept under the 30 s `ANSWER_BOUND`.
- `packages/escher-driver/src/lib.rs`: re-exports `LOAD_WAIT` (55 → 56).
- New: `tests/blitz-tests/tests/stand_load_wait.rs`.

## Test commands (see the fence)
- Unit: escher-driver, then the new check.
- 8 guard probes, each baselined and controlled in a scratch clone (`.andromeda/runs/2026-10-20T01-41-31-phase/p5-controls.md`).
- `agent-run boot` / `run stand`, then the `a11y`, `fast` and `doc` legs.
- Operator: hygiene, then push, then the CI read, then the per-OS tallies (control fired on CI#39348671810).

## Decisions taken at P4
- **5 s:** your answer to the one technical fork.
- **No founder fork:** no option widens a boundary, discharges a PROVISIONAL mark or comes by upstream merge
  (playbook read before the round). So there is no `founder-forks.md`.
- **Decisive leans:**
  - the bound is a constant, not an argument;
  - no span field;
  - `passes` counts the settle that went quiet;
  - `Session::act` waits too;
  - the fixture loads a stylesheet, not an image.

## Concretization preview — v020-07 (the one cap this chunk claims)
- **Current acceptance:** «On a fixture whose load completes inside the stated bound, an acting command returns
  settled with the loaded content on screen; on one whose load does not complete, the command returns within the
  bound and says the load is still in flight.»
- **Proposed concrete form:**
  - The fixture is `stand_load_wait`'s in-file page, held in a driver session in both layout modes, driven by
    `Session::run`.
  - **Answered arm.** A `click` on `fx-load` adds a body stylesheet link. Its `.test` request is answered on the
    stand-in's own thread 200 ms later. The click returns `settled: true` with no `busy`. Its diff names `fx-box`
    changed with `bounds` in `differs`, and the next snapshot reads `fx-box` at the sheet's height. The call takes
    less than `LOAD_WAIT`.
  - **Never-answered arm.** The same click with a `HeldNet` that never answers returns `settled: false`,
    `busy: "loads"`, no earlier than `LOAD_WAIT` and less than 2 s after it.
  - **The bound.** `LOAD_WAIT` is 5 s, the value the `settled` field's help states, pinned by a unit test.
  - **Gate:** `cargo test -p blitz-tests --locked --test stand_load_wait 2>&1` reads 2 passed.
- **Never-weaken read:** the outcome stays the same (wait, then settled or still-in-flight). The concrete form adds a
  fixed 5 s, the 2 s overrun slack, and the witness of the loaded content (the diff's `bounds`).

## Beyond the plan
none
