@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=5
@row_none n=8 rec=8 type=5
@row_none n=9 rec=9 type=5
@row_none n=10 rec=10 type=5
@row_none n=11 rec=11 type=5
@row_none n=12 rec=12 type=5
@row_none n=13 rec=13 type=5
@row_none n=14 rec=14 type=5
@row_none n=15 rec=15 type=4
@row_none n=16 rec=16 type=9
@row_none n=17 rec=17 type=5
@row_none n=18 rec=18 type=5
@row_add n=1 row=decision/stand-boots-offline area=test-scope-summary rec=19
name: Stand boots stay offline
rule: Every stand boot keeps no network provider, the load page's boot alone excepted.
guard: the_stand_options_stay_offline
except: The load page's boot, which carries a network provider.
@row_none n=19 rec=20 type=4
relied: The lint's overlap reading skips any entry inside a select's box, closed or open, so the stand's lint reads no overlap on select options and open-list rows.
@row_none n=20 rec=21 type=5
@row_none n=21 rec=22 type=5
@row_none n=22 rec=23 type=5
@row_add n=2 row=decision/harness-driven-tests area=test-strategy rec=24
name: Harness-driven engine tests
rule: Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
@row_add n=3 row=contract/harness-settle/notsettled area=test-strategy rec=24
boundary: The harness's settle
item: NotSettled
promise: Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
@row_add n=4 row=decision/headless-screenshot area=test-strategy rec=24
name: Headless screenshot from the harness
rule: The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
@row_add n=5 row=decision/contrast-reader-reads-only area=test-strategy rec=24
name: Contrast reader changes nothing
rule: The harness's contrast reader reads and changes nothing: no mutation, input, pump or settle.
@row_add n=6 row=contract/text-contrast/no-reading area=test-strategy rec=24
boundary: The harness's text contrast reading
item: no reading
promise: No contrast is read where the element bears no text, or where an element on the walk has opacity below 1, a filter, a background image or a mix-blend-mode other than normal.
@row_add n=7 row=decision/one-contrast-formula area=test-strategy rec=24
name: One contrast formula
rule: The session lint's contrast reads the same as the harness's contrast reader, with no second formula.
@row_add n=8 row=decision/wcag-2-contrast-ratio area=test-strategy rec=24
name: WCAG 2.x contrast ratio
rule: The contrast ratio is the WCAG 2.x ratio over the color crate's relative luminance.
@row_add n=9 row=decision/stable-tree-serialization area=test-strategy rec=24
name: Stable tree serialization
rule: The harness produces a stable one-node-per-line tree serialization with geometry.
why: Suitable for snapshot-style assertions.
@row_add n=10 row=decision/check-owns-its-host-process area=test-strategy rec=25
name: A check owns its host process
rule: A check that needs a session process starts and stops its own host, every wait bounded, with a guard that kills and reaps a host a failing check still holds and its state directory removed at stop.
@row_add n=11 row=decision/child-streams-to-null area=test-strategy rec=25
name: Child streams sent to null
rule: A host child's streams are sent to null unless the check reads them.
why: So the child's own libtest lines never reach the parent's output, where the agent run script would count them as tests.
@row_add n=12 row=decision/socket-checks-unix-gated area=test-strategy rec=25
name: Socket session checks are unix-gated
rule: A check holding a session over the socket is gated to unix.
except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
@row_add n=13 row=decision/expiry-wait-watches-state-dir area=test-strategy rec=25
name: Expiry waits watch the state directory
rule: A check that waits for a host's idle expiry watches the state directory rather than asking the host.
why: An attach, and so the status command, is an answered request and starts the expiry count again.
@row_add n=14 row=decision/started-host-held-behind-guard area=test-strategy rec=25
name: Started hosts held behind a guard
rule: The seven_guis checks hold a session spawned by the binary's start command behind a guard that stops it.
why: The start command closes the host's streams and neither kills nor reaps it.
@row_add n=15 row=decision/drain-host-pipes-while-running area=test-strategy rec=25
name: Host pipes drained while the host runs
rule: A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading.
why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
@row_add n=16 row=decision/mcp-capture-counts-by-span area=test-strategy rec=25
name: MCP child capture counted by span
rule: A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
@row_add n=17 row=decision/cli-checks-run-the-binary area=test-strategy rec=26
name: Command-line checks run the binary
rule: The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
@row_add n=18 row=contract/escher-session-exit-status/0 area=test-strategy rec=26
boundary: escher-session exit status
item: 0
promise: The command was accepted.
@row_add n=19 row=contract/escher-session-exit-status/1 area=test-strategy rec=26
boundary: escher-session exit status
item: 1
promise: The command was refused.
@row_add n=20 row=contract/escher-session-exit-status/2 area=test-strategy rec=26
boundary: escher-session exit status
item: 2
promise: A usage error.
@row_add n=21 row=contract/escher-session-exit-status/3 area=test-strategy rec=26
boundary: escher-session exit status
item: 3
promise: A session error.
@row_add n=22 row=decision/shell-flows-are-cargo-test-assets area=test-strategy rec=26
name: Shell flows are cargo test assets
rule: POSIX sh flow scripts run under plain sh from a Rust cargo test target; the scripts are assets of that target, not a runner of their own, and no test framework is added.
@row_add n=23 row=decision/cli-and-flow-targets-unix-gated area=test-strategy rec=26
name: Command-line and flow targets unix-gated
rule: The command-line check target, the shell flow target and the recording target are gated to unix.
@row_add n=24 row=contract/record-checks-script/arguments area=test-strategy rec=26
boundary: The committed checks' recording script
item: arguments
promise: Takes two arguments, the binary and an existing output directory.
@row_add n=25 row=contract/record-checks-script/rec-directory area=test-strategy rec=26
boundary: The committed checks' recording script
item: rec directory
promise: Makes its rec directory with mode 0700.
@row_add n=26 row=contract/record-checks-script/failure-output area=test-strategy rec=26
boundary: The committed checks' recording script
item: failure output
promise: Prints only step N on failure.
@row_add n=27 row=decision/recorded-checks-replayed-as-tests area=test-strategy rec=27
name: Recorded checks replayed as tests
rule: A recorded check is committed as data, read strictly and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs, answers compared as JSON values.
@row_add n=28 row=decision/recorded-checks-kept-fresh area=test-strategy rec=27
name: Recorded checks kept fresh
rule: The committed checks are kept fresh by a byte-for-byte comparison with what the binary records.
guard: seven_guis recording
@row_add n=29 row=decision/handler-coverage-by-replay area=test-strategy rec=27
name: Handler coverage measured by replay
rule: Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
@row_add n=30 row=contract/escher-session-commands/coverage area=test-strategy rec=27
boundary: escher-session commands
item: coverage <app> <check>...
promise: Lists the handlers no given recorded check exercised.
@row_add n=31 row=decision/capability-claims-name-their-checks area=test-strategy rec=28
name: Capability claims name their checks
rule: The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
@row_add n=32 row=decision/capability-check-names-held-to-schema area=test-strategy rec=28
name: Claim section names and counts held to the schema
rule: Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand.
guard: test_capability_statement.py
@row_add n=33 row=contract/handler-coverage-limits/canvas-handler area=test-strategy rec=28
boundary: Handler coverage limits
item: canvas handler
promise: A handler inside a canvas is not counted.
@row_add n=34 row=contract/handler-coverage-limits/unreached-screen area=test-strategy rec=28
boundary: Handler coverage limits
item: unreached screen
promise: A handler on no screen a replay reached is not listed.
@row_add n=35 row=contract/handler-coverage-limits/bubbling-dispatch area=test-strategy rec=28
boundary: Handler coverage limits
item: bubbling dispatch
promise: A bubbling dispatch counts every ancestor listener it passed.
@row_add n=36 row=decision/pixel-tests-cpu-buffer area=test-strategy rec=29
name: Pixel tests on a CPU buffer
rule: Pixel tests render to a CPU buffer and sample pixels, box-shadow expectations taken from Chromium renders with a 1px antialiasing tolerance.
@row_add n=37 row=decision/frames-compared-whole area=test-strategy rec=29
name: Frames compared whole
rule: A shot's PNG is compared whole, equal to the harness's raster and byte-identical across runs on one host.
guard: stand_act_screenshot, stand_screenshot_repeat
@row_add n=38 row=decision/no-shot-in-cross-os-byte-checks area=test-strategy rec=29
name: No shot in cross-OS byte comparisons
rule: A check compared byte for byte across OSes holds no shot.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
@row_add n=39 row=decision/png-decoding-in-checks area=test-strategy rec=29
name: PNG decoding in checks
rule: A blitz-tests check decodes a PNG through the png dev-dependency; seven_guis, which has none, reads it with a hand base64 reader and the PNG header.
@row_add n=40 row=decision/embolden-local-witness area=test-strategy rec=30
name: Font-embolden paint has a local witness
rule: A paint change under the OS-gated font-embolden feature has its local witness in the embolden step, the last step of the fast leg, with no CI job, reading on linux the numbers the macOS job reads.
guard: select_popup_painted
@row_add n=41 row=decision/stand-checks-bundled-font area=test-strategy rec=31
name: Stand checks under a bundled font
rule: The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off.
except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
@row_add n=42 row=decision/shot-refused-without-fonts area=test-strategy rec=31
name: Shot refused without a usable font
rule: A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path.
guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
@row_add n=43 row=contract/driver-refusals/fonts-unavailable area=test-strategy rec=31
boundary: Driver refusals
item: fonts-unavailable
promise: A screenshot of a document with no usable sans-serif font is refused with nothing painted, and the actions read names the same refusal for screenshot.
@row_add n=44 row=decision/wpt-pinned-commit area=test-strategy rec=32
name: WPT suite pinned to a commit
rule: The WPT conformance suites run at the commit the WPT_COMMIT file pins.
@row_add n=45 row=decision/wpt-conformance-report-only area=test-strategy rec=32
name: WPT conformance is report-only
rule: A WPT conformance reading is report-only and grades no criterion.
@row_add n=46 row=decision/inline-cfg-test-modules area=test-strategy rec=33,35
name: Unit tests in inline test modules
rule: Unit tests sit in cfg(test) modules inside their source files.
except: Eight cfg(test) modules held in files of their own through a path attribute at their parent.
@row_add n=47 row=decision/one-test-target-per-file area=test-strategy rec=33
name: One test target per file
rule: blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false.
why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target.
guard: BlitzTestsTargetsTest
@row_add n=48 row=decision/shared-test-modules-not-targets area=test-strategy rec=33
name: Shared test modules are no target
rule: Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
@row_none n=23 rec=34 type=5
@row_add n=49 row=decision/bridge-tests-child-module area=unit-test-strategy rec=35
name: Bridge tests a child of the dioxus document
rule: The dioxus document's bridge tests sit in a child module of the dioxus document module.
why: The event handler's two fields are private to that module and it has no constructor.
@row_add n=50 row=decision/test-module-split-keeps-paths area=unit-test-strategy rec=35
name: Split test modules keep their paths
rule: A unit-test module moved to a file of its own is moved out whole with its test paths unchanged.
@row_none n=24 rec=36 type=5
@row_none n=25 rec=37 type=5
@row_add n=51 row=contract/harness-settle/pass-limit area=unit-test-strategy rec=38
boundary: The harness's settle
item: pass limit
promise: Work for ever ends after exactly the settle pass limit of polls, reading Render.
@row_add n=52 row=contract/harness-settle/notsettled-message area=unit-test-strategy rec=38
boundary: The harness's settle
item: NotSettled message
promise: Every NotSettled message is non-empty, distinct and holds no slash.
@row_add n=53 row=decision/contrast-required-ratio area=unit-test-strategy rec=38
name: Required contrast ratio
rule: Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
