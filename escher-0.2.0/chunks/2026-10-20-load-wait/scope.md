# Scope — 2026-10-20-load-wait · Load wait

**Working entry (escher-0.2.0/working-route.md:40, verbatim):** Load wait — a command meeting a load in flight waits
it out within a stated bound, on a fixture (v020-07; per security-plan §API Security)

**Origin:** intent §6 «Ready for a real app» (OBSERVED: «No command waits out a network load (residual,
2026-10-07-command-and-refusal-schema)»; EXPECT: «a command that meets a load in flight waits it out within a stated
bound») · requirements.md:17 (v020-07) · vision §What "0.2.0 done" means («A load in flight is waited out within a
stated bound … proven on fixtures»).

**Capability:** v020-07 (unclaimed). Its acceptance as the matrix holds it: «On a fixture whose load completes inside
the stated bound, an acting command returns settled with the loaded content on screen; on one whose load does not
complete, the command returns within the bound and says the load is still in flight.»

**Freight:** none. `route.py pins` lists no block on line 40.

## What it builds
- **An acting command waits out a load in flight.** Today `Session::act` runs its step, then `Harness::settle` once.
  A load in flight after a quiet pass reads `NotSettled { busy: Loads }` at once and is never waited on
  (packages/escher-driver/src/session.rs:196-204; packages/blitz-test-harness/src/settle.rs:141-170). The chunk makes
  an acting command that meets a load in flight keep settling while the load is outstanding. It returns `settled` once
  the load is answered and the screen goes quiet. If the bound passes first, it returns with the load still named.
- **A stated bound.** The bound is fixed and said by the tool's own text: the schema's help and `help`'s
  self-description, among the fixed `'static` table text on the ratified carriers. It is stated in the architecture's
  Driver session contract and the security plan's «Settle and loads in flight» row. The bound is a constant,
  not a call argument: an argument is a new input on a validated surface. Its value, and whether it is ever
  caller-set, are P4's to settle. (Verified: research.md §Patterns detected, the `IDLE_EXPIRY` precedent.)
- **«Says the load is still in flight».** The existing result already says it: `settled: false` with
  `busy: "loads"`, the closed busy class `loads` (packages/escher-driver/src/schema.rs:34). So no new result field is
  needed, and the bound's passing reads as today's `loads` reading. (Verified: every acting verb settles in
  `settled_step`, execute.rs:453-470; research.md E2.)
- **Proven on a fixture,** in both layout modes, with the two arms of the acceptance:
  - a load answered inside the bound: the acting command returns settled, and the loaded content is on screen (in the
    returned diff and the next snapshot);
  - a load never answered: the command returns within the bound, `settled: false`, `busy: "loads"`, measured
    against the bound.

  The shared `HeldNet` (tests/blitz-tests/tests/fixtures/mod.rs:64-107) answers only from the check's own thread.
  That thread is blocked inside the command, so the arm that completes inside the bound needs a stand-in that answers
  on its own after a delay. [premise-corrected: the one-reader rule (fixtures/mod.rs module doc; test-plan §3) places
  a stand-in one check reads in that check's own file, so it is neither a shared stand-in nor `HeldNet` widened. The
  never-answered arm reads `fixtures::HeldNet` as it is.]

## Boundaries
- **Time.** A load is answered by the outside world on wall time, while every time a headless run reads comes from
  the session's `VirtualClock`, which only the driver moves (intent §5; arch Driver session; blitz-dom reads no
  `Instant::now()` outside `clock.rs`). The wait is measured in wall time and moves no virtual time: an
  animation or a timer does not run during a wait, and `advance` stays the one mover of the session's clock. Where
  the wall read lives (escher-driver or blitz-test-harness, never blitz-dom) is P4's. (Verified: research.md E3;
  settle reads the harness's time and never ticks, settle.rs:149-152.)
- **Which commands wait.** The acceptance names «an acting command» (`click` · `type` · `press` · `advance` ·
  `scroll`). `snapshot` and `screenshot` run no step and dispatch nothing, so they do not wait (verified:
  execute.rs:108-116). Whether a
  read with a load in flight should wait is a P4 question.
- **Recorded decisions this moves.** Each is a direction the chunk changes on the version's own requirement and is
  re-stated by the wrap:
  - «no driver command waits on a load» (CLAUDE.md, arch Driver session, security-plan §Input Validation);
  - the security plan's «Settle and loads in flight» row: settle «never waits on one: it waits on no clock and moves
    none … so the absent request timeout above cannot hang it»;
  - the schema's statement that no instance verb has an argument or a result field of a wait.

  The bounded wait keeps that row's guarantee in a new form: the absent request timeout still cannot hang
  a command, because the bound ends it (verified: research.md E2). The settle-detection entry's standing rule —
  «a driver command that would wait on a load with a clock is a new decision for this plan» (security-plan sidecar)
  — is that new decision, taken on the version's requirement (intent §6).
- **Harness settle stays as it is.** `Harness::settle` keeps reporting a load at once and reading no clock.
  The wait is a loop over it in the driver session. (Verified: settle's production callers are exactly
  `settled_step` and `Session::act`, research.md §Graph impact.)
- **Telemetry.** The `command` span keeps its eight fields of fixed words and counts. The time a command
  waited, or that it waited at all, is a new field only by a ruling (obs-plan §4; security-plan §Input Validation,
  the span rule). P4 settles whether one is wanted.
- **No crossing added.** No new verb, `Command`, carrier, listener, env read or exit. The fixture's stand-in is in
  process, served by no binary. A delayed answerer opens no socket and reads no env. The socket's 30 s read bound on
  a client's call answer (security-plan §API Security, Local session socket) and MCP's per-reply handling must stay
  above the stated bound. [premise-corrected: the host's 2 s `IO_BOUND` covers reading the request line and writing
  the reply only, not a call's run (host.rs:179-211). The binding bounds are the client's 30 s `ANSWER_BOUND`
  (wire.rs:18), the `Speaker`'s 30 s `REPLY_BOUND` (examples/seven_guis/tests/common/mod.rs:200) and the cold-agent
  replay's 30 s `REPLY_BOUND` (scripts/cold-agent.sh:62), research.md §Files inspected.]
- **No real network.** The fixture is offline: the stand's options, the held stand-in. `blitz-net` is not reached,
  and no agent- or user-supplied URL is routed through it (`.claude/rules/security.md`).

## Surfaces and contracts touched
- `packages/escher-driver/src/session.rs` (`Session::act`) · `execute.rs` (the acting verbs) · `schema.rs` /
  `describe.rs` (the stated bound in help text) · possibly `packages/blitz-test-harness/src/settle.rs` (read-only
  unless P4 places the wait there).
- `tests/blitz-tests/tests/fixtures/mod.rs` (a stand-in that answers on its own) and a new check beside
  `stand_settle.rs`.
- arch §Standard Contracts (Driver session; Harness settle) · security-plan §API Security («Settle and loads in
  flight») and §Input Validation · test-plan §3/§8 (network stand-in rows) · obs-plan §4 (the span, if a field is
  ruled). CLAUDE.md's Modules and Architecture lines follow by the wrap's cascade.

## CI read at the take-up (Setup 5a)
- `165cfe443f46` (the last wrap's commit, 2026-10-19-visual-lint, its record `gated`): CI#39354473207, verdict not
  yet available at P1 (in progress, 4/4 checks so far). Re-read at P5: green, 17/17 checks, wall 1015 s — nothing to
  fold.
- `5a9b806de224` (2026-10-19-visual-lint's operator pre-CI commit): CI#39348671810 green, 17/17 checks, wall 963 s.
- `e97a7f41b10a` (2026-10-19-shared-fixtures' wrap, the last flip to `complete`, Setup 5a's base): CI#39343307796
  green, 17/17 checks, wall 926 s.
