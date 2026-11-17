# architecture · the rows that bind 2026-11-17-a-component-in-isolation — picked 295 of 1529 (165 by every draw) · rule any · pin 1646

## design-philosophy · ## Design Philosophy
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.

## stack-and-technologies · ## Stack and Technologies
decision/cold-agent-mcp-server-is-own · Cold-agent MCP server is the workspace's own — The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
decision/dioxus-drives-blitz · Dioxus as the UI framework — The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
decision/no-prng-crate · No PRNG crate for seeded randomness — Seeded randomness takes no PRNG crate: splitmix64 is written by hand and the host fallback seed comes from std RandomState. · why: no package enters the lockfile
decision/cold-agent-client-host-only · Cold-agent client is a host tool — The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/keyboard-types-named-by-absolute-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types. · why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
decision/parley-text-layout · Parley for text and fonts — Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
decision/workspace-keyboard-types-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`. · why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7

## established-decisions · ## Established Decisions
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
decision/sequential-style-default · Sequential style threading by default — Style threading defaults to Sequential, which bypasses Stylo's global thread pool.
decision/session-library-depends-on-no-app · Session library depends on no app — The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/skipped-font-formats · Skipped font formats — SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/woff-default-in-dioxus-native · woff on by default in dioxus-native — The `woff` feature is default-on in dioxus-native. · why: so wasm callers, which cannot use system fonts, can bundle compressed fonts
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified
decision/author-stylesheet-order · Author stylesheets ordered by owner node id — Author stylesheets are ordered by owner node id.
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/dioxus-no-own-stylesheet · Dioxus document adds no stylesheet — A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported.
decision/optgroup-group-role-named-by-label · Optgroup reads the group role named by its label — The accessibility tree gives an optgroup the group role named by its non-empty label. · why: The driver's snapshot reads it; pinned by a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds
decision/option-disabled-own-or-group · Option disabled by itself or its group — An option is disabled by its own attribute or its group's.
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, the placing words select and select-list in a layout answer, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/stylo-prefs-set-once · Stylo prefs set once, by Stylo 0.22 names — Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1. · why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
decision/upstream-behaviour-changes-ratified-per-take · Upstream behaviour changes ratified per take — An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP.
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0.
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode.
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.

## conventions · ## Conventions
decision/dioxus-mutation-buffer-replay · Dioxus bridge buffers then replays mutations — The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer. · why: Works around borrow conflicts on the Dioxus bridge.
decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error.
decision/examples-panic-on-error · Examples fail by panicking — Examples fail with `.unwrap()`, `.expect(..)` or `panic!`.
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result. · why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/asset-macro · Assets through the asset macro — Apps reference assets through `asset!`.
decision/assets-through-asset-macro · Assets through the asset macro — Assets in apps and examples come through the asset! macro.
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.

## standard-contracts · ## Standard Contracts
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
decision/layout-answer-crossing · Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/render-with-document-unborrowed · Render with the document unborrowed — The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush. · why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
contract/accessibility-tree/img-empty-alt · Accessibility tree · img with an empty alt — An img whose alt is empty reads as GenericContainer, decorative and unnamed.
contract/accessibility-tree/name-sources · Accessibility tree · accessible name — A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
contract/accessibility-tree/native-control-toggled · Accessibility tree · native control toggled — A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
contract/accessibility-tree/node-names · blitz-dom accessibility tree · node names — Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
contract/accessibility-tree/non-checkable-role · blitz-dom accessibility tree · native control under a non-checkable role — A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/answer-json/layout · answer JSON · layout — {"id","bounds","laid_out","placed":{"by","container"?,"canvas"?,"at","relative"?},"shifts":[{"by","holder"?,"dx","dy"}],"width","height"}, each size {"px","by","size","min","max","flex"?}; ids, closed words and numbers only.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dioxus-native-dom-api/boundary-marks · dioxus-native-dom public API · boundary_marks and BoundaryMark — Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; the registry grows for the document's life, since a dropped boundary cannot be told.
contract/dioxus-native-dom-api/use-seeded-random · dioxus-native-dom public API · use_seeded_random and DEFAULT_SEED — Public and re-exported: use_seeded_random returns the root's SeededRandom from a root context, else a generator seeded from the host's per-process random keys, the windowed shell's case outside any run; DEFAULT_SEED is the u64 seed 0.
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fifteen verbs, eight closed causes.
contract/driver-session/layout · escher-driver session · Session::layout — A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the eleven instance verbs are kept, `lint`, `graph`, `actions` and `layout` among them with their answers whole; any other call is not kept.
contract/layout-reason/length · blitz-dom layout reason · Length — A stated length is closed: Auto, None, Px, Percent (0 to 100), Calc in px, or a SizeKeyword of MinContent, MaxContent, FitContent or Stretch.
contract/layout-reason/placed-by · blitz-dom layout reason · PlacedBy — The placing word is closed: Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList.
contract/layout-reason/shift-by · blitz-dom layout reason · ShiftBy — The shift word is closed: Scroll or Transform.
contract/layout-reason/size-cause · blitz-dom layout reason · SizeCause — The cause of a size is closed: Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout.
contract/mcp-server/tools-list · MCP server · tools/list — The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/net-provider/is-noop · NetProvider · is_noop — Defaults to false; when true, callers register no pending-critical resources.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/provider-traits/netprovider-noop · blitz-traits provider traits · no-op NetProvider — The no-op NetProvider registers no pending-critical resources.
contract/snapshot-node-keys/expanded · Snapshot node keys · expanded — The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
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
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/test-harness/run-seed · blitz-test-harness · harness run seed — Harness::from_vdom provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, before the document is built; a SeededRandom the builder gave as a root context stands; Harness::wrap provides none.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
contract/accessibility-tree/aria-labelledby · Accessibility tree · aria-labelledby — The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/select-expanded · blitz-dom accessibility tree · single select expanded — A single select writes its native expanded, true while its list stands open, with no aria-expanded read on it; every other element keeps the aria-expanded read.
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/layout-fallback · answer JSON · layout fallback words — A later member of an engine set is written unexplained, transform, layout or auto, never a panic.
contract/answer-json/layout-length · answer JSON · layout length — A length is "auto", "none", "min-content", "max-content", "fit-content", "stretch", {"px":n}, {"percent":n} or {"calc":n}.
contract/answer-json/layout-placing · answer JSON · layout placing word — placed's by is one of block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app, written kebab-case.
contract/answer-json/layout-shift · answer JSON · layout shift word — A shift's by is scroll or transform.
contract/answer-json/layout-size-cause · answer JSON · layout size cause word — A size's by is one of style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cold-agent-verdict/passed · cold-agent verdict.json · passed — The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all six apps met, and the agent's final check replaying passing and failing on an ablated replay.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/default-script-fetcher/schemes · Default script fetcher · schemes — The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
contract/dioxus-native-dom-api/layout-reason-types · dioxus-native-dom public API · layout reason types and BoundingRect — Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect.
contract/dioxus-native-dom-api/seeded-random · dioxus-native-dom public API · SeededRandom — Public and re-exported: a splitmix64 generator whose clones share one state, so every component of one document draws from one stream in draw order; from_seed(u64), next_u64 and below(bound), the high bits of the 128-bit product, 0 for a bound of 0, no panic path; Debug writes the type name alone.
contract/document-clock/clock-reexports · blitz-dom document clock · Clock, WallClock and VirtualClock re-exports — The blitz-dom crate root re-exports Clock, WallClock and VirtualClock.
contract/document-config/clock · DocumentConfig · clock — An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
contract/document-trait/poll · Document trait · poll — Defaults to returning false.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/layout-unexplained · escher-driver session · unexplained placer — An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the eleven instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/provider-traits/viewport-scale · blitz-traits provider traits · Viewport scale() — A `Viewport`'s `scale()` is hidpi_scale × zoom.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/unknown-verb · refusal causes · unknown-verb — The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/stable-element-id/code-edit-stability · Stable element ids · stability across code edits — An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's.
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.

## occupied-resources · ## Occupied Resources
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server. · why: chosen as one run capped at $3, with a JSON check and a retry
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/model-provider-only-on-live-cold-agent-run · Model provider reached only on the live cold-agent run — The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request. · why: ratified
decision/model-provider-operator-only · Model provider reached only on operator runs — The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields.

## infrastructure-patterns · ## Infrastructure Patterns
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light.
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/mounted-after-build-and-poll · Mounted listeners fire after build and poll — Dioxus mounted listeners fire after the initial build and after each poll.
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/embedder-services-default-noop · Embedder services default to no-ops — Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.

## existing-scopes · ## Existing Scopes
decision/dioxus-bridge-buffered-render · Dioxus bridge buffered render — The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts. · guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.

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
