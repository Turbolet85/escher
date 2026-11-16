@task_prose part=session
The session gave the agent-run contract's run verb a fourth selection word, session, beside stand, all and a blitz-tests file name.
session is a closed word matched before the file-stem arm; it fixes the run's one cargo call to cargo test -p seven_guis --locked --test cli_flow, the boot-path smoke of the escher-session binary.
The cargo call now takes its package from a variable set to blitz-tests by every other arm, so stand, all and a file stem call what they called; the run verb now reaches two packages, blitz-tests or seven_guis.
No new verb, event, event field, state file, environment read, listener or process of the script's own: run session emits the existing run.start, test and run.end events, and a failing or empty session run carries no captured output and is never a pass.
The contract tests grew from 14 to 20 cases and the ci-scripts leg's unittest count from 112 to 118; escher-session itself, the session socket and the MCP server are unchanged.
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run contract script — the harness CLI whose run verb takes a selection and reaches cargo test, its usage line, its exit codes and its run.start, test and run.end events
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run contract tests run by the ci-scripts leg — the cases that pin agent-run's selections, exit codes, events and that no captured output is printed
@task_item n=1 part=touches
decision/agent-run-selection-allowlist goes stale: the selection no longer reaches cargo only as a test stem; the closed word session reaches it as the fixed package seven_guis and fixed target cli_flow, matched before the file-stem arm, and no caller text reaches the cargo call.
@task_item n=2 part=touches
contract/agent-run-cli/2 goes stale where it lists the selections: session joins stand, all and a blitz-tests file name; run session with an extra argument exits 2, the argument count checked first.
@task_item n=3 part=touches
decision/agent-run-events-metadata-only gains run session as a run under it: the same run.start, test and run.end events with selection session, no new field, still harness metadata only.
@task_item n=4 part=touches
decision/agent-run-raw-log-unprinted now also covers the session flows: a failing session flow's captured output reaches neither stdout nor the events log, pinned by a new contract case.
@task_item n=5 part=touches
decision/locked-builds reaches a second cargo call of the harness: the session selection's call to seven_guis carries --locked like the blitz-tests calls.
@task_item n=6 part=touches
Indirect: rows that the harness holds no listener or process of its own, and decision/session-socket-sole-listener with absence/served-network-api, now meet a harness selection whose flows start escher-session hosts on their Unix sockets under the cli_flow target's own temporary state dirs; the script itself adds no listener or process.
@task_item n=7 part=touches
Indirect: the session state directory rows (decision/state-dir-owner-only, decision/state-dir-path-is-parameter, decision/state-dir-deletes-only-own) now rule over state dirs a harness selection reaches through the cli_flow flows, created under CARGO_TARGET_TMPDIR as before.
@task_item n=8 part=touches
Indirect: rows that the harness reads no secret or new environment hold unchanged under run session; no environment read was added to agent-run.
@task_fact n=1 slug=input-validation
fact: agent-run's run selections are a closed set of words — stand, all, session — each matched before the file-stem arm, so a test file named after a word never diverts it; session alone reaches the package seven_guis, by the fixed call cargo test -p seven_guis --locked --test cli_flow.
@task_fact n=2 slug=logging-monitoring
fact: an agent-run session run that ran no test exits 1 with outcome failed and is never reported as a pass.
@task_item n=1 part=untouched
The escher-session binary's own surfaces — its flags, roles, exit codes, stdout and stderr lines, and coverage line — are unchanged; the smoke only drives them.
@task_item n=2 part=untouched
The driver session socket's grammar, bounds, replies, idle expiry and refusal causes, and the MCP server's transport, request bounds, results and protocol errors are unchanged.
@task_item n=3 part=untouched
Engine and page input validation — select lists, accessibility, aria, range, password masking, CSS, HTML parsing, stable ids, snapshot text — is not reached.
@task_item n=4 part=untouched
Dependency security — the audit tool, licence gate, pins, git dependencies and workspace versions — is not reached: no dependency changed.
@task_item n=5 part=untouched
Cold-agent runs, recorded and kept checks, the coverage floor checker and apt-install are not reached.
@task_item n=6 part=untouched
CI tokens, secrets, signing material, publish guards and the WPT workflows are not reached.
@task_item n=7 part=untouched
Outbound HTTP, script fetchers, navigation schemes, cookies, cache and history storage are not reached.
