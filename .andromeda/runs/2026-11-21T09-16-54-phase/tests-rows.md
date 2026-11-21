# test-plan · the rows that bind 2026-11-21-engine-debts — picked 74 of 639 (46 by every draw) · rule any · pin 2822

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/buffered-render-on-dioxus-07-bridge · Buffered render on the Dioxus 0.7 bridge — The Dioxus 0.7 bridge renders into a mutation buffer with the document unborrowed, so a task polled during render can access the document. · why: Upstream's buffered render ported onto the Dioxus 0.7 bridge. · guard: dioxus_runtime_probe
decision/key-after-dropped-anonymous-block · Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts. · why: key_press_after_container_lost_its_box pins no panic, focus staying and the key still moving a select's choice, in both layout modes
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/upstream-buffered-render-test-kept-as-written · Upstream buffered-render test kept as written — Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7. · why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for. · guard: dioxus_runtime_probe
decision/upstream-mounted-flush-test-left-out · Upstream mounted-flush test left out — Upstream's test of an element removed before its mounted event is flushed is left out of the fork until the Dioxus 0.8 move, so the upstream guard on queued mounted events has no test of its own. · why: On Dioxus 0.7 it reads 1 mounted event where it asserts 0, from Dioxus 0.7's scheduling, not the borrow.
decision/failed-load-ends-settle-wait · Failed load ends the settle wait — A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good. · why: the harness test a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it pins it, and the driver's load wait settles through the same harness settle
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/blitz-dom-fixtures-built-manually · blitz-dom fixtures built without the HTML parser — blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML. · why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
decision/incremental-differential-oracle · Incremental layout differential oracle — Incremental layout is checked by running one fixture and mutation sequence in an incremental and a non-incremental document, comparing every node's final layout, layout children and paint tree after each step and asserting no damage remains. · guard: incremental_oracle
decision/pure-restyle-driven-by-hover · Pure-restyle paths driven by hover — Pure-restyle paths are tested by driving :hover rather than by attribute mutation. · why: mutations insert full damage and mask under-damaging bugs

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
contract/harness-settle/notsettled · The harness's settle · NotSettled — Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/bridge-tests-without-widening · Bridge tests without widening visibility — The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them. · why: DioxusEventHandler's two fields are private to that module and it has no constructor · except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
decision/component-alone-props-as-maker · Props of a component alone given as a maker — Props of a component alone are given as a maker run once in the mount's root scope, never as a value. · why: An EventHandler prop can only be made inside a Dioxus runtime.
decision/component-alone-root-context-before-build · Root context of a component alone provided before the build — A root context for a component alone is provided on the mount before the harness builds. · why: A context added after the build reaches no render.
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports keyboard-types' Key, Modifiers and NamedKey, the types press_with takes and the named-key enum a Key::Named holds. · why: so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/parsed-markup-pinned-with-real-parser · Parsed markup pinned where the real parser runs — Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests. · why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
contract/harness-construction/from-component-alone · Harness construction · Harness::from_component_alone — Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
contract/harness-construction/from-html · Harness construction · Harness::from_html — Harness::from_html(html) builds an HTML-backed harness and pumps once.
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-construction/vdom-alone · Harness construction · vdom_alone — vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
contract/harness-core/dispatch · Harness core · dispatch — dispatch does not pump.
contract/harness-core/dispatch-recorded · Harness core · dispatch_recorded — dispatch_recorded([UiEvent, ...]) returns the list of dispatched event names and does not pump.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-core/tick · Harness core · tick — tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/harness-input/key-event · Harness input helpers · key_event — Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
contract/command-span/graph · command span · graph — A graph call's command line records verb alone under its tool_call parent.
contract/command-span/screenshot · command span · screenshot — A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/harness-construction/from-component · Harness construction · Harness::from_component — Harness::from_component(fn) builds a Dioxus-backed harness and pumps once.
contract/harness-input/apple-keybinding · Harness input helpers · apple_keybinding — apple_keybinding(command) dispatches UiEvent::AppleStandardKeybinding(command) to the focused element, the event a macOS window delivers for a standard key binding.
contract/harness-settle/settle-pass-limit · Harness settle · SETTLE_PASS_LIMIT — SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bridge-tests-child-module · Bridge tests a child of the dioxus document — The dioxus document's bridge tests sit in a child module of the dioxus document module. · why: The event handler's two fields are private to that module and it has no constructor.
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
contract/harness-settle/pass-limit · The harness's settle · pass limit — Work for ever ends after exactly the settle pass limit of polls, reading Render.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/recorded-dispatch-bypasses-document · Recorded dispatch bypasses document handling — Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding. · why: pointer_leave, touch_events and range_input_interaction read event names through dispatch_recorded while other checks drive the real pipeline through dispatch

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/file-urls-only-for-file-documents · file: URLs loadable only from a file: document — A `file:` URL loads only for a `file:` document: else a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a sub-resource load, an SVG image or feImage href included, reads no local file, and the browser drops a `file:` favicon or link silently; user loads are not gated. · guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, local_files_for_local_documents
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/buffered-render-unborrowed-document · Buffered render with the document unborrowed — The Dioxus bridge renders into a mutation buffer with the document unborrowed and replays it afterwards, so a task polled during render, such as a MountedData call from a key handler's task, reaches the document without a borrow panic. · why: upstream's buffered render ported onto the Dioxus 0.7 bridge, its Dioxus 0.8 move not taken · guard: mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental in dioxus_runtime_probe

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the files that each witness an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.

## session-lifecycle · key of ## 3. Test Harness Contract
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
contract/driver-session/refusal-order · Driver session · refusal order — A `click` or `type` on an element that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
contract/driver-session/type-replaces · Driver session · type — After the focusing click, where the snapshot reads a value for the target, the driver selects all and types, so an empty text clears the control; a select is focused with no click and no DOM focus event.

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
