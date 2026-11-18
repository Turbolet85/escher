# test-plan · the rows that bind 2026-11-18-cold-agent-live-run — picked 226 of 630 (119 by every draw) · rule any · pin 2197

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/capability-statement-pinned · Capability statement pinned to the driver schema — The capability statement names every driver verb and refusal cause as a code span and states in words the schema's verb, cause, session-error, node-key and exit-status counts and the a11y and stand-repeat legs' counts; the README states the count of driver commands an MCP client gets as tools. · guard: test_capability_statement.py
decision/test-tier · Test tier — The project's test tier is 0.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/capabilities-doc-pinned-to-schema · Capability statement pinned to the driver schema — The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script. · guard: CapabilityStatementTest
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
decision/owner-only-session-files · Owner-only session socket and recordings — A session's state directory is 0700 and its socket 0600, and a recording directory open to its group is refused with nothing written. · guard: stand_session_lifecycle, stand_session_recording
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
decision/typed-password-never-leaves · A typed password never leaves the instance — A typed password reads only as the fixed mask and occurs in no snapshot, snapshot text, diff, accessibility-tree field, recording, span field, MCP result or screenshot pixels. · guard: stand_snapshot_state, stand_act_filled, stand_mcp_masked, stand_session_recording, password_painted_masked
decision/unrun-declared-write-reads-unconfirmed · Declared write unrun after its event reads unconfirmed — The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it. · why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
contract/mcp-server/screenshot · MCP server · screenshot — A screenshot tool call is answered with an image block then a text block.
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fifteen verbs in order, `layout` after `actions` and `help` last.
contract/session-recording/group-open-directory · session recording · group-open directory — A recording directory open to its group is refused before anything runs.
contract/session-recording/same-check-on-both-surfaces · session recording · same check on both surfaces — The command line and the mcp role keep the same check byte for byte, the state directory gone after stop.
decision/advance-moves-exactly-the-ms-asked · Advance moves exactly the time asked — An advance reports exactly the ms asked whatever the step does: the step is handed exactly ms, advanced_ms equals ms, and the clock moves by exactly ms. · why: ratified · guard: stand_act_timer
decision/advance-reports-exact-ms · Hosted advance reports exactly the ms asked — A hosted advance moves the timer session with no sleep, and every ask reports exactly the milliseconds asked. · why: Ratified. · guard: host_timer
decision/mcp-causes-served-with-remedy · Refusal causes served with their remedy over MCP — Every refusal cause a tool call reaches is served through MCP with its remedy. · why: ratified · guard: stand_mcp_tools · except: unknown-verb is not served through MCP
contract/driver-refusal-causes/fonts-unavailable · driver refusal causes · fonts-unavailable — A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.
contract/escher-session-stderr/service-name · escher-session stderr · service.name — Every stderr line is stamped service.name=seven_guis.
contract/mcp-server/actions-argument · MCP server · actions argument — An actions tool call given an argument is answered isError.
contract/mcp-server/stdout · MCP server · stdout — The mcp role writes protocol lines only on stdout.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
contract/driver-refusals/fonts-unavailable · Driver refusals · fonts-unavailable — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path, and the actions read names the same fonts-unavailable for screenshot.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-check-names-held-to-schema · Claim section names and counts held to the schema — Only the claim section's names and counts are pinned, the names and schema counts to the schema's tables and the a11y leg's file count and stand-repeat's named-target count to the CI leg script, so a check renamed or moved changes that section by hand. · guard: test_capability_statement.py
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/handler-coverage-by-replay · Handler coverage measured by replay — Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red. · guard: stand_requirement_sweep · except: A requirement with no screen names no witness.
decision/shot-refused-without-fonts · Shot refused without a usable font — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path. · guard: stand_act_screenshot, stand_mcp_tools, font_reading_tests
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
contract/escher-session-commands/coverage · escher-session commands · coverage <app> <check>... — Lists the handlers no given recorded check exercised.
contract/handler-coverage-limits/bubbling-dispatch · Handler coverage limits · bubbling dispatch — A bubbling dispatch counts every ancestor listener it passed.
contract/handler-coverage-limits/canvas-handler · Handler coverage limits · canvas handler — A handler inside a canvas is not counted.
contract/handler-coverage-limits/unreached-screen · Handler coverage limits · unreached screen — A handler on no screen a replay reached is not listed.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/cold-agent-events-content-free · Cold-agent events and verdict carry no content — No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one. · guard: the scrub-set key check and four planted markers under the shim
decision/cold-agent-live-run-off-ci · Live cold-agent run never in CI — The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests. · why: no CI secret
decision/cold-agent-passes-on-positive-evidence · Cold-agent run passes only on positive evidence — A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all seven apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
decision/cold-agent-prompt-names-no-ids · Cold-agent prompt gives no element ids or app slugs — The fixed stand prompt asks for the seven apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
decision/cold-agent-record-private · Cold-agent recordings kept private — The live server's kept checks sit in an owner-only record directory and are never printed, copied or committed; the parsed check file is never printed.
decision/cold-agent-replay-bounded · Cold-agent replay child bounded — Each replay child is spawned directly, without recording, with stderr discarded, every reply bounded at 30 s, then killed and reaped; a replay child that fails reads as the check not passing, or not able to fail.
decision/cold-agent-replayed-check · Cold agent's check judged by replay — The agent's JSON check is replayed in full on a fresh driver MCP child and must pass, then replayed ablated (its acting steps left out) on a second fresh child and must fail; a malformed check is never replayed. · why: ratified as the chosen option, a JSON check with replay
decision/cold-agent-run-bounded · Cold-agent client run bounded — The client runs under a 600 s timeout and a 3 USD budget; a timeout or non-zero client exit is a `client-exit` failure, never a propagated 124.
decision/cold-agent-server-log-counted-only · Cold-agent server log counted, never shown — The server's log is read only to count its span lines; it is never printed or copied.
decision/cold-agent-session-isolated · Cold-agent session isolated to the escher MCP server — The cold agent runs from a per-run session directory outside the repository holding only an MCP config with the one `escher` stdio server, with no other tools, no setting sources, no slash commands and no session persistence, and nothing copied into it.
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar of 5 % — A cold-agent run fails `wrong-calls-over-bar` when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold. · guard: ColdAgentPipeTest
decision/telemetry-target-allowlist · Telemetry target allowlist — A record from an engine target prints only its safe fields, one from an escher_ target prints with its content-named fields redacted, and one from any other target is dropped whole, at every level and whatever RUST_LOG names. · guard: telemetry_scrub, telemetry_drop
contract/cold-agent-check/extraction · the cold agent's check · extraction — The check is the reply's last fenced json block, at most 65,536 bytes, a JSON array, written to `check.json` and judged.
contract/cold-agent-check/step · the cold agent's check · step — A step holds `tool` among the fifteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
contract/cold-agent-events/run-end · cold-agent events · run.end — A JSON line holding every verdict field, printed and appended to `events.jsonl`.
contract/cold-agent-events/run-start · cold-agent events · run.start — A JSON line `{ts, task}`, printed and appended to `events.jsonl`.
contract/cold-agent-events/status · cold-agent events · status — Prints the verdict (exit 0) or `{outcome: "none"}` (exit 3); printed only, never appended.
contract/cold-agent-exit-codes/0 · cold-agent exit codes · 0 — Success, on every verb.
contract/cold-agent-exit-codes/1 · cold-agent exit codes · 1 — The verb ran and failed (a `failed` verdict).
contract/cold-agent-exit-codes/3 · cold-agent exit codes · 3 — A precondition is unmet, before any state: `claude` or `python3` not on PATH, the `escher-session` binary absent or not executable (one stderr line naming the build command; the pipe builds nothing), `status` with no verdict, or `logs` with no `events.jsonl`.
contract/cold-agent-invocation/environment · cold-agent pipe invocation · environment — The cold-agent pipe binds no port or socket and reads no environment variable of its own; `RUST_LOG=info` is set on its server child only.
contract/cold-agent-state/directory · cold-agent state · target/cold-agent — The pipe's state is only `verdict.json`, `events.jsonl`, `transcript.jsonl`, `client.log`, `server.log`, `check.json` and `record/` under `target/cold-agent/`, gitignored.
contract/cold-agent-tasks/stand · cold-agent pipe tasks · stand — `stand` is the one task: seven apps (counter, flight booker, timer, CRUD, cells, the load page and the order page) in one client session; any other task, `run counter` included, is a usage error.
contract/cold-agent-verdict/counts-agree · cold-agent verdict · counts_agree — True when the server's tool-call span lines equal the escher tool uses and its refused plus error lines equal the escher tool results marked `is_error`.
contract/cold-agent-verdict/isolated · cold-agent verdict · isolated — True when the init tools are exactly the fifteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — Reasons come from `client-exit · no-result · not-isolated · counts-disagree · no-tool-call · task-unmet · capability-unused · recording-not-kept · wrong-calls-over-bar · no-check · check-failed · check-cannot-fail`; `no-check` and an invalid check's `check-failed` stand alone.
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — True when each accepted `start` left one kept check opening with its app's start record.
contract/cold-agent-verdict/tasks-met · cold-agent verdict · tasks_met — An app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`, a paged screen read as its parts joined, never from the reply.
contract/command-span/graph · command span · graph — A graph call's command line records verb alone under its tool_call parent.
contract/command-span/screenshot · command span · screenshot — A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
contract/cold-agent-events/cleanup · cold-agent events · cleanup — Prints `{outcome: "done"}`; printed only, never appended.
contract/cold-agent-exit-codes/2 · cold-agent exit codes · 2 — Usage: a missing or unknown verb, a wrong argument count, or a task outside the allowlist (`run counter` included); checked before any precondition.
contract/cold-agent-verbs/cleanup · cold-agent verbs · cleanup — Removes the cold-agent state directory alone, idempotent, leaving `target/agent-run/` untouched.
contract/harness-construction/from-component-alone · Harness construction · Harness::from_component_alone — Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-construction/vdom-alone · Harness construction · vdom_alone — vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot-order · seven_guis headless stand · stand::boot_order — stand::boot_order(HarnessOptions) boots the order page offline in TaskShell with the options given, the seventh served task's stand boot.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.
contract/tool-call-span/help · tool_call span · help — A help call, answered by the server, writes one tool_call line, tool="help" ending="accepted", with no command line beneath it.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/agent-run-events-content-free · Agent-run output carries no panic text — A failing agent run puts no captured panic text in stdout or the events file, and no event carries a scrub-set key; the raw cargo output goes only to the run log. · guard: test_agent_run.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/capability-statement-from-schema · Capability statement read from the schema — The capability statement names every verb and every refusal cause and states the schema's counts in words read from the schema's tables, none hand-copied. · guard: CapabilityStatementTest
decision/cold-agent-isolation · Cold-agent run isolated — A cold-agent run fails on an isolation breach: an extra tool, a second server, a working directory naming the root, or an API key source. · guard: ColdAgentPipeTest
decision/cold-agent-output-scrubbed · Cold-agent output scrubbed — A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text. · guard: ColdAgentPipeTest
decision/cold-agent-session-outside-root · Cold-agent session outside the root — A cold-agent session runs outside the repository root and is removed after, and its prompt names no stand id. · guard: ColdAgentPipeTest
contract/cold-agent-verdict/capability-unused · Cold-agent verdict · capability-unused — A run that leaves a counted capability unused, or whose `caused_by_kind` reads 0 for any of panic, error and log, fails `capability-unused`.
contract/cold-agent-verdict/recording-not-kept · Cold-agent verdict · recording-not-kept — A `start` that left no kept check fails `recording-not-kept`.
contract/driver-json/masked-value · Driver JSON answers · masked value — A masked node value is written as `MASKED_VALUE`.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fifteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
decision/cold-agent-tests-offline · Cold-agent pipe tested offline — The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary. · guard: ColdAgentPipeTest
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/coverage-line/shape · The coverage line · shape — One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.
contract/coverage-role/unreadable-check · The coverage role · unreadable check — A missing file, a non-check, a check past its bound and another app's check are each refused `malformed`.
contract/driver-argument-kinds/part · Driver argument kinds · ArgKind::Part — A part argument is a whole number from 1 to `MAX_PART`, 1000.
contract/driver-json/number · Driver JSON answers · number — A number is written as its shortest decimal, and as `null` when not finite.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/password · driver session · typed password — A password typed through the driver never leaves in an answer or as pixels.
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
contract/driver-log/default-level · the driver's log line · default level — At the default level the driver writes no line.
contract/driver-log/per-call-line · the driver's log line · per-call line — With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-session/run · driver session · run — A call run through the session returns after settle with the diff of the snapshots taken before and after it.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/advance-accepted-on-every-session · advance accepted on every session — The `advance` verb is accepted on every session, never refused for the session it addresses. · why: ratified
decision/caused-owner-pinned-as-graph-reads · Caused owner pinned as the UI graph reads it — A caused item carries the owner the UI graph reads and no other, so a page keyed by HTML ids under no keyed element reads no owner on its caused items, and a check pins the graph's reading rather than an asserted owner. · why: A plan stating an owner for such a page states nothing to pin. · guard: stand_order_served
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver and a file input's path occur nowhere in what a call returns, graph, actions and layout answers included, nor in a kept or an explored path's check, in both layout modes; the value reads the mask, and a typed password never leaves as pixels. · guard: stand_act_screenshot, stand_act_filled, stand_masks_on_new_answers
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/refusal-cause/covered · driver refusal causes · covered — A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
contract/refusal-cause/disabled · driver refusal causes · disabled — A call on a disabled target is refused `disabled`, the instance unchanged.
contract/refusal-cause/fonts-unavailable · driver refusal causes · fonts-unavailable — A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/refusal-cause/not-found · driver refusal causes · not-found — A call naming an id not on the screen is refused `not-found`, the instance unchanged.
contract/refusal-cause/off-screen · driver refusal causes · off-screen — A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
contract/refusal-cause/stale · driver refusal causes · stale — A call naming a stale id is refused `stale`, the instance unchanged.
contract/refusal-cause/unknown-verb · driver refusal causes · unknown-verb — A call naming no known verb is refused `unknown-verb`, the instance unchanged.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the seven served screens in both layout modes.
contract/session-run/screenshot · escher_driver Session · screenshot — A `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster, and changes nothing.
contract/session-run/type · escher_driver Session · type — A `type` replaces what a filled control holds and an empty text clears it; a masked control's written form holds the mask, never the typed text.
contract/session-state-directory/session-sock · the session state directory · session.sock — The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
decision/driver-log-fixed-words-only · Driver log lines hold fixed words and counts only — At `info` each call run through `Session::run` leaves exactly one `escher_driver` line of fixed words and counts, never the ids, names, values or typed text it handled; at the default level no driver line. · guard: stand_act_spans
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/listeners-read-from-vdom-walk · Listeners read from the vdom walk — A node's current listeners are read from the vdom walk's dynamic attributes, mounted included, not from the bridge's listener attributes. · why: The bridge's listener attributes over-report: a listener removed on a re-render leaves its attribute standing, and a mounted listener writes none. · guard: dioxus_runtime_probe link_2_a_removed_listeners_attribute_stays
decision/load-wait-bound · Acting steps wait out loads in flight — An acting step that meets a load in flight waits for it up to `LOAD_WAIT`, five seconds of wall time, settling again every 10 ms and moving no virtual time; at the bound it ends not settled with `busy` `Loads`. · guard: stand_load_wait
decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/session-state-owner-only · Owner-only session state — The session state directory is created with mode 0700 and its socket with mode 0600. · guard: stand_session_lifecycle
decision/sink-drops-third-party-targets · Sink drops third-party targets — The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it. · guard: telemetry_drop
contract/actions-command/argument · the actions command · argument — An argument to actions is refused malformed.
contract/graph-command/argument · the graph command · argument — An argument to graph is refused malformed with nothing run.
contract/mcp-role-stdout/protocol-lines · the mcp role's stdout · protocol lines — The mcp role writes protocol lines only on stdout.
contract/session-act/settle · escher_driver Session · act — A step run through `Session::act` returns settled, or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-host-stderr/service-name · session host stderr lines · service.name — Every line the escher-session host writes to stderr carries `service.name=seven_guis`.
contract/session-run/changed-node-states · escher_driver Session · acting call result — An acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step.
contract/session-run/diff · escher_driver Session · run — A call run through `Session::run` on a held session returns after settle with the diff of the snapshots taken before and after it.
contract/session-run/scroll · escher_driver Session · scroll — A `scroll` is refused for none of the three checks that stand in front of a click and says in its result whether its element is then in view.
contract/session-run/type-on-select · escher_driver Session · type on a select — A `type` on a single-choice select chooses the first option whose label starts with the text, its diff naming the select with `value` and the options whose `selected` moved.
contract/session-status/served · the session status · served — Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/real-mcp-client-outside-ci · Real MCP client only in the operator pass — A real MCP client drives escher-session mcp in the operator pass, not in CI.
decision/checks-byte-equal-to-recording · Committed checks byte-equal to the binary's recording — The committed checks are recorded by escher-session and read byte-equal to what the binary records.
decision/mcp-tested-as-child-and-in-process · MCP role tested as a child and in process — The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present
contract/cli-record/permissions · escher-session CLI recording · --record — `start <task> --record <dir>` keeps one `0600` check in a `0700` directory; a group-open recording directory is refused, status 3, with nothing spawned.
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.
contract/mcp-record/parity · escher-session MCP role · --record — With `--record` the tool listing and `help` are the same bytes as without it, and the kept check is byte-identical to the command line's.
contract/mcp-tool-result/screenshot · escher-session MCP tool results · screenshot — A `screenshot` tool call answers an `image/png` image block then a text block, `isError` false.
contract/cli-coverage-role/stdout · escher-session CLI coverage role · coverage line — Replaying a kept check prints the handlers it left in one coverage line on stdout, status 0.
contract/cli-exit-codes/1 · escher-session CLI exit codes · 1 — A call a target cannot take is refused with its cause.
contract/cli-snapshot/part · escher-session CLI snapshot · --part — A part read by number answers one uncoloured JSON line, status 0; a part of 0, 1001 or a non-number exits 1 with the `malformed` refusal line, nothing typed written back.
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/mcp-protocol/revision · escher-session MCP role · protocol revision — The role speaks the newest MCP protocol revision.
contract/mcp-snapshot/parity · escher-session MCP role · snapshot — A fresh Counter's snapshot reads the same through MCP and the command line.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fifteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/graph-drives-confirm-every-write · Shared graph drives exercise every stand write — The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag. · why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
contract/driver-advance/advance · the driver's advance command · advance — Moves the session's clock by exactly ms inside one settled step.
contract/driver-advance/advanced-ms · the driver's advance command · advanced_ms — Is exactly ms, the clock's move, not what the Timer's counter gained.
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/advance-moves-clock-by-exactly-ms · Advance moves the virtual clock by exactly ms — The driver's advance moves the session's clock, the harness's VirtualClock that every harness-built document reads, by exactly ms inside one settled step, and advanced_ms reports exactly ms, not what the Timer's counter gained. · why: ratified · guard: stand_act_timer
decision/advance-reports-clock-move · advance reports the clock's move — advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained. · why: Ratified.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/sink-capture-counted-by-target · Sink capture counted by target — A check that reads a sink's capture counts lines by target, never in total. · why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/advance-moves-clock-exactly · advance moves the clock exactly — Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed. · why: Ratified.
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/screenshot-in-band · Screenshot pixels in band — A screenshot's pixels cross in band, a PNG as padded base64 inside the answer's one JSON line `{"width":…,"height":…,"png":…}`, under the unchanged answer bound. · why: Ratified; the largest whole-screen answer on the stand uses 2.1 % of the bound.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/check-replay/another-app · Check replay · another app — A check replayed on a session whose label is not the check's app runs nothing.
contract/check-replay/stop · Check replay · stop — Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/payload-unread · Driver session · <payload unread> — A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
contract/driver-session/refusal-order · Driver session · refusal order — A `click` or `type` on an element that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/driver-session/screenshot · Driver session · screenshot — Returns a PNG of the whole screen, or of an element's bounds cropped to the visible part and rounded outward, changing nothing; a crop of no pixel is `off-screen`, an app with no font to paint text `fonts-unavailable`.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/driver-session/stale-or-not-found · Driver session · stale · not-found — An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
contract/driver-session/type-replaces · Driver session · type — After the focusing click, where the snapshot reads a value for the target, the driver selects all and types, so an empty text clears the control; a select is focused with no click and no DOM focus event.
contract/escher-session-command-line/mcp · escher-session command line · mcp — The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
contract/escher-session-command-line/mcp-record-check · escher-session command line · mcp --record — `mcp --record` checks the recording directory before it serves; a refusal is status `3`, its fixed message on stderr and stdout empty.
contract/escher-session-command-line/record-flag · escher-session command line · --record — `--record <dir>` is taken at most once, on `start`, the host role and the `mcp` role; anywhere else, twice, or with no value it is usage, status `2`.
contract/session-recording/bounds · Session recording · bounds — Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and the failure reads true once; the call is not rolled back.
contract/session-recording/check-file · Session recording · check file — `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.
contract/session-recording/check-read · Session recording · Check::from_bytes — A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
contract/session-recording/directory · Session recording · directory — Unix only, `Unsupported` elsewhere; the directory is created `0700` or refused `RecordDirNotPrivate`.
contract/session-recording/record-fields · Session recording · record fields — Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
decision/two-clicks-without-advance-double-click · Two clicks with no advance are a double click — Under the virtual clock, two clicks with no `advance` between them are a double click. · why: Ratified. · guard: stand_virtual_clock
contract/driver-session/act · Driver session · act — Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
contract/driver-session/id-record · Driver session · record of ids — Holds id text only, at most 4096 ids, for the session's life, no id longer than `MAX_ID_BYTES`, read by nothing outside the crate.
contract/driver-session/load-wait · Driver session · LOAD_WAIT — A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
contract/driver-session/run-validation · Driver session · run — A call is validated first, and a refused call has run nothing.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
contract/escher-session-command-line/lint · escher-session command line · lint — `lint` takes no argument; its accepted answer is one line `{"findings":[…]}`, status `0`.
contract/escher-session-command-line/no-echo · escher-session command line · caller input — No ending writes back anything the caller typed.
contract/escher-session-command-line/screenshot · escher-session command line · screenshot — `--id` is optional; the accepted answer is one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`.
contract/escher-session-command-line/snapshot · escher-session command line · snapshot — `--part` is a whole number 1 to 1000; a paged answer is one line `{"text":…,"part":n,"parts":m}`, status `0`; a whole screen still answers `{"text":…}` alone.
contract/escher-session-command-line/start · escher-session command line · start — `start <task> [--record <dir>]`, the task one of the seven served tasks; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
contract/escher-session-command-line/usage-line · escher-session command line · usage line — Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, with `graph` after `lint`, `layout --id <id>` after `actions` and `help` after `stop`.
contract/session-errors/record-dir-not-private · Session errors · RecordDirNotPrivate — A recording directory open to its group or others.
contract/session-errors/recording-failed · Session errors · RecordingFailed — A recording that ended, read from an `ok v3 recording-failed` reply.
contract/session-wire/answer-bound · Session wire · answer bound — An answer is at most 1,048,576 bytes on its JSON line.

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
