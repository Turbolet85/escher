@row_add n=1 row=decision/target-sweep-argv-validation area=input-validation fact=F1 detector=D-security-input
name: Build-directory sweep takes plan or run alone
rule: The build-directory sweep takes only plan or run as its one argument; anything else prints one usage line on stderr and exits 2 with nothing run.
guard: test_bad_argv_exits_two_with_nothing_run
basis: fct-yez7fafg, fct-5upx6ywi
because: The chunk adds the sweep's argv as a new input surface, validated as a closed set before acting, and no row states it.
@row_add n=2 row=contract/target-sweep-cli/0 area=input-validation fact=F2 detector=none
boundary: target-sweep.py exit codes
item: 0
promise: The sweep swept or planned, its one line on stdout.
basis: fct-yez7fafg
because: The sweep's exit codes are a new closed set at its boundary, one row an item.
@row_add n=3 row=contract/target-sweep-cli/1 area=input-validation fact=F2 detector=none
boundary: target-sweep.py exit codes
item: 1
promise: The sweep failed, its one line on stdout naming the outcome failed.
basis: fct-yez7fafg
because: The sweep's exit codes are a new closed set at its boundary, one row an item.
@row_add n=4 row=contract/target-sweep-cli/2 area=input-validation fact=F2 detector=none
boundary: target-sweep.py exit codes
item: 2
promise: Usage: an argv other than plan or run prints one usage line on stderr and nothing is run.
basis: fct-yez7fafg
because: The sweep's exit codes are a new closed set at its boundary, one row an item.
@row_add n=5 row=contract/target-sweep-cli/3 area=input-validation fact=F2 detector=none
boundary: target-sweep.py exit codes
item: 3
promise: The sweep refused, its one line on stdout naming the outcome refused and its cause, nothing removed.
basis: fct-yez7fafg
because: The sweep's exit codes are a new closed set at its boundary, one row an item.
@row_add n=6 row=decision/target-sweep-removes-only-stale-units area=input-validation fact=F3 detector=none
name: Build-directory sweep removes only stale debug units
rule: The build-directory sweep removes only stale cargo units under the debug build directory outside a keep set read from a closed list of locked cargo runs, keeps any unit newer than its keep pass, touches nothing else under the build directory, and refuses while a build holds cargo's lock.
why: The sweep must be safe during a build or refuse.
guard: test_state_areas_and_top_level_files_stand_byte_equal, test_run_refuses_while_a_build_holds_the_lock
basis: fct-yez7fafg, fct-rik5xvof, fct-7mnpzgf5, fct-455immwu
because: The chunk adds a tool that deletes files on the dev host, and its reach and its refusal under a held lock bind future work on it.
@row_none n=1 fact=F4 type=4
relied: The sweep's keep pass skips any line of cargo's JSON output that is not a JSON object.
@row_add n=7 row=decision/target-sweep-reads-no-env area=secret-management fact=F5 detector=none
name: Build-directory sweep reads no environment
rule: The build-directory sweep reads no environment value of its own.
basis: fct-yez7fafg, fct-5upx6ywi
because: The chunk adds a dev-host script, and the secret-management rows state each tool's environment reads.
@row_add n=8 row=contract/environment-variables/ci area=secret-management fact=F5 detector=none
boundary: Environment variables
item: CI
promise: A flag with no content, read only by the CI leg runner's fast arm and tested only for empty or unset; set, the fast arm runs no build-directory sweep.
basis: fct-yez7fafg, fct-nwx6efse
because: The chunk adds CI as a new environment read, and the plan names the environment values gaining it.
@row_add n=9 row=decision/target-sweep-line-closed-words area=logging-monitoring fact=F6 detector=none
name: Build-directory sweep prints closed words and counts
rule: The build-directory sweep writes no log, and its one stdout line holds closed words for verb, outcome and cause and counts, never a path, unit name or key.
guard: test_every_ending_prints_one_line_without_a_path
basis: fct-yez7fafg, fct-why4n6no, fct-nbk4uzan
because: The chunk adds a new output line with a closed key set and no log, which future work on the sweep must keep.
@row_add n=10 row=decision/target-sweep-closed-refusal-causes area=error-handling fact=F7 detector=none
name: Build-directory sweep refusal causes closed
rule: A build-directory sweep refusal or failure names its cause from a closed set: build-in-flight, no-build-directory, unsupported-host or keep-run.
basis: fct-yez7fafg
because: The chunk adds a closed refusal cause set at the sweep's stdout boundary.
@row_add n=11 row=decision/sweep-never-reddens-fast-leg area=error-handling fact=F7 detector=none
name: A refused or failed sweep never reddens the fast leg
rule: The fast leg runs the build-directory sweep only after its legs read green and discards the sweep's exit, so a refused or failed sweep never reddens it.
why: Ratified: the fast leg is the sweep's one owning caller, never reddened by it.
guard: FastLegOwnsTheSweepTest
basis: fct-yez7fafg, fct-rik5xvof, fct-bf6a7l62
because: The chunk wires the sweep into the fast leg with its exit discarded, a ruled error-handling choice no row states.
@row_set n=1 row=decision/locked-builds cell=rule seen=1 fact=F8 detector=D-security-deps
value: Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified.
basis: fct-yez7fafg, fct-3zqs2fds, fct-wktieqmc
because: The chunk adds cargo runs outside the CI legs, every one locked, so the locked-builds rule now reaches them.
