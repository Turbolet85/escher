# test-plan · the rows that bind 2026-11-18-a11y-re-read — picked 101 of 627 (70 by every draw) · rule any · pin 1905

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/chosen-select-row-colours · Chosen select row colour pair — A chosen select row is painted with the engine default #c8c8c8 on #000000. · why: The pair is Chromium's inactive selected pair, reading 12.56:1. · guard: a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair
decision/failed-load-ends-settle-wait · Failed load ends the settle wait — A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good. · why: the harness test a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it pins it, and the driver's load wait settles through the same harness settle
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/advance-moves-exactly-the-ms-asked · Advance moves exactly the time asked — An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms. · why: ratified · guard: stand_act_timer
decision/key-after-dropped-anonymous-block · Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts. · why: key_press_after_container_lost_its_box pins no panic, focus staying and the key still moving a select's choice, in both layout modes
decision/open-pseudo-class-follows-attribute · Open pseudo-class follows the open attribute — The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute. · why: open_state pins it at construction, on mutator write and removal, and on summary click and Enter, read through an author :open rule

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/pixel-tests-on-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/font-tests-skip-without-font · Pre-existing font tests skip without a font — The pre-existing font-dependent tests skip at runtime when no usable font is available.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/contrast-reader-reads-only · Contrast reader changes nothing — The harness's contrast reader reads and changes nothing: no mutation, input, pump or settle.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/pixel-tests-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.
decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red. · guard: stand_requirement_sweep · except: A requirement with no screen names no witness.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/wcag-2-contrast-ratio · WCAG 2.x contrast ratio — The contrast ratio is the WCAG 2.x ratio over the color crate's relative luminance.
contract/harness-settle/notsettled · The harness's settle · NotSettled — Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
contract/text-contrast/no-reading · The harness's text contrast reading · no reading — No contrast is read where the element bears no text, or where an element on the walk has opacity below 1, a filter, a background image or a mix-blend-mode other than normal.
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/headless-screenshot · Headless screenshot from the harness — The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports keyboard-types' Key, Modifiers and NamedKey, the types press_with takes and the named-key enum a Key::Named holds. · why: so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/one-contrast-formula · One contrast formula — Session lint's contrast reads the same as the harness's text_contrast, with no second formula.
decision/scrolled-box-reads-where-it-stands · A scrolled box reads where it stands — For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do. · why: ratified · guard: scrolled_box_absolute_position
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
decision/virtual-time-for-manual-timers · Virtual time for manual timer driving — blitz-vibey-script's virtual time and without_timer_thread serve embedders that drive timers manually, test runners among them.
contract/harness-contrast/blend-mode · Harness contrast · mix-blend-mode — text_contrast reads None past an element on its walk whose mix-blend-mode is other than normal.
contract/harness-contrast/required-ratio · Harness contrast · required_ratio — TextContrast's required_ratio is 3.0 for large text, else 4.5.
contract/harness-contrast/text-contrast · Harness contrast · text_contrast — text_contrast(NodeId) -> Option<TextContrast> reads computed styles only, opens no span and prints or logs nothing.
contract/harness-core/tick · Harness core · tick — tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
contract/harness-input/clipped-click · Harness input helpers · click on a clipped element — A raw click on an element lying outside the padding box of a box that clips it does not land on it.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/harness-input/key-event · Harness input helpers · key_event — Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
contract/harness-input/pointer-coordinates · Harness input helpers · pointer coordinates — Synthesized pointer events set page, screen and client coordinates to the same values.
contract/harness-input/scroll-into-view · Harness input helpers · scroll_into_view — scroll_into_view(node_id) scrolls at once and by the least movement on each axis, every scrolling box that holds the node and then the viewport; a row of an open select list moves the list's scroll_y alone.
contract/harness-inspect/center-of · Harness inspection helpers · center_of — center_of returns an (x, y) tuple.
contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-settle/busy-loads · Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/settle-pass-limit · Harness settle · SETTLE_PASS_LIMIT — SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
decision/parsed-markup-pinned-with-real-parser · Parsed markup pinned where the real parser runs — Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests. · why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
decision/rust-tests-on-built-in-harness · Rust tests on the built-in harness — Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests. · except: upstream's merged accessibility_label and accessibility_description checks return TestResult<()> and assert with test-that's verify_that!, under the same harness
contract/harness-construction/from-component-alone · Harness construction · Harness::from_component_alone — Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-core/time · Harness core · time — time reads the harness's virtual clock in seconds.
contract/harness-input/apple-keybinding · Harness input helpers · apple_keybinding — apple_keybinding(command) dispatches UiEvent::AppleStandardKeybinding(command) to the focused element, the event a macOS window delivers for a standard key binding.
contract/harness-input/pointer-event · Harness input helpers · pointer_event — The harness crate exports a pointer_event(id, x, y, button, buttons, mods) builder.
contract/harness-screenshot/fonts-unavailable · Harness screenshot · ScreenshotError::FontsUnavailable — A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
contract/harness-screenshot/screenshot · Harness screenshot · screenshot — screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
contract/harness-settle/failed-load · Harness settle · failed load — A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/contrast-required-ratio · Required contrast ratio — Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal. · why: A check computing its expectation from the mutated constant reads green under the mutation.
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
contract/harness-settle/pass-limit · The harness's settle · pass limit — Work for ever ends after exactly the settle pass limit of polls, reading Render.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/harness-settle/notsettled-message · The harness's settle · NotSettled message — Every NotSettled message is non-empty, distinct and holds no slash.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/recorded-dispatch-bypasses-document · Recorded dispatch bypasses document handling — Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding. · why: pointer_leave, touch_events and range_input_interaction read event names through dispatch_recorded while other checks drive the real pipeline through dispatch

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/change-event-from-engine-only · change events come from the engine alone — A page's script hears one `change` per change, fired by the engine; the script runtime synthesizes none. · guard: radio_click_fires_change_only_where_it_was_not_checked
decision/harness-click-waits-on-nothing · Harness click waits on no load — A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`. · why: stand_actions' load_in_flight helper clicks load-fetch through the harness to hold the fetch in flight, and stand_contrast waits out the load itself afterwards
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/handler-coverage/refused-step · handler coverage · refused step — A refused step exercises no handler; a keyboard activation exercises the element's click handler.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/stand-load-page-fixed-answer · Load page stand-in answers after a fixed delay — The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider but the error-text check's failing page, whose stand-in fails its fetch. · why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.
decision/joining-file-keeps-its-legs · A file joining a leg keeps its legs — A test file joining a CI leg keeps every leg it already ran in.
decision/embolden-local-witness · Local witness for font-embolden paint — A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job. · why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux. · guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
decision/advance-moves-clock-exactly · advance moves the clock exactly — Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed. · why: Ratified.

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
