# /implement outcome — 2026-10-18-one-virtual-clock

**Outcome: green.** The smoke was skipped because no boot path changed. The plan's three `operator` entries (15–17) are still to run.

Gate run: trail `.andromeda/runs/2026-10-18T07-55-10-implement/gate-2026-10-18-one-virtual-clock.json`. Summary line, verbatim: `entries 17 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 3`. It went green on the first firing, in one iteration:
- the census probe (entry 8) read `0`;
- `stand-repeat` ended `3 of 3 rounds green`;
- coverage read 3 of 3 crates at or above their floors;
- `fast`, `doc`, `a11y` and `audit` were green.

Nothing was deferred, and every changed-surface gate ran.

Matrix: `v020-06` moved from `planned` to `implemented`. Its ref is `tests/blitz-tests/tests/stand_virtual_clock.rs::a_shot_after_a_scroll_repeats_until_advance_moves_the_fade`, and the write's trail is in this run dir.

Smoke: `skipped — no boot-path change`.
- No binary or entry point was touched; `session_host.rs` is unchanged.
- The plan lists no `smoke` or `self-verify` entry, and the project has no headful self-verify.
- The `escher-session` binary itself ran as a process in P2, through `host_timer`, `cli_commands`, `mcp_server`, `host_log` and `mcp_log` inside entry 7.

## Files changed

**New (3):**
- `packages/blitz-dom/src/clock.rs`
- `tests/blitz-tests/tests/virtual_clock_engine.rs`
- `tests/blitz-tests/tests/stand_virtual_clock.rs`

**Modified (24):**
- blitz-dom: `lib.rs`, `config.rs`, `document.rs`, `resolve.rs`, `iframe.rs`, `events/pointer.rs`, `scrolling.rs`
- blitz-test-harness: `harness.rs`
- escher-driver: `execute.rs`, `session.rs`, `refusal.rs`, `schema.rs`, `schema_tests.rs`, `lib.rs`, plus two recorded companions, `json.rs` and `describe.rs`
- seven_guis: `src/stand.rs`, `tests/host_timer.rs`, `tests/cli_commands.rs`
- blitz-tests: `stand_act_timer.rs`, `stand_act_spans.rs`, `stand_mcp_tools.rs`, `stand_screenshot_repeat.rs`, `stand_non_lean_tasks.rs`

Scope read: `scope: clean — changed 27 · listed 25 · recorded 2 (companion 2 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 55`.

## Deviations from plan, with justifications

1. **Two out-of-list companion edits, in `scope-record.md`.**
   - The record's lines:
     - `` - `packages/escher-driver/src/json.rs` · companion · serves packages/escher-driver/src/refusal.rs · self ``
     - `` - `packages/escher-driver/src/describe.rs` · companion · serves packages/escher-driver/src/refusal.rs · self ``
   - Plan step 8 said the two files "need no change of their own". Their code does iterate `CAUSES`, but their unit tests pin the count: `json.rs`'s 14 refusals became 13, and `describe.rs`'s causes row of 9 became 8, with its comment "ninth cause" now "eighth".
2. **`VirtualClock` design (`clock.rs`).**
   - It holds an `Arc<AtomicU64>` of nanoseconds, moved by a `compare_exchange_weak` loop with saturating add.
   - `AtomicU64::fetch_update` is deprecated on the host toolchain (renamed `try_update`), and `try_update` is past the MSRV.
   - A unit test pins the saturation.
3. **`resolve_scroll_animation` (`scrolling.rs`).**
   - The clock is read once, before the `match`. The fling and smooth arms borrow `scroll_animation` mutably, so `self.clock_millis()` cannot be called inside them.
   - Both arms read the same value per frame.
   - Two `pub(crate)` helpers were added on `BaseDocument`, `elapsed_since(Duration)` and `clock_millis()`.
4. **`Harness::tick`.** A negative or non-finite `dt_seconds` now moves nothing (`Duration::try_from_secs_f64`, else zero), because the clock is monotonic and never runs backwards. `tick` has no caller in the tree, and its doc comment says so.
5. **`stand_act_timer`'s claims fixture test**, renamed `an_advance_reports_exactly_what_was_asked_whatever_the_step_does`.
   - The old assertion "settled with an empty diff" cannot hold now. `advance` moves the clock, which moves the fixture's infinite CSS animation (`fx-animated`'s bounds).
   - It now asserts that:
     - the step is handed exactly the milliseconds asked;
     - `advanced_ms` equals what was asked;
     - the clock moved by exactly that;
     - nothing was added or removed;
     - `fx-press` reads changed exactly when the step pressed it.
   - Rows: 100/no press, 100/press, 1/press, 60 000/no press.
6. **The `timer_step` unit test was split in two.**
   - `the_timer_step_carries_milliseconds_onto_whole_ticks` has six cumulative rows, two of them carry rows. The cumulative totals are computed in the test and checked against the table.
   - `the_timer_step_takes_the_largest_advance` covers 60 000 ms.
   - The old "leaves the animation clock" assertion was dropped, as the plan directs.
7. **`stand_act_timer`'s no-step test** (`an_advance_on_a_session_with_no_time_step_moves_the_clock_and_settles`) runs Counter, FlightBooker and Crud, as the old refusal test did. The plan named the counter.
8. **`virtual_clock_engine` adds two readings beyond the plan.**
   - (b) gains a 499 ms boundary row beside 0 and 500.
   - (c) gains a 299 ms midway reading, which must lie strictly between start and target.
   - The fixture uses the stand's font and viewport (`stand::options`) for its text input.
   - (b) runs the document through `Harness::wrap`, so the document keeps the `VirtualClock` its config gave it.
9. **`stand_virtual_clock`'s double-click case.**
   - It types `Grace Hopper` and shoots `crud-name` alone.
   - Three fresh sessions per mode: two double clicks and one pair of single clicks.
10. **Help and doc texts.**
    - The `advanced_ms` field help now reads "the time the session's clock moved, in milliseconds: exactly `ms`".
    - The `ms` argument help now reads "how far to move the session's clock".
    - `schema_tests`' `amended` table moved from 4 to 7 rows: `screenshot`, `click`, `advance`, `advanced_ms`.
    - The crate docs gained a sentence that a screenshot no longer depends on wall time.
    - `cli_commands`' `//!` now says the binary is driven to "six causes, all but `covered` and `fonts-unavailable`".
11. **Not done:**
    - `stand_mcp_tools`' accepted `advance` row is judged by its result not being an error. That test does not read its `tool_call` span's `ending`, so the plan's "`ending` accepted" is not asserted separately.
    - The docstring at `tests/blitz-tests/tests/session_common/mod.rs:94` ("moves the app's time forward") was left as written. That file is not on the lists, and the text still reads true loosely.

## Surfaced / stuck
None. No spec↔reality gap was met, and every expected amendment is the wrap's, as plan §Implementation notes lists them.

## Process hygiene

| process | started by | final state |
|---|---|---|
| `escher-session`, `seven_guis_native`, `cargo` (comm), and the `escher-session` args | entries 7, 9, 11 and 13 (session hosts and MCP children) | `terminated` — `census: none running` (trail `.andromeda/runs/2026-10-18T07-55-10-implement/gate-2026-10-18-one-virtual-clock.json`) |

## Decisions & corrections
- A guard refusal and a recurring standing learning. The Bash guard refused a bare `cd` into `~/.claude/skills/andromeda-implement/references`: "Blocked: cd (…) moves the session cwd for every later call". This is host-linux «Paths & the working directory», already recorded as recurring in the handoff's deferred learnings. The call was re-issued once, as an absolute-path `grep`.

## Next
The operator pass first, then `/andromeda-wrap-session`. Entries for the operator to fire, in order:
- **15** — `gate.py hygiene`, before the pre-CI commit, whose subject is `chore(2026-10-18-one-virtual-clock): operator pre-CI commit…`.
- **16** — the `fast` leg, the clean-tree guard, then the push.
- **17** — `ci.py conclusion --sha HEAD`.
