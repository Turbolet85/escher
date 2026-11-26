# test-plan · the rows that bind 2026-11-26-canvas-keyboard-path — picked 107 of 654 (59 by every draw) · rule any · pin 4036

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states in words the schema's verb, cause, session-error, node-key and exit-status counts and the a11y and stand-repeat legs' counts; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint's readings over a held instance are tested by integration checks, not unit tests; the canvas label reader, which reads recorded paint commands and no held instance, is unit-tested. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script. · guard: CapabilityStatementTest
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/unrun-declared-write-reads-unconfirmed · Declared write unrun after its event reads unconfirmed — The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it. · why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/no-screenshot-step-in-committed-checks · No screenshot step in a committed check — A committed recorded check holds no screenshot step; repeatability of a shot holding document text is asserted on one host only, while a shot of a canvas alone is held equal across hosts by its answer digest. · why: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64, so such a shot is byte-identical on one host only · guard: stand_screenshot_repeat
decision/open-pseudo-class-follows-attribute · Open pseudo-class follows the open attribute — The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute. · why: open_state pins it at construction, on mutator write and removal, and on summary click and Enter, read through an author :open rule
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found; every id verb but `click` and `layout` refuses a virtual node's id not-found.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cross-os-checks-hold-no-shot · Cross-OS byte comparisons hold no shot — A check compared byte for byte across OSes holds no shot. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · except: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/pixel-tests-on-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels.
decision/shot-png-deterministic-per-host · Shot PNG deterministic on one host — A shot's PNG equals the harness's raster and is byte-identical across runs on one host.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/app-widget-shows-in-shots · App widget painted from Rust shows in shots — A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build. · guard: stand_canvas_shot
decision/canvas-commands-table-across-hosts · Canvas commands held to one table across hosts — Circle Drawer's canvas draw commands are read in process in both layout modes into one committed table, at boot, after a click and at 12 and 100 circles, as summary, full dump and parts, each answer held equal by size and digest on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_canvas_commands
decision/canvas-shots-cross-os-by-digest · Canvas shots compared across hosts by digest — Canvas shots are compared across hosts by one stand check: three canvas-alone fixtures shot whole and as their element, and Circle Drawer's canvas shot as its element, in both layout modes, their answer digests held byte-equal to committed tables, no pixels committed. · why: Ratified as byte equality required, not a tolerance; held on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_canvas_cross_os
decision/canvas-text-on-whole-device-pixels · Canvas text on whole device pixels across hosts — Text an app paints on a canvas that a cross-host shot comparison holds is drawn with its glyph origins on whole device pixels, so Circle Drawer's canvas with its diameter label reads one answer digest on linux, windows and macOS in both layout modes. · why: The CPU renderer snaps glyph origins differently on Apple hosts, so canvas text off whole device pixels differs in bytes across hosts. · guard: stand_canvas_cross_os
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/handler-coverage-by-replay · Handler coverage measured by replay — Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/headless-screenshot · Headless screenshot from the harness — The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
contract/handler-coverage-limits/bubbling-dispatch · Handler coverage limits · bubbling dispatch — A bubbling dispatch that still propagates counts every listener it passed; one its target or an ancestor stops counts its innermost listener alone, and a non-bubbling event its target alone.
contract/handler-coverage-limits/canvas-handler · Handler coverage limits · canvas handler — A canvas's virtual node's click is listed under its `{owner}///{key}` id, owned by its canvas, and exercised where a replayed click named it; a canvas click landing inside a node is counted for the canvas listener alone.
contract/handler-coverage-limits/unreached-screen · Handler coverage limits · unreached screen — A handler on no screen a replay reached is not listed.
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The seven served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/no-shot-in-cross-os-byte-checks · No shot in cross-OS byte comparisons — A check compared byte for byte across OSes holds no shot. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · except: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/pixel-tests-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.
contract/harness-settle/notsettled · The harness's settle · NotSettled — Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/box-presence-read-with-display · Box presence read with computed display — A check asking whether a node has a box reads the node's computed display beside its client rect. · why: A node a restyle turns display none keeps the client rect it last laid out at.
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports keyboard-types' Key, Modifiers and NamedKey, the types press_with takes and the named-key enum a Key::Named holds. · why: so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own
decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: A helper whose body or failure message differs from the shared one stays in its check; stand_element_ids keeps its own boot; task_keys, ceiling, component, the cards array and SECRET stay per file.
contract/harness-core/advance-time · Harness core · advance_time — advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/harness-input/key-event · Harness input helpers · key_event — Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
contract/harness-screenshot/screenshot · Harness screenshot · screenshot — screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
contract/harness-screenshot/screenshot-rect · Harness screenshot · screenshot_rect — screenshot_rect(PixelRect) paints like screenshot at the harness's own time, the rectangle clipped to the viewport; an empty rectangle is EmptyRect.
contract/stand-boot/boot-circle-drawer · seven_guis headless stand · stand::boot_circle_drawer — stand::boot_circle_drawer(HarnessOptions) boots Circle Drawer in process alone, served by no escher-session role.
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
contract/harness-core/tick · Harness core · tick — tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states the schema's counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal. · why: A check computing its expectation from the mutated constant reads green under the mutation. · except: A pixel-row pin that font-embolden moves is pinned as a stated band with its reason in a comment where the test crate cannot see the feature, its wholly-inside assertions kept exact.
decision/focus-test-pins-where-focus-lands · Focus tests pin where focus lands — A focus test pins the node focus lands on, never only that a hidden control is not focused. · why: Once the post-layout focus fixup runs, an assertion that a hidden control is not focused holds with or without the change under test.
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, a case it leaves green widened until it does, and the restored tree reads green; a control removing a bound loosens it to twice the budget instead, so the mutated run stays bounded.
contract/coverage-line/shape · The coverage line · shape — One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved for a canvas's virtual nodes and served in the accessibility tree, the snapshot and its diff, the UI graph, the actions, layout and coverage answers: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
decision/control-leaves-earlier-target-green · Control edit leaves earlier test targets green — A control whose case lies in the later of two test targets named by one gate entry with no fail-fast override needs an edit that leaves the earlier target green. · why: Cargo runs the targets in name order and stops at the first red one, so the named case would go unrun.
decision/inert-guard-dropped · Inert guard dropped for a behaviour reading — A guard no input can turn red is not kept; where its control reads it inert, the guard is dropped and the fact it guarded is pinned by a behaviour reading instead. · provisional
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
contract/coverage-line/owner · The coverage line · owner — A handler's `owner` is left out when it has none.
contract/refusal-causes/printed-form · Refusal causes · printed form — A refusal's printed form opens with its cause's name and ends with its remedy.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/recorded-dispatch-bypasses-document · Recorded dispatch bypasses document handling — Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding. · why: pointer_leave, touch_events and range_input_interaction read event names through dispatch_recorded while other checks drive the real pipeline through dispatch
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words, booleans and a part's part and parts counts only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/handler-coverage-bubbling-counts-ancestors · Bubbling dispatch counts its listeners until stopped — A bubbling dispatch that still propagates counts every listener it passed as exercised; a dispatch stopped by its target or an ancestor counts its innermost listener alone, and a handler's panic unwinding out of the dispatch counts as a stopped dispatch.
decision/handler-coverage-holds-no-typed-text · Handler coverage line holds no typed text — Text typed during a replayed check never reaches the handler coverage line. · guard: stand_handler_coverage
decision/handler-coverage-reads-every-replayed-screen · Handler coverage reads every replayed screen — Handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step.
decision/handler-coverage-unreached-screens · Handlers on unreached screens not listed — Handler coverage does not list a handler on a screen no replay reached. · why: by construction
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/reserved-graph-id-form · Reserved owner-key id form — No id of the lint answer holds the reserved `{owner}///{key}` form; the stand's UI graph and actions answer hold it only for the virtual nodes a canvas declares, Circle Drawer's circles. · guard: stand_graph, the_lint_holds_no_virtual_node
contract/handler-coverage/refused-step · handler coverage · refused step — A refused step exercises no handler; a keyboard activation exercises the element's click handler.
contract/refusal-cause/covered · driver refusal causes · covered — A target a hit at its centre does not reach, or a virtual node with an element over its region's centre, is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
contract/refusal-cause/disabled · driver refusal causes · disabled — A call on a disabled target is refused `disabled`, the instance unchanged.
contract/refusal-cause/off-screen · driver refusal causes · off-screen — A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected`, `invalid` or `highlighted` reading as it reads after the step.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
decision/listeners-read-from-vdom-walk · Listeners read from the vdom walk — A node's current listeners are read from the vdom walk's dynamic attributes, mounted included, not from the bridge's listener attributes. · why: The bridge's listener attributes over-report: a listener removed on a re-render leaves its attribute standing, and a mounted listener writes none. · guard: dioxus_runtime_probe link_2_a_removed_listeners_attribute_stays
contract/handler-coverage/other-app-check · handler coverage · check of another app — A check recorded on another app contributes nothing to the coverage.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/circles-apart-written-through-attribute · Circles apart written through the attribute — A Circle Drawer check that needs circles apart writes `data-circles` through the mutator rather than clicking. · why: Driver clicks on Circle Drawer all land at the canvas's centre.
decision/graph-drives-confirm-every-write · Shared graph drives exercise every stand write — The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag. · why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/doc-leg-gates-public-doc-edits · Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block. · why: The fast leg runs no rustdoc, so a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
contract/ci-artifacts/leg-log · CI logs and artifacts · target/ci-logs/{leg}.log — Holds each leg's merged output, truncated at the leg's start.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the files that each witness an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.
decision/ci-log-ansi-strip · ANSI strip for CI job-log tallies — A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[. · why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold. · why: Ratified as three crates, each floor one point below. · guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate · except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_session_recording · except: A `screenshot` of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/refusal-order · Driver session · refusal order — A `click` or `type` on an element, or a `click` on a virtual node, that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
contract/driver-session/stale-or-not-found · Driver session · stale · not-found — An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not; a virtual node's id is read `not-found` by every verb but `click` and `layout`, which take a virtual node the screen lists.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's whole `UiGraph` when no part is asked and the answer is within the snapshot budget; else `Outcome::GraphPart`, one part of whole nodes with its `part` and `parts`, the states in part 1 alone and a part past the last empty.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph`, `layout` and `canvas` run no step and wait on nothing.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.

# test-plan · its absences, names and overviews, whole

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
> NOT YET MEASURED — the reading gathered no tier justification, critical-path list or coverage triggers for this summary

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
absence/windowed-end-to-end-tests — An automated test that launches a window

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
absence/mocking-library — A mocking library

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
> NOT YET MEASURED — a performance budget: none is set

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-test-decisions-log · ## 12. Test Decisions Log
> NO RECORDED INTENT

## session-lifecycle · key of ## 3. Test Harness Contract
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is no verb of the five-command agent-run contract, which itself starts and binds nothing; its run session selection runs the session binary's cli_flow flows as tests, their hosts started by the flows.
