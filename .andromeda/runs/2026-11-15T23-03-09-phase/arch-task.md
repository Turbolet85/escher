@task_prose part=chunk
Builds a boot-path smoke of the session binary escher-session that the agent-run contract reaches: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0, 1, 2, 3.
Its shape (a new harness verb or a committed flow script, reusing an existing seven_guis flow or carrying its own) is left to P3 and P4; either way a plan can then name it as its role = 'smoke' gate entry instead of driving the binary by hand.
Adds a contract test pinning the smoke's shape in the CI scripts' unittest leg, the way the five agent-run verbs are pinned today, through a cargo shim or equivalent, with nothing live in CI the leg does not already run.
Makes the harness documents that describe the agent-run contract name the new smoke; the test-plan master itself is amended at the wrap, not here.
Re-verifies at HEAD that the agent-run harness holds exactly five verbs, each driving blitz-tests targets, none booting the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run harness: its command line of verbs (boot, run, status, cleanup, logs), its JSON-line events, its exit codes and its state directory — edits: the verb set and its dispatch, if the plan takes the harness-verb shape for the smoke
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest contract test that pins the agent-run harness's verbs, events and exit codes through a cargo shim — edits: a test case pinning the smoke's shape
@task_item n=1 part=needs
The agent-run JSON-line events (run.start, run.end, status, boot, test, cleanup): the smoke's verdict is reported as JSON lines in that same grammar.
@task_item n=2 part=needs
The agent-run exit codes 0, 1, 2 and 3: the smoke's exit status follows that grammar, so a plan's smoke entry reads pass, fail, usage error and environment fault the same way the other verbs do.
@task_item n=3 part=needs
The agent-run state directory: whatever run state the smoke keeps stays in the harness's local state area, and run state areas stay local only.
@task_item n=4 part=needs
The escher-session command line as the smoke drives it unchanged: start of a named task, the two-argument task and state-dir form, the --session flag, at least one acting or reading verb, stop, and the binary's own exit codes.
@task_item n=5 part=needs
The session state directory: caller-named, relative under target/tmp, owner-only as every session's is, and short enough to stay inside the 94-byte socket path bound; the session errors for a too-long or non-private state dir are what a wrong choice would hit.
@task_item n=6 part=needs
Occupied resources: the smoke's session uses the one registered local Unix-domain socket and adds no listener, port, IPC endpoint, environment read or credential path; no served network port appears.
@task_item n=7 part=needs
The session host process model the smoke boots and stops: one host process with one listener, a stale socket handled by the host, idle expiry as it stands; the smoke stops the session it started so nothing is left running.
@task_item n=8 part=needs
The named stand task the smoke boots: the seven_guis headless stand, which boots offline with its pinned environment, so the smoke needs no network.
@task_item n=9 part=needs
The smoke adds no driver verb, carrier, refusal cause, session error, log field or span: the validated verb set and the closed refusal-cause set are not widened, and the escher-session usage line and verb table stay as they are.
@task_item n=10 part=needs
Building the binary: the smoke's cargo build runs locked, like every cargo leg, and changes no build profile or debuginfo level.
@task_item n=11 part=needs
The contract test lives in the ci-scripts leg: Python stdlib only, unittest, the Python 3.11 floor, reaching cargo through a shim; the CI workflow contract stays pinned by tests.
@task_item n=12 part=needs
No new CI leg unless the plan finds an existing leg that must run the smoke: the ci-leg legs list (stand-repeat and test already run cli_flow over the committed flows) is unchanged by default.
@task_item n=13 part=needs
If the smoke takes the committed-flow shape, it is a POSIX flow under plain sh taking the binary and a fresh state dir as arguments, the way the seven_guis counter flow boots the session binary today.
@task_item n=14 part=needs
Raw run output is never printed: the smoke reports its verdict as JSON lines and keeps the binary's raw output out of what it prints.
@task_item n=15 part=needs
If a verb is added to the agent-run harness, its Windows script entry stays a pass-through, and cross-process sessions remain unsupported on Windows.
@task_item n=16 part=needs
The harness documents that describe the agent-run contract name the new smoke and how a plan names it as its smoke entry; the test-plan master is not edited here.
@task_standing n=1
convention: every new CI helper test is a Python unittest on the stdlib alone, at the ci-scripts leg's Python 3.11 floor
home: stack-and-technologies · conventions
@task_standing n=2
convention: every new test takes a descriptive name stating the behaviour it pins
home: conventions
@task_standing n=3
convention: every new committed command-line flow script runs under plain sh
home: stack-and-technologies
@task_item n=1 part=not_needed
Engine layout, style, Taffy and paint rows: the chunk touches no engine code, and the Taffy bump is the other entry's freight.
@task_item n=2 part=not_needed
Accessibility tree, ARIA state and AccessKit adapter rows: the smoke reads no accessibility state.
@task_item n=3 part=not_needed
Select list, form control, event dispatch and keyboard activation rows: the smoke drives the stand as it stands and changes no default action.
@task_item n=4 part=not_needed
The content keys of the snapshot, snapshot text, diff, lint, graph and actions answers: the smoke checks a call's outcome, not the keys of what it answers.
@task_item n=5 part=not_needed
MCP server and mcp role rows (handshake, tools listing, tool call spans, stdio bounds): the smoke drives the command line and the session socket, not MCP.
@task_item n=6 part=not_needed
Cold-agent pipe rows (its command line, events, verdict and reasons, live model provider): the cold-agent pipe is out of scope.
@task_item n=7 part=not_needed
Session recording, kept check file and check replay rows: the smoke records no session and keeps no check.
@task_item n=8 part=not_needed
WPT runner, WPT CI artifacts and PR-body WPT section rows.
@task_item n=9 part=not_needed
Browser app rows: history store, persistence, about pages, toolbar, publish workflow.
@task_item n=10 part=not_needed
Upstream sync, cherry-pick and per-take ratification rows: the chunk takes nothing from another tree.
@task_item n=11 part=not_needed
Dioxus bridge, stable element id, UI graph and reached record rows: the smoke adds no id, registration or record.
@task_item n=12 part=not_needed
Telemetry bootstrap, scrub and line-format rows: the smoke adds no span, event or log field to any binary.
@task_item n=13 part=not_needed
Crate publishing, lockstep versioning and README or capability statement rows: the chunk changes no crate, version or front page.
