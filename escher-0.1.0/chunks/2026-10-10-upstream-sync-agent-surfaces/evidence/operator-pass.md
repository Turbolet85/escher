# Operator pass — 2026-10-10-upstream-sync-agent-surfaces

The plan's four `leg = 'operator'` entries (35 · 36 · 37 · 38), each driven by hand in plan order, on
the word given in the implement session, 2026-10-10 (`inputs#I2`, verbatim): "Run the operator pass
for 2026-10-10-upstream-sync-agent-surfaces now, each entry by hand in plan order: entry 35 hygiene;
then the pre-CI commit, which IS the merge commit (MERGE_HEAD is set: subject and body as in the plan
implementation notes; scrolling.rs, the new standing check and the evidence go into it; no stash,
reset, merge --abort or checkout before it); entry 36 the fast leg with the clean-tree guard and the
push; entry 37 ci.py conclusion --sha HEAD --wait 3600; entry 38 the job-log count of our named
targets into evidence/ci.md. Push nothing on top until the CI run settles. Record it in
evidence/operator-pass.md. Stop and tell me if any entry is red. Do not start the wrap." Every
command is spelled as the gate tool prints it: the repository root as `.`, the home directory as `~`.

## Entry 35 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-10T01:33:03Z, bare, from the repository root, on the uncommitted merge the
  pre-CI commit then staged (`MERGE_HEAD` = `7832c177ff272128154bac58afe56c2b9164b417`)
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.13 · 3718c868
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 46 (runs 36 · evidence 6 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-10T01:33:14Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 47 (runs 36 · evidence 7 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.
