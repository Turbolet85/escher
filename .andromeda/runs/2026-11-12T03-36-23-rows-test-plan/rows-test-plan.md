@row_add n=1 row=decision/test-tier area=1-test-scope-summary rec=fct-qaaszuyh@1
name: Test tier
rule: The project's test tier is 0.
@row_none n=1 rec=fct-aa4u6dgj@1 type=S
@row_add n=2 row=decision/workspace-tests-in-fast-ci-leg area=1-test-scope-summary rec=fct-7exig2pt@1
name: Workspace tested in the fast CI leg
rule: Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
@row_none n=2 rec=fct-ooxsszz3@1 type=5
@row_none n=3 rec=fct-4kvmkl3r@1 type=5
@row_none n=4 rec=fct-7zigd34v@1 type=5
@row_none n=5 rec=fct-kvmzckeg@1 type=5
@row_none n=6 rec=fct-lkajfk2h@1 type=5
@row_none n=7 rec=fct-jp5khdk7@1 type=5
@row_none n=8 rec=fct-xj4chjtp@1 type=5
@row_none n=9 rec=fct-36efgbrv@1 type=5
@row_none n=10 rec=fct-u3g4qsnu@1 type=5
@row_none n=11 rec=fct-exzm4nio@1 type=5
@row_none n=12 rec=fct-jtnnwppy@1 type=5
@row_add n=3 row=decision/dedicated-headless-test-harness area=1-test-scope-summary rec=fct-zlogblkv@1
name: Dedicated headless test harness
rule: Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
@row_none n=13 rec=fct-3sg4hagn@1 type=5
@row_add n=4 row=decision/driver-executor-integration-tested area=1-test-scope-summary rec=fct-optamucx@1
name: Driver executor tested by integration checks
rule: The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests.
why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
@row_none n=14 rec=fct-p2fc4upb@1 type=5
@row_none n=15 rec=fct-a3ufmheh@1 type=7
@row_none n=16 rec=fct-jydhmppz@5 type=5
@row_none n=17 rec=fct-t7ihtp4n@1 type=7
@row_add n=5 row=decision/one-test-target-per-behaviour area=1-test-scope-summary rec=fct-aitehs62@3,fct-by5wg4wl@1
name: One integration test target per behaviour
rule: The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target.
why: differs from upstream, whose layout is one all target
guard: BlitzTestsTargetsTest
@row_none n=18 rec=fct-amhvfjwy@1 type=5
@row_add n=6 row=decision/wpt-conformance-runner area=1-test-scope-summary rec=fct-uczket25@1
name: WPT conformance runner
rule: Web Platform Tests conformance is run by the project's own WPT runner, covering reftests, checkLayout attr tests, crashtests, testharness.js tests and .any.js and .window.js tests.
@row_none n=19 rec=fct-hm4fszge@1 type=5
@row_none n=20 rec=fct-ofo6o2ym@1 type=S
@row_add n=7 row=decision/native-platform-ci-matrix area=1-test-scope-summary rec=fct-7m2wyfwg@1
name: Native platform CI matrix
rule: CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
@row_add n=8 row=decision/wpt-run-on-host-on-fork area=1-test-scope-summary rec=fct-3icuiuwg@1
name: WPT run on the host on the fork
rule: The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
@row_none n=21 rec=fct-s7fmur4k@1 type=S
@row_none n=22 rec=fct-p3ob2i7k@1 type=S
@row_add n=9 row=decision/inline-unit-test-modules area=2-test-strategy rec=fct-opx6s62j@1,fct-by5wg4wl@1
name: Inline unit test modules
rule: Unit tests are Rust test functions in cfg(test) modules beside the code under test.
except: modules held in files of their own through a path attribute
@row_add n=10 row=decision/blitz-dom-fixtures-built-manually area=2-test-strategy rec=fct-rxiov6jw@1
name: blitz-dom fixtures built without the HTML parser
rule: blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML.
why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
@row_none n=23 rec=fct-p6hppuul@1 type=4
@row_none n=24 rec=fct-bloeybfr@1 type=4
relied: Snapshot-style assertions rely on the harness's DOM string being a stable one-node-per-line tree serialization.
@row_none n=25 rec=fct-xyq3mbup@1 type=4
@row_add n=11 row=decision/session-check-owns-its-host area=2-test-strategy rec=fct-rpxytbam@1
name: A session check owns its host
rule: A check that needs a session process starts and stops its own host, with every wait bounded.
@row_add n=12 row=decision/failed-check-host-reaped area=2-test-strategy rec=fct-rpxytbam@1
name: Host of a failing check reaped
rule: A guard kills and reaps a host that a failing check still holds.
@row_add n=13 row=decision/check-host-state-dir-removed area=2-test-strategy rec=fct-rpxytbam@1
name: Check host state directory removed
rule: A check's host state directory is removed at stop.
@row_add n=14 row=decision/check-child-streams-to-null area=2-test-strategy rec=fct-rpxytbam@1
name: Check child streams sent to null
rule: A check's child process streams are sent to null unless the check reads them.
why: so the child's libtest lines never reach the agent run script's count
@row_add n=15 row=decision/socket-session-checks-unix-only area=2-test-strategy rec=fct-rpxytbam@1
name: Socket session checks gated to unix
rule: A check holding a session over the socket is gated to unix.
@row_add n=16 row=decision/sink-host-output-drained area=2-test-strategy rec=fct-jxt7cq26@1
name: Output of a sink-installing host drained
rule: A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
@row_add n=17 row=decision/cli-checks-run-own-binary area=2-test-strategy rec=fct-nsfd2ghz@1
name: Command-line checks run the package's own binary
rule: seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
@row_add n=18 row=decision/shell-flows-under-cargo-test area=2-test-strategy rec=fct-nsfd2ghz@1
name: Shell flows run under cargo test
rule: The POSIX sh flow scripts are assets of a cargo test target, not a runner of their own, and no test framework is added for them.
@row_add n=19 row=decision/seven-guis-cli-targets-unix-only area=2-test-strategy rec=fct-nsfd2ghz@1
name: seven_guis command-line targets gated to unix
rule: seven_guis' command-line and shell-flow test targets are gated to unix.
@row_add n=20 row=decision/incremental-differential-oracle area=2-test-strategy rec=fct-hzkxjxb5@1
name: Incremental layout differential oracle
rule: Incremental layout is checked by running one fixture and mutation sequence in an incremental and a non-incremental document, comparing every node's final layout, layout children and paint tree after each step and asserting no damage remains.
guard: incremental_oracle
@row_add n=21 row=decision/pixel-tests-on-cpu-buffer area=2-test-strategy rec=fct-sphnp6kt@1
name: Pixel tests on a CPU buffer
rule: Pixel tests render to a CPU buffer and sample pixels.
@row_add n=22 row=decision/box-shadow-chromium-expectations area=2-test-strategy rec=fct-sphnp6kt@1
name: Box-shadow expectations from Chromium
rule: Box-shadow pixel expectations come from Chromium renders, with a 1px antialiasing tolerance.
@row_add n=23 row=decision/shot-png-deterministic-per-host area=2-test-strategy rec=fct-sphnp6kt@1
name: Shot PNG deterministic on one host
rule: A shot's PNG equals the harness's raster and is byte-identical across runs on one host.
@row_add n=24 row=decision/cross-os-checks-hold-no-shot area=2-test-strategy rec=fct-sphnp6kt@1
name: Cross-OS byte comparisons hold no shot
rule: A check compared byte for byte across OSes holds no shot.
why: a shot's PNG bytes differ between OSes
@row_add n=25 row=decision/randomized-template-swap-stress area=2-test-strategy rec=fct-ky3gzvfl@1
name: Randomized template-swap stress
rule: Deterministic pseudo-random template swaps driven by an LCG run 1000 steps.
why: to shake out slab and ElementId reuse bugs
guard: stale_node_mapping
@row_add n=26 row=decision/pure-restyle-driven-by-hover area=2-test-strategy rec=fct-5jojzc3q@1
name: Pure-restyle paths driven by hover
rule: Pure-restyle paths are tested by driving :hover rather than by attribute mutation.
why: mutations insert full damage and mask under-damaging bugs
@row_add n=27 row=decision/font-tests-skip-without-font area=2-test-strategy rec=fct-bz62prox@1
name: Pre-existing font tests skip without a font
rule: The pre-existing font-dependent tests skip at runtime when no usable font is available.
@row_add n=28 row=decision/stand-checks-bundled-font-no-skip area=2-test-strategy rec=fct-bz62prox@1
name: Stand checks on the bundled font, no skip
rule: The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
@row_none n=26 rec=fct-spkohny7@1 type=4
@row_add n=29 row=decision/fork-wpt-reading-report-only area=2-test-strategy rec=fct-4rwtva2s@1
name: Fork WPT reading is report-only
rule: A conformance reading on the fork's dev host is report-only and grades no criterion.
@row_none n=27 rec=fct-fofajqsn@1 type=4
@row_add n=30 row=decision/wpt-script-tests-ported area=2-test-strategy rec=fct-fzigja74@1
name: WPT script tests ported to Rust
rule: WPT script tests are ported to Rust when the WPT runner cannot run them.
@row_add n=31 row=decision/msrv-verified-by-build area=2-test-strategy rec=fct-l2wptood@1
name: MSRV verified by build only
rule: The minimum supported Rust version is verified by build only, not by test.
@row_add n=32 row=decision/example-fixtures-human-read area=2-test-strategy rec=fct-mycmwhk6@1
name: Example fixtures read by a human
rule: Example fixtures are read by a human viewer, their prose and comments stating the expected rendering and what each case checks.
@row_add n=33 row=decision/real-page-snapshots-paired-reduced area=2-test-strategy rec=fct-mycmwhk6@1
name: Real-page snapshots paired with reduced variants
rule: Full real-page snapshot fixtures are paired with reduced variants.
@row_none n=28 rec=fct-rwlqbpe4@1 type=7
@row_none n=29 rec=fct-ikboyqku@1 type=S
@row_add n=34 row=decision/regression-tests-named-for-bug area=2-test-strategy rec=fct-gldoaooh@1
name: Regression tests named for their bug
rule: A regression test is named for the bug it guards.
@row_add n=35 row=decision/regression-tests-document-bug area=2-test-strategy rec=fct-gldoaooh@1
name: Regression tests document their bug
rule: A regression test documents the bug and the real-world page that exposed it.
@row_none n=30 rec=fct-3tbftfp4@1 type=5
@row_none n=31 rec=fct-ebblxmch@1 type=S
@row_none n=32 rec=fct-bxrtvjxs@1 type=S
@row_add n=36 row=decision/wpt-suites-run-in-ci area=3-test-harness-contract rec=fct-ugiopwd2@1
name: WPT suites run in CI
rule: CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.
@row_add n=37 row=contract/wpt-cli/calc-scores area=3-test-harness-contract rec=fct-ugiopwd2@1
boundary: the wpt cli
item: calc-scores
promise: The wpt cli provides a calc-scores command, which CI invokes on the WPT report.
@row_add n=38 row=contract/wpt-cli/diff-format-json area=3-test-harness-contract rec=fct-ugiopwd2@1
boundary: the wpt cli
item: diff --format json
promise: The wpt cli provides a diff command with a JSON output format, which CI invokes.
@row_add n=39 row=contract/wpt-runner-cli/wpt-dir area=3-test-harness-contract rec=fct-mxlrprf2@1
boundary: WPT runner CLI
item: WPT_DIR
promise: The WPT runner reads the WPT_DIR environment variable.
@row_add n=40 row=contract/wpt-runner-cli/suite-arguments area=3-test-harness-contract rec=fct-mxlrprf2@1
boundary: WPT runner CLI
item: suite arguments
promise: The WPT runner takes optional suite arguments.
@row_add n=41 row=contract/wpt-runner-cli/verbose area=3-test-harness-contract rec=fct-mxlrprf2@1
boundary: WPT runner CLI
item: --verbose
promise: The WPT runner accepts --verbose, or its short form -v.
@row_add n=42 row=contract/wpt-runner-cli/run-quarantined area=3-test-harness-contract rec=fct-mxlrprf2@1
boundary: WPT runner CLI
item: --run-quarantined
promise: The WPT runner accepts the --run-quarantined flag.
@row_add n=43 row=contract/wpt-runner-cli/list area=3-test-harness-contract rec=fct-mxlrprf2@1
boundary: WPT runner CLI
item: --list
promise: The WPT runner accepts the --list flag.
@row_add n=44 row=contract/wpt-runner-outputs/wpt-expectations-txt area=3-test-harness-contract rec=fct-c4l2xdp7@1
boundary: WPT runner outputs
item: wpt_expectations.txt
promise: The WPT runner writes wpt_expectations.txt in its output directory.
@row_add n=45 row=contract/wpt-runner-outputs/wptreport-json area=3-test-harness-contract rec=fct-c4l2xdp7@1,fct-ugiopwd2@1
boundary: WPT runner outputs
item: wptreport.json
promise: The WPT runner writes wptreport.json in its output directory.
@row_add n=46 row=contract/wpt-status/pass area=3-test-harness-contract rec=fct-scwehmef@1
boundary: WPT status set
item: PASS
promise: PASS is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=47 row=contract/wpt-status/fail area=3-test-harness-contract rec=fct-scwehmef@1
boundary: WPT status set
item: FAIL
promise: FAIL is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=48 row=contract/wpt-status/timeout area=3-test-harness-contract rec=fct-scwehmef@1
boundary: WPT status set
item: TIMEOUT
promise: TIMEOUT is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=49 row=contract/wpt-status/skip area=3-test-harness-contract rec=fct-scwehmef@1
boundary: WPT status set
item: SKIP
promise: SKIP is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=50 row=contract/wpt-status/crash area=3-test-harness-contract rec=fct-scwehmef@1
boundary: WPT status set
item: CRASH
promise: CRASH is one of the five WPT statuses: PASS, FAIL, TIMEOUT, SKIP, CRASH.
@row_add n=51 row=decision/python-script-tests-under-unittest area=3-test-harness-contract rec=fct-r7ljhf6w@1
name: Python script tests under unittest
rule: The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
@row_add n=52 row=decision/rust-tests-on-built-in-harness area=3-test-harness-contract rec=fct-rt66jcaz@1,fct-abp6m5eo@1
name: Rust tests on the built-in harness
rule: Rust tests run under the standard built-in harness: #[cfg(test)] modules of #[test] functions asserting with assert! and assert_eq!, and #[tokio::test] for async worker tests.
except: upstream's merged accessibility_label and accessibility_description checks return TestResult<()> and assert with test-that's verify_that!, under the same harness
@row_none n=33 rec=fct-2lgshksu@1 type=4
@row_none n=34 rec=fct-gn6s4wro@1 type=S
@row_add n=53 row=contract/harness-construction/from-html area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: Harness construction
item: Harness::from_html
promise: Harness::from_html(html) builds an HTML-backed harness and pumps once.
@row_add n=54 row=contract/harness-construction/from-html-with area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: Harness construction
item: Harness::from_html_with
promise: Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
@row_add n=55 row=contract/harness-construction/from-component area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: Harness construction
item: Harness::from_component
promise: Harness::from_component(fn) builds a Dioxus-backed harness and pumps once.
@row_add n=56 row=contract/harness-construction/from-vdom area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: Harness construction
item: Harness::from_vdom
promise: Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once.
@row_add n=57 row=contract/harness-construction/wrap area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: Harness construction
item: wrap
promise: Harness wrap builds a harness without pumping.
@row_add n=58 row=contract/stand-boot/boot area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: stand::boot
promise: stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
@row_add n=59 row=contract/stand-boot/options area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: stand::options
promise: stand::options(incremental) gives the pinned harness options the stand boots with.
@row_add n=60 row=contract/stand-boot/boot-cells area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: stand::boot_cells
promise: stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
@row_add n=61 row=contract/stand-boot/boot-load area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: stand::boot_load
promise: stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
@row_add n=62 row=contract/stand-boot/timer-step area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: stand::timer_step
promise: stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
@row_add n=63 row=decision/timer-step-carries-remainder area=3-test-harness-contract rec=fct-65u33zgx@1
name: Timer step carries its remainder
rule: The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance.
why: ratified
@row_add n=64 row=contract/harness-core/time area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness core
item: time
promise: time reads the harness's virtual clock in seconds.
@row_add n=65 row=contract/harness-core/pump area=3-test-harness-contract rec=fct-33n62qwf@1,fct-e36kow75@1
boundary: Harness core
item: pump
promise: pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
@row_add n=66 row=contract/harness-core/tick area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness core
item: tick
promise: tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
@row_add n=67 row=contract/harness-core/advance-time area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness core
item: advance_time
promise: advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
@row_add n=68 row=contract/harness-screenshot/screenshot area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness screenshot
item: screenshot
promise: screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
@row_add n=69 row=contract/harness-screenshot/fonts-unavailable area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness screenshot
item: ScreenshotError::FontsUnavailable
promise: A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
@row_add n=70 row=contract/harness-screenshot/no-output area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness screenshot
item: output
promise: No screenshot path prints, panics or logs.
@row_add n=71 row=contract/harness-contrast/text-contrast area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness contrast
item: text_contrast
promise: text_contrast(NodeId) -> Option<TextContrast> reads computed styles only, opens no span and prints or logs nothing.
@row_add n=72 row=contract/harness-contrast/required-ratio area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness contrast
item: required_ratio
promise: TextContrast's required_ratio is 3.0 for large text, else 4.5.
@row_add n=73 row=decision/harness-reexports-key-types area=3-test-harness-contract rec=fct-33n62qwf@1
name: Harness re-exports keyboard types
rule: The test harness re-exports keyboard-types' Key, Modifiers and NamedKey, the types press_with takes and the named-key enum a Key::Named holds.
why: so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own
@row_add n=74 row=decision/harness-virtual-clock area=3-test-harness-contract rec=fct-33n62qwf@1
name: Harness time is a virtual clock
rule: The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it.
why: ratified
@row_add n=75 row=contract/harness-settle/settle area=3-test-harness-contract rec=fct-33n62qwf@1,fct-e36kow75@1
boundary: Harness settle
item: settle
promise: settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
@row_add n=76 row=contract/harness-contrast/blend-mode area=3-test-harness-contract rec=fct-nas4n737@1
boundary: Harness contrast
item: mix-blend-mode
promise: text_contrast reads None past an element on its walk whose mix-blend-mode is other than normal.
@row_add n=77 row=contract/harness-core/dispatch area=3-test-harness-contract rec=fct-e36kow75@1
boundary: Harness core
item: dispatch
promise: dispatch does not pump.
@row_add n=78 row=contract/harness-settle/busy-loads area=3-test-harness-contract rec=fct-e36kow75@1
boundary: Harness settle
item: NotSettled { busy: Loads }
promise: After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
@row_add n=79 row=contract/harness-settle/settle-pass-limit area=3-test-harness-contract rec=fct-e36kow75@1,fct-33n62qwf@1
boundary: Harness settle
item: SETTLE_PASS_LIMIT
promise: SETTLE_PASS_LIMIT (64) passes with none quiet make settle return Render, Loads or Layout, read off the last pass.
@row_add n=80 row=contract/harness-settle/side-effects area=3-test-harness-contract rec=fct-e36kow75@1
boundary: Harness settle
item: side effects
promise: settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
@row_add n=81 row=contract/harness-settle/failed-load area=3-test-harness-contract rec=fct-4p33xo3p@1
boundary: Harness settle
item: failed load
promise: A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.
@row_add n=82 row=contract/harness-core/dispatch-recorded area=3-test-harness-contract rec=fct-gix7v44z@1,fct-e36kow75@1
boundary: Harness core
item: dispatch_recorded
promise: dispatch_recorded([UiEvent, ...]) returns the list of dispatched event names and does not pump.
@row_add n=83 row=contract/harness-input/input-helpers area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: input helpers
promise: Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
@row_add n=84 row=contract/harness-input/scroll-into-view area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: scroll_into_view
promise: scroll_into_view(node_id) scrolls at once and by the least movement on each axis, every scrolling box that holds the node and then the viewport; a row of an open select list moves the list's scroll_y alone.
@row_add n=85 row=contract/harness-input/apple-keybinding area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: apple_keybinding
promise: apple_keybinding(command) dispatches UiEvent::AppleStandardKeybinding(command) to the focused element, the event a macOS window delivers for a standard key binding.
@row_add n=86 row=contract/harness-input/clipped-click area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: click on a clipped element
promise: A raw click on an element lying outside the padding box of a box that clips it does not land on it.
@row_add n=87 row=contract/harness-input/pointer-coordinates area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: pointer coordinates
promise: Synthesized pointer events set page, screen and client coordinates to the same values.
@row_add n=88 row=contract/harness-input/key-event area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: key_event
promise: Synthesized key events use Code::Unidentified and Location::Standard and fill text only for pressed character keys.
@row_add n=89 row=contract/harness-input/pointer-event area=3-test-harness-contract rec=fct-dncmjjxa@1
boundary: Harness input helpers
item: pointer_event
promise: The harness crate exports a pointer_event(id, x, y, button, buttons, mods) builder.
@row_add n=90 row=contract/harness-inspect/layout-rect area=3-test-harness-contract rec=fct-542cugs7@1
boundary: Harness inspection helpers
item: layout_rect
promise: layout_rect(selector) returns a rect with x, y, width and height.
@row_add n=91 row=contract/harness-inspect/center-of area=3-test-harness-contract rec=fct-542cugs7@1
boundary: Harness inspection helpers
item: center_of
promise: center_of returns an (x, y) tuple.
@row_add n=92 row=decision/scrolled-box-reads-where-it-stands area=3-test-harness-contract rec=fct-542cugs7@1
name: A scrolled box reads where it stands
rule: For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do.
why: ratified
guard: scrolled_box_absolute_position
@row_none n=35 rec=fct-w2z3desc@1 type=S
@row_none n=36 rec=fct-vtn5kxcn@1 type=5
@row_none n=37 rec=fct-bvjwb26r@1 type=5
@row_add n=93 row=decision/virtual-time-for-manual-timers area=3-test-harness-contract rec=fct-lfgoedtw@1
name: Virtual time for manual timer driving
rule: blitz-vibey-script's virtual time and without_timer_thread serve embedders that drive timers manually, test runners among them.
@row_add n=94 row=decision/vibey-selection-tests-shape-real-text area=3-test-harness-contract rec=fct-lfgoedtw@1
name: Vibey-script selection tests shape real text
rule: blitz-vibey-script's dev-dependency on blitz-dom enables system-fonts.
why: so text inputs shape real text in the selection tests
@row_add n=95 row=decision/parsed-markup-pinned-with-real-parser area=3-test-harness-contract rec=fct-i47d76zr@2
name: Parsed markup pinned where the real parser runs
rule: Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests.
why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
@row_add n=96 row=decision/bridge-tests-without-widening area=3-test-harness-contract rec=fct-i47d76zr@2
name: Bridge tests without widening visibility
rule: The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them.
why: DioxusEventHandler's two fields are private to that module and it has no constructor
except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
@row_add n=97 row=decision/shared-stand-tables-in-one-module area=3-test-harness-contract rec=fct-og7dxjok@2
name: Shared stand tables in one module
rule: The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping.
except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
@row_add n=98 row=decision/session-checks-share-one-module area=3-test-harness-contract rec=fct-prxcxiqp@2
name: Session checks share one module
rule: What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module.
why: kept beside the stand checks' module because that module was held unchanged
@row_add n=99 row=decision/shared-fixtures-in-one-module area=3-test-harness-contract rec=fct-osorhgow@2
name: Shared fixtures in one module
rule: The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net.
except: a page with one reader, or whose ids differ, stays in its own check file
@row_add n=100 row=contract/stand-log/stream area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: stream
promise: The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.
@row_add n=101 row=contract/stand-log/event-line area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: event line
promise: One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
@row_add n=102 row=contract/stand-log/span-line area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: span line
promise: One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
@row_add n=103 row=decision/closed-spans-logged-as-lines area=3-test-harness-contract rec=fct-xxpbx2ay@2
name: Closed spans logged as lines
rule: The stand log carries closed spans its allowlist admits as a second record class beside printed events.
why: a boundary widening, ratified, as the obs-plan's log format states
@row_add n=104 row=contract/stand-log/service-name area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: service.name
promise: Both seven_guis binaries log under service.name=seven_guis.
@row_add n=105 row=decision/no-ids-or-names-in-stand-logs area=3-test-harness-contract rec=fct-xxpbx2ay@2
name: No ids or names in stand logs
rule: The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed.
guard: host_log, mcp_log, stand_act_spans
@row_add n=106 row=contract/stand-log/command-span area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: command span
promise: escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
@row_add n=107 row=contract/stand-log/tool-call-span area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: Stand log
item: tool_call span
promise: The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
@row_add n=108 row=contract/tool-call-span/ending area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: tool_call span
item: ending
promise: ending is one of accepted, refused or error.
@row_add n=109 row=contract/tool-call-span/help area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: tool_call span
item: help
promise: A help call, answered by the server, writes one tool_call line, tool="help" ending="accepted", with no command line beneath it.
@row_add n=110 row=contract/command-span/verb area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: command span
item: verb
promise: verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
@row_add n=111 row=contract/command-span/screenshot area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: command span
item: screenshot
promise: A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
@row_add n=112 row=contract/command-span/graph area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: command span
item: graph
promise: A graph call's command line records verb alone under its tool_call parent.
@row_add n=113 row=contract/mcp-role/stdout area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: escher-session mcp role
item: stdout
promise: The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
@row_add n=114 row=contract/host-role/stdout area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: escher-session host role
item: stdout
promise: The host role writes nothing to stdout.
@row_add n=115 row=contract/cli-command/stdout area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: escher-session commands
item: stdout
promise: A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
@row_add n=116 row=contract/cli-command/stderr area=3-test-harness-contract rec=fct-xxpbx2ay@2
boundary: escher-session commands
item: stderr
promise: An accepted command's stderr is empty with RUST_LOG unset.
@row_none n=38 rec=fct-ufzolpz6@1 type=S
@row_add n=117 row=contract/agent-run/invocation area=3-test-harness-contract rec=fct-yzfsia47@1
boundary: agent-run
item: invocation
promise: bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}, run from the repository root.
@row_add n=118 row=contract/agent-run/windows-entry area=3-test-harness-contract rec=fct-yzfsia47@1
boundary: agent-run
item: agent-run.ps1
promise: The Windows entry passes every argument unchanged to the bash script and exits with its code, holding no contract logic.
@row_add n=119 row=decision/agent-run-holds-no-process area=3-test-harness-contract rec=fct-yzfsia47@1
name: Agent-run holds no process
rule: The agent-run script starts, polls and listens on nothing (no daemon, PID file, status endpoint, port or socket) and reads no environment variable of its own; the stand checks it runs boot in process.
except: stand_session_lifecycle and stand_session_quiet start and stop a session host process and its Unix-domain socket, a surface of the session library, not of this contract
@row_add n=120 row=contract/agent-run-exit-codes/0 area=3-test-harness-contract rec=fct-ad7eoe6y@1
boundary: agent-run exit codes
item: 0
promise: Success.
@row_add n=121 row=contract/agent-run-verbs/boot area=3-test-harness-contract rec=fct-zw3j3igx@1
boundary: agent-run verbs
item: boot
promise: boot recreates target/agent-run/ with a fresh events.jsonl, builds every blitz-tests test binary (--locked --no-run, output to run.log), writes status.json (booted, boot_ts, head) and emits the boot event; exit 0 ready, 1 failed.
@row_add n=122 row=contract/agent-run-verbs/run area=3-test-harness-contract rec=fct-24pgoqqt@1
boundary: agent-run verbs
item: run
promise: run {selection} runs cargo test -p blitz-tests --locked with merged output to run.log, truncated at the run's start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
@row_add n=123 row=contract/agent-run-selection/stand area=3-test-harness-contract rec=fct-24pgoqqt@1
boundary: agent-run run selection
item: stand
promise: stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
@row_add n=124 row=contract/agent-run-selection/all area=3-test-harness-contract rec=fct-24pgoqqt@1
boundary: agent-run run selection
item: all
promise: all runs the whole package, with no --test.
@row_add n=125 row=contract/agent-run-selection/name area=3-test-harness-contract rec=fct-24pgoqqt@1
boundary: agent-run run selection
item: {name}
promise: {name} runs one file, matching ^[a-z0-9_]+$ with tests/blitz-tests/tests/{name}.rs present; any other selection exits 2.
@row_add n=126 row=decision/empty-run-is-never-a-pass area=3-test-harness-contract rec=fct-24pgoqqt@1
name: An empty run is never a pass
rule: A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
@row_add n=127 row=contract/agent-run-verbs/status area=3-test-harness-contract rec=fct-nftbaaws@1
boundary: agent-run verbs
item: status
promise: status prints one status object from status.json and builds or runs nothing; exit 0 booted, 3 not (no status.json, or booted false).
@row_add n=128 row=contract/agent-run-verbs/cleanup area=3-test-harness-contract rec=fct-wdmuqlpr@1
boundary: agent-run verbs
item: cleanup
promise: cleanup removes target/agent-run/ if present and prints the cleanup event; idempotent, exit 0 on a clean tree, touching nothing else under target/.
@row_add n=129 row=contract/agent-run-verbs/logs area=3-test-harness-contract rec=fct-34rravfl@1
boundary: agent-run verbs
item: logs
promise: logs prints events.jsonl verbatim, one JSON object per line, exit 3 when it is absent; raw cargo output stays in run.log, never printed.
@row_add n=130 row=decision/agent-run-events-json-by-python area=3-test-harness-contract rec=fct-hfyci2uo@1
name: Agent-run events encoded by python3 json
rule: Agent-run's JSON-lines events are encoded by python3's json module embedded in the script.
@row_add n=131 row=contract/agent-run-events/boot area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: boot
promise: boot {ts, outcome: ready|failed, cargo_exit, head}, printed to stdout and appended to events.jsonl.
@row_add n=132 row=contract/agent-run-events/run-start area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: run.start
promise: run.start {ts, selection, files}, files the --test stems passed ([] under all), printed to stdout and appended to events.jsonl.
@row_add n=133 row=contract/agent-run-events/test area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: test
promise: test {file, test, outcome: ok|failed|ignored}, one per libtest line, file the target binary's stem with its hash stripped or doc:{crate}; printed to stdout and appended to events.jsonl.
@row_add n=134 row=contract/agent-run-events/run-end area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: run.end
promise: run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome: passed|failed}, printed to stdout and appended to events.jsonl.
@row_add n=135 row=contract/agent-run-events/status area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: status
promise: status {booted, boot_ts, head, run: {selection, state: none|running|passed|failed, passed, failed, ignored}}, printed only.
@row_add n=136 row=contract/agent-run-events/cleanup area=3-test-harness-contract rec=fct-hfyci2uo@1
boundary: agent-run events
item: cleanup
promise: cleanup {outcome: done}, printed only.
@row_add n=137 row=decision/test-output-never-reaches-events area=3-test-harness-contract rec=fct-hfyci2uo@1
name: Test output never reaches agent-run events
rule: Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
@row_add n=138 row=decision/agent-run-events-are-harness-metadata area=3-test-harness-contract rec=fct-hfyci2uo@1
name: Agent-run events are harness metadata
rule: Agent-run's events are harness metadata, not an escher-telemetry sink.
@row_add n=139 row=contract/agent-run-state/files area=3-test-harness-contract rec=fct-rkjkceps@1
boundary: agent-run state
item: files
promise: Agent-run's state is target/agent-run/{status.json, events.jsonl, run.log} only, gitignored under target/.
@row_add n=140 row=decision/one-agent-run-driver-at-a-time area=3-test-harness-contract rec=fct-rkjkceps@1
name: One agent-run driver at a time
rule: One agent-run driver runs at a time: a killed run leaves running behind, which the next run overwrites.
@row_none n=39 rec=fct-lk6drchu@1 type=5
@row_none n=40 rec=fct-glytb3w3@4 type=5
@row_none n=41 rec=fct-srvuod5g@1 type=S
@row_add n=141 row=contract/cold-agent-tasks/stand area=3-test-harness-contract rec=fct-ea6x2kry@1
boundary: cold-agent pipe tasks
item: stand
promise: `stand` is the one task: six apps (counter, flight booker, timer, CRUD, cells and the load page) in one client session; any other task, `run counter` included, is a usage error.
@row_add n=142 row=contract/cold-agent-invocation/environment area=3-test-harness-contract rec=fct-ea6x2kry@1
boundary: cold-agent pipe invocation
item: environment
promise: The cold-agent pipe binds no port or socket and reads no environment variable of its own; `RUST_LOG=info` is set on its server child only.
@row_add n=143 row=contract/cold-agent-invocation/windows-entry area=3-test-harness-contract rec=fct-ea6x2kry@1
boundary: cold-agent pipe invocation
item: windows entry
promise: The Windows entry of the cold-agent pipe is a pass-through with no logic of its own.
@row_add n=144 row=contract/cold-agent-exit-codes/0 area=3-test-harness-contract rec=fct-rcsbr6d6@1
boundary: cold-agent exit codes
item: 0
promise: Success, on every verb.
@row_add n=145 row=contract/cold-agent-exit-codes/1 area=3-test-harness-contract rec=fct-rcsbr6d6@1
boundary: cold-agent exit codes
item: 1
promise: The verb ran and failed (a `failed` verdict).
@row_add n=146 row=decision/cold-agent-session-isolated area=3-test-harness-contract rec=fct-w3eifbrv@1
name: Cold-agent session isolated to the escher MCP server
rule: The cold agent runs from a per-run session directory outside the repository holding only an MCP config with the one `escher` stdio server, with no other tools, no setting sources, no slash commands and no session persistence, and nothing copied into it.
@row_add n=147 row=decision/cold-agent-run-bounded area=3-test-harness-contract rec=fct-w3eifbrv@1
name: Cold-agent client run bounded
rule: The client runs under a 600 s timeout and a 3 USD budget; a timeout or non-zero client exit is a `client-exit` failure, never a propagated 124.
@row_add n=148 row=contract/cold-agent-check/extraction area=3-test-harness-contract rec=fct-w3eifbrv@1
boundary: the cold agent's check
item: extraction
promise: The check is the reply's last fenced json block, at most 65,536 bytes, a JSON array, written to `check.json` and judged.
@row_add n=149 row=decision/cold-agent-prompt-names-no-ids area=3-test-harness-contract rec=fct-w3eifbrv@1
name: Cold-agent prompt gives no element ids or app slugs
rule: The fixed stand prompt asks for the six apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
@row_add n=150 row=decision/cold-agent-replayed-check area=3-test-harness-contract rec=fct-63656bwh@3
name: Cold agent's check judged by replay
rule: The agent's JSON check is replayed in full on a fresh driver MCP child and must pass, then replayed ablated (its acting steps left out) on a second fresh child and must fail; a malformed check is never replayed.
why: ratified as the chosen option, a JSON check with replay
@row_add n=151 row=contract/cold-agent-check/step area=3-test-harness-contract rec=fct-63656bwh@3
boundary: the cold agent's check
item: step
promise: A step holds `tool` among the fourteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
@row_add n=152 row=decision/cold-agent-replay-bounded area=3-test-harness-contract rec=fct-63656bwh@3
name: Cold-agent replay child bounded
rule: Each replay child is spawned directly, without recording, with stderr discarded, every reply bounded at 30 s, then killed and reaped; a replay child that fails reads as the check not passing, or not able to fail.
@row_add n=153 row=decision/cold-agent-server-log-counted-only area=3-test-harness-contract rec=fct-63656bwh@3,fct-bkp6z6tz@1
name: Cold-agent server log counted, never shown
rule: The server's log is read only to count its tool-call span lines, their refused and error endings and each counted verb's accepted lines; it is never printed or copied.
@row_add n=154 row=decision/cold-agent-passes-on-positive-evidence area=3-test-harness-contract rec=fct-szqqqnrt@3
name: Cold-agent run passes only on positive evidence
rule: A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all six apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
@row_add n=155 row=contract/cold-agent-verdict/reasons area=3-test-harness-contract rec=fct-szqqqnrt@3
boundary: cold-agent verdict
item: reasons
promise: Reasons come from `client-exit · no-result · not-isolated · counts-disagree · no-tool-call · task-unmet · capability-unused · recording-not-kept · wrong-calls-over-bar · no-check · check-failed · check-cannot-fail`; `no-check` and an invalid check's `check-failed` stand alone.
@row_add n=156 row=contract/cold-agent-verdict/tasks-met area=3-test-harness-contract rec=fct-szqqqnrt@3
boundary: cold-agent verdict
item: tasks_met
promise: An app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`, a paged screen read as its parts joined, never from the reply.
@row_add n=157 row=contract/cold-agent-verdict/isolated area=3-test-harness-contract rec=fct-szqqqnrt@3
boundary: cold-agent verdict
item: isolated
promise: True when the init tools are exactly the fourteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
@row_add n=158 row=contract/cold-agent-verdict/counts-agree area=3-test-harness-contract rec=fct-szqqqnrt@3
boundary: cold-agent verdict
item: counts_agree
promise: True when the server's tool-call span lines equal the escher tool uses and its refused plus error lines equal the escher tool results marked `is_error`.
@row_add n=159 row=contract/cold-agent-verdict/recording-kept area=3-test-harness-contract rec=fct-szqqqnrt@3
boundary: cold-agent verdict
item: recording_kept
promise: True when each accepted `start` left one kept check opening with its app's start record.
@row_add n=160 row=contract/cold-agent-events/run-start area=3-test-harness-contract rec=fct-wtmhm4m7@1
boundary: cold-agent events
item: run.start
promise: A JSON line `{ts, task}`, printed and appended to `events.jsonl`.
@row_add n=161 row=contract/cold-agent-events/run-end area=3-test-harness-contract rec=fct-wtmhm4m7@1
boundary: cold-agent events
item: run.end
promise: A JSON line holding every verdict field, printed and appended to `events.jsonl`.
@row_add n=162 row=contract/cold-agent-events/status area=3-test-harness-contract rec=fct-wtmhm4m7@1
boundary: cold-agent events
item: status
promise: Prints the verdict (exit 0) or `{outcome: "none"}` (exit 3); printed only, never appended.
@row_add n=163 row=contract/cold-agent-events/cleanup area=3-test-harness-contract rec=fct-wtmhm4m7@1
boundary: cold-agent events
item: cleanup
promise: Prints `{outcome: "done"}`; printed only, never appended.
@row_add n=164 row=decision/cold-agent-events-content-free area=3-test-harness-contract rec=fct-wtmhm4m7@1
name: Cold-agent events and verdict carry no content
rule: No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one.
guard: the scrub-set key check and four planted markers under the shim
@row_add n=165 row=decision/cold-agent-events-not-telemetry area=3-test-harness-contract rec=fct-wtmhm4m7@1
name: Cold-agent events are harness metadata
rule: The cold-agent events are harness metadata, not an escher-telemetry sink.
@row_add n=166 row=contract/cold-agent-state/directory area=3-test-harness-contract rec=fct-bkp6z6tz@1
boundary: cold-agent state
item: target/cold-agent
promise: The pipe's state is only `verdict.json`, `events.jsonl`, `transcript.jsonl`, `client.log`, `server.log`, `check.json` and `record/` under `target/cold-agent/`, gitignored.
@row_add n=167 row=decision/cold-agent-record-private area=3-test-harness-contract rec=fct-bkp6z6tz@1
name: Cold-agent recordings kept private
rule: The live server's kept checks sit in an owner-only record directory and are never printed, copied or committed; the parsed check file is never printed.
@row_add n=168 row=contract/cold-agent-verbs/cleanup area=3-test-harness-contract rec=fct-bkp6z6tz@1
boundary: cold-agent verbs
item: cleanup
promise: Removes the cold-agent state directory alone, idempotent, leaving `target/agent-run/` untouched.
@row_add n=169 row=decision/cold-agent-live-run-off-ci area=3-test-harness-contract rec=fct-u5f4aal7@1
name: Live cold-agent run never in CI
rule: The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests.
why: no CI secret
@row_add n=170 row=decision/test-data-is-code area=3-test-harness-contract rec=fct-6bs5asmb@3
name: Test data is code, nothing seeded
rule: Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
@row_add n=171 row=contract/agent-run-selections/stand area=3-test-harness-contract rec=fct-6bs5asmb@3
boundary: agent-run selections
item: run stand
promise: Runs the `stand_*` readers of the shared module.
@row_add n=172 row=contract/agent-run-selections/file area=3-test-harness-contract rec=fct-6bs5asmb@3
boundary: agent-run selections
item: run {file}
promise: Runs one named non-stand reader.
@row_add n=173 row=contract/agent-run-selections/all area=3-test-harness-contract rec=fct-6bs5asmb@3
boundary: agent-run selections
item: run all
promise: Runs the readers neither `run stand` nor a non-stand file selection covers; there is no `run fixtures` selection.
@row_none n=42 rec=fct-2rauolld@2 type=S
@row_none n=43 rec=fct-6digpenh@1 type=S
@row_add n=174 row=decision/unit-tests-in-source-file area=4-unit-test-strategy rec=fct-tkvntwtm@1
name: Unit tests sit in their source file
rule: Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged.
except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor
@row_none n=44 rec=fct-tz5urk7n@1 type=5
@row_none n=45 rec=fct-avgxyxaq@1 type=S
@row_none n=46 rec=fct-555owb3s@1 type=5
@row_add n=175 row=decision/call-assertions-carry-row-index area=4-unit-test-strategy rec=fct-mnyw2bw5@1
name: Call assertions carry a row index only
rule: The driver's assertions over a call carry a row index only, never an argument value.
why: so a failing log prints no argument value
@row_none n=47 rec=fct-3skvtylr@1 type=5
@row_add n=176 row=decision/ci-actions-pinned-to-sha area=4-unit-test-strategy rec=fct-lglsrvq2@1
name: CI actions pinned to commit SHAs
rule: Every action the CI workflow uses is pinned to a commit SHA.
guard: test_ci_workflows.py
@row_add n=177 row=decision/ci-no-secrets area=4-unit-test-strategy rec=fct-lglsrvq2@1
name: CI uses no secrets
rule: The CI workflow references no `secrets.`.
guard: test_ci_workflows.py
@row_add n=178 row=decision/blitz-tests-single-test-target area=4-unit-test-strategy rec=fct-c54rn2me@1
name: blitz-tests manifest holds one test target
rule: The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`.
why: the fork's standing two-line difference from upstream's manifest
guard: BlitzTestsTargetsTest
@row_add n=179 row=decision/agent-run-events-content-free area=4-unit-test-strategy rec=fct-iokpku7j@1
name: Agent-run output carries no panic text
rule: A failing agent run puts no captured panic text in stdout or the events file, and no event carries a scrub-set key; the raw cargo output goes only to the run log.
guard: test_agent_run.py
@row_none n=48 rec=fct-xdmoyulv@1 type=5
@row_none n=49 rec=fct-xhvydcuw@1 type=5
@row_none n=50 rec=fct-7rf6ma35@1 type=5
@row_none n=51 rec=fct-faqjogl7@1 type=5
@row_none n=52 rec=fct-v5grnlyx@1 type=5
@row_none n=53 rec=fct-ehsmv66a@1 type=5
@row_none n=54 rec=fct-iz7v4qag@1 type=5
@row_none n=55 rec=fct-g4jhk55d@1 type=5
@row_none n=56 rec=fct-6jvgfsgo@1 type=5
@row_none n=57 rec=fct-2gomcmqm@1 type=5
@row_none n=58 rec=fct-bzpislbe@1 type=S
@row_none n=59 rec=fct-6xpfsiyh@1 type=7
@row_none n=60 rec=fct-cgwacdcy@1 type=S
@row_none n=61 rec=fct-d754ogn3@1 type=S
@row_none n=62 rec=fct-q24sm5dl@1 type=4
@row_none n=63 rec=fct-q2tytpcr@1 type=5
@row_none n=64 rec=fct-wsfoykxe@1 type=5
@row_none n=65 rec=fct-uwcpu73r@1 type=5
@row_none n=66 rec=fct-bvs6y67q@1 type=5
@row_none n=67 rec=fct-d2wqhd7k@1 type=5
@row_none n=68 rec=fct-vu43sciq@1 type=4
relied: Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding.
@row_none n=69 rec=fct-fcqjphla@1 type=5
@row_none n=70 rec=fct-5mwtk4y2@1 type=5
@row_none n=71 rec=fct-zmnlwg25@1 type=5
@row_add n=180 row=contract/driver-session/held-state area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver session
item: held state
promise: One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
@row_add n=181 row=contract/driver-session/run area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver session
item: run
promise: A call run through the session returns after settle with the diff of the snapshots taken before and after it.
@row_add n=182 row=contract/driver-session/refused-call area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver session
item: refused call
promise: A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
@row_add n=183 row=contract/driver-session/password area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver session
item: typed password
promise: A password typed through the driver never leaves in an answer or as pixels.
@row_add n=184 row=contract/driver-verbs/graph area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver verbs
item: graph
promise: `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
@row_add n=185 row=contract/driver-verbs/actions area=5-integration-test-strategy rec=fct-gq2zcc44@3
boundary: driver verbs
item: actions
promise: `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
@row_add n=186 row=decision/no-wall-time-in-session-checks area=5-integration-test-strategy rec=fct-ubyqdssx@1
name: No wall time in session checks
rule: Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only.
except: the two checks whose subject is wall time: the virtual-clock check's one 800 ms sleep and the load-wait check's clock reads around its call with its late stand-in's sleep
@row_add n=187 row=contract/driver-log/per-call-line area=5-integration-test-strategy rec=fct-lm4npbmj@1
boundary: the driver's log line
item: per-call line
promise: With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
@row_add n=188 row=contract/driver-log/default-level area=5-integration-test-strategy rec=fct-lm4npbmj@1
boundary: the driver's log line
item: default level
promise: At the default level the driver writes no line.
@row_add n=189 row=contract/session-host/non-unix area=5-integration-test-strategy rec=fct-oknzmjfs@1
boundary: session host
item: non-unix arms
promise: Off unix the session host's arms return `Unsupported`.
@row_add n=190 row=decision/session-writes-no-screen-content area=5-integration-test-strategy rec=fct-kudmubnq@1
name: Session writes nothing of the screen
rule: The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
@row_add n=191 row=decision/host-stderr-no-screen-content area=5-integration-test-strategy rec=fct-kudmubnq@1
name: Host stderr carries no screen content at trace
rule: A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included.
guard: host_log
@row_none n=72 rec=fct-z73hajxl@1 type=5
@row_none n=73 rec=fct-vtpjw3vt@1 type=S
@row_none n=74 rec=fct-guo2i4qp@1 type=7
@row_none n=75 rec=fct-vsgkw6vc@1 type=S
@row_none n=76 rec=fct-jah6cdhx@1 type=S
@row_add n=192 row=decision/wpt-reftests-thai-font area=6-e2e-test-strategy rec=fct-dp2mr6q3@1
name: WPT reftests run with a Thai font
rule: WPT reftests are run with a Thai font installed.
why: some reftests depend on Thai glyph widths
@row_add n=193 row=decision/wpt-upstream-only area=6-e2e-test-strategy rec=fct-dp2mr6q3@1,fct-zrtlqcd7@1
name: WPT workflow on upstream only
rule: The WPT workflow, its runs and its posting of results, runs only on the upstream DioxusLabs/blitz repository, its jobs repository-guarded.
@row_none n=77 rec=fct-tfjcoowp@1 type=5
@row_none n=78 rec=fct-jx52yw2t@1 type=5
@row_add n=194 row=decision/unscrolled-containers-no-scrollbar-thumbs area=6-e2e-test-strategy rec=fct-aijyldbk@1
name: Never-scrolled containers paint no scrollbar thumbs
rule: A container that has never been scrolled paints no scrollbar thumbs.
why: keeps scrollbar thumbs out of static reftest screenshots
@row_add n=195 row=decision/checks-byte-equal-to-recording area=6-e2e-test-strategy rec=fct-3c5ihsc6@1,fct-wzfum7l4@1
name: Committed checks byte-equal to the binary's recording
rule: The committed checks are recorded by escher-session and read byte-equal to what the binary records.
@row_add n=196 row=decision/checks-hold-no-screenshot-step area=6-e2e-test-strategy rec=fct-3c5ihsc6@1,fct-wzfum7l4@1
name: No screenshot step in committed checks
rule: The scripts behind the committed checks hold no screenshot step.
why: a shot's PNG bytes differ by OS
@row_add n=197 row=decision/mcp-tested-as-child-and-in-process area=6-e2e-test-strategy rec=fct-pnkaq5az@1
name: MCP role tested as a child and in process
rule: The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
@row_add n=198 row=decision/real-mcp-client-outside-ci area=6-e2e-test-strategy rec=fct-pnkaq5az@1
name: Real MCP client only in the operator pass
rule: A real MCP client drives escher-session mcp in the operator pass, not in CI.
@row_none n=79 rec=fct-pvaztozi@1 type=S
@row_add n=199 row=decision/example-fixtures-are-manual area=6-e2e-test-strategy rec=fct-knszydyg@1,fct-vjj7iuxq@1,fct-3sx2lzbi@1
name: Example fixtures carry no automated assertion
rule: Example fixture pages state their click-through instructions and expected outcomes in text instead of automated assertions.
@row_none n=80 rec=fct-esav4sth@1 type=5
@row_none n=81 rec=fct-xogchgeh@1 type=5
@row_none n=82 rec=fct-y57z5hyx@1 type=S
@row_add n=200 row=decision/ui-driven-through-session-binary area=6-e2e-test-strategy rec=fct-3sx2lzbi@1
name: App UI driven through the session binary
rule: No test in the apps launches a window; the UI is driven through the escher-session binary.
@row_none n=83 rec=fct-qj2xx2qt@1 type=S
@row_none n=84 rec=fct-pb3uwv7o@1 type=S
@row_add n=201 row=contract/wpt-runner-env/wpt-dir area=7-test-data-fixtures rec=fct-gazseuex@1
boundary: WPT runner environment
item: WPT_DIR
promise: The runner reads its test inputs from the WPT checkout at WPT_DIR.
@row_none n=85 rec=fct-l2nx4sbf@1 type=5
@row_none n=86 rec=fct-fcmw5r6j@1 type=5
@row_none n=87 rec=fct-ixlzqejs@1 type=5
@row_add n=202 row=decision/checks-hold-no-private-data area=7-test-data-fixtures rec=fct-wzfum7l4@1
name: Committed checks hold no private data
rule: The committed checks hold no typed text, host path or secret.
@row_add n=203 row=decision/shared-fixture-pages area=7-test-data-fixtures rec=fct-adt6dvk7@1
name: Shared fixture pages stated once
rule: A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options.
except: a page with one reader stays in its check's file
@row_none n=88 rec=fct-c7r7pshj@1 type=S
@row_add n=204 row=decision/test-data-in-code area=7-test-data-fixtures rec=fct-3xrvezdp@1
name: Test data stated in code
rule: Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test.
except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
@row_none n=89 rec=fct-irm5pldn@1 type=S
@row_none n=90 rec=fct-676efqab@1 type=5
@row_none n=91 rec=fct-z5j75wsf@1 type=5
@row_none n=92 rec=fct-2m6ixabz@1 type=5
@row_none n=93 rec=fct-czpriias@1 type=5
@row_none n=94 rec=fct-px3qbelc@1 type=4
relied: Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads.
@row_none n=95 rec=fct-4qfz5352@1 type=4
relied: Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it.
@row_none n=96 rec=fct-aqym3m2p@1 type=S
@row_none n=97 rec=fct-obnl235t@1 type=7
@row_none n=98 rec=fct-womgr2md@1 type=S
@row_none n=99 rec=fct-sbawysc6@1 type=S
@row_add n=205 row=decision/hand-written-stand-ins area=8-mocking-stubbing-discipline rec=fct-xvhacwho@1,fct-nrlinzvj@1
name: Hand-written stand-ins, no mocking library
rule: Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
@row_add n=206 row=decision/network-stand-ins-counted area=8-mocking-stubbing-discipline rec=fct-hsnu3a5x@1
name: Network stand-ins keep their loads counted
rule: Network stand-ins do not override is_noop, so the harness counts their loads.
@row_add n=207 row=decision/stand-timer-ticks area=8-mocking-stubbing-discipline rec=fct-ywuwg5pe@1
name: Stand timer driven by TimerTicks
rule: Every headless-stand boot provides TimerTicks in place of the timer's real delay.
@row_add n=208 row=decision/settle-never-advances-time area=8-mocking-stubbing-discipline rec=fct-ywuwg5pe@1
name: Settle never advances time
rule: Settle neither advances the timer nor waits on a tick not yet delivered.
@row_add n=209 row=contract/driver-advance/advance area=8-mocking-stubbing-discipline rec=fct-ywuwg5pe@1
boundary: the driver's advance command
item: advance
promise: Moves the session's clock by exactly ms inside one settled step.
@row_add n=210 row=contract/driver-advance/advanced-ms area=8-mocking-stubbing-discipline rec=fct-ywuwg5pe@1
boundary: the driver's advance command
item: advanced_ms
promise: Is exactly ms, the clock's move, not what the Timer's counter gained.
@row_none n=100 rec=fct-iqvttxzo@1 type=S
@row_add n=211 row=decision/stand-pinned-real-font area=8-mocking-stubbing-discipline rec=fct-ywhfajcc@1
name: Stand checks keep a real pinned font
rule: The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
@row_none n=101 rec=fct-trq2qkpg@1 type=S
@row_none n=102 rec=fct-pzcfxvli@1 type=S
@row_add n=212 row=decision/ci-on-github-actions area=9-ci-integration rec=fct-ck6s7qej@1
name: CI on GitHub Actions
rule: GitHub Actions is what runs the project's CI.
@row_add n=213 row=decision/ci-trigger-branches area=9-ci-integration rec=fct-ck6s7qej@1
name: CI trigger branches
rule: Tests run on pull requests and on pushes to main, v0.* and build/** branches, escher's build branch included.
@row_none n=103 rec=fct-5qwppbpa@1 type=S
@row_add n=214 row=decision/stand-repeat-three-rounds area=9-ci-integration rec=fct-nymiowfp@1
name: Stand checks repeated three rounds
rule: The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
@row_add n=215 row=decision/ci-fast-slow-split area=9-ci-integration rec=fct-o3bnksvm@1
name: Fast/slow CI split
rule: fmt, clippy, test-features-default and ci-scripts run first with no dependencies, and every other job, the platform matrix included, needs all four.
@row_add n=216 row=decision/ci-rust-cache area=9-ci-integration rec=fct-pzb5kcr7@1
name: SHA-pinned Rust cache
rule: Every compiling CI job uses the SHA-pinned Swatinem/rust-cache.
except: coverage
@row_add n=217 row=decision/ci-cache-save-branches area=9-ci-integration rec=fct-pzb5kcr7@1
name: Cache saved on main and build branches
rule: CI caches are saved only on main and build/* branches.
@row_add n=218 row=decision/shared-test-cache area=9-ci-integration rec=fct-pzb5kcr7@1
name: Shared workspace test cache
rule: The test job saves the cache under shared-key workspace-test, which the a11y and stand-repeat jobs restore without saving.
@row_add n=219 row=contract/ci-artifacts/leg-log area=9-ci-integration rec=fct-k3iekajl@1
boundary: CI logs and artifacts
item: target/ci-logs/{leg}.log
promise: Holds each leg's merged output, truncated at the leg's start.
@row_add n=220 row=contract/ci-artifacts/ci-log-job-id area=9-ci-integration rec=fct-k3iekajl@1
boundary: CI logs and artifacts
item: ci-log-{job id}
promise: The leg's log, uploaded only on failure and kept 7 days.
@row_add n=221 row=contract/ci-artifacts/coverage-report area=9-ci-integration rec=fct-k3iekajl@1
boundary: CI logs and artifacts
item: coverage-report
promise: Uploaded by the coverage job alone from target/coverage/ on success, kept 7 days.
@row_add n=222 row=decision/package-installs-bounded-retried area=9-ci-integration rec=fct-fi6v7awk@1
name: Bounded, retried package installs
rule: Every package install of the CI workflow is bounded and retried: up to 3 attempts under a 120 s bound, each retry opening with a bounded dpkg repair.
why: the bound answers a package-install hang
@row_add n=223 row=contract/apt-install-exit-status/range area=9-ci-integration rec=fct-fi6v7awk@1
boundary: apt-install script exit status
item: 0, 1 or 2
promise: The script exits 0, 1 or 2 and never with the bound's 124 or 137.
@row_add n=224 row=decision/local-pre-push-gate area=9-ci-integration rec=fct-ovlcl3hy@1
name: Local pre-push gate
rule: The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
@row_add n=225 row=decision/embolden-leg-local-only area=9-ci-integration rec=fct-ovlcl3hy@1
name: Embolden witness leg without a CI job
rule: The font-embolden witness leg runs locally and has no CI job of its own.
why: macOS exercises the feature in CI
@row_add n=226 row=decision/benchmarks-ignored-by-default area=9-ci-integration rec=fct-czxobxvz@1
name: Benchmarks ignored by default
rule: Benchmark tests are marked ignored and run only on demand, in release mode with the ignored tests selected.
@row_add n=227 row=decision/font-assertions-on-system-fonts area=9-ci-integration rec=fct-42zxrqel@1
name: Font-dependent assertions on system fonts
rule: Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested.
except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
@row_add n=228 row=decision/sink-capture-counted-by-target area=9-ci-integration rec=fct-he2hzpm5@1,fct-prdavblg@3
name: Sink capture counted by target
rule: A check that reads a sink's capture counts lines by target, never in total.
why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.
@row_add n=229 row=decision/per-package-green-not-workspace-green area=9-ci-integration rec=fct-he2hzpm5@1,fct-prdavblg@3
name: Per-package green is not the workspace green
rule: A green per-package test run is not the workspace test leg's green.
why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
@row_add n=230 row=decision/bounds-reader-bodies-each-checked area=9-ci-integration rec=fct-he2hzpm5@1,fct-prdavblg@3
name: Each bounds-reader body holds a check
rule: Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own.
why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner.
guard: scrolled_box_client_rect
@row_none n=104 rec=fct-eu6eymmp@1 type=5
@row_add n=231 row=decision/worker-wait-bounded-in-tests area=9-ci-integration rec=fct-ss52zjj6@1
name: Bounded worker wait in tests
rule: A test that waits on the URL-suggestions worker bounds the wait with a timeout.
why: So a regression would not hang CI.
@row_add n=232 row=decision/wpt-harness-timeout-multiplier area=9-ci-integration rec=fct-tffheddu@1
name: WPT harness timeout multiplier
rule: The WPT harness applies a timeout multiplier to its test timeouts.
why: Heavy interpolation suites run longer on loaded CI machines.
@row_add n=233 row=decision/local-baseline area=9-ci-integration rec=fct-d74qnmiv@1
name: Local baseline for CI comparison
rule: The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run.
why: It is the reference the fork's CI is compared against.
@row_add n=234 row=decision/coverage-leg-exits-with-floor area=9-ci-integration rec=fct-k6zjxg3m@1
name: Coverage leg gated by the floors
rule: The coverage leg measures workspace line coverage and ends with the per-crate floor check, exiting with its status, so a missed floor fails the leg.
guard: coverage_floor.py
@row_none n=105 rec=fct-h3eogzs2@1 type=S
@row_add n=235 row=decision/keyboard-compat-excluded-from-coverage area=10-quality-gates-coverage-targets rec=fct-4t3fuckc@1,fct-k6zjxg3m@1
name: keyboard_compat.rs excluded from coverage
rule: Coverage measurement excludes dioxus-native-dom's keyboard_compat.rs.
why: It is upstream's temporary keyboard-types bridge, excluded until escher takes upstream's retirement of it.
@row_add n=236 row=decision/stand-checks-zero-failure-budget area=10-quality-gates-coverage-targets rec=fct-temy7g7i@1,fct-mrjxzany@4
name: Zero-failure budget for stand checks
rule: The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red.
guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds
@row_add n=237 row=decision/mutation-testing-dev-host-only area=10-quality-gates-coverage-targets rec=fct-lebzei5e@1
name: Mutation testing on the dev host
rule: Mutation testing is cargo-mutants run with nextest under a fail-fast mutants profile, on the dev host alone and never in CI.
@row_add n=238 row=decision/mutation-baseline-read-per-unit area=10-quality-gates-coverage-targets rec=fct-lebzei5e@1
name: Mutation baseline read per unit
rule: A mutation run skips cargo-mutants' own baseline and first reads each unit's test command green on the unmutated tree.
why: cargo-mutants' baseline tests the mutated package alone.
@row_none n=106 rec=fct-iuu6dmkz@1 type=S
@row_none n=107 rec=fct-jquuqm2x@1 type=S
@row_add n=239 row=decision/a11y-leg-membership area=ci-integration rec=fct-mrjxzany@4
name: a11y leg membership
rule: The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap.
why: Chosen over running the tier-0 witnesses alone.
@row_add n=240 row=decision/joining-file-keeps-its-legs area=ci-integration rec=fct-mrjxzany@4
name: A file joining a leg keeps its legs
rule: A test file joining a CI leg keeps every leg it already ran in.
@row_add n=241 row=decision/msrv-1-91 area=ci-integration rec=fct-mrjxzany@4
name: Minimum supported Rust version
rule: CI builds the workspace with Rust 1.91 as the minimum supported version.
@row_add n=242 row=decision/dependency-audit-advisories-and-licenses area=ci-integration rec=fct-mrjxzany@4
name: Dependency audit
rule: The audit leg checks advisories and licenses with cargo-deny over the whole locked workspace with all features.
@row_add n=243 row=decision/clippy-legs area=ci-integration rec=fct-mrjxzany@4
name: Clippy over the workspace and upstream blitz-dom
rule: The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.
@row_none n=108 rec=fct-scvojgvm@1 type=5
@row_none n=109 rec=fct-nrwelml5@1 type=5
@row_none n=110 rec=fct-o7txuegv@2 type=5
@row_add n=244 row=decision/ci-shared-test-cache area=ci-integration rec=fct-oaciwfbs@1
name: Shared CI build cache
rule: Every compiling CI job but coverage uses a SHA-pinned Swatinem/rust-cache saved on main and build branches; the a11y and stand-repeat jobs restore the test job's cache without saving.
@row_none n=111 rec=fct-5yh62djn@1 type=8
@row_none n=112 rec=fct-2vtfmcyi@1 type=8
@row_none n=113 rec=fct-pmze6kz2@1 type=4
relied: A red job's later targets go unread on that run, since cargo stops at the first failing target.
@row_add n=245 row=decision/only-stand-repeat-frees-disk area=ci-integration rec=fct-6usbd5au@1,fct-d5qhprza@1
name: Only the stand-repeat job frees disk
rule: Only the Stand checks repeated job frees runner disk, with a pinned free-disk action after the toolchain and before the cache; the matrix job holds no free-disk step.
guard: test_only_stand_repeat_frees_disk
@row_none n=114 rec=fct-2egr2bnc@1 type=9
@row_add n=246 row=contract/apt-install-exit-codes/0 area=ci-integration rec=fct-uj7cocux@1
boundary: apt-install script exit codes
item: 0
promise: An install attempt completed.
@row_add n=247 row=decision/scrollbar-fade-on-document-clock area=ci-integration rec=fct-prdavblg@3
name: Overlay scrollbar fade on the document clock
rule: The overlay scrollbar fade reads the document's clock, the harness VirtualClock for a harness document, so a screenshot after a scroll repeats until the clock is advanced.
why: Ratified.
guard: stand_virtual_clock
@row_add n=248 row=decision/ci-log-ansi-strip area=ci-integration rec=fct-qjdo4c7k@2
name: ANSI strip for CI job-log tallies
rule: A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[.
why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
@row_add n=249 row=decision/tally-move-attributed-by-test-name area=ci-integration rec=fct-3kqdhqos@1
name: Tally moves attributed by test name
rule: A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line.
why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.
@row_none n=115 rec=fct-kjlei6et@2 type=5
@row_add n=250 row=contract/coverage-floor-exit-codes/0 area=ci-integration rec=fct-na5rpohe@1
boundary: coverage floor script exit codes
item: 0
promise: Every crate named is at or above its floor; the verdict reads `coverage floor: N of N crates at or above their floors`.
@row_add n=251 row=contract/coverage-floor-exit-codes/1 area=ci-integration rec=fct-na5rpohe@1
boundary: coverage floor script exit codes
item: 1
promise: Some crates are under their floors; the verdict reads `coverage floor: K of N crates under their floors`.
@row_add n=252 row=contract/coverage-floor-exit-codes/2 area=ci-integration rec=fct-na5rpohe@1
boundary: coverage floor script exit codes
item: 2
promise: A crate named has no coverage record, or an argument is malformed.
@row_add n=253 row=contract/cli-exit-codes/1 area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI exit codes
item: 1
promise: A call a target cannot take is refused with its cause.
@row_add n=254 row=contract/cli-exit-codes/2 area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI exit codes
item: 2
promise: A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
@row_add n=255 row=contract/cli-help/output area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI help
item: help
promise: `help` answers status 0 and one line equal to `describe()`, reaching no session.
@row_add n=256 row=contract/cli-refusal/remedy area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI refusals
item: remedy
promise: Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
@row_add n=257 row=contract/cli-state-dir/max-length area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI state directory
item: length bound
promise: A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.
@row_add n=258 row=contract/cli-snapshot/budget area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI snapshot
item: size budget
promise: A snapshot answers the in-process text as one line inside its 10,000-byte budget.
@row_add n=259 row=contract/cli-snapshot/part area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI snapshot
item: --part
promise: A part read by number answers one uncoloured JSON line, status 0; a part of 0, 1001 or a non-number exits 1 with the `malformed` refusal line, nothing typed written back.
@row_add n=260 row=contract/cli-schema/refused-call area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI schema
item: refused call
promise: A call the schema refuses reaches no session.
@row_add n=261 row=contract/cli-stdout/closed area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI stdout
item: closed stdout
promise: A run with its stdout closed before it starts keeps its ending's status, prints no panic and writes nothing typed back.
@row_add n=262 row=contract/cli-record/permissions area=e2e-test-strategy rec=fct-csryo2zo@5
boundary: escher-session CLI recording
item: --record
promise: `start <task> --record <dir>` keeps one `0600` check in a `0700` directory; a group-open recording directory is refused, status 3, with nothing spawned.
@row_add n=263 row=decision/no-screenshot-in-recorded-check-scripts area=e2e-test-strategy rec=fct-csryo2zo@5
name: No screenshot step in committed recorded checks
rule: The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems.
why: a shot's PNG bytes differ by OS
guard: the_committed_checks_are_what_the_binary_records
@row_add n=264 row=contract/cli-coverage-role/stdout area=e2e-test-strategy rec=fct-ptmt2gej@1
boundary: escher-session CLI coverage role
item: coverage line
promise: Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
@row_none n=116 rec=fct-2oltbokg@1 type=5
@row_add n=265 row=contract/cli-actions/output area=e2e-test-strategy rec=fct-hh5tpkan@1
boundary: escher-session CLI actions verb
item: output
promise: `actions` against a started host answers one JSON line of the actionable nodes.
@row_add n=266 row=contract/cli-actions/argument area=e2e-test-strategy rec=fct-hh5tpkan@1
boundary: escher-session CLI actions verb
item: argument
promise: An argument to `actions` is refused, status 1.
@row_add n=267 row=contract/mcp-tools-list/order area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: tools/list
promise: `tools/list` offers the fourteen verbs in order, `graph` after `lint` and `actions` after `graph`.
@row_add n=268 row=contract/mcp-tool-result/screenshot area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP tool results
item: screenshot
promise: A `screenshot` tool call answers an `image/png` image block then a text block, `isError` false.
@row_add n=269 row=contract/mcp-tool-result/refused area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP tool results
item: refused call
promise: A refused tool call answers a single text block, `isError` true.
@row_add n=270 row=contract/mcp-protocol/revision area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: protocol revision
promise: The role speaks the newest MCP protocol revision.
@row_add n=271 row=contract/mcp-protocol/errors area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: protocol errors
promise: A protocol error runs nothing.
@row_add n=272 row=contract/mcp-help/parity area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: help
promise: `help` is answered before any `start`, the same bytes through MCP and the command line.
@row_add n=273 row=contract/mcp-snapshot/parity area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: snapshot
promise: A fresh Counter's snapshot reads the same through MCP and the command line.
@row_add n=274 row=contract/mcp-record/parity area=e2e-test-strategy rec=fct-gtzxj2kr@3
boundary: escher-session MCP role
item: --record
promise: With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
@row_add n=275 row=contract/mcp-actions/argument area=e2e-test-strategy rec=fct-wubzr4jw@1
boundary: escher-session MCP role
item: actions
promise: The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
@row_add n=276 row=contract/command-span/actions area=e2e-test-strategy rec=fct-wubzr4jw@1
boundary: driver command span
item: actions
promise: An `actions` tool call parents one `command` span that records `verb` alone.
@row_add n=277 row=decision/buffered-render-unborrowed-document area=e2e-test-strategy rec=fct-kgita7a3@6,fct-mealwlke@2
name: Buffered render with the document unborrowed
rule: The Dioxus bridge renders into a mutation buffer with the document unborrowed and replays it afterwards, so a task polled during render, such as a MountedData call from a key handler's task, reaches the document without a borrow panic.
why: upstream's buffered render ported onto the Dioxus 0.7 bridge, its Dioxus 0.8 move not taken
guard: mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental and _incremental in dioxus_runtime_probe
@row_none n=117 rec=fct-sys2coyr@1 type=4
relied: The buffered render's key-handler pins discriminate on Dioxus 0.7 because a task an event handler spawns while dirtying a scope is first polled inside `render_immediate`.
@row_none n=118 rec=fct-cd63wvqs@1 type=4
relied: The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own.
@row_none n=119 rec=fct-qu7747mx@1 type=7
@row_add n=278 row=absence/windowed-end-to-end-tests area=e2e-test-strategy rec=fct-h2aiavph@1
capability: An automated test that launches a window
@row_add n=279 row=decision/layout-tests-in-both-layout-modes area=integration-test-strategy rec=fct-322qvjpg@1
name: Layout assertions in both layout modes
rule: Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do.
guard: incremental_oracle
@row_add n=280 row=decision/file-urls-only-for-file-documents area=integration-test-strategy rec=fct-2mnilpgn@1
name: file: URLs loadable only from a file: document
rule: A `file:` URL is loaded by `fetch()`, a module `import` or `<script src>` only for a `file:` document, and refused with a `FetchError` otherwise.
guard: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls
@row_add n=281 row=decision/change-event-from-engine-only area=integration-test-strategy rec=fct-5glqoue5@1
name: change events come from the engine alone
rule: A page's script hears one `change` per change, fired by the engine; the script runtime synthesizes none.
guard: radio_click_fires_change_only_where_it_was_not_checked
@row_none n=120 rec=fct-amy2vm75@1 type=5
@row_add n=282 row=contract/session-act/settle area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: act
promise: A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
@row_add n=283 row=decision/load-wait-bound area=integration-test-strategy rec=fct-t7rg232i@1
name: Acting steps wait out loads in flight
rule: An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`.
guard: stand_load_wait
@row_add n=284 row=contract/session-run/diff area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: run
promise: A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
@row_add n=285 row=contract/session-run/screenshot area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: screenshot
promise: A `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster, and changes nothing.
@row_add n=286 row=decision/advance-accepted-on-every-session area=integration-test-strategy rec=fct-t7rg232i@1
name: advance accepted on every session
rule: The `advance` verb is accepted on every session, never refused for the session it addresses.
why: ratified
@row_add n=287 row=decision/refusal-leaves-instance-unchanged area=integration-test-strategy rec=fct-t7rg232i@1
name: Refusals change nothing and echo no input
rule: A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
@row_add n=288 row=contract/refusal-cause/unknown-verb area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: unknown-verb
promise: A call naming no known verb is refused `unknown-verb`, the instance unchanged.
@row_add n=289 row=contract/refusal-cause/malformed area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: malformed
promise: A malformed call is refused `malformed`, the instance unchanged.
@row_add n=290 row=contract/refusal-cause/not-found area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: not-found
promise: A call naming an id not on the screen is refused `not-found`, the instance unchanged.
@row_add n=291 row=contract/refusal-cause/stale area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: stale
promise: A call naming a stale id is refused `stale`, the instance unchanged.
@row_add n=292 row=contract/refusal-cause/disabled area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: disabled
promise: A call on a disabled target is refused `disabled`, the instance unchanged.
@row_add n=293 row=contract/refusal-cause/off-screen area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: off-screen
promise: A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
@row_add n=294 row=contract/refusal-cause/covered area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: covered
promise: A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
@row_add n=295 row=contract/refusal-cause/fonts-unavailable area=integration-test-strategy rec=fct-t7rg232i@1
boundary: driver refusal causes
item: fonts-unavailable
promise: A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
@row_add n=296 row=contract/session-run/scroll area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: scroll
promise: A `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view.
@row_add n=297 row=decision/typed-password-never-returned area=integration-test-strategy rec=fct-t7rg232i@1
name: Typed passwords never returned
rule: A password typed through the driver occurs nowhere in what the call returns, reads `MASKED_VALUE`, and never leaves as pixels in a screenshot.
guard: stand_act_screenshot, stand_act_filled
@row_add n=298 row=decision/no-sleep-or-wall-clock-in-checks area=integration-test-strategy rec=fct-t7rg232i@1
name: No sleep or wall-clock read in checks
rule: Session checks take no sleep and read no wall clock; time moves by `advance` alone.
except: stand_virtual_clock's one 800 ms sleep and stand_load_wait's clock reads and 200 ms stand-in sleep, both checks whose subject is wall time
@row_add n=299 row=decision/driver-log-fixed-words-only area=integration-test-strategy rec=fct-t7rg232i@1
name: Driver log lines hold fixed words and counts only
rule: At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line.
guard: stand_act_spans
@row_add n=300 row=contract/session-run/type area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: type
promise: A `type` replaces what a filled control holds and an empty text clears it; a masked control's written form holds the mask, never the typed text.
@row_add n=301 row=contract/session-run/changed-node-states area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: acting call result
promise: An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
@row_add n=302 row=contract/session-run/type-on-select area=integration-test-strategy rec=fct-t7rg232i@1
boundary: escher_driver Session
item: type on a select
promise: A `type` on a single-choice select chooses the first option whose label starts with the text, its diff naming the select with `value` and the options whose `selected` moved.
@row_none n=121 rec=fct-ome33psz@1 type=4
relied: A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`.
@row_add n=303 row=decision/reserved-graph-id-form area=integration-test-strategy rec=fct-csattoyz@1
name: Reserved owner-key id form
rule: No served id holds the reserved `{owner}///{key}` form.
guard: stand_graph
@row_add n=304 row=decision/handler-coverage-holds-no-typed-text area=integration-test-strategy rec=fct-ldnjdynp@1
name: Handler coverage line holds no typed text
rule: Text typed during a replayed check never reaches the handler coverage line.
guard: stand_handler_coverage
@row_add n=305 row=contract/handler-coverage/refused-step area=integration-test-strategy rec=fct-ldnjdynp@1
boundary: handler coverage
item: refused step
promise: A refused step exercises no handler; a keyboard activation exercises the element's click handler.
@row_add n=306 row=contract/handler-coverage/other-app-check area=integration-test-strategy rec=fct-ldnjdynp@1
boundary: handler coverage
item: check of another app
promise: A check recorded on another app contributes nothing to the coverage.
@row_add n=307 row=decision/handler-coverage-reads-every-replayed-screen area=integration-test-strategy rec=fct-qofvsdhn@2
name: Handler coverage reads every replayed screen
rule: Handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step.
state: provisional
@row_add n=308 row=decision/handler-coverage-canvas-uncounted area=integration-test-strategy rec=fct-omo3sn26@1
name: Canvas handlers not counted by handler coverage
rule: Handler coverage does not count a handler inside a canvas.
@row_add n=309 row=decision/handler-coverage-unreached-screens area=integration-test-strategy rec=fct-omo3sn26@1
name: Handlers on unreached screens not listed
rule: Handler coverage does not list a handler on a screen no replay reached.
why: by construction
@row_add n=310 row=decision/handler-coverage-bubbling-counts-ancestors area=integration-test-strategy rec=fct-omo3sn26@1
name: Bubbling dispatch counts every ancestor listener
rule: A bubbling dispatch counts every ancestor listener it passed as exercised, `stopPropagation` unread.
@row_none n=122 rec=fct-krn4eywd@1 type=5
@row_none n=123 rec=fct-xirvsrei@1 type=5
@row_none n=124 rec=fct-kzdrls5d@2 type=5
@row_none n=125 rec=fct-5t64ong4@2 type=5
@row_add n=311 row=decision/graph-answer-holds-no-value area=integration-test-strategy rec=fct-eja3jr5q@1
name: Graph answer holds no value
rule: A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text.
guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
@row_add n=312 row=contract/graph-command/argument area=integration-test-strategy rec=fct-eja3jr5q@1
boundary: the graph command
item: argument
promise: An argument to graph is refused malformed with nothing run.
@row_add n=313 row=decision/actions-answer-holds-no-value area=integration-test-strategy rec=fct-pocqbqtc@1
name: Actions answer holds no value
rule: An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field.
guard: stand_actions
@row_add n=314 row=contract/actions-command/argument area=integration-test-strategy rec=fct-pocqbqtc@1
boundary: the actions command
item: argument
promise: An argument to actions is refused malformed.
@row_add n=315 row=decision/session-state-owner-only area=integration-test-strategy rec=fct-2tbofxbl@1
name: Owner-only session state
rule: The session state directory is created with mode 0700 and its socket with mode 0600.
guard: stand_session_lifecycle
@row_add n=316 row=contract/session-errors/nosession area=integration-test-strategy rec=fct-2tbofxbl@1
boundary: session errors
item: NoSession
promise: Returned when a session is reached and none is running.
@row_add n=317 row=contract/session-errors/alreadyrunning area=integration-test-strategy rec=fct-2tbofxbl@1
boundary: session errors
item: AlreadyRunning
promise: Returned by start while a session is already up.
@row_add n=318 row=decision/session-refuses-out-of-grammar area=integration-test-strategy rec=fct-2tbofxbl@1
name: Out-of-grammar requests refused
rule: A session request outside the grammar or over the bound is refused with nothing run.
guard: stand_session_lifecycle
@row_add n=319 row=contract/session-protocol/hello area=integration-test-strategy rec=fct-2tbofxbl@1
boundary: the session protocol
item: hello
promise: hello answers idle_expiry_s.
@row_add n=320 row=contract/session-status/served area=integration-test-strategy rec=fct-2tbofxbl@1
boundary: the session status
item: served
promise: Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.
@row_add n=321 row=decision/session-idle-expiry area=integration-test-strategy rec=fct-2tbofxbl@1
name: Session idle expiry
rule: A session host that answers no request for its idle expiry ends exactly as on stop.
guard: stand_session_lifecycle
@row_add n=322 row=decision/session-logs-no-screen-content area=integration-test-strategy rec=fct-adkwyvky@5
name: Session logs nothing of the screen
rule: The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level.
guard: stand_session_quiet, host_log, mcp_log
@row_add n=323 row=decision/sink-drops-third-party-targets area=integration-test-strategy rec=fct-adkwyvky@5
name: Sink drops third-party targets
rule: The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it.
guard: telemetry_drop
@row_add n=324 row=contract/session-state-directory/session-sock area=integration-test-strategy rec=fct-adkwyvky@5
boundary: the session state directory
item: session.sock
promise: The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
@row_add n=325 row=contract/session-host-stderr/command-span area=integration-test-strategy rec=fct-adkwyvky@5
boundary: the session host's stderr
item: command span
promise: Exactly one command span line is written per call.
@row_add n=326 row=contract/mcp-role-stdout/protocol-lines area=integration-test-strategy rec=fct-adkwyvky@5
boundary: the mcp role's stdout
item: protocol lines
promise: The mcp role writes protocol lines only on stdout.
@row_add n=327 row=contract/mcp-role-stderr/tool-call area=integration-test-strategy rec=fct-adkwyvky@5
boundary: the mcp role's stderr
item: tool_call
promise: One tool_call line per call, carrying only its stated field names.
@row_none n=126 rec=fct-kyu3niri@1 type=5
@row_none n=127 rec=fct-222mhgz6@1 type=5
@row_none n=128 rec=fct-ptmssw6k@1 type=5
@row_none n=129 rec=fct-6jloyoi7@1 type=5
@row_none n=130 rec=fct-gkuwtzih@1 type=5
@row_none n=131 rec=fct-iyekes7g@1 type=5
@row_none n=132 rec=fct-3ss3nk3h@1 type=5
@row_none n=133 rec=fct-6g7tp3rl@1 type=5
@row_add n=328 row=decision/listeners-read-from-vdom-walk area=integration-test-strategy rec=fct-xs6og5lj@1
name: Listeners read from the vdom walk
rule: A node's current listeners are read from the vdom walk's dynamic attributes, mounted included, not from the bridge's listener attributes.
why: The bridge's listener attributes over-report: a listener removed on a re-render leaves its attribute standing, and a mounted listener writes none.
guard: dioxus_runtime_probe link_2_a_removed_listeners_attribute_stays
@row_add n=329 row=decision/graph-signal-registration area=integration-test-strategy rec=fct-3vcgyjbv@2
name: Task state registered for the UI graph
rule: The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change.
why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private.
guard: graph::tests, stand_graph
@row_none n=134 rec=fct-ssa4tu2c@1 type=7
@row_none n=135 rec=fct-evpcutg7@1 type=5
@row_none n=136 rec=fct-cl63gyie@1 type=5
@row_none n=137 rec=fct-sxplwlco@1 type=5
@row_none n=138 rec=fct-ecugpoxa@1 type=5
@row_none n=139 rec=fct-pmgeniqi@1 type=5
@row_none n=140 rec=fct-ogghnhg5@1 type=5
@row_none n=141 rec=fct-c3fx7sq3@1 type=5
@row_none n=142 rec=fct-yuul6eqq@1 type=5
@row_none n=143 rec=fct-fke3hzot@1 type=5
@row_none n=144 rec=fct-n5usxpjf@1 type=5
@row_none n=145 rec=fct-imcumwhv@1 type=5
@row_none n=146 rec=fct-2i5p76wj@1 type=5
@row_none n=147 rec=fct-n53ao5mu@1 type=5
@row_none n=148 rec=fct-ihjtktts@1 type=5
@row_none n=149 rec=fct-bz6tsqol@1 type=5
@row_none n=150 rec=fct-yylpqcib@1 type=5
@row_none n=151 rec=fct-ajbejuaf@1 type=5
@row_none n=152 rec=fct-lkbku6ez@1 type=5
@row_none n=153 rec=fct-3qjazz2x@1 type=5
@row_add n=330 row=decision/timer-driven-by-delivered-ticks area=mocking-stubbing-discipline rec=fct-7t2tqfsg@1
name: Timer driven by delivered ticks
rule: In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick.
guard: stand_timer, stand_settle
@row_add n=331 row=decision/advance-reports-clock-move area=mocking-stubbing-discipline rec=fct-7t2tqfsg@1
name: advance reports the clock's move
rule: advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained.
why: Ratified.
@row_none n=154 rec=fct-yh5nb256@1 type=5
@row_none n=155 rec=fct-be5nn6gv@1 type=5
@row_none n=156 rec=fct-es3mosg6@1 type=5
@row_none n=157 rec=fct-woxmjtvp@1 type=5
@row_none n=158 rec=fct-da2hnb5e@1 type=5
@row_add n=332 row=absence/mocking-library area=mocking-stubbing-discipline rec=fct-uyy2kykf@1,fct-duynbxnl@1
capability: A mocking library
@row_none n=159 rec=fct-5fficxxt@1 type=7
@row_none n=160 rec=fct-7agflx3x@1 type=7
@row_none n=161 rec=fct-uhjgy72w@1 type=7
@row_none n=162 rec=fct-pma3gjdm@1 type=7
@row_none n=163 rec=fct-quhgbzyl@1 type=7
@row_none n=164 rec=fct-e6u5yu6y@1 type=7
@row_none n=165 rec=fct-q5rsei7l@1 type=5
@row_add n=333 row=decision/keyboard-compat-coverage-exclusion area=quality-gates-coverage-targets rec=fct-fx4qekph@4
name: keyboard_compat.rs left out of coverage
rule: Both coverage runs exclude dioxus-native-dom's crate-private keyboard_compat.rs until escher takes upstream's retirement of it; the floors are unchanged by the exclusion.
why: The file is upstream's temporary keyboard-types 0.7/0.8 bridge whose conversion arms no test of ours reaches.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_none n=166 rec=fct-ucb2m2kd@1 type=5
@row_add n=334 row=decision/zero-failure-stand-repeat area=quality-gates-coverage-targets rec=fct-3ctmcebn@4
name: Zero-failure flakiness budget for stand checks
rule: The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg.
why: Ratified as a repeated job with zero failures.
guard: stand-repeat leg, job Stand checks repeated
@row_add n=335 row=decision/mutation-runs-under-nextest-profile area=quality-gates-coverage-targets rec=fct-5ane2bts@1
name: Mutation runs with cargo-mutants under nextest
rule: A mutation run is cargo-mutants with nextest as its test tool under a dedicated mutants profile that stops at the first failure and runs the direct tests first, the default profile untouched.
why: Under plain cargo test, hung integration binaries run before the library tests and make mutants read timeout.
@row_add n=336 row=decision/mutation-baseline-skipped area=quality-gates-coverage-targets rec=fct-5ane2bts@1
name: Mutation baseline skipped, read green by hand
rule: Mutation runs skip cargo-mutants' baseline; each unit's mutant test command is first read green on the unmutated tree.
why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, so a test set naming other packages cannot be its baseline and would read every mutant caught.
@row_add n=337 row=decision/mutation-runs-on-dev-host-only area=quality-gates-coverage-targets rec=fct-5ane2bts@1
name: Mutation runs on the dev host only
rule: Mutation runs are made on the dev host alone, never in CI.
why: cargo-mutants and cargo-nextest are host tools no lockfile resolves.
@row_add n=338 row=decision/complexity-ceiling-host-only area=quality-gates-coverage-targets rec=fct-zgelu5mp@2
name: Cognitive-complexity ceiling read on the host
rule: The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted; no CI leg runs it, it binds no function, and engine functions over it are left as they are.
@row_none n=167 rec=fct-jhfttgmh@1 type=5
@row_add n=339 row=contract/committed-checks/excluded-content area=test-data-fixtures rec=fct-6hjdlvz3@2
boundary: committed recorded checks
item: excluded content
promise: A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.
@row_none n=168 rec=fct-hutsqqej@1 type=5
@row_none n=169 rec=fct-dqcwjzuv@1 type=5
@row_none n=170 rec=fct-y2fxawue@1 type=5
@row_none n=171 rec=fct-zkjxcvz6@1 type=5
@row_none n=172 rec=fct-ay5qvau6@1 type=5
@row_none n=173 rec=fct-x42jilyi@1 type=5
@row_none n=174 rec=fct-iee65fcu@2 type=5
@row_none n=175 rec=fct-3qvwds5o@1 type=9
@row_none n=176 rec=fct-3ywgnxn6@1 type=5
@row_none n=177 rec=fct-xtrwomti@1 type=5
@row_none n=178 rec=fct-aro6klbd@1 type=5
@row_none n=179 rec=fct-rkqqewd2@1 type=5
@row_none n=180 rec=fct-mtrwcimp@1 type=5
@row_none n=181 rec=fct-wlhqy7jp@1 type=5
@row_none n=182 rec=fct-xblez74o@1 type=5
@row_none n=183 rec=fct-lb3dflix@1 type=5
@row_none n=184 rec=fct-zbl26ely@1 type=5
@row_none n=185 rec=fct-vovahn3t@1 type=5
@row_none n=186 rec=fct-gazbaorh@1 type=5
@row_none n=187 rec=fct-fxrwr2d3@1 type=5
@row_none n=188 rec=fct-afgwdd3i@1 type=5
@row_none n=189 rec=fct-72t265ab@1 type=5
@row_none n=190 rec=fct-idwsqern@1 type=5
@row_none n=191 rec=fct-d5jd3dwk@1 type=5
@row_none n=192 rec=fct-sztnzkhs@1 type=5
@row_none n=193 rec=fct-q7btvtti@1 type=5
@row_none n=194 rec=fct-yhbc56ti@1 type=5
@row_none n=195 rec=fct-pt7fkjy2@1 type=5
@row_none n=196 rec=fct-qbzx34mx@2 type=4
relied: The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider.
@row_none n=197 rec=fct-a24rqe7v@2 type=4
relied: The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag.
@row_none n=198 rec=fct-5d6i6kkv@1 type=5
@row_none n=199 rec=fct-j3iuwalc@1 type=5
@row_none n=200 rec=fct-aoszt6oj@1 type=5
@row_none n=201 rec=fct-243i7pja@1 type=5
@row_none n=202 rec=fct-qgatloyx@1 type=7
@row_add n=340 row=decision/blitz-tests-one-target-per-file area=test-scope-summary rec=fct-jal6nbq2@1
name: One blitz-tests target per file
rule: The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt.
guard: BlitzTestsTargetsTest
@row_add n=341 row=decision/capabilities-doc-pinned-to-schema area=test-scope-summary rec=fct-jal6nbq2@1
name: Capability statement pinned to the driver schema
rule: The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables.
guard: CapabilityStatementTest
@row_add n=342 row=decision/bounded-ci-package-installs area=test-scope-summary rec=fct-jal6nbq2@1
name: Bounded CI package installs
rule: Every package install in the CI workflow is bounded.
guard: test_ci_workflows.py
@row_add n=343 row=contract/escher-session-exit-codes/0 area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session exit codes
item: 0
promise: A host that served its session and stopped exits 0, with nothing on stdout and no state directory left.
@row_add n=344 row=contract/escher-session-exit-codes/1 area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session exit codes
item: 1
promise: An argument given to actions is refused with status 1.
@row_add n=345 row=contract/escher-session-exit-codes/2 area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session exit codes
item: 2
promise: An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
@row_add n=346 row=contract/escher-session-exit-codes/3 area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session exit codes
item: 3
promise: A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
@row_add n=347 row=contract/escher-session-exit-codes/closed-stdout area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session exit codes
item: closed stdout
promise: A run whose stdout is closed before it starts keeps its ending's status, printing no panic and creating no state directory.
@row_add n=348 row=contract/session-state-directory/length-bound area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: session state directory
item: length bound
promise: A state directory path of 94 bytes holds a session on Linux and of 90 bytes on other targets; one byte more is refused state-dir-too-long with its bound.
@row_add n=349 row=contract/escher-session-stderr/service-name area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session stderr
item: service.name
promise: Every stderr line is stamped service.name=seven_guis.
@row_add n=350 row=contract/escher-session-stderr/no-ids-names-or-typed-text area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session stderr
item: ids, names and typed text
promise: Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
@row_add n=351 row=contract/escher-session-stderr/tool-call-line area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: escher-session stderr
item: tool_call
promise: The mcp role writes one tool_call line per call, holding only its stated field names.
@row_add n=352 row=contract/mcp-server/stdout area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: stdout
promise: The mcp role writes protocol lines only on stdout.
@row_add n=353 row=contract/mcp-server/tools-list area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: tools/list
promise: tools/list offers the fourteen verbs in order.
@row_add n=354 row=contract/mcp-server/protocol-revision area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: protocol revision
promise: The server speaks the newest protocol revision it knows, answering a newer requested revision with that one.
@row_add n=355 row=contract/mcp-server/refused-input area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: refused input
promise: server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
@row_add n=356 row=contract/mcp-server/actions-argument area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: actions argument
promise: An actions tool call given an argument is answered isError.
@row_add n=357 row=contract/mcp-server/screenshot area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: screenshot
promise: A screenshot tool call is answered with an image block then a text block.
@row_add n=358 row=contract/mcp-server/help area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: MCP server
item: help
promise: help is answered before any start and reads the same bytes through MCP and through the command line.
@row_add n=359 row=contract/session-recording/same-check-on-both-surfaces area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: session recording
item: same check on both surfaces
promise: The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
@row_add n=360 row=contract/session-recording/group-open-directory area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: session recording
item: group-open directory
promise: A recording directory open to its group is refused before anything runs.
@row_add n=361 row=contract/session-recording/off-unix area=test-scope-summary rec=fct-k5y3m6cu@4
boundary: session recording
item: off unix
promise: Off unix, --record is refused Unsupported.
@row_add n=362 row=decision/advance-reports-exact-ms area=test-scope-summary rec=fct-k5y3m6cu@4
name: Hosted advance reports exactly the ms asked
rule: A hosted advance moves the timer session with no sleep, and every ask reports exactly the milliseconds asked.
why: Ratified.
guard: host_timer
@row_none n=203 rec=fct-obcdbfv5@1 type=5
@row_none n=204 rec=fct-bkltlzxy@1 type=5
@row_none n=205 rec=fct-76rd45c3@1 type=5
@row_none n=206 rec=fct-xzy32im2@1 type=9
@row_none n=207 rec=fct-6s4hzgr5@1 type=5
@row_none n=208 rec=fct-zfabgeus@1 type=5
@row_none n=209 rec=fct-2mgrwptl@1 type=5
@row_add n=363 row=decision/chosen-select-row-colours area=test-scope-summary rec=fct-wg3ym5ko@1
name: Chosen select row colour pair
rule: A chosen select row is painted with the engine default #c8c8c8 on #000000.
why: The pair is Chromium's inactive selected pair, reading 12.56:1.
guard: a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair
@row_none n=210 rec=fct-a7mopyf2@2 type=9
@row_none n=211 rec=fct-romvomei@1 type=4
relied: A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.
@row_none n=212 rec=fct-2ha2za3s@1 type=5
@row_none n=213 rec=fct-luwk6iun@2 type=5
@row_none n=214 rec=fct-4q3czvm2@2 type=4
relied: A task polled during render acts on the document as the previous render left it, so a node created in that same render cannot yet be focused.
@row_none n=215 rec=fct-4qy7sqin@1 type=4
relied: The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it.
@row_none n=216 rec=fct-wi27eq55@2 type=9
@row_none n=217 rec=fct-bclvkqhn@1 type=5
@row_none n=218 rec=fct-j7rhr47j@1 type=5
@row_none n=219 rec=fct-objnaett@1 type=5
@row_none n=220 rec=fct-n4zvuqt2@1 type=4
relied: The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute.
@row_none n=221 rec=fct-i7tzvn3e@1 type=5
@row_none n=222 rec=fct-roxq5ew4@1 type=5
@row_none n=223 rec=fct-hynoc6qu@1 type=5
@row_none n=224 rec=fct-j7vriwbc@1 type=5
@row_add n=364 row=decision/password-caret-placed-by-mask area=test-scope-summary rec=fct-rsfv2k4d@1
name: Password caret placed by the mask
rule: A click inside a password field holding a value places the caret by the painted mask bullets, so one click at one offset into two values of one length lands on one index whatever their characters, and a double click selects the whole value.
why: Ratified.
guard: password_caret_click
@row_none n=225 rec=fct-rwm6sik5@1 type=9
@row_none n=226 rec=fct-cnwuyq4f@1 type=5
@row_none n=227 rec=fct-j4q5jgse@1 type=4
relied: A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good.
@row_none n=228 rec=fct-viz3xpec@1 type=5
@row_add n=365 row=decision/eight-refusal-causes-in-order area=test-scope-summary rec=fct-nks3g3e7@6
name: Eight refusal causes in a fixed order
rule: The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy.
why: Ratified.
guard: the_eight_causes_are_named_in_order
@row_add n=366 row=decision/driver-instance-verbs-tested-by-stand area=test-scope-summary rec=fct-nks3g3e7@6
name: Driver verbs over a held instance tested by integration
rule: The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts.
why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
@row_none n=229 rec=fct-ohaypmtj@1 type=5
@row_none n=230 rec=fct-i5a6vv3t@5 type=5
@row_none n=231 rec=fct-tazqna57@1 type=5
@row_add n=367 row=decision/buffered-render-on-dioxus-07-bridge area=test-scope-summary rec=fct-e2zfwpza@3
name: Buffered render on the Dioxus 0.7 bridge
rule: The Dioxus 0.7 bridge renders into a mutation buffer with the document unborrowed, so a task polled during render can access the document.
why: Upstream's buffered render ported onto the Dioxus 0.7 bridge.
guard: dioxus_runtime_probe
@row_add n=368 row=decision/upstream-buffered-render-test-kept-as-written area=test-scope-summary rec=fct-e2zfwpza@3
name: Upstream buffered-render test kept as written
rule: Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7.
why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for.
guard: dioxus_runtime_probe
@row_add n=369 row=decision/upstream-mounted-flush-test-left-out area=test-scope-summary rec=fct-ybmlkzsq@2
name: Upstream mounted-flush test left out
rule: Upstream's test of an element removed before its mounted event is flushed is left out of the fork until the Dioxus 0.8 move, so the upstream guard on queued mounted events has no test of its own.
why: On Dioxus 0.7 it reads 1 mounted event where it asserts 0, from Dioxus 0.7's scheduling, not the borrow.
@row_none n=232 rec=fct-rjsmqegm@3 type=7
@row_add n=370 row=decision/stand-checks-in-both-layout-modes area=test-scope-summary rec=fct-z2ynupl3@6
name: Stand checks run in both layout modes
rule: A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
@row_add n=371 row=decision/typed-password-never-leaves area=test-scope-summary rec=fct-z2ynupl3@6
name: A typed password never leaves the instance
rule: A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels.
guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
@row_add n=372 row=decision/no-screenshot-step-in-committed-checks area=test-scope-summary rec=fct-z2ynupl3@6
name: No screenshot step in a committed check
rule: A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only.
why: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a shot is byte-identical on one host only
guard: stand_screenshot_repeat
@row_add n=373 row=decision/advance-moves-exactly-the-ms-asked area=test-scope-summary rec=fct-z2ynupl3@6
name: Advance moves exactly the time asked
rule: An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms.
why: ratified
guard: stand_act_timer
@row_add n=374 row=decision/mcp-causes-served-with-remedy area=test-scope-summary rec=fct-z2ynupl3@6
name: Refusal causes served with their remedy over MCP
rule: Every refusal cause a tool call reaches is served through MCP with its remedy.
why: ratified
guard: stand_mcp_tools
except: unknown-verb is not served through MCP
@row_add n=375 row=decision/one-virtual-clock area=test-scope-summary rec=fct-z2ynupl3@6
name: One virtual clock drives engine time
rule: The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock.
why: ratified
guard: virtual_clock_engine, stand_virtual_clock
@row_add n=376 row=decision/snapshot-text-budget area=test-scope-summary rec=fct-z2ynupl3@6
name: Snapshot text within a byte budget
rule: A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text.
guard: stand_snapshot_text, stand_snapshot_parts
@row_add n=377 row=decision/owner-only-session-files area=test-scope-summary rec=fct-z2ynupl3@6
name: Owner-only session socket and recordings
rule: A session's state directory is 0700 and its socket 0600, and a recording directory open to its group is refused with nothing written.
guard: stand_session_lifecycle, stand_session_recording
@row_add n=378 row=contract/driver-refusal-causes/not-found area=test-scope-summary rec=fct-z2ynupl3@6
boundary: driver refusal causes
item: not-found
promise: An id no screen of the session read, or that no element carries, is refused not-found.
@row_add n=379 row=contract/driver-refusal-causes/stale area=test-scope-summary rec=fct-z2ynupl3@6
boundary: driver refusal causes
item: stale
promise: An id an earlier screen of the session read that names no element now is refused stale.
@row_add n=380 row=contract/driver-refusal-causes/malformed area=test-scope-summary rec=fct-z2ynupl3@6
boundary: driver refusal causes
item: malformed
promise: An id longer than 1024 bytes is refused malformed before any lookup.
@row_add n=381 row=contract/driver-refusal-causes/fonts-unavailable area=test-scope-summary rec=fct-z2ynupl3@6
boundary: driver refusal causes
item: fonts-unavailable
promise: A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
@row_add n=382 row=contract/driver-command-span/one-line-per-call area=test-scope-summary rec=fct-z2ynupl3@6
boundary: driver command span
item: command line
promise: Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
@row_none n=233 rec=fct-eqb74sbl@1 type=5
@row_none n=234 rec=fct-gzdlndnj@1 type=5
@row_none n=235 rec=fct-fdugptjl@1 type=5
@row_none n=236 rec=fct-bjgtkrkr@1 type=5
@row_none n=237 rec=fct-lwk4jfam@1 type=5
@row_none n=238 rec=fct-ekyibivt@1 type=5
@row_none n=239 rec=fct-57a7c2li@1 type=5
@row_none n=240 rec=fct-h2tpz3mp@1 type=5
@row_none n=241 rec=fct-3ptaekqr@1 type=5
@row_none n=242 rec=fct-5psfaioe@1 type=5
@row_none n=243 rec=fct-nkerrabf@1 type=5
@row_none n=244 rec=fct-t7ntsbvc@1 type=5
@row_none n=245 rec=fct-bzkoiqdf@1 type=5
@row_none n=246 rec=fct-5bb5lxig@1 type=5
@row_none n=247 rec=fct-ejy2b2a5@1 type=4
@row_none n=248 rec=fct-xl4n5i4u@1 type=9
@row_none n=249 rec=fct-tz35iqmq@2 type=5
@row_none n=250 rec=fct-poisqw4h@1 type=5
@row_none n=251 rec=fct-vwiy3q7s@1 type=4
relied: The lint's overlap reading skips any entry inside a select's box, closed or open, so the stand's lint reads no overlap on select options and open-list rows.
@row_none n=252 rec=fct-rovrmg6e@1 type=5
@row_none n=253 rec=fct-odg47pg2@1 type=5
@row_none n=254 rec=fct-g7xlt7m3@1 type=5
@row_add n=383 row=decision/harness-driven-tests area=test-strategy rec=fct-yivrglnx@1
name: Harness-driven engine tests
rule: Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
@row_add n=384 row=contract/harness-settle/notsettled area=test-strategy rec=fct-yivrglnx@1
boundary: The harness's settle
item: NotSettled
promise: Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
@row_add n=385 row=decision/headless-screenshot area=test-strategy rec=fct-yivrglnx@1
name: Headless screenshot from the harness
rule: The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
@row_add n=386 row=decision/contrast-reader-reads-only area=test-strategy rec=fct-yivrglnx@1
name: Contrast reader changes nothing
rule: The harness's contrast reader reads and changes nothing: no mutation, input, pump or settle.
@row_add n=387 row=contract/text-contrast/no-reading area=test-strategy rec=fct-yivrglnx@1
boundary: The harness's text contrast reading
item: no reading
promise: No contrast is read where the element bears no text, or where an element on the walk has opacity below 1, a filter, a background image or a mix-blend-mode other than normal.
@row_add n=388 row=decision/wcag-2-contrast-ratio area=test-strategy rec=fct-yivrglnx@1
name: WCAG 2.x contrast ratio
rule: The contrast ratio is the WCAG 2.x ratio over the color crate's relative luminance.
@row_add n=389 row=decision/stable-tree-serialization area=test-strategy rec=fct-yivrglnx@1
name: Stable tree serialization
rule: The harness produces a stable one-node-per-line tree serialization with geometry.
why: Suitable for snapshot-style assertions.
@row_add n=390 row=decision/check-owns-its-host-process area=test-strategy rec=fct-jqnoqslu@1
name: A check owns its host process
rule: A check that needs a session process starts and stops its own host, every wait bounded, with a guard that kills and reaps a host a failing check still holds and its state directory removed at stop.
@row_add n=391 row=decision/child-streams-to-null area=test-strategy rec=fct-jqnoqslu@1
name: Child streams sent to null
rule: A host child's streams are sent to null unless the check reads them.
why: So the child's own libtest lines never reach the parent's output, where the agent run script would count them as tests.
@row_add n=392 row=decision/socket-checks-unix-gated area=test-strategy rec=fct-jqnoqslu@1
name: Socket session checks are unix-gated
rule: A check holding a session over the socket is gated to unix.
except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
@row_add n=393 row=decision/started-host-held-behind-guard area=test-strategy rec=fct-jqnoqslu@1
name: Started hosts held behind a guard
rule: The seven_guis checks hold a session spawned by the binary's start command behind a guard that stops it.
why: The start command closes the host's streams and neither kills nor reaps it.
@row_add n=394 row=decision/drain-host-pipes-while-running area=test-strategy rec=fct-jqnoqslu@1
name: Host pipes drained while the host runs
rule: A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading.
why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
@row_add n=395 row=decision/mcp-capture-counts-by-span area=test-strategy rec=fct-jqnoqslu@1
name: MCP child capture counted by span
rule: A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
@row_add n=396 row=decision/cli-checks-run-the-binary area=test-strategy rec=fct-slbj25yc@1
name: Command-line checks run the binary
rule: The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
@row_add n=397 row=decision/shell-flows-are-cargo-test-assets area=test-strategy rec=fct-slbj25yc@1
name: Shell flows are cargo test assets
rule: POSIX sh flow scripts run under plain sh from a Rust cargo test target; the scripts are assets of that target, not a runner of their own, and no test framework is added.
@row_add n=398 row=decision/cli-and-flow-targets-unix-gated area=test-strategy rec=fct-slbj25yc@1
name: Command-line and flow targets unix-gated
rule: The command-line check target, the shell flow target and the recording target are gated to unix.
@row_add n=399 row=contract/record-checks-script/arguments area=test-strategy rec=fct-slbj25yc@1
boundary: The committed checks' recording script
item: arguments
promise: Takes two arguments, the binary and an existing output directory.
@row_add n=400 row=contract/record-checks-script/rec-directory area=test-strategy rec=fct-slbj25yc@1
boundary: The committed checks' recording script
item: rec directory
promise: Makes its rec directory with mode 0700.
@row_add n=401 row=contract/record-checks-script/failure-output area=test-strategy rec=fct-slbj25yc@1
boundary: The committed checks' recording script
item: failure output
promise: Prints only step N on failure.
@row_add n=402 row=decision/handler-coverage-by-replay area=test-strategy rec=fct-5d4ttzku@2
name: Handler coverage measured by replay
rule: Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
@row_add n=403 row=contract/escher-session-commands/coverage area=test-strategy rec=fct-5d4ttzku@2
boundary: escher-session commands
item: coverage <app> <check>...
promise: Lists the handlers no given recorded check exercised.
@row_add n=404 row=decision/capability-claims-name-their-checks area=test-strategy rec=fct-krjmr356@2
name: Capability claims name their checks
rule: The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
@row_add n=405 row=decision/capability-check-names-held-to-schema area=test-strategy rec=fct-krjmr356@2
name: Claim section names and counts held to the schema
rule: Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand.
guard: test_capability_statement.py
@row_add n=406 row=contract/handler-coverage-limits/canvas-handler area=test-strategy rec=fct-krjmr356@2
boundary: Handler coverage limits
item: canvas handler
promise: A handler inside a canvas is not counted.
@row_add n=407 row=contract/handler-coverage-limits/unreached-screen area=test-strategy rec=fct-krjmr356@2
boundary: Handler coverage limits
item: unreached screen
promise: A handler on no screen a replay reached is not listed.
@row_add n=408 row=contract/handler-coverage-limits/bubbling-dispatch area=test-strategy rec=fct-krjmr356@2
boundary: Handler coverage limits
item: bubbling dispatch
promise: A bubbling dispatch counts every ancestor listener it passed.
@row_add n=409 row=decision/pixel-tests-cpu-buffer area=test-strategy rec=fct-phlik2ko@1
name: Pixel tests on a CPU buffer
rule: Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.
@row_add n=410 row=decision/frames-compared-whole area=test-strategy rec=fct-phlik2ko@1
name: Frames compared whole
rule: A shot's PNG is compared whole, equal to the harness's raster and byte-identical across runs on one host.
guard: stand_act_screenshot, stand_screenshot_repeat
@row_add n=411 row=decision/no-shot-in-cross-os-byte-checks area=test-strategy rec=fct-phlik2ko@1
name: No shot in cross-OS byte comparisons
rule: A check compared byte for byte across OSes holds no shot.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
@row_add n=412 row=decision/png-decoding-in-checks area=test-strategy rec=fct-phlik2ko@1
name: PNG decoding in checks
rule: A blitz-tests check decodes a PNG through the png dev-dependency; seven_guis, which has none, reads it with a hand base64 reader and the PNG header.
@row_add n=413 row=decision/stand-checks-bundled-font area=test-strategy rec=fct-2vwamopj@2
name: Stand checks under a bundled font
rule: The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off.
except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
@row_add n=414 row=decision/shot-refused-without-fonts area=test-strategy rec=fct-2vwamopj@2
name: Shot refused without a usable font
rule: A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path.
guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
@row_add n=415 row=decision/wpt-conformance-report-only area=test-strategy rec=fct-cl3n5ve2@1
name: WPT conformance is report-only
rule: A WPT conformance reading is report-only and grades no criterion.
@row_add n=416 row=decision/inline-cfg-test-modules area=test-strategy rec=fct-7m7lxsmy@5,fct-dssptnyw@1
name: Unit tests in inline test modules
rule: Unit tests sit in cfg(test) modules inside their source files.
except: Eight cfg(test) modules held in files of their own through a path attribute at their parent.
@row_add n=417 row=decision/one-test-target-per-file area=test-strategy rec=fct-7m7lxsmy@5
name: One test target per file
rule: blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false.
why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target.
guard: BlitzTestsTargetsTest
@row_add n=418 row=decision/shared-test-modules-not-targets area=test-strategy rec=fct-7m7lxsmy@5
name: Shared test modules are no target
rule: Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
@row_none n=255 rec=fct-xqiavvkf@1 type=5
@row_add n=419 row=decision/bridge-tests-child-module area=unit-test-strategy rec=fct-dssptnyw@1
name: Bridge tests a child of the dioxus document
rule: The dioxus document's bridge tests sit in a child module of the dioxus document module.
why: The event handler's two fields are private to that module and it has no constructor.
@row_add n=420 row=decision/test-module-split-keeps-paths area=unit-test-strategy rec=fct-dssptnyw@1
name: Split test modules keep their paths
rule: A unit-test module moved to a file of its own is moved out whole with its test paths unchanged.
@row_none n=256 rec=fct-wdvc4drt@1 type=5
@row_none n=257 rec=fct-xpytb3bu@1 type=5
@row_add n=421 row=contract/harness-settle/pass-limit area=unit-test-strategy rec=fct-u6hb3uyo@1
boundary: The harness's settle
item: pass limit
promise: Work for ever ends after exactly the settle pass limit of polls, reading Render.
@row_add n=422 row=contract/harness-settle/notsettled-message area=unit-test-strategy rec=fct-u6hb3uyo@1
boundary: The harness's settle
item: NotSettled message
promise: Every NotSettled message is non-empty, distinct and holds no slash.
@row_add n=423 row=decision/contrast-required-ratio area=unit-test-strategy rec=fct-u6hb3uyo@1
name: Required contrast ratio
rule: Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
@row_add n=424 row=decision/mutation-controlled-tests area=unit-test-strategy rec=fct-klxgwgcy@5,fct-l2ouwaqy@1,fct-3ferhyj7@3,fct-rg6yibnz@1
name: Tests controlled red-before-green
rule: A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again.
@row_add n=425 row=decision/assertions-print-no-argument-value area=unit-test-strategy rec=fct-klxgwgcy@5,fct-zot44d7e@3
name: Test failures print no argument value
rule: An assertion over a driver call names only its row and index and never an argument value.
why: A failing test log then prints no argument value.
@row_add n=426 row=decision/table-tests-count-their-rows area=unit-test-strategy rec=fct-klxgwgcy@5
name: Table-driven tests assert their row count
rule: Each table a driver test reads is stated in the test itself and its row count asserted.
@row_add n=427 row=decision/session-error-messages-fixed area=unit-test-strategy rec=fct-klxgwgcy@5
name: Session error messages are fixed text
rule: Every session error message is a non-empty fixed string holding no `/` and no exit code; the input-or-output arm adds only the kind's name and is spelled «input or output», since «I/O» holds a `/`.
@row_add n=428 row=contract/refusal-causes/cause-set area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Refusal causes
item: cause set
promise: A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
@row_add n=429 row=contract/refusal-causes/printed-form area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Refusal causes
item: printed form
promise: A refusal's printed form opens with its cause's name and ends with its remedy.
@row_add n=430 row=contract/lint-defects/overlap area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: overlap
promise: `overlap` is the first of the six lint defects, in their stated order.
@row_add n=431 row=contract/lint-defects/clipped area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: clipped
promise: `clipped` is the second of the six lint defects, in their stated order.
@row_add n=432 row=contract/lint-defects/truncated area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: truncated
promise: `truncated` is the third of the six lint defects, in their stated order.
@row_add n=433 row=contract/lint-defects/off-screen area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: off-screen
promise: `off-screen` is the fourth of the six lint defects, in their stated order.
@row_add n=434 row=contract/lint-defects/zero-size area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: zero-size
promise: `zero-size` is the fifth of the six lint defects, in their stated order.
@row_add n=435 row=contract/lint-defects/contrast area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint defects
item: contrast
promise: `contrast` is the sixth of the six lint defects, in their stated order.
@row_add n=436 row=contract/lint-findings/order area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Lint findings
item: order
promise: Lint findings come in document order, then in the stated order of the defects.
@row_add n=437 row=contract/driver-verbs/verb-table area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver verbs
item: verb table
promise: Fourteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
@row_add n=438 row=contract/driver-verbs/lookup area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver verbs
item: lookup
promise: A verb is found by its exact name only; a name differing in case or padding finds nothing.
@row_add n=439 row=contract/driver-verbs/level-check area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver verbs
item: level check
promise: A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
@row_add n=440 row=contract/driver-argument-kinds/name area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver argument kinds
item: ArgKind::Name
promise: A name argument is 1 to 32 bytes of `a-z`, `0-9` and `-`.
@row_add n=441 row=contract/driver-argument-kinds/part area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver argument kinds
item: ArgKind::Part
promise: A part argument is a whole number from 1 to `MAX_PART`, 1000.
@row_add n=442 row=contract/driver-json/number area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver JSON answers
item: number
promise: A number is written as its shortest decimal, and as `null` when not finite.
@row_add n=443 row=contract/driver-json/masked-value area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver JSON answers
item: masked value
promise: A masked node value is written as `MASKED_VALUE`.
@row_add n=444 row=contract/driver-cli/refused-line area=unit-test-strategy rec=fct-klxgwgcy@5
boundary: Driver command line
item: refused line
promise: A command line that is no command, or that the schema refuses, runs nothing.
@row_none n=258 rec=fct-fghufsse@3 type=5
@row_none n=259 rec=fct-5zhbhdnd@1 type=5
@row_none n=260 rec=fct-fwtnmbsl@2 type=5
@row_none n=261 rec=fct-qc3s4qp3@1 type=5
@row_add n=445 row=contract/coverage-line/shape area=unit-test-strategy rec=fct-jhsulcna@1
boundary: The coverage line
item: shape
promise: One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.
@row_add n=446 row=contract/coverage-line/owner area=unit-test-strategy rec=fct-jhsulcna@1
boundary: The coverage line
item: owner
promise: A handler's `owner` is left out when it has none.
@row_add n=447 row=contract/coverage-role/unreadable-check area=unit-test-strategy rec=fct-jhsulcna@1
boundary: The coverage role
item: unreadable check
promise: A missing file, a non-check, a check past its bound and another app's check are each refused `malformed`.
@row_add n=448 row=decision/actions-pinned-to-commit-sha area=unit-test-strategy rec=fct-vg67igpb@1
name: CI actions pinned to commit SHAs
rule: Every `uses:` in the CI workflows is pinned to a 40-hex commit SHA, and every Rust toolchain step names its toolchain.
guard: CiWorkflowTest
@row_add n=449 row=decision/ci-uses-no-secrets area=unit-test-strategy rec=fct-vg67igpb@1
name: CI uses no secrets
rule: No CI workflow step reads a `secrets.` value.
guard: CiWorkflowTest
@row_add n=450 row=decision/bounded-package-installs area=unit-test-strategy rec=fct-vg67igpb@1
name: Bounded package installs in CI
rule: No CI step runs `apt-get` directly: packages are installed through the install script with package names only, and the one action step that installs packages carries a `timeout-minutes` above 0.
guard: test_o_package_installs_are_bounded
@row_add n=451 row=decision/upstream-repository-guard area=unit-test-strategy rec=fct-vg67igpb@1
name: Upstream-only publishing jobs
rule: Every job of publish-browser, wpt and wpt-post-results runs only in the `DioxusLabs/blitz` repository.
guard: UpstreamGuardTest
@row_add n=452 row=decision/fast-jobs-gate-the-rest area=unit-test-strategy rec=fct-vg67igpb@1
name: Fast jobs gate the rest
rule: The four fast CI jobs need nothing and every other job needs all four.
guard: CiWorkflowTest
@row_add n=453 row=decision/failure-logs-uploaded area=unit-test-strategy rec=fct-vg67igpb@1
name: Leg logs uploaded on failure
rule: Each leg job uploads its log on failure, kept 7 days.
guard: CiWorkflowTest
@row_add n=454 row=decision/coverage-floor-per-crate area=unit-test-strategy rec=fct-vg67igpb@1
name: Coverage floor per crate
rule: The coverage leg exits 0 when every crate is at its floor and exits 1 when one is under it.
guard: LegScriptTest
@row_add n=455 row=contract/ci-leg-exit-codes/2 area=unit-test-strategy rec=fct-vg67igpb@1
boundary: CI leg script exit codes
item: 2
promise: An unknown leg exits 2.
@row_add n=456 row=decision/apt-install-retries area=unit-test-strategy rec=fct-vg67igpb@1
name: CI package install retried
rule: A failed install attempt is tried again from `update`, a stalled call ends at its bound and is tried again, and an install failed by its bound is repaired before the next attempt.
guard: InstallScriptTest
@row_add n=457 row=decision/blitz-tests-manifest-difference area=unit-test-strategy rec=fct-l2ouwaqy@1
name: blitz-tests manifest's difference from upstream
rule: The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`.
why: The fork's standing two-line difference from upstream's manifest.
guard: BlitzTestsTargetsTest
@row_add n=458 row=decision/cold-agent-tests-offline area=unit-test-strategy rec=fct-3ferhyj7@3
name: Cold-agent pipe tested offline
rule: The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary.
guard: ColdAgentPipeTest
@row_add n=459 row=contract/cold-agent-verdict/capability-unused area=unit-test-strategy rec=fct-3ferhyj7@3
boundary: Cold-agent verdict
item: capability-unused
promise: A run that leaves a capability unused fails `capability-unused`.
@row_add n=460 row=contract/cold-agent-verdict/recording-not-kept area=unit-test-strategy rec=fct-3ferhyj7@3
boundary: Cold-agent verdict
item: recording-not-kept
promise: A `start` that left no kept check fails `recording-not-kept`.
@row_add n=461 row=decision/cold-agent-isolation area=unit-test-strategy rec=fct-3ferhyj7@3
name: Cold-agent run isolated
rule: A cold-agent run fails on an isolation breach: an extra tool, a second server, a working directory naming the root, or an API key source.
guard: ColdAgentPipeTest
@row_add n=462 row=decision/cold-agent-output-scrubbed area=unit-test-strategy rec=fct-3ferhyj7@3
name: Cold-agent output scrubbed
rule: A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text.
guard: ColdAgentPipeTest
@row_add n=463 row=decision/cold-agent-session-outside-root area=unit-test-strategy rec=fct-3ferhyj7@3
name: Cold-agent session outside the root
rule: A cold-agent session runs outside the repository root and is removed after, and its prompt names no stand id.
guard: ColdAgentPipeTest
@row_add n=464 row=decision/capability-statement-from-schema area=unit-test-strategy rec=fct-rg6yibnz@1
name: Capability statement read from the schema
rule: The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied.
guard: CapabilityStatementTest
@row_none n=262 rec=fct-fuxjvpim@1 type=5
@row_none n=263 rec=fct-bvdt3u7x@1 type=5
@row_none n=264 rec=fct-6mdwjb2z@1 type=5
@row_none n=265 rec=fct-2uew2nc7@1 type=5
@row_none n=266 rec=fct-g34cljfv@1 type=5
@row_none n=267 rec=fct-gyr237f2@1 type=5
@row_none n=268 rec=fct-ritaop23@1 type=5
@row_none n=269 rec=fct-hskbfmoc@1 type=5
@row_none n=270 rec=fct-3gle5qdo@1 type=5
@row_none n=271 rec=fct-mi2j5ztx@1 type=5
@row_none n=272 rec=fct-aroqltkw@1 type=5
@row_add n=465 row=contract/snapshot-node/clickable area=unit-test-strategy rec=fct-5kl4r7k5@1
boundary: Snapshot node
item: clickable
promise: `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.
@row_add n=466 row=contract/element-id/virtual-form area=unit-test-strategy rec=fct-3cpa7acl@1
boundary: Element ids
item: virtual form
promise: The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
@row_add n=467 row=decision/one-actionable-rule area=unit-test-strategy rec=fct-53uokwll@1
name: One actionable rule for both readers
rule: Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark.
why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener.
guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
@row_none n=273 rec=fct-czgt6pi3@1 type=5
@row_none n=274 rec=fct-czv5nz5r@1 type=7
@row_none n=275 rec=fct-qjt5fv4i@1 type=7
@row_none n=276 rec=fct-ioeznsad@1 type=7
@row_none n=277 rec=fct-7qct2f7a@1 type=7
@row_none n=278 rec=fct-4jrdv2xc@1 type=9
@row_none n=279 rec=fct-wsv7ekbq@1 type=S
@row_add n=468 row=overview/session-lifecycle area=session-lifecycle rec=fct-tjhljset@1
text: The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
@row_add n=469 row=contract/driver-session/start-label area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: Session::start
promise: Takes a label of 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs once on the calling thread; an invalid label is `InvalidLabel` with nothing booted.
@row_add n=470 row=contract/driver-session/act area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: act
promise: Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
@row_add n=471 row=contract/driver-session/load-wait area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: LOAD_WAIT
promise: A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
@row_add n=472 row=contract/driver-session/run-validation area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: run
promise: A call is validated first, and a refused call has run nothing.
@row_add n=473 row=contract/driver-session/stale-or-not-found area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: stale · not-found
promise: An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
@row_add n=474 row=contract/driver-session/id-record area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: record of ids
promise: Holds id text only, at most 4096 ids, for the session's life, no id longer than `MAX_ID_BYTES`, read by nothing outside the crate.
@row_add n=475 row=contract/driver-session/refusal-order area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: refusal order
promise: A `click` or `type` on an element that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
@row_add n=476 row=contract/driver-session/acting-verbs area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: acting verbs
promise: `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after; `scroll` adds `in_view`.
@row_add n=477 row=contract/driver-session/reading-verbs area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: reading verbs
promise: `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
@row_add n=478 row=contract/driver-session/snapshot area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: snapshot
promise: Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
@row_add n=479 row=contract/driver-session/graph area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: graph
promise: Returns `Outcome::Graphed` holding the screen's `UiGraph`.
@row_add n=480 row=contract/driver-session/not-settled-result area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: settled
promise: A verb on an instance that did not go quiet is a result, `settled` false with its `busy` class, and is not rolled back.
@row_add n=481 row=contract/driver-session/screenshot area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: screenshot
promise: Returns a PNG of the whole screen, or of an element's bounds cropped to the visible part and rounded outward, changing nothing; a crop of no pixel is `off-screen`, an app with no font to paint text `fonts-unavailable`.
@row_add n=482 row=decision/advance-moves-clock-exactly area=session-lifecycle rec=fct-3ylhu62k@3
name: advance moves the clock exactly
rule: Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed.
why: Ratified.
@row_add n=483 row=contract/driver-session/session-verb-in-run area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: session-level verb
promise: `start`, `status` or `stop` handed to the instance call path is refused `unknown-verb`.
@row_add n=484 row=contract/driver-session/type-replaces area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: type
promise: After the focusing click, where the snapshot reads a value for the target, the driver selects all and types, so an empty text clears the control; a select is focused with no click and no DOM focus event.
@row_add n=485 row=contract/driver-session/run-carriers area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Driver session
item: run carriers
promise: The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
@row_add n=486 row=contract/session-recording/directory area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Session recording
item: directory
promise: Unix only, `Unsupported` elsewhere; the directory is created `0700` or refused `RecordDirNotPrivate`.
@row_add n=487 row=contract/session-recording/check-file area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Session recording
item: check file
promise: `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.
@row_add n=488 row=contract/session-recording/record-fields area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Session recording
item: record fields
promise: Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
@row_add n=489 row=contract/session-recording/bounds area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Session recording
item: bounds
promise: Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and the failure reads true once; the call is not rolled back.
@row_add n=490 row=contract/session-recording/check-read area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Session recording
item: Check::from_bytes
promise: A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
@row_add n=491 row=contract/check-replay/another-app area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Check replay
item: another app
promise: A check replayed on a session whose label is not the check's app runs nothing.
@row_add n=492 row=contract/check-replay/stop area=session-lifecycle rec=fct-3ylhu62k@3
boundary: Check replay
item: stop
promise: Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
@row_add n=493 row=decision/replay-through-the-call-path area=session-lifecycle rec=fct-3ylhu62k@3
name: Replay through the ordinary call path
rule: A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type.
why: Ratified.
@row_add n=494 row=contract/session-host/idle-expiry area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: IDLE_EXPIRY
promise: A host holds one session until a `stop` request or until no request has been answered for 1,800 s, ending then exactly as on `stop`.
@row_add n=495 row=contract/session-host/served area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: served
promise: Every answered request, a refused or malformed one included, counts toward `served` and starts the expiry count again.
@row_add n=496 row=contract/session-host/start area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: start
promise: Spawns the host command and returns its first answer.
@row_add n=497 row=contract/session-host/attach area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: attach
promise: Returns `Hello { pid, label, served, idle_expiry_s }`.
@row_add n=498 row=contract/session-host/stop area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: stop
promise: Returns when the state directory is gone, bounded at 5 s.
@row_add n=499 row=contract/session-host/call area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session host
item: call
promise: Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
@row_add n=500 row=contract/session-errors/error-set area=session-lifecycle rec=fct-6jnjrehj@1,fct-klxgwgcy@5
boundary: Session errors
item: error set
promise: Fifteen session errors, each stating a kind, a fixed meaning and a remedy; the state directory's message states its bound.
@row_add n=501 row=contract/session-errors/answer-too-large area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: AnswerTooLarge
promise: A call's answer over the socket's bound, read from an `oversize` reply, the call having run.
@row_add n=502 row=contract/session-errors/no-session area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: NoSession
promise: No socket file in the state directory.
@row_add n=503 row=contract/session-errors/already-running area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: AlreadyRunning
promise: A start where a session already answers.
@row_add n=504 row=contract/session-errors/timeout area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: Timeout
promise: An answer read past its 30 s per-read bound.
@row_add n=505 row=contract/session-errors/record-dir-not-private area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: RecordDirNotPrivate
promise: A recording directory open to its group or others.
@row_add n=506 row=contract/session-errors/recording-failed area=session-lifecycle rec=fct-6jnjrehj@1
boundary: Session errors
item: RecordingFailed
promise: A recording that ended, read from an `ok v3 recording-failed` reply.
@row_add n=507 row=contract/session-wire/connection area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: connection
promise: One request line and one reply line per connection, every request carrying version `v3`.
@row_add n=508 row=contract/session-wire/hello area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: hello v3
promise: Answered `ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}`.
@row_add n=509 row=contract/session-wire/stop area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: stop v3
promise: Answered `ok v3 stopping`.
@row_add n=510 row=contract/session-wire/call area=session-lifecycle rec=fct-2k7iskpv@2,fct-klxgwgcy@5
boundary: Session wire
item: call v3
promise: `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.
@row_add n=511 row=contract/session-wire/accepted area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: ok v3 accepted
promise: `ok v3 accepted <json>` carries an accepted call's answer.
@row_add n=512 row=contract/session-wire/refused area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: ok v3 refused
promise: `ok v3 refused <json>` carries a refused call's refusal.
@row_add n=513 row=contract/session-wire/oversize area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: ok v3 oversize
promise: Sent when a call's answer is over the answer bound.
@row_add n=514 row=contract/session-wire/recording-failed area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: ok v3 recording-failed
promise: Sent in place of the answer when the host's recording ended after the call, read as `SessionError::RecordingFailed`.
@row_add n=515 row=contract/session-wire/refused-version area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: refused version
promise: A known word with another version is answered `refused version`.
@row_add n=516 row=contract/session-wire/refused-malformed area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: refused malformed
promise: Any other line is answered `refused malformed`.
@row_add n=517 row=contract/session-wire/request-bound area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: request bound
promise: A request line is at most 16,384 bytes.
@row_add n=518 row=contract/session-wire/lifecycle-reply-bound area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: lifecycle reply bound
promise: A lifecycle reply is at most 128 bytes.
@row_add n=519 row=contract/session-wire/answer-bound area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: answer bound
promise: An answer is at most 1,048,576 bytes on its JSON line.
@row_add n=520 row=contract/session-wire/answer-read-bound area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: answer read bound
promise: A call's answer is read under 30 s per read; a listener that never answers ends the read as `SessionError::Timeout`.
@row_add n=521 row=contract/session-wire/other-io-bound area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: other read and write bound
promise: Every read and write other than a call's answer is bounded at 2 s.
@row_add n=522 row=decision/socket-carries-commands area=session-lifecycle rec=fct-2k7iskpv@2
name: The socket carries commands
rule: The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff and lint findings out.
why: Ratified, where until then none of them crossed.
@row_add n=523 row=decision/screenshot-in-band area=session-lifecycle rec=fct-2k7iskpv@2
name: Screenshot pixels in band
rule: A screenshot's pixels cross in band, a PNG as padded base64 inside the answer's one JSON line `{"width":…,"height":…,"png":…}`, under the unchanged answer bound.
why: Ratified; the largest whole-screen answer on the stand uses 2.1 % of the bound.
@row_add n=524 row=contract/session-wire/graph-answer area=session-lifecycle rec=fct-2k7iskpv@2
boundary: Session wire
item: graph answer
promise: Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
@row_add n=525 row=contract/session-state/directory area=session-lifecycle rec=fct-cneioaun@1
boundary: Session state directory
item: directory
promise: A directory the caller names, mode `0700`, holding `session.sock` alone while a session is up and absent after `stop` or the host's idle expiry.
@row_add n=526 row=contract/session-state/socket area=session-lifecycle rec=fct-cneioaun@1
boundary: Session state directory
item: session.sock
promise: The session's socket, mode `0600`.
@row_add n=527 row=contract/session-state/path-bound area=session-lifecycle rec=fct-cneioaun@1,fct-klxgwgcy@5
boundary: Session state directory
item: path bound
promise: A socket address leaves about a hundred bytes for the whole path: 94 bytes on Linux, 90 elsewhere, a longer one refused with that bound stated.
@row_add n=528 row=contract/escher-session-command-line/command-form area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: command form
promise: `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
@row_add n=529 row=contract/escher-session-command-line/snapshot area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: snapshot
promise: `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
@row_add n=530 row=contract/escher-session-command-line/screenshot area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: screenshot
promise: `--id` is optional; the accepted answer is one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`.
@row_add n=531 row=contract/escher-session-command-line/lint area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: lint
promise: `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
@row_add n=532 row=contract/escher-session-command-line/graph area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: graph
promise: `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
@row_add n=533 row=contract/escher-session-command-line/start area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: start
promise: `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
@row_add n=534 row=contract/escher-session-command-line/serve area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: serve
promise: The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
@row_add n=535 row=contract/escher-session-command-line/mcp area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: mcp
promise: The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
@row_add n=536 row=contract/escher-session-command-line/mcp-record-check area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: mcp --record
promise: `mcp --record` checks the recording directory before it serves; a refusal is status `3`, its fixed message on stderr and stdout empty.
@row_add n=537 row=contract/escher-session-command-line/record-flag area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: --record
promise: `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
@row_add n=538 row=contract/escher-session-command-line/help area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: help
promise: `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
@row_add n=539 row=contract/escher-session-command-line/coverage area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: coverage
promise: The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
@row_add n=540 row=contract/escher-session-command-line/coverage-refusal area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: coverage refusal
promise: A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
@row_add n=541 row=contract/escher-session-command-line/two-argument-form area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: two-argument form
promise: The two-argument form `<task> <state-dir>` reads as an unknown verb, refused, status `1`.
@row_add n=542 row=contract/escher-session-command-line/usage-line area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: usage line
promise: The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
@row_add n=543 row=contract/escher-session-command-line/no-echo area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session command line
item: caller input
promise: No ending writes back anything the caller typed.
@row_add n=544 row=contract/escher-session-error-object/fields area=session-lifecycle rec=fct-76fzwvj7@5
boundary: escher-session error object
item: fields
promise: A session error object carries `kind`, `message` and `remedy`.
@row_add n=545 row=decision/session-checks-own-their-host area=session-lifecycle rec=fct-nqlygan4@1
name: Session checks own their host
rule: A check that needs a session process starts and stops its own host, bounds every wait, holds the host behind a guard that kills and reaps it, and sends the child's streams to null unless it reads them.
@row_add n=546 row=decision/socket-session-checks-unix-gated area=session-lifecycle rec=fct-nqlygan4@1
name: Socket session checks unix-gated
rule: A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it.
except: The `mcp_server` command-line comparison is unix-gated.
@row_add n=547 row=decision/recording-checks-unix-gated area=session-lifecycle rec=fct-nqlygan4@1
name: Recording checks unix-gated
rule: A check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included, beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs.
@row_add n=548 row=decision/log-silence-read-on-a-sink-host area=session-lifecycle rec=fct-nqlygan4@1
name: Log silence read on a host with a sink
rule: A zero read from a host that installs no log sink says nothing about a host that does.
@row_add n=549 row=decision/no-screenshot-in-cross-os-checks area=session-lifecycle rec=fct-egnkfdyv@6
name: No screenshot step in cross-OS byte-compared checks
rule: A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
guard: stand_session_recording
@row_add n=550 row=decision/two-clicks-without-advance-double-click area=session-lifecycle rec=fct-egnkfdyv@6
name: Two clicks with no advance are a double click
rule: Under the virtual clock, two clicks with no `advance` between them are a double click.
why: Ratified.
guard: stand_virtual_clock
@row_none n=280 rec=fct-2m7jhiee@1 type=S
@row_add n=551 row=decision/ci-scripts-unit-tested area=1-test-scope-summary rec=fct-krapstim@1,fct-jal6nbq2@1
name: CI Python scripts unit-tested
rule: The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them.
guard: ci-scripts leg
@row_add n=552 row=decision/ci-package-installs-bounded area=1-test-scope-summary rec=fct-krapstim@1,fct-lglsrvq2@1
name: CI package installs bounded
rule: Every package install of the CI workflow is bounded, a workflow invariant held by the CI scripts' unit tests.
guard: test_ci_workflows.py
@row_add n=553 row=decision/capability-statement-pinned area=1-test-scope-summary rec=fct-krapstim@1,fct-pf335kvt@1
name: Capability statement pinned to the driver schema
rule: The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools.
guard: test_capability_statement.py
@row_add n=554 row=decision/expiry-wait-watches-state-dir area=2-test-strategy rec=fct-jxt7cq26@1,fct-jqnoqslu@1
name: Expiry waits watch the state directory
rule: A check that waits for a host's expiry watches the state directory rather than asking the host.
why: An attach, and so the status command, is an answered request and starts the expiry count again.
@row_add n=555 row=decision/recorded-checks-replayed-as-tests area=2-test-strategy rec=fct-obwdwo5i@1,fct-5d4ttzku@2
name: Recorded checks replayed as tests
rule: A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.
@row_add n=556 row=decision/recorded-checks-kept-fresh area=2-test-strategy rec=fct-obwdwo5i@1,fct-5d4ttzku@2
name: Recorded checks kept fresh
rule: A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records.
guard: seven_guis recording
@row_add n=557 row=contract/driver-refusals/fonts-unavailable area=2-test-strategy rec=fct-bz62prox@1,fct-2vwamopj@2
boundary: Driver refusals
item: fonts-unavailable
promise: A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path, and the actions read names the same fonts-unavailable for screenshot.
@row_add n=558 row=contract/stand-boot/boot-timer area=3-test-harness-contract rec=fct-65u33zgx@1
boundary: seven_guis headless stand
item: boot_timer
promise: boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
@row_add n=559 row=contract/harness-screenshot/screenshot-rect area=3-test-harness-contract rec=fct-33n62qwf@1
boundary: Harness screenshot
item: screenshot_rect
promise: screenshot_rect(PixelRect) paints like screenshot at the harness's own time, the rectangle clipped to the viewport; an empty rectangle is EmptyRect.
@row_add n=560 row=decision/one-contrast-formula area=3-test-harness-contract rec=fct-nas4n737@1,fct-yivrglnx@1
name: One contrast formula
rule: Session lint's contrast reads the same as the harness's text_contrast, with no second formula.
@row_add n=561 row=decision/telemetry-target-allowlist area=3-test-harness-contract rec=fct-xxpbx2ay@2,fct-z2ynupl3@6
name: Telemetry target allowlist
rule: A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names.
guard: telemetry_scrub, telemetry_drop
@row_add n=562 row=contract/agent-run-exit-codes/1 area=3-test-harness-contract rec=fct-ad7eoe6y@1,fct-iokpku7j@1
boundary: agent-run exit codes
item: 1
promise: The verb ran and failed: a cargo build failure (a failing boot, leaving status not-ready), a failing run, an empty run.
@row_add n=563 row=contract/agent-run-exit-codes/2 area=3-test-harness-contract rec=fct-ad7eoe6y@1,fct-iokpku7j@1
boundary: agent-run exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, an unknown or pattern-failing selection; a usage error is checked before not-booted.
@row_add n=564 row=contract/agent-run-exit-codes/3 area=3-test-harness-contract rec=fct-ad7eoe6y@1,fct-iokpku7j@1
boundary: agent-run exit codes
item: 3
promise: A precondition is unmet: not booted (status, run and logs before boot), or logs with no events file.
@row_add n=565 row=contract/cold-agent-exit-codes/2 area=3-test-harness-contract rec=fct-rcsbr6d6@1,fct-3ferhyj7@3
boundary: cold-agent exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, or a task outside the allowlist (`run counter` included); checked before any precondition.
@row_add n=566 row=contract/cold-agent-exit-codes/3 area=3-test-harness-contract rec=fct-rcsbr6d6@1,fct-3ferhyj7@3
boundary: cold-agent exit codes
item: 3
promise: A precondition is unmet, before any state: `claude` or `python3` not on PATH, the `escher-session` binary absent or not executable (one stderr line naming the build command; the pipe builds nothing), `status` with no verdict, or `logs` with no `events.jsonl`.
@row_add n=567 row=decision/cold-agent-wrong-call-bar area=3-test-harness-contract rec=fct-szqqqnrt@3,fct-3ferhyj7@3
name: Cold-agent wrong-call bar of 5 %
rule: A cold-agent run fails `wrong-calls-over-bar` when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold.
guard: ColdAgentPipeTest
@row_add n=568 row=decision/ci-read-only-permissions area=4-unit-test-strategy rec=fct-lglsrvq2@1,fct-vg67igpb@1
name: CI read-only permissions
rule: The CI workflow grants `contents: read` at workflow level and no job adds a grant.
guard: CiWorkflowTest in test_ci_workflows.py
@row_add n=569 row=decision/ci-legs-through-leg-script area=4-unit-test-strategy rec=fct-lglsrvq2@1,fct-vg67igpb@1
name: CI legs run through the leg script
rule: Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite.
guard: CiWorkflowTest in test_ci_workflows.py
@row_add n=570 row=decision/cli-checks-through-own-binary area=6-e2e-test-strategy rec=fct-3c5ihsc6@1
name: Command-line checks through the package's own binary
rule: Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
@row_add n=571 row=decision/wpt-pinned-commit area=7-test-data-fixtures rec=fct-gazseuex@1,fct-cl3n5ve2@1
name: WPT pinned to a recorded commit
rule: WPT tests are cloned and the conformance suites run at the commit the WPT_COMMIT file pins.
@row_add n=572 row=decision/stand-boots-offline area=7-test-data-fixtures rec=fct-552qmxvp@1,fct-hsnu3a5x@1,fct-eeplrkyg@1,fct-cyo4xvcu@1
name: Stand boots offline
rule: Every headless stand boot keeps no network provider, and no real network is used by the stand.
guard: the_stand_options_stay_offline
except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.
@row_add n=573 row=decision/font-metric-tests-never-skip area=8-mocking-stubbing-discipline rec=fct-3ydth2ct@1,fct-ywhfajcc@1,fct-ya5o4ovd@1
name: Font-metric tests never skip
rule: A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping.
why: The skip came from the fixture's no-namespace element, not from missing system fonts.
guard: hover_state_tests
@row_add n=574 row=decision/ci-legs-host-reproducible area=9-ci-integration rec=fct-nymiowfp@1,fct-mrjxzany@4
name: Host-reproducible CI legs
rule: Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
@row_add n=575 row=decision/cargo-legs-locked area=9-ci-integration rec=fct-nymiowfp@1,fct-mrjxzany@4
name: Cargo legs run locked
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello, which has no Cargo.lock
@row_add n=576 row=decision/per-crate-coverage-floors area=10-quality-gates-coverage-targets rec=fct-qpxjumbg@1,fct-4t3fuckc@1,fct-na5rpohe@1,fct-fx4qekph@4
name: Per-crate line coverage floors
rule: The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold.
why: Ratified as three crates, each floor one point below.
guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate
except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.
@row_add n=577 row=decision/embolden-local-witness area=ci-integration rec=fct-as7aeq72@1,fct-gemhwfdl@1,fct-jym22btw@1
name: Local witness for font-embolden paint
rule: A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job.
why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux.
guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted
@row_add n=578 row=decision/package-installs-bounded-and-retried area=ci-integration rec=fct-uj7cocux@1
name: Bounded, retried package installs
rule: Every CI package install is bounded and retried: up to 3 attempts, each apt-get update then install under a 120 s bound with a kill grace, a bounded dpkg repair before each retry.
why: Answers a CI run whose package-install step hung twice.
guard: test_o_package_installs_are_bounded; InstallScriptTest
except: The matrix job's third-party Install apt deps action step carries timeout-minutes: 10 instead.
@row_add n=579 row=contract/apt-install-exit-codes/1 area=ci-integration rec=fct-uj7cocux@1,fct-vg67igpb@1
boundary: apt-install script exit codes
item: 1
promise: Every attempt failed, after three attempts; never the bound's 124 or 137.
@row_add n=580 row=contract/apt-install-exit-codes/2 area=ci-integration rec=fct-uj7cocux@1,fct-vg67igpb@1
boundary: apt-install script exit codes
item: 2
promise: A usage error or bad call — no package, a name outside the closed form, an unknown or malformed option, zero attempts or a zero bound — exits 2 before anything runs.
@row_add n=581 row=contract/cli-exit-codes/3 area=e2e-test-strategy rec=fct-csryo2zo@5,fct-2tbofxbl@1
boundary: escher-session CLI exit codes
item: 3
promise: A session error: a command that finds no session, a state directory too long, or a group-open recording directory.
@row_add n=582 row=contract/session-errors/dead area=integration-test-strategy rec=fct-2tbofxbl@1,fct-6jnjrehj@1
boundary: Session errors
item: Dead
promise: A socket file that nothing answers on, as after the session host is killed.
@row_add n=583 row=contract/session-errors/unsupported area=integration-test-strategy rec=fct-2tbofxbl@1,fct-6jnjrehj@1
boundary: Session errors
item: Unsupported
promise: Returned by the session arms on a non-unix target.
@row_add n=584 row=contract/session-run/graph area=integration-test-strategy rec=fct-csattoyz@1,fct-t7rg232i@1
boundary: escher_driver Session
item: graph
promise: The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
@row_add n=585 row=contract/session-host-stderr/service-name area=integration-test-strategy rec=fct-adkwyvky@5,fct-76fzwvj7@5
boundary: session host stderr lines
item: service.name
promise: Every line the escher-session host writes to stderr carries `service.name=seven_guis`.
@row_add n=586 row=decision/advance-moves-clock-by-exactly-ms area=mocking-stubbing-discipline rec=fct-7t2tqfsg@1
name: Advance moves the virtual clock by exactly ms
rule: The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained.
why: ratified
guard: stand_act_timer
@row_add n=587 row=contract/escher-session-exit-status/0 area=session-lifecycle rec=fct-76fzwvj7@5,fct-slbj25yc@1
boundary: escher-session exit status
item: 0
promise: Accepted: the answer as one uncoloured line of JSON on stdout.
@row_add n=588 row=contract/escher-session-exit-status/1 area=session-lifecycle rec=fct-76fzwvj7@5,fct-slbj25yc@1
boundary: escher-session exit status
item: 1
promise: Refused: the refusal on stdout.
@row_add n=589 row=contract/escher-session-exit-status/2 area=session-lifecycle rec=fct-76fzwvj7@5,fct-slbj25yc@1
boundary: escher-session exit status
item: 2
promise: Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
@row_add n=590 row=contract/escher-session-exit-status/3 area=session-lifecycle rec=fct-76fzwvj7@5,fct-slbj25yc@1
boundary: escher-session exit status
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
