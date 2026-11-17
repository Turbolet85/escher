@task_prose part=chunk
Mounts one component of an app alone, with props its caller gives, into a headless document at the stand's pinned viewport (800 x 600), bundled font and default seed, named across the UI graph's component boundary.
Proves the isolated mount's snapshot reading (ids, roles, names, states, and bounds where the box is the same) equals the same component's subtree inside its app, ids anchored at the component.
Validates the given props before anything mounts, refusing a bad value as a typed value; how props cross (in process, a closed table, open props on the carriers) is the founder's fork, and no carrier is built before his word.
Tests the document's boundary-marks fallback for a document built after its vdom's first build, by the isolated mount or a fixture.
Adds layout to the cold-agent pipe's verb list and its contract tests so the pipe's isolation check matches the server's fifteen tools again; no live run.
@task_file n=1 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe, its verb list the isolation check compares with the MCP server's tools — edits: the pipe's verb list, gaining layout
@task_file n=2 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests, including its isolation check against the MCP server's fifteen tools — edits: the contract tests' verb list, gaining layout
@task_item n=1 part=needs
The isolated mount boots under the same conditions a stand task reads: the pinned 800 x 600 viewport, the bundled font, the run's default seed, a headless document.
@task_item n=2 part=needs
The mounted component is one the UI graph names as an owner, and the isolated document's UI graph names it as its root's component; the UI graph stays read-only and mirrors the snapshot one node per node.
@task_item n=3 part=needs
The isolated snapshot equals the in-app subtree of the same component under a stated equality: ids anchored at the component (a stable id is a component path, so they differ by a prefix), roles, names and states equal, bounds equal only where the component's box is the same.
@task_item n=4 part=needs
The snapshot of the isolated mount is read from the accessibility tree through the document's own method, the one snapshot model every reader shares; no second reading path for the isolated case.
@task_item n=5 part=needs
Roles, names and states of the isolated mount come from the same rules as in the app: role from role attribute, element, else Unknown; accessible name precedence; ARIA states and native checked state; hidden and inert content kept out of the tree.
@task_item n=6 part=needs
Given props are validated before mounting: a value the component cannot take is refused as a typed value, never a panic on untrusted input.
@task_item n=7 part=needs
No carrier of the isolated mount (verb, argument, MCP tool, kept check) is built until the founder answers the fork; the fork's options and prices are written for him, none taken for him.
@task_item n=8 part=needs
The document's boundary-marks fallback branch, for a document built around an already-built vdom whose root boundary is not in the registry, gains a test that does not re-run rebuild on a built vdom.
@task_item n=9 part=needs
layout joins the cold-agent pipe's verb list and its contract tests, as graph and actions joined; the isolation check compares the list with the MCP server's fifteen tools and holds; a layout answer carries ids, closed words and numbers, no text.
@task_item n=10 part=needs
The served node ids keep their form and reserved separator; no engine NodeId crosses any boundary from the isolated mount.
@task_item n=11 part=needs
The isolated mount and the cold-agent change add no new log field or span content; the existing sink's allowlist and redaction stay as they are.
@task_item n=12 part=needs
No engine change for every document, no new socket, port, IPC endpoint or credential path; the accessibility feature stays as the session library enables it.
@task_standing n=1
convention: each new accessibility test is a Rust test over the AccessKit tree, not a browser or external checker
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: expected roles, names and stand expectations are stated once in shared tables, which a new stand test reads rather than restates
home: 7-screen-reader-support · screen-reader-support
@task_standing n=3
convention: a new stand check runs in the slow-tier accessibility test leg and is repeated three rounds
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Keyboard activation, Tab order, range input, Cells grid, CRUD list and single or multiple select key and list-open behaviour rows: the chunk sends no key, click or focus move.
@task_item n=2 part=not_needed
Contrast, colour, scrollbar, select-list colour and motion and visual lint rows: the chunk measures no contrast and draws nothing new.
@task_item n=3 part=not_needed
Platform adapter and assistive technology reach rows: no stand binary gains an adapter and the tier stays as it is.
@task_item n=4 part=not_needed
Password masking, file input value, IME and caret rows: no text entry is part of the isolated mount.
@task_item n=5 part=not_needed
Cognitive accessibility rows on invalid-state presentation: the chunk changes no component's markup or state presentation.
@task_item n=6 part=not_needed
Driver screenshot, element shot and driver click refusal rows: the chunk adds no capture or click path.
