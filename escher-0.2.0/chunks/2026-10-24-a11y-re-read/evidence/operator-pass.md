# Operator pass — 2026-10-24-a11y-re-read

Run by hand in plan order on the operator's word ("Run the operator pass for 2026-10-24-a11y-re-read now, each entry
by hand in plan order", 2026-10-24), 2026-10-24T21:25Z–21:48Z. No entry read red.

## Entry 18 — hygiene (fired as written)

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, exit 0:

```
root . · case exact · planes rust · base HEAD (no --marker)
hygiene: clean — read 468 (runs 463 · evidence 2 · inputs 3) · trails 14 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`git add -A`, then committed `4cbf0a4c93e9c60ac192968004daa95c2ba4cb32` with the subject
`chore(2026-10-24-a11y-re-read): operator pre-CI commit, for the run this chunk's verdict reads`. The tree after it:
0 rows in `git status --short`.

## Entry 19 — fast, the clean-tree guard, the push

`gate.py run --plan escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md --operator 19`. The dry-run printed
`history tripwire: git`. The firing:

```
 19 probe       green · exit 0 · 104.11s · 114143 B → 19.log · history moved: refs/remotes/origin/build/escher-0.2.0 db585b71→4cbf0a4c · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

The history move is the push the entry exists for. From its log (`$TMPDIR/andromeda-gate/2026-10-24-a11y-re-read/run-20261024T212647Z/19.log`):
the workspace tally `lines 206 passed 1105 failed 0 ignored 12`, the CI scripts `Ran 100 tests` · `OK`, and the push
`db585b711..4cbf0a4c9  build/escher-0.2.0 -> build/escher-0.2.0`.

## Entry 20 — the CI read (fired as written)

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`, exit 0:

```
repo Turbolet85/escher (the push remote `origin`) · polled 37× over 1114 s
4cbf0a4c93e9 verdict: green · checks 17/17 · wall 1092 s · runs CI#40134371519 completed/success
runs: CI#40134371519 push completed/success
```

## Entry 21 — the per-job tally (report-only)

`gate.py run --plan … --operator 21 --id 40134371519`:

```
 21 probe       recorded · exit 0 · 6.07s · 267 B → 21.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 21 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 20
```

Its log, whole:

```
Coverage report · 187 · 1102 · 0 · 12
Accessibility (a11y) tests · 14 · 84 · 0 · 0
Test (macos) · 187 · 1102 · 0 · 12
Test (windows) · 187 · 1057 · 0 · 9
Test [default features] · 206 · 1105 · 0 · 12
Stand checks repeated · 198 · 837 · 0 · 21
```

Recorded against CI#40125511019, with the a11y job's move attributed per file, in `evidence/ci.md`.

## Not committed

This file and `evidence/ci.md` were written after the push. They are untracked, ride the wrap's commit, and change no
file CI read.
