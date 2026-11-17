# test-plan · the rows that bind 2026-11-17-stand-requirement-sweep — picked 175 of 626 (99 by every draw) · rule any · pin 1850

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/typed-password-never-leaves · A typed password never leaves the instance — A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels. · guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/service-name · escher-session stderr · service.name — Every stderr line is stamped service.name=seven_guis.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/buffered-render-on-dioxus-07-bridge · Buffered render on the Dioxus 0.7 bridge — The Dioxus 0.7 bridge renders into a mutation buffer with the document unborrowed, so a task polled during render can access the document. · why: Upstream's buffered render ported onto the Dioxus 0.7 bridge. · guard: dioxus_runtime_probe
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest
decision/chosen-select-row-colours · Chosen select row colour pair — A chosen select row is painted with the engine default #c8c8c8 on #000000. · why: The pair is Chromium's inactive selected pair, reading 12.56:1. · guard: a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/unrun-declared-write-reads-unconfirmed · Declared write unrun after its event reads unconfirmed — The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it. · why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
decision/upstream-buffered-render-test-kept-as-written · Upstream buffered-render test kept as written — Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7. · why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for. · guard: dioxus_runtime_probe

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/sink-host-output-drained · Output of a sink-installing host drained — A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/drain-host-pipes-while-running · Host pipes drained while the host runs — A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading. · why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/pinned-viewport-read-from-document · Pinned viewport read from the document — A boot-condition check of the pinned viewport reads the document viewport, never the root element's bounds. · why: A component alone has no shell CSS, so its root element sizes to its content.
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/count-and-rename-sweeps · Verb-count and test-rename sweeps — A change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices, and a test rename sweeps the old name held as a string literal, such as a requirement's witness. · why: A word sweep misses a numeric pin and a test name held as a string.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/component-alone-props-as-maker · Props of a component alone given as a maker — Props of a component alone are given as a maker run once in the mount's root scope, never as a value. · why: An EventHandler prop can only be made inside a Dioxus runtime.
decision/component-alone-root-context-before-build · Root context of a component alone provided before the build — A root context for a component alone is provided on the mount before the harness builds. · why: A context added after the build reaches no render.
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/scrolled-box-reads-where-it-stands · A scrolled box reads where it stands — For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do. · why: ratified · guard: scrolled_box_absolute_position
decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
contract/agent-run-selection/all · agent-run run selection · all — all runs the whole package, with no --test.
contract/agent-run-selection/session · agent-run run selection · session — session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm, so a blitz-tests file named session does not divert it, the argument count checked first (exit 2) and the boot next (exit 3).
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/agent-run-verbs/run · agent-run verbs · run — run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
contract/command-span/graph · command span · graph — A graph call's command line records verb alone under its tool_call parent.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/harness-construction/from-component-alone · Harness construction · Harness::from_component_alone — Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
contract/harness-construction/vdom-alone · Harness construction · vdom_alone — vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
contract/harness-input/scroll-into-view · Harness input helpers · scroll_into_view — scroll_into_view(node_id) scrolls at once and by the least movement on each axis, every scrolling box that holds the node and then the viewport; a row of an open select list moves the list's scroll_y alone.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/service-name · Stand log · service.name — Both seven_guis binaries log under service.name=seven_guis.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
contract/agent-run-events/run-start · agent-run events · run.start — run.start {ts, selection, files}, files the --test stems passed ([] under all), printed to stdout and appended to events.jsonl.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.
contract/tool-call-span/help · tool_call span · help — A help call, answered by the server, writes one tool_call line, tool="help" ending="accepted", with no command line beneath it.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/coverage-floor-per-crate · Coverage floor per crate — The coverage leg exits 0 when every crate is at its floor and exits 1 when one is under it. · guard: LegScriptTest
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
contract/driver-json/masked-value · Driver JSON answers · masked value — A masked node value is written as `MASKED_VALUE`.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
contract/driver-json/number · Driver JSON answers · number — A number is written as its shortest decimal, and as `null` when not finite.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/password · driver session · typed password — A password typed through the driver never leaves in an answer or as pixels.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/handler-coverage-holds-no-typed-text · Handler coverage line holds no typed text — Text typed during a replayed check never reaches the handler coverage line. · guard: stand_handler_coverage
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper. · why: Un-escaping quotes alone leaves a Windows path's backslashes doubled. · guard: stand_act_errors
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/sink-drops-third-party-targets · Sink drops third-party targets — The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it. · guard: telemetry_drop
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver occurs nowhere in what the call returns, reads `MASKED_VALUE`, and never leaves as pixels in a screenshot. · guard: stand_act_screenshot, stand_act_filled
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-host-stderr/service-name · session host stderr lines · service.name — Every line the escher-session host writes to stderr carries `service.name=seven_guis`.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
contract/session-run/type · escher_driver Session · type — A `type` replaces what a filled control holds and an empty text clears it; a masked control's written form holds the mask, never the typed text.
decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/stated-tolerance-read-inclusively · Stated tolerance read inclusively — A tolerance stated as to 1/64 px is read inclusively: the absolute difference of the two values is at most 1/64. · why: A sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/example-fixtures-are-manual · Example fixtures carry no automated assertion — Example fixture pages state their click-through instructions and expected outcomes in text instead of automated assertions.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/cli-actions/output · escher-session CLI actions verb · output — `actions` against a started host answers one JSON line of the actionable nodes.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/coverage-leg-exits-with-floor · Coverage leg gated by the floors — The coverage leg measures workspace line coverage and ends with the per-crate floor check, exiting with its status, so a missed floor fails the leg. · guard: coverage_floor.py
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/joining-file-keeps-its-legs · A file joining a leg keeps its legs — A test file joining a CI leg keeps every leg it already ran in.
contract/coverage-floor-exit-codes/0 · coverage floor script exit codes · 0 — Every crate named is at or above its floor; the verdict reads `coverage floor: N of N crates at or above their floors`.
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.
decision/ci-log-ansi-strip · ANSI strip for CI job-log tallies — A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[. · why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
decision/tally-move-attributed-by-test-name · Tally moves attributed by test name — A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line. · why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.
contract/coverage-floor-exit-codes/1 · coverage floor script exit codes · 1 — Some crates are under their floors; the verdict reads `coverage floor: K of N crates under their floors`.
contract/coverage-floor-exit-codes/2 · coverage floor script exit codes · 2 — A crate named has no coverage record, or an argument is malformed.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/keyboard-compat-excluded-from-coverage · keyboard_compat.rs excluded from coverage — Coverage measurement excludes dioxus-native-dom's keyboard_compat.rs. · why: It is upstream's temporary keyboard-types bridge, excluded until escher takes upstream's retirement of it.
decision/mutation-baseline-read-per-unit · Mutation baseline read per unit — A mutation run skips cargo-mutants' own baseline and first reads each unit's test command green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone.
decision/mutation-testing-dev-host-only · Mutation testing on the dev host — Mutation testing is cargo-mutants run with nextest under a fail-fast mutants profile, on the dev host alone and never in CI.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold. · why: Ratified as three crates, each floor one point below. · guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate · except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/keyboard-compat-coverage-exclusion · keyboard_compat.rs left out of coverage — Both coverage runs exclude dioxus-native-dom's crate-private keyboard_compat.rs until escher takes upstream's retirement of it; the floors are unchanged by the exclusion. · why: The file is upstream's temporary keyboard-types 0.7/0.8 bridge whose conversion arms no test of ours reaches. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
decision/mutation-baseline-skipped · Mutation baseline skipped, read green by hand — Mutation runs skip cargo-mutants' baseline; each unit's mutant test command is first read green on the unmutated tree. · why: cargo-mutants' baseline tests the mutated package alone yet passes every test argument, so a test set naming other packages cannot be its baseline and would read every mutant caught.
decision/mutation-runs-on-dev-host-only · Mutation runs on the dev host only — Mutation runs are made on the dev host alone, never in CI. · why: cargo-mutants and cargo-nextest are host tools no lockfile resolves.
decision/mutation-runs-under-nextest-profile · Mutation runs with cargo-mutants under nextest — A mutation run is cargo-mutants with nextest as its test tool under a dedicated mutants profile that stops at the first failure and runs the direct tests first, the default profile untouched. · why: Under plain cargo test, hung integration binaries run before the library tests and make mutants read timeout.
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated
decision/complexity-ceiling-host-only · Cognitive-complexity ceiling read on the host — The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted; no CI leg runs it, it binds no function, and engine functions over it are left as they are. · except: The engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/escher-session-command-line/mcp · escher-session command line · mcp — The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
contract/escher-session-command-line/serve · escher-session command line · serve — The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
contract/session-host/start · Session host · start — Spawns the host command and returns its first answer.
contract/session-recording/exploration-checks · session recording · exploration checks — With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a path whose accepted answer carries a panic item ends panicked with no payload, a re-reach step diverged, and its check holds the panicked step.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
decision/recording-checks-unix-gated · Recording checks unix-gated — A check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included, beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/socket-session-checks-unix-gated · Socket session checks unix-gated — A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it. · except: The `mcp_server` command-line comparison is unix-gated.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/payload-unread · Driver session · <payload unread> — A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/coverage · escher-session command line · coverage — The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
contract/escher-session-command-line/coverage-refusal · escher-session command line · coverage refusal — A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/record-flag · escher-session command line · --record — `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, with `graph` after `lint`, `layout --id <id>` after `actions` and `help` after `stop`.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/session-host/call · Session host · call — Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.

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
