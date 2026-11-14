@task_prose part=session
The session added auto-exploration to the driver: one in-process call explores the available actions breadth-first within a step budget and keeps each explored path as a replayable check.
Each path runs on a fresh session booted for it; its last call runs under a panic catch; a path ends accepted (settled or not), refused by its cause, panicked or diverged, and only an accepted path reaching a screen no earlier path reached is expanded.
The budget is a whole number from 1 to 10,000 acting calls, re-reach steps counted; the first screen is read once, unrecorded, at no cost; exploration stops when the frontier empties (exhausted) or the next path would pass the budget.
The verbs explored are the action table's target verbs whose result holds no image field, chosen by the field's kind (today click, type, scroll; screenshot left out).
The kept check's opening record gained a third key, format, written 1 and read strictly (absent read as 1, any other value refused as malformed); the two committed Counter checks were re-recorded and differ by that key alone.
Measured: the Counter never exhausts a breadth-first frontier (each increment shows a new count); exhaustion is witnessed on a finite refusing-control page. No verb, role, argument, MCP tool, exit, cause, session error, span or log site was added or retired.
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the driver's auto-exploration — the explore call, its step budget and bound, the exploration report with its paths and closed path endings, the explored-verb selection and the panic catch
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: unit checks of the exploration budget's bounds and of which action verbs are explored (an image-result verb left out by its field kind)
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the driver's public surface — the session, its re-exported calls, types and constants
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the actions answer — the driver's list of the verbs a node takes and the call naming each target
@task_file n=5 path=packages/escher-driver/src/recording.rs
what: session recording as a replayable check — the kept check's form, its opening record, the check files written into the owner-only record directory and the strict read of a check
@task_file n=6 path=packages/escher-driver/src/recording_tests.rs
what: unit checks of session recording and of reading a kept check, the opening record's keys and the malformed refusal
@task_file n=7 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand check of auto-exploration over the served tasks in both layout modes — budget bound, Counter replay, refusal by cause, panic caught, kept checks replaying green, a group-open record directory refused
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: the committed kept check of the Counter task recorded through the command line
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: the committed kept check of the Counter task recorded through the MCP server
@task_file n=10 path=docs/capabilities.md
what: the capability statement — what the driver and stand can do, its counts and its stated limits and absences
@task_item n=1 part=touches
decision/session-recording-replayable-check: a recording's kept check now opens with a format key (1) beside tool and arguments; and exploration is a second producer of kept checks, one per explored path, each replaying green.
@task_item n=2 part=touches
contract/escher-session-record/check-file: the check-{n}.json opening record moved from two keys to three (tool, arguments, format 1); format absent reads as 1, any other number or type refuses the check as malformed holding nothing of the bytes.
@task_item n=3 part=touches
contract/escher-session-record/record-dir-not-private: exploration with a record directory refuses a group- or other-open directory with the same session error before anything boots, writing nothing.
@task_item n=4 part=touches
contract/escher-session-coverage/check-bound: a kept check's record bound (400 records) now also bounds an explored path's recording — a path deeper than it ends its recording and that check is removed.
@task_item n=5 part=touches
contract/escher-session-actions/answer: the actions table's target verbs and its call naming per node are now also what exploration drives, each node taking one child per explored verb.
@task_item n=6 part=touches
contract/escher-session-screenshot/answer and contract/escher-session-mcp/screenshot-result: screenshot is left out of exploration because its result holds an image field (by field kind, not name).
@task_item n=7 part=touches
contract/escher-session-served-tasks rows (counter, cells, crud, flight-booker, load, timer): every served task is now explored within its budget in both layout modes; the Counter never exhausts its frontier.
@task_item n=8 part=touches
contract/stand-element-ids/counter-increment and counter-value: each increment shows a count no earlier screen showed, so exploration of the Counter keeps finding new screens until the budget ends it.
@task_item n=9 part=touches
contract/escher-session-refusals rows: a refused explored path ends reported by its cause (closed word) and is never expanded; no new cause added, causes still eight.
@task_item n=10 part=touches
decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot: exploration boots a fresh session per path through a caller-given boot and re-reaches the path's screen by replaying its earlier calls.
@task_item n=11 part=touches
decision/no-caller-input-echoed: the exploration report holds only closed words, indices, calls and bools — no panic payload, node id, screen text or digest — and nothing is logged or printed.
@task_item n=12 part=touches
contract/escher-session-mcp/tools and contract/escher-session-help/answer: exploration is in process only — no MCP tool, verb, role or argument reaches it; tools stay fourteen.
@task_fact n=1 slug=surface-cli
fact: The driver explores in process: one call walks the available actions breadth-first from the first screen within a step budget of 1 to 10,000 acting calls (re-reach steps counted, the first screen free), each path on a fresh session, its last call under a panic catch, stopping when the frontier empties (exhausted) or the next path would pass the budget.
@task_fact n=2 slug=surface-cli
fact: An explored path ends in one closed word — accepted (settled or not), refused with its cause, panicked, or diverged at a step — and only an accepted path reaching a screen no earlier path reached is expanded; refused, panicked and diverged paths are never expanded.
@task_fact n=3 slug=surface-cli
fact: The verbs explored are the actions table's target verbs whose result holds no image field, selected by the field's kind and never its name; screenshot is left out.
@task_fact n=4 slug=surface-cli
fact: Exploration reaches no carrier — no verb, role, argument, command-line form or MCP tool runs it; it is one in-process call, by the founder's own word.
@task_fact n=5 slug=surface-cli
fact: Exploration follows no navigation — it explores actions on one screen tree only; this absence stands provisional pending the founder's choice.
@task_fact n=6 slug=surface-cli
fact: A kept check's opening record states the check format, the whole number 1; a check without it reads as format 1, and any other value (another number, 1.0, a string, null, a bool) refuses the check as malformed.
@task_fact n=7 slug=surface-cli
fact: Exploration given a record directory keeps one replayable check per explored path in that owner-only directory, the panicked path's check holding the steps before the panic.
@task_item n=1 part=untouched
surface-desktop-native window chrome: the ShellProvider window chrome contracts (drag, maximise, minimise, close, decorations) — no window or shell provider changed.
@task_item n=2 part=untouched
Stand stable element ids other than the Counter's: no id added, renamed or retired; the cells, crud, circle, flight, load, temp, timer and task-card ids stand.
@task_item n=3 part=untouched
Stand look and boot decisions: accent colour, document skeleton and main mount, pinned viewport, attribute-only names, roles, states and ids, lints clean at boot, clipping and scroll-into-view, open list bounds — none moved.
@task_item n=4 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command-line contracts and their exit codes, usage, stdout and Windows pass-through — no script or harness changed.
@task_item n=5 part=untouched
The escher-session exit codes, command form, host log service name, MCP protocol revisions, stdio and end, result bound, snapshot answer and its part flag, graph and lint answers — no exit, argument, endpoint or protocol changed.
@task_item n=6 part=untouched
Font refusals and off-screen and unknown-verb refusal shapes, and the recording-failed error — no refusal or session error added or moved.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates and the mobile-native and web-spa surfaces — the session reaches no document template, mobile or web layout.
