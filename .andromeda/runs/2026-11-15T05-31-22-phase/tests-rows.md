# test-plan · the rows that bind 2026-11-15-upstream-sync-errors-and-explanation — picked 173 of 605 (98 by every draw) · rule any · pin 1072

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
decision/wpt-conformance-runner · WPT conformance runner — Web Platform Tests conformance is run by the project's own WPT runner, covering reftests, checkLayout attr tests, crashtests, testharness.js tests and .any.js and .window.js tests.
decision/wpt-run-on-host-on-fork · WPT run on the host on the fork — The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/advance-moves-exactly-the-ms-asked · Advance moves exactly the time asked — An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms. · why: ratified · guard: stand_act_timer
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/buffered-render-on-dioxus-07-bridge · Buffered render on the Dioxus 0.7 bridge — The Dioxus 0.7 bridge renders into a mutation buffer with the document unborrowed, so a task polled during render can access the document. · why: Upstream's buffered render ported onto the Dioxus 0.7 bridge. · guard: dioxus_runtime_probe
decision/chosen-select-row-colours · Chosen select row colour pair — A chosen select row is painted with the engine default #c8c8c8 on #000000. · why: The pair is Chromium's inactive selected pair, reading 12.56:1. · guard: a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair
decision/key-after-dropped-anonymous-block · Key press after a dropped anonymous block — A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts. · why: key_press_after_container_lost_its_box pins no panic, focus staying and the key still moving a select's choice, in both layout modes
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/password-caret-placed-by-mask · Password caret placed by the mask — A click inside a password field holding a value places the caret by the painted mask bullets, so one click at one offset into two values of one length lands on one index whatever their characters, and a double click selects the whole value. · why: Ratified. · guard: password_caret_click
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/typed-password-never-leaves · A typed password never leaves the instance — A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels. · guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
decision/upstream-buffered-render-test-kept-as-written · Upstream buffered-render test kept as written — Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7. · why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for. · guard: dioxus_runtime_probe
decision/upstream-mounted-flush-test-left-out · Upstream mounted-flush test left out — Upstream's test of an element removed before its mounted event is flushed is left out of the fork until the Dioxus 0.8 move, so the upstream guard on queued mounted events has no test of its own. · why: On Dioxus 0.7 it reads 1 mounted event where it asserts 0, from Dioxus 0.7's scheduling, not the borrow.
decision/advance-reports-exact-ms · Hosted advance reports exactly the ms asked — A hosted advance moves the timer session with no sleep, and every ask reports exactly the milliseconds asked. · why: Ratified. · guard: host_timer
decision/failed-load-ends-settle-wait · Failed load ends the settle wait — A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good. · why: the harness test a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it pins it, and the driver's load wait settles through the same harness settle
decision/open-pseudo-class-follows-attribute · Open pseudo-class follows the open attribute — The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute. · why: open_state pins it at construction, on mutator write and removal, and on summary click and Enter, read through an author :open rule
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/box-shadow-chromium-expectations · Box-shadow expectations from Chromium — Box-shadow pixel expectations come from Chromium renders, with a 1px antialiasing tolerance.
decision/fork-wpt-reading-report-only · Fork WPT reading is report-only — A conformance reading on the fork's dev host is report-only and grades no criterion.
decision/incremental-differential-oracle · Incremental layout differential oracle — Incremental layout is checked by running one fixture and mutation sequence in an incremental and a non-incremental document, comparing every node's final layout, layout children and paint tree after each step and asserting no damage remains. · guard: incremental_oracle
decision/msrv-verified-by-build · MSRV verified by build only — The minimum supported Rust version is verified by build only, not by test.
decision/randomized-template-swap-stress · Randomized template-swap stress — Deterministic pseudo-random template swaps driven by an LCG run 1000 steps. · why: to shake out slab and ElementId reuse bugs · guard: stale_node_mapping
decision/shot-png-deterministic-per-host · Shot PNG deterministic on one host — A shot's PNG equals the harness's raster and is byte-identical across runs on one host.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/pixel-tests-on-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels.
decision/sink-host-output-drained · Output of a sink-installing host drained — A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
decision/wpt-script-tests-ported · WPT script tests ported to Rust — WPT script tests are ported to Rust when the WPT runner cannot run them.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/frames-compared-whole · Frames compared whole — A shot's PNG is compared whole, equal to the harness's raster and byte-identical across runs on one host. · guard: stand_act_screenshot, stand_screenshot_repeat
decision/headless-screenshot · Headless screenshot from the harness — The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
decision/shot-refused-without-fonts · Shot refused without a usable font — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path. · guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/wpt-conformance-report-only · WPT conformance is report-only — A WPT conformance reading is report-only and grades no criterion.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/drain-host-pipes-while-running · Host pipes drained while the host runs — A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading. · why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/pixel-tests-cpu-buffer · Pixel tests on a CPU buffer — Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
decision/scrolled-box-reads-where-it-stands · A scrolled box reads where it stands — For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do. · why: ratified · guard: scrolled_box_absolute_position
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
decision/virtual-time-for-manual-timers · Virtual time for manual timer driving — blitz-vibey-script's virtual time and without_timer_thread serve embedders that drive timers manually, test runners among them.
decision/wpt-suites-run-in-ci · WPT suites run in CI — CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-core/advance-time · Harness core · advance_time — advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-core/tick · Harness core · tick — tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
contract/harness-core/time · Harness core · time — time reads the harness's virtual clock in seconds.
contract/harness-input/clipped-click · Harness input helpers · click on a clipped element — A raw click on an element lying outside the padding box of a box that clips it does not land on it.
contract/harness-input/scroll-into-view · Harness input helpers · scroll_into_view — scroll_into_view(node_id) scrolls at once and by the least movement on each axis, every scrolling box that holds the node and then the viewport; a row of an open select list moves the list's scroll_y alone.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-screenshot/screenshot · Harness screenshot · screenshot — screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
contract/harness-screenshot/screenshot-rect · Harness screenshot · screenshot_rect — screenshot_rect(PixelRect) paints like screenshot at the harness's own time, the rectangle clipped to the viewport; an empty rectangle is EmptyRect.
contract/harness-settle/busy-loads · Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
contract/harness-settle/failed-load · Harness settle · failed load — A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
decision/bridge-tests-without-widening · Bridge tests without widening visibility — The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them. · why: DioxusEventHandler's two fields are private to that module and it has no constructor · except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/parsed-markup-pinned-with-real-parser · Parsed markup pinned where the real parser runs — Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests. · why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
decision/rust-tests-on-built-in-harness · Rust tests on the built-in harness — Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests. · except: upstream's merged accessibility_label and accessibility_description checks return TestResult<()> and assert with test-that's verify_that!, under the same harness
contract/harness-construction/from-component · Harness construction · Harness::from_component — Harness::from_component(fn) builds a Dioxus-backed harness and pumps once.
contract/harness-construction/from-html · Harness construction · Harness::from_html — Harness::from_html(html) builds an HTML-backed harness and pumps once.
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/harness-input/key-event · Harness input helpers · key_event — Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
contract/harness-input/pointer-coordinates · Harness input helpers · pointer coordinates — Synthesized pointer events set page, screen and client coordinates to the same values.
contract/harness-input/pointer-event · Harness input helpers · pointer_event — The harness crate exports a pointer_event(id, x, y, button, buttons, mods) builder.
contract/harness-inspect/focused · Harness inspection helpers · focused — With no node focused, the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
contract/harness-screenshot/fonts-unavailable · Harness screenshot · ScreenshotError::FontsUnavailable — A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
contract/harness-screenshot/no-output · Harness screenshot · output — No screenshot path prints, panics or logs.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/settle-pass-limit · Harness settle · SETTLE_PASS_LIMIT — SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/wpt-cli/calc-scores · the wpt cli · calc-scores — The wpt cli provides a calc-scores command, which CI invokes on the WPT report.
contract/wpt-cli/diff-format-json · the wpt cli · diff --format json — The wpt cli provides a diff command with a JSON output format, which CI invokes.
contract/wpt-runner-outputs/wpt-expectations-txt · WPT runner outputs · wpt_expectations.txt — The WPT runner writes wpt_expectations.txt in its output directory.
contract/wpt-runner-outputs/wptreport-json · WPT runner outputs · wptreport.json — The WPT runner writes wptreport.json in its output directory.
contract/wpt-status/crash · WPT status set · CRASH — CRASH is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/fail · WPT status set · FAIL — FAIL is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/pass · WPT status set · PASS — PASS is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/skip · WPT status set · SKIP — SKIP is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
contract/wpt-status/timeout · WPT status set · TIMEOUT — TIMEOUT is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/blitz-tests-single-test-target · blitz-tests manifest holds one test target — The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: the fork's standing two-line difference from upstream's manifest · guard: BlitzTestsTargetsTest
decision/ci-legs-through-leg-script · CI legs run through the leg script — Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite. · guard: CiWorkflowTest in test_ci_workflows.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/blitz-tests-manifest-difference · blitz-tests manifest's difference from upstream — The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: The fork's standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/upstream-repository-guard · Upstream-only publishing jobs — Every job of publish-browser, wpt and wpt-post-results runs only in the `DioxusLabs/blitz` repository. · guard: UpstreamGuardTest
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/recorded-dispatch-bypasses-document · Recorded dispatch bypasses document handling — Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding. · why: pointer_leave, touch_events and range_input_interaction read event names through dispatch_recorded while other checks drive the real pipeline through dispatch
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/file-urls-only-for-file-documents · file: URLs loadable only from a file: document — A `file:` URL is loaded only for a `file:` document: otherwise a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a blitz-net sub-resource load reads no local file, and the browser app drops a `file:` favicon or link navigation silently; user loads are not gated. · guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, local_files_for_local_documents
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/sink-drops-third-party-targets · Sink drops third-party targets — The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it. · guard: telemetry_drop
decision/advance-accepted-on-every-session · advance accepted on every session — The `advance` verb is accepted on every session, never refused for the session it addresses. · why: ratified
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/start-in-state-reads-as-by-hand-path · Start in a state held to its by-hand path — A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the session reading as boot. · guard: stand_start_in_state
contract/handler-coverage/refused-step · handler coverage · refused step — A refused step exercises no handler; a keyboard activation exercises the element's click handler.
contract/refusal-cause/covered · driver refusal causes · covered — A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
contract/refusal-cause/not-found · driver refusal causes · not-found — A call naming an id not on the screen is refused `not-found`, the instance unchanged.
contract/refusal-cause/off-screen · driver refusal causes · off-screen — A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
contract/refusal-cause/stale · driver refusal causes · stale — A call naming a stale id is refused `stale`, the instance unchanged.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/scroll · escher_driver Session · scroll — A `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view.
contract/session-run/type-on-select · escher_driver Session · type on a select — A `type` on a single-choice select chooses the first option whose label starts with the text, its diff naming the select with `value` and the options whose `selected` moved.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/wpt-upstream-only · WPT workflow on upstream only — The WPT workflow, its runs and its posting of results, runs only on the upstream DioxusLabs/blitz repository, its jobs repository-guarded.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present
decision/buffered-render-unborrowed-document · Buffered render with the document unborrowed — The Dioxus bridge renders into a mutation buffer with the document unborrowed and replays it afterwards, so a task polled during render, such as a MountedData call from a key handler's task, reaches the document without a borrow panic. · why: upstream's buffered render ported onto the Dioxus 0.7 bridge, its Dioxus 0.8 move not taken · guard: mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental in dioxus_runtime_probe

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/stand-load-page-fixed-answer · Load page stand-in answers after a fixed delay — The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider. · why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/advance-moves-clock-by-exactly-ms · Advance moves the virtual clock by exactly ms — The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained. · why: ratified · guard: stand_act_timer
decision/real-provider-inside-stand-in · Real network provider inside a hand-written stand-in — A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it. · guard: a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all
decision/advance-reports-clock-move · advance reports the clock's move — advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained. · why: Ratified.
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/embolden-leg-local-only · Embolden witness leg without a CI job — The font-embolden witness leg runs locally and has no CI job of its own. · why: macOS exercises the feature in CI
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
decision/sink-capture-counted-by-target · Sink capture counted by target — A check that reads a sink's capture counts lines by target, never in total. · why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/ci-on-github-actions · CI on GitHub Actions — GitHub Actions is what runs the project's CI.
decision/ci-trigger-branches · CI trigger branches — Tests run on pull requests and on pushes to main, v0.* and build/** branches, escher's build branch included.
contract/ci-artifacts/leg-log · CI logs and artifacts · target/ci-logs/{leg}.log — Holds each leg's merged output, truncated at the leg's start.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.
decision/blitz-dom-requests-through-stamping-constructor · blitz-dom builds requests through one stamping constructor — blitz-dom builds a network request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect, so the clippy leg fails on any other construction. · why: Upstream's blitz-dom has no such clippy config, a new merge surface. · guard: clippy leg
decision/ci-log-ansi-strip · ANSI strip for CI job-log tallies — A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[. · why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
decision/clippy-legs · Clippy over the workspace and upstream blitz-dom — The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.
decision/dependency-audit-advisories-and-licenses · Dependency audit — The audit leg checks advisories and licenses with cargo-deny over the whole locked workspace with all features.
decision/embolden-local-witness · Local witness for font-embolden paint — A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job. · why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux. · guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted
decision/joining-file-keeps-its-legs · A file joining a leg keeps its legs — A test file joining a CI leg keeps every leg it already ran in.
decision/msrv-1-91 · Minimum supported Rust version — CI builds the workspace with Rust 1.91 as the minimum supported version.
decision/scrollbar-fade-on-document-clock · Overlay scrollbar fade on the document clock — The overlay scrollbar fade reads the document's clock, the harness VirtualClock for a harness document, so a screenshot after a scroll repeats until the clock is advanced. · why: Ratified. · guard: stand_virtual_clock
decision/tally-move-attributed-by-test-name · Tally moves attributed by test name — A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line. · why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/complexity-ceiling-host-only · Cognitive-complexity ceiling read on the host — The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted; no CI leg runs it, it binds no function, and engine functions over it are left as they are. · except: The engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms.
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/advance-moves-clock-exactly · advance moves the clock exactly — Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed. · why: Ratified.
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording

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
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
