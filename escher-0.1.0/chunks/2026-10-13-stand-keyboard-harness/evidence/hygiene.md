# Operator entry 15 — hygiene

Fired as written, by its own command line (`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`),
before the pre-CI commit, 2026-10-13. Exit 0. Expect `['exit 0', 'contains hygiene: clean']`: held.

```
gate v1.17 · 383c6f7b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 83 (runs 75 · evidence 0 · inputs 8) · trails 14 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
