# The operator pass — 2026-10-10-scrolling-box-bounds-and-hit

Run on the operator's word, given in the implement session on 2026-10-10 after its report: "Run the operator pass
for 2026-10-10-scrolling-box-bounds-and-hit now, in plan order: entry 22 hygiene as written, then the commit […],
then entry 23 fired as the letter names for an operator entry, then entry 24. […] Stop and tell me if any entry is
red." — the operator, 2026-10-10. The entries are the plan's three `leg = 'operator'` entries (22, 23, 24), in block
order. Each record below was written after its entry was read.

## Entry 22 — hygiene, before the pre-CI commit

**The first reading** — the entry fired as written,
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, 2026-10-10T15:14Z, exit 0:

```
gate v1.14 · e8e4a72c
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
P1 .andromeda/runs/2026-10-10T13-07-20-phase/relay-3.md:3 ×1 · home
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 47 (runs 37 · evidence 3 · inputs 7) · trails 13 not read · copies 5 not read by P1 — 1 host paths kept · binary 0 not read by P1
```

**Verdict: red** — `exit 0` holds, `contains hygiene: clean` does not: the summary word is `refused`.

**The one row, read:** the phase's run-dir copy of the operator's own review message at P5 (629 bytes). Its third
line names, once, the file the review pointed the phase to — a home path outside this repository, the form word
`home`. The row is not `in-root`, so the respell does not apply. The same bytes stand as the chunk's verbatim input
copy `inputs/I3-relay-3.md.txt` (`cmp` reads the two identical), which the predicate does not read — it is the
`1 host paths kept` of the summary line. No file of this chunk's implement run is listed: its run dir and the three
evidence files it wrote read clean.

The pass stopped here on the operator's instruction, with nothing rewritten and nothing committed, and the row was
put to him: the file is the operator's words in a phase record.

**The rewrite, on the operator's word:** "Rewrite that one token in relay-3.md to
<overseer>/relays/scrolling-box-fork-answers.md, nothing else, record the rewrite (count, bytes, sha before and
after) in evidence/operator-pass.md, then hygiene again until clean and go on: the pre-CI commit, entry 23, entry
24." — the operator, 2026-10-10. Done as one exact replacement in
`.andromeda/runs/2026-10-10T13-07-20-phase/relay-3.md`:

- occurrences replaced: 1 — the token went from 91 bytes to 47; 0 occurrences left, 0 home paths left in the file
- file size: 629 → 585 bytes (−44)
- sha256 before `12173582f7fba713bdf8acfe746c140f992b346f63f49b48b2090c074f1d260f`
- sha256 after `95ff21bd734dda3aebee80e7398996a7d481a96b4ef20be650f26d6df9794d3e`
- LF terminators kept: 3 before, 3 after, no CR

Nothing else in the file was touched. The chunk's verbatim input copy `inputs/I3-relay-3.md.txt` was not touched: it
keeps the message's bytes as given, and no longer equals the run-dir copy.

**The entry, fired again as written**, 2026-10-10T15:15Z, exit 0 — the header and the control line as above, no row,
and:

```
hygiene: clean — read 48 (runs 37 · evidence 4 · inputs 7) · trails 13 not read · copies 5 not read by P1 — 1 host paths kept · binary 0 not read by P1
```

**Read again with the rewrite recorded here**, the same call, 2026-10-10T15:15Z, exit 0 — the same header, control
line and summary line, no row. The `1 host paths kept` is the verbatim input copy named above.

**Verdict: green** — `exit 0`, `contains hygiene: clean`, on the tree the pre-CI commit carries.
