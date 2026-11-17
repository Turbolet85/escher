# test-plan · the rows that bind 2026-11-17-layout-on-the-carriers — picked 188 of 616 (122 by every draw) · rule any · pin 1580

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its counts, and the README's count of MCP tools, are held equal to the driver schema's tables. · guard: CapabilityStatementTest
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — An argv that is no command ends as a usage error, its usage line holding nothing the caller typed, nothing booted and no state created.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error: a host that cannot serve, no session, or a state directory too long, the message holding no path.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
contract/mcp-server/help · MCP server · help — help is answered before any start and reads the same bytes through MCP and through the command line.
contract/mcp-server/refused-input · MCP server · refused input — server/discover, a non-JSON line, a 65,537-byte line, an unknown tool, a refused or a malformed call are each answered without running anything, no reply holding the refused id, the tool name or the filler.
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fourteen verbs in order.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/typed-password-never-leaves · A typed password never leaves the instance — A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels. · guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
contract/escher-session-exit-codes/closed-stdout · escher-session exit codes · closed stdout — A run whose stdout is closed before it starts keeps its ending's status, printing no panic and creating no state directory.
contract/escher-session-stderr/service-name · escher-session stderr · service.name — Every stderr line is stamped service.name=seven_guis.
contract/mcp-server/stdout · MCP server · stdout — The mcp role writes protocol lines only on stdout.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/cross-os-checks-hold-no-shot · Cross-OS byte comparisons hold no shot — A check compared byte for byte across OSes holds no shot. · why: a shot's PNG bytes differ between OSes
decision/recorded-checks-replayed-as-tests · Recorded checks replayed as tests — A check a session recorded is committed as data and run as an ordinary cargo test target, every step re-validated through the session and red at the first step whose acceptance or kept answer differs.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are held to the schema's tables, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/no-shot-in-cross-os-byte-checks · No shot in cross-OS byte comparisons — A check compared byte for byte across OSes holds no shot. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
contract/record-checks-script/arguments · The committed checks' recording script · arguments — Takes two arguments, the binary and an existing output directory.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/harness-virtual-clock · Harness time is a virtual clock — The harness's time is a virtual clock its documents read, moved by tick and advance_time; screenshot, screenshot_rect, text_contrast and settle never move it. · why: ratified
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
decision/timer-step-carries-remainder · Timer step carries its remainder — The stand's timer step returns nothing and carries its remainder below one tick across calls, and a driver session moves its clock on every advance. · why: ratified
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/cold-agent-check/step · the cold agent's check · step — A step holds `tool` among the fourteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
contract/cold-agent-verdict/isolated · cold-agent verdict · isolated — True when the init tools are exactly the fourteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
contract/command-span/screenshot · command span · screenshot — A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
contract/harness-core/advance-time · Harness core · advance_time — advance_time(Duration) advances the virtual clock by exactly the duration and then pumps.
contract/harness-core/pump · Harness core · pump — pump polls the document with no waker context and resolves at the harness's virtual clock time, applying pending changes after a mutation.
contract/harness-core/tick · Harness core · tick — tick(dt) advances the virtual clock by dt and then pumps; a negative or non-finite dt moves nothing.
contract/harness-settle/busy-loads · Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.
contract/harness-settle/settle · Harness settle · settle — settle(&mut self) -> Result<Settled, NotSettled> repeats the pump pass until a quiet pass (poll answered false, the hover node did not change, no load finished) and returns Settled { passes, animating }.
contract/harness-settle/side-effects · Harness settle · side effects — settle reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing; pump and the input helpers never settle.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
contract/stand-boot/timer-step · seven_guis headless stand · stand::timer_step — stand::timer_step(TimerTicks) is the time step a driver Session on the Timer takes through with_time, with TIMER_TICK_MS (100) as its tick.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/ci-legs-through-leg-script · CI legs run through the leg script — Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite. · guard: CiWorkflowTest in test_ci_workflows.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/fast-jobs-gate-the-rest · Fast jobs gate the rest — The four fast CI jobs need nothing and every other job needs all four. · guard: CiWorkflowTest
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
contract/driver-cli/refused-line · Driver command line · refused line — A command line that is no command, or that the schema refuses, runs nothing.
contract/driver-json/number · Driver JSON answers · number — A number is written as its shortest decimal, and as `null` when not finite.
contract/driver-verbs/level-check · Driver verbs · level check — A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
contract/driver-verbs/lookup · Driver verbs · lookup — A verb is found by its exact name only; a name differing in case or padding finds nothing.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fourteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
contract/refusal-causes/printed-form · Refusal causes · printed form — A refusal's printed form opens with its cause's name and ends with its remedy.
decision/contrast-required-ratio · Required contrast ratio — Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/ci-leg-exit-codes/2 · CI leg script exit codes · 2 — An unknown leg exits 2.
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
contract/lint-defects/clipped · Lint defects · clipped — `clipped` is the second of the six lint defects, in their stated order.
contract/lint-defects/contrast · Lint defects · contrast — `contrast` is the sixth of the six lint defects, in their stated order.
contract/lint-defects/off-screen · Lint defects · off-screen — `off-screen` is the fourth of the six lint defects, in their stated order.
contract/lint-defects/overlap · Lint defects · overlap — `overlap` is the first of the six lint defects, in their stated order.
contract/lint-defects/truncated · Lint defects · truncated — `truncated` is the third of the six lint defects, in their stated order.
contract/lint-defects/zero-size · Lint defects · zero-size — `zero-size` is the fifth of the six lint defects, in their stated order.
contract/lint-findings/order · Lint findings · order — Lint findings come in document order, then in the stated order of the defects.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/session-host/non-unix · session host · non-unix arms — Off unix the session host's arms return `Unsupported`.
contract/driver-log/default-level · the driver's log line · default level — At the default level the driver writes no line.
contract/driver-session/password · driver session · typed password — A password typed through the driver never leaves in an answer or as pixels.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper. · why: Un-escaping quotes alone leaves a Windows path's backslashes doubled. · guard: stand_act_errors
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/session-refuses-out-of-grammar · Out-of-grammar requests refused — A session request outside the grammar or over the bound is refused with nothing run. · guard: stand_session_lifecycle
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/mcp-role-stdout/protocol-lines · the mcp role's stdout · protocol lines — The mcp role writes protocol lines only on stdout.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/refusal-cause/not-found · driver refusal causes · not-found — A call naming an id not on the screen is refused `not-found`, the instance unchanged.
contract/refusal-cause/stale · driver refusal causes · stale — A call naming a stale id is refused `stale`, the instance unchanged.
contract/refusal-cause/unknown-verb · driver refusal causes · unknown-verb — A call naming no known verb is refused `unknown-verb`, the instance unchanged.
contract/session-errors/unsupported · Session errors · Unsupported — Returned by the session arms on a non-unix target.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-host-stderr/service-name · session host stderr lines · service.name — Every line the escher-session host writes to stderr carries `service.name=seven_guis`.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
decision/load-wait-bound · Acting steps wait out loads in flight — An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`. · guard: stand_load_wait
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver occurs nowhere in what the call returns, reads `MASKED_VALUE`, and never leaves as pixels in a screenshot. · guard: stand_act_screenshot, stand_act_filled
contract/session-errors/dead · Session errors · Dead — A socket file that nothing answers on, as after the session host is killed.
contract/session-errors/nosession · session errors · NoSession — Returned when a session is reached and none is running.
contract/session-status/served · the session status · served — Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/real-mcp-client-outside-ci · Real MCP client only in the operator pass — A real MCP client drives escher-session mcp in the operator pass, not in CI.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/cli-actions/output · escher-session CLI actions verb · output — `actions` against a started host answers one JSON line of the actionable nodes.
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.
contract/cli-exit-codes/2 · escher-session CLI exit codes · 2 — A line that is no command, a misplaced `--record` or a malformed `coverage` role call is a usage error.
contract/cli-exit-codes/3 · escher-session CLI exit codes · 3 — A session error: a command that finds no session, a state directory too long, or a group-open recording directory.
contract/cli-help/output · escher-session CLI help · help — `help` answers status 0 and one line equal to `describe()`, reaching no session.
contract/cli-refusal/remedy · escher-session CLI refusals · remedy — Every refusal cause the binary provokes reads a `remedy` equal to its cause's remedy in the table.
contract/cli-schema/refused-call · escher-session CLI schema · refused call — A call the schema refuses reaches no session.
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
contract/mcp-help/parity · escher-session MCP role · help — `help` is answered before any `start`, the same bytes through MCP and the command line.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fourteen verbs in order, `graph` after `lint` and `actions` after `graph`.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/cli-state-dir/max-length · escher-session CLI state directory · length bound — A state directory of up to 94 bytes on Linux, 90 bytes off Linux, starts and stops; a longer one ends `state-dir-too-long`, status 3, its message naming the bound and no path.
contract/mcp-protocol/errors · escher-session MCP role · protocol errors — A protocol error runs nothing.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/doc-leg-gates-public-doc-edits · Doc leg gates a public doc comment edit — A chunk that edits a public doc comment lists the doc leg in its local gate block. · why: The fast leg runs no rustdoc, so a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
decision/sink-capture-counted-by-target · Sink capture counted by target — A check that reads a sink's capture counts lines by target, never in total. · why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold. · why: Ratified as three crates, each floor one point below. · guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate · except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/no-screenshot-in-cross-os-checks · No screenshot step in cross-OS byte-compared checks — A check compared byte for byte across OSes, a committed check's digest included, holds no `screenshot` step; the shot path's kept form is covered by the recording test on linux and macOS. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64. · guard: stand_session_recording
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, and lint findings out. · why: Ratified, where until then none of them crossed.
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/id-record · Driver session · record of ids — Holds id text only, at most 4096 ids, for the session's life, no id longer than `MAX_ID_BYTES`, read by nothing outside the crate.
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/driver-session/stale-or-not-found · Driver session · stale · not-found — An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fourteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/help · escher-session command line · help — `help` takes no argument and at most one `--session <dir>`, not read, and answers the self-description line on stdout, status `0`, reaching no session; an argument is refused `malformed`, status `1`; `--session` with no directory or given twice is usage, status `2`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — The usage line reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, listing `graph` after `lint` and `help` after `stop`.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted: the answer as one uncoloured line of JSON on stdout.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused: the refusal on stdout.
contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage: one fixed line on stderr with stdout empty, nothing booted and no state created.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error: the error object on stdout and its fixed message on stderr; `RecordDirNotPrivate` and `RecordingFailed` end as session errors.
contract/session-errors/answer-too-large · Session errors · AnswerTooLarge — A call's answer over the socket's bound, read from an `oversize` reply, the call having run.
contract/session-host/call · Session host · call — Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/accepted · Session wire · ok v3 accepted — `ok v3 accepted <json>` carries an accepted call's answer.
contract/session-wire/answer-bound · Session wire · answer bound — An answer is at most 1,048,576 bytes on its JSON line.
contract/session-wire/call · Session wire · call v3 — `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.
contract/session-wire/oversize · Session wire · ok v3 oversize — Sent when a call's answer is over the answer bound.
contract/session-wire/refused · Session wire · ok v3 refused — `ok v3 refused <json>` carries a refused call's refusal.
decision/recording-checks-unix-gated · Recording checks unix-gated — A check that records is `cfg(unix)`-gated item by item, in-process and `mcp` stdio ones included, beside an off-unix check that a recording is refused `Unsupported`, which the windows job alone runs.
decision/socket-session-checks-unix-gated · Socket session checks unix-gated — A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it. · except: The `mcp_server` command-line comparison is unix-gated.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/escher-session-command-line/record-flag · escher-session command line · --record — `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
contract/escher-session-error-object/fields · escher-session error object · fields — A session error object carries `kind`, `message` and `remedy`.
contract/session-errors/error-set · Session errors · error set — Fifteen session errors, each stating a kind, a fixed meaning and a remedy; the state directory's message states its bound.
contract/session-wire/answer-read-bound · Session wire · answer read bound — A call's answer is read under 30 s per read; a listener that never answers ends the read as `SessionError::Timeout`.
contract/session-wire/connection · Session wire · connection — One request line and one reply line per connection, every request carrying version `v3`.

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
