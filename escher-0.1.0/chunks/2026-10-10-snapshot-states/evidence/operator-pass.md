# The operator pass — 2026-10-10-snapshot-states

Run on the operator's word, given in the implement session on 2026-10-10 after its report: "Run the operator pass
for 2026-10-10-snapshot-states now, in plan order: entry 18 hygiene as written, then the commit
"chore(2026-10-10-snapshot-states): operator pre-CI commit, for the run this chunk verdict reads", then entry 19
fired as the letter names for an operator entry, then entry 20. Record each in evidence/operator-pass.md, the
operator firing line with its history reading verbatim and no host path in the record. Stop and tell me if any
entry is red." — the operator, 2026-10-10. The entries are the plan's three `leg = 'operator'` entries (18, 19, 20),
in block order. Each record below was written after its entry was read.

## Entry 18 — hygiene, before the pre-CI commit

Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, 2026-10-10T17:31Z,
exit 0:

```
gate v1.14 · e8e4a72c
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 50 (runs 41 · evidence 2 · inputs 7) · trails 15 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

No row. The two evidence files read are implement's `controls.md` and `readings.md`.

**Read again with this record in the tree**, the same call, 2026-10-10T17:31Z and once more after this paragraph
was written, exit 0 both times — the same header and control line, no row, and:

```
hygiene: clean — read 51 (runs 41 · evidence 3 · inputs 7) · trails 15 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**Verdict: green** — `exit 0`, `contains hygiene: clean`, on the tree the pre-CI commit carries.
