@row_add n=1 row=contract/target-sweep-cli/usage area=surface-cli fact=F1 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: usage
promise: target-sweep.py takes exactly one verb, plan or run, run from the repository root.
basis: fct-yez7fafg
because: The chunk adds the dev-host build-directory sweep command line, a new CLI surface whose usage no row of the cli surface states.
@row_add n=2 row=contract/target-sweep-cli/exit-2 area=surface-cli fact=F1 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: exit 2
promise: Any argv other than one verb, plan or run, prints one usage line on stderr and exits 2 with nothing run.
basis: fct-yez7fafg
because: The new sweep command line promises a usage exit that no row states.
@row_add n=3 row=contract/target-sweep-cli/stdout area=surface-cli fact=F2 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: stdout
promise: One JSON line of closed words and counts, no path: verb and outcome (swept, planned, refused, failed); cause on a refusal or failure (build-in-flight, no-build-directory, unsupported-host, keep-run); on sweep or plan units_kept, units_removed or units_to_remove, bytes_before, bytes_after, seconds.
basis: fct-yez7fafg, fct-why4n6no
because: The sweep's stdout line is a closed key set per outcome, a new output contract of the cli surface.
@row_add n=4 row=contract/target-sweep-cli/exit-0 area=surface-cli fact=F3 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: exit 0
promise: The sweep swept or planned.
basis: fct-yez7fafg
because: The new sweep command line's exit codes are a contract no row states.
@row_add n=5 row=contract/target-sweep-cli/exit-1 area=surface-cli fact=F3 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: exit 1
promise: The sweep failed.
basis: fct-yez7fafg
because: The new sweep command line's exit codes are a contract no row states.
@row_add n=6 row=contract/target-sweep-cli/exit-3 area=surface-cli fact=F3 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: exit 3
promise: The sweep refused, among its causes build-in-flight while a build holds the cargo lock.
basis: fct-yez7fafg, fct-7mnpzgf5
because: The new sweep command line refuses at exit 3, a contract no row states.
@row_add n=7 row=contract/target-sweep-cli/cargo-lock area=surface-cli fact=F3 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: cargo lock
promise: The sweep refuses build-in-flight while a build holds target/debug/.cargo-lock, and holds that lock itself across its removal.
basis: fct-yez7fafg, fct-m3xx6xyj
because: The sweep's lock behaviour is what makes it safe beside a build, a promise other work relies on.
@row_add n=8 row=contract/target-sweep-cli/windows-pass-through area=surface-cli fact=F4 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: target-sweep.ps1
promise: The Windows pass-through hands every argument to target-sweep.py and returns its exit code; the sweep itself refuses off unix with unsupported-host.
basis: fct-yez7fafg
because: The chunk adds the sweep's Windows pass-through, a contract no row states.
@row_add n=9 row=contract/target-sweep-cli/removal-scope area=surface-cli fact=F5 detector=D-layout-surface
boundary: target-sweep.py build-directory sweep
item: removal scope
promise: The sweep removes only units of target/debug deps, .fingerprint, build and incremental outside its closed keep list of seven locked cargo runs; it reads no environment value of its own, writes no log and touches nothing else under target/.
basis: fct-yez7fafg, fct-455immwu, fct-5upx6ywi
because: The sweep's reach under target/ and its absence of env reads and logs are promises at the new command line's boundary.
