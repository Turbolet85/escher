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

## The operator pre-CI commit

On the operator's word — "Then the commit "chore(2026-10-10-driver-cli): operator pre-CI commit, for the run this
chunk verdict reads"" — the operator, 2026-10-10. `git add -A`, then the commit, on `build/escher-0.1.0`:

- sha `e144b44d69f36a981478371d487655949caaffde`, parent `0493d26a82fb6675fa7bf89caf4d02855f1b889f`
- subject `chore(2026-10-10-driver-cli): operator pre-CI commit, for the run this chunk verdict reads`
- 91 files changed, 12,142 insertions, 335 deletions; the tree read clean after it (0 rows of `git status --short`)

It carries the chunk's source and checks, its chunk folder with this record as it stood at entry 24, the three run
dirs the tree held, and the ledger edits earlier steps left uncommitted. This record's sections from here down were
written after the push and are not in that commit.

## Entry 25 — the local pre-push gate, the clean-tree guard, the push

Fired once through the gate tool, as the gate contract names for an operator entry —
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan escher-0.1.0/chunks/2026-10-10-driver-cli/plan.md --operator 25`
— with no run dir (the pass runs under no skill), 2026-10-10T09:44Z. The tripwire line, the entry's line and the
call's summary line, verbatim:

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 95.43s · 80896 B → 25.log · history moved: refs/remotes/origin/build/escher-0.1.0 0493d26a→e144b44d · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The entry carries no `expect` key, so its line asserts the exit alone; what the report reads of its output, copied
from its log (`$TMPDIR/andromeda-gate/2026-10-10-driver-cli/run-20261010T094432Z/25.log`):

- the `fast` leg on the committed tree: 162 result lines · 760 passed · 0 failed · 11 ignored; the CI scripts
  `Ran 78 tests` · `OK`
- the push: `To https://github.com/Turbolet85/escher.git` · `0493d26a..e144b44d  build/escher-0.1.0 -> build/escher-0.1.0`

**The history reading:** `history moved: refs/remotes/origin/build/escher-0.1.0 0493d26a→e144b44d` — the one ref
the push was meant to move, and no other. Read back after the call: `git ls-remote origin build/escher-0.1.0`
answers `e144b44d69f36a981478371d487655949caaffde`, equal to `HEAD`; the branch reads 0 ahead of its remote and the
tree clean.

**Verdict: green** — `exit 0`; the moved history is the intended push.

## Entry 26 — the fork's CI on the pushed sha

Fired as written — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
— after the push, returned 2026-10-10T10:01Z, exit 0. Its lines, verbatim:

```
ci v1.2 · 72093aa2
repo Turbolet85/escher (the push remote `origin`) · polled 31× over 929 s
e144b44d69f3 verdict: green · checks 16/16 · wall 908 s · runs CI#38042555355 completed/success
runs: CI#38042555355 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

**Verdict: green** — `exit 0`, `contains verdict: green`: CI#38042555355, 16 of 16 checks, on
`e144b44d69f36a981478371d487655949caaffde`. This run is the one witness of what the plan did not run locally: the
MSRV build, the windows build of the unix-gated files, and the macOS reading of the select-all modifier and of the
two shell flows (the unix-gated checks run on the macOS job).

**For the route's watch** (a fork CI job hanging in its package-install step): the run's log was read once,
`gh run view 38042555355 -R Turbolet85/escher --log`, 22,723 lines. Lines holding `apt-install`: 1,182 (the known
positive — the script's name is in the log). Lines holding `apt-install: attempt`: 0. The script prints that line
only for an attempt that failed, so no install bound was seen firing in this run, and no job was held: the run
completed in 908 s. Not read: per-job step timings.

## The pass

| Entry | Call | Verdict |
|---|---|---|
| 24 | `gate.py hygiene`, as written | green — `hygiene: clean` (after the one rewrite the operator named) |
| — | the operator pre-CI commit | `e144b44d` on `build/escher-0.1.0` |
| 25 | `gate.py run … --operator 25` | green — exit 0 · `history moved: refs/remotes/origin/build/escher-0.1.0 0493d26a→e144b44d` |
| 26 | `ci.py conclusion --sha HEAD --wait 1800`, as written | green — `verdict: green · checks 16/16`, CI#38042555355 |

No entry read red. The tree holds one uncommitted file after the pass: this record, as extended after the push.
