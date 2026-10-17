# P5 check 4 (9) — baselines and controls, 2026-10-16-fast-failing-mutants

Baselines: each `new = true` entry fired on the untouched tree at `494abd258fee55528006cfeaef68c145fd291de6` through
`gate.py run --plan … --run-dir .andromeda/runs/2026-10-16T18-14-30-phase --marker 2026-10-16-fast-failing-mutants
--entry {n}`, read by 2026-10-16T19:31Z; logs `$TMPDIR/andromeda-gate/2026-10-16-fast-failing-mutants/phase-2026-10-16T18-14-30/{n}.log`.

| entry | baseline | cause read from the log |
|---|---|---|
| 1 | red | `test result: ok. 1 passed` — the direct Timer test is step 2's |
| 2 | red | `test result: ok. 73 passed` — the direct framing test is step 3's |
| 3 | red | `test result: ok. 68 passed` — the direct writer test is step 4's |
| 4 | red | `error: profile `mutants` not found (known profiles: default, default-miri)`, exit 96 — the profile is step 5's |
| 8 · 9 · 10 | red | jq exit 2, `evidence/mutants-{timer,framing,writer}.json` absent — step 7 writes them |
| 11 | red | grep exit 2, `evidence/controls.md` absent — step 6 writes it |
| 12 · 13 | green | exit 0 — the two files as at the merge base and the base |
| 14 · 15 · 16 | green | exit 1, last line 0 — no change, no new file, no print |

Controls: each fired AS THE ENTRY through `gate.py run --plan … --entry {n} --project-root
$TMPDIR/{scratchpad}/clone`, no `--run-dir`, over `git clone --shared` of the project at HEAD
`494abd258fee55528006cfeaef68c145fd291de6`; each input written in the clone and reverted before the next; the clone
read `status clean` after each script. Scripts: `$TMPDIR/{scratchpad}/controls.py` (12–16), `controls2.py` (8–11).

| entry | clone input | reading |
|---|---|---|
| 12 | unmodified | green, exit 0 |
| 12 | must-fail: `mutation_writer.rs` line 100 edited | red, exit 1 |
| 13 | unmodified | green, exit 0 |
| 13 | must-pass: a `#[cfg(test)] mod tests {}` appended after line 196 | green, exit 0 |
| 13 | must-fail: `timer.rs` line 100 edited | red, exit 1 |
| 14 | unmodified | green, exit 1, last line 0 |
| 14 | must-fail: `tests/blitz-tests/tests/stand_timer.rs` edited | red, exit 0 |
| 14 | must-pass: `mcp_tests.rs` and `timer.rs` edited | green, exit 1, last line 0 |
| 15 | unmodified | green, exit 1, last line 0 |
| 15 | must-fail: a new `examples/seven_guis/src/control.rs` | red, exit 0 |
| 15 | must-pass: a new `.config/nextest.toml` | green, exit 1, last line 0 |
| 16 | unmodified | green, exit 1, last line 0 |
| 16 | must-fail: a `println!` line in `mcp_tests.rs` | red, exit 0 |
| 16 | must-pass: a print-free test module appended to `timer.rs` | green, exit 1, last line 0 |
| 8 | must-pass: the six named as cargo-mutants lists them, each a real `CaughtMutant` record (0.40 s) | green, last line 6 |
| 9 | must-pass: the same three records, baselines `Success` | green, last line 0 |
| 10 | must-pass: 137 non-baseline records over the three files | green, last line 137 |
| 8 | must-fail: the two framing mutants as P3's run really recorded them (`Timeout`) | red, last line 4 |
| 9 | must-fail: the same | red, last line 2 |
| 10 | must-fail: 136 non-baseline records | red, last line 136 |
| 8 | must-fail: one of the six caught at 599 s | red, last line 5 |
| 11 | must-pass: six rows in the plan's row form | green, last line 6 |
| 11 | must-fail: one row's restore `sha256 differs` | red, last line 5 |

Real records behind 8–10: the entries' own `jq` filter over two real `outcomes.json`. The first is P5's one-mutant run
inside inputs#I1's limits, `timer.rs:21:9` (`TimerTicks::deliver` → `()`), `-p seven_guis --test-package=seven_guis
--cargo-test-arg=--lib --timeout 600 -j 2`, taskset 12-15,28-31, output `target/tmp/ffm-p5-20261016T193004Z`.
cargo-mutants exited 0 and recorded `[{"mutant":"baseline","summary":"Success","test_s":0.751812995},{"mutant":"examples/seven_guis/src/tasks/timer.rs:21:9:
replace TimerTicks::deliver with ()","summary":"CaughtMutant","test_s":0.401303153}]`. The second is P3's framing run,
`target/tmp/ffm-p3/escher-driver`, two `Timeout` records.

Entry 4's must-pass reading is implement's first firing after step 5: the profile and the three tests it lists are
this chunk's. Its must-fail side is the absent profile (the baseline above), and its warning text was measured at P4
on cargo-nextest 0.9.146, where a scratch config with a misspelled `priorityy` read `ignoring unknown configuration
key` and the correct keys read 0 warnings (research.md §Open questions).
