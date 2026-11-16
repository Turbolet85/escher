@task_prose part=session
The session added an in-process layout read: why an element sits where it does at its size, from the style and layout chain (v030-11, built and not claimed).
blitz-dom gains a read-only reader on every document, BaseDocument::layout_reason, answering a placing word (PlacedBy), shifts (Scroll, Transform) and per-axis size causes (SizeCause), or None for an unresolved id, a non-element, an unstyled element or a box-less element; a closed select's option reads Select.
escher-driver gains Session::layout, in process only: it resolves its id through the run path's resolve (stale, not-found), answers String ids, closed words and numbers, runs no step, pump, tick or settle, and opens no span; no verb, MCP tool, socket call or kept check reaches it, and that absence of every carrier is PROVISIONAL.
Two new blitz-tests targets, layout_reason and stand_layout, run in both layout modes; stand_layout joins stand-repeat by its prefix and declares session_common; a unit-test child module covers the driver read.
Measured: the plan's pump control of the no-change read did not hold (a pump on a settled screen changes nothing), so a Timer case with delivered, unapplied ticks was added and the control then held; escher-driver line coverage read 97.99 % against a 98.05 % baseline, every floor still held; two functions over the complexity ceiling of 15 were split.
No verb, argument, result field, refusal cause, log field or span field was added; VERBS still holds fourteen and the capability pin stays green.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: new blitz-dom module holding the layout reader on the document and its closed words (placing words, shift kinds, size causes, lengths, size keywords)
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom crate root, edited to declare the layout reader module and re-export its public types
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: dioxus-native-dom crate root, edited to re-export the layout reader's ten types and BoundingRect beside NodeId
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: new escher-driver module holding the driver session's in-process layout read and its answer types, with the reserved app placer
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: new unit-test child module of the driver's layout read, split out by path
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver session's run path executor, edited so its id resolver (stale, not-found) and its target's node are visible inside the crate
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: escher-driver crate root, edited to declare the layout module and re-export its answer types
@task_file n=8 path=docs/capabilities.md
what: the capability statement, pinned to the driver schema, edited with the layout read's rules and the absence of its carrier
@task_file n=9 path=tests/blitz-tests/tests/layout_reason.rs
what: new blitz-tests integration test target reading every placing word and size cause on fixture pages in both layout modes
@task_file n=10 path=tests/blitz-tests/tests/stand_layout.rs
what: new stand check target on the seven_guis served screens, in both layout modes and in stand-repeat, proving the layout read answers every snapshot node and changes nothing
@task_item n=1 part=touches
The one-test-target-per-file and one-target-per-behaviour decisions: blitz-tests gains two targets, layout_reason and stand_layout, each one file and one behaviour.
@task_item n=2 part=touches
The blitz-tests manifest decisions (single test target, difference from upstream): two new target files join the tests directory.
@task_item n=3 part=touches
The both-layout-modes decisions for stand checks and layout assertions: both new targets loop every case over incremental false and true.
@task_item n=4 part=touches
The stand-repeat decisions (three rounds, zero-failure budget, a file joining a leg keeps its legs): stand_layout joins stand-repeat by its stand_ prefix, read 3 of 3 rounds green.
@task_item n=5 part=touches
The per-crate coverage floor decisions and the coverage leg gated by the floors: escher-driver line coverage moved to 97.99 % from a 98.05 % baseline; every crate still holds its floor.
@task_item n=6 part=touches
The cognitive-complexity ceiling read on the host (15): two new functions read 17 and were split to fit, one a stand check's test body.
@task_item n=7 part=touches
The capability statement pinned to the driver schema and the claim section names and counts held to it: the statement gained the layout read and its carrier absence, VERBS still fourteen, the pin green in the fast leg.
@task_item n=8 part=touches
Capability claims name their checks: the layout read's claim is held by layout_reason and stand_layout, and v030-11 stays unclaimed.
@task_item n=9 part=touches
The contrast reader changes nothing decision: a second reader on the document now carries the same read-only rule, asserted by stand_layout's no-change cases.
@task_item n=10 part=touches
Session checks share one module and shared stand tables in one module: session_common gains a declarer, stand_layout.
@task_item n=11 part=touches
The driver session's stale and not-found contract and its record of ids: the layout read resolves its id through the run path's resolver, refuses stale or not-found, and records the screen's ids.
@task_item n=12 part=touches
The driver session's reading verbs, run carriers and the verb table: the layout read is a Session method in process only and no verb or carrier; the absence of every carrier is PROVISIONAL.
@task_item n=13 part=touches
The cold-agent wrong-call bar: it cannot count the layout read while the read stays in process.
@task_item n=14 part=touches
Tests controlled red-before-green: the plan's pump control of the no-change read read DOES NOT HOLD on a settled screen and held only on an added Timer case with pending ticks.
@task_item n=15 part=touches
Table-driven tests assert their row count: layout_reason's placing table holds 13 rows over 11 words and its size table 11 rows over 11 causes.
@task_item n=16 part=touches
Stand timer driven by delivered ticks and timer driven by delivered ticks: stand_layout delivers Timer ticks and reads them unapplied through the read.
@task_item n=17 part=touches
Settle never advances time and the harness core pump: the layout read runs no step, pump, tick or settle.
@task_item n=18 part=touches
No ids or names in stand logs, session logs nothing of the screen and sink capture counted by target: stand_layout's TRACE capture of the read counts 0 escher_ lines.
@task_item n=19 part=touches
A scrolled box reads where it stands: the layout read gives a scrolled holder's child a Scroll shift held by its holder, with the holder's own bounds unmoved.
@task_item n=20 part=touches
Parsed markup pinned with the real parser and blitz-dom fixtures built without the HTML parser: layout_reason's fixtures are HTML pages built through the harness's from_html_with.
@task_item n=21 part=touches
Unit tests in inline modules and split test modules keep their paths: the driver read's unit tests sit in a path-attributed child module of the layout module.
@task_item n=22 part=touches
Stand checks on the bundled font, no skip, and the seven_guis headless stand boots: stand_layout holds served screens, the Timer and cells boots, in both modes.
@task_item n=23 part=touches
The harness inspection layout_rect contract: the layout read answers the snapshot's bounds and an offset chain that holds to 1/64 px beside it.
@task_fact n=1 slug=unit-test-strategy
fact: A control of a read that must change nothing needs pending work a pass would apply; a pump on a settled screen is invisible and no control.
@task_fact n=2 slug=integration-test-strategy
fact: A tolerance stated as to 1/64 px is read inclusively, the absolute difference at most 1/64, since a sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
@task_fact n=3 slug=integration-test-strategy
fact: The layout read changes nothing: no change pending, snapshot, focus, diff and clock unchanged, an open list stays open and delivered Timer ticks stay unapplied, asserted on the stand in both modes.
@task_fact n=4 slug=e2e-test-strategy
fact: The layout read has no session, command-line, MCP or kept-check test, as it has no carrier; that absence is PROVISIONAL until the founder's word.
@task_fact n=5 slug=integration-test-strategy
fact: Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, its tables counting their rows.
@task_item n=1 part=untouched
WPT conformance runner, its suites, statuses, expectations and pinned commit: no WPT test or expectation moved.
@task_item n=2 part=untouched
The agent-run and cold-agent contracts, verbs, events, exit codes and verdict fields: none changed.
@task_item n=3 part=untouched
The session wire, socket, host lifecycle, session errors and session state directory: no message, error or bound was added.
@task_item n=4 part=untouched
The escher-session command line, MCP role, tools list, exit status and recorded checks: no verb, tool, argument or recording field was added.
@task_item n=5 part=untouched
Driver refusal causes and their order: no cause was added; the read reuses stale and not-found.
@task_item n=6 part=untouched
Lint defects, handler coverage, the graph and actions answers and the snapshot text budget: none read or changed.
@task_item n=7 part=untouched
CI workflow jobs, actions pinning, caches, package installs, permissions and secrets: no job or install changed.
@task_item n=8 part=untouched
Pixel, screenshot and contrast tests and the paint path: the layout read paints nothing and no document behaviour changed.
@task_item n=9 part=untouched
Mutation testing tooling and its baseline: no mutation run was taken or changed.
