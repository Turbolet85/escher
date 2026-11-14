@task_prose part=session
A driver session can now start in a named fixture state of its app: the driver library gains Fixture, FixtureStep, ReachError and Session::reach, which runs each step (click, type, press, advance) through the session's validated run path in order and stops at the first refused or unsettled step.
A fixture name is compared whole against the app's closed table; any other name is refused UnknownState with fixed text, and the stand checks the name before it boots.
The stand gains three states (CRUD fourteen-people and person-selected, Flight Booker return-flight) and a start_in entry; the shared test fixtures gain hold_in beside hold; a new stand check asserts each state reads as its by-hand path from boot, in both layout modes.
In process only, by the founder's own word (F1 = C): no state crosses the command line, MCP, the session socket or a kept recording; no verb, argument, cause, node key, span field, log site or markup changed.
One Android-only compile fix: a bare Result under the dioxus_native prelude glob became std::result::Result.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver's session library — the named fixture state of an app (Fixture, its steps), the closed-table name lookup, the reach error and Session::reach, every step through the validated run path
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit checks of the fixture state — step to call, whole-name lookup, label-form names, fixed reach-error text, stop at the first refused step, busy read only from an unsettled outcome
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the driver session library's crate root — its public surface and crate docs, now exporting the fixture state types
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the seven_guis stand binary's task wiring — its boot per task, now its fixture-state table per task (CRUD, Flight Booker) and start_in, the name checked before the boot
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixtures test module — the fixture boot and hold stated once, now with hold_in, a hold in a named state
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the stand check of a start in a state — each named state reads as its by-hand path from boot and takes the next call alike, an unknown name refused, a recorded session replays from boot, both layout modes
@task_file n=7 path=docs/capabilities.md
what: the capability statement — the driver's capabilities and their reach, now the start in a named state and its in-process-only absence
@task_item n=1 part=touches
decision/stand-expectation-tables and decision/role-name-tables-stated-once: the stand's fixture-state table is a new per-task table stated once beside the stand; check it reads as an addition beside those tables, not a second copy of expected roles or names.
@task_item n=2 part=touches
decision/stand-checks-repeated-three-rounds and decision/a11y-test-leg: a new stand check joins the stand-repeat leg by its stand_ prefix and the test and coverage legs by its file; no leg list or threshold changed.
@task_item n=3 part=touches
decision/stand-builds-no-platform-adapter and absence/assistive-technology-reach: the stand binary changed (start_in, fixture tables) but gained no platform adapter; the absence stands, confirm it.
@task_item n=4 part=touches
decision/driver-requires-accessibility and decision/agent-readers-behind-accessibility: the session library gained a public surface (Fixture, Session::reach) that runs under the same session, accessibility still always on.
@task_item n=5 part=touches
decision/snapshot-model-single-source, decision/snapshot-reads-accessibility-tree and decision/snapshot-reads-the-tree: a session started in a state is asserted to read in its snapshot as the same state reached by hand from boot; no new reader of the tree.
@task_item n=6 part=touches
decision/stand-lint-clean-at-boot and decision/stand-contrast-asserted: the stand now starts in non-boot states (fourteen CRUD people, a selected person, a return flight); the lint and contrast rows read at boot and after input are not asserted in these states by this session.
@task_item n=7 part=touches
contract/crud-list/arrowdown-arrowup-home-end, contract/crud-list/tab-stop and decision/active-descendant-kept-unread: the person-selected and fourteen-people states are reached by clicks on the CRUD create button and the first person row; the list's keys and focus contract are unchanged.
@task_item n=8 part=touches
decision/synthetic-click-at-node-centre and decision/driver-type-focuses-select-no-click: each fixture step is an ordinary driver click, type, press or advance through the validated run path, so these rules govern how a state is reached.
@task_item n=9 part=touches
decision/keyed-wire-form and decision/driver-schema-has-no-role-set: the driver's carriers and schema are unchanged; a fixture state crosses none of them (in process only, ratified by the founder).
@task_item n=10 part=touches
contract/ui-graph-state/use-graph-state, contract/ui-graph-state/no-registered-state, contract/ui-graph-state/unconfirmed and contract/ui-graph-state/undeclared: a named fixture state is a start state reached by steps, not a registered UI graph state; check no row confuses the two.
@task_item n=11 part=touches
decision/capability-statement-reach-by-hand: the capability statement gained the start in a named state and its in-process-only absence, edited by hand.
@task_item n=12 part=touches
decision/stand-shared-tab-order-table: the stand's focus order table is unchanged; a session started in a state still reads the shared table's order (no markup, id or class added to any task).
@task_item n=1 part=untouched
ARIA role mapping, HTML-AAM roles, accessible name precedence and ARIA property and state reads: no role, name, attribute or state read changed.
@task_item n=2 part=untouched
Range input, single-choice and multiple select keyboard and event contracts, and Cells grid keys: no key handling or event order changed.
@task_item n=3 part=untouched
Hidden content, inert subtrees and hit-testing: nothing about exclusion from the tree or hits changed.
@task_item n=4 part=untouched
WCAG contrast thresholds, the lint's contrast finding fields, scrollbar and select-list colours and motion: no colour, token, ratio or finding changed.
@task_item n=5 part=untouched
The platform adapter's creation order, AccessKit event handling and the adapter tree: no adapter code changed.
@task_item n=6 part=untouched
Cognitive accessibility (invalid state not by colour or class alone) and password masking: untouched.
@task_item n=7 part=untouched
The snapshot text form, snapshot diff, node id separator, actions answer and MCP result shapes: no node key, cause, field or wire shape changed.
