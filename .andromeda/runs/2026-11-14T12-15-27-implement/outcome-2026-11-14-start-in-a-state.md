# Implement outcome — 2026-11-14-start-in-a-state

**Outcome: green.** Smoke was skipped because no boot path changed. Next: the OPERATOR PASS (entries 6 hygiene, 7 push, 8 CI read), then `/andromeda-wrap-session`.

## Files changed
- New: `packages/escher-driver/src/fixture.rs`, `packages/escher-driver/src/fixture_tests.rs`, `tests/blitz-tests/tests/stand_start_in_state.rs`
- Modified: `packages/escher-driver/src/lib.rs`, `examples/seven_guis/src/stand.rs`, `tests/blitz-tests/tests/fixtures/mod.rs`, `docs/capabilities.md`
- Scope read (`.andromeda/runs/2026-11-14T12-15-27-implement/gate-2026-11-14-start-in-a-state.json`): `scope: clean — changed 7 · listed 7 · recorded 0`. There are no scope-record lines.

## Gates (1 iteration)
`entries 8 · green 5 · red 0 · recorded 0 · timeout 0 · not-run 3` (gate trail in this run dir).
- 1 `cargo test -p escher-driver --locked`: green
- 2 `cargo test -p blitz-tests --test stand_start_in_state --locked`: green (6 tests)
- 3 `cargo test -p seven_guis --locked`: green
- 4 `ci-leg.sh doc`: green
- 5 `ci-leg.sh fast`: green (245.67 s)
- 6, 7 and 8 are `leg operator`. They are not run and are left to the operator pass.

No gate was deferred. The plan has no `gate.py guard` entry.

## Remove-the-guard controls (`control-1.toml`, each case the test name alone per inputs#I3)
`control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`
- `reach-stop`: the early return on a refused step was removed. Case `a_step_the_screen_refuses_stops_the_reach_naming_its_index_and_cause`. Result: holds.
- `twelve-creates`: one Create was removed from `fourteen-people`. Case `every_named_state_reads_as_its_by_hand_path_from_boot`. Result: holds.

The lines are in `.andromeda/cache/chunks/2026-11-14-start-in-a-state/evidence/control-1.txt`.

## Matrix
`v030-08` changed from planned to implemented. Its ref is the three `stand_start_in_state` tests the acceptance names.

## Deviations from plan, with reasons
1. **`stand::start_in` checks the name before it boots.** Plan step 4 orders it `Session::start`, then `Fixture::named`, then `reach`. The scope says a name the table does not hold is "refused as a typed value with nothing started", and that order would boot first. I built name-first. As a result `start_in` returns no session for a refused name. So the test's clause "a session the refused name was asked for answers `snapshot` as a fresh boot does" is read on a held boot session, after `Fixture::named` refuses the name.
2. **The new check also declares `mod session_common;`.** The plan names only `mod fixtures;`. The check uses `session_common`'s `hold`, call builders and `state_dir`, as the other session checks do. This makes `stand_start_in_state` the 37th reader of `session_common`. The leaf `.claude/rules/testing.md` counts 36 readers, and the plan's expected amendments name only the fixtures module's count.
3. **One more unit test in `stand.rs`.** It is `every_fixture_state_name_is_of_the_label_form_and_named_once`. Plan step 1 says the label form is asserted by unit tests over the stand's table. The driver's crate cannot reach that table, so the test sits beside the table, in a listed file.
4. **`Fixture::steps()` is a public reader beside `name()`.** The recording test uses it to count the reach's kept steps.
5. **Extra driver unit tests.** `a_reach_stops_at_the_first_refused_step_naming_its_index_and_cause` runs on an empty page (`NotFound` at step 1). `only_an_unsettled_acting_outcome_reads_busy` is a 5-row table. No stand state ever reaches `ReachError::NotSettled`, so that table is its only witness.

## Master rows binding this work that the plan does not cite (per inputs#I3)
- I found no master row of that kind. One leaf count is affected and the plan does not name it: `testing.md`'s count of `session_common` readers (36, now 37; see deviation 2). A store search of test-plan, architecture and obs-plan for `session_common` read 0 hits, so I found no master row behind that count.

## Surfaced
- Nothing was surfaced as a spec↔reality gap.
- F1 (how a state crosses the carriers) stays the founder's, in `.andromeda/runs/2026-11-14T11-32-16-phase/founder-forks.md`. Option C was built as provisional. I raised no new fork in this run.

## Process hygiene
`census: none running` (comm cargo, rustc, escher-session, stand_start_in_state).

| process | started by | final state |
|---|---|---|
| cargo test / rustc (gates 1–5, control clone firings) | this run, through gate.py | terminated |

## Decisions & corrections
- The operator's directive for this run is snapped as inputs#I3: a fork goes to founder-forks.md with no dialog; a controls case is the test name alone; name an uncited binding master row; stop after the report. Applied as follows: the controls cases are bare test names, the master-row section is above, and there was no dialog.
- The Bash guard refused a `cd` into the inputs folder and a pipe of `rec.py find` into `head`. Each was re-issued once with an absolute path or run bare.
