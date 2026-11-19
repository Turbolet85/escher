@task_prose part=session
The session measured why the dev host's cargo target directory grows: a lock move adds a new hash per unit above the moved crate, a source edit adds none, and a replayed working day grew it by 57.90 GB unmended against 2.04 GB shrunk once mended.
It added a dev-host target sweep, a stdlib Python script with a pass-through PowerShell entry, that removes stale build units of target/debug outside a keep set read from seven locked cargo runs of the fast leg's selections.
The sweep refuses while a build holds cargo's build lock, refuses off unix, prints one JSON line of closed words and counts with no path, and exits 0 swept or planned, 1 failed, 2 usage, 3 refused.
The fast leg of the CI leg runner now ends with the sweep after its five legs read green on a dev host, skipped when the CI env value is set, the sweep's exit never reddening it; FAST_LEGS stays five.
A new unittest file joins the ci-scripts leg, pinning the sweep and the fast leg's ownership of it; no Rust crate, manifest, lockfile or dependency changed.
Measured and left open: running the fast leg with CI=true rebuilds about 48 GB of the tree under new hashes, its mechanism not found.
@task_file n=1 path=scripts/target-sweep.py
what: the dev-host target sweep (new) - the plan and run command line that removes stale cargo build units of target/debug outside a keep set, under cargo's build lock, printing one JSON line
@task_file n=2 path=scripts/target-sweep.ps1
what: the Windows script entry for the target sweep (new) - a pass-through that hands every argument to the Python sweep and returns its exit code
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the ci-scripts leg's unittest contract file for the target sweep (new) - its argv, plan, run, lock, endings and the fast leg owning the sweep
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the one CI leg runner (edited) - its fast arm gains a final step running the target sweep after the five legs read green, skipped when CI is set, the sweep's exit discarded
@task_item n=1 part=touches
contract/ci-leg-legs/fast is stale by omission: the fast leg now ends with the target sweep after its five legs read green on a dev host, skipped under CI, never reddening the leg; its five legs are unchanged.
@task_item n=2 part=touches
decision/local-pre-push-gate: the local pre-push gate (the fast leg) now also sweeps stale target/debug units after a green verdict on the dev host, skipped under CI.
@task_item n=3 part=touches
decision/run-state-areas-local-only and decision/agent-state-areas-local: a new remover stands beside the state areas - the sweep removes target/debug build units alone and leaves every state area and top-level file byte-equal.
@task_item n=4 part=touches
The name/other env rows (RUST_LOG, TMPDIR, CARGO_TARGET_TMPDIR and the like): CI is a newly read env name, read by the fast arm of the CI leg runner alone as a flag carrying no content.
@task_item n=5 part=touches
contract/ci-leg-script/exit-status and contract/ci-leg-script/leg-log: the fast leg's exit status and printed log gain the sweep's one line, while a refused or failed sweep leaves the exit status at the legs' verdict.
@task_item n=6 part=touches
decision/one-ci-leg-runner: the one leg runner gains a step that is not a leg - the sweep runs inside the fast arm, so FAST_LEGS stays five.
@task_item n=7 part=touches
decision/windows-entries-pass-through: a new Windows script entry joins the pass-throughs, the sweep's PowerShell entry, while the sweep itself refuses off unix as unsupported-host.
@task_item n=8 part=touches
decision/ci-scripts-stdlib-python, decision/ci-scripts-python-floor and decision/ci-python-unittest: a new stdlib-only Python script and a new unittest file join the ci-scripts leg under these rules; any row cell counting the leg's unittest files moves by one.
@task_item n=9 part=touches
decision/locked-cargo-legs and decision/cargo-legs-locked: every cargo call of the sweep's keep pass runs locked with JSON messages, extending the locked rule beyond the legs to the sweep.
@task_item n=10 part=touches
decision/ci-contract-pinned: the CI leg runner's fast arm owning the sweep is now pinned by unittest cases (the sweep runs once after the embolden leg, never under CI, never after a red leg, never reddening fast).
@task_item n=11 part=touches
decision/raw-run-output-never-printed: the sweep discards its cargo runs' stderr and prints only closed words and counts, no path or unit name.
@task_item n=12 part=touches
contract/ci-leg-legs/ci-scripts: the ci-scripts leg gains a unittest file for the sweep; a touch only where the row holds the leg's file list or count.
@task_item n=13 part=touches
decision/one-debuginfo-level and decision/dev-line-tables-debuginfo, indirectly: measured that a dev-host fast leg run with CI=true rebuilds 42 packages under new unit hashes, about 48 GB, so dev-host and CI-flagged builds do not share units; the mechanism is not found.
@task_fact n=1 slug=occupied-resources
fact: The dev-host target sweep removes stale build units of target/debug alone (deps, .fingerprint, build and incremental entries outside the keep set and older than its keep pass) and touches nothing else under target; it reads no environment value of its own and writes no log.
@task_fact n=2 slug=occupied-resources
fact: CI is an env name read by the CI leg runner's fast arm alone, a flag carrying no content: empty or unset runs the target sweep after a green verdict, any value skips it.
@task_fact n=3 slug=occupied-resources
fact: The script names target-sweep.py and target-sweep.ps1 are occupied by the dev-host target sweep and its Windows pass-through entry.
@task_fact n=4 slug=standard-contracts
fact: The target sweep's command line takes exactly one of plan or run from the repository root; any other argv prints one usage line on stderr and exits 2 with nothing run; exits are 0 swept or planned, 1 failed, 2 usage, 3 refused.
@task_fact n=5 slug=standard-contracts
fact: The target sweep prints one JSON line on stdout with verb (plan or run) and outcome (swept, planned, refused or failed); refused or failed add cause (build-in-flight, no-build-directory, unsupported-host or keep-run); swept or planned add units_kept, units_removed or units_to_remove, bytes_before, bytes_after and seconds; no path or unit name is ever printed.
@task_fact n=6 slug=infrastructure-patterns
fact: The target sweep refuses while a build holds cargo's target/debug build lock, probing it non-blocking before the keep pass and holding it across the removal, and keeps any unit newer than the keep pass.
@task_fact n=7 slug=infrastructure-patterns
fact: The target sweep's keep set is the units named by a closed list of seven cargo runs mirroring the fast leg's selections, each run locked with JSON messages; build-script units are keyed by their build directory, incremental entries are kept per crate by count newest first, and a unit with no deps or build entry keeps its fingerprint.
@task_fact n=8 slug=infrastructure-patterns
fact: The target sweep has one owning caller, the fast leg on a dev host after a green verdict, skipped under CI; a refused or failed sweep never reddens the leg and a red leg calls no sweep; it is also run by hand.
@task_fact n=9 slug=infrastructure-patterns
fact: Measured limit: a fast leg run with CI=true on the dev host rebuilds the tree under new unit hashes (42 packages, about 48 GB) though no project source reads CI; the mechanism is not found and the question is carried open.
@task_item n=1 part=untouched
Engine design, layout, style, paint, scrolling, select-list and event-dispatch decisions and contracts: no Rust crate or module changed.
@task_item n=2 part=untouched
Driver, session, MCP role, snapshot, verb, refusal-cause, kept-check and recording contracts: the session reached no driver or session surface.
@task_item n=3 part=untouched
Stack and dependency choices, workspace manifest, lockfile, toolchain pin and Nix flake rows: no dependency, manifest or lockfile changed, and the sweep uses only the Python standard library.
@task_item n=4 part=untouched
The other ci-leg legs (fmt, clippy, test, embolden, doc, audit, coverage, msrv, wasm, a11y, stand-repeat, counter, build) and their contents: none changed, and FAST_LEGS membership stays five.
@task_item n=5 part=untouched
GitHub Actions workflow rows - permissions, concurrency, caches, pinned actions, failure log artifacts: the workflow is unchanged and CI never runs the sweep.
@task_item n=6 part=untouched
Telemetry, tracing and logging contracts: the sweep writes no log and installs no subscriber.
@task_item n=7 part=untouched
Cold-agent and agent-run scripts, their events, exit codes, verdicts and state directories: not reached.
@task_item n=8 part=untouched
Coverage floors, the coverage role and the mutation tier: no crate's code or coverage moved.
@task_item n=9 part=untouched
Crate and binary names, publishing and versioning rows: no crate, binary or version changed.
