@task_prose part=session
The session measured why the build directory grows on a dev host: a source edit adds no new cargo hash, a lockfile move adds one per unit above the moved crate, and running the fast leg with CI set rebuilds about 48 GB under new hashes for a cause it did not find.
It added a dev-host build-directory sweep, a Python standard-library script taking only plan or run, which removes stale cargo units under the debug build directory that sit outside a keep set read from seven locked cargo runs.
The sweep refuses while a build holds cargo's lock, reads no environment value of its own, writes no log, and prints one stdout line of closed words and counts with no path.
The local fast CI leg now runs the sweep after its five legs read green, only when CI is unset or empty, and a refused or failed sweep never turns it red. It is the sweep's one caller.
Measured: an unmended working day grew the build directory by 57.90 GB and the mended day shrank it by 2.04 GB. No Rust crate, manifest, lockfile or dependency changed, and nothing was retired.
@task_file n=1 path=scripts/target-sweep.py
what: the new dev-host build-directory sweep: a closed argv of plan or run (exit 2 on anything else), a closed keep list of locked cargo runs whose JSON output is parsed line by line, a non-blocking lock probe on cargo's build lock, removal of debug build units alone, and one stdout line of closed words and counts
@task_file n=2 path=scripts/target-sweep.ps1
what: the new PowerShell wrapper that passes every argument to the build-directory sweep and returns its exit code; the sweep refuses off unix
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the new ci-scripts unittest contract file for the sweep, with 16 cases: argv refusal, plan and run removal, state areas left byte-equal, refusal under a held lock, every ending printed as one line without a path, every keep run locked, and the fast leg owning the sweep
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner script; its fast arm, the local pre-push gate, now ends with the sweep after its five legs read green and skips it when the CI environment flag is set
@task_item n=1 part=touches
Input-validation rules for a closed argv reader and its exit 2 (the CLI closed argv reader, agent-run selection allowlist, apt-install argument validation, cold-agent call allowlist, coverage floor checker validation): a new dev-host script now validates its argv as a closed set and refuses it before acting, so it stands beside them.
@task_item n=2 part=touches
Exit-code contracts for usage refusals (agent-run, apt-install, cold-agent and coverage-floor exit 2): the sweep adds its own codes, 0 swept or planned, 1 failed, 2 usage, 3 refused, and none of these rows holds them.
@task_item n=3 part=touches
Environment-value rows (driver reads only RUST_LOG, the environment variables contract, apt-install touches no environment): the fast leg runner now reads CI, a flag with no content, and the sweep itself reads none.
@task_item n=4 part=touches
Dependency-security rows on the fast leg (font-embolden tested by a locked fast leg, run the audit leg locally before a push, locked builds): the local fast gate gains a step after its five legs, and every cargo call the sweep makes is locked. The leg count stays five and no audit runs in the new step.
@task_item n=5 part=touches
Deletion-scope rules (session cleanup deletes only its own files, non-recursive cleanup, recorder removes only its own files, recording never replaces foreign files): the sweep is a new remover. It removes only stale debug build units outside its keep set, touches nothing else under the build directory, and refuses while a build holds the lock.
@task_item n=6 part=touches
Logging rows on harness output being metadata only (agent-run events metadata only, cold-agent events metadata only, coverage floor prints no paths): the sweep writes no log, and its one stdout line holds closed words and counts with no path, unit name or key.
@task_item n=7 part=touches
Closed refusal-cause rows under error handling: the sweep has its own closed refusal causes (build-in-flight, no-build-directory, unsupported-host, keep-run), and the fast leg discards the sweep's exit so it never reddens.
@task_item n=8 part=touches
Untrusted-input parsing rules (serde_json as the untrusted input reader, model reply as untrusted input): the sweep reads cargo's JSON message output line by line and skips any line that is not an object.
@task_item n=9 part=touches
The Input Validation overview and the dependency-security note on the standard library: the sweep uses only the Python standard library and adds no dependency.
@task_fact n=1 slug=input-validation
fact: The build-directory sweep takes only plan or run as its one argument. Anything else prints one usage line on stderr and exits 2 with nothing run.
@task_fact n=2 slug=input-validation
fact: The build-directory sweep's exit codes are a closed set: 0 swept or planned, 1 failed, 2 usage, 3 refused.
@task_fact n=3 slug=input-validation
fact: The build-directory sweep removes only stale cargo units under the debug build directory that sit outside a keep set read from a closed list of locked cargo runs. It touches nothing else under the build directory, keeps any unit newer than its keep pass, and refuses while a build holds cargo's lock.
@task_fact n=4 slug=input-validation
fact: The build-directory sweep reads cargo's JSON message output line by line and skips any line that is not a JSON object.
@task_fact n=5 slug=secret-management
fact: The build-directory sweep reads no environment value of its own. The CI leg runner's fast arm reads CI, a flag with no content tested only for empty or unset, and runs the sweep only when it is unset or empty.
@task_fact n=6 slug=logging-monitoring
fact: The build-directory sweep writes no log. Its stdout is one line of closed words (verb, outcome and cause) and counts, and holds no path, unit name or key.
@task_fact n=7 slug=error-handling
fact: The build-directory sweep refuses with a closed cause set: build-in-flight, no-build-directory, unsupported-host or keep-run. The fast leg discards the sweep's exit, so a refused or failed sweep never reddens it.
@task_fact n=8 slug=dependency-security
fact: The build-directory sweep runs every cargo call locked and uses only the Python standard library, so it adds no dependency.
@task_item n=1 part=untouched
Driver session socket, MCP server, stable element id, snapshot text and refusal JSON rows: no driver verb, wire format, refusal cause or protocol error changed.
@task_item n=2 part=untouched
Engine content rows (select lists, accessibility, password masking, range inputs, CSS, HTML parsing, serialization, iframes): no Rust crate or engine source changed.
@task_item n=3 part=untouched
API security rows on outbound HTTP, navigation, load waits and the per-host limit: the session adds no listener and no network request.
@task_item n=4 part=untouched
Data protection rows on history, cookies, the HTTP cache, kept checks and cold-agent recordings: no stored data or recording changed.
@task_item n=5 part=untouched
Authentication, CI token, publish job and CI secret rows: no workflow file, token, secret or publish job changed.
@task_item n=6 part=untouched
Threat-model crossing decisions and the cold-agent run rows: no carrier, crossing or cold-agent path changed.
@task_item n=7 part=untouched
Dependency pin, audit tool, licence gate and advisory-ignore rows: no manifest, lockfile or audit configuration changed.
