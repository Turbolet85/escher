# /andromeda-implement — 2026-10-20-load-wait · outcome: green (smoke skipped — no boot-path change)

## Files changed
- `packages/escher-driver/src/session.rs` — `pub const LOAD_WAIT` (5 s), the private `LOAD_POLL` (10 ms), the
  crate-private `settle_waiting` over the generic `settle_within`, `Session::act` through it with its doc restated, and
  four unit tests of the wait.
- `packages/escher-driver/src/execute.rs` — `settled_step` settles through `settle_waiting`; `Session::run`'s doc
  reads «Settling moves no time; a load in flight is waited for up to `LOAD_WAIT`».
- `packages/escher-driver/src/schema.rs` — the `settled` help: «whether the app went quiet after the step, a load in
  flight waited for up to 5 s».
- `packages/escher-driver/src/schema_tests.rs` — `the_settled_help_states_the_load_wait_and_the_wait_ends_before_an_answer_read`.
- `packages/escher-driver/src/lib.rs` — `pub use session::{LOAD_WAIT, Session};` (re-exports 55 → 56).
- `tests/blitz-tests/tests/stand_load_wait.rs` (new) — the in-file page, boot and self-answering stand-in `LateNet`;
  both arms in both layout modes.
- Ledger: `escher-0.2.0/verification-matrix.json` v020-07 planned → implemented, ref the two `stand_load_wait`
  tests. Evidence: `escher-0.2.0/chunks/2026-10-20-load-wait/evidence/counts.md`.

## Gates
Block fired once, `.andromeda/runs/2026-10-20T02-02-07-implement/gate-2026-10-20-load-wait.json`:
`entries 20 · green 16 · red 0 · recorded 0 · timeout 0 · not-run 4` (17–20 `leg operator`). One iteration. No
deferral (rust delta 6). Counts all as predicted: escher-driver 99 passed; `stand_load_wait` 2 passed (10.10 s);
`run stand` 185 · 0 · 7 over 46 files; a11y 6 · 6 · 4 · 8 · 5 · 14 = 43; workspace `lines 202 passed 1062 failed 0
ignored 12`; doc green.

## Deviations from plan (with justifications)
1. **Step 1's stopping rule.** The plan says the loop returns the last `Loads` «once the next poll would pass the
   bound» and also that it «never sleeps past `LOAD_WAIT`, so the wait ends within one settle of the bound». Read
   literally the first returns before the bound, which arm 2's «at least `LOAD_WAIT`» contradicts. Built: the last
   sleep is `min(poll, time left)`, then one more settle, so a never-answered wait returns at ≥ the bound and within
   one settle of it.
2. **Unit test (c)** asserts one settle and no time assertion: a sleep only ever precedes another settle, so
   `settles == 1` proves it slept none, and a wall-time assertion would be a CI flake source.
3. **The pin (step 5)** also asserts the five acting verbs carry the `SETTLED` field and that `LOAD_WAIT` is whole
   seconds, so the help's «5 s» is the value's exact rendering.
4. **The check (step 8)** adds two assertions beyond the plan's list: arm 1's wall time is also ≥ the stand-in's
   200 ms (the step did wait), and arm 2's next snapshot still reads the box at no height. `box_height` reads the
   driver's `snapshot` text line for `fx-box`.
5. **`execute.rs`'s doc** links `LOAD_WAIT` by path (`crate::LOAD_WAIT`) rather than importing it, since an import
   used only by a doc link reads as unused to rustc.

No scope-record line: every changed file is on research's lists (`gate.py scope`: `clean — changed 6 · listed 6 ·
recorded 0`).

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries (gate entries 1, 2, 11–16) | this run (gate tool) | terminated — `gate.py census --comm cargo --comm rustc --comm stand_load_wait --comm escher-session --comm seven_guis_nati`: `census: none running` |
| `LateNet` answer threads | `stand_load_wait` (in process) | ended with their test process |

## Decisions & corrections
none

## Next
The operator pass: entry 17 (`gate.py hygiene`) before the pre-CI commit (subject exactly
`chore(2026-10-20-load-wait): operator pre-CI commit, for the run this chunk's verdict reads`), entry 18 (fast gate
and push), entry 19 (`ci.py conclusion`), entry 20 (per-OS tallies into `evidence/ci.md`), then
`/andromeda-wrap-session`.
