# test-plan · the rows that bind 2026-11-14-start-in-a-state — picked 202 of 602 (122 by every draw) · rule any · pin 865

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
decision/no-screenshot-step-in-committed-checks · No screenshot step in a committed check — A committed recorded check holds no screenshot step; screenshot repeatability is asserted on one host only. · why: a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a shot is byte-identical on one host only · guard: stand_screenshot_repeat
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — A host that served its session and stopped exits 0, with nothing on stdout and no state directory left.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — An argument given to actions is refused with status 1.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
contract/mcp-server/help · MCP server · help — help is answered before any start and reads the same bytes through MCP and through the command line.
contract/mcp-server/refused-input · MCP server · refused input — server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fourteen verbs in order.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.
contract/escher-session-stderr/service-name · escher-session stderr · service.name — Every stderr line is stamped service.name=seven_guis.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cross-os-checks-hold-no-shot · Cross-OS byte comparisons hold no shot — A check compared byte for byte across OSes holds no shot. · why: a shot's PNG bytes differ between OSes
decision/recorded-checks-kept-fresh · Recorded checks kept fresh — A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records. · guard: seven_guis recording
decision/recorded-checks-replayed-as-tests · Recorded checks replayed as tests — A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/handler-coverage-by-replay · Handler coverage measured by replay — Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
decision/no-shot-in-cross-os-byte-checks · No shot in cross-OS byte comparisons — A check compared byte for byte across OSes holds no shot. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
contract/escher-session-commands/coverage · escher-session commands · coverage <app> <check>... — Lists the handlers no given recorded check exercised.
contract/handler-coverage-limits/unreached-screen · Handler coverage limits · unreached screen — A handler on no screen a replay reached is not listed.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
decision/shared-fixtures-in-one-module · Shared fixtures in one module — The fixture boot and hold, and every fixture more than one check reads, are stated once in one fixtures module with no test and no test target, holding no print, tracing call, env read, file, socket, listener or blitz_net. · except: a page with one reader, or whose ids differ, stays in its own check file
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/all · agent-run selections · run all — Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-settle/busy-loads · Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
contract/host-role/stdout · escher-session host role · stdout — The host role writes nothing to stdout.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-cells · seven_guis headless stand · stand::boot_cells — stand::boot_cells(HarnessOptions) boots Cells as Task::Cells in TaskShell, with no LeanTask.
contract/stand-boot/boot-load · seven_guis headless stand · stand::boot_load — stand::boot_load(HarnessOptions) boots the load page, TaskShell titled Load, its options' net_provider replaced by the stand's LoadNet and every other field kept.
contract/stand-boot/boot-timer · seven_guis headless stand · boot_timer — boot_timer(HarnessOptions) boots the Timer as a fresh VirtualDom over from_vdom rendering the task in TaskShell, and returns the harness with its TimerTicks.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — True when each accepted `start` left one kept check opening with its app's start record.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/blitz-tests-single-test-target · blitz-tests manifest holds one test target — The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: the fork's standing two-line difference from upstream's manifest · guard: BlitzTestsTargetsTest

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/session-error-messages-fixed · Session error messages are fixed text — Every session error message is a non-empty fixed string holding no `/` and no exit code; the input-or-output arm adds only the kind's name and is spelled «input or output», since «I/O» holds a `/`.
contract/coverage-role/unreadable-check · The coverage role · unreadable check — A missing file, a non-check, a check past its bound and another app's check are each refused `malformed`.
contract/driver-cli/refused-line · Driver command line · refused line — A command line that is no command, or that the schema refuses, runs nothing.
contract/driver-verbs/level-check · Driver verbs · level check — A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fourteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
decision/blitz-tests-manifest-difference · blitz-tests manifest's difference from upstream — The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: The fork's standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
contract/refusal-causes/printed-form · Refusal causes · printed form — A refusal's printed form opens with its cause's name and ends with its remedy.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/session-host/non-unix · session host · non-unix arms — Off unix the session host's arms return `Unsupported`.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/advance-accepted-on-every-session · advance accepted on every session — The `advance` verb is accepted on every session, never refused for the session it addresses. · why: ratified
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/handler-coverage-reads-every-replayed-screen · Handler coverage reads every replayed screen — Handler coverage reads every screen the replays reach, the UI graph read at boot and after each replayed step.
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/session-refuses-out-of-grammar · Out-of-grammar requests refused — A session request outside the grammar or over the bound is refused with nothing run. · guard: stand_session_lifecycle
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/session-errors/alreadyrunning · session errors · AlreadyRunning — Returned by start while a session is already up.
contract/session-errors/unsupported · Session errors · Unsupported — Returned by the session arms on a non-unix target.
contract/session-protocol/hello · the session protocol · hello — hello answers idle_expiry_s.
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/load-wait-bound · Acting steps wait out loads in flight — An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`. · guard: stand_load_wait
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
contract/handler-coverage/other-app-check · handler coverage · check of another app — A check recorded on another app contributes nothing to the coverage.
contract/mcp-role-stdout/protocol-lines · the mcp role's stdout · protocol lines — The mcp role writes protocol lines only on stdout.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-state-directory/session-sock · the session state directory · session.sock — The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
contract/session-status/served · the session status · served — Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/checks-hold-no-screenshot-step · No screenshot step in committed checks — The scripts behind the committed checks hold no screenshot step. · why: a shot's PNG bytes differ by OS
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
contract/cli-help/output · escher-session CLI help · help — `help` answers status 0 and one line equal to `describe()`, reaching no session.
contract/cli-record/permissions · escher-session CLI recording · --record — `start <task> --record <dir>` keeps one `0600` check in a `0700` directory; a group-open recording directory is refused, status 3, with nothing spawned.
contract/cli-refusal/remedy · escher-session CLI refusals · remedy — Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
contract/cli-schema/refused-call · escher-session CLI schema · refused call — A call the schema refuses reaches no session.
contract/cli-snapshot/budget · escher-session CLI snapshot · size budget — A snapshot answers the in-process text as one line inside its 10,000-byte budget.
contract/cli-stdout/closed · escher-session CLI stdout · closed stdout — A run with its stdout closed before it starts keeps its ending's status, prints no panic and writes nothing typed back.
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/mcp-help/parity · escher-session MCP role · help — `help` is answered before any `start`, the same bytes through MCP and the command line.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems. · why: a shot's PNG bytes differ by OS · guard: the_committed_checks_are_what_the_binary_records
contract/cli-actions/argument · escher-session CLI actions verb · argument — An argument to `actions` is refused, status 1.
contract/cli-actions/output · escher-session CLI actions verb · output — `actions` against a started host answers one JSON line of the actionable nodes.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/cli-exit-codes/3 · escher-session CLI exit codes · 3 — A session error: a command that finds no session, a state directory too long, or a group-open recording directory.
contract/cli-snapshot/part · escher-session CLI snapshot · --part — A part read by number answers one uncoloured JSON line, status 0; a part of 0, 1001 or a non-number exits 1 with the `malformed` refusal line, nothing typed written back.
contract/cli-state-dir/max-length · escher-session CLI state directory · length bound — A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
contract/mcp-protocol/errors · escher-session MCP role · protocol errors — A protocol error runs nothing.
contract/mcp-protocol/revision · escher-session MCP role · protocol revision — The role speaks the newest MCP protocol revision.
contract/mcp-snapshot/parity · escher-session MCP role · snapshot — A fresh Counter's snapshot reads the same through MCP and the command line.
contract/mcp-tool-result/screenshot · escher-session MCP tool results · screenshot — A `screenshot` tool call answers an `image/png` image block then a text block, `isError` false.
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fourteen verbs in order, `graph` after `lint` and `actions` after `graph`.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.
decision/stand-load-page-fixed-answer · Load page stand-in answers after a fixed delay — The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider. · why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/network-stand-ins-counted · Network stand-ins keep their loads counted — Network stand-ins do not override is_noop, so the harness counts their loads.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.
contract/driver-advance/advance · the driver's advance command · advance — Moves the session's clock by exactly ms inside one settled step.
contract/driver-advance/advanced-ms · the driver's advance command · advanced_ms — Is exactly ms, the clock's move, not what the Timer's counter gained.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle
decision/advance-moves-clock-by-exactly-ms · Advance moves the virtual clock by exactly ms — The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained. · why: ratified · guard: stand_act_timer
decision/advance-reports-clock-move · advance reports the clock's move — advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained. · why: Ratified.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold. · why: Ratified as three crates, each floor one point below. · guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate · except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff and lint findings out. · why: Ratified, where until then none of them crossed.
contract/check-replay/another-app · Check replay · another app — A check replayed on a session whose label is not the check's app runs nothing.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after; `scroll` adds `in_view`.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/driver-session/start-label · Driver session · Session::start — Takes a label of 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs once on the calling thread; an invalid label is `InvalidLabel` with nothing booted.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/coverage · escher-session command line · coverage — The coverage role is `coverage <app> <check>...`, no `--session` or `--record` among its paths; it reads each named kept check bounded and answers the coverage line on stdout, status `0`; any other shape is usage, `2`; a failing boot is its session error, `3`.
contract/escher-session-command-line/help · escher-session command line · help — `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/start · escher-session command line · start — `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
contract/escher-session-error-object/fields · escher-session error object · fields — A session error object carries `kind`, `message` and `remedy`.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused: the refusal on stdout.
contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
contract/session-errors/already-running · Session errors · AlreadyRunning — A start where a session already answers.
contract/session-host/start · Session host · start — Spawns the host command and returns its first answer.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/connection · Session wire · connection — One request line and one reply line per connection, every request carrying version `v3`.
contract/session-wire/refused-malformed · Session wire · refused malformed — Any other line is answered `refused malformed`.
contract/session-wire/refused-version · Session wire · refused version — A known word with another version is answered `refused version`.
contract/session-wire/request-bound · Session wire · request bound — A request line is at most 16,384 bytes.
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
contract/driver-session/session-verb-in-run · Driver session · session-level verb — `start`, `status` or `stop` handed to the instance call path is refused `unknown-verb`.
contract/escher-session-command-line/coverage-refusal · escher-session command line · coverage refusal — A check file the coverage role cannot read, one past the bound, one that is no check or a check of another app is refused `malformed`, status `1`, holding nothing of the path or the file.
contract/escher-session-command-line/record-flag · escher-session command line · --record — `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
contract/escher-session-command-line/serve · escher-session command line · serve — The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/session-host/attach · Session host · attach — Returns `Hello { pid, label, served, idle_expiry_s }`.
contract/session-host/served · Session host · served — Every answered request, a refused or malformed one included, counts toward `served` and starts the expiry count again.
contract/session-state/directory · Session state directory · directory — A directory the caller names, mode `0700`, holding `session.sock` alone while a session is up and absent after `stop` or the host's idle expiry.
contract/session-wire/accepted · Session wire · ok v3 accepted — `ok v3 accepted <json>` carries an accepted call's answer.
contract/session-wire/call · Session wire · call v3 — `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.
contract/session-wire/hello · Session wire · hello v3 — Answered `ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}`.
contract/session-wire/other-io-bound · Session wire · other read and write bound — Every read and write other than a call's answer is bounded at 2 s.
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
