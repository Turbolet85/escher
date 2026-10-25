# P5 controls — 2026-10-25-capability-statement

Baselines fired on the real tree through `gate.py run --entry {n} --run-dir … --marker …` (logs
`$TMPDIR/andromeda-gate/2026-10-25-capability-statement/phase-2026-10-25T02-58-45/`). Controls fired as the entry
through `gate.py run --entry {n} --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/60540bca-8160-4a4e-a17f-e7fd50efbefc/scratchpad/p5clone`
(a `git clone --shared` at e2eb01b44, its chunk folder holding copies of `plan.md`, `research.md` and `scope.md`, which
the guard reads), or — entries 7 and 8, whose paths lie outside the clone — typed in the header's shell
`/usr/bin/bash -o pipefail -c`, differing from the entry by the path operand alone.

| entry | real tree (baseline) | clone, unmodified | control input | reading |
|---|---|---|---|---|
| 1 unit pin | red — exit 5, NO TESTS RAN (file absent) | — | the suite this chunk writes: implement's red-before-green | vacuous now |
| 2 eleven gone | red — exit 0, count 1 (README.md:48) | — | must-pass: line 48 edited to twelve + the link | green — exit 1, 0 |
| 3 README link | red — exit 1, count 0 | — | must-pass: the same edit | green — exit 0, 1 |
| 4 secret search | green — exit 1, 0 over absent files (vacuous) | green, same | must-fail: `docs/capabilities.md` holding a home-directory path under the made-up user someone | red — last line 1 |
| | | | must-pass: both files present, clean | green — exit 1, 0 |
| 5 README numstat | red — empty numstat | red, same | must-pass: line 48 edited alone | green — `1 1` |
| | | | must-fail: line 50 edited too | red — `2 2` |
| 6 guard | green — `guard: clean`, changed 0 | green, same | must-pass: the three files changed/minted | green — `guard: clean` |
| | | | must-fail: `lint.rs` edited and `docs/stray.md` minted | red — `guard: outside 2 files` |
| 7 Viola pointer | red — exit 2, file absent | — (outside the clone) | must-pass: a file with the three lines (typed) | 3, exit 0 |
| | | | must-fail: a file with two lines (typed) | 2, exit 0 — red against `last line 3` |
| 8 pointer ignored | green — exit 0 | — (outside the clone) | must-fail: `viola-0.1.0/requirements.md`, a tracked Viola path (typed) | exit 1 |
