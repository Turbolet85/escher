# The operator pass — 2026-10-10-driver-cli

Run on the operator's word, given in the implement session on 2026-10-10 after its report: "Run the operator pass
for 2026-10-10-driver-cli now, in plan order." — the operator, 2026-10-10. The entries are the plan's three
`leg = 'operator'` entries (24, 25, 26), in block order. Each record below was written after its entry was read.

## Entry 24 — hygiene, before the pre-CI commit

**The first reading (at implement's report, 2026-10-10):** `hygiene: refused 1 files — P1 1 · P2 0 · P3 0`, the one
row `P1 .andromeda/runs/2026-10-10T02-50-31-phase/p4-founder-forks.md:35 ×27 · tmp` — the session address in the
phase's forks text, spelt as a temp-dir path 27 times.

**The rewrite, on the operator's word:** "in .andromeda/runs/2026-10-10T02-50-31-phase/p4-founder-forks.md rewrite
each of the 27 [the address] to <dir> (the plan spells the address so), nothing else, then hygiene again until
clean" — the operator, 2026-10-10. The bracketed words stand for the one token of his sentence this record does not
repeat: the temp-dir path itself, which the hygiene predicate reads wherever it is written (a first draft of this
record quoted it and was refused, `P1 … evidence/operator-pass.md ×1 · tmp`; the token was taken out and the entry
read again, below). Done as one exact replacement: 27 occurrences, 27,265 → 27,211 bytes (27 × 2), sha256
`7e7d8aba4a7be6f5` → `ed8861891fe96256` (first 16 hex digits), 0 occurrences left, LF terminators kept. Nothing else
in the file was touched; the one other temp-dir spelling in it holds a `<uid>` placeholder and is design text the
predicate does not read.

**The entry, fired as written** — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`,
2026-10-10T09:43Z, exit 0:

```
gate v1.14 · e8e4a72c
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 51 (runs 40 · evidence 5 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**Read again with this record in the tree**, the same call, 2026-10-10T09:44Z, exit 0 — the header and the control
line as above, and:

```
hygiene: clean — read 52 (runs 40 · evidence 6 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**Verdict: green** — `exit 0`, `contains hygiene: clean`, on the tree the pre-CI commit carries.
