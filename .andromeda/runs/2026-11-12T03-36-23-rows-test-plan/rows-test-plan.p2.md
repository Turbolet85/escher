@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=S
@row_none n=3 rec=3 type=S
@row_add n=1 row=decision/wpt-suites-run-in-ci area=3-test-harness-contract rec=4
name: WPT suites run in CI
rule: CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.
@row_add n=2 row=contract/wpt-cli/calc-scores area=3-test-harness-contract rec=4
boundary: the wpt cli
item: calc-scores
promise: The wpt cli provides a calc-scores command, which CI invokes on the WPT report.
@row_add n=3 row=contract/wpt-cli/diff-format-json area=3-test-harness-contract rec=4
boundary: the wpt cli
item: diff --format json
promise: The wpt cli provides a diff command with a JSON output format, which CI invokes.
@row_add n=4 row=contract/wpt-runner-cli/wpt-dir area=3-test-harness-contract rec=5
boundary: WPT runner CLI
item: WPT_DIR
promise: The WPT runner reads the WPT_DIR environment variable.
@row_add n=5 row=contract/wpt-runner-cli/suite-arguments area=3-test-harness-contract rec=5
boundary: WPT runner CLI
item: suite arguments
promise: The WPT runner takes optional suite arguments.
@row_add n=6 row=contract/wpt-runner-cli/verbose area=3-test-harness-contract rec=5
boundary: WPT runner CLI
item: --verbose
promise: The WPT runner accepts --verbose, or its short form -v.
@row_add n=7 row=contract/wpt-runner-cli/run-quarantined area=3-test-harness-contract rec=5
boundary: WPT runner CLI
item: --run-quarantined
promise: The WPT runner accepts the --run-quarantined flag.
@row_add n=8 row=contract/wpt-runner-cli/list area=3-test-harness-contract rec=5
boundary: WPT runner CLI
item: --list
promise: The WPT runner accepts the --list flag.
@row_add n=9 row=contract/wpt-runner-outputs/wpt-expectations-txt area=3-test-harness-contract rec=6
boundary: WPT runner outputs
item: wpt_expectations.txt
promise: The WPT runner writes wpt_expectations.txt in its output directory.
@row_add n=10 row=contract/wpt-runner-outputs/wptreport-json area=3-test-harness-contract rec=6,4
boundary: WPT runner outputs
item: wptreport.json
promise: The WPT runner writes wptreport.json in its output directory.
@row_add n=11 row=contract/wpt-status/pass area=3-test-harness-contract rec=7
boundary: WPT status set
item: PASS
promise: PASS is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=12 row=contract/wpt-status/fail area=3-test-harness-contract rec=7
boundary: WPT status set
item: FAIL
promise: FAIL is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=13 row=contract/wpt-status/timeout area=3-test-harness-contract rec=7
boundary: WPT status set
item: TIMEOUT
promise: TIMEOUT is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=14 row=contract/wpt-status/skip area=3-test-harness-contract rec=7
boundary: WPT status set
item: SKIP
promise: SKIP is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=15 row=contract/wpt-status/crash area=3-test-harness-contract rec=7
boundary: WPT status set
item: CRASH
promise: CRASH is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=16 row=decision/python-script-tests-under-unittest area=3-test-harness-contract rec=8
name: Python script tests under unittest
rule: The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
@row_add n=17 row=decision/rust-tests-on-built-in-harness area=3-test-harness-contract rec=9,11
name: Rust tests on the built-in harness
rule: Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests.
except: upstream's merged accessibility_label and accessibility_description checks return TestResult<()> and assert with test-that's verify_that!, under the same harness
@row_none n=4 rec=10 type=4
@row_none n=5 rec=12 type=S
@row_add n=18 row=contract/harness-construction/from-html area=3-test-harness-contract rec=13
boundary: Harness construction
item: Harness::from_html
promise: Harness::from_html(html) builds an HTML-backed harness and pumps once.
@row_add n=19 row=contract/harness-construction/from-html-with area=3-test-harness-contract rec=13
boundary: Harness construction
item: Harness::from_html_with
promise: Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
@row_add n=20 row=contract/harness-construction/from-component area=3-test-harness-contract rec=13
boundary: Harness construction
item: Harness::from_component
promise: Harness::from_component(fn) builds a Dioxus-backed harness and pumps once.
@row_add n=21 row=contract/harness-construction/from-vdom area=3-test-harness-contract rec=13
boundary: Harness construction
item: Harness::from_vdom
promise: Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once.
@row_add n=22 row=contract/harness-construction/wrap area=3-test-harness-contract rec=13
boundary: Harness construction
item: wrap
promise: Harness wrap builds a harness without pumping.
@row_add n=23 row=contract/stand-boot/boot area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::boot
promise: stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
@row_add n=24 row=contract/stand-boot/boot-timer area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::boot_timer
promise: stand::boot_timer(HarnessOptions) boots the Timer in TaskShell and returns the harness with its TimerTicks handle.
@row_add n=25 row=contract/stand-boot/options area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::options
promise: stand::options(incremental) gives the pinned harness options the stand boots with.
@row_add n=26 row=contract/stand-boot/boot-cells area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::boot_cells
promise: stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
@row_add n=27 row=contract/stand-boot/boot-load area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::boot_load
promise: stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
@row_add n=28 row=contract/stand-boot/timer-step area=3-test-harness-contract rec=13
boundary: seven_guis headless stand
item: stand::timer_step
promise: stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
@row_add n=29 row=decision/timer-step-carries-remainder area=3-test-harness-contract rec=13
name: Timer step carries its remainder
rule: The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance.
why: ratified
@row_add n=30 row=contract/harness-core/time area=3-test-harness-contract rec=14
boundary: Harness core
item: time
promise: time reads the harness's virtual clock in seconds.
@row_add n=31 row=contract/harness-core/pump area=3-test-harness-contract rec=14,16
boundary: Harness core
item: pump
promise: pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
@row_add n=32 row=contract/harness-core/tick area=3-test-harness-contract rec=14
boundary: Harness core
item: tick
promise: tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
@row_add n=33 row=contract/harness-core/advance-time area=3-test-harness-contract rec=14
boundary: Harness core
item: advance_time
promise: advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
@row_add n=34 row=contract/harness-screenshot/screenshot area=3-test-harness-contract rec=14
boundary: Harness screenshot
item: screenshot
promise: screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
@row_add n=35 row=contract/harness-screenshot/screenshot-rect area=3-test-harness-contract rec=14
boundary: Harness screenshot
item: screenshot_rect
promise: screenshot_rect(PixelRect) paints like screenshot, the rectangle clipped to the viewport; an empty rectangle gives ScreenshotError::EmptyRect.
@row_add n=36 row=contract/harness-screenshot/fonts-unavailable area=3-test-harness-contract rec=14
boundary: Harness screenshot
item: ScreenshotError::FontsUnavailable
promise: A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
@row_add n=37 row=contract/harness-screenshot/no-output area=3-test-harness-contract rec=14
boundary: Harness screenshot
item: output
promise: No screenshot path prints, panics or logs.
@row_add n=38 row=contract/harness-contrast/text-contrast area=3-test-harness-contract rec=14
boundary: Harness contrast
item: text_contrast
promise: text_contrast(NodeId) -> Option<TextContrast> reads computed styles only, opens no span and prints or logs nothing.
@row_add n=39 row=contract/harness-contrast/required-ratio area=3-test-harness-contract rec=14
boundary: Harness contrast
item: required_ratio
promise: TextContrast's required_ratio is 3.0 for large text, else 4.5.
@row_add n=40 row=decision/harness-reexports-key-types area=3-test-harness-contract rec=14
name: Harness re-exports keyboard types
rule: The test harness re-exports keyboard-types' Key, Modifiers and NamedKey, the types press_with takes and the named-key enum a Key::Named holds.
why: so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own
@row_add n=41 row=decision/harness-virtual-clock area=3-test-harness-contract rec=14
name: Harness time is a virtual clock
rule: The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it.
why: ratified
@row_add n=42 row=contract/harness-settle/settle area=3-test-harness-contract rec=14,16
boundary: Harness settle
item: settle
promise: settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
@row_add n=43 row=contract/harness-contrast/blend-mode area=3-test-harness-contract rec=15
boundary: Harness contrast
item: mix-blend-mode
promise: text_contrast reads None past an element on its walk whose mix-blend-mode is other than normal.
@row_add n=44 row=decision/one-contrast-formula area=3-test-harness-contract rec=15
name: One contrast formula
rule: Session lint's contrast check reads the harness's text_contrast, with no second formula.
@row_add n=45 row=contract/harness-core/dispatch area=3-test-harness-contract rec=16
boundary: Harness core
item: dispatch
promise: dispatch does not pump.
@row_add n=46 row=contract/harness-settle/busy-loads area=3-test-harness-contract rec=16
boundary: Harness settle
item: NotSettled { busy: Loads }
promise: After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
@row_add n=47 row=contract/harness-settle/settle-pass-limit area=3-test-harness-contract rec=16,14
boundary: Harness settle
item: SETTLE_PASS_LIMIT
promise: SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
@row_add n=48 row=contract/harness-settle/side-effects area=3-test-harness-contract rec=16
boundary: Harness settle
item: side effects
promise: settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
@row_add n=49 row=contract/harness-settle/failed-load area=3-test-harness-contract rec=17
boundary: Harness settle
item: failed load
promise: A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.
@row_add n=50 row=contract/harness-core/dispatch-recorded area=3-test-harness-contract rec=18,16
boundary: Harness core
item: dispatch_recorded
promise: dispatch_recorded([UiEvent, ...]) returns the list of dispatched event names and does not pump.
@row_add n=51 row=contract/harness-input/input-helpers area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: input helpers
promise: Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
@row_add n=52 row=contract/harness-input/scroll-into-view area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: scroll_into_view
promise: scroll_into_view(node_id) scrolls at once and by the least movement on each axis, every scrolling box that holds the node and then the viewport; a row of an open select list moves the list's scroll_y alone.
@row_add n=53 row=contract/harness-input/apple-keybinding area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: apple_keybinding
promise: apple_keybinding(command) dispatches UiEvent::AppleStandardKeybinding(command) to the focused element, the event a macOS window delivers for a standard key binding.
@row_add n=54 row=contract/harness-input/clipped-click area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: click on a clipped element
promise: A raw click on an element lying outside the padding box of a box that clips it does not land on it.
@row_add n=55 row=contract/harness-input/pointer-coordinates area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: pointer coordinates
promise: Synthesized pointer events set page, screen and client coordinates to the same values.
@row_add n=56 row=contract/harness-input/key-event area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: key_event
promise: Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
@row_add n=57 row=contract/harness-input/pointer-event area=3-test-harness-contract rec=19
boundary: Harness input helpers
item: pointer_event
promise: The harness crate exports a pointer_event(id, x, y, button, buttons, mods) builder.
@row_add n=58 row=contract/harness-inspect/layout-rect area=3-test-harness-contract rec=20
boundary: Harness inspection helpers
item: layout_rect
promise: layout_rect(selector) returns a rect with x, y, width and height.
@row_add n=59 row=contract/harness-inspect/center-of area=3-test-harness-contract rec=20
boundary: Harness inspection helpers
item: center_of
promise: center_of returns an (x, y) tuple.
@row_add n=60 row=decision/scrolled-box-reads-where-it-stands area=3-test-harness-contract rec=20
name: A scrolled box reads where it stands
rule: For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do.
why: ratified
guard: scrolled_box_absolute_position
@row_none n=6 rec=21 type=S
@row_none n=7 rec=22 type=5
@row_none n=8 rec=23 type=5
@row_add n=61 row=decision/virtual-time-for-manual-timers area=3-test-harness-contract rec=24
name: Virtual time for manual timer driving
rule: blitz-vibey-script's virtual time and without_timer_thread serve embedders that drive timers manually, test runners among them.
@row_add n=62 row=decision/vibey-selection-tests-shape-real-text area=3-test-harness-contract rec=24
name: Vibey-script selection tests shape real text
rule: blitz-vibey-script's dev-dependency on blitz-dom enables system-fonts.
why: so text inputs shape real text in the selection tests
@row_add n=63 row=decision/parsed-markup-pinned-with-real-parser area=3-test-harness-contract rec=25
name: Parsed markup pinned where the real parser runs
rule: Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests.
why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
@row_add n=64 row=decision/bridge-tests-without-widening area=3-test-harness-contract rec=25
name: Bridge tests without widening visibility
rule: The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them.
why: DioxusEventHandler's two fields are private to that module and it has no constructor
except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
@row_add n=65 row=decision/shared-stand-tables-in-one-module area=3-test-harness-contract rec=26
name: Shared stand tables in one module
rule: The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping.
except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
@row_add n=66 row=decision/session-checks-share-one-module area=3-test-harness-contract rec=27
name: Session checks share one module
rule: What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module.
why: kept beside the stand checks' module because that module was held unchanged
@row_add n=67 row=decision/shared-fixtures-in-one-module area=3-test-harness-contract rec=28
name: Shared fixtures in one module
rule: The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net.
except: a page with one reader, or whose ids differ, stays in its own check file
@row_add n=68 row=contract/stand-log/stream area=3-test-harness-contract rec=29
boundary: Stand log
item: stream
promise: The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.
@row_add n=69 row=contract/stand-log/event-line area=3-test-harness-contract rec=29
boundary: Stand log
item: event line
promise: One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
@row_add n=70 row=contract/stand-log/span-line area=3-test-harness-contract rec=29
boundary: Stand log
item: span line
promise: One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
@row_add n=71 row=decision/closed-spans-logged-as-lines area=3-test-harness-contract rec=29
name: Closed spans logged as lines
rule: The stand log carries closed spans its allowlist admits as a second record class beside printed events.
why: a boundary widening, ratified, as the obs-plan's log format states
@row_add n=72 row=contract/stand-log/service-name area=3-test-harness-contract rec=29
boundary: Stand log
item: service.name
promise: Both seven_guis binaries log under service.name=seven_guis.
@row_add n=73 row=decision/telemetry-target-allowlist area=3-test-harness-contract rec=29
name: Telemetry target allowlist
rule: A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names.
guard: telemetry_scrub, telemetry_drop
@row_add n=74 row=decision/no-ids-or-names-in-stand-logs area=3-test-harness-contract rec=29
name: No ids or names in stand logs
rule: The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed.
guard: host_log, mcp_log, stand_act_spans
@row_add n=75 row=contract/stand-log/command-span area=3-test-harness-contract rec=29
boundary: Stand log
item: command span
promise: escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
@row_add n=76 row=contract/stand-log/tool-call-span area=3-test-harness-contract rec=29
boundary: Stand log
item: tool_call span
promise: The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
@row_add n=77 row=contract/tool-call-span/ending area=3-test-harness-contract rec=29
boundary: tool_call span
item: ending
promise: ending is one of accepted, refused or error.
@row_add n=78 row=contract/tool-call-span/help area=3-test-harness-contract rec=29
boundary: tool_call span
item: help
promise: A help call, answered by the server, writes one tool_call line, tool="help" ending="accepted", with no command line beneath it.
@row_add n=79 row=contract/command-span/verb area=3-test-harness-contract rec=29
boundary: command span
item: verb
promise: verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
@row_add n=80 row=contract/command-span/screenshot area=3-test-harness-contract rec=29
boundary: command span
item: screenshot
promise: A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
@row_add n=81 row=contract/command-span/graph area=3-test-harness-contract rec=29
boundary: command span
item: graph
promise: A graph call's command line records verb alone under its tool_call parent.
@row_add n=82 row=contract/mcp-role/stdout area=3-test-harness-contract rec=29
boundary: escher-session mcp role
item: stdout
promise: The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
@row_add n=83 row=contract/host-role/stdout area=3-test-harness-contract rec=29
boundary: escher-session host role
item: stdout
promise: The host role writes nothing to stdout.
@row_add n=84 row=contract/cli-command/stdout area=3-test-harness-contract rec=29
boundary: escher-session commands
item: stdout
promise: A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
@row_add n=85 row=contract/cli-command/stderr area=3-test-harness-contract rec=29
boundary: escher-session commands
item: stderr
promise: An accepted command's stderr is empty with RUST_LOG unset.
@row_none n=9 rec=30 type=S
@row_add n=86 row=contract/agent-run/invocation area=3-test-harness-contract rec=31
boundary: agent-run
item: invocation
promise: bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}, run from the repository root.
@row_add n=87 row=contract/agent-run/windows-entry area=3-test-harness-contract rec=31
boundary: agent-run
item: agent-run.ps1
promise: The Windows entry passes every argument unchanged to the bash script and exits with its code, holding no contract logic.
@row_add n=88 row=decision/agent-run-holds-no-process area=3-test-harness-contract rec=31
name: Agent-run holds no process
rule: The agent-run script starts, polls and listens on nothing (no daemon, PID file, status endpoint, port or socket) and reads no environment variable of its own; the stand checks it runs boot in process.
except: stand_session_lifecycle and stand_session_quiet start and stop a session host process and its Unix-domain socket, a surface of the session library, not of this contract
@row_add n=89 row=contract/agent-run-exit-codes/0 area=3-test-harness-contract rec=32
boundary: agent-run exit codes
item: 0
promise: Success.
@row_add n=90 row=contract/agent-run-exit-codes/1 area=3-test-harness-contract rec=32
boundary: agent-run exit codes
item: 1
promise: The verb ran and failed: a cargo build failure, a failing run, an empty run.
@row_add n=91 row=contract/agent-run-exit-codes/2 area=3-test-harness-contract rec=32
boundary: agent-run exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, an unknown selection; a usage error is checked before not-booted.
@row_add n=92 row=contract/agent-run-exit-codes/3 area=3-test-harness-contract rec=32
boundary: agent-run exit codes
item: 3
promise: A precondition is unmet: not booted, or logs with no events file.
@row_add n=93 row=contract/agent-run-verbs/boot area=3-test-harness-contract rec=33
boundary: agent-run verbs
item: boot
promise: boot recreates target/agent-run/ with a fresh events.jsonl, builds every blitz-tests test binary (--locked --no-run, output to run.log), writes status.json (booted, boot_ts, head) and emits the boot event; exit 0 ready, 1 failed.
@row_add n=94 row=contract/agent-run-verbs/run area=3-test-harness-contract rec=34
boundary: agent-run verbs
item: run
promise: run {selection} runs cargo test -p blitz-tests --locked with merged output to run.log, truncated at the run's start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
@row_add n=95 row=contract/agent-run-selection/stand area=3-test-harness-contract rec=34
boundary: agent-run run selection
item: stand
promise: stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
@row_add n=96 row=contract/agent-run-selection/all area=3-test-harness-contract rec=34
boundary: agent-run run selection
item: all
promise: all runs the whole package, with no --test.
@row_add n=97 row=contract/agent-run-selection/name area=3-test-harness-contract rec=34
boundary: agent-run run selection
item: {name}
promise: {name} runs one file, matching ^[a-z0-9_]+$ with tests/blitz-tests/tests/{name}.rs present; any other selection exits 2.
@row_add n=98 row=decision/empty-run-is-never-a-pass area=3-test-harness-contract rec=34
name: An empty run is never a pass
rule: A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
@row_add n=99 row=contract/agent-run-verbs/status area=3-test-harness-contract rec=35
boundary: agent-run verbs
item: status
promise: status prints one status object from status.json and builds or runs nothing; exit 0 booted, 3 not (no status.json, or booted false).
@row_add n=100 row=contract/agent-run-verbs/cleanup area=3-test-harness-contract rec=36
boundary: agent-run verbs
item: cleanup
promise: cleanup removes target/agent-run/ if present and prints the cleanup event; idempotent, exit 0 on a clean tree, touching nothing else under target/.
@row_add n=101 row=contract/agent-run-verbs/logs area=3-test-harness-contract rec=37
boundary: agent-run verbs
item: logs
promise: logs prints events.jsonl verbatim, one JSON object per line, exit 3 when it is absent; raw cargo output stays in run.log, never printed.
@row_add n=102 row=decision/agent-run-events-json-by-python area=3-test-harness-contract rec=38
name: Agent-run events encoded by python3 json
rule: Agent-run's JSON-lines events are encoded by python3's json module embedded in the script.
@row_add n=103 row=contract/agent-run-events/boot area=3-test-harness-contract rec=38
boundary: agent-run events
item: boot
promise: boot {ts, outcome: ready|failed, cargo_exit, head}, printed to stdout and appended to events.jsonl.
@row_add n=104 row=contract/agent-run-events/run-start area=3-test-harness-contract rec=38
boundary: agent-run events
item: run.start
promise: run.start {ts, selection, files}, files the --test stems passed ([] under all), printed to stdout and appended to events.jsonl.
@row_add n=105 row=contract/agent-run-events/test area=3-test-harness-contract rec=38
boundary: agent-run events
item: test
promise: test {file, test, outcome: ok|failed|ignored}, one per libtest line, file the target binary's stem with its hash stripped or doc:{crate}; printed to stdout and appended to events.jsonl.
@row_add n=106 row=contract/agent-run-events/run-end area=3-test-harness-contract rec=38
boundary: agent-run events
item: run.end
promise: run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome: passed|failed}, printed to stdout and appended to events.jsonl.
@row_add n=107 row=contract/agent-run-events/status area=3-test-harness-contract rec=38
boundary: agent-run events
item: status
promise: status {booted, boot_ts, head, run: {selection, state: none|running|passed|failed, passed, failed, ignored}}, printed only.
@row_add n=108 row=contract/agent-run-events/cleanup area=3-test-harness-contract rec=38
boundary: agent-run events
item: cleanup
promise: cleanup {outcome: done}, printed only.
@row_add n=109 row=decision/test-output-never-reaches-events area=3-test-harness-contract rec=38
name: Test output never reaches agent-run events
rule: Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
@row_add n=110 row=decision/agent-run-events-are-harness-metadata area=3-test-harness-contract rec=38
name: Agent-run events are harness metadata
rule: Agent-run's events are harness metadata, not an escher-telemetry sink.
@row_add n=111 row=contract/agent-run-state/files area=3-test-harness-contract rec=39
boundary: agent-run state
item: files
promise: Agent-run's state is target/agent-run/{status.json, events.jsonl, run.log} only, gitignored under target/.
@row_add n=112 row=decision/one-agent-run-driver-at-a-time area=3-test-harness-contract rec=39
name: One agent-run driver at a time
rule: One agent-run driver runs at a time: a killed run leaves running behind, which the next run overwrites.
@row_none n=10 rec=40 type=5
