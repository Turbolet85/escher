# Implement outcome — 2026-11-14-auto-exploration

**Outcome: green**, with one plan premise surfaced. Smoke skipped: no boot path or UI surface changed. Exploration
is in-process library code, and entries 4–5 rebuilt and drove the `escher-session` binary.

## Files changed
New:
- `packages/escher-driver/src/explore.rs`
- `packages/escher-driver/src/explore_tests.rs`
- `tests/blitz-tests/tests/stand_explore.rs`

Modified:
- `packages/escher-driver/src/recording.rs` — adds `CHECK_FORMAT`; writes `"format":1`; reads it strictly; `fnv1a`
  is now `pub(crate)`
- `packages/escher-driver/src/recording_tests.rs`
- `packages/escher-driver/src/actions.rs` — `call_naming` and `target_verbs` are now `pub(crate)`
- `packages/escher-driver/src/lib.rs`
- `docs/capabilities.md`
- `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` — re-recorded by `record_checks.sh` (entry 4).
  The only difference is `"format":1` on each opening record.

`gate.py scope`: clean — changed 10 · listed 10 · recorded 0. No scope-record line was needed.

## Gates
Trail: `.andromeda/runs/2026-11-14T14-32-52-implement/gate-2026-11-14-auto-exploration.json`.
- First firing: entries 1–7 green, red 0. Entries 8–10 are `leg operator` and were not run.
- After the import fix below, entries 2 and 7 were re-run: both green (`2.2.log`, `7.2.log`).
- No deferrals. The block has no `gate.py guard` entry.

## Matrix
`v030-09` planned → implemented. The ref is the seven test names in `stand_explore.rs`, as written.

## Deviations from the plan, with justifications
1. **Counter test (step 8.2).** It is renamed `the_counter_is_explored_and_each_path_replays_green` and asserts
   `exhausted` **false**.
   - The plan's premise («the Counter, budget 200: `exhausted` reads true») is false. Every click on
     `counter-increment` shows a count no earlier screen showed, so a breadth-first run never empties its frontier.
   - The exhaustion witness moved to the finite refusing-control page (step 8.3, budget 2000). It asserts
     `exhausted` true.
   - SURFACED: `matrix#v030-09`'s acceptance lists `the_counter_is_explored_whole_and_each_path_replays_green`, a
     name that no longer exists. The behaviour that acceptance states is met as written. The wrap reconciles the
     test name in the acceptance.
2. **The planted panic is a unit `fn fail()`.** A Dioxus `onclick` closure whose body is just `panic!` has type
   `!`, which fails `SpawnIfAsync`.
3. **The image exclusion reads `FieldKind::Png`.** The plan says «the result field `png`». The selector reads the
   field's kind, not its name, and a unit test pins that (`explore_tests`, rows 2 and 4).
4. **The first screen is not a path.** It boots once, is not recorded and costs 0 steps. So `boots == paths + 1`
   (test 8.7), and the directory holds one check per path.
5. **Panic open question, answered.** A session whose step panicked is dropped under `catch_unwind`, and the next
   boot on the same thread answers a snapshot (test 8.4). No `mem::forget` was needed.
6. **More tests than the plan named.**
   - `recording_tests`: format rows `1.0`, `0`, `null` and `true`, beside the plan's 2, `"1"`, -1 and 1.5.
   - `stand_explore`: each kept check's calls equal its path's answered calls.
7. **Windows check.** A `cargo check --target x86_64-pc-windows-gnu` (testing rule 2026-10-19) found
   `SessionError` unused off unix in `stand_explore`. The import is now `cfg(unix)`, and the windows check reads
   no warning from that file.
8. **A stated limit added to `docs/capabilities.md`.** A path deeper than a kept check's 400 records ends its
   recording, and that check is removed.

9. **Controls the plan did not name, fired on the operator's word.** The word, after the first report: «write a
   controls file for stand_explore (remove-the-guard cases for the panic catch, the budget bound and the stop on a
   refused path; case = the test name alone), fire it by gate.py control, record it in evidence/ and name it as a
   deviation for the wrap». File: `.andromeda/runs/2026-11-14T14-32-52-implement/control-1.toml`. Record:
   `{work_dir}/evidence/control-1.txt`. Each control fired entry 2 in the tool's clone at HEAD 4b50320b:
   - `panic-catch`: `run_caught` runs the step with no `catch_unwind`. **holds** (cases
     `a_panicking_handler_is_caught_and_the_exploration_goes_on`,
     `a_panicking_paths_kept_check_holds_the_steps_before_it_and_replays_green`).
   - `budget-bound`: a path is taken while it fits twice the budget. **holds** (case
     `the_budget_bounds_every_acting_call`). This loosens the bound rather than deleting it, so the removed run stays
     bounded: with no bound at all, Cells' 2,034 first-level paths would all run.
   - `refused-stop`: a path ending `Refused` is expanded, while the accepted-screen dedupe stays. **holds** (case
     `a_refusing_control_is_met_and_reported_by_its_cause`). A refused call changes no screen, so the `Accepted`
     check alone is redundant with the dedupe: a refused path's screen is its parent's, which is already reached.
     The control therefore removes the stop for refused endings as a whole.
   - Summary: `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.

## Bound master row the plan does not cite
- `test-plan · decision/mutation-controlled-tests` (dec-7jkoc3jb): «A new check is controlled: a named mutation of
  what it guards turns it red».
- The plan names no control for `stand_explore`. That gap is now closed by deviation 9, on the operator's word.
- The new unit tests (`explore_tests`, the format rows in `recording_tests`) stay uncontrolled.

## Founder forks
None new. F1 and F2 (inputs#I4) stay on A, provisionally, as the plan holds them.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started outside the gate tool's entries | — | `census: none running` (escher-session, seven_guis, stand_explore, cargo) |

## Next
The operator pass fires entries 8 (hygiene), 9 (push) and 10 (CI read). After that, `/andromeda-wrap-session`.

## Decisions & corrections
- The operator's directive: a founder fork goes to `founder-forks.md`, with no dialog. No new fork arose.
- The operator's directive: in a controls file the case is the test name alone. No control was fired, so it was not
  exercised.
- The Bash guard refused a `cd` twice, into the skill references dir and the driver src dir. Each call was
  re-issued once with absolute paths.
- Standing learning, applied: the windows cross-check after new `cfg(unix)` gates (testing.md 2026-10-19) caught
  one off-unix unused import.
