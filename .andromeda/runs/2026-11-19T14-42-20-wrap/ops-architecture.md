@row_set n=1 row=decision/run-state-areas-local-only cell=rule seen=1 fact=F1 detector=D-arch-resources
value: The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job; the dev-host target sweep removes stale build units of the debug build directory alone and touches no state area.
basis: fct-yez7fafg
because: A new remover under target/ landed, the target sweep, which removes only stale debug build units, reads no environment value of its own and writes no log, so the row stating what is removed under target/ must name it.
@row_set n=2 row=contract/ci-leg-legs/fast cell=promise seen=1 fact=F8 detector=none
value: The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.
basis: fct-yez7fafg, fct-bf6a7l62, fct-rik5xvof
because: The fast arm gained a final sweep step after its five legs, skipped under CI and never reddening the leg, so the contract of what fast runs is incomplete.
@row_set n=3 row=decision/local-pre-push-gate cell=rule seen=1 detector=none
value: The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep, whose refusal or failure never reddens it.
basis: fct-yez7fafg, fct-bf6a7l62
because: The pre-push gate now ends with the target sweep as its owning caller, so the decision of what the gate runs must carry it.
@row_set n=4 row=contract/ci-leg-legs/embolden cell=promise seen=1 detector=none
value: The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last leg of `fast`, before the dev-host target sweep step.
basis: fct-yez7fafg, fct-smft3xp2
because: The fast arm now runs a sweep step after embolden, so embolden is no longer the last step of fast but stays its last leg.
@row_set n=5 row=decision/windows-entries-pass-through cell=rule seen=1 detector=none
value: The Windows entries of the agent-run and cold-agent pipes and of the target sweep are pass-throughs with no logic.
basis: fct-yez7fafg
because: A new Windows entry, target-sweep.ps1, landed as a pass-through handing every argument to the Python sweep and returning its exit code, so the convention's list of entries is incomplete.
@row_add n=1 row=contract/environment-variables/ci area=occupied-resources fact=F2 detector=D-arch-resources
boundary: Environment variables
item: CI
promise: CI is read by the CI leg runner's fast arm alone as a flag carrying no content: empty or unset runs the target sweep after a green verdict, any value skips it.
basis: fct-yez7fafg
because: The fast arm reads a new env name, CI, which the registry of environment variables must hold.
@row_add n=2 row=name/other/ci area=occupied-resources detector=D-arch-resources
class: other
name: CI
basis: fct-yez7fafg
because: CI became an env name the project reads, an occupied name the register must hold.
@row_add n=3 row=name/other/target-sweep-py area=occupied-resources fact=F3 detector=D-arch-resources
class: other
name: target-sweep.py
basis: fct-zd5gm7ps, fct-yez7fafg
because: The script name target-sweep.py is newly occupied by the dev-host target sweep.
@row_add n=4 row=name/other/target-sweep-ps1 area=occupied-resources fact=F3 detector=D-arch-resources
class: other
name: target-sweep.ps1
basis: fct-zd5gm7ps, fct-yez7fafg
because: The script name target-sweep.ps1 is newly occupied by the target sweep's Windows pass-through entry.
@row_add n=5 row=contract/target-sweep-invocation/verb area=standard-contracts fact=F4 detector=D-arch-resources
boundary: target-sweep.py invocation
item: verb
promise: The target sweep takes exactly one of plan or run, from the repository root; any other argv prints one usage line on stderr and exits 2 with nothing run.
basis: fct-yez7fafg
because: A new command line landed whose argv is a promise its callers rely on.
@row_add n=6 row=contract/target-sweep-exit-codes/0 area=standard-contracts fact=F4 detector=D-arch-resources
boundary: target-sweep.py exit codes
item: 0
promise: Swept or planned.
basis: fct-yez7fafg
because: The new sweep command promises a closed set of exit codes, one row an item.
@row_add n=7 row=contract/target-sweep-exit-codes/1 area=standard-contracts fact=F4 detector=D-arch-resources
boundary: target-sweep.py exit codes
item: 1
promise: Failed.
basis: fct-yez7fafg
because: The new sweep command promises a closed set of exit codes, one row an item.
@row_add n=8 row=contract/target-sweep-exit-codes/2 area=standard-contracts fact=F4 detector=D-arch-resources
boundary: target-sweep.py exit codes
item: 2
promise: Usage error, with nothing run.
basis: fct-yez7fafg
because: The new sweep command promises a closed set of exit codes, one row an item.
@row_add n=9 row=contract/target-sweep-exit-codes/3 area=standard-contracts fact=F4 detector=D-arch-resources
boundary: target-sweep.py exit codes
item: 3
promise: Refused.
basis: fct-yez7fafg
because: The new sweep command promises a closed set of exit codes, one row an item.
@row_add n=10 row=contract/target-sweep-stdout/line area=standard-contracts fact=F5 detector=D-arch-resources
boundary: target-sweep.py stdout line
item: line
promise: The target sweep prints one JSON line on stdout, a closed key set per outcome, and never prints a path or a unit name.
basis: fct-yez7fafg, fct-why4n6no
because: The sweep's stdout line is a new format its caller prints as is, a contract at its boundary.
@row_add n=11 row=contract/target-sweep-stdout/verb area=standard-contracts fact=F5 detector=D-arch-resources
boundary: target-sweep.py stdout line
item: verb
promise: verb is plan or run.
basis: fct-yez7fafg
because: The new stdout line carries a verb field of closed words.
@row_add n=12 row=contract/target-sweep-stdout/outcome area=standard-contracts fact=F5 detector=D-arch-resources
boundary: target-sweep.py stdout line
item: outcome
promise: outcome is one of swept, planned, refused or failed.
basis: fct-yez7fafg
because: The new stdout line carries an outcome field of closed words.
@row_add n=13 row=contract/target-sweep-stdout/cause area=standard-contracts fact=F5 detector=D-arch-resources
boundary: target-sweep.py stdout line
item: cause
promise: A refused or failed outcome adds cause, one of build-in-flight, no-build-directory, unsupported-host or keep-run.
basis: fct-yez7fafg
because: The new stdout line carries a cause field of closed words on a refusal or failure.
@row_add n=14 row=contract/target-sweep-stdout/counts area=standard-contracts fact=F5 detector=D-arch-resources
boundary: target-sweep.py stdout line
item: counts
promise: A swept or planned outcome adds units_kept, units_removed (run) or units_to_remove (plan), bytes_before, bytes_after and seconds.
basis: fct-yez7fafg
because: The new stdout line carries a closed set of count fields on a sweep or plan.
@row_add n=15 row=decision/target-sweep-refuses-during-build area=infrastructure-patterns fact=F6 detector=none
name: Target sweep refuses during a build
rule: The target sweep refuses while a build holds cargo's debug build lock, probing it non-blocking before the keep pass and holding it across the removal, and keeps any unit newer than the keep pass.
why: The keep runs' own cargo would wait out a build in flight and then read the lock free, so a sweep fired during a build would sweep rather than refuse.
guard: test_run_refuses_while_a_build_holds_the_lock
basis: fct-yez7fafg, fct-m3xx6xyj, fct-7mnpzgf5
because: A new dev-host remover of build units landed whose safety around a build is a decision future build tooling must respect.
@row_add n=16 row=decision/target-sweep-keep-set area=infrastructure-patterns fact=F7 detector=none
name: Target sweep keep set
rule: The target sweep keeps the units named by a closed list of seven locked, JSON-message cargo runs mirroring the fast leg's selections; build-script units are keyed by their build directory, incremental entries kept per crate by count, newest first.
why: Incremental names carry no cargo hash, and a build-script unit's hash stands only in its directory name.
except: A unit with no deps or build entry keeps its fingerprint.
basis: fct-yez7fafg, fct-lrempz7f, fct-gphr4c4m, fct-kzbdky47
because: The sweep's keep set is a new decision that a change to the fast leg's selections must follow.
@row_none n=1 fact=F9 type=9
what: The CI=true rebuild under new unit hashes is a measured limit carried open, its mechanism not found; the route's CARRY and test-plan's measured limit hold it.
