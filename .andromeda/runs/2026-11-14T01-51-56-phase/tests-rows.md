# test-plan · the rows that bind 2026-11-14-seeded-randomness — picked 214 of 601 (142 by every draw) · rule any · pin 741

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/advance-moves-exactly-the-ms-asked · Advance moves exactly the time asked — An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms. · why: ratified · guard: stand_act_timer
decision/advance-reports-exact-ms · Hosted advance reports exactly the ms asked — A hosted advance moves the timer session with no sleep, and every ask reports exactly the milliseconds asked. · why: Ratified. · guard: host_timer
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
decision/no-screenshot-step-in-committed-checks · No screenshot step in a committed check — A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only. · why: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a shot is byte-identical on one host only · guard: stand_screenshot_repeat
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
contract/mcp-server/help · MCP server · help — help is answered before any start and reads the same bytes through MCP and through the command line.
contract/mcp-server/refused-input · MCP server · refused input — server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fourteen verbs in order.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/recorded-checks-kept-fresh · Recorded checks kept fresh — A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records. · guard: seven_guis recording
decision/recorded-checks-replayed-as-tests · Recorded checks replayed as tests — A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.
decision/sink-host-output-drained · Output of a sink-installing host drained — A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/seven-guis-cli-targets-unix-only · seven_guis command-line targets gated to unix — seven_guis' command-line and shell-flow test targets are gated to unix.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/drain-host-pipes-while-running · Host pipes drained while the host runs — A check that captures a host installing escher's sink drains its pipes on threads from the spawn on, or captures to a file, and never waits for exit before reading. · why: A sink that leaks again can write far more than a pipe holds, so a check that must fail red cannot wait for exit.
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/cli-and-flow-targets-unix-gated · Command-line and flow targets unix-gated — The command-line check target and the shell flow target are gated to unix; the recording target is gated to unix item by item, beside an off-unix check that runs elsewhere.
decision/shared-test-modules-not-targets · Shared test modules are no target — Shared test modules, the flow scripts directory and the committed checks data directory are no test target; checks read shared modules through mod declarations.
decision/started-host-held-behind-guard · Started hosts held behind a guard — The seven_guis checks hold a session spawned by the binary's start command behind a guard that stops it. · why: The start command closes the host's streams and neither kills nor reaps it.
contract/record-checks-script/failure-output · The committed checks' recording script · failure output — Prints only step N on failure.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/bridge-tests-without-widening · Bridge tests without widening visibility — The event-bridge tests sit in a cfg(test) child module of the Dioxus document module, and no function under test gains a constructor or a pub for them. · why: DioxusEventHandler's two fields are private to that module and it has no constructor · except: the one cfg(test) shim, DioxusDocument::dispatch_to_vdom, which dispatches a chain into the vdom for a bridge test
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
decision/virtual-time-for-manual-timers · Virtual time for manual timer driving — blitz-vibey-script's virtual time and without_timer_thread serve embedders that drive timers manually, test runners among them.
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/harness-construction/from-component · Harness construction · Harness::from_component — Harness::from_component(fn) builds a Dioxus-backed harness and pumps once.
contract/harness-construction/from-html · Harness construction · Harness::from_html — Harness::from_html(html) builds an HTML-backed harness and pumps once.
contract/harness-construction/from-html-with · Harness construction · Harness::from_html_with — Harness::from_html_with(html, HarnessOptions) builds an HTML-backed harness with the given options and pumps once.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once.
contract/harness-construction/wrap · Harness construction · wrap — Harness wrap builds a harness without pumping.
contract/harness-core/advance-time · Harness core · advance_time — advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
contract/host-role/stdout · escher-session host role · stdout — The host role writes nothing to stdout.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/parsed-markup-pinned-with-real-parser · Parsed markup pinned where the real parser runs — Parsed markup is pinned in blitz-tests, where the harness has the real HTML parser, not in dioxus-native-dom unit tests. · why: the unit tests' default DocumentConfig carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level
decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
contract/agent-run-selection/name · agent-run run selection · {name} — {name} runs one file, matching ^[a-z0-9_]+$ with tests/blitz-tests/tests/{name}.rs present; any other selection exits 2.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-core/time · Harness core · time — time reads the harness's virtual clock in seconds.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
contract/driver-cli/refused-line · Driver command line · refused line — A command line that is no command, or that the schema refuses, runs nothing.
contract/driver-verbs/level-check · Driver verbs · level check — A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fourteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
contract/refusal-causes/printed-form · Refusal causes · printed form — A refusal's printed form opens with its cause's name and ends with its remedy.
decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/session-error-messages-fixed · Session error messages are fixed text — Every session error message is a non-empty fixed string holding no `/` and no exit code; the input-or-output arm adds only the kind's name and is spelled «input or output», since «I/O» holds a `/`.
decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
contract/coverage-role/unreadable-check · The coverage role · unreadable check — A missing file, a non-check, a check past its bound and another app's check are each refused `malformed`.
contract/driver-verbs/lookup · Driver verbs · lookup — A verb is found by its exact name only; a name differing in case or padding finds nothing.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.
contract/session-host/non-unix · session host · non-unix arms — Off unix the session host's arms return `Unsupported`.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/advance-accepted-on-every-session · advance accepted on every session — The `advance` verb is accepted on every session, never refused for the session it addresses. · why: ratified
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/session-refuses-out-of-grammar · Out-of-grammar requests refused — A session request outside the grammar or over the bound is refused with nothing run. · guard: stand_session_lifecycle
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/session-errors/unsupported · Session errors · Unsupported — Returned by the session arms on a non-unix target.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
contract/session-state-directory/session-sock · the session state directory · session.sock — The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
contract/mcp-role-stdout/protocol-lines · the mcp role's stdout · protocol lines — The mcp role writes protocol lines only on stdout.
contract/session-errors/alreadyrunning · session errors · AlreadyRunning — Returned by start while a session is already up.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-status/served · the session status · served — Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/checks-hold-no-screenshot-step · No screenshot step in committed checks — The scripts behind the committed checks hold no screenshot step. · why: a shot's PNG bytes differ by OS
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems. · why: a shot's PNG bytes differ by OS · guard: the_committed_checks_are_what_the_binary_records
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.
contract/cli-help/output · escher-session CLI help · help — `help` answers status 0 and one line equal to `describe()`, reaching no session.
contract/cli-refusal/remedy · escher-session CLI refusals · remedy — Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
contract/cli-schema/refused-call · escher-session CLI schema · refused call — A call the schema refuses reaches no session.
contract/mcp-help/parity · escher-session MCP role · help — `help` is answered before any `start`, the same bytes through MCP and the command line.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
contract/cli-record/permissions · escher-session CLI recording · --record — `start <task> --record <dir>` keeps one `0600` check in a `0700` directory; a group-open recording directory is refused, status 3, with nothing spawned.
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fourteen verbs in order, `graph` after `lint` and `actions` after `graph`.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.
decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/graph-drives-confirm-every-write · Shared graph drives exercise every stand write — The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag. · why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.
decision/stand-load-page-fixed-answer · Load page stand-in answers after a fixed delay — The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider. · why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.
contract/driver-advance/advance · the driver's advance command · advance — Moves the session's clock by exactly ms inside one settled step.
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
contract/driver-advance/advanced-ms · the driver's advance command · advanced_ms — Is exactly ms, the clock's move, not what the Timer's counter gained.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/advance-moves-clock-by-exactly-ms · Advance moves the virtual clock by exactly ms — The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained. · why: ratified · guard: stand_act_timer
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle
decision/advance-reports-clock-move · advance reports the clock's move — advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained. · why: Ratified.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/sink-capture-counted-by-target · Sink capture counted by target — A check that reads a sink's capture counts lines by target, never in total. · why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/scrollbar-fade-on-document-clock · Overlay scrollbar fade on the document clock — The overlay scrollbar fade reads the document's clock, the harness VirtualClock for a harness document, so a screenshot after a scroll repeats until the clock is advanced. · why: Ratified. · guard: stand_virtual_clock

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/advance-moves-clock-exactly · advance moves the clock exactly — Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed. · why: Ratified.
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff and lint findings out. · why: Ratified, where until then none of them crossed.
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after; `scroll` adds `in_view`.
contract/driver-session/not-settled-result · Driver session · settled — A verb on an instance that did not go quiet is a result, `settled` false with its `busy` class, and is not rolled back.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/driver-session/start-label · Driver session · Session::start — Takes a label of 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs once on the calling thread; an invalid label is `InvalidLabel` with nothing booted.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/help · escher-session command line · help — `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/serve · escher-session command line · serve — The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
contract/escher-session-command-line/start · escher-session command line · start — `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused: the refusal on stdout.
contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
contract/session-host/start · Session host · start — Spawns the host command and returns its first answer.
contract/session-recording/bounds · Session recording · bounds — Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and the failure reads true once; the call is not rolled back.
contract/session-recording/check-file · Session recording · check file — `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/call · Session wire · call v3 — `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.
contract/session-wire/lifecycle-reply-bound · Session wire · lifecycle reply bound — A lifecycle reply is at most 128 bytes.
contract/session-wire/other-io-bound · Session wire · other read and write bound — Every read and write other than a call's answer is bounded at 2 s.
contract/session-wire/refused-malformed · Session wire · refused malformed — Any other line is answered `refused malformed`.
contract/session-wire/request-bound · Session wire · request bound — A request line is at most 16,384 bytes.
decision/recording-checks-unix-gated · Recording checks unix-gated — A check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included, beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs.
contract/check-replay/another-app · Check replay · another app — A check replayed on a session whose label is not the check's app runs nothing.
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/driver-session/session-verb-in-run · Driver session · session-level verb — `start`, `status` or `stop` handed to the instance call path is refused `unknown-verb`.
contract/escher-session-command-line/coverage · escher-session command line · coverage — The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
contract/escher-session-command-line/coverage-refusal · escher-session command line · coverage refusal — A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
contract/session-errors/already-running · Session errors · AlreadyRunning — A start where a session already answers.
contract/session-host/call · Session host · call — Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
contract/session-state/directory · Session state directory · directory — A directory the caller names, mode `0700`, holding `session.sock` alone while a session is up and absent after `stop` or the host's idle expiry.
contract/session-wire/accepted · Session wire · ok v3 accepted — `ok v3 accepted <json>` carries an accepted call's answer.
contract/session-wire/connection · Session wire · connection — One request line and one reply line per connection, every request carrying version `v3`.
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
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
