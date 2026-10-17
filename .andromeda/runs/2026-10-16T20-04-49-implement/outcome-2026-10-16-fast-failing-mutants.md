# /implement outcome — 2026-10-16-fast-failing-mutants

**Outcome: surfaced** — a plan↔reality gap in the three mutation-run entries (5–7), which cannot pass as written.
On the operator's word it goes back to `/andromeda-phase` for a revision. Nothing was authored, and no spec, plan or
matrix was touched.

## The gap
- **Contract (plan.md step 7 and entries 5–7):** cargo-mutants 27.1.0 runs each file with the 0.1.0 audit's test set
  (`--test-package={unit},blitz-tests,…` and `--cargo-test-arg=--lib --cargo-test-arg=--test=…`) and "the default
  baseline".
- **Reality:** in cargo-mutants 27.1.0 the baseline tests only the packages it mutates. `lab.rs:179-190`
  (`run_baseline`) builds `PackageSelection::Explicit(all_mutated_packages)` and ignores `--test-package`, but it
  still passes every `--cargo-test-arg`.
  - Effect on entry 5: the baseline ran `cargo nextest run --package=seven_guis@0.1.0 --lib --test=cli_commands …
    --test=stand_timer`, and cargo refused it with ``no test target named `font_face_sniffed_source` in
    `seven_guis@0.1.0` package``. The baseline read `Failure(101)`, cargo-mutants exited 4 after 59.6 s, and no
    mutant ran.
  - Entry 5 line: `red · exit 0 ✗ (exit 4) · 59.59s · history: unmoved` (trail
    `.andromeda/runs/2026-10-16T20-04-49-implement/gate-2026-10-16-fast-failing-mutants.json`). Its `record.json`
    reads `[{"mutant":"baseline","summary":"Failure","test_s":0.603213522}]`, under
    `target/tmp/ffm-timer-20261016T200851Z` (72 KB, gitignored, kept for the revision).
  - Entries 6 and 7 would fail the same way: their target lists also name blitz-tests targets. They were not fired.
- **Why P3/P4 did not see it:** P3's six-mutant run and the audit both ran with `--baseline=skip`. Mutant runs, as
  opposed to the baseline, do carry every listed package: P3's log shows `--package=blitz-test-harness … --package=seven_guis`.
  The 491-pass nextest measurement was a direct `cargo nextest run`, not a run through cargo-mutants.
- **What the revision needs to decide:** for example, `--baseline=skip`, the audit's own form, with probe 9's "no
  baseline failed" clause re-read. That is the phase's call.

## Done and green (stands for the revised plan)
- Code (P1, 3 modified · 1 new):
  - `examples/seven_guis/src/tasks/timer.rs` — `#[cfg(test)] mod tests` appended after line 196,
    `a_delivered_tick_is_taken_once_and_an_empty_wait_parks` (a 3-row table, polled under `Waker::noop()`).
  - `packages/escher-driver/src/mcp_tests.rs` — `read_request_takes_each_line_with_its_terminator` (2 readers:
    the slice, and `BufReader` with capacity 1; at most len+1 reads).
  - `packages/dioxus-native-dom/src/dioxus_document_tests.rs` — `a_created_text_node_is_mapped_and_left_on_the_stack`,
    plus one `//!` clause and the `WriteMutations` import.
  - `.config/nextest.toml` — the `mutants` profile (fail-fast max-fail 1 with immediate terminate, 4 test threads,
    the three tests at priority 100).
- Step 6, the hand controls: `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/evidence/controls.md` has 6 rows.
  Each mutant was applied as `cargo mutants --list --diff` writes it and read red in 0.9–1.1 s (the crate recompiled,
  the named test FAILED on its own assertion or panic, exit 101). Each file was restored sha256-equal, then read green.
  The nextest command-line note step 7 asks for is not yet written, because no mutant ran.
- Gate lines (this run's trail):
  - 1 `green · 2.22s` · 2 `green · 1.57s` · 3 `green · 0.7s` · 4 `green · 25.64s` (`last line 3`)
  - 11 `green` (6 rows) · 12 `green` · 13 `green` · 14 `green · exit 1` · 15 `green · exit 1` · 16 `green · exit 1`
- Not run: 5 (fired, red, as above) · 6 and 7 (not fired after the gap) · 8–10 (they read step 7's records, which do
  not exist) · 17 a11y, 18 fast and 19 doc (left for the post-revision run) · 20–22 (the operator's).
- Matrix: no `ref` was written. v020-02's acceptance rests on the mutation runs.
- Scope read: `scope: clean — changed 4 · listed 4 · recorded 0`. There is no `scope-record.md`, because no edit
  fell outside research's lists.

## Deviations from plan
- Step 3 says "bound each loop by the expected count plus one, never by End". The loop makes at most `len + 1` reads
  and stops early at `End`. A read past `End` is not asserted; the read count is asserted equal to the table's.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `cargo test` ×12 (`controls.py`, 6 red + 6 green) | this run | terminated |
| `cargo test` / `cargo nextest list` (entries 1–4) | this run | terminated |
| `cargo mutants` + its `cargo nextest` (entry 5) | this run | terminated (exit 4); `target/mutants-tmp/` empty |

Measured by `ps -eo pid,comm,args` at 2026-10-16T20:12Z: 0 cargo-mutants, nextest or `controls.py` processes. The 2
rows that matched were the probe's own shell and grep.

## Decisions & corrections
- The operator's ruling at this run's halt, by question dialog: «Re-plan the entries (Recommended)». The mutation-run
  entries go back to `/andromeda-phase` for a revision, rather than being fired by hand with `--baseline=skip`.
- The Bash guard refused one `cd` into the skill's references dir (it would move the session cwd). Re-issued once with
  an absolute path.
- A standing learning recurred (host-linux «Processes», the handoff's deferred learning): the census grep counted its
  own probing shell (2 → 0 once the rows were read).

## Next
`/andromeda-phase`, whose pending guard offers the plan's revision for entries 5–7. After that, `/implement` again:
step 7, probes 8–10, the a11y, fast and doc legs, then the operator pass (20–22), then the wrap.
