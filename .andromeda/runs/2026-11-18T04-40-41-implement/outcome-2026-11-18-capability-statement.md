# Outcome — 2026-11-18-capability-statement

**Outcome:** green. No smoke: no boot-path or UI-surface change.
**Next:** the OPERATOR PASS. Plan entries 9 (`gate.py hygiene`), 10 (the clean-tree push) and 11 (`ci.py conclusion --sha
HEAD --wait 2700`) are `leg = 'operator'`; this skill fires none of them. The window that runs the pass writes
`.andromeda/cache/chunks/2026-11-18-capability-statement/evidence/operator-pass.md`. Then `/andromeda-wrap-session`.

## Files changed
- `docs/capabilities.md`: steps 1–6. The page is at 0.3.0 and gains three sections: errors with the action, seeded
  randomness, and bundled fonts with one reading on every host. §The stand names the six served tasks and a component
  alone. In §Stated limits, the layout-carrier line is dropped and four 0.3.0 limits are added. §Accessibility reach
  reads 18 files and 18 · 163 · 0 · 0. The hidden-focus SC 2.4.3 gap is named, and the windowed witness's owner is now
  `res-llkgqygv`. `caused` is in the MCP carried set. The claim map reads seventeen targets, 64 files and
  273 · 1467 · 0 · 30, and names `WITNESSES_030`.
- `README.md`: step 8. `### 0.3.0 — built, its cold-agent run to come`, six built stage lines, and one line «16 of its
  17 capabilities are verified headlessly; …».
- `deny.toml`: step 7. The owner clause of RUSTSEC-2024-0436 now names `res-kdhnfj5p`, and those of
  RUSTSEC-2026-0370 · 0371 · 0372 name `res-p73cueyv`. IDs unchanged (6 ignores, the same 6 IDs).
- `.github/scripts/test_capability_statement.py`: step 9. `test_the_leg_counts_are_stated_in_words` with helpers
  `a11y_leg_files` and `stand_repeat_named_targets` (no `test_` prefix). It pins 17 and 18 as literals and as the
  page's phrases.
- `.claude/rules/a11y.md`, `.claude/docs/a11y-summary.md`, `.claude/docs/tests-summary.md`,
  `.claude/docs/services/seven_guis.md`: step 10, landed by `rec.py harness land` (tx 2020–2023).

## Deviations, with their justifications
1. **`WITNESSES_020` reads 49, not 46.** Step 6 asked for a method that first reads 46 on `WITNESSES_020`. At HEAD, no
   count of the table reads 46: it holds 49 witnesses. The method was therefore controlled at `e7e71221d`, the commit
   that wrote «46 witnesses» on the page, where it reads 46. Since `55bf34259` (2026-10-25-cold-agent-bar) the table has
   read 49. The page's 0.2.0 line now says 49, and `WITNESSES_030` reads 74 by the same method. Research's 43 came from
   a subject pattern that missed `Task(LeanTask::…)`. Evidence: `evidence/readings.md`.
2. **«Eight» became «Eleven».** The a11y files that also run in stand-repeat number 11 at HEAD, read from
   `ci-leg.sh`. The plan said to restate the overlap without giving the figure.
3. **The control's verdict is INDETERMINATE.** It reads `INDETERMINATE: case … no result line` because the tool's case
   grammar does not read unittest's `FAIL:` form. Read by hand, the removed log shows `FAIL:
   test_the_leg_counts_are_stated_in_words … AssertionError: 16 != 17`, and the in-place log shows `Ran 121 tests … OK`.
   Recorded in `evidence/control-1.txt`.
4. **README wording beyond a tense change.** Epoch 2's «the lint reading a select's options as overlapping» now reads
   «the lint no longer reading …», so that as a built fact it states the fix. Epochs 3 and 5 name the `graph`,
   `actions` and `layout` commands. «handler log» became «log», since `caused` carries any record. Epoch 6 drops the
   fresh-agent clause, which the closing line states as the last step.
5. **Taffy's git pin.** The page's stack sentence names Taffy as «upstream's git source, pinned to one revision», read
   from `Cargo.toml:105`.
6. **New limit lines.** Step 3's four limits were placed by standing. Under Pinned by a check: engine items on a
   tracing build (`stand_act_error_text`). Under Asserted by no check: `caused` echoing app text, and the buffered
   render's same-render limit (owner `res-433ttjb2`). Under Platform and carriers: a seed in process only.

scope record: none — `gate.py scope` clean, 0 recorded.

## Gates
- `bash .github/scripts/ci-leg.sh ci-scripts`: green · exit 0 · 11.3s
- `bash .github/scripts/ci-leg.sh audit`: green · exit 0
- `git grep … fifteen named|… -- docs/capabilities.md`: green · exit 1 · no output
- `git grep -n "residuals.md" -- deny.toml docs/capabilities.md README.md`: green · exit 1 · no output
- the secret search: green · exit 1 · no output
- the leaves stale-count probe: green · exit 1 · no output
- `gate.py guard … --base 6ebf42e65…`: green · `guard: clean`
- `bash .github/scripts/ci-leg.sh fast`: green · exit 0 · 379.81s
- `gate.py hygiene`, the push, `ci.py conclusion`: not run (`leg operator`), left for the operator pass.

The guard's pass line, read from entry 7's log (`gate.py show --n 7`):

    scope-record.md: absent — no companion is excused
    guard: allowed 4 — .github/scripts/test_capability_statement.py, README.md, deny.toml, docs/capabilities.md · companions excluded 0 · changed 190 · excluded 186
    guard: clean

Allowed: the four product touchpoints. The 186 excluded files are the pipeline's own artifacts and `.claude/`, which
the guard excludes by its own rule; entry 6 reads the four leaf pages instead.

The control (`.andromeda/runs/2026-11-18T04-40-41-implement/control-1.toml`, `--timeout 300`): with the guard in
place the tree reads green. With `--test open_state ` deleted from the stand-repeat list, the case reads `FAIL`
(16 != 17), and test_ci_workflows' own pin reads `FAIL` beside it. The tool reads 1 indeterminate (`no result line`).

## Surfaced
Nothing is a spec↔reality gap. Three leaf lines whose counts step 10 removed now state a membership that is false at
HEAD, since the a11y leg has run `select_popup`, `select_popup_painted` and `stand_act_popup` since
2026-11-18-a11y-re-read:
- `.claude/rules/a11y.md:42`: «none is among this leg's files»;
- `.claude/docs/tests-summary.md:21`: `stand_act_popup` «not among the `a11y` leg's files»;
- `.claude/docs/tests-summary.md:21`: «none of the `a11y` leg's files», for the select checks.

Step 10 removes counts only (inputs#I2) and re-states nothing, so these are left for the wrap's leaves pass.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (the `fast` and `audit` legs, the control's clone) | this run | terminated (census: no `cargo` or `rustc` live) |
| python3 hostwatch.py, python3 indicator.py, dbus-broker ×3, gnome-keyring-daemon | not this run (host and another session) | left running — not this run's |

Census: `gate.py census --comm cargo --comm rustc --comm python3 --args control`, which read `census: 6 running`,
none of them this run's.

## Decisions & corrections
- Guard refusal: `bash-guard.sh` refused `rec.py get … | sed` (a pipeline tool's output into a filter). Re-issued bare.
- Guard refusal: `bash-guard.sh` refused a `cat >> file <<EOF` append to a scratch script. Re-issued through the Edit
  tool.
- Guard refusal: `bash-guard.sh` refused `cd` into the skill's references dir. Re-issued with absolute paths.
- Sweep hazard: a witness-count regex over `stand_requirement_sweep.rs` that takes a subject as a bare capitalised word
  misses `Task(LeanTask::…)` subjects, reading 43 and 51 for 49 and 74.
- Sweep hazard: a count stated on the page («46 witnesses») can be stale against its own source with no leg reading it.
  The table grew after the page line was written, and only a count controlled at the writing commit separates the
  two.
- Instrument: `gate.py control` does not read Python unittest result lines, so a control on the `ci-scripts` leg can
  never read `holds`.
- Hygiene, on the operator's word (inputs#I4, «Placeholder it (Recommended)»): the first read refused
  `.andromeda/runs/2026-11-18T03-58-49-phase/p5-controls.md:16 ×1 · home`. That line held the phase's planted
  known positive, a made-up home path. Its literal was rewritten to the placeholder `/home/<user>/path`, nothing else
  changed. The snap's own `why` and `relay-1.md` quoted the literal and were placeholdered the same way. The re-read
  reads `hygiene: clean — read 195 …`.
