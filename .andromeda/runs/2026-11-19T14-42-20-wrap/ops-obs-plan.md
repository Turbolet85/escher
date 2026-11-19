@row_add n=1 row=contract/target-sweep-line/line area=ci-integration fact=F1 detector=D-obs-pii
boundary: The build-directory sweep's stdout line
item: line
promise: Each ending of the dev-host build-directory sweep prints exactly one JSON line on stdout, a closed key set per outcome, holding no path, unit name or key.
basis: fct-yez7fafg, fct-why4n6no, fct-nbk4uzan
because: The chunk adds the sweep's stdout line as a closed key set, and obs-plan §9 has no row for it.
@row_add n=2 row=contract/target-sweep-line/verb area=ci-integration fact=F1 detector=none
boundary: The build-directory sweep's stdout line
item: verb
promise: Every line carries verb, one of plan or run.
basis: fct-yez7fafg
because: The sweep's line is a new closed contract, and its verb key is one item of it.
@row_add n=3 row=contract/target-sweep-line/outcome area=ci-integration fact=F1 detector=none
boundary: The build-directory sweep's stdout line
item: outcome
promise: Every line carries outcome, one of swept, planned, refused or failed.
basis: fct-yez7fafg
because: The sweep's line is a new closed contract, and its outcome key is one item of it.
@row_add n=4 row=contract/target-sweep-line/cause area=ci-integration fact=F1 detector=none
boundary: The build-directory sweep's stdout line
item: cause
promise: A refused or failed line adds cause, one of build-in-flight, no-build-directory, unsupported-host or keep-run.
basis: fct-yez7fafg
because: The sweep's line is a new closed contract, and its cause key is one item of it.
@row_add n=5 row=contract/target-sweep-line/counts area=ci-integration fact=F1 detector=none
boundary: The build-directory sweep's stdout line
item: counts
promise: A swept or planned line adds the counts units_kept, units_removed (run) or units_to_remove (plan), bytes_before, bytes_after and seconds.
basis: fct-yez7fafg
because: The sweep's line is a new closed contract, and its count keys are one item of it.
@row_add n=6 row=contract/target-sweep-exit-codes/0 area=ci-integration fact=F2 detector=none
boundary: The build-directory sweep's exit codes
item: 0
promise: Swept or planned.
basis: fct-yez7fafg
because: The chunk adds the sweep with a closed set of exit codes, and obs-plan holds none of them.
@row_add n=7 row=contract/target-sweep-exit-codes/1 area=ci-integration fact=F2 detector=none
boundary: The build-directory sweep's exit codes
item: 1
promise: Failed.
basis: fct-yez7fafg
because: The chunk adds the sweep with a closed set of exit codes, and obs-plan holds none of them.
@row_add n=8 row=contract/target-sweep-exit-codes/2 area=ci-integration fact=F2 detector=none
boundary: The build-directory sweep's exit codes
item: 2
promise: Usage: any argv other than plan or run prints one usage line on stderr, and nothing is run.
basis: fct-yez7fafg
because: The chunk adds the sweep with a closed set of exit codes, and obs-plan holds none of them.
@row_add n=9 row=contract/target-sweep-exit-codes/3 area=ci-integration fact=F2 detector=none
boundary: The build-directory sweep's exit codes
item: 3
promise: Refused.
basis: fct-yez7fafg
because: The chunk adds the sweep with a closed set of exit codes, and obs-plan holds none of them.
@row_add n=10 row=decision/target-sweep-touches-only-stale-units area=ci-integration fact=F3 detector=none
name: Build-directory sweep touches only stale units
rule: The build-directory sweep removes only stale units of target/debug's deps, .fingerprint, build and incremental areas; the run state areas and top-level files of target/ stand byte-equal.
guard: test_state_areas_and_top_level_files_stand_byte_equal
basis: fct-yez7fafg, fct-455immwu
because: The chunk adds a remover under target/ beside the scoped cleanups, and obs-plan §9 must state what it may touch.
@row_add n=11 row=decision/target-sweep-no-log-no-env area=ci-integration fact=F3 detector=D-obs-instrumentation
name: Build-directory sweep writes no log and reads no env
rule: The build-directory sweep writes no log and reads no environment value of its own; its one stdout line of closed words and counts is its only record.
why: No log by design; the stdout line carries the outcome.
basis: fct-yez7fafg, fct-nbk4uzan
because: The chunk adds an uninstrumented tool by design, and obs-plan must say so that its lack of a log is not read as drift.
@row_add n=12 row=decision/fast-leg-owns-target-sweep area=ci-integration fact=F4 detector=none
name: Local fast leg owns the build-directory sweep
rule: The local fast leg runs the build-directory sweep once after its five legs read green, on a dev host only, skipped when CI is set and non-empty; the sweep's line is printed as is and its exit discarded, so a refused or failed sweep never reddens the leg and a red leg calls none.
guard: FastLegOwnsTheSweepTest
basis: fct-yez7fafg, fct-bf6a7l62, fct-rik5xvof
because: The chunk gives the sweep one owning caller in the fast leg, and obs-plan §9 must say where its line is printed and that it never decides the leg.
@row_add n=13 row=decision/target-sweep-refuses-under-build-lock area=ci-integration fact=F5 detector=none
name: Build-directory sweep refuses under a build's lock
rule: The build-directory sweep refuses with cause build-in-flight while a build holds cargo's target/debug lock, probing the lock before its keep pass and holding it across the removal, so a second taker fails while the sweep holds it.
why: The keep runs' own cargo would wait out a build in flight and then read the lock free, so a sweep fired during a build would sweep rather than refuse.
guard: test_run_refuses_while_a_build_holds_the_lock, test_a_second_taker_fails_while_the_sweep_holds_the_lock
basis: fct-yez7fafg, fct-m3xx6xyj, fct-7mnpzgf5
because: The chunk adds a sweep that must be safe during a build, and obs-plan §9 must state when its refusal is the outcome.
