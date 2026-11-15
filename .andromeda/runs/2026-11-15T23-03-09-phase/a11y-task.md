@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session that the agent-run contract reaches: one documented command builds the binary, boots a session on a named stand task, runs at least one existing driver call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0 / 1 / 2 / 3.
Its shape (a harness verb or a committed flow script, reusing an existing seven_guis flow or carrying its own) is left to P3 and P4; a plan can then name it as its role = 'smoke' gate entry instead of driving the binary by hand.
Adds its contract test to the CI scripts' unittest leg, pinned the way the five agent-run verbs are pinned today, with a cargo shim or equivalent and nothing live in CI.
The harness documents that describe the agent-run contract name the new smoke; the test-plan master is amended at the wrap, not here.
Re-verifies at HEAD the measured mechanism: the harness holds five verbs that drive blitz-tests targets and none boots the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run harness, the contract's five verbs boot run status cleanup logs, each driving the blitz-tests targets — edits: its verb set, should the smoke take the harness-verb shape
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest leg pinning the agent-run contract's verbs through a cargo shim — edits: the contract tests, adding the smoke's shape test
@task_file n=3 path=examples/seven_guis/tests/flows/counter.sh
what: a committed POSIX flow that boots the session binary on the seven_guis stand (start, snapshot, click, refused click, stop, session error) given the binary and a fresh state directory, an asset of the seven_guis cli_flow target
@task_item n=1 part=needs
The new smoke: builds escher-session, boots one session on a named stand task, runs at least one driver call, stops the session, and prints one JSON-line verdict with exit 0, 1, 2 or 3 under the agent-run contract's exit grammar.
@task_item n=2 part=needs
The smoke drives the session binary as it stands: it adds no driver verb, carrier, refusal cause, session error, log field or span, so the calls it makes and the answers it reads keep their current shapes.
@task_item n=3 part=needs
The call the smoke runs against the booted session (a snapshot, a click or a refused click on a disabled control) reads the snapshot model taken from the accessibility tree, which the session library always enables; the smoke's build of the binary must keep that feature on.
@task_item n=4 part=needs
The stand task the smoke boots is a stand binary with no platform accessibility adapter; the smoke reaches the tree only through the session's driver, never through an assistive-technology adapter.
@task_item n=5 part=needs
The smoke's session uses the one registered Unix socket in a caller-named, relative target/tmp state directory, owner-only as every session's is, inside the 94-byte socket bound; it opens no new listener, port, IPC endpoint, env read or credential path.
@task_item n=6 part=needs
The smoke's contract test runs in the CI scripts' unittest leg through a cargo shim or equivalent and runs nothing live in CI that the leg does not already run.
@task_item n=7 part=needs
No new CI leg unless the plan finds an existing leg that must run the smoke; the stand-repeat and test legs already run cli_flow over the committed flows, three rounds over the stand checks.
@task_item n=8 part=needs
The smoke's verdict and the session's own stderr follow the existing obs log format and its target allowlist and redaction; the smoke prints its verdict as its own JSON line and adds no log line to the session.
@task_item n=9 part=needs
The harness documents describing the agent-run contract name the new smoke and its role = 'smoke' entry; the plan template's smoke entry form and the test-plan master stay as they are here.
@task_item n=10 part=needs
Re-verify at HEAD the measured L1 mechanism: the harness drives blitz-tests targets and no verb boots the session binary, which until now was smoked by hand through target/debug/escher-session with a relative target/tmp state dir.
@task_item n=1 part=not_needed
ARIA role mapping, accessible-name precedence and ARIA property and state reads: the smoke reads whatever the tree carries and changes no mapping.
@task_item n=2 part=not_needed
Keyboard navigation contracts: Tab order, keyboard activation, range input keys, single and multiple select keys and lists, the Cells grid and CRUD list keys; the smoke presses no key behaviour of its own and changes none.
@task_item n=3 part=not_needed
Visual design verification: contrast thresholds, the lint's contrast finding, stand accent colour, select list colours and motion, scrollbar thumb strokes, hidpi and zoom; the smoke measures no pixels.
@task_item n=4 part=not_needed
Hidden and inert content rules and hit-testing of inert elements: the smoke changes no tree membership or hit rule.
@task_item n=5 part=not_needed
Cognitive accessibility rows: invalid-state signalling beyond colour or class and the shared stand tab-order table.
@task_item n=6 part=not_needed
Dioxus document internals: buffered render, reached-handler record, UI graph state registration, listeners from the vdom walk.
@task_item n=7 part=not_needed
Password masking, caret placement and IME rules, and the screenshot and actions-answer contracts: the smoke adds no field to any answer and reads none of these behaviours.
