# architecture · the rows that bind 2026-11-18-cold-agent-live-run — picked 449 of 1532 (261 by every draw) · rule any · pin 2197

## stack-and-technologies · ## Stack and Technologies
decision/cold-agent-client-host-only · Cold-agent client is a host tool — The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
decision/cold-agent-judge-stdlib-python · Cold-agent judge on stdlib Python — The cold-agent pipe's judge is python3 on its standard library, with no package added. · why: ratified
decision/cold-agent-mcp-server-is-own · Cold-agent MCP server is the workspace's own — The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
decision/python3-build-time-for-stylo · python3 at build time for Stylo — python3 is required at build time to generate code for Stylo.

## established-decisions · ## Established Decisions
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified.
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged.
contract/actions-verb/arguments · the actions verb · arguments — The actions verb takes no argument; a call given one is refused malformed with nothing run.
contract/graph-verb/arguments · the graph verb · arguments — The graph verb takes no argument; a call given one is refused malformed with nothing run.
contract/mcp-stdio/result · the mcp role's stdio · result — A result is at most 1,048,576 bytes.
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed single select, under a group too, takes no click and stays refused `covered`.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, the placing words select and select-list in a layout answer, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/mcp-stdio/request-line · the mcp role's stdio · request line — A request line is at most 65,536 bytes.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.

## conventions · ## Conventions
decision/capture-before-own-recorder · Capture opened before a check's own recorder — A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder. · why: So no call site's interest is cached from a thread with no subscriber.
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated

## standard-contracts · ## Standard Contracts
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/advance-moves-session-clock · Advance moves the session's clock — An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move. · why: Ratified.
decision/advanced-ms-is-the-clock-move · advanced_ms reports the clock's move — An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page. · why: Ratified.
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls.
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/layout-answer-crossing · Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency.
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · guard: stand_masks_on_new_answers
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/screenshot-png-in-band · Screenshot image carried in band — A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result. · why: Ratified as the chosen fork.
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs.
contract/acting-verb-results/busy · acting verb results · busy — One of the busy classes, present only when settled is false.
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
contract/acting-verb-results/caused-past · acting verb results · caused_past — Written after caused only when non-empty on click, type, press, advance and scroll: one entry per kind and source counting the items past caused's 16, a panic or error entry writing no source.
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed.
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do.
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`.
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout`, `layout` with no cause.
contract/actions-verb/unpaged · The actions verb · paging — `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/layout · answer JSON · layout — {"id","bounds","laid_out","placed":{"by","container"?,"canvas"?,"at","relative"?},"shifts":[{"by","holder"?,"dx","dy"}],"width","height"}, each size {"px","by","size","min","max","flex"?}; ids, closed words and numbers only.
contract/answer-json/layout-fallback · answer JSON · layout fallback words — A later member of an engine set is written unexplained, transform, layout or auto, never a panic.
contract/answer-json/layout-length · answer JSON · layout length — A length is "auto", "none", "min-content", "max-content", "fit-content", "stretch", {"px":n}, {"percent":n} or {"calc":n}.
contract/answer-json/layout-placing · answer JSON · layout placing word — placed's by is one of block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app, written kebab-case.
contract/answer-json/layout-shift · answer JSON · layout shift word — A shift's by is scroll or transform.
contract/answer-json/layout-size-cause · answer JSON · layout size cause word — A size's by is one of style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout.
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
contract/answer-json/screenshot · answer JSON · screenshot — {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
contract/answer-json/string-escaping · answer JSON · string escaping — A string escapes ", \ and U+0000 to U+001F.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/argument-kinds/milliseconds · argument kinds · milliseconds — A whole number from 1 to 60000.
contract/argument-kinds/name · argument kinds · name — 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
contract/argument-kinds/part · argument kinds · part — A whole number from 1 to 1000, the snapshot part asked for.
contract/argument-kinds/text · argument kinds · text — 0 to 4096 bytes of text.
contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor); no log, no panic.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/cold-agent-cli/run-task · cold-agent.sh command line · run <task> — The cold-agent pipe's `run` takes the one task `stand`.
contract/cold-agent-events/cleanup · cold-agent.sh JSON-line events · cleanup — `cleanup` is one of the closed set of JSON-line events the cold-agent pipe emits.
contract/cold-agent-events/run-end · cold-agent.sh JSON-line events · run.end — `run.end` is one of the closed set of JSON-line events the cold-agent pipe emits.
contract/cold-agent-events/run-start · cold-agent.sh JSON-line events · run.start — `run.start` is one of the closed set of JSON-line events the cold-agent pipe emits.
contract/cold-agent-events/status · cold-agent.sh JSON-line events · status — `status` is one of the closed set of JSON-line events the cold-agent pipe emits.
contract/cold-agent-exit-codes/0 · cold-agent.sh exit codes · 0 — Success.
contract/cold-agent-exit-codes/1 · cold-agent.sh exit codes · 1 — A `failed` verdict.
contract/cold-agent-exit-codes/2 · cold-agent.sh exit codes · 2 — Usage error, checked before any precondition.
contract/cold-agent-exit-codes/3 · cold-agent.sh exit codes · 3 — A precondition is unmet (`claude` or `python3` missing, or the escher-session binary absent), with one stderr line naming the build command, no verdict and no events file.
contract/cold-agent-recording/live-server · cold-agent recording · live server child — The cold-agent live server child records into the owner-only `target/cold-agent/record/`; its two replay children run without recording.
contract/cold-agent-state/state-directory · cold-agent.sh state · state directory — The cold-agent pipe keeps its state in `target/cold-agent/` only.
contract/cold-agent-verdict-reasons/capability-unused · cold-agent verdict reasons · capability-unused — The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read, a list is opened and `caused_by_kind` reads at least 1 for each of panic, error and log.
contract/cold-agent-verdict-reasons/recording-not-kept · cold-agent verdict reasons · recording-not-kept — The verdict fails with `recording-not-kept` unless one check is kept per accepted `start`.
contract/cold-agent-verdict-reasons/wrong-calls-over-bar · cold-agent verdict reasons · wrong-calls-over-bar — The verdict fails with `wrong-calls-over-bar` when wrong calls exceed 5 % of tool calls; the three reasons come after `task-unmet` and before the check's.
contract/cold-agent-verdict/actions-calls · cold-agent verdict.json · actions_calls — The cold-agent verdict and its run.end event carry `actions_calls`, the count of the agent's `actions` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/advance-calls · cold-agent verdict.json · advance_calls — The cold-agent verdict carries `advance_calls`, the count of `advance` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/caused-by-kind · cold-agent verdict.json · caused_by_kind — The cold-agent verdict and its run.end event carry `caused_by_kind`, an object of the closed words panic, error and log to counts of accepted click, type, press, advance and scroll answers whose caused list holds an item of that kind, each kind counted once per answer; a non-JSON answer is skipped.
contract/cold-agent-verdict/graph-calls · cold-agent verdict.json · graph_calls — The cold-agent verdict and its run.end event carry `graph_calls`, the count of the agent's `graph` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/layout-calls · cold-agent verdict.json · layout_calls — The cold-agent verdict and its run.end event carry `layout_calls`, the count of the agent's `layout` calls read from the tool_call span, which the `passed` bar requires to be used.
contract/cold-agent-verdict/lint-calls · cold-agent verdict.json · lint_calls — The cold-agent verdict carries `lint_calls`, the count of `lint` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/list-opens · cold-agent verdict.json · list_opens — The cold-agent verdict carries `list_opens`, the count of accepted click or press answers whose changed list holds a ListBoxOption row whose differs is exactly bounds; an answer that does not decode as JSON is skipped.
contract/cold-agent-verdict/part-reads · cold-agent verdict · part_reads — The cold-agent verdict adds `part_reads`.
contract/cold-agent-verdict/passed · cold-agent verdict.json · passed — The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all seven apps met, and the agent's final check replaying passing and failing on an ablated replay.
contract/cold-agent-verdict/recorded-sessions · cold-agent verdict · recorded_sessions — The cold-agent verdict adds `recorded_sessions`.
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — The cold-agent verdict adds `recording_kept`.
contract/cold-agent-verdict/refused-by-cause · cold-agent verdict.json · refused_by_cause — The cold-agent verdict carries `refused_by_cause`, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
contract/cold-agent-verdict/screenshot-calls · cold-agent verdict.json · screenshot_calls — The cold-agent verdict carries `screenshot_calls`, the count of `screenshot` calls, which the `passed` bar requires to be used.
contract/cold-agent-verdict/wrong-call-bar-pct · cold-agent verdict.json · wrong_call_bar_pct — The verdict carries `wrong_call_bar_pct`, 5.
contract/command-line/mcp · command line · mcp — mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fifteen verbs, eight closed causes.
contract/driver-session/layout · escher-driver session · Session::layout — A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
contract/driver-session/reach · escher-driver session · Session::reach — Session::reach runs each acting step (click, type, press, advance) of a named fixture state as a validated call through the one run path, in process, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives seven served stand tasks: the four lean ones, Cells, the load page and the order page.
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session.
contract/escher-session/advance · escher-session command line · advance --ms — The verb advance takes --ms.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's seven served tasks: counter, flight-booker, timer, crud, cells, load, order.
contract/escher-session/lint · escher-session command line · lint — The verb lint takes no argument beside --session.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same seven tasks through the same boot.
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift.
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/graph-verb/argument · The graph verb · argument — `graph` takes no argument; an argument is refused `malformed` with nothing run.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session.
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the eleven instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
contract/kept-check/accepted · Kept check file · accepted — Each record states whether the call was accepted.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/bounds · Kept check file · size bounds — A kept check holds at most 400 records and 1,048,576 bytes.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the eleven instance verbs are kept, `lint`, `graph`, `actions` and `layout` among them with their answers whole; any other call is not kept.
contract/kept-check/opening-record · Kept check file · opening record — A kept check opens with the record `{"tool":"start","arguments":{"app":<label>},"format":1}`, `format` the public check format number CHECK_FORMAT, now 1; a reader accepts only that whole number or its absence, read as 1, and refuses any other value malformed.
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called.
contract/lint-verb/defects · The lint verb · defect set — The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
contract/lint-verb/finding · The lint verb · finding — The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
contract/lint-verb/fonts-unavailable · The lint verb · fonts-unavailable — The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/mcp-server/idle-expiry · MCP server · idle expiry — Checked when a request arrives with a session held; no timer, no thread.
contract/mcp-server/recording-failed · MCP server tool result · RecordingFailed — The MCP server answers a call whose recording ended with a failed tool result (`isError`) carrying `SessionError::RecordingFailed`.
contract/mcp-server/result-bound · MCP server · result bound — A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
contract/mcp-server/tools-list · MCP server · tools/list — The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/recording/directory · Session recording · record directory — A recording makes its directory owner-only or refuses it with `RecordDirNotPrivate`.
contract/recording/failure · Session recording · recording failure — Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/malformed · refusal causes · malformed — An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/refusal-causes/stale · refusal causes · stale — The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
contract/refusal-causes/unknown-verb · refusal causes · unknown-verb — The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/session-errors/already-running · session errors · already-running — A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
contract/session-errors/record-dir-not-private · session errors · record-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
contract/session-errors/recording-failed · session errors · recording-failed — The host's recording ended after the call; the wire's recording-failed reply reads as it.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A multiple select's rows and groups read their own client rects as their bounds.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/target-check/screenshot · Per-verb target check · screenshot with an id — A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
contract/telemetry-capture/bounds · escher-telemetry capture · bounds — Each kept text is cut at 1024 bytes, saying how much; at most 8 fields a record and 16 records a source are kept, the rest counted.
contract/telemetry-panic/panic-event · Telemetry panic event · panic event — A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook.
contract/telemetry/subscriber · escher-telemetry bootstrap · subscriber — Public: returns the sink's subscriber with the capture layer, built uninstalled, at a given LevelFilter.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word().
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
contract/unkeyed-actionable/remedy · Actionable-key check · remedy — Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.
contract/unkeyed-actionable/unkeyed-actionable · Actionable-key check · unkeyed_actionable — Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/capture-dispatcher-pair · Capture's process-wide dispatcher pair — Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode. · why: Per-thread dispatchers die with their threads and leave tracing-core in its one-dispatcher mode; they were rejected.
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts.
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken.
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/argument-kinds/flag · argument kinds · flag — A boolean.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/blitz-dom-crate-root/tracing-compiled · blitz-dom crate root · TRACING_COMPILED — Public: true when the engine's tracing feature is compiled in.
contract/busy-classes/layout · busy classes · layout — A busy class word of an instance that did not go quiet.
contract/busy-classes/loads · busy classes · loads — A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
contract/busy-classes/render · busy classes · render — A busy class word of an instance that did not go quiet, and of an acting step whose settle panicked a second time.
contract/check-replay/another-app · Check replay · another app — A replay runs nothing on a session whose label is not the check's app.
contract/check-replay/change-keys · Check replay · change keys — A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-coverage-role/0 · Command line coverage role · 0 — Accepted: the coverage line is printed on stdout.
contract/cli-coverage-role/1 · Command line coverage role · 1 — A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
contract/cli-coverage-role/3 · Command line coverage role · 3 — A boot that fails is the session error.
contract/cli-coverage-role/shape · Command line coverage role · shape — `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/command-line/closed-pipe · command line · closed pipe — A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
contract/command-line/start · command line · start — start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id.
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dioxus-native-dom-api/boundary-marks · dioxus-native-dom public API · boundary_marks and BoundaryMark — Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; the registry grows for the document's life, since a dropped boundary cannot be told.
contract/dioxus-native-dom-api/is-masked-input · dioxus-native-dom public API · is_masked_input — Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
contract/dioxus-native-dom-api/root-error-mark · dioxus-native-dom public API · root_error_mark and RootErrorMark — Public and re-exported: root_error_mark reads the root error boundary's error context; RootErrorMark compares by identity and its Debug prints its name alone.
contract/dioxus-native-dom-api/seeded-random · dioxus-native-dom public API · SeededRandom — Public and re-exported: a splitmix64 generator whose clones share one state, so every component of one document draws from one stream in draw order; from_seed(u64), next_u64 and below(bound), the high bits of the 128-bit product, 0 for a bound of 0, no panic path; Debug writes the type name alone.
contract/dioxus-native-dom-api/use-seeded-random · dioxus-native-dom public API · use_seeded_random and DEFAULT_SEED — Public and re-exported: use_seeded_random returns the root's SeededRandom from a root context, else a generator seeded from the host's per-process random keys, the windowed shell's case outside any run; DEFAULT_SEED is the u64 seed 0.
contract/driver-explore/exhausted · escher-driver explore · exhausted — An exploration stops exhausted when its frontier empties, which only a finite set of screens reaches; an app whose screens never repeat never exhausts it, and such an exploration stops when the next path would pass the step budget.
contract/driver-explore/expansion · escher-driver explore · path expansion — A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
contract/driver-explore/explore · escher-driver explore · explore — An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching through its earlier calls; a boot error ends it.
contract/driver-explore/explored-verbs · escher-driver explore · explored verbs — The verbs explored are the actions table's target verbs less any whose result holds an image or a layout reading field (png, rect, placement, shifts or size kind), selected by the field's kind and never its name; today click, type and scroll.
contract/driver-explore/path-ending · escher-driver explore · PathEnding — An explored path ends accepted (settled or not), refused with its cause, panicked, or diverged at a step; a handler panic is caught per path and the exploration goes on.
contract/driver-explore/step-budget · escher-driver explore · StepBudget — A step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS), refused as none at 0 and above the bound, and counts every acting call an exploration runs, re-reach steps included.
contract/driver-forms/closed-select-of · driver form-control readers · closed_select_of — The public closed_select_of answers None for a select whose list is open.
contract/driver-forms/is-masked-input · driver form-control readers · is_masked_input — is_masked_input is the one masked-control test: a password or file input.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/layout-unexplained · escher-driver session · unexplained placer — An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/escher-session-mcp-exit-codes/0 · escher-session mcp role exit codes · 0 — The mcp role exits 0 on Ok, writing nothing on stdout at its end.
contract/escher-session-mcp-exit-codes/3 · escher-session mcp role exit codes · 3 — The mcp role exits 3 on Err with the error's fixed message on stderr, writing nothing on stdout at its end.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role.
contract/handler-coverage/reached · Handler coverage reader · reached handlers — The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
contract/handler-coverage/unreached-screens · Handler coverage reader · unreached screens — A handler on no screen a replay reached is not listed, by construction.
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/kept-check-reader/refusal · Kept check reader · malformed refusal — Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
contract/kept-check/digest · Kept check file · digest — `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
contract/kept-check/masked · Kept check file · masked — `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
contract/key-names/arrow-down · key names · arrow-down — A key name press accepts.
contract/key-names/arrow-left · key names · arrow-left — A key name press accepts.
contract/key-names/arrow-right · key names · arrow-right — A key name press accepts.
contract/key-names/arrow-up · key names · arrow-up — A key name press accepts.
contract/key-names/backspace · key names · backspace — A key name press accepts; on macOS it also dispatches deleteBackward.
contract/key-names/delete · key names · delete — A key name press accepts.
contract/key-names/end · key names · end — A key name press accepts.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/escape · key names · escape — A key name press accepts.
contract/key-names/home · key names · home — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/key-names/tab · key names · tab — A key name press accepts.
contract/layout-reason/length · blitz-dom layout reason · Length — A stated length is closed: Auto, None, Px, Percent (0 to 100), Calc in px, or a SizeKeyword of MinContent, MaxContent, FitContent or Stretch.
contract/layout-reason/placed-by · blitz-dom layout reason · PlacedBy — The placing word is closed: Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList.
contract/layout-reason/shift-by · blitz-dom layout reason · ShiftBy — The shift word is closed: Scroll or Transform.
contract/layout-reason/size-cause · blitz-dom layout reason · SizeCause — The cause of a size is closed: Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout.
contract/lint-reason-keys/clip · lint reason keys · clip — visible.
contract/lint-reason-keys/contrast · lint reason keys · contrast — ratio, its shortest decimal or null when not finite, and required.
contract/lint-reason-keys/off-screen · lint reason keys · off-screen — No reason keys.
contract/lint-reason-keys/overlap · lint reason keys · overlap — other and area.
contract/lint-reason-keys/truncation · lint reason keys · truncation — text then visible, visible left out when no part can be seen.
contract/lint-reason-keys/zero-size · lint reason keys · zero-size — No reason keys.
contract/lint-verb/changes-nothing · The lint verb · side effects — The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
contract/lint-verb/clipped-off-screen · The lint verb · clipped and off-screen — `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
contract/lint-verb/contrast · The lint verb · contrast — A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
contract/lint-verb/overlap · The lint verb · overlap — Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
contract/lint-verb/overlap-select-rows · The lint verb · overlap of select options and list rows — A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
contract/lint-verb/zero-size · The lint verb · zero-size — A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/mcp-server/initialize · MCP server · initialize — Answers capabilities {"tools":{}} and serverInfo with name escher and the driver's package version.
contract/mcp-server/protocol-revisions · MCP server · protocol revisions — Newline-delimited JSON-RPC 2.0 in the four handshake revisions of PROTOCOL_VERSIONS; initialize answers the asked revision when it is one of the four, else LATEST_VERSION, the newest of them.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/resolved-style/resolved-style-value · Resolved style · resolved_style_value — Returns an empty string for an unknown property and the used value for a layout-dependent one.
contract/resolved-style/width-height · Resolved style · width and height — Read from the unrounded layout, so a reader gets sub-pixel values where the final layout rounds.
contract/screenshot-error/fonts-unavailable · ScreenshotError · FontsUnavailable — Returned with nothing painted when the document cannot paint text.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/session-client/call-answer-read-bound · session client · call answer read bound — A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
contract/session-client/start · session client · start — Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
contract/session-errors/host-exited · session errors · host-exited — The spawned host exited before the session answered, carrying its exit code where there is one.
contract/session-errors/no-session · session errors · no-session — No socket file is in the state directory, or no session is held for an instance verb.
contract/session-errors/not-settled · session errors · not-settled — An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
contract/session-errors/timeout · session errors · timeout — A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
contract/stable-element-id/anchored-path · Stable element ids · anchored path tier — The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
contract/stable-element-id/component-path · Stable element ids · component path tier — The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
contract/target-check/scroll · Per-verb target check · scroll — `scroll` meets no target check.
contract/telemetry-line/event-line · Telemetry line · event line — One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
contract/telemetry-line/span-close-line · Telemetry line · span close line — One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/telemetry/line-format · escher-telemetry bootstrap · output line — One line is written per event and per closed span, stamped service.name and service.version.
contract/telemetry/panic-hook · escher-telemetry bootstrap · panic hook — The installed panic hook chains to the previous one.
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — The sink drops a target outside the engine and escher prefix sets whole at every level.
contract/test-harness/run-seed · blitz-test-harness · harness run seed — Harness::from_vdom provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, before the document is built; a SeededRandom the builder gave as a root context stands; Harness::wrap provides none.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.

## occupied-resources · ## Occupied Resources
decision/agent-state-areas-local · Agent state areas stay local — The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider from the driver's own MCP server: element ids, accessible names, control values, snapshot texts, diffs, graph and actions answers, and acting answers' caused text, the order page's authored messages among it. · why: chosen as one run capped at $3, with a JSON check and a retry
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/model-provider-only-on-live-cold-agent-run · Model provider reached only on the live cold-agent run — The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request. · why: ratified
decision/model-provider-operator-only · Model provider reached only on operator runs — The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/record-dir-launcher-named-private · Recording directory launcher-named and owner-only — The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
decision/recording-directory · Recording directory — The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
contract/mcp-role/screenshot-result · the mcp role · screenshot result — A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/recording-removes-only-own-checks · Recording removes only its own checks — Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory.
decision/recording-removes-own-checks-only · Recording removes only its own checks — escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory.
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/mcp-role/handshake-revision · the mcp role · handshake revision — The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default.
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields.

## infrastructure-patterns · ## Infrastructure Patterns
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/agent-driven-development · Agent-driven development style — Development is agent-driven: the framework's own driver is meant to become the harness.
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber.
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.
decision/deferred-stylo-dioxus-bumps · Deferred Stylo and Dioxus bumps — Stylo stays "0.22.0" and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version.
decision/readme-is-escher-own · Front page is escher's own — The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
decision/readme-is-escher-page · Root README is escher's own page — The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.

## existing-scopes · ## Existing Scopes
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
