@task_prose part=session
The session measured why the dev host's target directory grows and mended it with a stale-hash sweep of target/debug units.
Measured cause: a source edit adds no cargo unit hash; a lockfile move adds one per unit above the moved crate (184 of 385 hashes); a replayed working day grew +57.90 GB unmended and -2.04 GB mended.
Added a dev-host sweep script with plan and run verbs, a closed keep list of seven locked cargo selections, a non-blocking probe of the cargo build lock, and one stdout line of closed words and counts.
The fast CI leg now ends, on a dev host only and after its five legs read green, with the sweep; it is skipped when CI is set, and a refused or failed sweep never reddens it.
Added a 16-case unittest file to the ci-scripts leg; no Rust crate, manifest, lockfile or source changed.
Measured limit left open: running the fast leg with CI=true recompiled 42 packages under new hashes (about 48 GB); the mechanism is not found.
@task_file n=1 path=scripts/target-sweep.py
what: added: the dev-host target-directory sweep, removing stale build units under target/debug outside a closed keep list of locked cargo selections, refusing while a build holds the cargo lock, printing one line of closed words and counts
@task_file n=2 path=scripts/target-sweep.ps1
what: added: the Windows entry of the target-directory sweep, passing every argument through; the sweep refuses off unix
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: added: the ci-scripts leg's unittest contract for the sweep and for the fast leg owning it, 16 cases in 6 test cases, red-first controlled
@task_file n=4 path=.github/scripts/ci-leg.sh
what: edited: the CI leg script, its fast arm gaining the sweep step after the five legs read green, skipped when the CI env value is set, its exit discarded
@task_item n=1 part=touches
decision/local-pre-push-gate: the local fast leg run before a push now ends with the target sweep after a green verdict on the dev host, skipped under CI, never reddening the gate; the row is stale by omission.
@task_item n=2 part=touches
decision/local-baseline: the local baseline gains the measured build-directory footprint, a working day's growth of +57.90 GB unmended against -2.04 GB mended.
@task_item n=3 part=touches
decision/ci-legs-through-leg-script and decision/ci-legs-host-reproducible: the leg script's fast arm now behaves differently on a dev host and under CI (the sweep runs only when CI is unset), and CI=true was measured to rebuild 42 packages under new hashes.
@task_item n=4 part=touches
decision/workspace-tests-in-fast-ci-leg and decision/ci-fast-slow-split: the fast leg gains a step after its five legs; the leg count stays five and the sweep is a step, not a leg.
@task_item n=5 part=touches
contract/ci-leg-exit-codes/2: the fast leg's exit status is unchanged by the sweep, whose own exit (0 swept or planned, 1 failed, 2 usage, 3 refused) is discarded.
@task_item n=6 part=touches
decision/cargo-legs-locked: every cargo call of the sweep's keep pass runs --locked with JSON artifact messages, a new caller of locked cargo outside the legs.
@task_item n=7 part=touches
decision/shared-test-cache and decision/ci-shared-test-cache: the shared workspace build cache on the dev host is now pruned of units outside the keep list after each green fast leg.
@task_item n=8 part=touches
decision/clippy-legs: the sweep's clippy keep lines carry the leg's -D warnings so the clippy units are not rebuilt in place; a change to the clippy legs' arguments now owes the keep list.
@task_item n=9 part=touches
decision/embolden-leg-local-only and decision/embolden-local-witness: the font-embolden witness selection is one of the seven kept selections, and the sweep runs after the embolden leg.
@task_item n=10 part=touches
decision/ci-scripts-unit-tested, decision/python-script-tests-under-unittest and decision/joining-file-keeps-its-legs: a new unittest file joins the ci-scripts leg, testing a script outside .github/scripts; no row holds the file count.
@task_item n=11 part=touches
decision/mutation-controlled-tests: two sweep defects and the fast-leg wiring were each fixed behind a red-first case, read red before and green after.
@task_item n=12 part=touches
decision/free-disk-on-test-and-stand-repeat: disk on the dev host is now bounded by the sweep; the CI jobs' disk freeing is unchanged and the sweep never runs in CI.
@task_item n=13 part=touches
contract/cold-agent-state/directory, contract/agent-run-state/files and contract/ci-artifacts/leg-log: state areas and leg logs under target/ stand byte-equal across a sweep, which touches only deps, fingerprint, build and incremental units under target/debug.
@task_item n=14 part=touches
decision/msrv-verified-by-build and decision/stand-repeat-three-rounds: rows naming fast-leg selections whose build units the keep list must hold; a new selection in the fast leg owes a keep line or it is rebuilt after each sweep.
@task_fact n=1 slug=9-ci-integration
fact: The growth of the dev host's target directory is caused by lockfile moves, each adding a unit hash per unit above the moved crate, while a source edit adds none; four separate resolutions of shared crates are costed as a measurement only and none is unified.
@task_fact n=2 slug=9-ci-integration
fact: On a dev host the fast leg ends, after its five legs read green and only when CI is unset, with a sweep of stale target/debug units outside a closed keep list of seven locked cargo selections; a refused or failed sweep never reddens the leg, a red leg calls none, and the sweep has this one owning caller beside runs by hand.
@task_fact n=3 slug=9-ci-integration
fact: The target sweep refuses while a build holds the cargo build lock, probing it before its keep pass and holding it across removal, keeps any unit newer than its keep pass, and touches nothing under target/ beyond deps, fingerprint, build and incremental units.
@task_fact n=4 slug=9-ci-integration
fact: The target sweep's contract: verbs plan and run only, any other argument refused with exit 2 before anything runs; one stdout line of closed words and counts with no path (verb, outcome swept, planned, refused or failed, a cause on refusal or failure, unit and byte counts and seconds on a sweep or plan); exit 0 swept or planned, 1 failed, 2 usage, 3 refused; no environment value read and no log written.
@task_fact n=5 slug=9-ci-integration
fact: A replayed working day of the legs grows the build directory by +57.90 GB unmended and by -2.04 GB with the sweep, every replayed leg green.
@task_fact n=6 slug=9-ci-integration
fact: Measured limit, open: running the fast leg with CI=true recompiles 42 packages under new unit hashes (about 48 GB of new units); the project source reads no CI value and the mechanism is not found.
@task_fact n=7 slug=9-ci-integration
fact: The sweep's controls are read by property: a refused sweep removed nothing during an in-flight cold build (whose own file count moves), and after a sweep the kept selections add no new unit entries, seven_guis and blitz-tests rebuilding in place on any alternation of selections with or without a sweep.
@task_item n=1 part=untouched
Driver, session, session wire, MCP role and escher-session command-line contracts, refusal causes and exit statuses: no Rust crate or binary changed.
@task_item n=2 part=untouched
Harness construction, settle, input, inspection, screenshot and contrast contracts, and the virtual clock and advance decisions: no harness code moved.
@task_item n=3 part=untouched
Cold-agent and agent-run contracts, verdicts, events and bars: those scripts and their tests are unchanged.
@task_item n=4 part=untouched
WPT runner, its CLI, statuses, outputs and pinned commit: not reached.
@task_item n=5 part=untouched
Coverage floors, coverage-floor script exit codes, mutation baseline and complexity ceiling, and the stand zero-failure budget: no floor, threshold or budget moved.
@task_item n=6 part=untouched
GitHub workflow decisions: actions pinned to SHAs, no secrets, read-only permissions, trigger branches, cache save branches, upstream-only jobs and the apt-install script with its exit codes: the workflow and the install script did not change.
@task_item n=7 part=untouched
Test data, fixtures, mocking discipline, stand boot and recorded-check decisions, and the windowed end-to-end absence: nothing of app tests or fixtures moved.
@task_item n=8 part=untouched
Lint defects, snapshot, handler coverage and coverage-line contracts: no engine or driver reading changed.
