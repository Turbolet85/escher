@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session that the agent-run contract reaches: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0 · 1 · 2 · 3.
A plan can then name it as its role = smoke gate entry instead of driving the binary by hand; its shape (a new harness verb or a committed flow script, reusing the seven_guis counter flow or carrying its own) is left to P3 and P4.
Adds its contract test in the CI scripts' unittest leg, pinning the smoke's shape the way the five verbs are pinned today, with a cargo shim or equivalent and nothing live in CI that the leg does not already run.
The harness documents that describe the agent-run contract name the new smoke; the test plan's agent-run contract itself is amended only at the wrap.
Re-verifies at HEAD that the agent-run driver holds exactly five verbs, each driving blitz-tests, none booting the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run contract's driver script, its five verbs boot run status cleanup logs over the blitz-tests targets — edits: its verbs, if the smoke lands as a harness verb
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest check of the agent-run driver, pinning its verbs with a cargo shim — edits: the tests pinning the new smoke's shape
@task_item n=1 part=needs
The new smoke entry of the agent-run contract: a verb or selection reachable by the documented invocation that builds escher-session, boots a session on a named stand task, runs at least one call, and stops it.
@task_item n=2 part=needs
The smoke's JSON-line verdict and events: agent-run event lines (run.start, run.end, status and the like) are harness metadata, encoded by python3 json, carrying no element ids, app names, typed text or panic text, and no test or build output reaching them.
@task_item n=3 part=needs
The agent-run exit codes 0 · 1 · 2 · 3: the smoke's exit status follows that grammar, mapping the escher-session exit statuses it reads into it.
@task_item n=4 part=needs
The escher-session command line the smoke drives as it stands: start on a named app, a call such as snapshot or run, stop, and the session errors it may meet (AlreadyRunning, NoSession, Dead, Timeout) read as a failed smoke.
@task_item n=5 part=needs
The session state directory: caller-named, relative under target/tmp, inside the socket path length bound, owner-only, holding the one registered session.sock; the smoke opens no new listener, port, IPC endpoint, env read or credential path.
@task_item n=6 part=needs
Indirect, through the agent-run contract: agent-run holds no process and one driver runs at a time, so the smoke stops the session it booted and leaves no host behind, even when a step fails.
@task_item n=7 part=needs
Indirect, through the session host it starts: a check owns its host process, reaps it on failure, and sends child streams to null or drains them while the host runs.
@task_item n=8 part=needs
An empty run is never a pass: the smoke passes only on positive evidence that a booted session answered a call.
@task_item n=9 part=needs
Socket session checks are unix-gated: the smoke's behaviour off unix, including the agent-run windows entry and the session host's non-unix arms, is stated rather than left to fail at random.
@task_item n=10 part=needs
The named stand task the smoke boots: the seven_guis headless stand boots offline on its bundled font, with no skip.
@task_item n=11 part=needs
Indirect, through the agent-run driver's build: the cargo build of escher-session the smoke runs follows the locked, host-reproducible cargo rule its legs keep.
@task_item n=12 part=needs
The contract test is offline: CI scripts are unit-tested under unittest, the cargo shim is a hand-written stand-in with no mocking library, and no live session binary runs in the unittest leg.
@task_item n=13 part=needs
No new CI leg: the smoke is not a leg of its own unless an existing leg must run it; the stand-repeat and test legs keep running cli_flow over the committed flows.
@task_item n=14 part=needs
The smoke adds no driver verb, carrier, refusal cause, session error, log field or span: it drives the binary as it stands, and the session's stderr and log lines stay as they are.
@task_standing n=1
convention: each new Python script test runs under unittest, offline, in the CI scripts' unittest leg
home: 3-test-harness-contract · 1-test-scope-summary
@task_standing n=2
convention: each new test is controlled red-before-green, prints no argument value on failure, and a table-driven test asserts its row count
home: unit-test-strategy
@task_standing n=3
convention: each new test states its data in code and seeds nothing
home: 3-test-harness-contract
@task_standing n=4
convention: each new check uses no sleep or wall-clock read; waits are bounded
home: integration-test-strategy
@task_item n=1 part=not_needed
WPT runner, WPT status set, WPT conformance and WPT CI decisions: the smoke touches no web-platform test run.
@task_item n=2 part=not_needed
The Rust harness API (construction, core, input and inspection helpers, settle, contrast, screenshot): the chunk adds no Rust harness test.
@task_item n=3 part=not_needed
Lint defects, lint findings, handler coverage, the coverage line and coverage role: the smoke runs neither lint nor coverage.
@task_item n=4 part=not_needed
The MCP server and the escher-session MCP role contracts: the smoke drives the command line, not the mcp role.
@task_item n=5 part=not_needed
The cold-agent pipe's verbs, events, verdict, state and invocation: out of scope by the chunk's boundary.
@task_item n=6 part=not_needed
Pixel tests, PNG decoding, shot determinism and screenshot answers: the smoke compares no frame.
@task_item n=7 part=not_needed
Session recording, committed recorded checks and the checks' recording script: the smoke records and replays no check.
@task_item n=8 part=not_needed
The session wire framing (hello, call, answer and request bounds): the smoke reaches the session only through the binary's command line.
@task_item n=9 part=not_needed
Layout, paint and widget decisions (Taffy, buffered render, select list colours, password caret, scrollbar thumbs, box-shadow): engine behaviour the smoke does not change.
@task_item n=10 part=not_needed
The apt-install script and the coverage floor script exit codes, and CI artifact naming: other scripts' contracts the chunk does not touch.
@task_item n=11 part=not_needed
Driver refusal-cause contracts and their order: the smoke adds none and asserts none.
