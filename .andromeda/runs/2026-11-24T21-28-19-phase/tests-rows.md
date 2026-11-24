# test-plan · the rows that bind 2026-11-24-canvas-lint — picked 141 of 653 (67 by every draw) · rule any · pin 3685

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states in words the schema's verb, cause, session-error, node-key and exit-status counts and the a11y and stand-repeat legs' counts; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script. · guard: CapabilityStatementTest
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/no-screenshot-step-in-committed-checks · No screenshot step in a committed check — A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only. · why: a shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64, so such a shot is byte-identical on one host only · guard: stand_screenshot_repeat
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/driver-refusal-causes/fonts-unavailable · driver refusal causes · fonts-unavailable — A screenshot, lint or canvas on an app with no usable font is refused fonts-unavailable with nothing changed.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/mcp-server/help · MCP server · help — help is answered before any start and reads the same bytes through MCP and through the command line.
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cross-os-checks-hold-no-shot · Cross-OS byte comparisons hold no shot — A check compared byte for byte across OSes holds no shot. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · except: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/recorded-checks-kept-fresh · Recorded checks kept fresh — A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records. · guard: seven_guis recording
decision/recorded-checks-replayed-as-tests · Recorded checks replayed as tests — A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/contrast-reader-reads-only · Contrast reader changes nothing — The harness's contrast readers, of an element's text and of an open list's highlighted row, read and change nothing: no mutation, input, pump or settle.
decision/count-and-rename-sweeps · Verb-count and test-rename sweeps — A change of the driver's verb count or of a CI leg's file count sweeps the tests, examples and CI-script tests for pins of that count held as a literal, and a test rename sweeps the old name held as a string literal, such as a requirement's witness. · why: A word sweep misses a numeric pin and a test name held as a string. · except: A test name holding a count word that another check pins as a witness string is kept by a count sweep, the name searched before any rename.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The seven served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/no-shot-in-cross-os-byte-checks · No shot in cross-OS byte comparisons — A check compared byte for byte across OSes holds no shot. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · except: A shot of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red. · guard: stand_requirement_sweep · except: A requirement with no screen names no witness.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/wcag-2-contrast-ratio · WCAG 2.x contrast ratio — The contrast ratio is the WCAG 2.x ratio over the color crate's relative luminance.
contract/record-checks-script/arguments · The committed checks' recording script · arguments — Takes two arguments, the binary and an existing output directory.
decision/app-widget-shows-in-shots · App widget painted from Rust shows in shots — A widget an app paints from Rust shows in a harness shot and in a driver shot, whole and cropped to its element, in both layout modes, under each per-package test selection and in the workspace build. · guard: stand_canvas_shot
decision/canvas-commands-table-across-hosts · Canvas commands held to one table across hosts — Circle Drawer's canvas draw commands are read in process in both layout modes into one committed table, at boot, after a click and at 12 and 100 circles, as summary, full dump and parts, each answer held equal by size and digest on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_canvas_commands
decision/canvas-shots-cross-os-by-digest · Canvas shots compared across hosts by digest — Canvas shots are compared across hosts by one stand check: three canvas-alone fixtures shot whole and as their element, and Circle Drawer's canvas shot as its element, in both layout modes, their answer digests held byte-equal to committed tables, no pixels committed. · why: Ratified as byte equality required, not a tolerance; held on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_canvas_cross_os
decision/canvas-text-on-whole-device-pixels · Canvas text on whole device pixels across hosts — Text an app paints on a canvas that a cross-host shot comparison holds is drawn with its glyph origins on whole device pixels, so Circle Drawer's canvas with its diameter label reads one answer digest on linux, windows and macOS in both layout modes. · why: The CPU renderer snaps glyph origins differently on Apple hosts, so canvas text off whole device pixels differs in bytes across hosts. · guard: stand_canvas_cross_os
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/headless-screenshot · Headless screenshot from the harness — The harness paints a shot with no display through the CPU image renderer, composited over opaque white, at the harness's own time with no tick or pump.
decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
contract/harness-settle/notsettled · The harness's settle · NotSettled — Returned when work is still due after settling, naming the Busy class that holds it; otherwise Settled.
contract/text-contrast/no-reading · The harness's text contrast reading · no reading — No contrast is read where the element bears no text, or where an element on the walk has opacity below 1, a filter, a background image or a mix-blend-mode other than normal.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/one-contrast-formula · One contrast formula — Session lint's contrast reads through the harness's contrast readers, with no second formula.
decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/harness-contrast/required-ratio · Harness contrast · required_ratio — TextContrast's required_ratio is 3.0 for large text, else 4.5.
contract/stand-boot/boot-circle-drawer · seven_guis headless stand · stand::boot_circle_drawer — stand::boot_circle_drawer(HarnessOptions) boots Circle Drawer in process alone, served by no escher-session role.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
decision/box-presence-read-with-display · Box presence read with computed display — A check asking whether a node has a box reads the node's computed display beside its client rect. · why: A node a restyle turns display none keeps the client rect it last laid out at.
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: A helper whose body or failure message differs from the shared one stays in its check; stand_element_ids keeps its own boot; task_keys, ceiling, component, the cards array and SECRET stay per file.
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/agent-run-verbs/run · agent-run verbs · run — run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
contract/harness-contrast/blend-mode · Harness contrast · mix-blend-mode — text_contrast reads None past an element on its walk whose mix-blend-mode is other than normal.
contract/harness-contrast/text-contrast · Harness contrast · text_contrast — text_contrast(NodeId) -> Option<TextContrast> reads computed styles only, opens no span and prints or logs nothing.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/harness-screenshot/screenshot · Harness screenshot · screenshot — screenshot(&mut self) -> Result<Raster, ScreenshotError> resolves at the harness's own time with no tick and no pump, paints the viewport at its scale and device size and composites over opaque white.
contract/harness-settle/failed-load · Harness settle · failed load — A failed stylesheet load reaches the document and stops counting as a pending render-blocking resource, so settle does not read NotSettled { busy: Loads } for it.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states the schema's counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/contrast-required-ratio · Required contrast ratio — Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
contract/lint-defects/clipped · Lint defects · clipped — `clipped` is the second of the six lint defects, in their stated order.
contract/lint-defects/contrast · Lint defects · contrast — `contrast` is the sixth of the six lint defects, in their stated order.
contract/lint-defects/off-screen · Lint defects · off-screen — `off-screen` is the fourth of the six lint defects, in their stated order.
contract/lint-defects/overlap · Lint defects · overlap — `overlap` is the first of the six lint defects, in their stated order.
contract/lint-defects/truncated · Lint defects · truncated — `truncated` is the third of the six lint defects, in their stated order.
contract/lint-defects/zero-size · Lint defects · zero-size — `zero-size` is the fifth of the six lint defects, in their stated order.
contract/lint-findings/order · Lint findings · order — Lint findings come in document order, then in the stated order of the defects.
decision/harness-graph-tests-enable-accessibility · Harness graph unit tests turn accessibility on — A harness unit test of the UI graph turns the accessibility feature on through its dev-dependency, for the test build only. · why: A per-package test build leaves the accessibility feature off, and the UI graph is gated by it.
contract/driver-json/number · Driver JSON answers · number — A number is written as its shortest decimal, and as `null` when not finite.
contract/driver-verbs/verb-table · Driver verbs · verb table — Sixteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/default-level · the driver's log line · default level — At the default level the driver writes no line.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/password · driver session · typed password — A password typed through the driver never leaves in an answer or as pixels.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver and a file input's path occur nowhere in what a call returns, graph, actions, layout and canvas answers included, nor in a kept or an explored path's check, in both layout modes; the value reads the mask, and a typed password never leaves as pixels. · guard: stand_act_screenshot, stand_act_filled, stand_masks_on_new_answers
contract/refusal-cause/fonts-unavailable · driver refusal causes · fonts-unavailable — A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper. · why: Un-escaping quotes alone leaves a Windows path's backslashes doubled. · guard: stand_act_errors
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/stated-tolerance-read-inclusively · Stated tolerance read inclusively — A tolerance stated as to 1/64 px is read inclusively: the absolute difference of the two values is at most 1/64. · why: A sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/checks-hold-no-screenshot-step · No screenshot step in committed checks — The scripts behind the committed checks hold no screenshot step. · why: a shot holding document text differs in its PNG bytes by OS
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems. · why: a shot holding document text differs in its PNG bytes by OS · guard: the_committed_checks_are_what_the_binary_records
contract/cli-help/output · escher-session CLI help · help — `help` answers status 0 and one line equal to `describe()`, reaching no session.
contract/mcp-help/parity · escher-session MCP role · help — `help` is answered before any `start`, the same bytes through MCP and the command line.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.
decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/stand-load-page-fixed-answer · Load page stand-in answers after a fixed delay — The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider but the error-text check's failing page, whose stand-in fails its fetch. · why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/hosted-runner-cancel-hand-rerun · Hosted-runner cancel answered by a hand re-run — A CI job cancelled with no runner and no step run is a limit of the hosted runners, answered by a hand re-run of the cancelled job; no automatic re-run is built. · why: An automatic re-run needs a write-scoped CI token, and the CI token stays read-only.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/ci-trigger-branches · CI trigger branches — Tests run on pull requests and on pushes to main, v0.* and build/** branches, escher's build branch included.
decision/doc-leg-gates-public-doc-edits · Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block. · why: The fast leg runs no rustdoc, so a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-log-ansi-strip · ANSI strip for CI job-log tallies — A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[. · why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
decision/tally-move-attributed-by-test-name · Tally moves attributed by test name — A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line. · why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · guard: stand_session_recording · except: A `screenshot` of a canvas alone, its PNG bytes held equal on linux x86_64, windows x86_64 and macOS aarch64, is compared across OSes by its answer digest.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings, layout answers and canvas answers out. · why: Ratified, where until then none of them crossed.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph`, `layout` and `canvas` run no step and wait on nothing.
contract/escher-session-command-line/canvas · escher-session command line · canvas — `canvas --id <id> [--full] [--part <part>]` takes `--part` as a whole number 1 to 1000 and `--full` once, anything else refused `malformed`, status `1`; an accepted answer is one line, status `0`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/screenshot-in-band · Screenshot pixels in band — A screenshot's pixels cross in band, a PNG as padded base64 inside the answer's one JSON line `{"width":…,"height":…,"png":…}`, under the unchanged answer bound. · why: Ratified; the largest whole-screen answer on the stand uses 2.1 % of the bound.
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/screenshot · Driver session · screenshot — Returns a PNG of the whole screen, or of an element's bounds cropped to the visible part and rounded outward, changing nothing; a crop of no pixel is `off-screen`, an app with no font to paint text `fonts-unavailable`.
contract/driver-session/stale-or-not-found · Driver session · stale · not-found — An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
contract/escher-session-command-line/help · escher-session command line · help — `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
contract/escher-session-command-line/lint · escher-session command line · lint — `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
contract/escher-session-command-line/screenshot · escher-session command line · screenshot — `--id` is optional; the accepted answer is one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/session-recording/check-file · Session recording · check file — `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).

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
