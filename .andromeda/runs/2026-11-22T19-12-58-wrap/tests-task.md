@task_prose part=session
Epoch 2 corrective: a behaviour-preserving refactor; no public symbol, verb, refusal cause, answer or test name changed.
The driver's call-validation source moved its inline unit test module whole into a sibling child file, its 7 tests keeping their command::tests paths; the source fell from 848 to 249 code lines, the test file 599.
In three upstream blitz-dom bodies (the accessibility node builder, the key-or-input event handler, the presentational-hints computation) escher's additions moved into private functions of escher's own, one call each in upstream's body.
Measured: cognitive complexity 50 to 13, 58 to 46, 176 to 167, the highest fork-own function 14; escher-driver line coverage 97.97 to 97.80 percent against its floor of 97; the stand duplication reading 44 pairs / 372 lines under the 47 / 550 bar.
The lock bumped url 2.5.8 to 2.5.9 and form_urlencoded 1.2.2 to 1.2.3 closing RUSTSEC-2026-0592, with the bindgen and cssparser-macros syn edges kept at 3.0.6 by the operator's ruling, a lock cargo update -p url does not itself produce; the audit leg reads advisories ok, licenses ok with no ignore added.
A plan measurement that fired only the audit leg on that lock move was disproved: the lock it named did not build (E0277 in bindgen), caught by the workspace build legs.
@task_file n=1 path=packages/escher-driver/src/command.rs
what: edited - the driver's call validation (verb table lookup, level check, malformed-call refusals in their fixed order); its inline cfg(test) module replaced by a path-attributed child module declaration
@task_file n=2 path=packages/escher-driver/src/command_tests.rs
what: moved (new file) - the driver call-validation unit tests, the former inline tests module byte-equal, table-driven rows with row-index assertions, 7 tests under command::tests
@task_file n=3 path=packages/blitz-dom/src/accessibility.rs
what: edited - the accessibility tree builder: ARIA expanded, selected, label, toggled and invalid states read once each, now in escher-own helper functions called from upstream's node builder
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs
what: edited - the keyboard and text-input event handler: Tab closing an open select list, and the control-key arms (range input, single list box, drop-down select, multiple select, Enter activation) moved into escher-own helpers
@task_file n=5 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: edited - the presentational hints: a list-box select's size height, a single list box's overflow-y auto and its row blocks, now one escher-own helper call
@task_file n=6 path=Cargo.lock
what: edited - the workspace dependency lock read by the locked cargo legs and the dependency audit: url 2.5.9, form_urlencoded 1.2.3, syn edges of bindgen and cssparser-macros kept at 3.0.6
@task_item n=1 part=touches
The decisions that unit tests sit inline in their source file (inline unit test modules, unit tests in inline test modules, unit tests sit in their source file): the driver call-validation tests now stand in a sibling child file declared by a path attribute, still the source's tests child - an exception or a stale wording.
@task_item n=2 part=touches
The decision that a split test module keeps its test paths: this split held it - 7 command::tests paths unchanged, the body byte-equal.
@task_item n=3 part=touches
The per-crate line coverage floors (both coverage-floor rows): escher-driver reads 97.80 percent against its floor 97, held; the floor value unchanged, 3 of 3 floors held.
@task_item n=4 part=touches
The cognitive-complexity ceiling read on the host: the three upstream bodies read 13, 46 and 167, every fork-own function in them at or under 14; functions over 15 read ours 0, upstream 93.
@task_item n=5 part=touches
The decisions on upstream-authored oversize files and upstream's duplicated test pairs kept as upstream's: escher's files over 800 lines outside those left for later went 1 to 0, and the stand duplication reading holds at 44 pairs / 372 lines under the 47 / 550 bar.
@task_item n=6 part=touches
The dependency audit decision (advisories and licenses): RUSTSEC-2026-0592 closed by the url bump, no ignore added, the audit leg green.
@task_item n=7 part=touches
Cargo legs run locked: the committed lock now holds a state cargo's own url update does not resolve to (syn edges kept at 3.0.6); a later url update re-breaks the locked build until bindgen and prettyplease agree on syn.
@task_item n=8 part=touches
Table-driven tests assert their row count and call assertions carry a row index only: the moved driver validation tests are such tables, unchanged.
@task_item n=9 part=touches
Driver verbs contracts (verb table, lookup, level check) and the eight refusal causes in a fixed order: their unit tests moved file, every claim unchanged.
@task_item n=10 part=touches
The a11y leg membership and the accessibility-graph tests (harness graph tests turning accessibility on): the accessibility node builder's state reads were restructured, each ARIA attribute read once, the a11y leg green.
@task_item n=11 part=touches
Select list and keyboard behaviour rows (select list painted by the scene painter, chosen select row colours, fixture select controlled, change events from the engine alone, focus tests pinning where focus lands): the list-box hints and the control-key and Tab arms moved into helpers, their order and gates unchanged.
@task_item n=12 part=touches
Fast/slow split and the workspace tested in the fast leg: a lock move measured by the audit leg alone proved no build; the fast, a11y and coverage legs caught the unbuildable lock.
@task_fact n=1 slug=9-ci-integration
fact: A lock move is measured by the locked workspace build as well as the audit leg; an audit-only reading of a moved lock proves no build.
@task_item n=1 part=untouched
Session wire, session host, session state directory and session errors contracts: no protocol, bound or error changed.
@task_item n=2 part=untouched
MCP role, command-line, exit-code and help contracts of escher-session: no output or code changed.
@task_item n=3 part=untouched
Cold-agent and agent-run contracts and decisions: not reached.
@task_item n=4 part=untouched
WPT runner, status set, conformance and pinned-commit rows: not reached.
@task_item n=5 part=untouched
Harness construction, settle, virtual clock, advance and screenshot contracts: no harness API changed.
@task_item n=6 part=untouched
Mocking and stubbing discipline, stand-ins and test data fixtures: no fixture or stand-in added.
@task_item n=7 part=untouched
Mutation testing rows and the keyboard compatibility coverage exclusion: no mutation run and no exclusion changed.
@task_item n=8 part=untouched
CI actions pinning, permissions, secrets, cache and package-install rows: no workflow or script changed.
