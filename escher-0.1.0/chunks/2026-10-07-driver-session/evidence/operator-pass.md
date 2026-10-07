# Operator pass — entries 25 · 26 · 27

Driven by the agent on the operator's direction (2026-10-07 UTC, given in the implement session as a pasted
ruling the operator then confirmed as theirs to act on — "All of it"; the text and the confirmation round are
`inputs#I3`: "run the operator pass by hand in plan order: entry 25, the pre-CI commit …, entry 26, entry 27.
Record it in evidence/operator-pass.md and stop if anything is red"), each by hand, in plan order.

## Before the pass
- No source, test or manifest was edited between the implement report and this pass. The block read
  `entries 27 · green 24 · red 0 · recorded 0 · timeout 0 · not-run 3` (run dir
  `.andromeda/runs/2026-10-07T05-31-58-implement/`; `evidence/gate-readings.md`), and the scope read after it
  `scope: clean — changed 19 · listed 18 · recorded 1 (companion 0 · mechanical 0 · in-intent 1 · widening 0)`.
- Written between the report and this pass, all under the chunk folder or the run dir: the ruling snapped as
  `inputs#I3` (its source `relay-3.md` in the run dir), `evidence/wrap-directions.md`, and this file.
- The implement outcome was **surfaced**, not green: the gap and the ruling on it are in
  `evidence/wrap-directions.md`. The pass was run on the operator's direction after that ruling.

## Entry 25 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-07T06:17Z): `hygiene: clean — read 46 (runs 39 · evidence 2 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above and `evidence/wrap-directions.md` were written, over the evidence set the pre-CI commit carries (2026-10-07T06:17Z). That run read `hygiene: clean — read 48 (runs 39 · evidence 4 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1` at exit 0.
