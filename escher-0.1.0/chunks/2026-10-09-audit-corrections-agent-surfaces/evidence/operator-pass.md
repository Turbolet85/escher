# Operator pass — 2026-10-09-audit-corrections-agent-surfaces

The plan's four `leg = 'operator'` entries (16 · 17 · 18 · 19), each driven by hand in plan order, on
the word given in the implement session, 2026-10-09 (`inputs#I3`, verbatim): "Run the operator pass
for 2026-10-09-audit-corrections-agent-surfaces now, each entry by hand in plan order: hygiene, the
commit "chore(2026-10-09-audit-corrections-agent-surfaces): operator pre-CI commit, for the run this
chunk verdict reads", the fast leg with the clean-tree guard and the push, then ci.py conclusion
--sha HEAD --wait 1800, then the install-step durations. Record it in evidence/operator-pass.md.
Stop and tell me if any entry is red. Do NOT start the wrap after it: a seam hold is in force until
my next line." Every command is spelled as the gate tool prints it: the repository root as `.`, the
home directory as `~`.

## Entry 16 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-09T20:13:00Z, bare, from the repository root, on the uncommitted tree the
  pre-CI commit then staged
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.13 · 3718c868
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 42 (runs 36 · evidence 1 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-09T20:13:11Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 43 (runs 36 · evidence 2 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.
