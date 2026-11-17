# test-plan · the rows that bind 2026-11-17-answer-writer-by-family — picked 180 of 626 (66 by every draw) · rule any · pin 1805

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
decision/no-screenshot-step-in-committed-checks · No screenshot step in a committed check — A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only. · why: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a shot is byte-identical on one host only · guard: stand_screenshot_repeat
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/typed-password-never-leaves · A typed password never leaves the instance — A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels. · guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
contract/mcp-server/actions-argument · MCP server · actions argument — An actions tool call given an argument is answered isError.
contract/mcp-server/help · MCP server · help — help is answered before any start and reads the same bytes through MCP and through the command line.
contract/mcp-server/screenshot · MCP server · screenshot — A screenshot tool call is answered with an image block then a text block.
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fifteen verbs in order, `layout` after `actions` and `help` last.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
decision/unrun-declared-write-reads-unconfirmed · Declared write unrun after its event reads unconfirmed — The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it. · why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
contract/driver-refusal-causes/fonts-unavailable · driver refusal causes · fonts-unavailable — A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.
contract/mcp-server/refused-input · MCP server · refused input — server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
contract/session-state-directory/length-bound · session state directory · length bound — A state directory path of 94 bytes holds a session on Linux and of 90 bytes on other targets; one byte more is refused state-dir-too-long with its bound.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/recorded-checks-replayed-as-tests · Recorded checks replayed as tests — A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.
decision/shell-flows-under-cargo-test · Shell flows run under cargo test — The POSIX sh flow scripts are assets of a cargo test target, not a runner of their own, and no test framework is added for them.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
contract/driver-refusals/fonts-unavailable · Driver refusals · fonts-unavailable — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path, and the actions read names the same fonts-unavailable for screenshot.
decision/cross-os-checks-hold-no-shot · Cross-OS byte comparisons hold no shot — A check compared byte for byte across OSes holds no shot. · why: a shot's PNG bytes differ between OSes
decision/inline-unit-test-modules · Inline unit test modules — Unit tests are Rust test functions in cfg(test) modules beside the code under test. · except: modules held in files of their own through a path attribute
decision/recorded-checks-kept-fresh · Recorded checks kept fresh — A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records. · guard: seven_guis recording
decision/seven-guis-cli-targets-unix-only · seven_guis command-line targets gated to unix — seven_guis' command-line and shell-flow test targets are gated to unix.
decision/socket-session-checks-unix-only · Socket session checks gated to unix — A check holding a session over the socket is gated to unix.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/shell-flows-are-cargo-test-assets · Shell flows are cargo test assets — POSIX sh flow scripts run under plain sh from a Rust cargo test target; the scripts are assets of that target, not a runner of their own, and no test framework is added.
decision/shot-refused-without-fonts · Shot refused without a usable font — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path. · guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
contract/escher-session-commands/coverage · escher-session commands · coverage <app> <check>... — Lists the handlers no given recorded check exercised.
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/cli-and-flow-targets-unix-gated · Command-line and flow targets unix-gated — The command-line check target and the shell flow target are gated to unix; the recording target is gated to unix item by item, beside an off-unix check that runs elsewhere.
decision/count-and-rename-sweeps · Verb-count and test-rename sweeps — A change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices, and a test rename sweeps the old name held as a string literal, such as a requirement's witness. · why: A word sweep misses a numeric pin and a test name held as a string.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/inline-cfg-test-modules · Unit tests in inline test modules — Unit tests sit in cfg(test) modules inside their source files. · except: cfg(test) modules held in files of their own through a path attribute at their parent.
decision/no-shot-in-cross-os-byte-checks · No shot in cross-OS byte comparisons — A check compared byte for byte across OSes holds no shot. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
decision/socket-checks-unix-gated · Socket session checks are unix-gated — A check holding a session over the socket is gated to unix. · except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
decision/started-host-held-behind-guard · Started hosts held behind a guard — The seven_guis checks hold a session spawned by the binary's start command behind a guard that stops it. · why: The start command closes the host's streams and neither kills nor reaps it.

## 3-test-harness-contract · ## 3. Test Harness Contract
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
contract/command-span/screenshot · command span · screenshot — A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/session-error-messages-fixed · Session error messages are fixed text — Every session error message is a non-empty fixed string holding no `/` and no exit code; the input-or-output arm adds only the kind's name and is spelled «input or output», since «I/O» holds a `/`.
decision/test-module-split-keeps-paths · Split test modules keep their paths — A unit-test module moved to a file of its own is moved out whole with its test paths unchanged.
contract/coverage-line/owner · The coverage line · owner — A handler's `owner` is left out when it has none.
contract/coverage-line/shape · The coverage line · shape — One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.
contract/driver-json/masked-value · Driver JSON answers · masked value — A masked node value is written as `MASKED_VALUE`.
contract/driver-json/number · Driver JSON answers · number — A number is written as its shortest decimal, and as `null` when not finite.
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/harness-graph-tests-enable-accessibility · Harness graph unit tests turn accessibility on — A harness unit test of the UI graph turns the accessibility feature on through its dev-dependency, for the test build only. · why: A per-package test build leaves the accessibility feature off, and the UI graph is gated by it.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fifteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
contract/lint-defects/clipped · Lint defects · clipped — `clipped` is the second of the six lint defects, in their stated order.
contract/lint-defects/contrast · Lint defects · contrast — `contrast` is the sixth of the six lint defects, in their stated order.
contract/lint-defects/off-screen · Lint defects · off-screen — `off-screen` is the fourth of the six lint defects, in their stated order.
contract/lint-defects/overlap · Lint defects · overlap — `overlap` is the first of the six lint defects, in their stated order.
contract/lint-defects/truncated · Lint defects · truncated — `truncated` is the third of the six lint defects, in their stated order.
contract/lint-defects/zero-size · Lint defects · zero-size — `zero-size` is the fifth of the six lint defects, in their stated order.
contract/lint-findings/order · Lint findings · order — Lint findings come in document order, then in the stated order of the defects.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
contract/refusal-causes/printed-form · Refusal causes · printed form — A refusal's printed form opens with its cause's name and ends with its remedy.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/driver-session/password · driver session · typed password — A password typed through the driver never leaves in an answer or as pixels.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
contract/session-host/non-unix · session host · non-unix arms — Off unix the session host's arms return `Unsupported`.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/handler-coverage-holds-no-typed-text · Handler coverage line holds no typed text — Text typed during a replayed check never reaches the handler coverage line. · guard: stand_handler_coverage
decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper. · why: Un-escaping quotes alone leaves a Windows path's backslashes doubled. · guard: stand_act_errors
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver occurs nowhere in what the call returns, reads `MASKED_VALUE`, and never leaves as pixels in a screenshot. · guard: stand_act_screenshot, stand_act_filled
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
contract/session-run/type · escher_driver Session · type — A `type` replaces what a filled control holds and an empty text clears it; a masked control's written form holds the mask, never the typed text.
decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
contract/actions-command/argument · the actions command · argument — An argument to actions is refused malformed.
contract/graph-command/argument · the graph command · argument — An argument to graph is refused malformed with nothing run.
contract/refusal-cause/covered · driver refusal causes · covered — A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
contract/refusal-cause/disabled · driver refusal causes · disabled — A call on a disabled target is refused `disabled`, the instance unchanged.
contract/refusal-cause/fonts-unavailable · driver refusal causes · fonts-unavailable — A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/refusal-cause/not-found · driver refusal causes · not-found — A call naming an id not on the screen is refused `not-found`, the instance unchanged.
contract/refusal-cause/off-screen · driver refusal causes · off-screen — A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
contract/refusal-cause/stale · driver refusal causes · stale — A call naming a stale id is refused `stale`, the instance unchanged.
contract/refusal-cause/unknown-verb · driver refusal causes · unknown-verb — A call naming no known verb is refused `unknown-verb`, the instance unchanged.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-errors/alreadyrunning · session errors · AlreadyRunning — Returned by start while a session is already up.
contract/session-errors/dead · Session errors · Dead — A socket file that nothing answers on, as after the session host is killed.
contract/session-errors/nosession · session errors · NoSession — Returned when a session is reached and none is running.
contract/session-errors/unsupported · Session errors · Unsupported — Returned by the session arms on a non-unix target.
contract/session-protocol/hello · the session protocol · hello — hello answers idle_expiry_s.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
contract/session-run/screenshot · escher_driver Session · screenshot — A `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster, and changes nothing.
contract/session-run/scroll · escher_driver Session · scroll — A `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view.
contract/session-run/type-on-select · escher_driver Session · type on a select — A `type` on a single-choice select chooses the first option whose label starts with the text, its diff naming the select with `value` and the options whose `selected` moved.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/checks-hold-no-screenshot-step · No screenshot step in committed checks — The scripts behind the committed checks hold no screenshot step. · why: a shot's PNG bytes differ by OS
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems. · why: a shot's PNG bytes differ by OS · guard: the_committed_checks_are_what_the_binary_records
contract/cli-actions/output · escher-session CLI actions verb · output — `actions` against a started host answers one JSON line of the actionable nodes.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/cli-help/output · escher-session CLI help · help — `help` answers status 0 and one line equal to `describe()`, reaching no session.
contract/cli-refusal/remedy · escher-session CLI refusals · remedy — Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
contract/cli-snapshot/budget · escher-session CLI snapshot · size budget — A snapshot answers the in-process text as one line inside its 10,000-byte budget.
contract/cli-snapshot/part · escher-session CLI snapshot · --part — A part read by number answers one uncoloured JSON line, status 0; a part of 0, 1001 or a non-number exits 1 with the `malformed` refusal line, nothing typed written back.
contract/cli-state-dir/max-length · escher-session CLI state directory · length bound — A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
contract/mcp-help/parity · escher-session MCP role · help — `help` is answered before any `start`, the same bytes through MCP and the command line.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
contract/mcp-snapshot/parity · escher-session MCP role · snapshot — A fresh Counter's snapshot reads the same through MCP and the command line.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
contract/mcp-tool-result/screenshot · escher-session MCP tool results · screenshot — A `screenshot` tool call answers an `image/png` image block then a text block, `isError` false.
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fifteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last.
contract/cli-actions/argument · escher-session CLI actions verb · argument — An argument to `actions` is refused, status 1.
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/doc-leg-gates-public-doc-edits · Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block. · why: The fast leg runs no rustdoc, so a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/ci-trigger-branches · CI trigger branches — Tests run on pull requests and on pushes to main, v0.* and build/** branches, escher's build branch included.
decision/embolden-leg-local-only · Embolden witness leg without a CI job — The font-embolden witness leg runs locally and has no CI job of its own. · why: macOS exercises the feature in CI
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-log-ansi-strip · ANSI strip for CI job-log tallies — A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[. · why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
decision/clippy-legs · Clippy over the workspace and upstream blitz-dom — The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.
decision/tally-move-attributed-by-test-name · Tally moves attributed by test name — A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line. · why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/screenshot-in-band · Screenshot pixels in band — A screenshot's pixels cross in band, a PNG as padded base64 inside the answer's one JSON line `{"width":…,"height":…,"png":…}`, under the unchanged answer bound. · why: Ratified; the largest whole-screen answer on the stand uses 2.1 % of the bound.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/not-settled-result · Driver session · settled — A verb on an instance that did not go quiet is a result, `settled` false with its `busy` class, and is not rolled back.
contract/driver-session/payload-unread · Driver session · <payload unread> — A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/escher-session-command-line/coverage · escher-session command line · coverage — The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
contract/escher-session-command-line/coverage-refusal · escher-session command line · coverage refusal — A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
contract/escher-session-command-line/help · escher-session command line · help — `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
contract/escher-session-command-line/lint · escher-session command line · lint — `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/screenshot · escher-session command line · screenshot — `--id` is optional; the accepted answer is one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`.
contract/escher-session-command-line/snapshot · escher-session command line · snapshot — `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
contract/escher-session-error-object/fields · escher-session error object · fields — A session error object carries `kind`, `message` and `remedy`.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused: the refusal on stdout.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
contract/session-errors/error-set · Session errors · error set — Fifteen session errors, each stating a kind, a fixed meaning and a remedy; the state directory's message states its bound.
contract/session-wire/accepted · Session wire · ok v3 accepted — `ok v3 accepted <json>` carries an accepted call's answer.
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
contract/session-wire/refused · Session wire · ok v3 refused — `ok v3 refused <json>` carries a refused call's refusal.

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
