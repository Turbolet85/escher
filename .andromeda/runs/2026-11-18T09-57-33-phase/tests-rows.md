# test-plan · the rows that bind 2026-11-18-served-error-page — picked 172 of 628 (87 by every draw) · rule any · pin 2112

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states in words the schema's verb, cause, session-error, node-key and exit-status counts and the a11y and stand-repeat legs' counts; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script. · guard: CapabilityStatementTest
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
decision/stand-checks-in-both-layout-modes · Stand checks run in both layout modes — A stand check that reads a screen asserts its readings in both layout modes, and the two modes' readings are held equal where the check compares them.
decision/unrun-declared-write-reads-unconfirmed · Declared write unrun after its event reads unconfirmed — The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it. · why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/driver-refusal-causes/fonts-unavailable · driver refusal causes · fonts-unavailable — A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
contract/escher-session-stderr/service-name · escher-session stderr · service.name — Every stderr line is stamped service.name=seven_guis.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/cli-checks-run-own-binary · Command-line checks run the package's own binary — seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/seven-guis-cli-targets-unix-only · seven_guis command-line targets gated to unix — seven_guis' command-line and shell-flow test targets are gated to unix.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/cli-checks-run-the-binary · Command-line checks run the binary — The seven_guis command-line checks run the escher-session binary once per command as a client process and read its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/one-test-target-per-file · One test target per file — blitz-tests keeps one auto-discovered test target per file under its tests directory, upstream's single all target held unbuilt at test = false. · why: A ratified difference from upstream, whose layout is autotests = false with all the crate's one test target. · guard: BlitzTestsTargetsTest
decision/pinned-viewport-read-from-document · Pinned viewport read from the document — A boot-condition check of the pinned viewport reads the document viewport, never the root element's bounds. · why: A component alone has no shell CSS, so its root element sizes to its content.
decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red. · guard: stand_requirement_sweep · except: A requirement with no screen names no witness.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/cli-and-flow-targets-unix-gated · Command-line and flow targets unix-gated — The command-line check target and the shell flow target are gated to unix; the recording target is gated to unix item by item, beside an off-unix check that runs elsewhere.
decision/cross-os-readings-by-answer-digest · Cross-OS readings of the served tasks — The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file. · guard: stand_cross_os_readings
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/socket-checks-unix-gated · Socket session checks are unix-gated — A check holding a session over the socket is gated to unix. · except: The MCP checks speak to the mcp child over piped stdin and are not unix-gated, bar the MCP server's command-line comparisons, so the windows CI job runs them.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.
decision/wcag-2-contrast-ratio · WCAG 2.x contrast ratio — The contrast ratio is the WCAG 2.x ratio over the color crate's relative luminance.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/cold-agent-events-content-free · Cold-agent events and verdict carry no content — No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one. · guard: the scrub-set key check and four planted markers under the shim
decision/cold-agent-live-run-off-ci · Live cold-agent run never in CI — The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests. · why: no CI secret
decision/cold-agent-passes-on-positive-evidence · Cold-agent run passes only on positive evidence — A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all six apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
decision/cold-agent-prompt-names-no-ids · Cold-agent prompt gives no element ids or app slugs — The fixed stand prompt asks for the six apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
decision/cold-agent-record-private · Cold-agent recordings kept private — The live server's kept checks sit in an owner-only record directory and are never printed, copied or committed; the parsed check file is never printed.
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
decision/test-output-never-reaches-events · Test output never reaches agent-run events — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
contract/agent-run-verbs/logs · agent-run verbs · logs — logs prints events.jsonl verbatim, one JSON object per line, exit 3 when it is absent; raw cargo output stays in run.log, never printed.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cold-agent-events/run-end · cold-agent events · run.end — A JSON line holding every verdict field, printed and appended to `events.jsonl`.
contract/cold-agent-exit-codes/1 · cold-agent exit codes · 1 — The verb ran and failed (a `failed` verdict).
contract/cold-agent-tasks/stand · cold-agent pipe tasks · stand — `stand` is the one task: six apps (counter, flight booker, timer, CRUD, cells and the load page) in one client session; any other task, `run counter` included, is a usage error.
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — Reasons come from `client-exit · no-result · not-isolated · counts-disagree · no-tool-call · task-unmet · capability-unused · recording-not-kept · wrong-calls-over-bar · no-check · check-failed · check-cannot-fail`; `no-check` and an invalid check's `check-failed` stand alone.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
decision/cold-agent-server-log-counted-only · Cold-agent server log counted, never shown — The server's log is read only to count its span lines; it is never printed or copied.
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar of 5 % — A cold-agent run fails `wrong-calls-over-bar` when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold. · guard: ColdAgentPipeTest
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
decision/one-contrast-formula · One contrast formula — Session lint's contrast reads the same as the harness's text_contrast, with no second formula.
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
decision/session-checks-share-one-module · Session checks share one module — What the checks that hold a session share, in process and by a host process of their own, is stated in one session test module with no test and no test target of its own, beside the stand checks' module. · why: kept beside the stand checks' module because that module was held unchanged
contract/agent-run-events/test · agent-run events · test — test {file, test, outcome: ok|failed|ignored}, one per libtest line, file the target binary's stem with its hash stripped or doc:{crate}; printed to stdout and appended to events.jsonl.
contract/agent-run-selection/stand · agent-run run selection · stand — stand passes one --test per tests/blitz-tests/tests/stand_*.rs present; with none present it is an empty run, no cargo call, exit 1.
contract/agent-run-selections/stand · agent-run selections · run stand — Runs the `stand_*` readers of the shared module.
contract/agent-run-verbs/run · agent-run verbs · run — run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/cold-agent-exit-codes/0 · cold-agent exit codes · 0 — Success, on every verb.
contract/cold-agent-invocation/windows-entry · cold-agent pipe invocation · windows entry — The Windows entry of the cold-agent pipe is a pass-through with no logic of its own.
contract/cold-agent-verdict/counts-agree · cold-agent verdict · counts_agree — True when the server's tool-call span lines equal the escher tool uses and its refused plus error lines equal the escher tool results marked `is_error`.
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — True when each accepted `start` left one kept check opening with its app's start record.
contract/cold-agent-verdict/tasks-met · cold-agent verdict · tasks_met — An app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`, a paged screen read as its parts joined, never from the reply.
contract/command-span/graph · command span · graph — A graph call's command line records verb alone under its tool_call parent.
contract/harness-screenshot/fonts-unavailable · Harness screenshot · ScreenshotError::FontsUnavailable — A document whose can_paint_text reads false gives FontsUnavailable, with nothing painted.
contract/host-role/stdout · escher-session host role · stdout — The host role writes nothing to stdout.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/agent-run-events-content-free · Agent-run output carries no panic text — A failing agent run puts no captured panic text in stdout or the events file, and no event carries a scrub-set key; the raw cargo output goes only to the run log. · guard: test_agent_run.py
decision/blitz-tests-single-test-target · blitz-tests manifest holds one test target — The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: the fork's standing two-line difference from upstream's manifest · guard: BlitzTestsTargetsTest

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/blitz-tests-manifest-difference · blitz-tests manifest's difference from upstream — The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: The fork's standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states the schema's counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/cold-agent-output-scrubbed · Cold-agent output scrubbed — A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text. · guard: ColdAgentPipeTest
decision/cold-agent-session-outside-root · Cold-agent session outside the root — A cold-agent session runs outside the repository root and is removed after, and its prompt names no stand id. · guard: ColdAgentPipeTest
decision/cold-agent-tests-offline · Cold-agent pipe tested offline — The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary. · guard: ColdAgentPipeTest
decision/contrast-required-ratio · Required contrast ratio — Large text, at least 24 px or 14 pt at weight 700, requires a contrast ratio of 3.0; all other text requires 4.5.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
decision/prompt-goal-control-phrase-once · Prompt goal controls pin a phrase standing once — A deletion control on a prompt goal pins the goal by a phrase that stands once in the prompt. · why: A phrase that also stands elsewhere in the prompt makes the control vacuous.
contract/driver-argument-kinds/name · Driver argument kinds · ArgKind::Name — A name argument is 1 to 32 bytes of `a-z`, `0-9` and `-`.
contract/driver-verbs/level-check · Driver verbs · level check — A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.
decision/harness-graph-tests-enable-accessibility · Harness graph unit tests turn accessibility on — A harness unit test of the UI graph turns the accessibility feature on through its dev-dependency, for the test build only. · why: A per-package test build leaves the accessibility feature off, and the UI graph is gated by it.
contract/cold-agent-verdict/capability-unused · Cold-agent verdict · capability-unused — A run that leaves a capability unused fails `capability-unused`.
contract/cold-agent-verdict/recording-not-kept · Cold-agent verdict · recording-not-kept — A `start` that left no kept check fails `recording-not-kept`.
contract/driver-argument-kinds/part · Driver argument kinds · ArgKind::Part — A part argument is a whole number from 1 to `MAX_PART`, 1000.
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
contract/lint-defects/clipped · Lint defects · clipped — `clipped` is the second of the six lint defects, in their stated order.
contract/lint-defects/contrast · Lint defects · contrast — `contrast` is the sixth of the six lint defects, in their stated order.
contract/lint-defects/off-screen · Lint defects · off-screen — `off-screen` is the fourth of the six lint defects, in their stated order.
contract/lint-defects/overlap · Lint defects · overlap — `overlap` is the first of the six lint defects, in their stated order.
contract/lint-defects/truncated · Lint defects · truncated — `truncated` is the third of the six lint defects, in their stated order.
contract/lint-defects/zero-size · Lint defects · zero-size — `zero-size` is the fifth of the six lint defects, in their stated order.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/mcp-result-decoded-whole · MCP tool result text decoded whole — A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper. · why: Un-escaping quotes alone leaves a Windows path's backslashes doubled. · guard: stand_act_errors
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/layout-tests-in-both-layout-modes · Layout assertions in both layout modes — Layout assertions run in both layout modes, non-incremental and incremental, as the pipeline tests do. · guard: incremental_oracle · except: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
decision/sink-drops-third-party-targets · Sink drops third-party targets — The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it. · guard: telemetry_drop
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
contract/session-status/served · the session status · served — Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/cli-checks-through-own-binary · Command-line checks through the package's own binary — Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/example-fixtures-are-manual · Example fixtures carry no automated assertion — Example fixture pages state their click-through instructions and expected outcomes in text instead of automated assertions.
decision/real-mcp-client-outside-ci · Real MCP client only in the operator pass — A real MCP client drives escher-session mcp in the operator pass, not in CI.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/cli-snapshot/budget · escher-session CLI snapshot · size budget — A snapshot answers the in-process text as one line inside its 10,000-byte budget.
decision/no-screenshot-in-recorded-check-scripts · No screenshot step in committed recorded checks — The scripts that record the committed checks hold no `screenshot` step, so the kept checks stay byte-equal across operating systems. · why: a shot's PNG bytes differ by OS · guard: the_committed_checks_are_what_the_binary_records

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/checks-hold-no-private-data · Committed checks hold no private data — The committed checks hold no typed text, host path or secret.
decision/shared-fixture-pages · Shared fixture pages stated once — A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options. · except: a page with one reader stays in its check's file
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/graph-drives-confirm-every-write · Shared graph drives exercise every stand write — The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag. · why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag
contract/committed-checks/excluded-content · committed recorded checks · excluded content — A committed check holds ids, names, values, snapshot text and diffs, and no typed text, host path, secret or screenshot step.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/stand-repeat-three-rounds · Stand checks repeated three rounds — The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/joining-file-keeps-its-legs · A file joining a leg keeps its legs — A test file joining a CI leg keeps every leg it already ran in.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/stand-checks-zero-failure-budget · Zero-failure budget for stand checks — The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red. · guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/zero-failure-stand-repeat · Zero-failure flakiness budget for stand checks — The stand checks hold a zero-failure budget: the stand_*.rs files, the named blitz-tests targets and seven_guis' process checks run three times with no retry, and any failure fails the leg. · why: Ratified as a repeated job with zero failures. · guard: stand-repeat leg, job Stand checks repeated

## session-lifecycle · key of ## 3. Test Harness Contract
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/payload-unread · Driver session · <payload unread> — A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/escher-session-command-line/mcp · escher-session command line · mcp — The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
contract/escher-session-command-line/serve · escher-session command line · serve — The host role is `serve <task> --session <dir> [--record <dir>]`; it ends with nothing written and status `0` after a stop or an expiry, and `3` on a session error.
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/socket-session-checks-unix-gated · Socket session checks unix-gated — A check holding a session over the socket is `cfg(unix)`-gated; a check driving the `mcp` stdio child bounds every read, kills and reaps the child in its guard's drop and is not unix-gated, so the windows job runs it. · except: The `mcp_server` command-line comparison is unix-gated.
contract/driver-session/id-record · Driver session · record of ids — Holds id text only, at most 4096 ids, for the session's life, no id longer than `MAX_ID_BYTES`, read by nothing outside the crate.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
contract/driver-session/start-label · Driver session · Session::start — Takes a label of 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs once on the calling thread; an invalid label is `InvalidLabel` with nothing booted.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
contract/escher-session-command-line/lint · escher-session command line · lint — `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
contract/escher-session-command-line/snapshot · escher-session command line · snapshot — `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
contract/session-host/call · Session host · call — Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
contract/session-host/start · Session host · start — Spawns the host command and returns its first answer.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/exploration-checks · session recording · exploration checks — With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a path whose accepted answer carries a panic item ends panicked with no payload, a re-reach step diverged, and its check holds the panicked step.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/accepted · Session wire · ok v3 accepted — `ok v3 accepted <json>` carries an accepted call's answer.
contract/session-wire/call · Session wire · call v3 — `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.

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
