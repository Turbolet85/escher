@task_prose part=session
The session added auto-exploration to the escher driver: one in-process library call explores a booted app's available actions breadth-first within a step budget and reports each path with a closed ending.
Each path runs on a fresh session from a boot callback; a handler panic is caught and reported, a refused, panicked or diverged path is never expanded, and with a recording directory each path is kept as a replayable check.
The kept check's opening record gained a third key, format, written as 1, read strictly, absent read as 1; any other value refuses the check as malformed. The two committed stand Counter checks were re-recorded and differ by that key alone.
Measured: an unbounded app such as the Counter never empties the frontier, so exhaustion is witnessed only on a finite page; a session dropped after a mid-handler panic drops cleanly.
The founder's own word ratified three forks: exploration stays in process with no carrier (standing), the no-seed-on-carriers decision loses its provisional state, and exploration following navigation stays provisional.
No verb, role, argument, MCP tool, cause, session error, node key, span, log site, env var or dependency was added; no file was retired.
@task_file n=1 path=packages/escher-driver/src/explore.rs
what: the driver's auto-exploration component, new: the in-process explore call, its step budget, its exploration report and closed path endings, the explored-verb selection and the per-path panic catch
@task_file n=2 path=packages/escher-driver/src/explore_tests.rs
what: the driver's unit tests for exploration, a path child of the explore module: the budget's bounds and the image-field verb exclusion
@task_file n=3 path=tests/blitz-tests/tests/stand_explore.rs
what: a new blitz-tests stand target for exploration over the seven_guis stand in both layout modes: every served task within its budget, kept checks replaying green, a refusing control, a panicking handler, the private recording directory and the budget bound
@task_file n=4 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root and its public re-exports, now naming explore, StepBudget, Exploration, ExploredPath, PathEnding and CHECK_FORMAT
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: the driver's actions verb and its target-verb and call-naming readers, two of them now crate-visible for exploration
@task_file n=6 path=packages/escher-driver/src/recording.rs
what: the driver's session recording and kept check writer and strict reader: the opening record, the check format number and the digest helper
@task_file n=7 path=packages/escher-driver/src/recording_tests.rs
what: the driver's recording unit tests: the opening record's format rows read and refused, and a two-target verb's step written and read back
@task_file n=8 path=tests/blitz-tests/checks/counter-cli.json
what: a committed stand Counter kept check recorded over the command line, read by the replay and coverage stand tests
@task_file n=9 path=tests/blitz-tests/checks/counter-mcp.json
what: a committed stand Counter kept check recorded over the MCP role, read by the replay and coverage stand tests
@task_file n=10 path=docs/capabilities.md
what: the capability statement: its exploration section, the in-process-only and no-navigation limits, and the limit that a path deeper than a kept check's record bound ends its recording and that check is removed
@task_item n=1 part=touches
The kept check opening record contract now holds three keys, tool, arguments and format, not two; format is written as the whole number 1.
@task_item n=2 part=touches
The kept check reader's strict read and its malformed refusal gain the format key: absent reads as 1, any other number, a float, a string, null or a bool refuses the check as malformed holding nothing of the bytes.
@task_item n=3 part=touches
The kept check file's closed key set and the recorded session kept check decisions: the opening record gained a key; no step or answer key moved.
@task_item n=4 part=touches
The no-seed-on-carriers decision: the founder confirmed it by his own word, so its state moves from provisional to standing and its why no longer says provisional.
@task_item n=5 part=touches
The committed stand Counter checks decision: both committed checks were re-recorded through the binary and now carry the format key.
@task_item n=6 part=touches
The kept check replays through the one run path: every explored call, re-reach steps included, runs through the session's run and its validation, so each explored path's check replays green.
@task_item n=7 part=touches
The recording directory and the owner-only kept check medium: an exploration given a recording directory checks it before anything boots and returns the record-dir-not-private session error when it is group- or other-open; one check per path lands there.
@task_item n=8 part=touches
The kept check size bound: a path deeper than a kept check's 400 records ends its recording and that check is removed, a limit now stated in the capability statement.
@task_item n=9 part=touches
The actions verb's target verbs: exploration is a second reader of the actions table's target verbs and its call naming; the screenshot verb is excluded because its result field is of image kind.
@task_item n=10 part=touches
Validated commands only and the validated verb set widened only by ratification: exploration drives only existing verbs through validation and adds none.
@task_item n=11 part=touches
The driver opens spans and emits no event, the driver library has no side channels and logs no content: exploration adds no span, field, log site, print or env read; each driven call leaves its own command span.
@task_item n=12 part=touches
The telemetry panic hook: a handler panic caught by exploration still runs the process's panic hook before the path is reported as panicked.
@task_item n=13 part=touches
No NodeId crosses a session and the driver public API holds no serde_json types: the new public report types hold closed words, indices, calls and bools only, no NodeId, screen text, digest or panic payload.
@task_item n=14 part=touches
The capability statement and its schema check: a new exploration section and two limits were added; its five counts did not move.
@task_item n=15 part=touches
The new-carrier-widens-boundary decision and the fixture-state-on-carriers absence: exploration is a sibling capability that reaches no carrier, by the founder's own word.
@task_item n=16 part=touches
Integration tests live in blitz-tests, one target per file, scenarios in both layout modes: a new stand target for exploration runs every scenario in both modes.
@task_item n=17 part=touches
Driver unit tests in path children and restricted crate internals: the explore module's tests are a path child, and two actions readers and the recording digest helper became crate-visible, not public.
@task_item n=18 part=touches
The snapshot text to_text contract: exploration dedupes screens by a digest over the snapshot's text form, never kept or logged.
@task_fact n=1 slug=standard-contracts
fact: The driver's explore call runs in process only: it boots once to read the first screen (unrecorded, costing no step), then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot, re-reaching through the earlier calls; a path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; it stops when the frontier empties (exhausted) or the next path would pass the budget; a boot error ends it.
@task_fact n=2 slug=standard-contracts
fact: A step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS), refused as none at 0 and above the bound, and counts every acting call an exploration runs, re-reach steps included.
@task_fact n=3 slug=standard-contracts
fact: An explored path ends accepted (settled or not), refused with its cause, panicked or diverged at a step; a refused, panicked or diverged path is never expanded, and a handler panic is caught per path so the exploration goes on.
@task_fact n=4 slug=standard-contracts
fact: The verbs explored are the actions table's target verbs less any whose result holds an image-kind field, selected by the field's kind and never its name; today click, type and scroll.
@task_fact n=5 slug=standard-contracts
fact: The exploration report holds closed words, indices, calls and bools only; it holds no panic payload, NodeId, screen text or screen digest and is never logged or printed.
@task_fact n=6 slug=standard-contracts
fact: An unbounded app (one whose screens never repeat, such as the Counter) never exhausts the exploration frontier; exhaustion is reached only on a finite set of screens, so the budget is what bounds such an exploration.
@task_fact n=7 slug=standard-contracts
fact: Absence, standing by the founder's own word: exploration on the driver's carriers; no verb, role, argument, MCP tool, session socket call or kept check reaches exploration, which is one in-process library call.
@task_fact n=8 slug=standard-contracts
fact: Absence, provisional pending the founder: exploration follows no navigation; it explores the actions of the app's current screens and never navigates to another document.
@task_fact n=9 slug=standard-contracts
fact: The kept check format number is a public driver constant, CHECK_FORMAT, now 1; the writer always states it in the opening record and the reader accepts only that whole number or its absence.
@task_item n=1 part=untouched
The WPT runner's own panic catch and net provider reset: exploration's panic catch is separate and leaves the runner unchanged.
@task_item n=2 part=untouched
The shell and navigation providers and the harness's dummy navigation provider: they stay true; the provisional no-navigation absence rests on them without changing them.
@task_item n=3 part=untouched
The session socket wire, its versions and bounds, the MCP server protocol and the escher-session command line, roles and exit codes: no message, method, flag or exit moved.
@task_item n=4 part=untouched
Engine rows: style, layout, paint, hit testing, scrolling, select lists, accessibility tree, DOM events, script runtime and fonts.
@task_item n=5 part=untouched
The browser app, its history and persistence, rdme, the WPT runner's outputs and CI artifacts.
@task_item n=6 part=untouched
The stack and dependencies, the Nix flake, cargo-deny, coverage floors, CI workflow legs and their scripts: no dependency, tool, leg or gate changed.
@task_item n=7 part=untouched
The telemetry crate's install, line format, scrub and target filter: no span, field or target was added.
@task_item n=8 part=untouched
Upstream sync, versioning, publishing and the occupied binary, crate and env-var names: no name was taken or freed.
