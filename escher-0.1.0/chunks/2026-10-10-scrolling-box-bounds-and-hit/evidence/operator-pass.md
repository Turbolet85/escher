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

## The operator pre-CI commit

On the operator's word — "then the commit "chore(2026-10-10-scrolling-box-bounds-and-hit): operator pre-CI commit,
for the run this chunk verdict reads"" — the operator, 2026-10-10. `git add -A`, then the commit, on
`build/escher-0.1.0`:

- sha `5cc38cba18577fee37732305a96d53bae6a1e99d`, parent `1b195c9a94723fa6a24ec32c105aec0157583608`
- subject `chore(2026-10-10-scrolling-box-bounds-and-hit): operator pre-CI commit, for the run this chunk verdict reads`
- 71 files changed, 13,309 insertions, 135 deletions; the tree read clean after it (0 rows of `git status --short`)

It carries the chunk's source and checks, its chunk folder with this record as it stood at entry 22, the phase's and
the implement's run dirs, and the route, master, handoff and ledger edits earlier steps left uncommitted. This
record's sections from here down were written after the push and are not in that commit.

## Entry 23 — the local pre-push gate, the clean-tree guard, the push

Fired once through the gate tool, as the gate contract names for an operator entry —
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/plan.md --operator 23`
— with no run dir (the pass runs under no skill), 2026-10-10T15:15Z. The tripwire line, the entry's line and the
call's summary line, verbatim:

```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 90.25s · 82739 B → 23.log · history moved: refs/remotes/origin/build/escher-0.1.0 1b195c9a→5cc38cba · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The entry carries no `expect` key, so its line asserts the exit alone; what the report reads of its output, copied
from its log:

- the `fast` leg on the committed tree: 165 result lines · 776 passed · 0 failed · 11 ignored; the CI scripts
  `Ran 78 tests` · `OK`
- the push: `To https://github.com/Turbolet85/escher.git` · `1b195c9a..5cc38cba  build/escher-0.1.0 -> build/escher-0.1.0`

**The history reading:** `history moved: refs/remotes/origin/build/escher-0.1.0 1b195c9a→5cc38cba` — the one ref
the push was meant to move, and no other. Read back after the call: `git ls-remote origin build/escher-0.1.0`
answers `5cc38cba18577fee37732305a96d53bae6a1e99d`, equal to `HEAD`; the branch reads 0 ahead of its remote and the
tree clean.

**Verdict: green** — `exit 0`; the moved history is the intended push.

## Entry 24 — the fork's CI on the pushed sha

Fired as written — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
— after the push, returned 2026-10-10T15:28Z, exit 0. Its lines, verbatim:

```
ci v1.2 · 72093aa2
repo Turbolet85/escher (the push remote `origin`) · polled 23× over 682 s
5cc38cba1857 verdict: green · checks 16/16 · wall 667 s · runs CI#38062991988 completed/success
runs: CI#38062991988 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

**Verdict: green** — `exit 0`, `contains verdict: green`: CI#38062991988, 16 of 16 checks, on
`5cc38cba18577fee37732305a96d53bae6a1e99d`. This run is the one witness of what the plan did not run locally: the
MSRV build, and the windows, macos, ios and android legs over the two fixed engine functions — the three new check
files carry no `cfg(unix)` gate, so the windows and macos jobs ran them.

**For the route's watch** (a fork CI job hanging in its package-install step): the run's log was read once,
`gh run view 38062991988 -R Turbolet85/escher --log`, 17,545 lines. Lines holding `apt-install`: 1,182 (the known
positive — the script's name is in the log). Lines holding `apt-install: attempt`: 0. The script prints that line
only for an attempt that failed, so no install bound was seen firing in this run, and no job was held: the run
completed in 667 s. Not read: per-job step timings.

## The pass

| Entry | Call | Verdict |
|---|---|---|
| 22 | `gate.py hygiene`, as written | green — `hygiene: clean` (first reading `refused 1 files`; clean after the one rewrite the operator named) |
| — | the operator pre-CI commit | `5cc38cba` on `build/escher-0.1.0` |
| 23 | `gate.py run … --operator 23` | green — exit 0 · `history moved: refs/remotes/origin/build/escher-0.1.0 1b195c9a→5cc38cba` |
| 24 | `ci.py conclusion --sha HEAD --wait 1800`, as written | green — `verdict: green · checks 16/16`, CI#38062991988 |

Entry 22 read red once, before the rewrite; no entry reads red at the end of the pass. The tree holds one uncommitted
file after the pass: this record, as extended after the push.
