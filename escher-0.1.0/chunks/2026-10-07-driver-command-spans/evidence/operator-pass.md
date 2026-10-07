# Operator pass — 2026-10-07-driver-command-spans

The plan's three `leg = 'operator'` entries (26 · 27 · 28), each driven by hand in plan order, on
the operator's word given in the implement session, 2026-10-07: "Run the operator pass for
2026-10-07-driver-command-spans now, each entry by hand in plan order: hygiene, the commit
"chore(2026-10-07-driver-command-spans): operator pre-CI commit, for the run this chunk verdict
reads", the fast leg with the clean-tree guard and the push, then ci.py conclusion --sha HEAD
--wait 1800. Record it in evidence/operator-pass.md. The apt-get step hung in every run tonight:
five minutes after the push read each open job step through the API; a job in apt-get over three
minutes is cancelled and the unfinished jobs re-run, as often as needed. Stop and tell me on a real
red." Every command is spelled as the gate tool prints it: the repository root as `.`, the home
directory as `~`.

## Entry 26 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-07T22:12:29Z, bare, from the repository root, on the uncommitted tree the
  pre-CI commit then staged
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.12 · 8357110d
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 37 (runs 32 · evidence 2 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-07T22:12:40Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 38 (runs 32 · evidence 3 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.
