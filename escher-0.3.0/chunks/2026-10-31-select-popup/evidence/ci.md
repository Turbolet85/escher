# CI — 2026-10-31-select-popup

Two CI reads. The first, on the pre-CI commit, stopped the operator pass red on `Test (macos)`, a defect of this chunk's. The
operator's word took the fix and the second read: "your fix, a header heavier than a row on every build, the test as written;
a run with no synthesized bold paints as before."

## The first pass, in plan order

| Entry | What ran | Reading |
|---|---|---|
| 28 | `gate.py hygiene` | `hygiene: clean — read 708 (runs 697 · evidence 2 · inputs 9) · trails 14 not read · copies 7 not read by P1 — 0 host paths kept · binary 0 not read by P1` |
| the commit | `chore(2026-10-31-select-popup): operator pre-CI commit, for the run this chunk's verdict reads` | `8cd9c1286757039fbd74a6fd4b8491cb380b81f5` · 760 files · tree clean after |
| 29 | `ci-leg.sh fast`, the clean-tree guard, `git push origin build/escher-0.3.0` | exit 0 · `da536ea7a..8cd9c1286` · the remote read back at `8cd9c1286`, the two records-door commits carried |
| 30 | `ci.py conclusion --sha HEAD --wait 1800` (the operator's bound; the plan writes 2400) | **red**, below |
| 31 | the per-OS tally | not run, the pass having stopped |

Entry 30, verbatim:

```
8cd9c1286757 verdict: red · checks 17/17 · first-fail +767 s Test (macos) · runs CI#41233234207 in_progress/-
runs: CI#41233234207 push in_progress/-
failed 1: Test (macos) (failure)
running 3: oldest Test (windows) 385 s
run open: CI#41233234207 in_progress
```

### CI#41233234207, read to its end

`completed · failure`, 17 jobs. **Failure, one:** Test (macos). **Success, sixteen:** Test [default features] · Rustfmt · Clippy
· Test CI scripts · Build counter example · Build wasm examples · Documentation · Dependency audit · MSRV Build [Rust 1.91] ·
Coverage report · Stand checks repeated · Test (ios) · Test (windows) · Test (android) · Accessibility (a11y) tests · Build
[default features].

The failure, from job 124633136964's log:

```
test a_groups_header_is_darker_than_a_row_by_weight ... FAILED
thread 'a_groups_header_is_darker_than_a_row_by_weight' panicked at tests/blitz-tests/tests/select_popup_painted.rs:292:9:
incremental false: the header reads 70972, the row 70972
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo` stops at the first failing target by default, so that job's later targets were not read on this run.

## The cause, reproduced on this host

`packages/blitz-paint/src/text.rs` emboldened a run when `FONT_EMBOLDEN_ENABLED || (paint.synthesize_bold &&
synthesis.embolden())`, one vector for both arms. So on a build with the flag on, every run took one embolden and a header
read no heavier than a row. `FONT_EMBOLDEN_ENABLED` is true on macOS under `apple-font-embolden`, which `apps/browser`'s
default features name, and on any platform under `font-embolden`.

`cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, before the
fix, on this linux host:

```
test a_groups_header_is_darker_than_a_row_by_weight ... FAILED
incremental false: the header reads 70972, the row 70972
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The same two numbers macOS read. The same target with no feature read `6 passed; 0 failed`. No local gate builds that feature,
so the full block's green did not witness this, and the plan's "no OS-sensitive case" did not hold for it.

## The fix, read both ways

A synthesized bold now adds to the build's embolden, in place of standing beside it. The test is as written:
`git diff 8cd9c1286 -- tests/blitz-tests/tests/select_popup_painted.rs` prints nothing.

`--test select_popup --test select_popup_painted --test stand_act_popup`, after the fix:

| Build | `select_popup` | `select_popup_painted` | `stand_act_popup` |
|---|---|---|---|
| plain | 25 · 0 | 6 · 0 | 12 · 0 |
| `blitz-paint/font-embolden` | 25 · 0 | 6 · 0 | 12 · 0 |

### A run with no synthesized bold paints as before

A hash of the whole frame on each build, before and after the fix, from a throwaway test target removed before the commit.
The authored-bold paragraph goes through the ordinary path, which asks for no synthesized bold.

| Frame | Plain, before | Plain, after | Emboldening, before | Emboldening, after |
|---|---|---|---|---|
| closed stand | `0d0c736a9a887c2d` | same | `2d2d15cbd4c193db` | same |
| open list, no header | `c523c16ac28cb124` | same | `543ea04ccbe22e96` | same |
| authored bold, closed | `8e827d95625ef994` | same | `ecee47a20afb4abd` | same |
| authored bold, open list | `d5bdbc31be8a2285` | same | `e981874a7ac52497` | same |
| open list with a header | `5f679e03df361e51` | same | `84b3332ff404b632` | **`f8295ffd12736ac5`** |

Only the frame holding a synthesized header moved, and only on the build that had the defect.
