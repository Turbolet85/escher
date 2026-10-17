# Operator pass — 2026-10-16-fast-failing-mutants

The operator's directive (this session, 2026-10-16): "Operator pass for 2026-10-16-fast-failing-mutants (G3 deployed,
go): fire entry 23 (gate.py hygiene) as written; then commit "chore(2026-10-16-fast-failing-mutants): operator pre-CI
commit, for the run this chunk verdict reads"; then entry 24 via gate.py run --operator 24 in the background; then
entry 25 (ci.py conclusion --wait) as written. Record each in evidence/operator-pass.md. Start no wrap; stop after
entry 25 and report the sha and verdict."

## Entry 23 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, fired as written

```
gate v1.19 · bebab50b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 60 (runs 51 · evidence 5 · inputs 4) · trails 21 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

`hygiene: clean`, so the entry's `contains hygiene: clean` atom holds. The tool's stamp reads `bebab50b`, where the
/implement run's `gate.py run` lines read `457c1da1`: the tools were redeployed in between ("G3 deployed").
