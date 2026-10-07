# Codebase Research — 2026-10-07-command-and-refusal-schema

## Scope
- **Depth:** moderate · **Reads:** 17 · **Globs/Greps:** 11 · **Graph queries:** 4 (trail `tree-query-2026-10-07-command-and-refusal-schema.json` in the run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its one Session Addition included; none applied — no live leg in this chunk (its proof is unit tests in `escher-driver` and the local gate legs).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** none — every fact this chunk turns on lives in this repository.

## Files inspected
- `packages/escher-driver/src/lib.rs` (full) — five private modules, re-exports at the crate root, `#![deny(missing_docs)]` (`:21`), the crate doc states "this crate names no app" (`:4`) and what crosses the socket (`:13-16`). The schema's modules and re-exports land here.
- `packages/escher-driver/src/error.rs` (full) — `SessionError`, twelve variants (`:12-37`), every message a fixed string (`:10`); the message pin lists its variants by hand (`:78-96`), so a new variant is covered only by an edit of that list.
- `packages/escher-driver/src/session.rs` (full) — `Session::start` checks the label before `boot` runs (`:39-41`); `Session::act` runs the step, then `Harness::settle`, and does not roll back (`:70-78`).
- `packages/escher-driver/src/wire.rs` (full) — the closed lifecycle grammar: two words, `refused version` / `refused malformed`, a 64-byte request and a 128-byte reply bound (`:16-20`, `:61-81`); the shape of "each admitted form round-trips, a table of forms outside the grammar is each refused" (`:151-230`).
- `packages/escher-driver/src/host.rs` (full) — `serve` binds, then `answer_until_stop(&listener, session.label())` (`:48`); the loop (`:107-123`) blocks in `accept` and never touches the held session, which is dropped after it (`:50`).
- `packages/escher-driver/src/client.rs` (1-70) — `attach` · `stop` · `start` and their `Hello` / `Started` results: the lifecycle's Rust shapes; `start` takes a `Command` the caller configured and a `Duration`.
- `packages/escher-driver/Cargo.toml` (full) — two dependencies, no features.
- `examples/seven_guis/src/session_host.rs` (full) — closed argv of two (`:24-32`); the tick handle bound at `:35`, stored at `:39`, dropped at `:45`; three `eprintln!` lines (`:21`, `:30`, `:50`).
- `examples/seven_guis/src/tasks/timer.rs` (1-60) — `TimerTicks::deliver(n)` queues ticks and wakes the timer (`:20-26`); each tick applied adds 0.1 s (`:57-60`).
- `packages/blitz-test-harness/src/settle.rs` (1-190) — `settle` takes `self.time()` once (`:152`), reads no other clock, and returns `NotSettled { busy: Busy::Loads }` at once for a load in flight (`:164-171`); `Busy` names a class only (`:19-28`).
- `packages/blitz-test-harness/src/harness.rs` (139-160) — the harness's own clock: `tick(dt_seconds)` adds to `time` and pumps (`:152-155`).
- `packages/blitz-test-harness/src/input.rs` (full) — what input exists to state shapes over: `click` by selector or point, `type_text` into the focused element, `press` / `press_with` a `Key` with `Modifiers`, `wheel_at`, `drag`, `tap`, `ime`. None takes a stable element id; none settles.
- `packages/dioxus-native-dom/src/actionable.rs` (1-70) — `UnkeyedActionable::remedy()` formats the element's tag and id into its text (`:38-43`).
- `packages/dioxus-native-dom/src/snapshot.rs` (1-90) — `Snapshot`, `SnapshotNode { id, role, name, state, bounds, children }`, `NodeState { enabled, checked, value, focused }`, `MASKED_VALUE`.
- `scripts/cold_agent_stub.py` (index by grep) — a toy counter, three tools; an unknown tool is refused `not-found` (`:103`), a non-object arguments value `malformed` (`:105`), a display pressed `not-pressable` (`:82`).
- `.github/scripts/test_cold_agent.py` (163-171) — the pin on the stub's four cause names, nine rows.
- `escher-0.1.0/working-route.md` (lines 62 · 64 · 71 · 73 · 75 · 77, whole) — the neighbours' own text and freight (see Patterns detected, "Decisions the route reserves").

## Graph impact (from the code-graph query; plane `rust`)
- **`escher-driver` crate edges** — out: `dioxus-native-dom`, `blitz-test-harness`; in: `seven_guis`, `blitz-tests`. An additive public item in the crate has two consumer crates and no other blast radius.
- **`escher-driver` external surface** — 19 public items reached from outside `packages/escher-driver/src/`: `Session` 30 sites in 8 files, `harness_mut` 26 in 6, `harness` 25 in 5, `stop` 12 in 4, `attach` 11 in 4, `SessionError` 9 in 2, `act` 7 in 1. This chunk changes the signature of none.
- **`SessionError` outside the crate** — `tests/blitz-tests/tests/stand_session_lifecycle.rs:14` · `:44` · `:45` · `:63` · `:76` · `:86` · `:87` and `tests/blitz-tests/tests/stand_settle.rs:17` · `:331`; every site compares against one named variant — no exhaustive `match` (tree sweep `SessionError::` over `packages examples tests apps`, outside the crate's `src/`: 7 hits · 0 changed · 7 no-change (equality on a named variant)).
- **`Session::act` outside the crate** — `stand_settle.rs:51` · `:58` · `:66` · `:94` · `:116` · `:173` · `:329`; unchanged here.
- **Names the schema would mint** — `Refusal`, `Cause`, `Verb`, `Command`, `Remedy`, `Schema`, `Shape`, `RefusalCause`: 0 rows in `symbol` (existence query, `kind <> 'meta'`) — all free in the workspace.

## Patterns detected
- **Refuse before the work, change nothing** (`packages/escher-driver/src/session.rs:39-41`): the label is checked and `InvalidLabel` returned with `boot` not called; pinned by a test whose boot closure panics (`session.rs:93-102`).
- **Closed grammar with a refusal table** (`packages/escher-driver/src/wire.rs:61-81`, tests `:151-230`): admitted forms round-trip; fourteen forms outside the grammar are each refused under one named reply.
- **Class-only outcome with one fixed message per class** (`packages/blitz-test-harness/src/settle.rs:19-28`, `:48-56`; `packages/escher-driver/src/error.rs:39-68`).
- **Two remedy precedents that differ** — `SessionError`'s messages are fixed strings (`error.rs:10`); `UnkeyedActionable::remedy()` interpolates the tag and the id (`actionable.rs:38-43`). Only the first is a fixed-text precedent.
- **A validation that takes no session cannot change one.** `Session::act` does not roll back (`session.rs:66-67`), so "a refused call changes nothing" holds only when the call is decided before any step runs. A function from a call to `Result<command, refusal>` that receives no `Session` holds it by construction; a held-instance check of it would assert nothing the signature does not already state.
- **The host cannot reach the tick handle while serving.** `serve` consumes the session and blocks in `accept` (`host.rs:48`, `:107-123`); the handle lives in `main`'s frame on the same thread (`session_host.rs:35-45`). A host that delivers ticks needs a change inside `escher-driver::serve` or a hook the caller hands the session — the binary alone cannot do it.
- **Two clocks, both the caller's.** The Timer's time moves by `TimerTicks::deliver` (`timer.rs:20-26`); animation time moves by `Harness::tick` (`harness.rs:152-155`), which `settle` reads and never advances (`settle.rs:145-148`, `:152`).
- **Serialization crates.** `serde = "1"` is a workspace dependency (`Cargo.toml:165`), consumed by `blitz-traits`; `serde_json` is in no `[workspace.dependencies]` line — only `wpt/runner/Cargo.toml:50` names it — and resolves in `Cargo.lock` at 1.0.151. A schema stated as `'static` Rust data (tables of names, kinds and help texts) is readable as data with no serialization crate; producing JSON from it is the surface's step.
- **Decisions the route reserves for later promotions** (working-route.md, read whole): whether a command exposes the actionable-key check (line 71, "decided at this entry's promotion"); how JSON stdout carries the snapshot text — "a string field, or a JSON view of the same model" (line 71); the orphaned-host answer, "an idle expiry, or a list-and-stop command" (line 71); the `accessibility` feature on `escher-driver` — "the entry that first reads the snapshot through a session decides" (line 62); typed deletion that works on every platform and the absent `select` / range interaction (line 62); a screenshot's form and the password painted in the clear (line 77).
- **The stub's vocabulary is a toy model's, pinned and ratified.** Three of its four names have a counterpart in the schema's set (`not-found`, `disabled`, `malformed`); `not-pressable` has none, and `stale`, covered and off-screen have none in the stub. It refuses an unknown tool as `not-found`.

## Conventions to follow
- **Private module, root re-export, every public item documented**: `packages/escher-driver/src/lib.rs:21-34`.
- **Unit tests inline at the end of the source file**, built-in harness, tables stated in the test: `packages/escher-driver/src/wire.rs:143-297`, `error.rs:72-120`.
- **No print, log, env read or `tracing` in the crate**: grep `println!|eprintln!|dbg!|tracing|log::|std::env|env::var` over `packages/escher-driver/src/*.rs` — 0 hits.
- **Platform gates only on the socket**: `cfg(unix)` stands at `client.rs:66` · `:180`, `host.rs:24` · `:149` and `lib.rs:27` alone (grep `cfg\(|cfg_attr` over the crate's `src/`); `session.rs` and `error.rs` compile everywhere, and so must the schema.
- **MSRV 1.91** is built only by the fork's CI; the dev host compiles on a newer stable — the schema uses no std API newer than 1.91.
- **Cause names in the stub's spelling where they overlap**: lowercase, hyphenated (`not-found`, `disabled`, `malformed` — `scripts/cold_agent_stub.py:6`).

## New files to create
- `packages/escher-driver/src/schema.rs` — the verb table: each verb's name, help text, argument specs and result shape, as `'static` data
- `packages/escher-driver/src/refusal.rs` — the closed cause set with each cause's name and remedy, and the refusal value
- `packages/escher-driver/src/command.rs` — a call as data, its validation against the schema into a typed command or a refusal

## Files to modify
- `packages/escher-driver/src/lib.rs` — module declarations, root re-exports and the crate doc

## Open questions
- How does an agent move the Timer's time through a driver session — a verb whose step the session's caller supplies, a host behaviour, or not in this version? → blocks: plan-decision (the operator's; the CARRY's first question)
- May a driver command wait on a load with a clock? → blocks: plan-decision (the operator's; a new security-plan §API Security decision)
- Does the verb set hold the whole 0.1.0 vocabulary now, or the verbs whose shapes rest on what is built, in one table the later entries extend? → blocks: plan-decision (six shape decisions are reserved by later entries' freight)
