@task_prose part=session
The session measured why the local cargo build directory grows: a lock move adds a new hash per unit above the moved crate, while a source edit adds none.
It added a dev-host sweep that removes stale build units of target/debug outside a closed keep list of seven locked cargo runs, and refuses while a build holds cargo's lock.
The local fast leg now runs that sweep once after its five legs read green, only on a dev host (skipped when CI is set), printing the sweep's one line and never reddening on it.
The sweep prints one JSON stdout line of closed words and counts with no path, writes no log and reads no environment value of its own.
It measured a working day's growth: +57.90 GB unmended against -2.04 GB mended, and a measured limit that CI=true rebuilds the tree under new hashes (about 48 GB), cause not found.
No Rust crate, manifest, lockfile, telemetry sink, span or log site changed; nothing was retired.
@task_file n=1 path=scripts/target-sweep.py
what: the dev-host build-directory sweep: removes stale cargo units of target/debug outside a keep list, under cargo's build lock, printing one closed JSON stdout line of verb, outcome, cause and counts
@task_file n=2 path=scripts/target-sweep.ps1
what: the PowerShell wrapper of the dev-host build-directory sweep, passing its arguments and exit code through
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the ci-scripts leg's unittest contract file for the sweep: its stdout line's closed key set, exit codes, lock refusal, state areas left byte-equal, and the fast leg owning the sweep
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner; its fast arm (the local pre-push gate's legs) now ends with the sweep on a dev host, skipped under CI, its exit discarded
@task_item n=1 part=touches
decision/agent-run-cleanup-scoped and decision/cold-agent-cleanup-scoped: a new remover stands beside them, the sweep, which removes only stale target/debug units and leaves the state areas and top-level files byte-equal.
@task_item n=2 part=touches
decision/local-state-never-uploaded and decision/session-state-dir-private: the sweep never reads, removes or prints anything of the local run state areas; a rule over local state now has a sweep acting in the build directory beside it.
@task_item n=3 part=touches
decision/harness-json-via-python-json: a new harness JSON line, the sweep's stdout line, is encoded by python's json module.
@task_item n=4 part=touches
decision/harness-log-not-telemetry-sink: the sweep writes no log and its stdout line is a harness output, not a telemetry sink.
@task_item n=5 part=touches
decision/log-check-failures-name-kind-count and decision/fixed-diagnostics: the sweep's refusal and failure name a closed cause word and counts only; a bad argv prints one fixed usage line on stderr and exits 2.
@task_item n=6 part=touches
decision/ci-platform-github-workflows: the fast leg gains a dev-host-only step skipped when CI is set, so GitHub CI runs no sweep; CI is a new env name read by the fast leg alone.
@task_item n=7 part=touches
decision/ci-log-artifact-on-failure: the fast leg's output now carries the sweep's line on a dev host; a refused or failed sweep never reddens the leg, so it causes no failure upload.
@task_item n=8 part=touches
decision/committed-checks-hold-no-private-data and decision/committed-kept-checks-clean: the sweep's line holds no path, unit name or key, pinned by a contract test in the ci-scripts leg.
@task_item n=9 part=touches
decision/checks-silent: the new ci-scripts unittest cases add to the silent checks; the sweep itself prints exactly one line per ending.
@task_item n=10 part=touches
decision/telemetry-via-tracing and absence/telemetry-backend: the sweep is a Python dev tool emitting no telemetry and adds no backend; both stand, the sweep's stdout line not being telemetry.
@task_fact n=1 slug=ci-integration
fact: The dev-host build-directory sweep prints exactly one JSON stdout line per ending: verb (plan or run) and outcome (swept, planned, refused or failed); a refusal or failure adds cause (build-in-flight, no-build-directory, unsupported-host or keep-run); a sweep or plan adds the counts units_kept, units_removed or units_to_remove, bytes_before, bytes_after and seconds; no path, unit name or key.
@task_fact n=2 slug=ci-integration
fact: The sweep's exit codes are 0 swept or planned, 1 failed, 2 usage (bad argv, nothing run) and 3 refused.
@task_fact n=3 slug=ci-integration
fact: The sweep touches only stale units in target/debug's deps, .fingerprint, build and incremental areas; the run state areas and top-level files stand byte-equal, and it writes no log and reads no environment value of its own.
@task_fact n=4 slug=ci-integration
fact: The local fast leg runs the sweep once after its five legs read green, on a dev host only, skipped when CI is set or non-empty; its line is printed as is and its exit discarded, so a refused or failed sweep never reddens the leg and a red leg calls none.
@task_fact n=5 slug=ci-integration
fact: The sweep refuses with cause build-in-flight while a build holds cargo's target/debug lock, probing it before the keep pass and holding it across the removal; a second taker fails while the sweep holds it.
@task_item n=1 part=untouched
Span and trace coverage: the driver's command and tool_call spans, their fields, endings and refusal cause names are unchanged.
@task_item n=2 part=untouched
escher's telemetry sink and its log line contracts (format, stream, RUST_LOG, scrub, allowlist, service keys) and the sink-installation decisions are unchanged.
@task_item n=3 part=untouched
The cold-agent and agent-run harness events, verdict fields, counted verbs and their privacy decisions are unchanged.
@task_item n=4 part=untouched
PII scrubbing over engine and escher targets, the select, snapshot, layout, actions and coverage answers, and recording contents are unchanged.
@task_item n=5 part=untouched
Error capture (window error event, script error labels, panic hook, WPT report messages) and metric coverage and its absences are unchanged.
@task_item n=6 part=untouched
Session recording bounds, recording file contracts and the session lifecycle, and the fork CI artifact uploads and WPT publishing decisions are unchanged.
