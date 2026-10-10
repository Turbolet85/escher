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

## The operator pre-CI commit

On the operator's word — "then the commit "chore(2026-10-10-snapshot-states): operator pre-CI commit, for the run
this chunk verdict reads"" — the operator, 2026-10-10. `git add -A`, then the commit, on `build/escher-0.1.0`:

- sha `2fc3282965443aac2b7ff4d70729031ff1b0f945`, parent `f0cb9dd733c00f8effec29b242abd152eed486f3`
- subject `chore(2026-10-10-snapshot-states): operator pre-CI commit, for the run this chunk verdict reads`
- 72 files changed, 7,916 insertions, 39 deletions; the tree read clean after it (0 rows of `git status --short`)

It carries the chunk's source and checks, its chunk folder with this record as it stood at entry 18, the phase's and
the implement's run dirs, and the route, master, handoff and ledger edits earlier steps left uncommitted. This
record's sections from here down were written after the push and are not in that commit.

## Entry 19 — the local pre-push gate, the clean-tree guard, the push

Fired once through the gate tool, as the gate contract names for an operator entry —
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan escher-0.1.0/chunks/2026-10-10-snapshot-states/plan.md --operator 19`
— with no run dir (the pass runs under no skill), 2026-10-10T17:32Z. The tripwire line, the entry's line and the
call's summary line, verbatim:

```
operator entry 19 · history tripwire: git
 19 probe       green · exit 0 · 96.89s · 84217 B → 19.log · history moved: refs/remotes/origin/build/escher-0.1.0 f0cb9dd7→2fc32829 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```

The entry carries no `expect` key, so its line asserts the exit alone; what the report reads of its output, copied
from its log:

- the `fast` leg on the committed tree: 167 result lines · 791 passed · 0 failed · 11 ignored; the CI scripts
  `Ran 78 tests` · `OK`
- the push: `To https://github.com/Turbolet85/escher.git` · `f0cb9dd7..2fc32829  build/escher-0.1.0 -> build/escher-0.1.0`

**The history reading:** `history moved: refs/remotes/origin/build/escher-0.1.0 f0cb9dd7→2fc32829` — the one ref
the push was meant to move, and no other. Read back after the call: `git ls-remote origin build/escher-0.1.0`
answers `2fc3282965443aac2b7ff4d70729031ff1b0f945`, equal to `HEAD`; the branch reads 0 ahead of its remote and the
tree clean.

**Verdict: green** — `exit 0`; the moved history is the intended push.

## Entry 20 — the fork's CI on the pushed sha

Fired as written — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
— after the push, started 2026-10-10T17:34Z, returned 2026-10-10T17:47Z, exit 0. Its lines, verbatim:

```
ci v1.2 · 72093aa2
repo Turbolet85/escher (the push remote `origin`) · polled 26× over 774 s
2fc328296544 verdict: green · checks 16/16 · wall 767 s · runs CI#38072258834 completed/success
runs: CI#38072258834 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

**Verdict: green** — `exit 0`, `contains verdict: green`: CI#38072258834, 16 of 16 checks, on
`2fc3282965443aac2b7ff4d70729031ff1b0f945`. This run is the one witness of what the plan did not run locally: the
MSRV build, and the windows, macos, ios and android legs over the builder, the model and the two new check files —
neither new check file carries a `cfg(unix)` gate, so the windows and macos jobs compile and run them. Not read: the
run's log, per-job timings, or which tests each mobile job executed.

## The pass

| Entry | Call | Verdict |
|---|---|---|
| 18 | `gate.py hygiene`, as written | green — `hygiene: clean`, no row, on all three readings |
| — | the operator pre-CI commit | `2fc32829` on `build/escher-0.1.0` |
| 19 | `gate.py run … --operator 19` | green — exit 0 · `history moved: refs/remotes/origin/build/escher-0.1.0 f0cb9dd7→2fc32829` |
| 20 | `ci.py conclusion --sha HEAD --wait 1800`, as written | green — `verdict: green · checks 16/16`, CI#38072258834 |

No entry read red. The tree holds one uncommitted file after the pass: this record, as extended after the push.
