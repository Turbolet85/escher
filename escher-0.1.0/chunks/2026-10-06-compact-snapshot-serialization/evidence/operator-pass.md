# Operator pass — entries 17 · 18 · 19

Driven by the agent on the operator's direction (2026-10-06 UTC, given in the implement session), each by hand, in plan order.

## Before the pass — the quoting change (the operator's direction, same message)
- `Snapshot::to_text` writes a name, an id and a value in `str`'s `Debug` form (`{:?}`) in place of `str::escape_debug`, so an apostrophe is not backslash-escaped. The plan's step 1 names `escape_debug`; the operator's word supersedes it. The doc comment says so, `strings_are_quoted_and_escaped` gained a case with an apostrophe in a name (the unit count stays 7), and the stand check builds its expected fields the same way.
- Local reading after it: the plan's block through the gate tool, `entries 19 · green 16 · red 0 · recorded 0 · timeout 0 · not-run 3`. The eight screen sizes read unchanged (`sizes.txt`).

## Entry 17 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-06T23:46Z): `hygiene: clean — read 33 (runs 32 · evidence 1 · inputs 0) · trails 12 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above were written, over the evidence set the pre-CI commit carries. That run read `hygiene: clean — read 34 (runs 32 · evidence 2 · inputs 0) · trails 12 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` at exit 0.

## Pre-CI commit
- `d13935da` `chore(2026-10-06-compact-snapshot-serialization): operator pre-CI commit, for the run this chunk's verdict reads` (`git add -A`, the whole tree: 46 files).

## Entry 18 — fast · clean-tree guard · push
- run: `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- exit 0 (2026-10-06T23:47Z) · fast green (workspace 518 · 0 · 5 over 130 result lines from `target/ci-logs/test.log`; `Ran 64 tests`) · tree clean · push `97cc4842..d13935da  build/escher-0.1.0 -> build/escher-0.1.0` · after it, `HEAD` = `origin/build/escher-0.1.0` = `d13935da269b`.

## Entry 19 — CI conclusion: green
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- exit 0 · atom `contains verdict: green` ✓ (read 2026-10-06T23:57Z)
- `d13935da269b verdict: green · checks 16/16 · wall 566 s · runs CI#37548499153 completed/success` (repo Turbolet85/escher, polled 20× over 589 s)
- The pre-CI commit section, entry 18 and this section were written after the push, so they ride the wrap's commit.
