@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session, reachable through the agent-run contract: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar (0, 1, 2, 3).
A plan can then name that command as its role=smoke gate entry instead of driving the binary by hand; its shape (a harness verb or a committed flow script) is the phase's to choose.
Adds the smoke's contract test in the CI scripts' unittest leg, pinned the way the five agent-run verbs are pinned today (a cargo shim or equivalent; nothing live in CI the leg does not already run).
Names the new smoke in the harness documents that describe the agent-run contract; the test-plan master is amended at the wrap, not here.
Re-verifies at HEAD the measured mechanism: the agent-run harness drives blitz-tests targets and no verb boots the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run harness — the agent-run contract's five verbs (boot, run, status, cleanup, logs), each driving blitz-tests targets, and its exit grammar — edits: its verb set, where the smoke becomes a harness verb
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest leg that pins the agent-run contract's verbs with a cargo shim — edits: the tests pinning the new smoke's shape
@task_item n=1 part=needs
The escher-session binary as the smoke boots and drives it from the command line: started on a named stand task, given at least one call, stopped — the binary as it stands, with no new command, argument or role added to it.
@task_item n=2 part=needs
The escher-session stand tasks (counter, crud, cells, flight-booker, timer, load, and the unknown-task answer): the smoke names one of them to boot on.
@task_item n=3 part=needs
The escher-session commands the smoke calls against the booted session (snapshot, screenshot, actions, coverage, graph, help) and the answers each gives on the command line.
@task_item n=4 part=needs
The escher-session exit statuses and exit codes 0, 1, 2, 3: the smoke reads the binary's exit statuses to form its own verdict, and its own verdict reports under the agent-run contract's same four-value exit grammar.
@task_item n=5 part=needs
The escher-session stdout command line output line and the stderr session error and usage error, with the session error's remedy: what the smoke reads back from each call to judge it, and what it surfaces when a call fails.
@task_item n=6 part=needs
The smoke's own output is one JSON-line verdict read by an agent, not a person: it follows the rule that the driver's command line output is for agents.
@task_item n=7 part=needs
Exit statuses read from the schema's table: the smoke's mapping of outcomes onto 0, 1, 2, 3 reads the one table, not a second hand-written list.
@task_item n=8 part=needs
The session's socket and state directory: the smoke's session uses the one registered Unix socket in a caller-named, relative target/tmp state directory, owner-only as every session's is, so its path stays inside the socket path length bound; it adds no listener, port, IPC endpoint, env read or credential path.
@task_item n=9 part=needs
The smoke's contract test reaches the command through a cargo shim or equivalent and runs nothing live in CI that the unittest leg does not already run; it pins the command's verdict line and exit statuses as the existing verb tests pin theirs.
@task_item n=10 part=needs
The boot-path smoke as a role=smoke gate entry a plan can name: the command must be invocable by name from a plan with a single verdict an operator or agent can read back.
@task_standing n=1
convention: every new command line output of the session tooling is read by an agent: plain text or JSON lines, no colour and no terminal test.
home: surface
@task_standing n=2
convention: every new exit status reported over the session binary follows the four-value exit grammar 0, 1, 2, 3 read from the schema's table.
home: surface-cli
@task_item n=1 part=not_needed
Colour palette rows (engine colours, colour scheme, select list colours, colour tokens): the smoke paints nothing and sets no colour.
@task_item n=2 part=not_needed
Typography rows (font fallback, font context, system fonts in tests, font element hints, lang locale): the smoke changes no text layout or font set-up.
@task_item n=3 part=not_needed
Spacing and depth-strategy rows (margins, cell spacing, z-index paint levels, hit-testing at paint clips, stacking contexts): layout and paint order are untouched.
@task_item n=4 part=not_needed
Motion rows (virtual clock, scroll instant, smooth scroll, scrollbar fades, select list motion): the smoke adds no driver verb and changes no clock or scroll behaviour.
@task_item n=5 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon): no cursor or icon is touched.
@task_item n=6 part=not_needed
Select element rows (select list as engine overlay, list events, open gestures, chosen row, select accessibility, select list rows in snapshot or screenshot): the smoke changes no select behaviour.
@task_item n=7 part=not_needed
WPT runner output and result colour rows: the WPT runner is not reached.
@task_item n=8 part=not_needed
Shell provider rows (window drag, minimize, maximize, decorations, set cursor), the JS window object rows and the dioxus-native configuration rows (base colour, alpha mode, font context, default user-agent sheet): no window, script runtime or native config is changed.
@task_item n=9 part=not_needed
Web-spa surface and mobile-target rows: the smoke runs on the desktop host only.
@task_item n=10 part=not_needed
MountedData-during-render and buffered render replay rows: renderer internals are not touched.
@task_item n=11 part=not_needed
Brand identity, border radius, anti-patterns, self-validation protocol and design decisions log areas: they carry no rows that bind this chunk.
