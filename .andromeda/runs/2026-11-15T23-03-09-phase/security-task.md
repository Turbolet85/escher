@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session that the agent-run contract reaches: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0, 1, 2, 3, so a plan can name it as its role=smoke gate entry.
Its shape — a new harness verb beside the five agent-run verbs, or a committed POSIX flow script (reusing the committed Counter flow or carrying its own) — is the phase's to measure; the smoke drives the binary as it stands.
Adds a contract test pinning the smoke's shape in the CI scripts' unittest leg the way the agent-run verbs are pinned today, through a cargo shim or equivalent, with nothing live in CI that the leg does not already run.
The harness documents that describe the agent-run contract name the new smoke; the test-plan master is not amended here.
Re-verifies at HEAD the measured mechanism: the agent-run harness drives blitz-tests targets only and no verb boots the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run harness — the agent-run contract's command line, its verbs (boot run status cleanup logs), its argument selection reaching cargo as a test stem, its exit codes and its JSON-line events — edits: its verb set and usage, if the smoke takes the harness-verb shape
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest contract test of the agent-run harness, pinning its verbs through a cargo shim — edits: its contract tests, gaining the smoke's shape
@task_file n=3 path=examples/seven_guis/tests/flows/counter.sh
what: the committed POSIX flow that boots the session binary on a fresh state directory (start, snapshot, click, refused click, stop, session error), run as an asset of the seven_guis cli_flow target; the smoke may reuse it
@task_item n=1 part=needs
The agent-run contract's exit grammar: the smoke's verdict and process exit follow the codes 0, 1, 2, 3 of the agent-run command line, a usage or argument fault taking its fixed code.
@task_item n=2 part=needs
The agent-run harness's argument handling: any argument the smoke takes (a stand task name, a state directory) is validated against a closed allowlist before use and reaches cargo only as a test stem or a fixed build command, never as free text.
@task_item n=3 part=needs
The smoke's JSON-line verdict and events: they hold harness metadata only (verb, task, outcome, counts, exit), with the raw build and session output kept unprinted; no screen content, attribute text, control value or caller-typed text reaches the verdict.
@task_item n=4 part=needs
The escher-session command line the smoke drives as it stands: its roles and flags (--session and the call flags), its exit codes 0, 1, 2, 3, its stdout command result line and its stderr usage and error messages, read by the smoke to form its verdict.
@task_item n=5 part=needs
The driver session socket: the smoke's session uses the one registered Unix socket as the sole listener, reachable by the owning user only, with one bounded connection at a time; the smoke adds no listener, port, IPC endpoint or credential path.
@task_item n=6 part=needs
The session state directory: a caller-named, relative target/tmp path passed as a parameter, owner-only as every session's is; the socket path bound (94 bytes) refuses a too-long state directory with its session error; session cleanup deletes only its own files, non-recursively.
@task_item n=7 part=needs
The session lifecycle over the socket the smoke exercises: hello, at least one call under the closed request grammar, and stop with its lifecycle reply; a session it starts is stopped by it, leaving the host idle expiry as the fallback only.
@task_item n=8 part=needs
The call the smoke runs: an existing driver verb under the closed driver verb schema, its answer or refusal read by closed fields; the smoke adds no verb, refusal cause, session error, log field or span.
@task_item n=9 part=needs
Environment: the driver reads only RUST_LOG; the smoke adds no environment read to the binary or harness and passes no secret.
@task_item n=10 part=needs
The smoke's build of escher-session follows the locked-builds rule (the lockfile held, no dependency resolved anew).
@task_item n=11 part=needs
The contract test: it runs in the CI scripts' unittest leg through a cargo shim or equivalent, live-free, pinning the smoke's arguments, exit codes and verdict shape as the agent-run verbs are pinned.
@task_item n=12 part=needs
No CI leg of its own unless the plan finds an existing leg that must run it; the stand-repeat and test legs already run cli_flow over the committed flows, and any leg change keeps the CI job rules.
@task_item n=13 part=needs
The harness documents describing the agent-run contract name the new smoke as a role=smoke gate entry; the test-plan master's agent-run contract is amended at the wrap only.
@task_standing n=1
convention: every harness script the plan adds or extends validates each argument first against a closed allowlist and exits with its documented usage code before running anything
home: input-validation
@task_standing n=2
convention: every exit code of a harness command line is a contract row of that script's exit-code boundary, pinned by its contract test
home: input-validation · error-handling
@task_standing n=3
convention: every harness event or log line holds run metadata only, raw build and session output kept unprinted
home: logging-monitoring
@task_standing n=4
convention: every session state directory a script or test creates is caller-named, owner-only, and cleaned of its own files only
home: input-validation
@task_item n=1 part=not_needed
Engine page-content rules — CSS parsing, HTML parsing with scripting disabled, select lists, range inputs, password painting, accessibility names, stable id derivation, serialization: the chunk changes no engine or driver code.
@task_item n=2 part=not_needed
MCP server rows — stdio transport, protocol errors, initialize, tool-call span and tool results: the smoke drives the session role, not the mcp role, and adds none.
@task_item n=3 part=not_needed
Recording and kept-check rows — recorded check files, recording directory, kept check files and the coverage role and its answer line: the smoke records nothing and runs no coverage.
@task_item n=4 part=not_needed
Cold-agent pipe rows — its call and task allowlist, fixed prompt, replay isolation, transcripts and recordings: the cold-agent pipe is out of scope.
@task_item n=5 part=not_needed
Browser application rows — JavaScript opt-in, external navigation, history in sqlite, HTTP cache, cookies, clipboard, file dialog, TLS: the smoke touches no browser app.
@task_item n=6 part=not_needed
Outbound request rows — per-host concurrency, GET-only sub-resources, CORS and same-origin absences, response size and timeout absences, script fetcher URL rules: the smoke makes no outbound request.
@task_item n=7 part=not_needed
Release and publish rows — publish job repository guard, publish secrets reachable only upstream, signing material removal, release publishing: the chunk touches no release or publish job.
@task_item n=8 part=not_needed
Dependency choice rows — advisory ignores, licence gate, engine pins, crate choices such as png, serde_json, color: the chunk adds no dependency.
@task_item n=9 part=not_needed
WPT runner and its workflow rows — quarantine entries, post-results workflow, runner result names, script type detection: the chunk does not reach the WPT runner.
