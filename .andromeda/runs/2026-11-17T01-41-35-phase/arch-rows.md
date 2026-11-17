# architecture · the rows that bind 2026-11-17-layout-on-the-carriers — picked 355 of 1521 (158 by every draw) · rule any · pin 1580

## design-philosophy · ## Design Philosophy
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.

## stack-and-technologies · ## Stack and Technologies
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
decision/serde-json-driver-reader · serde_json as the driver's JSON reader — serde_json is escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares. · why: already in the graph through dioxus-html
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds

## established-decisions · ## Established Decisions
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, closed words, booleans and counts cross a driver session, never a `NodeId`.
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0.
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged.
contract/mcp-stdio/result · the mcp role's stdio · result — A result is at most 1,048,576 bytes.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified
decision/apple-system-font-alias · Apple system font names map to system-ui — `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS.
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/coverage-reads-named-checks · Coverage role reads named check files — The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly. · why: a ratified boundary widening
decision/coverage-role-reads-named-checks · Coverage role reads kept checks its caller names — The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path. · why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified.
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified.
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.

## conventions · ## Conventions
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
decision/capture-before-own-recorder · Capture opened before a check's own recorder — A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder. · why: So no call site's interest is cached from a thread with no subscriber.
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.

## standard-contracts · ## Standard Contracts
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/hand-written-answers · Driver answers written by hand — Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
decision/layout-read-in-process-only · Layout read stays in process — No verb, MCP tool, session socket call or kept check reaches the driver's layout read; it is answered in process only. · provisional
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified.
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do.
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`.
contract/actions-verb/unpaged · The actions verb · paging — `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/string-escaping · answer JSON · string escaping — A string escapes ", \ and U+0000 to U+001F.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor); no log, no panic.
contract/blitz-dom-crate-root/layout-reason-types · blitz-dom crate root · LayoutReason, Placed, Shift, SizeReason, FlexInputs and their enums — The layout reason's structs and its non-exhaustive enums PlacedBy, ShiftBy, SizeCause, Length and SizeKeyword are public read types re-exported from the crate root.
contract/check-replay/another-app · Check replay · another app — A replay runs nothing on a session whose label is not the check's app.
contract/check-replay/change-keys · Check replay · change keys — A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/cli-exit-statuses/2 · CLI exit statuses · 2 — usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
contract/command-line/help · command line · help — help with no argument and at most one unread --session answers the self-description on stdout, status 0, reaching no session; an argument is refused malformed.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-native-dom-api/layout-reason-types · dioxus-native-dom public API · layout reason types and BoundingRect — Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect.
contract/driver-explore/explored-verbs · escher-driver explore · explored verbs — The verbs explored are the actions table's target verbs less any whose result holds an image-kind field, selected by the field's kind and never its name; today click, type and scroll.
contract/driver-public-api/no-serde-json-types · Driver public API · serde_json types — No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
contract/driver-session/layout · escher-driver session · Session::layout — An in-process read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id as the run path does, refusing stale or not-found, records the screen's ids, runs no step, pump, tick or settle and opens no span.
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/help · escher-session command line · help — Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once.
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept.
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called.
contract/layout-reason/placed-by · blitz-dom layout reason · PlacedBy — The placing word is closed: Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList.
contract/layout-reason/size-cause · blitz-dom layout reason · SizeCause — The cause of a size is closed: Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout.
contract/lint-verb/changes-nothing · The lint verb · side effects — The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/mcp-server/result-bound · MCP server · result bound — A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
contract/mcp-server/tools-list · MCP server · tools/list — The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/refusal-causes/malformed · refusal causes · malformed — An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
contract/refusal-causes/stale · refusal causes · stale — The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
contract/refusal-causes/unknown-verb · refusal causes · unknown-verb — The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
contract/refusal-faults/missing · refusal faults · Missing — A required argument is absent; beside malformed only.
contract/refusal-faults/out-of-bound · refusal faults · OutOfBound — An argument's value is outside its bound, or a text is no key name; beside malformed only.
contract/refusal-faults/repeated · refusal faults · Repeated — An argument name was passed before in the same call; beside malformed only.
contract/refusal-faults/unnamed · refusal faults · Unnamed — An argument names nothing the verb takes; beside malformed only.
contract/refusal-faults/wrong-kind · refusal faults · WrongKind — An argument's value is of another kind than the schema's; beside malformed only.
contract/session-client/call-answer-read-bound · session client · call answer read bound — A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
contract/session-errors/no-session · session errors · no-session — No socket file is in the state directory, or no session is held for an instance verb.
contract/session-errors/unsupported · session errors · unsupported — On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/session-wire/ok-accepted · session wire · ok v3 accepted — A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
contract/session-wire/ok-refused · session wire · ok v3 refused — A call's refusal, its JSON line after the words.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/seen-ids-record-bounded · Session's record of seen ids bounded — Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged. · why: Ratified for ids longer than a call can name. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified.
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
contract/accessibility-tree/img-empty-alt · Accessibility tree · img with an empty alt — An img whose alt is empty reads as GenericContainer, decorative and unnamed.
contract/accessibility-tree/name-sources · Accessibility tree · accessible name — A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
contract/accessibility-tree/native-control-toggled · Accessibility tree · native control toggled — A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
contract/accessibility-tree/node-names · blitz-dom accessibility tree · node names — Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
contract/accessibility-tree/non-checkable-role · blitz-dom accessibility tree · native control under a non-checkable role — A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`.
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
contract/base-document/select-value · BaseDocument · select_value — Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
contract/base-document/select-values · BaseDocument · select_values — Public and read-only: the select's chosen values.
contract/base-document/take-changed-nodes · BaseDocument · take_changed_nodes — Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
contract/busy-classes/loads · busy classes · loads — A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
contract/cli-coverage-role/0 · Command line coverage role · 0 — Accepted: the coverage line is printed on stdout.
contract/cli-coverage-role/1 · Command line coverage role · 1 — A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
contract/cli-exit-statuses/3 · CLI exit statuses · 3 — session-error: the error object on stdout and its fixed message on stderr.
contract/command-span/added · command span · added — The length of the diff's added list.
contract/command-span/busy · command span · busy — The busy class word, recorded when the outcome carries one.
contract/command-span/changed · command span · changed — The length of the diff's changed list.
contract/command-span/passes · command span · passes — Settle's pass count, recorded when settle answered quiet.
contract/command-span/removed · command span · removed — The length of the diff's removed list.
contract/command-span/settled · command span · settled — Recorded when the outcome carries one.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/document-config/ua-stylesheets · DocumentConfig · ua_stylesheets — A Vec of UserAgentStylesheet, defaulting to the one default sheet; a config that sets it is used as given.
contract/driver-explore/expansion · escher-driver explore · path expansion — A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
contract/driver-explore/step-budget · escher-driver explore · StepBudget — A step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS), refused as none at 0 and above the bound, and counts every acting call an exploration runs, re-reach steps included.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role.
contract/escher-session-cli/serve-role · escher-session command line · serve — escher-session has a serve host role.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused; one line of JSON answers it on stdout.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/form-controls/is-disabled · blitz-dom form control readers · ElementData::is_disabled — ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it.
contract/geometry/visible-region-non-finite · blitz-dom geometry readers · visible_region on a non-finite box — visible_region answers None for a mapped box with a non-finite edge.
contract/geometry/visible-region-select-list · blitz-dom geometry readers · visible_region of an open select list's row — A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
contract/geometry/visible-region-transforms · blitz-dom geometry readers · visible_region through transforms — visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session.
contract/kept-check-reader/refusal · Kept check reader · malformed refusal — Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
contract/kept-check/accepted · Kept check file · accepted — Each record states whether the call was accepted.
contract/kept-check/bounds · Kept check file · size bounds — A kept check holds at most 400 records and 1,048,576 bytes.
contract/kept-check/digest · Kept check file · digest — `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
contract/kept-check/masked · Kept check file · masked — `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
contract/kept-check/opening-record · Kept check file · opening record — A kept check opens with the record `{"tool":"start","arguments":{"app":<label>},"format":1}`, `format` the public check format number CHECK_FORMAT, now 1; a reader accepts only that whole number or its absence, read as 1, and refuses any other value malformed.
contract/layout-reason/length · blitz-dom layout reason · Length — A stated length is closed: Auto, None, Px, Percent (0 to 100), Calc in px, or a SizeKeyword of MinContent, MaxContent, FitContent or Stretch.
contract/layout-reason/shift-by · blitz-dom layout reason · ShiftBy — The shift word is closed: Scroll or Transform.
contract/lint-reason-keys/clip · lint reason keys · clip — visible.
contract/lint-reason-keys/contrast · lint reason keys · contrast — ratio, its shortest decimal or null when not finite, and required.
contract/lint-reason-keys/off-screen · lint reason keys · off-screen — No reason keys.
contract/lint-reason-keys/overlap · lint reason keys · overlap — other and area.
contract/lint-reason-keys/truncation · lint reason keys · truncation — text then visible, visible left out when no part can be seen.
contract/lint-reason-keys/zero-size · lint reason keys · zero-size — No reason keys.
contract/lint-verb/clipped-off-screen · The lint verb · clipped and off-screen — `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
contract/lint-verb/contrast · The lint verb · contrast — A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
contract/lint-verb/defects · The lint verb · defect set — The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
contract/lint-verb/finding · The lint verb · finding — The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
contract/lint-verb/fonts-unavailable · The lint verb · fonts-unavailable — The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
contract/lint-verb/overlap · The lint verb · overlap — Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
contract/lint-verb/overlap-select-rows · The lint verb · overlap of select options and list rows — A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
contract/lint-verb/zero-size · The lint verb · zero-size — A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
contract/mcp-server/recording-failed · MCP server tool result · RecordingFailed — The MCP server answers a call whose recording ended with a failed tool result (`isError`) carrying `SessionError::RecordingFailed`.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.
contract/recording/failure · Session recording · recording failure — Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/session-error-json/shape · session error JSON · shape — A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
contract/session-wire/version · session wire · version — The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
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
contract/stable-element-id/anchored-path · Stable element ids · anchored path tier — The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
contract/stable-element-id/component-path · Stable element ids · component path tier — The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
contract/target-check/scroll · Per-verb target check · scroll — `scroll` meets no target check.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.

## occupied-resources · ## Occupied Resources
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.
contract/mcp-role/help · the mcp role · help — The help tool is answered by the server with no session run.
contract/mcp-role/self-description · the mcp role · self-description and remedy — A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields.
contract/session-socket-wire/answer-bound · the session socket's wire · answer bound — An answer is at most 1,048,576 bytes.
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer.
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported.
contract/session-socket-wire/recording-failed · the session socket's wire · recording-failed — A call's replies include the fixed recording-failed reply.
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/record-dir-launcher-named-private · Recording directory launcher-named and owner-only — The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
decision/recording-directory · Recording directory — The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
contract/recording-directory/platform · The recording directory · platform — Recording is unix only; elsewhere it is refused Unsupported.
contract/session-socket-wire/request-line-bound · the session socket's wire · request line bound — A request line is at most 16,384 bytes.

## infrastructure-patterns · ## Infrastructure Patterns
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
decision/engine-tracing-gated-per-call-site · Engine tracing gated per call site — Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off. · except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.

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
