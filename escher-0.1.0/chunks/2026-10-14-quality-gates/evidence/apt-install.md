# The bounded dpkg repair — CARRY 5, the operator's fork 4

## The change

`.github/scripts/apt-install.sh` gains `repair`, beside `bounded`: `sudo timeout --kill-after="$GRACE" "$BOUND"
dpkg --configure -a`, the same privilege, bound and kill grace as each `apt-get` call. `attempt_once` takes the
attempt's number and, on every attempt after the first, runs the repair first in phase `repair`; a failed
repair fails that attempt, and the existing diagnostic names it — `apt-install: attempt {n} of {N} failed in
repair (exit {status})`. The first attempt runs no repair, so a healthy install calls no `dpkg`. The usage
gate is unchanged and still exits 2 before `sudo`, `timeout`, `apt-get` or `dpkg` runs. The header comment
states the repair and why: a bound firing inside an install can leave dpkg interrupted.

## Its checks (`InstallScriptTest`, the `ci-scripts` leg)

A stand-in `dpkg` beside the stand-in `sudo` and `apt-get`, each call recorded in order in one file:

| case | calls recorded | exit · diagnostic |
|---|---|---|
| `test_a_healthy_install_runs_one_update_and_one_install` | `update` · `install -y …` | 0 |
| `test_a_healthy_install_runs_no_repair` (new) | no `dpkg` call | 0 |
| `test_a_failed_attempt_is_tried_again_from_update` | `update` · `dpkg --configure -a` · `update` · `install -y …` | 0 · `attempt 1 of 3 failed in update (exit 100)` |
| `test_an_install_failed_by_its_bound_is_repaired_before_the_next_attempt` (new) | `update` · `install -y …` · `dpkg --configure -a` · `update` · `install -y …` | 0 · `attempt 1 of 3 failed in install (exit 124)` |
| `test_a_failed_repair_fails_its_attempt_in_phase_repair` (new) | `update` · `dpkg --configure -a` | 1 · `attempt 2 of 2 failed in repair (exit 1)` |
| `test_every_attempt_failing_exits_1_after_three_attempts` | `update` · `dpkg …` · `update` · `dpkg …` · `update` | 1 · `attempt 3 of 3 failed in update (exit 100)` |
| `test_a_stalled_call_ends_at_its_bound_and_is_tried_again` | `update` · `dpkg --configure -a` · `update` | 1 · `attempt 2 of 2 failed in update (exit 124)`, inside 30 s |
| `test_a_bad_call_exits_2_before_anything_runs` | none (nine malformed calls) | 2 · `usage:` |

## The eight-not-nine witness — recorded, not changed

Read on CI#38649787488 (`eb5ba90c`), each job's log read whole through `gh run view -R Turbolet85/escher --job
{id} --log`: the script ran on real runners in eight jobs — Clippy, Test [default features], Documentation,
Accessibility (a11y) tests, MSRV Build, Build [default features], Build counter example, Coverage report.
In the ninth, `Test CI scripts`, the step `python3 -c 'import yaml' || bash .github/scripts/apt-install.sh
python3-yaml` opened at 22:45:33.908Z and the next step opened at 22:45:33.987Z with no line between: `import
yaml` succeeded (PyYAML is on the runner image), so the script was not reached. The guard stays as it is: it
is what lets the job run on an image without PyYAML. With this chunk's `Stand checks repeated` job the script
runs in nine jobs of ten that name it.
