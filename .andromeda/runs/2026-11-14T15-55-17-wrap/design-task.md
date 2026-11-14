@task_prose part=session
The session added auto-exploration to the driver as one in-process library call: it explores the available actions breadth-first within a step budget, from 1 to 10,000 acting calls, and replays every path on a fresh session.
Each path ends in a closed word: accepted (settled or not), refused by its cause, panicked (the handler's panic is caught), or diverged at a step. Only an accepted path on a screen no earlier path reached is explored further.
The verbs explored are the action table's target verbs whose result holds no image field, chosen by the field's kind: click, type and scroll, with screenshot left out. The call for each node is the one the actions line names.
When a recording directory is given, each path is kept as a replayable check. The directory must be owner-only. A path deeper than a kept check's 400 records ends its recording and loses its check.
The kept check's opening record gained a third key, format, written as 1 and read strictly: when absent it reads as 1, and any other value refuses the check as malformed. The two committed Counter checks were re-recorded with that key.
No verb, role, argument, carrier, exit status, cause, session error, tool, span or log site was added. Exploration reaches no carrier (ratified by the founder) and follows no navigation (still provisional).
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the driver's auto-exploration: the step budget, the breadth-first walk over the actions, the closed path endings, the panic catch, and the recording of each path as a kept check
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: unit checks of the exploration's budget bounds and of which verbs are explored (the target verbs with no image field)
@task_file n=3 path=tests/blitz-tests/tests/stand_explore.rs
what: the integration checks of auto-exploration over the served tasks, a refusing control, a panicking handler, the owner-only recording directory and the budget bound, in both layout modes
@task_file n=4 path=packages/escher-driver/src/lib.rs
what: the driver library's public surface, its re-exports (now adding the exploration, the step budget, the path endings and the check format number)
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: the actions line: the target verbs of the verb table and the call that names an actionable node, now shared with exploration
@task_file n=6 path=packages/escher-driver/src/recording.rs
what: session recording and the kept check: the opening record, its strict read, the step records and the answer digest
@task_file n=7 path=packages/escher-driver/src/recording_tests.rs
what: unit checks of the kept check form: the opening record's format key read and refused, and a two-target step written and read back in schema order
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: the committed kept check of the Counter task recorded through the command line role
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: the committed kept check of the Counter task recorded through the mcp role
@task_file n=10 path=docs/capabilities.md
what: the capability statement: what the driver and escher-session can do, their counts and their limits and absences
@task_item n=1 part=touches
The decision that actions are decided by the verbs' own checks now has a second consumer: exploration drives each node with the actions line's call and explores the table's target verbs. The rule is unchanged, but it now rules over exploration too.
@task_item n=2 part=touches
The actions line's contract items (content, nodes, stale) now feed exploration. Exploration adds one child path per actions node and explored verb, so any row on what the actions line lists bears on what is explored.
@task_item n=3 part=touches
The screenshot verb and the screenshot result contracts: a verb whose result holds an image field is never explored, decided by the field's kind and not its name. Screenshot is the one verb excluded today.
@task_item n=4 part=touches
The contracts for the click and type driver verbs: these, with scroll, are the verbs exploration drives. A refused call leaves the screen unchanged, and a refused path is never expanded.
@task_item n=5 part=touches
The coverage check files read bound and refusal contracts: the kept check's opening record now holds three keys (tool, arguments, format), and a format other than the whole number 1 refuses the check as malformed and holds nothing of its bytes. Any row stating a two-key opening record or the refusal set is moved.
@task_item n=6 part=touches
The kept check's 400-record bound now also limits exploration: a path deeper than it stops recording and its check is removed. A row holding that bound gains this consequence.
@task_item n=7 part=touches
The escher-session commands, exit statuses, mcp role results and stdout contracts: exploration is not offered as a command, tool, role or argument on any carrier, so none of those rows gains an item. Any row listing the session's capabilities as complete now has a capability that exists only in process.
@task_item n=8 part=touches
The Counter task contract: its two committed kept checks were re-recorded and differ only by the opening record's format key. Exploration of the Counter never exhausts, because each increment shows a new count.
@task_item n=9 part=touches
The decision that escher-session output is read by an agent, and the decision on driver command-line output read by agents: exploration's report holds only closed words, indices, calls and booleans, and is never printed or logged.
@task_item n=10 part=touches
The rows on escher-session tasks (cells, crud, flight-booker, load, timer, counter): every served task is now explored within a budget in both layout modes. The task rows themselves are not changed.
@task_fact n=1 slug=surface
fact: Auto-exploration is one in-process driver call and reaches no carrier: no verb, role, argument, command or mcp tool exposes it (the founder's own word, ratified).
@task_fact n=2 slug=surface
fact: Auto-exploration follows no navigation; it explores the one app it boots (provisional, pending the founder's choice).
@task_fact n=3 slug=surface
fact: An exploration's step budget is a whole number from 1 to 10,000 acting calls, re-reach steps counted; 0 or more than 10,000 is no budget.
@task_fact n=4 slug=surface
fact: An explored path ends in one closed word: accepted (with whether it settled), refused (with its cause), panicked, or diverged (at a step index). It holds no panic payload, node id, screen text or digest.
@task_fact n=5 slug=surface
fact: The verbs explored are the action table's target verbs whose result holds no image field, chosen by the field's kind and not its name. Screenshot is excluded.
@task_fact n=6 slug=surface
fact: A kept check's opening record carries the format number 1. When absent it reads as 1; any other value (another number, 1.0, a string, null, a boolean) refuses the check as malformed.
@task_fact n=7 slug=surface
fact: An exploration that records keeps one check per path, in an owner-only directory. A directory open to the group or to others is refused before anything boots.
@task_item n=1 part=untouched
The color palette, typography, spacing, depth strategy and border radius areas: no colour, font, spacing, paint order or hit-testing rule moved.
@task_item n=2 part=untouched
Both motion areas: the virtual clock, advance, scroll instantness and animation settling are unchanged. Exploration drives calls through the same session run and adds no clock rule.
@task_item n=3 part=untouched
The select list rows (its events, accessibility, overlay, colours, gestures and placement): none is changed.
@task_item n=4 part=untouched
The WPT runner output and result colour rows, the shell provider rows, the cursor and favicon rows of iconography, and the JS window rows: none is reached.
@task_item n=5 part=untouched
The web-spa surface and its canvas, the mobile build decision, and the native config rows for fonts and alpha mode: none is reached.
@task_item n=6 part=untouched
The escher-session exit codes and exit statuses: no exit, cause or session error was added or renumbered.
@task_item n=7 part=untouched
The UI graph line and coverage line content: no field of either moved.
