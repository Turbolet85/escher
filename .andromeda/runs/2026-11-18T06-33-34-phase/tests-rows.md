# test-plan · the rows that bind 2026-11-18-cold-agent-bar — picked 132 of 627 (64 by every draw) · rule any · pin 2041

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/eight-refusal-causes-in-order · Eight refusal causes in a fixed order — The driver's refusal causes are eight, named in a fixed order with `fonts-unavailable` last, each stating its meaning and a distinct remedy, and a refusal reads as its cause's name, then its remedy. · why: Ratified. · guard: the_eight_causes_are_named_in_order
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.
contract/driver-refusal-causes/fonts-unavailable · driver refusal causes · fonts-unavailable — A screenshot or lint on an app with no usable font is refused fonts-unavailable with nothing changed.
contract/driver-refusal-causes/malformed · driver refusal causes · malformed — An id longer than 1024 bytes is refused malformed before any lookup.
contract/driver-refusal-causes/not-found · driver refusal causes · not-found — An id no screen of the session read, or that no element carries, is refused not-found.
contract/driver-refusal-causes/stale · driver refusal causes · stale — An id an earlier screen of the session read that names no element now is refused stale.
contract/mcp-server/actions-argument · MCP server · actions argument — An actions tool call given an argument is answered isError.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
contract/driver-refusals/fonts-unavailable · Driver refusals · fonts-unavailable — A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path, and the actions read names the same fonts-unavailable for screenshot.
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/mcp-capture-counts-by-span · MCP child capture counted by span — A check capturing the mcp child counts its log lines by span and target, each tool call span the parent of that call's command line.
decision/requirement-witness-inside-stand-repeat · Every requirement witnessed inside stand-repeat — Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red. · guard: stand_requirement_sweep · except: A requirement with no screen names no witness.
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.
decision/engine-items-asserted-on-tracing-build · Engine caused items asserted on a tracing build only — A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds. · guard: stand_act_error_text
decision/handler-coverage-by-replay · Handler coverage measured by replay — Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so recorded checks also measure what they leave unexercised.
decision/pinned-viewport-read-from-document · Pinned viewport read from the document — A boot-condition check of the pinned viewport reads the document viewport, never the root element's bounds. · why: A component alone has no shell CSS, so its root element sizes to its content.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
contract/escher-session-commands/coverage · escher-session commands · coverage <app> <check>... — Lists the handlers no given recorded check exercised.
contract/handler-coverage-limits/bubbling-dispatch · Handler coverage limits · bubbling dispatch — A bubbling dispatch counts every ancestor listener it passed.
contract/handler-coverage-limits/canvas-handler · Handler coverage limits · canvas handler — A handler inside a canvas is not counted.
contract/handler-coverage-limits/unreached-screen · Handler coverage limits · unreached screen — A handler on no screen a replay reached is not listed.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/cold-agent-events-content-free · Cold-agent events and verdict carry no content — No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one. · guard: the scrub-set key check and four planted markers under the shim
decision/cold-agent-live-run-off-ci · Live cold-agent run never in CI — The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests. · why: no CI secret
decision/cold-agent-passes-on-positive-evidence · Cold-agent run passes only on positive evidence — A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all six apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
decision/cold-agent-prompt-names-no-ids · Cold-agent prompt gives no element ids or app slugs — The fixed stand prompt asks for the six apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
decision/cold-agent-server-log-counted-only · Cold-agent server log counted, never shown — The server's log is read only to count its tool-call span lines, their refused and error endings and each counted verb's accepted lines; it is never printed or copied.
decision/cold-agent-session-isolated · Cold-agent session isolated to the escher MCP server — The cold agent runs from a per-run session directory outside the repository holding only an MCP config with the one `escher` stdio server, with no other tools, no setting sources, no slash commands and no session persistence, and nothing copied into it.
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar of 5 % — A cold-agent run fails `wrong-calls-over-bar` when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold. · guard: ColdAgentPipeTest
contract/cold-agent-events/run-end · cold-agent events · run.end — A JSON line holding every verdict field, printed and appended to `events.jsonl`.
contract/cold-agent-invocation/environment · cold-agent pipe invocation · environment — The cold-agent pipe binds no port or socket and reads no environment variable of its own; `RUST_LOG=info` is set on its server child only.
contract/cold-agent-tasks/stand · cold-agent pipe tasks · stand — `stand` is the one task: six apps (counter, flight booker, timer, CRUD, cells and the load page) in one client session; any other task, `run counter` included, is a usage error.
contract/cold-agent-verdict/counts-agree · cold-agent verdict · counts_agree — True when the server's tool-call span lines equal the escher tool uses and its refused plus error lines equal the escher tool results marked `is_error`.
contract/cold-agent-verdict/isolated · cold-agent verdict · isolated — True when the init tools are exactly the fifteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — Reasons come from `client-exit · no-result · not-isolated · counts-disagree · no-tool-call · task-unmet · capability-unused · recording-not-kept · wrong-calls-over-bar · no-check · check-failed · check-cannot-fail`; `no-check` and an invalid check's `check-failed` stand alone.
contract/cold-agent-verdict/tasks-met · cold-agent verdict · tasks_met — An app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`, a paged screen read as its parts joined, never from the reply.
contract/command-span/graph · command span · graph — A graph call's command line records verb alone under its tool_call parent.
contract/command-span/screenshot · command span · screenshot — A shot's command line records verb, and cause when refused, never the image, its size or its dimensions.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/stand-log/command-span · Stand log · command span — escher-session's host role writes one closed command span line under escher_driver per call it runs through Session::run, the same with a recording held.
contract/stand-log/span-line · Stand log · span line — One line per closed span the allowlist admits: the event prefix, then span={name}, the span's fields in recorded order, message="close", time.busy and time.idle; no line when a span is created, recorded to, entered or exited.
contract/stand-log/stream · Stand log · stream — The two seven_guis binaries, seven_guis_native and escher-session, write their log to stderr only, with no ANSI.
contract/stand-log/tool-call-span · Stand log · tool_call span — The mcp role writes one closed tool_call span line under escher_driver per tools/call request, carrying only tool and ending, parent of the command line of a call run through Session::run.
contract/tool-call-span/ending · tool_call span · ending — ending is one of accepted, refused or error.
decision/cold-agent-replayed-check · Cold agent's check judged by replay — The agent's JSON check is replayed in full on a fresh driver MCP child and must pass, then replayed ablated (its acting steps left out) on a second fresh child and must fail; a malformed check is never replayed. · why: ratified as the chosen option, a JSON check with replay
decision/component-alone-props-as-maker · Props of a component alone given as a maker — Props of a component alone are given as a maker run once in the mount's root scope, never as a value. · why: An EventHandler prop can only be made inside a Dioxus runtime.
decision/component-alone-root-context-before-build · Root context of a component alone provided before the build — A root context for a component alone is provided on the mount before the harness builds. · why: A context added after the build reaches no render.
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
contract/cli-command/stderr · escher-session commands · stderr — An accepted command's stderr is empty with RUST_LOG unset.
contract/cli-command/stdout · escher-session commands · stdout — A command's stdout is its answer and nothing else: one line of JSON, with no ESC byte.
contract/cold-agent-check/extraction · the cold agent's check · extraction — The check is the reply's last fenced json block, at most 65,536 bytes, a JSON array, written to `check.json` and judged.
contract/cold-agent-check/step · the cold agent's check · step — A step holds `tool` among the fifteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
contract/cold-agent-exit-codes/0 · cold-agent exit codes · 0 — Success, on every verb.
contract/cold-agent-exit-codes/1 · cold-agent exit codes · 1 — The verb ran and failed (a `failed` verdict).
contract/cold-agent-exit-codes/2 · cold-agent exit codes · 2 — Usage: a missing or unknown verb, a wrong argument count, or a task outside the allowlist (`run counter` included); checked before any precondition.
contract/cold-agent-invocation/windows-entry · cold-agent pipe invocation · windows entry — The Windows entry of the cold-agent pipe is a pass-through with no logic of its own.
contract/harness-construction/from-component-alone · Harness construction · Harness::from_component_alone — Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope; it rides from_vdom and adds no carrier.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/harness-construction/vdom-alone · Harness construction · vdom_alone — vdom_alone(component, props maker) is the harness's public mount of a component alone, a VirtualDom whose root context a caller may add before the harness builds.
contract/harness-input/input-helpers · Harness input helpers · input helpers — Each input helper (click, tap, drag, press, type_text, wheel_at, scroll_into_view, ime and the rest) pumps after dispatch.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.
contract/mcp-role/stdout · escher-session mcp role · stdout — The mcp role, a stdio child with no socket, writes only its protocol lines to stdout, with no ESC byte.
contract/stand-boot/boot-alone · seven_guis headless stand · stand::boot_alone — stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.
contract/stand-log/event-line · Stand log · event line — One line per printed event: {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} then {field}={value} pairs.
contract/tool-call-span/help · tool_call span · help — A help call, answered by the server, writes one tool_call line, tool="help" ending="accepted", with no command line beneath it.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/cold-agent-isolation · Cold-agent run isolated — A cold-agent run fails on an isolation breach: an extra tool, a second server, a working directory naming the root, or an API key source. · guard: ColdAgentPipeTest
decision/cold-agent-output-scrubbed · Cold-agent output scrubbed — A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text. · guard: ColdAgentPipeTest
decision/cold-agent-session-outside-root · Cold-agent session outside the root — A cold-agent session runs outside the repository root and is removed after, and its prompt names no stand id. · guard: ColdAgentPipeTest
decision/cold-agent-tests-offline · Cold-agent pipe tested offline — The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary. · guard: ColdAgentPipeTest
decision/expected-value-pinned-as-literal · Expected value pinned as a literal — A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal. · why: A check computing its expectation from the mutated constant reads green under the mutation.
contract/cold-agent-verdict/capability-unused · Cold-agent verdict · capability-unused — A run that leaves a capability unused fails `capability-unused`.
contract/refusal-causes/cause-set · Refusal causes · cause set — A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/cold-agent-verdict/recording-not-kept · Cold-agent verdict · recording-not-kept — A `start` that left no kept check fails `recording-not-kept`.
contract/coverage-line/shape · The coverage line · shape — One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
contract/driver-session/held-state · driver session · held state — One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/root-boundary-err-on-own-session · Root-boundary Err checked on its own session — A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain. · why: The root boundary replaces the app and renders the error on screen. · guard: stand_act_errors
contract/mcp-role-stderr/tool-call · the mcp role's stderr · tool_call — One tool_call line per call, carrying only its stated field names.
contract/refusal-cause/covered · driver refusal causes · covered — A target a hit at its centre does not reach is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
contract/refusal-cause/disabled · driver refusal causes · disabled — A call on a disabled target is refused `disabled`, the instance unchanged.
contract/refusal-cause/fonts-unavailable · driver refusal causes · fonts-unavailable — A call that needs fonts the instance lacks is refused `fonts-unavailable`, the instance unchanged.
contract/refusal-cause/malformed · driver refusal causes · malformed — A malformed call is refused `malformed`, the instance unchanged.
contract/refusal-cause/not-found · driver refusal causes · not-found — A call naming an id not on the screen is refused `not-found`, the instance unchanged.
contract/refusal-cause/off-screen · driver refusal causes · off-screen — A target outside its scrolling box's view, or an element shot cropping to no pixel, is refused `off-screen`, the instance unchanged.
contract/refusal-cause/stale · driver refusal causes · stale — A call naming a stale id is refused `stale`, the instance unchanged.
contract/refusal-cause/unknown-verb · driver refusal causes · unknown-verb — A call naming no known verb is refused `unknown-verb`, the instance unchanged.
contract/session-host-stderr/command-span · the session host's stderr · command span — Exactly one command span line is written per call.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
decision/actions-answer-holds-no-value · Actions answer holds no value — An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field. · guard: stand_actions
decision/advance-accepted-on-every-session · advance accepted on every session — The `advance` verb is accepted on every session, never refused for the session it addresses. · why: ratified
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
decision/typed-password-never-returned · Typed passwords never returned — A password typed through the driver and a file input's path occur nowhere in what a call returns, graph, actions and layout answers included, nor in a kept or an explored path's check, in both layout modes; the value reads the mask, and a typed password never leaves as pixels. · guard: stand_act_screenshot, stand_act_filled, stand_masks_on_new_answers
contract/actions-command/argument · the actions command · argument — An argument to actions is refused malformed.
contract/graph-command/argument · the graph command · argument — An argument to graph is refused malformed with nothing run.
contract/session-host-stderr/service-name · session host stderr lines · service.name — Every line the escher-session host writes to stderr carries `service.name=seven_guis`.
contract/session-run/screenshot · escher_driver Session · screenshot — A `screenshot` returns the screen, or the element its optional id names, as a PNG equal to the harness's raster, and changes nothing.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/real-mcp-client-outside-ci · Real MCP client only in the operator pass — A real MCP client drives escher-session mcp in the operator pass, not in CI.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/command-span/actions · driver command span · actions — An `actions` tool call parents one `command` span that records `verb` alone.
contract/mcp-tool-result/refused · escher-session MCP tool results · refused call — A refused tool call answers a single text block, `isError` true.
decision/select-list-painted-by-paint-scene · Select list painted by the scene painter alone — The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own. · why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present
contract/mcp-actions/argument · escher-session MCP role · actions — The `actions` tool call answers its line of the actionable nodes; an argument to `actions` reads `isError`.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/sink-capture-counted-by-target · Sink capture counted by target — A check that reads a sink's capture counts lines by target, never in total. · why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/log-silence-read-on-a-sink-host · Log silence read on a host with a sink — A zero read from a host that installs no log sink says nothing about a host that does.
contract/driver-session/acting-verbs · Driver session · acting verbs — `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after, with `caused` where the step caused something; `scroll` adds `in_view`.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
contract/driver-session/refusal-order · Driver session · refusal order — A `click` or `type` on an element that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/driver-session/stale-or-not-found · Driver session · stale · not-found — An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
contract/escher-session-command-line/graph · escher-session command line · graph — `graph` takes no argument (an argument is refused `malformed`); its accepted answer is one line `{"nodes":[…],"states":[…]}`, status `0`.
contract/escher-session-command-line/mcp · escher-session command line · mcp — The `mcp` role is `mcp` alone or `mcp --record <dir>` (anything else is usage): an MCP server on the binary's standard streams holding its session in process, ending with nothing on stdout, `0` on `Ok`, `3` with the error's fixed message on stderr on `Err`.
contract/escher-session-command-line/start · escher-session command line · start — `start <task> [--record <dir>]`, the task one of `counter` · `flight-booker` · `timer` · `crud`; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/not-settled-result · Driver session · settled — A verb on an instance that did not go quiet is a result, `settled` false with its `busy` class, and is not rolled back.
contract/driver-session/payload-unread · Driver session · <payload unread> — A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
contract/driver-session/screenshot · Driver session · screenshot — Returns a PNG of the whole screen, or of an element's bounds cropped to the visible part and rounded outward, changing nothing; a crop of no pixel is `off-screen`, an app with no font to paint text `fonts-unavailable`.
contract/driver-session/session-verb-in-run · Driver session · session-level verb — `start`, `status` or `stop` handed to the instance call path is refused `unknown-verb`.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/usage-line · escher-session command line · usage line — Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, with `graph` after `lint`, `layout --id <id>` after `actions` and `help` after `stop`.
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.

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
