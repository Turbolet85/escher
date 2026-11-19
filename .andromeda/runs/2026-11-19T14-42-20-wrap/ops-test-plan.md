@row_set n=1 row=decision/local-pre-push-gate cell=rule seen=1 fact=F2 detector=none
value: The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate; on a dev host with CI unset, a green fast leg ends with the target sweep, whose refusal or failure never reddens the leg, and a red leg calls none.
basis: fct-yez7fafg, fct-bf6a7l62, fct-vk54jzyk
because: The fast leg gained the target sweep as its one owning caller after a green verdict, skipped under CI and never reddening it, so the gate's rule is stale by omission.
@row_add n=1 row=decision/target-sweep-safe-around-builds area=9-ci-integration fact=F3 detector=none
name: Target sweep safe around a build
rule: The target sweep removes only deps, fingerprint, build and incremental units of the debug build directory outside a closed keep list of locked cargo selections, keeps any unit newer than its keep pass, and refuses while a build holds the cargo build lock.
why: The lock is probed before the keep pass as well as held across removal, since the keep runs' own cargo would wait out a build in flight and then read the lock free.
guard: test_run_refuses_while_a_build_holds_the_lock
basis: fct-yez7fafg, fct-m3xx6xyj, fct-7mnpzgf5
because: The chunk adds the dev-host target sweep with its keep list and lock, a new rule future builds and legs must respect.
@row_add n=2 row=contract/target-sweep-invocation/verbs area=9-ci-integration fact=F4 detector=none
boundary: target sweep invocation
item: plan and run
promise: The sweep takes one verb, plan or run, from the repository root; any other argument is refused with exit 2 before anything runs.
basis: fct-yez7fafg, fct-5upx6ywi
because: The new sweep's argv is a contract its one owning caller and runs by hand rely on.
@row_add n=3 row=contract/target-sweep-invocation/environment area=9-ci-integration fact=F4 detector=none
boundary: target sweep invocation
item: environment and log
promise: The sweep reads no environment value of its own and writes no log.
basis: fct-yez7fafg, fct-5upx6ywi
because: The new sweep promises no environment read and no log, a contract at its boundary.
@row_add n=4 row=contract/target-sweep-stdout/line area=9-ci-integration fact=F4 detector=none
boundary: target sweep stdout
item: ending line
promise: One line of closed words and counts with no path: the verb, the outcome swept, planned, refused or failed, a cause on refusal or failure, and unit and byte counts and seconds on a sweep or plan.
basis: fct-yez7fafg, fct-why4n6no
because: The new sweep's stdout line is a closed format its owning leg prints as is.
@row_add n=5 row=contract/target-sweep-exit-codes/0 area=9-ci-integration fact=F4 detector=none
boundary: target sweep exit codes
item: 0
promise: Swept or planned.
basis: fct-yez7fafg
because: The new sweep's exit codes are a contract, one row a code.
@row_add n=6 row=contract/target-sweep-exit-codes/1 area=9-ci-integration fact=F4 detector=none
boundary: target sweep exit codes
item: 1
promise: Failed.
basis: fct-yez7fafg
because: The new sweep's exit codes are a contract, one row a code.
@row_add n=7 row=contract/target-sweep-exit-codes/2 area=9-ci-integration fact=F4 detector=none
boundary: target sweep exit codes
item: 2
promise: Usage: an argument other than plan or run, with nothing run.
basis: fct-yez7fafg
because: The new sweep's exit codes are a contract, one row a code.
@row_add n=8 row=contract/target-sweep-exit-codes/3 area=9-ci-integration fact=F4 detector=none
boundary: target sweep exit codes
item: 3
promise: Refused, the cause named on the stdout line.
basis: fct-yez7fafg
because: The new sweep's exit codes are a contract, one row a code.
@row_none n=1 fact=F1 type=0
what: a measured cause of the build directory's growth in cargo's own behaviour, and the four resolutions costed as a measurement only with nothing ruled; the sidecar holds it
@row_none n=2 fact=F5 type=5
what: the replayed day's measured byte counts; the sidecar holds them
@row_none n=3 fact=F6 type=9
what: an open measured limit, the CI-set rebuild under new hashes with its mechanism not found, owned by a CARRY on the route
@row_none n=4 fact=F7 type=0
what: how the chunk's two controls were read by property, evidence of this run; the sidecar holds it
