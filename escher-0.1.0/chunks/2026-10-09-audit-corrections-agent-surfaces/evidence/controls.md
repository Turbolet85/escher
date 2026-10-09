# Mutation controls — 2026-10-09-audit-corrections-agent-surfaces

Hand record of the one-shot controls of plan steps 2, 4 and 6, run by /andromeda-implement on 2026-10-09 (UTC),
on the tree the chunk's edits left over base `6d626775`. Every mutation was reverted; the restore check of each is
stated with it. Excerpts only: `test … FAILED` lines, assertion messages and result lines.

## Step 2 — the surviving snapshot mutant is killed

The command, in all three runs (the audit's unit command for `dioxus-native-dom/snapshot`, the plan's second gate
entry):

```
cargo test --locked -p dioxus-native-dom -p blitz-tests --lib --test=stand_snapshot --test=stand_snapshot_state --test=stand_snapshot_text --test=stand_diff --test=stand_actionable_keys --test=stand_act_diff
```

**1. Unmutated tree (step 1's edits in place) — exit 0.** Eight result lines, 95 passed, 0 failed, 0 ignored
(94 at the audit and at the base; the prediction of 95 holds):

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test a_textarea_reads_back_typed_text ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**2. The mutation** — the single line `"textarea" => (true, false),` deleted at
`packages/dioxus-native-dom/src/snapshot.rs:187` (`git diff --stat -- packages`: 1 file changed, 1 deletion).
**The same command — exit 101.** The crate compiled; the check file ran its 8 tests; one line reads `FAILED`, the
new test, at its first value assertion (the boot reading `Some("")` against the mutant's `None`):

```
running 8 tests
test a_textarea_reads_back_typed_text ... FAILED
test the_fixture_controls_are_keyed_named_and_typed ... ok
test the_range_input_reads_the_apps_duration ... ok
test a_checkbox_and_a_radio_read_checked_after_a_click ... ok
test a_typed_password_never_appears_in_the_snapshot ... ok
test focus_reads_on_exactly_the_focused_control ... ok
test typed_text_reads_back_as_the_value ... ok
test a_disabled_control_reads_disabled_until_the_app_enables_it ... ok

thread 'a_textarea_reads_back_typed_text' panicked at tests/blitz-tests/tests/stand_snapshot_state.rs:514:9:
incremental=false: an empty textarea reads empty

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass `-p blitz-tests --test stand_snapshot_state`
```

As listed, cargo stops at the first failing target, so two of the eight result lines are not printed. The same
command with `--no-fail-fast` on the mutated tree (exit 101) prints all eight and shows no second failure:
0 · 6 · 3 · 9 · 8 passed, `FAILED. 7 passed; 1 failed`, 6 passed, 55 passed — `error: 1 target failed`.
`stand_snapshot_state.rs:514` is the `assert!` on `reads_value(&harness, "fx-notes", Some(""))`.

**3. The restore.** The line put back; `git diff --quiet 6d62677517b172e01764ab92b826d03ff75a7a11 -- packages`
exits 0. The same command — exit 0, the eight result lines of reading 1 again (95 passed),
`test a_textarea_reads_back_typed_text ... ok`.

A first restore edit did not apply (it named a neighbouring line that is not in the file); the restore check read
exit 1 and the command read the mutant's red a second time, which is how it was seen. The second edit applied and
the readings above are after it.

## Step 4 — the invariant read red on the untouched workflow

With `InstallScriptTest` and `test_o_package_installs_are_bounded` written and `.github/workflows/ci.yml` still at
the base:

```
python3 -m unittest discover -s .github/scripts -v -k InstallScriptTest -k test_o_package_installs_are_bounded
```

Exit 1 — `Ran 6 tests`, `FAILED (failures=11)`. The five script tests read `ok`. The invariant's failures: nine
sub-tests `'apt-get' unexpectedly found in …` (jobs build-msrv, build-features-default, test-features-default,
build-counter, clippy, ci-scripts, doc, a11y, coverage), one sub-test `<class 'NoneType'> is not <class 'int'>`
(job matrix_test, step `Install apt deps`: no `timeout-minutes`), and the method's own
`Items in the second set but not the first` (no job calls the script).

After step 5's edits: exit 0 — `Ran 6 tests`, `OK`; the whole discovery reads `Ran 70 tests`, `OK`.

## Step 6 — the script's and the invariant's controls

Each mutation applied to one file, the focused command above run, the file's bytes written back and its sha256
compared with the reading taken before the mutation (equal in all four).

| # | mutation | file | exit | what read red |
|---|---|---|---|---|
| 6.1 | the attempt loop's bound `attempt <= ATTEMPTS` cut to `attempt <= 1` | `.github/scripts/apt-install.sh` | 1 | `test_a_failed_attempt_is_tried_again_from_update ... FAIL` — `AssertionError: 1 != 0`; also `test_a_stalled_call_ends_at_its_bound_and_is_tried_again` (`['update'] != ['update', 'update']`) and `test_every_attempt_failing_exits_1_after_three_attempts` (`['update'] != ['update', 'update', 'update']`) — `FAILED (failures=3)` |
| 6.2 | `bounded update` replaced by the bare `sudo apt-get update` | `.github/scripts/apt-install.sh` | 1 | `test_a_stalled_call_ends_at_its_bound_and_is_tried_again ... FAIL` — `AssertionError: the script was still running after 30 s`; `Ran 6 tests in 30.112s`, `FAILED (failures=1)` |
| 6.3a | the build-msrv step put back to `sudo apt-get update && sudo apt-get install -y libfontconfig1-dev` | `.github/workflows/ci.yml` | 1 | `test_o_package_installs_are_bounded ... FAIL` — the sub-test `'apt-get' unexpectedly found in …` and the method's set comparison; `FAILED (failures=2)` |
| 6.3b | the line `timeout-minutes: 10` removed from the matrix job's `Install apt deps` step | `.github/workflows/ci.yml` | 1 | `test_o_package_installs_are_bounded` sub-test (job matrix_test) `... FAIL` — `<class 'NoneType'> is not <class 'int'>`; `FAILED (failures=1)` |

Control 6.1 turns three tests red, not test 2 alone: tests 3 and 4 also pin the number of attempts. Control 6.2's
red is the test's own wait bound: it killed the script's process group, and a process census after the run found
no `sleep` of the stand-in left.

**After the four restores:** the focused command exits 0 — `Ran 6 tests`, `OK`, all six lines `ok`;
`.github/scripts/apt-install.sh` reads mode 755.
