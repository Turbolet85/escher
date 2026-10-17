# P5 review — 2026-10-16-fast-failing-mutants (revision)

**Plan:** `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/plan.md` (8 steps, 25 gate entries, 8 criteria) ·
**research:** `research.md` · **scope:** `scope.md` (one `[premise-corrected]` bullet added from your word, inputs#I2)

## Why it stopped
cargo-mutants 27.1.0's baseline tests only the mutated package, but it still passes every `--cargo-test-arg`
(`lab.rs:179-190`, `cargo.rs:160-162`). So the audit's per-unit test set cannot be its baseline. The `seven_guis`
baseline was refused `no test target named font_face_sniffed_source`, and no mutant ran. No flag changes this:
`--test-workspace` moves only what a mutant tests. A mutant's own test phase does carry the whole `--test-package` list.

## What the revision changes
- **The three mutation runs (now entries 8–10)** gain `--baseline=skip`, the 0.1.0 audit's own form. Everything else
  is unchanged: the operator limits (inputs#I1), the nextest `mutants` profile, the audit's test set, the per-run
  output and the record.
- **Three new entries (5–7) run first.** Each runs its unit's mutant test command directly on the unmutated tree:
  `NEXTEST_PROFILE=mutants cargo nextest run --locked -p … --lib --test=…`, the package set and arguments cargo-mutants
  builds for a mutant. They carry the weight the skipped baseline drops. This matters because cargo-mutants reads any
  failing test phase as `caught`, nextest's "no tests run" included, so a test command that cannot pass would read every
  mutant caught. A red baseline entry stops step 7 before its run.
  - Baselined green: 201 · 276 · 494 tests run, all passed, 7 skipped each, 29–67 s.
  - Must-fail control: the same command, its list path pointed at a target the package lacks, reads red (exit 101).
- **Entry 12's note is re-read.** With the baseline skipped, the records hold mutant rows only, so `Failure` there is a
  mutant's unattributed failure. Its `run` and `expect` are unchanged.
- **Steps 1–6 are done and stand.** That covers the three direct tests, `.config/nextest.toml`, the six hand controls,
  and the entries the stopped run read green. Step 3's loop as built (at most count + 1 reads, early stop at `End`,
  read count asserted) is accepted. /andromeda-implement re-enters at step 7.
- **Rejected** (in the plan's Constraints):
  - the default baseline, which is impossible in 27.1.0;
  - dropping the `--test=` targets, which still leaves the baseline narrower than the mutants and loses the audit's set;
  - firing by hand outside the plan, which you rejected at implement's halt.
- **Expected amendment** (test-plan §10) grows by one clause: the recipe includes `--baseline=skip` and the unmutated
  read first.

## Concretization preview — v020-02 (already claimed; re-claimed on your word with a dated note)
**Current:** "Under cargo-mutants 27.1.0 with `--test-tool nextest` and `.config/nextest.toml`'s `mutants` profile (…), one
mutation run per file over the three files that hold the six timeout-only mutants (…, 137 mutants at 494abd25) — each
file against the 0.1.0 audit's test set for its unit, at `-j 2`, `taskset -c 12-15,28-31` and `--timeout 600` (the
operator's limits), exits 0: 0 mutants read missed or timeout; each of the six reads `CaughtMutant` with a test phase
under 120 s; and each of the six turns its named direct test red in a hand mutation control restored sha256-equal."

**Proposed:** the same, plus two clauses:
- the runs use `--baseline=skip`, and each unit's mutant test command is first read green on the unmutated tree (all
  tests passed);
- "0 mutants read missed or timeout" widens to "missed, timeout or an unattributed failure".

What moves, for your never-weaken check:
- **The baseline moves from cargo-mutants' own run to a direct read of the same command.** This is stronger, not
  weaker. The plan's default baseline could never have run, and the direct read tests the full package set the mutants
  run, where cargo-mutants' baseline would have tested the mutated package alone.
- **One more outcome word counts against the score** (`Failure`). This tightens the bar.

## Beyond the plan
none
