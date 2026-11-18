# architecture · the rows that bind 2026-11-18-served-error-page — picked 287 of 1532 (160 by every draw) · rule any · pin 2112

## design-philosophy · ## Design Philosophy
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.

## stack-and-technologies · ## Stack and Technologies
decision/cli-flow-under-plain-sh · CLI flow tests under plain sh — seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement. · why: no dependency, manifest line or test framework is added for it
decision/cold-agent-client-host-only · Cold-agent client is a host tool — The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
decision/cold-agent-judge-stdlib-python · Cold-agent judge on stdlib Python — The cold-agent pipe's judge is python3 on its standard library, with no package added. · why: ratified
decision/cold-agent-mcp-server-is-own · Cold-agent MCP server is the workspace's own — The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds

## established-decisions · ## Established Decisions
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a multiple select's row focuses its select
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified.
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/damage-bits-drive-relayout · Damage bits decide relayout — Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/dioxus-no-own-stylesheet · Dioxus document adds no stylesheet — A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching.
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/session-library-depends-on-no-app · Session library depends on no app — The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/mcp-stdio/result · the mcp role's stdio · result — A result is at most 1,048,576 bytes.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.

## conventions · ## Conventions
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
decision/capture-before-own-recorder · Capture opened before a check's own recorder — A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder. · why: So no call site's interest is cached from a thread with no subscriber.
decision/const-declared-writers · Task writers written as const — A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item. · why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter.
decision/dioxus-example-shape · Dioxus example shape — Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings.
decision/mutex-poison-recovery · Mutex locks recover from poisoning — Mutex locks recover from poisoning by taking the inner value.
decision/reactive-state-store-derive · Reactive state derives Store — Reactive state in apps and examples uses the Store derive.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result. · why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees.
decision/store-reactive-state · Store-derived reactive state — Reactive app state uses `#[derive(Store)]` with `#[store]` impls.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.

## standard-contracts · ## Standard Contracts
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts.
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py
decision/cold-agent-six-apps · Cold-agent prompt names six apps — The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id.
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
contract/argument-kinds/name · argument kinds · name — 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cold-agent-recording/live-server · cold-agent recording · live server child — The cold-agent live server child records into the owner-only `target/cold-agent/record/`; its two replay children run without recording.
contract/cold-agent-verdict/passed · cold-agent verdict.json · passed — The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all six apps met, and the agent's final check replaying passing and failing on an ablated replay.
contract/command-line/mcp · command line · mcp — mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
contract/command-line/serve · command line · serve — serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
contract/command-line/start · command line · start — start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dioxus-native-dom-api/root-error-mark · dioxus-native-dom public API · root_error_mark and RootErrorMark — Public and re-exported: root_error_mark reads the root error boundary's error context; RootErrorMark compares by identity and its Debug prints its name alone.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/driver-forms/is-select · driver form-control readers · is_select — is_select is read by the driver's locate alone.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role.
contract/escher-session-cli/serve-role · escher-session command line · serve — escher-session has a serve host role.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/lint · escher-session command line · lint — The verb lint takes no argument beside --session.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
contract/escher-session/serve · escher-session command line · serve <task> --session <dir> [--record <dir>] — The host role, which the start command spawns.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role.
contract/graph-registry/declared-write · Graph registry · DeclaredWrite — Names an element's stable id or a prefix ending in *, and an event written without on.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page and one component alone in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the eleven instance verbs are kept, `lint`, `graph`, `actions` and `layout` among them with their answers whole; any other call is not kept.
contract/lint-verb/clipped-off-screen · The lint verb · clipped and off-screen — `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
contract/lint-verb/contrast · The lint verb · contrast — A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
contract/lint-verb/defects · The lint verb · defect set — The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
contract/lint-verb/finding · The lint verb · finding — The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
contract/lint-verb/fonts-unavailable · The lint verb · fonts-unavailable — The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
contract/lint-verb/overlap · The lint verb · overlap — Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/lint-verb/zero-size · The lint verb · zero-size — A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/session-wire/ok-accepted · session wire · ok v3 accepted — A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/telemetry-capture/bounds · escher-telemetry capture · bounds — Each kept text is cut at 1024 bytes, saying how much; at most 8 fields a record and 16 records a source are kept, the rest counted.
contract/telemetry-panic/panic-event · Telemetry panic event · panic event — A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook.
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/telemetry/subscriber · escher-telemetry bootstrap · subscriber — Public: returns the sink's subscriber with the capture layer, built uninstalled, at a given LevelFilter.
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — The sink drops a target outside the engine and escher prefix sets whole at every level.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/unkeyed-actionable/unkeyed-actionable · Actionable-key check · unkeyed_actionable — Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls.
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/2 · CLI exit statuses · 2 — usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
contract/cold-agent-verdict-reasons/capability-unused · cold-agent verdict reasons · capability-unused — The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read and a list is opened.
contract/cold-agent-verdict/actions-calls · cold-agent verdict.json · actions_calls — The cold-agent verdict and its run.end event carry `actions_calls`, the count of the agent's `actions` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/advance-calls · cold-agent verdict.json · advance_calls — The cold-agent verdict carries `advance_calls`, the count of `advance` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/graph-calls · cold-agent verdict.json · graph_calls — The cold-agent verdict and its run.end event carry `graph_calls`, the count of the agent's `graph` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/layout-calls · cold-agent verdict.json · layout_calls — The cold-agent verdict and its run.end event carry `layout_calls`, the count of the agent's `layout` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/lint-calls · cold-agent verdict.json · lint_calls — The cold-agent verdict carries `lint_calls`, the count of `lint` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/list-opens · cold-agent verdict.json · list_opens — The cold-agent verdict carries `list_opens`, the count of accepted click or press answers whose changed list holds a ListBoxOption row whose differs is exactly bounds; an answer that does not decode as JSON is skipped.
contract/cold-agent-verdict/part-reads · cold-agent verdict · part_reads — The cold-agent verdict adds `part_reads`.
contract/cold-agent-verdict/recorded-sessions · cold-agent verdict · recorded_sessions — The cold-agent verdict adds `recorded_sessions`.
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — The cold-agent verdict adds `recording_kept`.
contract/cold-agent-verdict/refused-by-cause · cold-agent verdict.json · refused_by_cause — The cold-agent verdict carries `refused_by_cause`, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
contract/cold-agent-verdict/screenshot-calls · cold-agent verdict.json · screenshot_calls — The cold-agent verdict carries `screenshot_calls`, the count of `screenshot` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/wrong-call-bar-pct · cold-agent verdict.json · wrong_call_bar_pct — The verdict carries `wrong_call_bar_pct`, 5.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-native-dom-api/boundary-marks · dioxus-native-dom public API · boundary_marks and BoundaryMark — Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; the registry grows for the document's life, since a dropped boundary cannot be told.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/dom-event-kind/from-str · DomEventKind · from_str — Strips a leading on, maps composition to Ime and change to Change.
contract/dom-events/pointer-events · DOM event dispatch · pointer-derived events — Mouse compatibility events and touch events are dispatched from pointers.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON.
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/event-dispatch/targeting · Event dispatch · event target — Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/mcp-server/result-bound · MCP server · result bound — A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
contract/net-provider/is-noop · NetProvider · is_noop — Defaults to false; when true, callers register no pending-critical resources.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/provider-traits/netprovider-noop · blitz-traits provider traits · no-op NetProvider — The no-op NetProvider registers no pending-critical resources.
contract/reached-record/reached-handler · Reached record · ReachedHandler — Carries id and event, re-exported from the crate root.
contract/reached-record/reached-handlers · Reached record · reached_handlers — Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
contract/reached-record/record-reached · Reached record · record_reached — Switches the record on at the root scope; a second call keeps the one record.
contract/reached-record/take-step-reached · Reached record · take_step_reached — Drains the handlers reached since the last drain, each once, in first-reached order.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/session-client/start · session client · start — Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/telemetry-init/init · Telemetry init · init — Installs the stderr sink and returns Installed, or AlreadyInstalled with nothing changed on a second call.
contract/telemetry-line/event-line · Telemetry line · event line — One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/telemetry/init · escher-telemetry bootstrap · init — init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global.
contract/telemetry/line-format · escher-telemetry bootstrap · output line — One line is written per event and per closed span, stamped service.name and service.version.
contract/telemetry/panic-hook · escher-telemetry bootstrap · panic hook — The installed panic hook chains to the previous one.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/unkeyed-actionable/remedy · Actionable-key check · remedy — Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.

## occupied-resources · ## Occupied Resources
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts, diffs, and graph and actions answers, from the driver's own MCP server. · why: chosen as one run capped at $3, with a JSON check and a retry
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/model-provider-only-on-live-cold-agent-run · Model provider reached only on the live cold-agent run — The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request. · why: ratified
decision/model-provider-operator-only · Model provider reached only on operator runs — The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
decision/telemetry-one-install-site · One telemetry install site per binary — Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer.
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run.

## infrastructure-patterns · ## Infrastructure Patterns
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.

## inherited-defaults · ## Inherited Defaults
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
decision/embedder-services-default-noop · Embedder services default to no-ops — Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.

## existing-scopes · ## Existing Scopes
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
contract/escher-session-command-line/coverage · escher-session command line · coverage <app> <check>... — The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
