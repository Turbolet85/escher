@task_prose part=session
The session measured why the build directory target/ grows on a dev host: a lock move adds a new cargo hash per unit above the moved crate, a source edit adds none; a replayed working day grew it by 57.90 GB unmended and shrank it by 2.04 GB mended.
It added a dev-host command line, the build-directory sweep, run as plan or run from the repository root, with a Windows pass-through script that hands every argument on and returns its exit code.
The sweep removes only stale units of target/debug (deps, .fingerprint, build, incremental) outside a closed keep list of seven locked cargo runs, refuses while a build holds the cargo lock, refuses off unix, reads no environment value of its own and writes no log.
It prints one JSON line of closed words and counts with no path: verb, outcome (swept, planned, refused, failed), a cause on refusal or failure (build-in-flight, no-build-directory, unsupported-host, keep-run), and unit and byte counts; it exits 0 swept or planned, 1 failed, 2 usage, 3 refused.
The CI leg runner's fast leg now runs the sweep after its five legs read green, only when the CI flag is unset or empty, its exit discarded so it never reddens the leg; a new unittest contract file pins the sweep and that ownership.
No Rust crate, stand app, layout, element id, MCP server or escher-session behaviour changed; nothing was retired. Measured limit, left open: CI=true rebuilds the tree under new hashes, mechanism not found.
@task_file n=1 path=scripts/target-sweep.py
what: the dev-host build-directory sweep command line, plan or run, its usage line, its exit codes and its one JSON stdout line
@task_file n=2 path=scripts/target-sweep.ps1
what: the sweep command line's Windows pass-through script
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the sweep's test contract, its usage, exits, lock refusal, stdout key set and the fast leg owning the sweep
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner, its fast leg, now ending with the sweep on a dev host and reading the CI flag
@task_item n=1 part=touches
contract/agent-run-cli/usage, exit-2, stdout and windows-pass-through: a new dev-host script command line takes the same shape beside them, a usage line on stderr with exit 2 and nothing run, one stdout line, and a .ps1 pass-through returning its exit code.
@task_item n=2 part=touches
contract/cold-agent-cli/usage, exit-2, exit-3, stdout and windows-pass-through: the sweep uses the same exit convention beside them, 2 for usage and 3 for a refusal, and a pass-through script for Windows.
@task_item n=3 part=touches
contract/agent-run-cli/run-log: its log lives under target/; the sweep removes only stale units of target/debug and touches nothing else under target/, so the log stands.
@task_item n=4 part=touches
contract/cold-agent-cli/record-dir: its recording directory lives under target/; the sweep leaves it untouched, removing build units of target/debug alone.
@task_item n=5 part=touches
contract/escher-session-record/check-file and record-dir-not-private: any recording state under target/ stands beside the sweep, which removes no state area and no top-level file.
@task_item n=6 part=touches
decision/cold-agent-pipe-builds-nothing: the cold-agent pipe relies on an already-built driver; the sweep keeps every unit of its keep list and escher-session stood after the final sweep, so prebuilt artifacts the pipe uses stay, unless they fall outside the seven keep runs.
@task_item n=7 part=touches
contract/escher-session-command/form: the escher-session binary built under target/debug is kept by the sweep's keep list; the command form itself is unchanged.
@task_item n=8 part=touches
decision/no-caller-input-echoed: the sweep echoes no argument and prints no path, unit name or key on its stdout line, beside this rule.
@task_item n=9 part=touches
contract/multi-doc-cli/multi-doc-flat, multi-doc-n and multi-doc-seq: rows naming env values a command line reads; a new env name CI is read by the CI leg runner's fast leg alone as a flag carrying no content, and the sweep reads none of its own.
@task_fact n=1 slug=surface-cli
fact: The build-directory sweep command line takes exactly one verb, plan or run, from the repository root; any other argv prints one usage line on stderr and exits 2 with nothing run.
@task_fact n=2 slug=surface-cli
fact: The sweep prints one JSON stdout line of closed words and counts and no path: verb and outcome (swept, planned, refused, failed), a cause on a refusal or failure (build-in-flight, no-build-directory, unsupported-host, keep-run), and on a sweep or plan units_kept, units_removed or units_to_remove, bytes_before, bytes_after and seconds.
@task_fact n=3 slug=surface-cli
fact: The sweep exits 0 swept or planned, 1 failed, 2 usage, 3 refused; it refuses while a build holds the cargo lock (build-in-flight) and holds that lock across its removal.
@task_fact n=4 slug=surface-cli
fact: The sweep's Windows pass-through hands every argument on and returns its exit code; the sweep itself refuses off unix with unsupported-host.
@task_fact n=5 slug=surface-cli
fact: The sweep removes only units of target/debug deps, .fingerprint, build and incremental outside its closed keep list of seven locked cargo runs, reads no environment value of its own, writes no log and touches nothing else under target/.
@task_item n=1 part=untouched
The stand element ids (contract/stand-element-ids/*): no stand app, control or id changed.
@task_item n=2 part=untouched
The desktop-native stand decisions (keying, a11y attributes, viewport, accent colour, document skeleton, scrolling, clipping, lints at boot): no layout or stand code changed.
@task_item n=3 part=untouched
The ShellProvider window chrome controls: no shell or window code changed.
@task_item n=4 part=untouched
The escher-session verbs, answers, refusals, exit codes, MCP server, snapshot, screenshot, lint, graph, layout, coverage and served tasks: the driver's behaviour is unchanged; only its build units' cleanup is new.
@task_item n=5 part=untouched
The cold-agent prompt and MCP decisions and the session-recording decision: the cold-agent run and its verdict are not changed.
@task_item n=6 part=untouched
The paint_bench command line: its arguments and backend are unchanged.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates, and the mobile-native and web-spa surfaces: nothing reached them.
