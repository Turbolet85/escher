# architecture · the rows that bind 2026-11-17-stand-requirement-sweep — picked 496 of 1527 (259 by every draw) · rule any · pin 1850

## design-philosophy · ## Design Philosophy
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport
decision/select-list-painted-in-scene · Open select list painted inside the scene — An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
decision/text-painted-from-container-style · Text painted from its container's style — Text is painted from its container element's resolved styles. · why: matches how HTML works
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.

## stack-and-technologies · ## Stack and Technologies
decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
decision/cargo-deny-and-llvm-cov-tools · CI-installed audit and coverage tools — CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
decision/cli-flow-under-plain-sh · CLI flow tests under plain sh — seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement. · why: no dependency, manifest line or test framework is added for it
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/mutation-tier-host-only · Host-run mutation tier — A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
decision/parley-text-layout · Parley for text and fonts — Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.

## established-decisions · ## Established Decisions
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed single select, under a group too, takes no click and stays refused `covered`.
decision/coverage-reads-named-checks · Coverage role reads named check files — The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly. · why: a ratified boundary widening
decision/coverage-role-reads-named-checks · Coverage role reads kept checks its caller names — The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path. · why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/dioxus-no-own-stylesheet · Dioxus document adds no stylesheet — A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching.
decision/gap-normal-zero · gap normal resolves to zero — gap: normal resolves to 0.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/media-type-screen-default · CSS media type defaults to screen — The CSS media type defaults to screen.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/scroll-offsets-clamped-after-layout · Scroll offsets clamped after layout — After layout rounding every scroll container's offset is clamped to its scroll range, the viewport and a running scroll animation are clamped too, and a removed subtree's scroll state is reset, so a list whose content shrinks after a delete reads offset 0. · why: an upstream behaviour change taken, ratified for its take · guard: stand_act_scroll, percentage_height_scroller_clamps_after_viewport_resize
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, the placing words select and select-list in a layout answer, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/windowed-documents-parallel · Windowed documents style in parallel — The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading.
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0.
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged.
contract/coverage-role/answer · the command line's coverage role · answer — One line on stdout of ids, owner ids, event names and two counts.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
decision/apple-system-font-alias · Apple system font names map to system-ui — `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS.
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision
decision/cssom-geometry-snapping · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px.
decision/failed-stylesheet-unblocks · A failed stylesheet stops blocking render — A failed, empty or non-CSS stylesheet stops blocking render. · guard: render_blocking_stylesheet
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/html-no-quirks-only · Documents are HTML in no-quirks mode — Documents are always HTML in no-quirks mode.
decision/html5ever-parse-settings · html5ever parse settings — html5ever runs with scripting off, the doctype dropped and no-quirks mode.
decision/input-control-data-at-construction · Input control data at construction — Text, checkbox, radio and range inputs get their control data at construction.
decision/key-types-through-harness · Key types through the harness's re-exports — The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
decision/macos-ignores-safe-area · macOS ignores safe-area insets — On macOS safe-area insets are ignored. · why: so content can draw in the titlebar
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported.
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet
decision/scrollbar-width-zero · Layout scrollbar width is zero — The scrollbar width handed to layout is always 0.
decision/select-list-painted-overlay · Select list painted over the page — A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included
decision/select-list-rejected-designs · Rejected designs for the select list — The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
decision/sequential-style-default · Sequential style threading by default — Style threading defaults to Sequential, which bypasses Stylo's global thread pool.
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only.
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
decision/static-insets-auto · Static boxes report auto insets — Insets are reported auto for position: static.
decision/stylo-prefs-set-once · Stylo prefs set once, by Stylo 0.22 names — Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1. · why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
contract/actions-verb/arguments · the actions verb · arguments — The actions verb takes no argument; a call given one is refused malformed with nothing run.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled.
contract/graph-verb/arguments · the graph verb · arguments — The graph verb takes no argument; a call given one is refused malformed with nothing run.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.

## conventions · ## Conventions
decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`.
decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants.
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
decision/reduced-fixture-variants · Reduced fixture variants — Reduced variants of HTML fixtures sit beside the full pages.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file.
decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.

## standard-contracts · ## Standard Contracts
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/layout-answer-crossing · Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency.
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py
decision/reached-record-before-dispatch · Reached record written before dispatch — The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render. · why: So a handler whose own run removes its element still counts.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/seen-ids-record-bounded · Session's record of seen ids bounded — Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged. · why: Ratified for ids longer than a call can name. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do.
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout`, `layout` with no cause.
contract/actions-verb/unpaged · The actions verb · paging — `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/layout · answer JSON · layout — {"id","bounds","laid_out","placed":{"by","container"?,"canvas"?,"at","relative"?},"shifts":[{"by","holder"?,"dx","dy"}],"width","height"}, each size {"px","by","size","min","max","flex"?}; ids, closed words and numbers only.
contract/answer-json/layout-length · answer JSON · layout length — A length is "auto", "none", "min-content", "max-content", "fit-content", "stretch", {"px":n}, {"percent":n} or {"calc":n}.
contract/answer-json/layout-placing · answer JSON · layout placing word — placed's by is one of block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app, written kebab-case.
contract/answer-json/layout-shift · answer JSON · layout shift word — A shift's by is scroll or transform.
contract/answer-json/layout-size-cause · answer JSON · layout size cause word — A size's by is one of style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor); no log, no panic.
contract/check-replay/another-app · Check replay · another app — A replay runs nothing on a session whose label is not the check's app.
contract/check-replay/change-keys · Check replay · change keys — A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-coverage-role/0 · Command line coverage role · 0 — Accepted: the coverage line is printed on stdout.
contract/cli-coverage-role/shape · Command line coverage role · shape — `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/command-line/closed-pipe · command line · closed pipe — A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
contract/command-line/mcp · command line · mcp — mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
contract/command-line/serve · command line · serve — serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/coverage-floor-exit-codes/0 · coverage_floor.py exit codes · 0 — Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
contract/coverage-floor-output/crate-line · coverage_floor.py output · crate line — The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dioxus-native-dom-api/is-masked-input · dioxus-native-dom public API · is_masked_input — Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
contract/dioxus-native-dom-api/layout-reason-types · dioxus-native-dom public API · layout reason types and BoundingRect — Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/driver-explore/exhausted · escher-driver explore · exhausted — An exploration stops exhausted when its frontier empties, which only a finite set of screens reaches; an app whose screens never repeat never exhausts it, and such an exploration stops when the next path would pass the step budget.
contract/driver-explore/expansion · escher-driver explore · path expansion — A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
contract/driver-explore/explore · escher-driver explore · explore — An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching through its earlier calls; a boot error ends it.
contract/driver-explore/explored-verbs · escher-driver explore · explored verbs — The verbs explored are the actions table's target verbs less any whose result holds an image or a layout reading field (png, rect, placement, shifts or size kind), selected by the field's kind and never its name; today click, type and scroll.
contract/driver-explore/path-ending · escher-driver explore · PathEnding — An explored path ends accepted (settled or not), refused with its cause, panicked, or diverged at a step; a handler panic is caught per path and the exploration goes on.
contract/driver-explore/step-budget · escher-driver explore · StepBudget — A step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS), refused as none at 0 and above the bound, and counts every acting call an exploration runs, re-reach steps included.
contract/driver-forms/is-masked-input · driver form-control readers · is_masked_input — is_masked_input is the one masked-control test: a password or file input.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/layout · escher-driver session · Session::layout — A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own.
contract/driver-session/layout-unexplained · escher-driver session · unexplained placer — An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step.
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON.
contract/escher-session-cli/coverage-role · escher-session command line · coverage — escher-session has a coverage role.
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role.
contract/escher-session-cli/serve-role · escher-session command line · serve — escher-session has a serve host role.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/handler-coverage/bubbling · Handler coverage reader · bubbling dispatch — A bubbling dispatch counts every ancestor listener it passed; a `stopPropagation` is not read.
contract/handler-coverage/canvas · Handler coverage reader · canvas handlers — Handlers inside a canvas are not counted.
contract/handler-coverage/per-check-boot · Handler coverage reader · per-check boot — A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
contract/handler-coverage/reached · Handler coverage reader · reached handlers — The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
contract/handler-coverage/unreached-screens · Handler coverage reader · unreached screens — A handler on no screen a replay reached is not listed, by construction.
contract/handler-coverage/written-form · Handler coverage reader · written form — Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page and one component alone in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the eleven instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
contract/kept-check/accepted · Kept check file · accepted — Each record states whether the call was accepted.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
contract/kept-check/digest · Kept check file · digest — `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the eleven instance verbs are kept, `lint`, `graph`, `actions` and `layout` among them with their answers whole; any other call is not kept.
contract/kept-check/masked · Kept check file · masked — `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
contract/kept-check/opening-record · Kept check file · opening record — A kept check opens with the record `{"tool":"start","arguments":{"app":<label>},"format":1}`, `format` the public check format number CHECK_FORMAT, now 1; a reader accepts only that whole number or its absence, read as 1, and refuses any other value malformed.
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called.
contract/layout-reason/length · blitz-dom layout reason · Length — A stated length is closed: Auto, None, Px, Percent (0 to 100), Calc in px, or a SizeKeyword of MinContent, MaxContent, FitContent or Stretch.
contract/layout-reason/placed-by · blitz-dom layout reason · PlacedBy — The placing word is closed: Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList.
contract/layout-reason/shift-by · blitz-dom layout reason · ShiftBy — The shift word is closed: Scroll or Transform.
contract/layout-reason/size-cause · blitz-dom layout reason · SizeCause — The cause of a size is closed: Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout.
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/provider-traits/viewport-scale · blitz-traits provider traits · Viewport scale() — A `Viewport`'s `scale()` is hidpi_scale × zoom.
contract/select-readers/select-value · BaseDocument select readers · select_value — select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
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
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/telemetry-line/event-line · Telemetry line · event line — One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
contract/telemetry-line/span-close-line · Telemetry line · span close line — One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — The sink drops a target outside the engine and escher prefix sets whole at every level.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word().
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
contract/acting-verb-results/caused-past · acting verb results · caused_past — Written after caused only when non-empty on click, type, press, advance and scroll: one entry per kind and source counting the items past caused's 16, a panic or error entry writing no source.
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`.
contract/answer-json/layout-fallback · answer JSON · layout fallback words — A later member of an engine set is written unexplained, transform, layout or auto, never a panic.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/argument-kinds/milliseconds · argument kinds · milliseconds — A whole number from 1 to 60000.
contract/argument-kinds/text · argument kinds · text — 0 to 4096 bytes of text.
contract/blitz-dom-crate-root/layout-reason-types · blitz-dom crate root · LayoutReason, Placed, Shift, SizeReason, FlexInputs and their enums — The layout reason's structs and its non-exhaustive enums PlacedBy, ShiftBy, SizeCause, Length and SizeKeyword are public read types re-exported from the crate root.
contract/blitz-dom-crate-root/select-list-view-types · blitz-dom crate root · SelectListView and SelectListRow — The open select list's view and rows are public read types re-exported from the crate root.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/cli-coverage-role/1 · Command line coverage role · 1 — A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
contract/cli-coverage-role/2 · Command line coverage role · 2 — Any other shape is usage.
contract/cli-coverage-role/3 · Command line coverage role · 3 — A boot that fails is the session error.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/cli-exit-statuses/2 · CLI exit statuses · 2 — usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
contract/cli-exit-statuses/3 · CLI exit statuses · 3 — session-error: the error object on stdout and its fixed message on stderr.
contract/command-span/added · command span · added — The length of the diff's added list.
contract/command-span/busy · command span · busy — The busy class word, recorded when the outcome carries one.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/command-span/changed · command span · changed — The length of the diff's changed list.
contract/command-span/passes · command span · passes — Settle's pass count, recorded when settle answered quiet.
contract/command-span/removed · command span · removed — The length of the diff's removed list.
contract/command-span/settled · command span · settled — Recorded when the outcome carries one.
contract/coverage-floor-exit-codes/1 · coverage_floor.py exit codes · 1 — Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-form-events/select-change · Dioxus form events · select Input and Change — A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
contract/dioxus-native-launch/config-downcast · dioxus-native launch · configs — Launch configs are downcast to `Features`, `Limits`, `WindowAttributes` or `Config`; an unmatched config is dropped.
contract/dom-events/driver-type-on-select · DOM event dispatch · driver type on a select — The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-mcp-exit-codes/3 · escher-session mcp role exit codes · 3 — The mcp role exits 3 on Err with the error's fixed message on stderr, writing nothing on stdout at its end.
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session.
contract/escher-session/advance · escher-session command line · advance --ms — The verb advance takes --ms.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/escher-session/serve · escher-session command line · serve <task> --session <dir> [--record <dir>] — The host role, which the start command spawns.
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once.
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role.
contract/escher-session/stop · escher-session command line · stop — The verb stop takes no argument beside --session.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/button-click-focus · Event dispatch · button click — Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/range-input · Event dispatch · range input input event — A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/event-dispatch/targeting · Event dispatch · event target — Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64.
contract/geometry/visible-region-select-list · blitz-dom geometry readers · visible_region of an open select list's row — A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
contract/graph-registry/debug · Graph registry · Debug — The registry's Debug prints counts alone.
contract/graph-registry/declared-write · Graph registry · DeclaredWrite — Names an element's stable id or a prefix ending in *, and an event written without on.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session.
contract/kept-check-reader/refusal · Kept check reader · malformed refusal — Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
contract/kept-check/bounds · Kept check file · size bounds — A kept check holds at most 400 records and 1,048,576 bytes.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/load-wait/no-time-moves · Driver acting step · wait side effects — No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/net-provider/is-noop · NetProvider · is_noop — Defaults to false; when true, callers register no pending-critical resources.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/nodes-widgets/range-input-data · blitz-dom nodes and widgets · RangeInputData — RangeInputData is read through the ElementData range accessors.
contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.
contract/paint/password-masked · blitz-paint · password input — A password input paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
contract/paint/select-list-order · blitz-paint · open select list painting — An open select's list is painted last, after the root's render and before the devtools overlays, read from SelectListView and past any clip or opacity on its select's ancestors.
contract/provider-traits/netprovider-noop · blitz-traits provider traits · no-op NetProvider — The no-op NetProvider registers no pending-critical resources.
contract/provider-traits/shellprovider · blitz-traits provider traits · ShellProvider — ShellProvider has no-op defaults.
contract/reached-record/debug · Reached record · Debug — The record's Debug prints its count alone.
contract/reached-record/reached-handler · Reached record · ReachedHandler — Carries id and event, re-exported from the crate root.
contract/reached-record/reached-handlers · Reached record · reached_handlers — Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
contract/reached-record/record-reached · Reached record · record_reached — Switches the record on at the root scope; a second call keeps the one record.
contract/reached-record/take-step-reached · Reached record · take_step_reached — Drains the handlers reached since the last drain, each once, in first-reached order.
contract/recording/directory · Session recording · record directory — A recording makes its directory owner-only or refuses it with `RecordDirNotPrivate`.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/scroll-hit/absolute-position · BaseDocument scroll and hit testing · Node::absolute_position — Node::absolute_position subtracts only ancestors' scroll offsets.
contract/scroll-hit/hit-with-scrollbar · BaseDocument scroll and hit testing · hit_with_scrollbar — hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
contract/scroll-hit/scroll-into-view · BaseDocument scroll and hit testing · scroll_into_view — scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/select-list-default-actions/choice · Select list default actions · choice — A choice from the list dispatches input then change.
contract/select-list-default-actions/click-select · Select list default actions · click on select — Clicking a single select opens its list, or closes one standing, focusing the select.
contract/select-list-default-actions/open-keys · Select list default actions · Enter and Space — Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more.
contract/select-list-default-actions/pointerdown · Select list default actions · pointerdown — A press in the open list stays there; a press elsewhere closes the list.
contract/select-list/open-select · BaseDocument open select list · open_select — open_select is public and read-only.
contract/select-list/select-list-bounds · BaseDocument open select list · select_list_bounds — select_list_bounds is public and read-only.
contract/select-list/select-list-view · BaseDocument open select list · select_list_view — select_list_view is public and read-only.
contract/select-list/select-row-layout · BaseDocument open select list · select_row_layout — select_row_layout is public and read-only.
contract/session-errors/record-dir-not-private · session errors · record-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
contract/session-errors/unsupported · session errors · unsupported — On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/session-wire/ok-accepted · session wire · ok v3 accepted — A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
contract/session-wire/version · session wire · version — The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
contract/snapshot-node-keys/expanded · Snapshot node keys · expanded — The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A multiple select's rows and groups read their own client rects as their bounds.
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
contract/stable-element-id/anchored-path · Stable element ids · anchored path tier — The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
contract/stable-element-id/component-path · Stable element ids · component path tier — The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.
contract/target-check/screenshot · Per-verb target check · screenshot with an id — A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
contract/telemetry-init/init · Telemetry init · init — Installs the stderr sink and returns Installed, or AlreadyInstalled with nothing changed on a second call.
contract/telemetry/init · escher-telemetry bootstrap · init — init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global.
contract/telemetry/line-format · escher-telemetry bootstrap · output line — One line is written per event and per closed span, stamped service.name and service.version.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.

## occupied-resources · ## Occupied Resources
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory.
decision/record-dir-launcher-named-private · Recording directory launcher-named and owner-only — The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
decision/recording-directory · Recording directory — The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
decision/telemetry-one-install-site · One telemetry install site per binary — Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run.
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields.
contract/recording-directory/platform · The recording directory · platform — Recording is unix only; elsewhere it is refused Unsupported.
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer.
contract/session-socket-wire/hello · the session socket's wire · hello — The hello request is served.
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported.

## infrastructure-patterns · ## Infrastructure Patterns
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
decision/browser-load-aborts-previous · A browser document load aborts the previous load — Each browser document load aborts the previous one, as reload and drop do. · why: The browser's tab navigation calls load_document and the toolbar reload calls reload, so a superseded page's fetches stop; DocumentLoader's Drop aborts too
decision/capabilities-doc-beside-upstream · Capability statement beside upstream docs — escher's capability statement sits in a file of its own under docs, beside upstream's three files. · why: an upstream change under docs never conflicts with it

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber.
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified.
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.
contract/telemetry-line/service-name · escher telemetry log line · service.name — Stamped on every line.
contract/telemetry-line/service-version · escher telemetry log line · service.version — Stamped on every line.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
decision/escher-capitalised-in-prose · Escher capitalised in prose — The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged.
decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
decision/readme-is-escher-own · Front page is escher's own — The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
decision/readme-is-escher-page · Root README is escher's own page — The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.
decision/upstream-sync-merge-base · Upstream sync merge base and cherry-picks — The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
decision/upstream-taken-by-cherry-pick · Upstream changes taken by cherry-pick — Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason.
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.

## inherited-defaults · ## Inherited Defaults
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
decision/embedder-services-default-noop · Embedder services default to no-ops — Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/engine-tracing-gated-per-call-site · Engine tracing gated per call site — Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off. · except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.

## existing-scopes · ## Existing Scopes
decision/keyboard-compat-coverage-exclusion · Upstream keyboard_compat out of the coverage leg — The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename. · why: It is upstream's code, kept until escher takes upstream's retirement of it.
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
