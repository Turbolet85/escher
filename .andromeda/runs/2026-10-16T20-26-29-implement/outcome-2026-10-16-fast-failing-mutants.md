# /implement outcome — 2026-10-16-fast-failing-mutants

**Outcome: green** (smoke skipped: no boot path or UI surface changed). This run re-entered the revised plan at
step 7, after the stopped run `.andromeda/runs/2026-10-16T20-04-49-implement/` and the revision
`.andromeda/runs/2026-10-16T20-13-52-phase/`. It took 0 fix iterations.

## What ran (trail `.andromeda/runs/2026-10-16T20-26-29-implement/gate-2026-10-16-fast-failing-mutants.json`)
- **Unmutated baselines first** (`--only 1-7`): 1–4 green, then 5 `green · 58.82s`, 6 `green · 29.1s` and
  7 `green · 83.7s`. Those are 201, 276 and 494 tests run, all passed.
- **The three mutation runs, each fired once** through `--operator {n}`:
  - 8 `green · exit 0 · 743.92s · history: unmoved`: timer.rs, 19 mutants, 18 caught, 1 unviable.
  - 9 `green · exit 0 · 1893.94s · history: unmoved`: mcp/framing.rs, 42 mutants, 39 caught, 3 unviable.
  - 10 `green · exit 0 · 2823.86s · history: unmoved`: mutation_writer.rs, 76 mutants, 69 caught, 7 unviable.
  - Each record was copied byte for byte (sha256-equal) to
    `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/evidence/mutants-{timer,framing,writer}.json`. The entry
    lines and logs are in `evidence/operator-mutation-runs.txt`.
- **The six**, by test-phase seconds:
  - timer.rs:29:9 2.47 s · timer.rs:32:31 `+=` 2.87 s · timer.rs:32:31 `/=` 2.62 s;
  - framing.rs:47:53 `-` 2.62 s · framing.rs:47:53 `*` 2.72 s;
  - mutation_writer.rs:172:9 2.97 s.
  - Every one reads CaughtMutant. Over all 137 the longest test phase was 73.33 s.
- **The whole block:** `entries 25 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 6`. The 6 not run are the
  operator entries 8–10 and 23–25.
  - Probes: 11 `last line 6`, 12 `last line 0`, 13 `last line 137`, 14 (controls, 6 rows) green, 15–19 green.
  - Legs: 20 a11y `green · 11.99s`, 21 fast `green · 131.33s`, 22 doc `green · 28.12s`.
- **Deferrals:** none. Rust delta is 3 files, and every changed-surface gate ran.

## Matrix
v020-02 is now `planned → implemented`. Its ref is the three evidence records read by entries 11–13, after the
baseline entries 5–7, plus the hand controls and the three direct tests (`matrix-2026-10-16-fast-failing-mutants.json`
trail). Every clause of the acceptance was read against the records as written.

## Files changed (this run)
- `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/evidence/controls.md` — step 7's note on the nextest command
  line, read from the first timer mutant's log. It carries `--package=blitz-tests` and `--package=seven_guis`,
  `--lib`, and the 50 `--test=` arguments of the unit's list, under `nextest profile: mutants` with 201 tests started.
  The note adds no table row.
- `evidence/mutants-{timer,framing,writer}.json` (new) — the three records.
- `evidence/operator-mutation-runs.txt` (new) — the operator entry lines with their logs.
- `escher-0.2.0/verification-matrix.json` — v020-02's ref and status.
- The stopped run's code stands unchanged: the three test edits and `.config/nextest.toml`.
- Scope read: `scope: clean — changed 4 · listed 4 · recorded 0`. There is no `scope-record.md`.

## Deviations from plan
- None in code.
- Step 3's loop as built (at most count + 1 reads) was accepted by the revision.
- Step 7 asks to record the command line in `controls.md`. It was written as a prose section after the table, so
  entry 14's row count stays 6.
- One observation, which no criterion reads: under `priority = 100` the direct timer test was the fourth START line of
  the first batch of four (counter 1/201), not the first line. It failed in 0.028 s.

## Left on disk (gitignored, for the operator to clear; the host refuses a recursive `rm`)
- This run's outputs: `target/tmp/ffm-timer-20261016T203018Z` 724K · `target/tmp/ffm-framing-20261016T204318Z` 2.9M ·
  `target/tmp/ffm-writer-20261016T211508Z` 5.7M.
- Earlier runs left `ffm-timer-20261016T200851Z` 72K, `ffm-p3`, `ffm-p5-*` and `ffm-rev-*` logs.
- `target/mutants-tmp/` is empty.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `cargo test` / `cargo nextest` (entries 1–7, block re-run) | this run | terminated |
| `cargo mutants` + its nextest children ×3 (entries 8–10) | this run | terminated (each exit 0); `target/mutants-tmp/` empty |
| `ci-leg.sh` a11y · fast · doc (entries 20–22) | this run | terminated |
| bash watch loop pid 2983036 (`waitm.sh escher-builder`, polls `pgrep cargo-mutants`) | not this run: the operator's overseer session | left running: not this run's; its owner stops it |

Measured by `ps -eo pid,comm,args` at 2026-10-16T22:08:48Z. On the `comm` column there are 0 `cargo-mutants` and 0
nextest processes. Of the other rows that matched, 2 were the probe's own shell and grep, and 1 was the watch loop above.

## Decisions & corrections
- Guard refusal: the Bash guard refused one `cd` into the skill's references dir, because it would move the session
  cwd. The call was re-issued once with absolute paths.
- A standing learning recurred (host-linux «Processes»): the census grep over `args` matched its own probing shell and
  grep. It was read on the `comm` column.

## Next
The operator pass: entry 23 (`gate.py hygiene`, before the pre-CI commit), entry 24 (fast, the clean-tree guard and
the push), and entry 25 (`ci.py conclusion` on the pushed sha). Then `/andromeda-wrap-session`.
