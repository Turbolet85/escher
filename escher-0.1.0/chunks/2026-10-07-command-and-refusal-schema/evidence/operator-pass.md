# Operator pass — entries 12 · 13 · 14

Driven by the agent on the operator's direction, given in the implement session after its report: "Run the operator
pass for 2026-10-07-command-and-refusal-schema now, each entry by hand in plan order: hygiene, the commit
"chore(2026-10-07-command-and-refusal-schema): operator pre-CI commit, for the run this chunk verdict reads", the fast
leg with the clean-tree guard and the push, then ci.py conclusion --sha HEAD --wait 1800. Record it in
evidence/operator-pass.md. Stop and tell me if any entry is red." — the operator, 2026-10-07. Each entry by hand, in
plan order.

## Before the pass
- No source, test or manifest was edited between the implement report and this pass (`git status --short` read the
  same 1 modified and 3 new source files the report lists, 13 rows in all). The block read
  `entries 14 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 3` twice, before and after the controls (run dir
  `.andromeda/runs/2026-10-07T12-10-41-implement/`), and the scope read after it
  `scope: clean — changed 4 · listed 4 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 42`.
- Written between the report and this pass: this file only.
- The implement outcome was **green**, smoke skipped (no boot-path or UI-surface change); the chunk claims no
  capability of the matrix.

## Entry 12 — hygiene
- run: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` ✓
- First read (2026-10-07T12:22Z): `hygiene: clean — read 34 (runs 33 · evidence 1 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- It was re-fired after the lines above were written, over the evidence set the pre-CI commit carries (2026-10-07T12:22Z). That run read at exit 0: `hygiene: clean — read 35 (runs 33 · evidence 2 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` (one file more than the first read: this record).
