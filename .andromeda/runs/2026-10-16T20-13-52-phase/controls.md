# P5 check 4 (9) — the revision's baselines and controls, 2026-10-16-fast-failing-mutants

The revision adds three entries (5, 6, 7: each unit's mutant test command run directly on the unmutated tree) and
changes three (8, 9, 10: the mutation runs, `--baseline=skip` added; `leg = 'operator'`, baseline-exempt). Every other
entry's `run` and `expect` are unchanged, and each keeps the take-up's baseline
(`.andromeda/runs/2026-10-16T18-14-30-phase/controls.md`).

Baselines: each fired through `gate.py run --plan … --run-dir .andromeda/runs/2026-10-16T20-13-52-phase --marker
2026-10-16-fast-failing-mutants --entry {n}`, on the tree as the stopped /andromeda-implement run left it (its three
direct tests and `.config/nextest.toml` uncommitted on top of `494abd258fee55528006cfeaef68c145fd291de6`); logs
`$TMPDIR/andromeda-gate/2026-10-16-fast-failing-mutants/phase-2026-10-16T20-13-52/{n}.log`.

| entry | baseline | reading |
|---|---|---|
| 5 | green | exit 0, `201 tests run: 201 passed, 7 skipped`, 48.68 s |
| 6 | green | exit 0, `276 tests run: 276 passed, 7 skipped`, 28.76 s |
| 7 | green | exit 0, `494 tests run: 494 passed, 7 skipped`, 66.67 s |

The count atoms were copied from nextest's own Summary line on the same command, measured first at P3 of the revision
(`target/tmp/ffm-rev-baseline-{unit}.log`), so they share the run's representation.

Must-fail control of the entry form, typed beside entry 5 with the input operand alone changed (`L` set to a scratch
list `$TMPDIR/{scratchpad}/ctl-list.txt`, first line `seven_guis`, one line `--test=font_face_sniffed_source` — the
stopped run's failure), run as `/usr/bin/bash -o pipefail -c '…'` from the project root:

| control | input | reading |
|---|---|---|
| entry 5's form, must-fail | a target the selected package lacks | red, exit 101, `error: no test target named font_face_sniffed_source in seven_guis package`; the count atom absent |

The clone pair was not used: a `git clone --shared` lacks both the stopped run's uncommitted files and `target/`, so
the entry would read red there on the absent `mutants` profile, a different cause, and the operand-only typed form
reads the same command on the real tree.
