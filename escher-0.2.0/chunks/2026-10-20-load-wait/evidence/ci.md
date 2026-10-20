# Operator pass — 2026-10-20-load-wait

Entries of `plan.md` §Test Commands, fired in plan order on the operator's word (2026-10-20).

## Entry 17 — `gate.py hygiene` (fired as written)

First firing, red:

```
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 41 (runs 40 · evidence 1 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The one file was `.andromeda/runs/2026-10-20T01-41-31-phase/p5-controls.sh` (phase P5's control script), whose lines 4–6
held the repository root, a session scratchpad path and a home path (rows `in-root,tmp,home`, so `gate.py respell`
could not take it). On the operator's word the three lines were rewritten by hand in the run-dir spelling:
`ROOT=.` · `C=$TMPDIR/{scratchpad}/clone-load-wait` · `GATE=~/.claude/skills/andromeda-tools/scripts/gate.py`.

Re-fired, green:

```
hygiene: clean — read 41 (runs 40 · evidence 1 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

With this file written, re-fired before the commit, green: `hygiene: clean — read 42 (runs 40 · evidence 2 · inputs 0) …`.

## The pre-CI commit

`8dfd1c02b chore(2026-10-20-load-wait): operator pre-CI commit, for the run this chunk's verdict reads` — its body
names the respell above.

## Entry 18 — fast leg, clean-tree guard, push (`gate.py run --plan … --operator 18`)

```
 18 probe       green · exit 0 · 99.65s · 110341 B → 18.log · history moved: refs/remotes/origin/build/escher-0.2.0 165cfe44→8dfd1c02 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```

The history move is the intended push of `8dfd1c02b` to `origin/build/escher-0.2.0`.

## Entry 19 — `ci.py conclusion --sha HEAD --wait 1800` (fired as written)

```
repo Turbolet85/escher (the push remote `origin`) · polled 38× over 1147 s
8dfd1c02b090 verdict: green · checks 17/17 · wall 1127 s · runs CI#39360124515 completed/success
runs: CI#39360124515 push completed/success
```

The coverage job's floor lines (read from the run's log, colour codes stripped):

```
packages/escher-driver 4695/4794 = 97.93 % (floor 97 %)
packages/dioxus-native-dom 2565/3027 = 84.74 % (floor 83 %)
packages/blitz-test-harness 949/1008 = 94.15 % (floor 90 %)
coverage floor: 3 of 3 crates at or above their floors
```

## Entry 20 — per-OS tallies (`gate.py run --plan … --operator 20 --id 39360124515`)

```
 20 probe       recorded · exit 0 · 6.3s · 266 B → 20.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 20 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 19
```

Lines · passed · failed · ignored per job, against the control CI#39348671810:

| job | CI#39348671810 | CI#39360124515 | moved |
|---|---|---|---|
| Test [default features] | 201 · 1055 · 0 · 12 | 202 · 1062 · 0 · 12 | +1 line (`stand_load_wait`), +7 (2 `stand_load_wait` + 5 escher-driver) |
| Test (macos) | 182 · 1052 · 0 · 12 | 183 · 1059 · 0 · 12 | +1, +7 (the same) |
| Coverage report | 182 · 1052 · 0 · 12 | 183 · 1059 · 0 · 12 | +1, +7 (the same) |
| Test (windows) | 182 · 1010 · 0 · 9 | 183 · 1017 · 0 · 9 | +1, +7 (the same) |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | none: `stand_load_wait` is not in the a11y leg |
| Stand checks repeated | 159 · 639 · 0 · 21 | 162 · 645 · 0 · 21 | +3 lines, +6 (`stand_load_wait`'s 2 tests × 3 rounds, zero failures) |

Every moved count is the prediction: +1 line and +2 passed on every job that runs blitz-tests, +5 where escher-driver's
unit tests run, and the stand-repeat leg holding the new check three rounds under the zero-failure budget.
