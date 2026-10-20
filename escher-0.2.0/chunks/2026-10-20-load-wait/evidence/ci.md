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
