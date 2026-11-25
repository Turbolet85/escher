# architecture · the rows that bind 2026-11-25-virtual-nodes-as-elements — picked 302 of 1584 (176 by every draw) · rule any · pin 3791

## design-philosophy · ## Design Philosophy
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization
decision/single-incremental-pipeline · One incremental pipeline — Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
decision/traits-crate-for-interop · Shared traits crate for interoperability — Types and traits shared between Blitz crates live in blitz-traits. · why: so the crates interoperate without circular or unnecessary dependencies

## stack-and-technologies · ## Stack and Technologies
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
decision/geometry-and-color-crates · Geometry and color crates — Geometry and color come from kurbo, euclid, app_units, color and peniko.
decision/keyboard-types-named-by-absolute-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types. · why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.

## established-decisions · ## Established Decisions
decision/bridge-removal-drops-subtree · Bridge removal drops the subtree — A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
decision/click-after-scroll-advances-past-fade · Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing. · why: a measured hazard of the overlay scrollbar, no engine change · except: A row of an open select list is never taken as a thumb drag.
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a list box's row focuses its select; a click on a custom-widget target runs no click walk and clears no focus
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/custom-widget-content-box-paint · Custom widget painted at its content box — A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0, and its scene is appended at the content origin inside the element's transform, unclipped. · why: Ratified; answers upstream issue 1578, a direct engine change flagged upstreamable · guard: stand_canvas_placement
decision/custom-widget-object · Custom content as an object element — Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/driver-accessibility-feature · Driver names only the accessibility feature — The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own. · why: Its executor cannot be built without the snapshot model.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents. · why: Each is a direct engine change, flagged upstreamable.
decision/hit-stops-at-clip-padding-box · Hits stop at clipping padding boxes — A hit stops at the padding box of every box paint clips.
decision/hit-stops-at-paint-clip · Hit walk stops where paint clips — A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk. · why: Uses paint's own clipping predicate so hit and paint agree; ratified. · guard: hit_clipped_at_scrolling_box · except: the root element
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph, actions and canvas answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified.
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, canvas readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/node-id-layout · NodeId as slot index plus version — `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version. · why: the ordering roughly matches creation order
decision/non-invertible-transform-takes-no-hit · Non-invertible transform takes no hit — A box under a non-invertible transform takes no hit. · why: a ratified direct engine change for every document, flagged upstreamable
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/virtual-node-declaration-marks-nothing · A declaration change marks and signals nothing yet — A change of a canvas's declared virtual nodes writes no changed-set mark and sends no invalidation signal; both are left to the first reader that refreshes, «Virtual nodes as elements». · why: a mark or a signal now would amend the ratified changed-set write sites and marks for every document, while no reader in this chunk refreshes on it · provisional
decision/virtual-node-role-is-accesskit-role · A declared virtual node's role is AccessKit's Role — A declared virtual node's role is AccessKit's Role itself, read by the tree as declared, with no closed role set or second role mapping of escher's own. · why: chosen over an escher role enum, a second role table to keep in step with the tree; the public declaration API names AccessKit's type in exchange · provisional
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable
decision/widget-click-point-content-box · Widget click point read against the content box — An app that places custom-widget content from a click point reads that point against the widget's content box, subtracting border and padding from the element's client rect. · why: A widget paints in its content box while an element's client rect is its border box. · guard: stand_circle_drawer_canvas
decision/widget-click-synthesis · Click synthesis on a custom-widget target — On a custom-widget target the engine, behind the custom-widget feature, synthesises the click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click, after the widget's own handling and the focus set, with no click walk and no focus clear. · why: Ratified as a direct engine change for every document behind custom-widget, flagged upstreamable; upstream's widget arm returned before any click, the widget still hearing every pointer event first. · guard: stand_canvas_click
contract/custom-widget-embed/object-data · custom widget embedding · object data — Custom content embeds as an object element whose data carries a CustomWidgetAttr.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`.
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/hit-and-paint-skip-dropped-nodes · Hit test and paint tree skip nodes dropped before resolve — A hit test and the paint tree's walk skip a node dropped since the last resolve. · why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen's snapshot or UI graph over the snapshot budget crosses in parts, one within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified.
decision/open-select-list-hit-first · An open select list answers hits first — Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either, the words select and select-list in a layout answer, and pixels in a screenshot; no new carrier, exit, cause, session error, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/unstyled-elements-hit-tested · Elements without computed styles are hit tested — Elements with no computed styles yet are hit tested, and such a node is never a scroll container. · why: an upstream behaviour change taken, ratified for its take
decision/z-rotate-planar · Z-axis rotate applied as planar — 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation. · guard: rotate_z_axis
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.

## conventions · ## Conventions
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features.
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.

## standard-contracts · ## Standard Contracts
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/canvas-placement-at-click-centre · Canvas placement only at the click centre — A driver click lands at the centre of its target element, so an agent places content on a canvas only at the canvas's centre, no verb landing a pointer elsewhere. · why: Stated as a measured limit of the driver where an agent reads it.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region and paint narrow only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified.
decision/seen-ids-record-bounded · Session's record of seen ids bounded — Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged. · why: Ratified for ids longer than a call can name. · guard: the_record_holds_no_id_longer_than_a_call_can_name
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/build-accessibility-tree · blitz-dom accessibility tree · build_accessibility_tree — build_accessibility_tree returns an AccessKit TreeUpdate.
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs.
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed.
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, canvas answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and no carrier serves it.
contract/dioxus-bridge/virtual-nodes · Dioxus DOM bridge · DioxusDocument::virtual_nodes — An ungated in-process read of one canvas's declarations, answering the taken nodes, each with id {owner}///{key}, and the refused declarations by index, or NoCanvas for a stale or non-canvas node and NotLaidOut before the first layout; no panic path.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/dom-events/pointer-events · DOM event dispatch · pointer-derived events — Mouse compatibility events and touch events are dispatched from pointers.
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/driver-session/canvas · escher-driver session · Session::canvas — A read answering a CanvasReading of one canvas in process, resolving its id as Session::layout does, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes.
contract/driver-session/layout · escher-driver session · Session::layout — A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/canvas · driver verbs · canvas — Instance level, after layout; takes id, optional full and part (1 to 1000); answers a summary of per-kind command counts and digests, with full a line-per-command dump, in parts past the snapshot text budget; refused not-found, stale, fonts-unavailable or malformed; changes nothing.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused; one line of JSON answers it on stdout.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/event-dispatch/pointer-compatibility-events · Event dispatch · pointer compatibility events — Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen.
contract/event-dispatch/targeting · Event dispatch · event target — Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
contract/event-state/fields · EventState · fields — cancelled, propagation_stopped and redraw_requested, combined by merge.
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64.
contract/geometry/visible-region-non-finite · blitz-dom geometry readers · visible_region on a non-finite box — visible_region answers None for a mapped box with a non-finite edge.
contract/geometry/visible-region-transforms · blitz-dom geometry readers · visible_region through transforms — visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/nodes-widgets/virtual-nodes · blitz-dom nodes and widgets · Widget::virtual_nodes and VirtualNodeDeclaration — Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name and region as a BoundingRect; the engine never calls it.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/refusal-causes/stale · refusal causes · stale — The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/session-errors/not-settled · session errors · not-settled — An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable, highlighted; absent where the model holds nothing, invalid, clickable and highlighted written only when true.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
contract/stable-element-id/virtual-node-id · Stable element ids · virtual_node_id — A reserved fifth form {owner}///{key}, which no tier produces and only the in-process declared-node read gives a canvas's taken nodes, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
contract/target-check/canvas · Per-verb target check · canvas — `canvas` meets `fonts-unavailable` alone.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/target-check/screenshot · Per-verb target check · screenshot with an id — A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
contract/target-check/scroll · Per-verb target check · scroll — `scroll` meets no target check.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only.
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified.
decision/hand-written-answers · Driver answers written by hand — Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/reached-record-before-dispatch · Reached record resolved before dispatch, written after — The reached record resolves the chain's nodes whose current listeners hold the event before it runs, holding no borrow across the render, and writes them after: all while it propagates, the innermost alone when stopped or a handler's panic unwinds, a non-bubbling event's target alone. · why: So a handler whose own run removes its element still counts.
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
contract/accessibility-tree/active-descendant · Accessibility tree · active descendant — While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select and a page's aria-activedescendant is never read.
contract/accessibility-tree/name-sources · Accessibility tree · accessible name — A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
contract/accessibility-tree/node-names · blitz-dom accessibility tree · node names — Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
contract/accessibility-tree/platform-refresh · blitz-dom accessibility tree · platform tree refresh — blitz-shell refreshes the platform accessibility tree on a redraw, after resolve, when the changed set is non-empty, and then clears the set.
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent, highlighted written only when true; id, role, name, invalid, focused, clickable and bounds always written.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
contract/answer-json/string-escaping · answer JSON · string escaping — A string escapes ", \ and U+0000 to U+001F.
contract/base-document/has-changes · BaseDocument · has_changes — Reads true while the changed set is non-empty, from the first tracked write until the next drain.
contract/base-document/take-changed-nodes · BaseDocument · take_changed_nodes — Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
contract/check-replay/change-keys · Check replay · change keys — A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
contract/check-replay/first-difference · Check replay · first difference — Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/cold-agent-verdict/refused-by-cause · cold-agent verdict.json · refused_by_cause — The cold-agent verdict carries `refused_by_cause`, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift, --ctrl and --full are bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/added · command span · added — The length of the diff's added list.
contract/command-span/changed · command span · changed — The length of the diff's changed list.
contract/command-span/passes · command span · passes — Settle's pass count, recorded when settle answered quiet.
contract/command-span/removed · command span · removed — The length of the diff's removed list.
contract/command-span/settled · command span · settled — Recorded when the outcome carries one.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/virtual-node-refusal · Dioxus DOM bridge · VirtualNodeRefusal — Validated in declaration order, a declaration is refused with one closed reason: EmptyKey, KeyHoldsSlash, DuplicateKey (the first of a key kept), EmptyName or RegionOutside (not inside the content box with positive size, NaN and infinities failing); a refusal holds the declaration's index alone.
contract/document-core/changed-set · blitz-dom document core · changed set — The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
contract/driver-explore/expansion · escher-driver explore · path expansion — A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
contract/driver-public-api/no-serde-json-types · Driver public API · serde_json types — No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
contract/driver-verbs/graph · driver verbs · graph — One optional part (1 to 1000); result nodes and states, plus part and parts when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked; dispatches nothing, runs no step.
contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines, a canvas widget's labels read by their ink boxes; dispatches nothing.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session/canvas · escher-session command line · canvas --id [--full] [--part] — The verb canvas takes --id and the optional bare --full and --part.
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/event-dispatch/focus-change · Event dispatch · focus change — A focus change, a programmatic one included, dispatches blur, focusout, focus, focusin; only the focus fix-up under inert, the focus fix-up after a layout for a node that no longer shows, and the driver's select focus dispatch none.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}`, a part's adding `part` and `parts`, holds ids, component, state and event names, closed flag words, booleans and the part counts only, never a value.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/kept-check/accepted · Kept check file · accepted — Each record states whether the call was accepted.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/kept-check/masked · Kept check file · masked — `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
contract/lint-verb/changes-nothing · The lint verb · side effects — The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
contract/lint-verb/finding · The lint verb · finding — The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order, a canvas element's `truncated` read once per label in paint order.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`; a canvas element reads `truncated` once per label whose ink box does not lie inside its region.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/lint-verb/zero-size · The lint verb · zero-size — A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
contract/load-wait/no-time-moves · Driver acting step · wait side effects — No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `canvas` wait on nothing.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/scroll-hit/absolute-position · BaseDocument scroll and hit testing · Node::absolute_position — Node::absolute_position subtracts only ancestors' scroll offsets.
contract/scroll-hit/hit-clip · BaseDocument scroll and hit testing · hit on a clipping box — A hit stops at the padding box of a box paint clips, and a hit stopped there answers in that box's content-box frame, the frame of the final self hit and the text hits.
contract/scroll-hit/hit-degenerate-transform · BaseDocument scroll and hit testing · hit under a degenerate transform — A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
contract/scroll-hit/hit-dropped-node · BaseDocument scroll and hit testing · hit on a dropped node — A hit skips a node dropped since the last resolve.
contract/scroll-hit/hit-with-scrollbar · BaseDocument scroll and hit testing · hit_with_scrollbar — hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/session-wire/ok-refused · session wire · ok v3 refused — A call's refusal, its JSON line after the words.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/tool-call-span/ending · the tool_call span · ending — One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.

## occupied-resources · ## Occupied Resources
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider from the driver's own MCP server: element ids, accessible names, control values, snapshot texts, diffs, graph and actions answers, and acting answers' caused text, the order page's authored messages among it. · why: chosen as one run capped at $3, with a JSON check and a retry
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed, booleans and a part's part and parts counts; never a value, typed text or NodeId.
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
contract/mcp-role/self-description · the mcp role · self-description and remedy — A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
contract/session-socket-wire/answer-bound · the session socket's wire · answer bound — An answer is at most 1,048,576 bytes.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a redraw, after resolve, only when the document's changed set is non-empty, and then clears the set. · why: ratified as the shell half of the changed-set item
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked changes, typed text, range moves, value-changing select choices and a select list opening, closing or moving its highlight to a new row; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber.
decision/forwarded-event-coordinates · Forwarded events adjust coordinates — Sub-document and custom-widget events are forwarded with adjusted coordinates.
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
decision/resolve-before-js-reads · Style and layout resolved before JS reads — Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads.
decision/sub-document-event-forwarding · Events forwarded to sub-documents and widgets — Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space.
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled, and preventDefault on a non-cancelable event cancels nothing. · except: a cancelled pointerup still clicks and still ends a scrollbar drag
decision/events-queued-on-document · Events queued on the document — Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer; queued events for dropped nodes are discarded. · guard: event_queue
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled. · except: a cancelled pointerup still clicks and still ends a scrollbar drag
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.

## project-intent · ## Project Intent
decision/cold-agent-result-stated-measured · Cold-agent result stated as measured, with its reach — The capability statement states a cold-agent result as measured, with its reach: the measured instance names its apps, tools, calls, wrong calls and client, and every capability outside that run's reach is named with its headless stand witness. · why: Ratified.
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls. · except: The opt-in devtools feature's CDP server, a TCP listener on 127.0.0.1, which no default feature set turns on.

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
