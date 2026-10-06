# Operator pass — entries 15 · 16 · 17

Driven by the agent on the operator's direction (2026-10-06), each by hand, in plan order.

## Entry 15 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-06T19:03Z): `hygiene: clean — read 34 (runs 34 · evidence 0 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after this section was written, over the evidence set the pre-CI commit carries. That run read `hygiene: clean — read 35 (runs 34 · evidence 1 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` at exit 0.

## Pre-CI commit
- `910d1237` `chore(2026-10-06-snapshot-model): operator pre-CI commit, for the run this chunk's verdict reads` (`git add -A`, the whole tree).

## Entry 16 — fast · clean-tree guard · push
- run: `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- exit 0 · fast green (workspace 471 · 0 · 5 over 125 result lines from `target/ci-logs/test.log`; `Ran 64 tests`) · tree clean · push `ee88e85e..910d1237  build/escher-0.1.0 -> build/escher-0.1.0` · after it, `HEAD` = `origin/build/escher-0.1.0` = `910d1237e4e2`.

## Entry 17 — CI conclusion
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- exit 0 · atom `contains verdict: green` ✓
- `910d1237e4e2 verdict: green · checks 16/16 · wall 515 s · runs CI#37516167752 completed/success` (repo Turbolet85/escher, polled 18× over 527 s)
- Everything in this file below the entry 15 section was written after the push, so it rides the wrap's commit.
