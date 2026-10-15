# The operator pass — entries 16, 17 and 18

## Entry 16 — hygiene, fired by its own command line

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, from the repository root, before the
pre-CI commit — exit 0:

```
gate v1.17 · 383c6f7b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 71 (runs 54 · evidence 11 · inputs 6) · trails 14 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Atoms: `exit 0` ✓ · `contains hygiene: clean` ✓.
