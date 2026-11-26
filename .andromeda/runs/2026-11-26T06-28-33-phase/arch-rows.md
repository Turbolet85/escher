# architecture · the rows that bind 2026-11-26-canvas-keyboard-path — picked 328 of 1592 (196 by every draw) · rule any · pin 4036

## design-philosophy · ## Design Philosophy
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.
decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · except: blitz-vibey-script keeps its own integration tests in its tests directory
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport
decision/paint-tree-topology-only · Paint tree holds topology only — The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation. · why: so a cached list cannot paint a box where layout no longer puts it · guard: paint_tree_incremental
decision/traits-crate-for-interop · Shared traits crate for interoperability — Types and traits shared between Blitz crates live in blitz-traits. · why: so the crates interoperate without circular or unnecessary dependencies
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.

## stack-and-technologies · ## Stack and Technologies
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
decision/fork-additions-in-own-functions · Fork additions in functions of escher's own — A fork addition to an upstream function over the cognitive complexity ceiling goes into a function of escher's own, with one call in upstream's body. · why: ratified
decision/keyboard-types-named-by-absolute-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types. · why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
decision/workspace-keyboard-types-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`. · why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7
decision/geometry-and-color-crates · Geometry and color crates — Geometry and color come from kurbo, euclid, app_units, color and peniko.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py

## established-decisions · ## Established Decisions
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/canvas-command-grouping · Canvas commands grouped under virtual nodes — A canvas reading groups each draw command under the canvas's virtual node whose region holds the command's centre, the region whose centre lies nearest first, by its 1-based positional index, or none; a glyph run's bounds are its ink box, its glyph-origins box the fallback. · why: A glyph run's origins sit on its baseline and group some labels under another node, while its ink box groups every label under its own.
decision/canvas-crossing · Canvas content crosses as draw commands and pixels — A canvas's app-painted content crosses the driver's carriers, the session socket, the command line, MCP and a kept check, as the canvas verb's answer of draw commands beside a screenshot's pixels, with no new carrier, exit, cause, session error or span field. · why: A boundary widening, ratified.
decision/canvas-reading-diffable · Canvas reading deterministic and diffable by line — A canvas reading is deterministic and diffable by line: two reads give the same bytes, a changed command reads as one line removed and one added, both closing on one node tag where a virtual node groups it, and a layer's filter is written as a flag only.
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a list box's row focuses its select; a click on a custom-widget target runs no click walk and clears no focus
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/custom-widget-content-box-paint · Custom widget painted at its content box — A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0, and its scene is appended at the content origin inside the element's transform, unclipped. · why: Ratified; answers upstream issue 1578, a direct engine change flagged upstreamable · guard: stand_canvas_placement
decision/custom-widget-object · Custom content as an object element — Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/driver-accessibility-feature · Driver names only the accessibility feature — The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own. · why: Its executor cannot be built without the snapshot model.
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/focusability-disabled-presence · Focusability reads disabled by presence — Focusability reads the disabled attribute by presence, and on a form control alone. · why: ratified
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents. · why: Each is a direct engine change, flagged upstreamable.
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified
decision/key-types-through-harness · Key types through the harness's re-exports — The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, canvas readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/repeated-click-window · Repeated-click window — Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP and rests in a kept check as the graph verb's answer, whole or in parts: stable ids, owner ids, component, state and handler event names, declared writes, flags undeclared and unconfirmed, booleans, part counts; no value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/virtual-node-role-is-accesskit-role · A declared virtual node's role is AccessKit's Role — A declared virtual node's role is AccessKit's Role itself, read by the tree as declared, with no closed role set or second role mapping of escher's own. · why: chosen over an escher role enum, a second role table to keep in step with the tree; the public declaration API names AccessKit's type in exchange · provisional
decision/widget-click-point-content-box · Widget click point read against the content box — An app that places custom-widget content from a click point reads that point against the widget's content box, subtracting border and padding from the element's client rect. · why: A widget paints in its content box while an element's client rect is its border box. · guard: stand_circle_drawer_canvas
decision/widget-click-synthesis · Click synthesis on a custom-widget target — On a custom-widget target the engine, behind the custom-widget feature, synthesises the click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click, after the widget's own handling and the focus set, with no click walk and no focus clear. · why: Ratified as a direct engine change for every document behind custom-widget, flagged upstreamable; upstream's widget arm returned before any click, the widget still hearing every pointer event first. · guard: stand_canvas_click
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged.
contract/coverage-role/answer · the command line's coverage role · answer — One line on stdout of ids, owner ids, event names and two counts.
contract/custom-widget-embed/object-data · custom widget embedding · object data — Custom content embeds as an object element whose data carries a CustomWidgetAttr.
decision/anonymous-block-drop-clears-layout-parent · Dropped anonymous block clears its children's layout parent — Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts. · why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`.
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/harness-paints-custom-widgets · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness. · guard: stand_canvas_shot
decision/object-not-replaced-element · Object excluded from replaced elements — Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced. · why: so an object's fallback children render
decision/open-pseudo-class-details-dialog · :open matches details or dialog carrying open — :open matches a details or dialog that carries the open attribute. · why: Pinned by the open_state tests at_construction_open_is_read_from_the_attribute and a_write_and_a_removal_through_the_mutator_move_it; select_popup checks a select never matches
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.

## conventions · ## Conventions
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features.
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error.
decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`.
decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`.

## standard-contracts · ## Standard Contracts
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified
decision/canvas-placement-at-click-centre · Canvas placement only at the click centre — A driver click lands at the centre of its target element, or on a canvas at a declared virtual node's region, so an agent places content on a canvas only at the canvas's centre or a declared region, no verb landing a pointer elsewhere. · why: Stated as a measured limit of the driver where an agent reads it.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified.
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports the keyboard types a key press is named with. · why: A crate depending on the harness names a key without a keyboard dependency of its own.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/reached-record-before-dispatch · Reached record resolved before dispatch, written after — The reached record resolves the chain's nodes whose current listeners hold the event before it runs, holding no borrow across the render, and writes them after: all while it propagates, the innermost alone when stopped or a handler's panic unwinds, a non-bubbling event's target alone. · why: So a handler whose own run removes its element still counts.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/virtual-id-verb-table · One table of the verbs taking a virtual id — The verbs that may take a virtual node id are named in one table keyed on the verb, which the run path, the in-process layout and actions all read, so no reader keeps its own list of them. · why: Keyed on the verb rather than a command, so the in-process layout, which holds no command, reads the same table as click and actions. · provisional
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
contract/accessibility-tree/active-descendant · Accessibility tree · active descendant — While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select and a page's aria-activedescendant is never read.
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
contract/accessibility-tree/virtual-nodes · Accessibility tree · virtual nodes — Under the accessibility feature the Dioxus document's tree holds each canvas's taken virtual nodes as children of the canvas's node, after its own: role and label as declared, author_id {owner}///{key}, disabled where declared; a hidden, collapsed or inert canvas, or one whose read errs, takes none.
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs.
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout` · `canvas`, `layout` with no cause and `canvas` with none on a document that can paint text.
contract/actions-verb/virtual-node · The actions verb · virtual node — A listed virtual node's verbs are read against its canvas, each cause the one its call returns: a verb that takes no virtual id reads not-found, and click and layout meet the checks their call meets.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent, highlighted written only when true; id, role, name, invalid, focused, clickable and bounds always written.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/canvas-verb/brush-forms · The canvas verb · brush forms — A brush is one of five closed forms: solid, gradient, image, resource or pattern.
contract/canvas-verb/command-kinds · The canvas verb · command kinds — The command kind set is closed at eight, named Layer · Clip · Mask · Pop · Fill · Stroke · Glyphs · Shadow.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and the accessibility tree serves it, and from it the snapshot, its diff, the UI graph, actions and layout.
contract/dioxus-bridge/virtual-nodes · Dioxus DOM bridge · DioxusDocument::virtual_nodes — An ungated in-process read of one canvas's declarations, answering the taken nodes, each with id {owner}///{key}, and the refused declarations by index, or NoCanvas for a stale or non-canvas node and NotLaidOut before the first layout; no panic path.
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-config/clock · DocumentConfig · clock — An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/dom-events/focus-change · DOM event dispatch · focus change — A focus change dispatches blur, focusout, focus, focusin, except the focus fixups under inert and after a layout for a node that no longer shows, which clear focus with none.
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-session/canvas · escher-driver session · Session::canvas — A read answering a CanvasReading of one canvas in process, resolving its id as the run path does with a virtual id read not-found, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/canvas · driver verbs · canvas — Instance level, after layout; takes id, optional full and part (1 to 1000); answers a summary of per-kind command counts and digests, with full a line-per-command dump, in parts past the snapshot text budget; refused not-found, stale, fonts-unavailable or malformed; changes nothing.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.
contract/driver-verbs/click-virtual-node · driver verbs · click on a virtual node id — click takes a virtual node id, as layout does: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run; type, scroll, screenshot with an id and canvas read a virtual id not-found.
contract/driver-verbs/graph · driver verbs · graph — One optional part (1 to 1000); result nodes and states, plus part and parts when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked; dispatches nothing, runs no step.
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/focus-change · Event dispatch · focus change — A focus change, a programmatic one included, dispatches blur, focusout, focus, focusin; only the focus fix-up under inert, the focus fix-up after a layout for a node that no longer shows, and the driver's select focus dispatch none.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/event-dispatch/targeting · Event dispatch · event target — Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
contract/form-controls/is-disabled · blitz-dom form control readers · ElementData::is_disabled — ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/unpaged · The graph verb · paging — graph answers whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes within it; the states ride in part 1 alone, a later part's empty states meaning read part 1, and a part past the last answers empty nodes and states.
contract/handler-coverage/canvas · Handler coverage reader · canvas handlers — A virtual node's click is listed under its {owner}///{key} id, owned by the canvas, exercised where a replayed click named it; a canvas click landing inside a node counts the canvas's own listener under the canvas's id alone.
contract/handler-coverage/reached · Handler coverage reader · reached handlers — The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, Circle Drawer, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font; Circle Drawer's boot runs in process only, served by no escher-session role.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/key-names/arrow-down · key names · arrow-down — A key name press accepts.
contract/key-names/arrow-left · key names · arrow-left — A key name press accepts.
contract/key-names/arrow-right · key names · arrow-right — A key name press accepts.
contract/key-names/arrow-up · key names · arrow-up — A key name press accepts.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/key-names/tab · key names · tab — A key name press accepts.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/nodes-widgets/virtual-nodes · blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration — Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name, region as a BoundingRect and a disabled flag; the engine never calls it.
contract/reached-record/reached-handlers · Reached record · reached_handlers — Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
contract/reached-record/take-step-reached · Reached record · take_step_reached — Drains the handlers reached since the last drain, each once, in first-reached order.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings, and a virtual node reads covered only by an element over its region's centre.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot, lint and canvas refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/scroll-hit/hit-clip · BaseDocument scroll and hit testing · hit on a clipping box — A hit stops at the padding box of a box paint clips, and a hit stopped there answers in that box's content-box frame, the frame of the final self hit and the text hits.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/reading · Snapshot diff · Reading — Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, a virtual node's region moved by its canvas's content-box origin and read untransformed, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/virtual-node · Snapshot · virtual node — A virtual node reads its id, declared role and name, enabled as not declared disabled, invalid and focused, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/virtual-node-id · Stable element ids · virtual_node_id — A reserved fifth form {owner}///{key}, given a canvas's taken nodes and served in the accessibility tree and from it the snapshot, its diff, the UI graph, actions and layout, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/test-harness/canvas-scene · blitz-test-harness · canvas_scene — Harness::canvas_scene reads one custom widget's scene at the viewport's scale, with no resolve, tick or pump.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to, or of the canvas that declares a virtual node.
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word().
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region and paint narrow only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified.
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The seven served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
contract/accessibility-tree/aria-selected · Accessibility tree · aria-selected — true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/platform-refresh · blitz-dom accessibility tree · platform tree refresh — blitz-shell refreshes the platform accessibility tree on a redraw, after resolve, when the changed set is non-empty, and then clears the set.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/blitz-dom-crate-root/virtual-node-role · blitz-dom crate root · VirtualNodeRole — Behind custom-widget, the crate root re-exports AccessKit's Role as VirtualNodeRole, the type a virtual node declaration's role takes, so an app reaches it without naming accesskit.
contract/ci-leg-script/leg-log · ci-leg.sh invocation · leg log — The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
contract/ci-log-artifacts/ci-log · fork CI log artifacts · ci-log-{job id} — Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
contract/cold-agent-verdict/refused-by-cause · cold-agent verdict.json · refused_by_cause — The cold-agent verdict carries `refused_by_cause`, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/document-clock/clock-reexports · blitz-dom document clock · Clock, WallClock and VirtualClock re-exports — The blitz-dom crate root re-exports Clock, WallClock and VirtualClock.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/document-trait/handle-ui-event · Document trait · handle_ui_event — Defaults to dispatch through an EventDriver with a NoopEventHandler.
contract/dom-events/inert-focus-fixup · DOM event dispatch · focus fixup under inert — The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
contract/dom-events/pointer-events · DOM event dispatch · pointer-derived events — Mouse compatibility events and touch events are dispatched from pointers.
contract/driver-explore/expansion · escher-driver explore · path expansion — A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
contract/driver-explore/explore · escher-driver explore · explore — An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching through its earlier calls; a boot error ends it.
contract/driver-explore/explored-verbs · escher-driver explore · explored verbs — The verbs explored are the actions table's target verbs less any whose result holds an image, a layout reading or a canvas reading field (png, rect, placement, shifts, size, commands or command-counts kind), selected by the field's kind and never its name; today click, type and scroll.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/layout-virtual-node · escher-driver session · app placer of a virtual node — Layout on a virtual node id answers it placed by app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes layout; the engine is never asked for a region's reason.
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/event-dispatch/non-bubbling · Event dispatch · non-bubbling events — A non-bubbling event reaches only its target.
contract/event-dispatch/pointer-compatibility-events · Event dispatch · pointer compatibility events — Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.
contract/geometry/visible-region-non-finite · blitz-dom geometry readers · visible_region on a non-finite box — visible_region answers None for a mapped box with a non-finite edge.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}`, a part's adding `part` and `parts`, holds ids, component, state and event names, closed flag words, booleans and the part counts only, never a value.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/handler-coverage/bubbling · Handler coverage reader · bubbling dispatch — A bubbling dispatch that still propagates counts every listener it passed; a dispatch a listener stops, or a handler's panic unwinds, counts its innermost listener alone.
contract/handler-coverage/per-check-boot · Handler coverage reader · per-check boot — A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
contract/handler-coverage/unreached-screens · Handler coverage reader · unreached screens — A handler on no screen a replay reached is not listed, by construction.
contract/handler-coverage/written-form · Handler coverage reader · written form — Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
contract/lint-reason-keys/truncation · lint reason keys · truncation — text then visible, visible left out when no part can be seen.
contract/lint-verb/clipped-off-screen · The lint verb · clipped and off-screen — `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
contract/lint-verb/finding · The lint verb · finding — The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order, a canvas element's `truncated` read once per label in paint order.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`; a canvas element reads `truncated` once per label whose ink box does not lie inside its region.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/load-wait/no-time-moves · Driver acting step · wait side effects — No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.
contract/mcp-server/request-line-bound · MCP server · request line bound — A request line of at most 65,536 bytes; a longer one is read to its terminator, dropped and answered -32700.
contract/mcp-server/result-bound · MCP server · result bound — A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
contract/mcp-server/tools-list · MCP server · tools/list — The sixteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/paint/custom-widget-scene · blitz-paint · custom_widget_scene — custom_widget_scene returns one custom widget's scene outside a raster at a given scale, behind the custom-widget feature, beside build_custom_widget_scenes; an escher addition flagged upstreamable.
contract/reached-record/reached-handler · Reached record · ReachedHandler — Carries id and event, re-exported from the crate root.
contract/reached-record/record-reached · Reached record · record_reached — Switches the record on at the root scope; a second call keeps the one record.
contract/refusal-causes/malformed · refusal causes · malformed — An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it, or names a virtual node to a verb other than click and layout; nothing runs.
contract/refusal-faults/out-of-bound · refusal faults · OutOfBound — An argument's value is outside its bound, or a text is no key name; beside malformed only.
contract/script-events/dispatch-order · Script event handling · dispatch order — Events reach JS listeners before Blitz default actions, and JS `preventDefault` / `stopPropagation` are fed back into the event state.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/select-list-default-actions/open-keys · Select list default actions · Enter and Space — On a drop-down Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more; on a single list box neither opens anything.
contract/select-list-default-actions/tab · Select list default actions · Tab — Tab, either direction, chooses the current row and closes the list before moving focus.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable, highlighted; absent where the model holds nothing, invalid, clickable and highlighted written only when true.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves, or a canvas's taken virtual node; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/unkeyed-actionable/unkeyed-actionable · Actionable-key check · unkeyed_actionable — Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds sixteen verbs, `graph` listed after `lint`, `actions` after `graph`, `layout` after `actions` and `canvas` after `layout`.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.

## occupied-resources · ## Occupied Resources
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed, booleans and a part's part and parts counts; never a value, typed text or NodeId.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider from the driver's own MCP server: element ids, accessible names, control values, snapshot texts, diffs, graph and actions answers, and acting answers' caused text, the order page's authored messages among it. · why: chosen as one run capped at $3, with a JSON check and a retry
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-graph/argument · the graph verb · argument — graph takes one optional part, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused malformed with nothing run.
contract/mcp-role/handshake-revision · the mcp role · handshake revision — The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default.
contract/session-socket-wire/answer-bound · the session socket's wire · answer bound — An answer is at most 1,048,576 bytes.
contract/session-socket-wire/request-line-bound · the session socket's wire · request line bound — A request line is at most 16,384 bytes.

## infrastructure-patterns · ## Infrastructure Patterns
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a redraw, after resolve, only when the document's changed set is non-empty, and then clears the set. · why: ratified as the shell half of the changed-set item
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked changes, typed text, range moves, value-changing select choices and a select list opening, closing or moving its highlight to a new row; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node
decision/declaration-change-marks-nothing · A virtual node declaration change marks nothing — A change of what a canvas declares writes no changed-set mark and sends no invalidation signal, so a windowed accessibility adapter sees it only at its next rebuild, while each snapshot reads the declarations afresh. · why: a mark or a signal would amend the ratified changed-set write sites and marks for every document
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled, and preventDefault on a non-cancelable event cancels nothing. · except: a cancelled pointerup still clicks and still ends a scrollbar drag
decision/events-queued-on-document · Events queued on the document — Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer; queued events for dropped nodes are discarded. · guard: event_queue
decision/forwarded-event-coordinates · Forwarded events adjust coordinates — Sub-document and custom-widget events are forwarded with adjusted coordinates.
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled. · except: a cancelled pointerup still clicks and still ends a scrollbar drag
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
decision/reached-recorded-before-handlers · Reached record resolved before handlers run — Where a reached record is switched on, the chain's nodes whose current listeners hold the event's name are resolved by stable id before the handlers run and written after: all while it propagates, the innermost alone when stopped or unwound by a panic, a non-bubbling event's target alone. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
decision/sub-document-event-forwarding · Events forwarded to sub-documents and widgets — Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space.
decision/dioxus-skips-unhandled-event-kinds · Dioxus skips unhandled event kinds — Dioxus skips event kinds with zero handlers.
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
decision/cold-agent-result-stated-measured · Cold-agent result stated as measured, with its reach — The capability statement states a cold-agent result as measured, with its reach: the measured instance names its apps, tools, calls, wrong calls and client, and every capability outside that run's reach is named with its headless stand witness. · why: Ratified.
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.

## inherited-defaults · ## Inherited Defaults
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest

## existing-scopes · ## Existing Scopes
decision/dioxus-bridge-buffered-render · Dioxus bridge buffered render — The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts. · guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot holding document text differs in PNG bytes between linux x86_64 and macOS aarch64; a canvas-only shot reads equal on linux x86_64, windows x86_64 and macOS aarch64. · except: A canvas-only shot, held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 by one committed digest table, no pixels committed; a shot holding document text stays out.
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
absence/canvas-lint-checks-beyond-truncation — Lint checks in a canvas beyond label truncation: label overlap, label contrast, out-of-region commands, no-area shapes
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port in a default build: no TCP or UDP port any binary serves with its default features
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-cdp-server — blitz-vibey-cdp-server
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
name/other/blitz-cdp-port — BLITZ_CDP_PORT
name/other/blitz-devtools-frontend-dir — BLITZ_DEVTOOLS_FRONTEND_DIR
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/ci — CI
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
name/other/skia-binaries-url — SKIA_BINARIES_URL
name/other/target-sweep-ps1 — target-sweep.ps1
name/other/target-sweep-py — target-sweep.py
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener in any default feature set
