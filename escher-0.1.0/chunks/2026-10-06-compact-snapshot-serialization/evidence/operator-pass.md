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
