# Operator pass — entries 18 · 19 · 20

Driven by the agent on the operator's direction, given in the implement session after its report: "Run the operator
pass for 2026-10-07-sink-target-allowlist now, each entry by hand in plan order: the hygiene entry, then the commit
"chore(2026-10-07-sink-target-allowlist): operator pre-CI commit, for the run this chunk's verdict reads", then the
fast leg with the clean-tree guard and the push, then ci.py conclusion --sha HEAD --wait 1800. Record it in
evidence/operator-pass.md as the last chunk did. Stop and tell me if any entry is red." — the operator, 2026-10-07.
Each entry by hand, in plan order.

## Before the pass
- No source, test or manifest was edited between the implement report and this pass. The block read
  `entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3` (run dir
  `.andromeda/runs/2026-10-07T07-57-42-implement/`), and the scope read after it
  `scope: clean — changed 6 · listed 6 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 41`.
- Written between the report and this pass: this file only.
- The implement outcome was **green + smoke**; the chunk claims no capability of the matrix.

## Entry 18 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-07T08:12Z): `hygiene: clean — read 35 (runs 32 · evidence 3 · inputs 0) · trails 14 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above were written, over the evidence set the pre-CI commit carries (2026-10-07T08:12Z). That run read the same line at exit 0: `hygiene: clean — read 35 (runs 32 · evidence 3 · inputs 0) · trails 14 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` (this file was already in the first read's set).
