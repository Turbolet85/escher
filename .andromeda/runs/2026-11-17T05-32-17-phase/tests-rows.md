# test-plan · the rows that bind 2026-11-17-a-component-in-isolation — picked 87 of 619 (38 by every draw) · rule any · pin 1646

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/dedicated-headless-test-harness · Dedicated headless test harness — Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
decision/driver-executor-integration-tested · Driver executor tested by integration checks — The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests. · why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/one-test-target-per-behaviour · One integration test target per behaviour — The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target. · why: differs from upstream, whose layout is one all target · guard: BlitzTestsTargetsTest
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
contract/mcp-server/tools-list · MCP server · tools/list — tools/list offers the fifteen verbs in order, `layout` after `actions` and `help` last.
decision/blitz-tests-one-target-per-file · One blitz-tests target per file — The blitz-tests manifest holds one test target per file, with upstream's all target left unbuilt. · guard: BlitzTestsTargetsTest
decision/driver-instance-verbs-tested-by-stand · Driver verbs over a held instance tested by integration — The driver's executor verbs and its readings over a held instance are covered by integration checks, not unit tests, and its JSON writer's node rows are unit-tested over plain parts. · why: The driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance.
decision/one-virtual-clock · One virtual clock drives engine time — The scrollbar fade, double-click timing and smooth scroll move only with the document's clock, which a session moves by advance alone; a document built with no clock runs by the wall clock. · why: ratified · guard: virtual_clock_engine, stand_virtual_clock
decision/snapshot-text-budget · Snapshot text within a byte budget — A screen's snapshot text is read whole when inside SNAPSHOT_TEXT_BUDGET (10,000 bytes); a larger screen is read in parts, each within the budget, the parts concatenating to the whole text. · guard: stand_snapshot_text, stand_snapshot_parts
contract/driver-command-span/one-line-per-call · driver command span · command line — Each call run through the session leaves exactly one escher_driver line at INFO, its span reading command, carrying no field outside the stated eight and none of the ids, names or supplied texts.
contract/escher-session-stderr/no-ids-names-or-typed-text · escher-session stderr · ids, names and typed text — Even at trace level, stderr holds no stable id, no accessible name, no typed text and no screenshot bytes.
contract/escher-session-stderr/tool-call-line · escher-session stderr · tool_call — The mcp role writes one tool_call line per call, holding only its stated field names.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/stand-checks-bundled-font-no-skip · Stand checks on the bundled font, no skip — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
decision/real-page-snapshots-paired-reduced · Real-page snapshots paired with reduced variants — Full real-page snapshot fixtures are paired with reduced variants.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/harness-driven-tests · Harness-driven engine tests — Engine tests drive any document through the test harness, which supplies deterministic construction defaults, a pump and tick loop, a settle loop repeated until no work is due, inspection helpers and input synthesis.
decision/stand-checks-bundled-font · Stand checks under a bundled font — The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans, with system fonts off. · guard: stand_bundled_font_families · except: The pre-existing font-dependent tests skip at runtime when no usable font is available.
decision/stable-tree-serialization · Stable tree serialization — The harness produces a stable one-node-per-line tree serialization with geometry. · why: Suitable for snapshot-style assertions.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/cold-agent-events-content-free · Cold-agent events and verdict carry no content — No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one. · guard: the scrub-set key check and four planted markers under the shim
decision/cold-agent-live-run-off-ci · Live cold-agent run never in CI — The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests. · why: no CI secret
decision/cold-agent-session-isolated · Cold-agent session isolated to the escher MCP server — The cold agent runs from a per-run session directory outside the repository holding only an MCP config with the one `escher` stdio server, with no other tools, no setting sources, no slash commands and no session persistence, and nothing copied into it.
decision/cold-agent-verbs-without-layout · Cold-agent pipe's verb list without layout — The cold-agent pipe's verb list holds the fourteen driver verbs other than `layout`. · why: The conservative option of an open fork, applied ahead of its ratification. · provisional
contract/cold-agent-check/step · the cold agent's check · step — A step holds `tool` among the fourteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
contract/cold-agent-invocation/environment · cold-agent pipe invocation · environment — The cold-agent pipe binds no port or socket and reads no environment variable of its own; `RUST_LOG=info` is set on its server child only.
contract/cold-agent-verdict/isolated · cold-agent verdict · isolated — True when the init tools are exactly the fourteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
contract/harness-construction/from-vdom · Harness construction · Harness::from_vdom — Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
contract/stand-boot/boot · seven_guis headless stand · stand::boot — stand::boot(LeanTask, HarnessOptions) boots a lean task (Counter, FlightBooker, Timer, Crud) as a fresh VirtualDom over from_vdom, rendering the task in TaskShell.
contract/stand-boot/options · seven_guis headless stand · stand::options — stand::options(incremental) gives the pinned harness options the stand boots with.
decision/agent-run-holds-no-process · Agent-run holds no process — The agent-run script starts, polls and listens on nothing (no daemon, PID file, status endpoint, port or socket) and reads no environment variable of its own; the stand checks it runs boot in process. · except: stand_session_lifecycle, stand_session_quiet and the session selection's cli_flow flows start and stop a session host process and its socket, a surface of the session library, not of this contract
decision/closed-spans-logged-as-lines · Closed spans logged as lines — The stand log carries closed spans its allowlist admits as a second record class beside printed events. · why: a boundary widening, ratified, as the obs-plan's log format states
decision/cold-agent-passes-on-positive-evidence · Cold-agent run passes only on positive evidence — A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all six apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
decision/cold-agent-server-log-counted-only · Cold-agent server log counted, never shown — The server's log is read only to count its tool-call span lines, their refused and error endings and each counted verb's accepted lines; it is never printed or copied.
decision/empty-run-is-never-a-pass · An empty run is never a pass — A run's outcome is passed only when cargo exited 0, no test failed and at least one test line parsed; otherwise it is failed, exit 1.
decision/no-ids-or-names-in-stand-logs · No ids or names in stand logs — The stand binaries' stderr holds no stable id and no accessible name at any level up to trace, and the session host's none of the text typed. · guard: host_log, mcp_log, stand_act_spans
decision/scrolled-box-reads-where-it-stands · A scrolled box reads where it stands — For a box that is itself scrolled, layout_rect, layout_rect_of and center_of read where the box stands at every scroll offset of its own, in both layout modes, as the client bounding rect and the snapshot's bounds do. · why: ratified · guard: scrolled_box_absolute_position
decision/shared-stand-tables-in-one-module · Shared stand tables in one module — The tables and helpers the stand checks share are stated once in one shared test module with no test and no test target of its own; the tables are stated expectation, never a second role or name mapping. · except: stand_element_ids keeps its own boot (it books the flight), and task_keys, ceiling, component, the cards array and each file's SECRET stay per-file
contract/cold-agent-events/run-end · cold-agent events · run.end — A JSON line holding every verdict field, printed and appended to `events.jsonl`.
contract/cold-agent-events/status · cold-agent events · status — Prints the verdict (exit 0) or `{outcome: "none"}` (exit 3); printed only, never appended.
contract/command-span/verb · command span · verb — verb and tool_call's tool are closed sets of words, help, screenshot and graph among them, added with no span field.
contract/harness-construction/wrap · Harness construction · wrap — Harness wrap builds a harness without pumping.
contract/harness-inspect/layout-rect · Harness inspection helpers · layout_rect — layout_rect(selector) returns a rect with x, y, width and height.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/cold-agent-isolation · Cold-agent run isolated — A cold-agent run fails on an isolation breach: an extra tool, a second server, a working directory naming the root, or an API key source. · guard: ColdAgentPipeTest
decision/cold-agent-output-scrubbed · Cold-agent output scrubbed — A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text. · guard: ColdAgentPipeTest
decision/cold-agent-tests-offline · Cold-agent pipe tested offline — The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary. · guard: ColdAgentPipeTest
contract/element-id/virtual-form · Element ids · virtual form — The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
contract/snapshot-node/clickable · Snapshot node · clickable — `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.
decision/bridge-tests-child-module · Bridge tests a child of the dioxus document — The dioxus document's bridge tests sit in a child module of the dioxus document module. · why: The event handler's two fields are private to that module and it has no constructor.
decision/no-change-read-control-needs-pending-work · Control of a no-change read needs pending work — A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen. · why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
decision/one-actionable-rule · One actionable rule for both readers — Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark. · why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener. · guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
contract/cold-agent-verdict/capability-unused · Cold-agent verdict · capability-unused — A run that leaves a capability unused fails `capability-unused`.
contract/driver-verbs/verb-table · Driver verbs · verb table — Fifteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/session-writes-no-screen-content · Session writes nothing of the screen — The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
decision/host-stderr-no-screen-content · Host stderr carries no screen content at trace — A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included. · guard: host_log
contract/driver-session/refused-call · driver session · refused call — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
contract/driver-verbs/graph · driver verbs · graph — `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/graph-signal-registration · Task state registered for the UI graph — The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change. · why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private. · guard: graph::tests, stand_graph
decision/refusal-leaves-instance-unchanged · Refusals change nothing and echo no input — A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
decision/reserved-graph-id-form · Reserved owner-key id form — No served id holds the reserved `{owner}///{key}` form. · guard: stand_graph
decision/stated-tolerance-read-inclusively · Stated tolerance read inclusively — A tolerance stated as to 1/64 px is read inclusively: the absolute difference of the two values is at most 1/64. · why: A sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
contract/session-layout/changes-nothing · escher_driver Session · layout — A layout read, in process or through the `layout` verb, changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
contract/session-run/layout-answer · escher_driver Session · layout verb answer — The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the six served screens in both layout modes.
decision/graph-answer-holds-no-value · Graph answer holds no value — A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text. · guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
decision/session-logs-no-screen-content · Session logs nothing of the screen — The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level. · guard: stand_session_quiet, host_log, mcp_log
contract/session-run/graph · escher_driver Session · graph — The `graph` verb answers the document's UI graph, every graph node a snapshot node, changing nothing and holding no typed text.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/real-mcp-client-outside-ci · Real MCP client only in the operator pass — A real MCP client drives escher-session mcp in the operator pass, not in CI.
decision/ui-driven-through-session-binary · App UI driven through the session binary — No test in the apps launches a window; the UI is driven through the escher-session binary.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
contract/mcp-tools-list/order · escher-session MCP role · tools/list — `tools/list` offers the fifteen verbs in order, `graph` after `lint`, `actions` after `graph`, `layout` after `actions` and `help` last.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/harness-counts-fetching-providers-only · Harness counts loads of fetching providers only — Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads. · why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
decision/stand-boots-offline · Stand boots offline — Every headless stand boot keeps no network provider, and no real network is used by the stand. · guard: the_stand_options_stay_offline · except: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.
decision/wrapped-document-keeps-own-clock · Wrapped document keeps the clock its config gave — Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it. · why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/stand-pinned-real-font · Stand checks keep a real pinned font — The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
decision/stand-timer-ticks · Stand timer driven by TimerTicks — Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.
decision/settle-never-advances-time · Settle never advances time — Settle neither advances the timer nor waits on a tick not yet delivered.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/timer-driven-by-delivered-ticks · Timer driven by delivered ticks — In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick. · guard: stand_timer, stand_settle

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/bounds-reader-bodies-each-checked · Each bounds-reader body holds a check — Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own. · why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner. · guard: scrolled_box_client_rect
decision/font-assertions-on-system-fonts · Font-dependent assertions on system fonts — Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested. · except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/a11y-leg-membership · a11y leg membership — The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap. · why: Chosen over running the tier-0 witnesses alone.

## session-lifecycle · key of ## 3. Test Harness Contract
decision/socket-carries-commands · The socket carries commands — The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, lint findings and layout answers out. · why: Ratified, where until then none of them crossed.
contract/driver-session/graph · Driver session · graph — Returns `Outcome::Graphed` holding the screen's `UiGraph`.
contract/driver-session/reading-verbs · Driver session · reading verbs — `snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.
contract/driver-session/snapshot · Driver session · snapshot — Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
contract/session-wire/graph-answer · Session wire · graph answer — Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
decision/replay-through-the-call-path · Replay through the ordinary call path — A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type. · why: Ratified.
contract/driver-session/run-carriers · Driver session · run carriers — The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
contract/escher-session-command-line/command-form · escher-session command line · command form — `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` bare).
contract/escher-session-command-line/usage-line · escher-session command line · usage line — Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app> <check>...`, with `graph` after `lint`, `layout --id <id>` after `actions` and `help` after `stop`.

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
