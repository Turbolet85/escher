@row_add n=1 row=decision/unit-test-placement area=conventions rec=1
name: Unit-test placement
rule: Unit tests sit in a test module at the end of the source file, or in a child file of their own.
@row_add n=2 row=decision/integration-test-one-behavior area=conventions rec=1
name: One behavior per integration test file
rule: Each integration test file covers one behavior and opens with a doc comment naming it.
@row_add n=3 row=decision/scenarios-both-layout-modes area=conventions rec=1
name: Scenarios in both layout modes
rule: Scenarios run in both layout modes.
@row_add n=4 row=decision/stand-check-shared-helpers area=conventions rec=1
name: Stand-check helpers in non-target modules
rule: Stand checks share helpers through modules under the tests directory that are no test target.
@row_add n=5 row=decision/tests-assert-fixture-first area=conventions rec=1
name: Tests assert the fixture first
rule: A test first asserts that its fixture produces the condition under test.
@row_add n=6 row=decision/blitz-tests-one-target-per-file area=conventions rec=1
name: One test target per file in blitz-tests
rule: blitz-tests keeps one test target per file.
except: upstream's tests/all.rs, declared and held at test = false.
@row_add n=7 row=decision/moved-unit-tests-keep-paths area=conventions rec=2
name: Moved unit-test modules keep their test paths
rule: Unit-test modules moved to files of their own keep their test paths.
@row_add n=8 row=decision/shared-fixtures-stated-once area=conventions rec=2
name: Shared blitz-tests fixtures stated once
rule: The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
@row_add n=9 row=decision/single-reader-page-stays-local area=conventions rec=2
name: Single-reader fixture page stays local
rule: A fixture page with one reader stays in its check's file.
@row_add n=10 row=decision/windows-subsystem-outside-tests area=conventions rec=3
name: Windows subsystem for binaries
rule: Binaries set the Windows subsystem to "windows" outside tests.
@row_add n=11 row=decision/android-entry-point area=conventions rec=3
name: Android entry point
rule: Android entry points are android_main calling set_android_app.
@row_add n=12 row=decision/reactive-state-store-derive area=conventions rec=3
name: Reactive state derives Store
rule: Reactive state in apps and examples uses the Store derive.
@row_add n=13 row=decision/assets-through-asset-macro area=conventions rec=3
name: Assets through the asset macro
rule: Assets in apps and examples come through the asset! macro.
@row_add n=14 row=decision/dioxus-example-shape area=conventions rec=3
name: Dioxus example shape
rule: Dioxus examples launch through dioxus-native with component functions and signal and store hooks, their CSS in const raw strings.
@row_add n=15 row=decision/reduced-fixture-variants area=conventions rec=4
name: Reduced HTML fixture variants
rule: Reduced HTML fixture variants sit beside the full pages.
@row_add n=16 row=decision/feature-fixture-numbered-cases area=conventions rec=4
name: Numbered cases in feature fixtures
rule: Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
@row_add n=17 row=decision/fixture-relative-asset-urls area=conventions rec=4
name: Relative asset URLs in fixtures
rule: HTML fixtures reference sibling assets with ./-relative URLs.
@row_add n=18 row=decision/captures-kept-as-captured area=conventions rec=4
name: Captured fixtures kept as captured
rule: Captured HTML fixtures stand as captured: graphite.html minified, some without a closing head, body or html tag or a doctype.
@row_add n=19 row=contract/document-core/document-trait area=standard-contracts rec=5
boundary: blitz-dom document core
item: Document trait
promise: Every document wrapper implements the Document trait.
@row_add n=20 row=contract/document-core/documentconfig area=standard-contracts rec=5
boundary: blitz-dom document core
item: DocumentConfig
promise: DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
@row_add n=21 row=contract/document-core/changed-set area=standard-contracts rec=5
boundary: blitz-dom document core
item: changed set
promise: The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
@row_add n=22 row=contract/document-core/can-paint-text area=standard-contracts rec=5
boundary: blitz-dom document core
item: can_paint_text
promise: can_paint_text is read-only.
@row_add n=23 row=contract/document-core/layout-as-text-input area=standard-contracts rec=5
boundary: blitz-dom document core
item: layout_as_text_input
promise: layout_as_text_input is read-only.
@row_add n=24 row=contract/document-clock/documentconfig-clock area=standard-contracts rec=6
boundary: blitz-dom document clock
item: DocumentConfig.clock
promise: The document's clock is the config's clock, or else a fresh WallClock.
@row_add n=25 row=contract/document-clock/clock-reexports area=standard-contracts rec=6
boundary: blitz-dom document clock
item: Clock, WallClock and VirtualClock re-exports
promise: The blitz-dom crate root re-exports Clock, WallClock and VirtualClock.
@row_add n=26 row=contract/document-clock/virtualclock area=standard-contracts rec=6
boundary: blitz-dom document clock
item: VirtualClock
promise: Every clone of a VirtualClock reads one time.
@row_add n=27 row=decision/no-wall-time-outside-clock area=standard-contracts rec=6
name: No direct time reads in blitz-dom
rule: No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module.
@row_add n=28 row=contract/select-readers/select-value area=standard-contracts rec=6
boundary: BaseDocument select readers
item: select_value
promise: select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
@row_add n=29 row=contract/select-readers/select-values area=standard-contracts rec=6
boundary: BaseDocument select readers
item: select_values
promise: select_values is public and read-only and reads a select's chosen set.
@row_add n=30 row=contract/select-readers/select-of area=standard-contracts rec=6
boundary: BaseDocument select readers
item: select_of
promise: select_of is public and read-only and reads an option's or group's select.
@row_add n=31 row=contract/select-readers/clear-select-typeahead area=standard-contracts rec=6
boundary: BaseDocument select readers
item: clear_select_typeahead
promise: The public clear_select_typeahead clears a select's typed text; its one non-test caller is the driver's focus_select.
@row_add n=32 row=contract/select-readers/choice-writers area=standard-contracts rec=6
boundary: BaseDocument select readers
item: select choice writers
promise: The writers of a select's choice are crate-private.
@row_add n=33 row=contract/select-list/open-select area=standard-contracts rec=7
boundary: BaseDocument open select list
item: open_select
promise: open_select is public and read-only.
@row_add n=34 row=contract/select-list/select-list-bounds area=standard-contracts rec=7
boundary: BaseDocument open select list
item: select_list_bounds
promise: select_list_bounds is public and read-only.
@row_add n=35 row=contract/select-list/select-list-view area=standard-contracts rec=7
boundary: BaseDocument open select list
item: select_list_view
promise: select_list_view is public and read-only.
@row_add n=36 row=contract/select-list/select-row-layout area=standard-contracts rec=7
boundary: BaseDocument open select list
item: select_row_layout
promise: select_row_layout is public and read-only.
@row_add n=37 row=contract/select-list/close-select-list area=standard-contracts rec=7
boundary: BaseDocument open select list
item: close_select_list
promise: The public close_select_list closes an open list; its callers are keyboard, pointer, reconcile and the driver's focus_select.
@row_add n=38 row=contract/select-list/list-machinery area=standard-contracts rec=7
boundary: BaseDocument open select list
item: rest of the list machinery
promise: Every other part of an open select list's machinery is crate-private or private.
@row_add n=39 row=contract/mutation-query/mutate area=standard-contracts rec=8
boundary: blitz-dom mutation, query and CSSOM
item: doc.mutate()
promise: Mutations go through doc.mutate().
@row_add n=40 row=contract/mutation-query/selector-queries area=standard-contracts rec=8
boundary: blitz-dom mutation, query and CSSOM
item: selector queries
promise: Selector queries return a Result and never match the scope node.
@row_add n=41 row=contract/mutation-query/cssom area=standard-contracts rec=8
boundary: blitz-dom mutation, query and CSSOM
item: CSSOM addressing
promise: CSSOM is addressed by owner NodeId and an index path.
@row_add n=42 row=contract/accessibility-tree/build-accessibility-tree area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: build_accessibility_tree
promise: build_accessibility_tree returns an AccessKit TreeUpdate.
@row_add n=43 row=contract/accessibility-tree/node-names area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: node names
promise: Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
@row_add n=44 row=contract/accessibility-tree/state-attributes area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: state attributes
promise: The accessibility tree writes state attributes, each read once.
@row_add n=45 row=contract/accessibility-tree/checked-state area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: native checked state
promise: A native control's checked state is written on a checkable role.
@row_add n=46 row=contract/accessibility-tree/select-expanded area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: single select expanded
promise: A single select writes its native expanded, true while its list stands open, with no aria-expanded read on it; every other element keeps the aria-expanded read.
@row_add n=47 row=contract/accessibility-tree/platform-refresh area=standard-contracts rec=8
boundary: blitz-dom accessibility tree
item: platform tree refresh
promise: blitz-shell refreshes the platform accessibility tree when a poll took a non-empty changed set.
@row_add n=48 row=contract/accessibility-tree/non-checkable-role area=standard-contracts rec=9
boundary: blitz-dom accessibility tree
item: native control under a non-checkable role
promise: A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
@row_add n=49 row=contract/dom-events/pointer-events area=standard-contracts rec=10
boundary: DOM event dispatch
item: pointer-derived events
promise: Mouse compatibility events and touch events are dispatched from pointers.
@row_add n=50 row=contract/dom-events/non-bubbling area=standard-contracts rec=10
boundary: DOM event dispatch
item: non-bubbling events
promise: Non-bubbling events are dispatched to the target only.
@row_add n=51 row=contract/dom-events/focus-change area=standard-contracts rec=10
boundary: DOM event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_add n=52 row=contract/dom-events/inert-focus-fixup area=standard-contracts rec=10
boundary: DOM event dispatch
item: focus fixup under inert
promise: The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
@row_add n=53 row=contract/dom-events/driver-type-on-select area=standard-contracts rec=10
boundary: DOM event dispatch
item: driver type on a select
promise: The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
@row_add n=54 row=contract/dom-events/range-input-default area=standard-contracts rec=10
boundary: DOM event dispatch
item: range input default actions
promise: A range input's default actions dispatch input.
@row_add n=55 row=contract/dom-events/enter-space-click area=standard-contracts rec=10
boundary: DOM event dispatch
item: Enter and released Space
promise: Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
@row_add n=56 row=contract/dom-events/change-after-input area=standard-contracts rec=10
boundary: DOM event dispatch
item: change
promise: The engine dispatches change after input on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one change.
@row_add n=57 row=contract/dom-events/text-range-input-no-change area=standard-contracts rec=10
boundary: DOM event dispatch
item: text or range input's input
promise: A text or range input's input brings no change event.
@row_add n=58 row=contract/scroll-hit/scroll-into-view area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: scroll_into_view
promise: scroll_into_view scrolls every scrolling box on the target's chain, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
@row_add n=59 row=contract/scroll-hit/visible-region area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: visible_region
promise: visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
@row_add n=60 row=contract/scroll-hit/hit-clip area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: hit on a clipping box
promise: A hit stops at the padding box of a box paint clips.
@row_add n=61 row=contract/scroll-hit/hit-dropped-node area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: hit on a dropped node
promise: A hit skips a node dropped since the last resolve.
@row_add n=62 row=contract/scroll-hit/hit-degenerate-transform area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: hit under a degenerate transform
promise: A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
@row_add n=63 row=contract/scroll-hit/hit-with-scrollbar area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: hit_with_scrollbar
promise: hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
@row_add n=64 row=contract/scroll-hit/absolute-position area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: Node::absolute_position
promise: Node::absolute_position subtracts only ancestors' scroll offsets.
@row_add n=65 row=contract/scroll-hit/stale-id-indexing area=standard-contracts rec=11
boundary: BaseDocument scroll and hit testing
item: NodeTree indexing
promise: Indexing the NodeTree with a stale id panics.
@row_add n=66 row=contract/geometry/get-client-bounding-rect area=standard-contracts rec=12
boundary: blitz-dom geometry readers
item: get_client_bounding_rect
promise: get_client_bounding_rect reads an element where its transforms move it.
@row_add n=67 row=contract/geometry/visible-region-transforms area=standard-contracts rec=12
boundary: blitz-dom geometry readers
item: visible_region through transforms
promise: visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
@row_add n=68 row=contract/geometry/visible-region-non-finite area=standard-contracts rec=12
boundary: blitz-dom geometry readers
item: visible_region on a non-finite box
promise: visible_region answers None for a mapped box with a non-finite edge.
@row_add n=69 row=contract/geometry/visible-region-select-list area=standard-contracts rec=12
boundary: blitz-dom geometry readers
item: visible_region of an open select list's row
promise: A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
@row_add n=70 row=contract/geometry/text-extent area=standard-contracts rec=12
boundary: blitz-dom geometry readers
item: BaseDocument::text_extent
promise: BaseDocument::text_extent is a read-only reader that mutates nothing.
@row_add n=71 row=contract/layout/taffy-traits area=standard-contracts rec=13
boundary: blitz-dom layout
item: Taffy traversal and layout traits
promise: The crate-private LayoutPassState implements Taffy's traversal and layout traits for one pass; BaseDocument implements none.
@row_add n=72 row=contract/layout/stylo-taffy-exports area=standard-contracts rec=13
boundary: blitz-dom layout
item: stylo_taffy exports
promise: stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt.
@row_add n=73 row=contract/nodes-widgets/special-element-data area=standard-contracts rec=14
boundary: blitz-dom nodes and widgets
item: SpecialElementData
promise: SpecialElementData is not #[non_exhaustive], RangeInput among its variants.
@row_add n=74 row=contract/nodes-widgets/range-input-data area=standard-contracts rec=14
boundary: blitz-dom nodes and widgets
item: RangeInputData
promise: RangeInputData is read through the ElementData range accessors.
@row_add n=75 row=contract/nodes-widgets/widget-trait area=standard-contracts rec=14
boundary: blitz-dom nodes and widgets
item: Widget trait
promise: A widget's resources are unregistered when its node drops.
@row_add n=76 row=contract/form-controls/is-disabled area=standard-contracts rec=15
boundary: blitz-dom form control readers
item: ElementData::is_disabled
promise: ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
@row_add n=77 row=contract/form-controls/option-disabled area=standard-contracts rec=15
boundary: blitz-dom form control readers
item: option_disabled
promise: An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
@row_add n=78 row=contract/form-controls/select-rules area=standard-contracts rec=15
boundary: blitz-dom form control readers
item: node::select
promise: The select's rules live in the crate-private node::select, with no public type.
@row_add n=79 row=contract/form-controls/select-list-state area=standard-contracts rec=15
boundary: blitz-dom form control readers
item: node::select_list
promise: An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
@row_add n=80 row=contract/provider-traits/netprovider-noop area=standard-contracts rec=16
boundary: blitz-traits provider traits
item: no-op NetProvider
promise: The no-op NetProvider registers no pending-critical resources.
@row_add n=81 row=contract/provider-traits/shellprovider area=standard-contracts rec=16
boundary: blitz-traits provider traits
item: ShellProvider
promise: ShellProvider has no-op defaults.
@row_add n=82 row=contract/provider-traits/viewport-scale area=standard-contracts rec=16
boundary: blitz-traits provider traits
item: Viewport::scale
promise: A Viewport's scale() is hidpi_scale times zoom.
@row_add n=83 row=contract/html-net/document-html-parser area=standard-contracts rec=17
boundary: blitz-html and blitz-net crates
item: DocumentHtmlParser
promise: blitz-html's DocumentHtmlParser is an html5ever TreeSink.
@row_add n=84 row=contract/html-net/provider area=standard-contracts rec=17
boundary: blitz-html and blitz-net crates
item: blitz-net Provider
promise: blitz-net's Provider implements NetProvider::fetch, with typed ProviderErrors and byte, form or empty request bodies.
@row_add n=85 row=contract/paint/paint-scene area=standard-contracts rec=18
boundary: blitz-paint
item: paint_scene
promise: paint_scene paints a document into a PaintScene; a shell or renderer reaches painting through paint_scene alone.
@row_add n=86 row=contract/paint/select-list-order area=standard-contracts rec=18
boundary: blitz-paint
item: open select list painting
promise: An open select's list is painted last, after the root's render and before the devtools overlays, read from SelectListView and past any clip or opacity on its select's ancestors.
@row_add n=87 row=contract/paint/password-masked area=standard-contracts rec=18
boundary: blitz-paint
item: password input
promise: A password input paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
@row_add n=88 row=contract/test-harness/harness area=standard-contracts rec=19
boundary: blitz-test-harness
item: Harness
promise: Harness is generic over the document; its input helpers pump and never settle.
@row_add n=89 row=contract/test-harness/screenshot area=standard-contracts rec=19
boundary: blitz-test-harness
item: screenshot and screenshot_rect
promise: screenshot and screenshot_rect paint a raster through blitz-paint, printing, panicking and logging nothing.
@row_add n=90 row=contract/test-harness/text-contrast area=standard-contracts rec=19
boundary: blitz-test-harness
item: text_contrast
promise: text_contrast reads a WCAG contrast and changes nothing.
@row_add n=91 row=contract/test-harness/settle area=standard-contracts rec=19
boundary: blitz-test-harness
item: settle
promise: settle drives render and layout to quiet, reports a load in flight, and never waits on one or moves time.
@row_add n=92 row=contract/test-harness/virtual-clock area=standard-contracts rec=20
boundary: blitz-test-harness
item: harness VirtualClock
promise: The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
@row_add n=93 row=contract/headless-stand/boot area=standard-contracts rec=21
boundary: seven_guis headless stand
item: seven_guis::stand
promise: seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
@row_add n=94 row=contract/headless-stand/offline-boots area=standard-contracts rec=21
boundary: seven_guis headless stand
item: network
promise: Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
@row_add n=95 row=contract/headless-stand/timer-step area=standard-contracts rec=21
boundary: seven_guis headless stand
item: timer_step
promise: timer_step hands the Timer its ticks for a driver session.
@row_add n=96 row=contract/headless-stand/no-telemetry area=standard-contracts rec=21
boundary: seven_guis headless stand
item: telemetry and environment
promise: The stand installs no telemetry, reads no env var and names no blitz_net.
@row_add n=97 row=contract/telemetry/init area=standard-contracts rec=22
boundary: escher-telemetry bootstrap
item: init
promise: init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global.
@row_add n=98 row=contract/telemetry/line-format area=standard-contracts rec=22
boundary: escher-telemetry bootstrap
item: output line
promise: One line is written per event and per closed span, stamped service.name and service.version.
@row_add n=99 row=contract/telemetry/target-filter area=standard-contracts rec=22
boundary: escher-telemetry bootstrap
item: target filter
promise: A target outside the engine and escher prefix sets is dropped whole at every level.
@row_add n=100 row=contract/telemetry/engine-fields area=standard-contracts rec=22
boundary: escher-telemetry bootstrap
item: engine target fields
promise: An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
@row_add n=101 row=contract/telemetry/panic-hook area=standard-contracts rec=22
boundary: escher-telemetry bootstrap
item: panic hook
promise: The installed panic hook chains to the previous one.
@row_add n=102 row=decision/validated-commands-only area=standard-contracts rec=23
name: Driver commands run only after validation
rule: Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
@row_add n=103 row=contract/driver-session/command-schema area=standard-contracts rec=23
boundary: escher-driver session
item: command and refusal schema
promise: The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
@row_add n=104 row=contract/driver-session/schema-carriers area=standard-contracts rec=23
boundary: escher-driver session
item: schema carriers
promise: The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
@row_add n=105 row=contract/driver-session/refusal-content area=standard-contracts rec=23
boundary: escher-driver session
item: refusal
promise: No refusal holds what a call supplied.
@row_add n=106 row=contract/driver-session/no-nodeid area=standard-contracts rec=23
boundary: escher-driver session
item: NodeId
promise: No NodeId crosses the driver session boundary.
@row_add n=107 row=decision/driver-logs-no-content area=standard-contracts rec=23
name: Driver logs no content
rule: Nothing in escher-driver logs an id, a name or a value.
@row_add n=108 row=decision/hand-written-answers area=standard-contracts rec=24
name: Driver answers written by hand
rule: Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
@row_add n=109 row=decision/no-echo-of-typed-input area=standard-contracts rec=24
name: No echo of typed input
rule: No driver command's ending writes back anything the caller typed.
@row_add n=110 row=contract/driver-recording/kept-check area=standard-contracts rec=25
boundary: escher-driver recording
item: kept check
promise: The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels.
@row_add n=111 row=contract/driver-recording/replay area=standard-contracts rec=25
boundary: escher-driver recording
item: Check::replay
promise: Check::replay runs a kept check's calls again through Session::run.
@row_add n=112 row=contract/driver-recording/handler-coverage area=standard-contracts rec=25
boundary: escher-driver recording
item: handler_coverage
promise: handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
@row_add n=113 row=contract/driver-session/lint area=standard-contracts rec=25
boundary: escher-driver session
item: Session::lint
promise: Session::lint reads the screen and changes nothing.
@row_add n=114 row=contract/driver-session/load-wait area=standard-contracts rec=25
boundary: escher-driver session
item: LOAD_WAIT
promise: LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
@row_add n=115 row=decision/boa-script-engine area=standard-contracts rec=26
name: Boa runs document scripts
rule: A document's scripts run on Boa, JS listeners before Blitz default actions.
@row_add n=116 row=contract/script/document-may-load area=standard-contracts rec=26
boundary: blitz-vibey-script
item: document_may_load
promise: The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
@row_add n=117 row=contract/script/blitz-globals area=standard-contracts rec=26
boundary: blitz-vibey-script
item: __blitz_ globals
promise: __blitz_ globals carry messages and synchronous fetches.
@row_add n=118 row=contract/script/parse-search-params-deleted area=standard-contracts rec=26
boundary: blitz-vibey-script
item: __blitz_parse_search_params
promise: The bootstrap's own __blitz_parse_search_params is deleted from globalThis before any page script.
@row_add n=119 row=contract/script/bootstrap area=standard-contracts rec=26
boundary: blitz-vibey-script
item: JS bootstrap
promise: The JS bootstrap defines history, fetch, DOMException, the CSSOM classes and URLSearchParams with URL.prototype.searchParams, href and search live both ways.
@row_add n=120 row=decision/no-script-text-selection-readers area=standard-contracts rec=26
name: Script text readers unused
rule: escher uses none of the script text-selection and innerText readers.
@row_add n=121 row=contract/launch/dioxus-native-configs area=standard-contracts rec=27
boundary: high-level launch
item: dioxus-native launch configs
promise: dioxus-native's launch, launch_cfg and launch_cfg_with_props downcast configs and drop unmatched ones.
@row_add n=122 row=contract/dioxus-bridge/event-routing area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: data-dioxus-id
promise: DOM events reach the vdom through data-dioxus-id.
@row_add n=123 row=contract/dioxus-bridge/boolean-attributes area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: boolean attributes
promise: A falsy value of upstream's 28 boolean attributes clears the attribute.
@row_add n=124 row=contract/dioxus-bridge/stable-id area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: element_id and element_ids
promise: element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
@row_add n=125 row=contract/dioxus-bridge/virtual-node-id area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: virtual_node_id
promise: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
@row_add n=126 row=contract/dioxus-bridge/screen-readers area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: snapshot, text form, diff and UI graph
promise: Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
@row_add n=127 row=contract/dioxus-bridge/clickable area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: clickable
promise: The snapshot's clickable reads an element's own current click listener, on no disabled form control.
@row_add n=128 row=contract/dioxus-bridge/use-graph-state area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: use_graph_state
promise: An app registers graphed state through the ungated use_graph_state.
@row_add n=129 row=contract/dioxus-bridge/registry-dispatch area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: handle_event registry dispatch
promise: The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
@row_add n=130 row=contract/dioxus-bridge/reached-record area=standard-contracts rec=28
boundary: Dioxus DOM bridge
item: reached record
promise: The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
@row_add n=131 row=decision/no-snapshot-in-logs area=standard-contracts rec=29
name: No snapshot content in logs
rule: No log or event carries the snapshot, its text or a diff.
except: a diff's three counts, added, removed and changed, on escher-driver's command span.
@row_add n=132 row=decision/actionable-elements-keyed area=standard-contracts rec=30
name: Actionable elements carry author keys
rule: Every element an agent can act on reads an author key.
@row_add n=133 row=contract/actionable-readers/unkeyed-actionable area=standard-contracts rec=30
boundary: Dioxus actionable readers
item: DioxusDocument::unkeyed_actionable
promise: unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
@row_add n=134 row=contract/actionable-readers/actionable-nodes area=standard-contracts rec=30
boundary: Dioxus actionable readers
item: DioxusDocument::actionable_nodes
promise: actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
@row_add n=135 row=decision/one-actionable-rule area=standard-contracts rec=30
name: One actionable rule
rule: Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
@row_add n=136 row=contract/driver-forms/is-masked-input area=standard-contracts rec=31
boundary: driver form-control readers
item: is_masked_input
promise: is_masked_input is the one masked-control test: a password or file input.
@row_add n=137 row=contract/driver-forms/closed-select-of area=standard-contracts rec=31
boundary: driver form-control readers
item: closed_select_of
promise: The public closed_select_of answers None for a select whose list is open.
@row_add n=138 row=contract/driver-forms/is-select area=standard-contracts rec=31
boundary: driver form-control readers
item: is_select
promise: is_select is read by the driver's locate alone.
@row_add n=139 row=decision/select-tests-name-element area=standard-contracts rec=31
name: Select tests name the element
rule: The select tests the driver reads name the element and never the role.
@row_add n=140 row=contract/driver-forms/is-multiple-select area=standard-contracts rec=31
boundary: driver form-control readers
item: is_multiple_select
promise: is_multiple_select stays re-exported, read within the workspace by closed_select_of alone.
@row_add n=141 row=contract/driver-forms/select-form-data area=standard-contracts rec=31
boundary: driver form-control readers
item: select form data
promise: A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
@row_add n=142 row=contract/driver-forms/select-snapshot-options area=standard-contracts rec=31
boundary: driver form-control readers
item: select options in the snapshot
promise: A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
@row_add n=143 row=contract/accesskit-xplat/window-event area=standard-contracts rec=32
boundary: accesskit_xplat adapter
item: WindowEvent
promise: WindowEvent carries InitialTreeRequested, ActionRequested and AccessibilityDeactivated.
@row_add n=144 row=contract/accesskit-xplat/adapter area=standard-contracts rec=32
boundary: accesskit_xplat adapter
item: Adapter
promise: Adapter offers with_split_handlers, with_combined_handler, update_if_active, set_focus and set_window_bounds.
@row_add n=145 row=contract/accesskit-xplat/platform-adapter area=standard-contracts rec=32
boundary: accesskit_xplat adapter
item: platform adapter
promise: Each platform adapter exposes new, update_if_active, set_focus and set_window_bounds.
@row_add n=146 row=contract/escher-session-cli/usage area=standard-contracts rec=33
boundary: escher-session command line
item: usage
promise: escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
@row_add n=147 row=decision/session-apps-one-table area=standard-contracts rec=33
name: Session apps from one table
rule: The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
@row_add n=148 row=contract/escher-session-cli/serve-role area=standard-contracts rec=33
boundary: escher-session command line
item: serve
promise: escher-session has a serve host role.
@row_add n=149 row=contract/escher-session-cli/mcp-role area=standard-contracts rec=33
boundary: escher-session command line
item: mcp
promise: escher-session has an mcp role.
@row_add n=150 row=contract/escher-session-cli/coverage-role area=standard-contracts rec=33
boundary: escher-session command line
item: coverage
promise: escher-session has a coverage role.
@row_add n=151 row=contract/escher-session-cli/answer area=standard-contracts rec=33
boundary: escher-session command line
item: answer
promise: escher-session answers one line of JSON.
@row_add n=152 row=contract/escher-session-exit-codes/0 area=standard-contracts rec=33
boundary: escher-session exit codes
item: 0
promise: The call was accepted.
@row_add n=153 row=contract/escher-session-exit-codes/1 area=standard-contracts rec=33
boundary: escher-session exit codes
item: 1
promise: The call was refused.
@row_add n=154 row=contract/escher-session-exit-codes/2 area=standard-contracts rec=33
boundary: escher-session exit codes
item: 2
promise: A usage error.
@row_add n=155 row=contract/escher-session-exit-codes/3 area=standard-contracts rec=33
boundary: escher-session exit codes
item: 3
promise: A session error.
@row_none n=1 rec=34 type=5
@row_add n=156 row=contract/wpt-output/output-directory area=standard-contracts rec=35
boundary: WPT runner outputs
item: output directory
promise: An output directory is recreated each run, holding wpt_expectations.txt, wptreport.json and the reftest images.
@row_add n=157 row=contract/ci/wpt-diff-to-pr area=standard-contracts rec=36
boundary: CI contracts
item: wpt_diff_to_pr.py
promise: wpt_diff_to_pr.py writes WPT results into a PR body.
@row_add n=158 row=decision/upstream-only-workflows area=standard-contracts rec=36,48,52,53
name: Upstream-only workflows
rule: The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
@row_add n=159 row=decision/one-ci-leg-runner area=standard-contracts rec=36,53
name: One CI leg runner
rule: Every CI leg runs through one leg runner script, the same command the dev host runs.
@row_add n=160 row=decision/bounded-retried-package-install area=standard-contracts rec=36,53
name: Bounded, retried package install
rule: Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
@row_add n=161 row=decision/per-crate-coverage-floor area=standard-contracts rec=36
name: Per-crate coverage floor
rule: CI holds a coverage floor per crate.
guard: coverage_floor.py
@row_none n=2 rec=37 type=5
@row_add n=162 row=absence/served-network-port area=occupied-resources rec=38
capability: A served network port, TCP or UDP
@row_add n=163 row=decision/one-local-unix-listener area=occupied-resources rec=38
name: One local Unix-domain listener
rule: The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
@row_add n=164 row=decision/mcp-over-stdio area=occupied-resources rec=38
name: MCP role over stdio
rule: The escher-session mcp role speaks over its own stdio, with no listener.
@row_add n=165 row=decision/model-provider-operator-only area=occupied-resources rec=39
name: Model provider reached only on operator runs
rule: The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
@row_add n=166 row=name/other/dioxus-scheme area=occupied-resources rec=40
class: other
name: dioxus://index.html
@row_add n=167 row=name/other/blitz-user-agent area=occupied-resources rec=40
class: other
name: Mozilla/5.0 (compatible; Blitz)
@row_add n=168 row=name/other/com-dioxuslabs-blitz area=occupied-resources rec=40
class: other
name: com.dioxuslabs.blitz
@row_add n=169 row=name/other/blitz-target area=occupied-resources rec=40
class: other
name: blitz-target
@row_add n=170 row=decision/session-state-directory area=occupied-resources rec=41
name: Driver session state directory
rule: The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
@row_add n=171 row=decision/agent-state-areas-local area=occupied-resources rec=41
name: Agent state areas stay local
rule: The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
@row_add n=172 row=decision/recording-directory area=occupied-resources rec=42
name: Recording directory
rule: The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
@row_add n=173 row=decision/recording-removes-own-checks-only area=occupied-resources rec=42
name: Recording removes only its own checks
rule: escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory.
@row_add n=174 row=decision/kept-checks-owner-only area=occupied-resources rec=42
name: Kept checks on an owner-only medium
rule: A kept check lives on an owner-only medium.
except: the two committed checks under the blitz-tests checks directory.
@row_add n=175 row=decision/mimalloc-global-allocator area=occupied-resources rec=43
name: mimalloc global allocator
rule: mimalloc is the global allocator of the browser app, by default, and of the WPT runner.
@row_add n=176 row=decision/session-host-process-model area=occupied-resources rec=43
name: Session host process model
rule: The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
@row_add n=177 row=contract/telemetry/no-thread area=occupied-resources rec=43
boundary: escher-telemetry bootstrap
item: threads
promise: escher-telemetry's global subscriber, log bridge and chaining panic hook spawn no thread.
@row_none n=3 rec=44 type=5
@row_none n=4 rec=45 type=5
@row_add n=178 row=contract/telemetry/rust-log area=occupied-resources rec=46
boundary: escher-telemetry bootstrap
item: RUST_LOG
promise: RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
@row_add n=179 row=name/other/rust-log area=occupied-resources rec=46
class: other
name: RUST_LOG
@row_add n=180 row=name/other/wpt-dir area=occupied-resources rec=46
class: other
name: WPT_DIR
@row_add n=181 row=name/other/paint-tree-bench-html area=occupied-resources rec=46
class: other
name: PAINT_TREE_BENCH_HTML
@row_add n=182 row=name/other/multi-doc-n area=occupied-resources rec=46
class: other
name: MULTI_DOC_N
@row_add n=183 row=name/other/multi-doc-seq area=occupied-resources rec=46
class: other
name: MULTI_DOC_SEQ
@row_add n=184 row=name/other/multi-doc-flat area=occupied-resources rec=46
class: other
name: MULTI_DOC_FLAT
@row_add n=185 row=name/other/tmpdir area=occupied-resources rec=46
class: other
name: TMPDIR
@row_add n=186 row=name/crate/escher-telemetry area=occupied-resources rec=47
class: crate
name: escher-telemetry
@row_add n=187 row=name/crate/escher-driver area=occupied-resources rec=47
class: crate
name: escher-driver
@row_add n=188 row=decision/escher-crates-unpublished area=occupied-resources rec=47
name: escher crates unpublished
rule: escher-telemetry and escher-driver are publish = false libraries.
@row_add n=189 row=name/binary/blitz area=occupied-resources rec=47
class: binary
name: blitz
@row_add n=190 row=name/binary/seven-guis-native area=occupied-resources rec=47
class: binary
name: seven_guis_native
@row_add n=191 row=name/binary/escher-session area=occupied-resources rec=47
class: binary
name: escher-session
@row_add n=192 row=name/binary/todomvc-native area=occupied-resources rec=47
class: binary
name: todomvc_native
@row_add n=193 row=name/other/cold-agent-sh area=occupied-resources rec=47
class: other
name: cold-agent.sh
@row_add n=194 row=name/other/cold-agent-ps1 area=occupied-resources rec=47
class: other
name: cold-agent.ps1
@row_add n=195 row=name/server/escher area=occupied-resources rec=47
class: server
name: escher
@row_add n=196 row=name/other/mcp-escher area=occupied-resources rec=47
class: other
name: mcp__escher
@row_add n=197 row=decision/test-cache-restore-only area=occupied-resources rec=48,53
name: Shared test cache restored, never saved
rule: The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it.
@row_add n=198 row=decision/workspace-shared-dependencies area=infrastructure-patterns rec=49
name: Shared workspace dependencies
rule: The Cargo workspace declares its dependencies once in a shared workspace dependency table.
@row_add n=199 row=decision/fmt-clippy-gates area=infrastructure-patterns rec=49
name: fmt and clippy gates
rule: CI gates every change on rustfmt and clippy.
@row_add n=200 row=decision/rustdoc-warnings-gate area=infrastructure-patterns rec=49,53
name: Rustdoc warnings gate
rule: Rustdoc warnings are errors, enforced by the docs job over every workspace library crate.
@row_add n=201 row=decision/cargo-deny-advisory-gate area=infrastructure-patterns rec=49
name: cargo-deny advisory gate
rule: A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
@row_add n=202 row=decision/nix-flake area=infrastructure-patterns rec=49
name: Nix flake
rule: A Nix flake exposes the browser package as default and a blitz-dev devShell, wrapping Linux packages so the dlopen'd graphics and windowing libraries are found.
@row_add n=203 row=decision/objc2-encoding-assertions-off area=infrastructure-patterns rec=49
name: objc2 encoding assertions disabled
rule: blitz-dom depends on objc2 with disable-encoding-assertions on Apple targets.
why: a hack to stop debug builds panicking
@row_none n=5 rec=50 type=S
@row_add n=204 row=decision/nextest-mutants-profile area=infrastructure-patterns rec=51
name: nextest mutants profile
rule: cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched.
@row_add n=205 row=decision/capabilities-doc-beside-upstream area=infrastructure-patterns rec=51
name: Capability statement beside upstream docs
rule: escher's capability statement sits in a file of its own under docs, beside upstream's three files.
why: an upstream change under docs never conflicts with it
@row_add n=206 row=decision/sites-outside-workspace area=infrastructure-patterns rec=51
name: sites outside the workspace
rule: The sites directory is excluded from the Cargo workspace.
@row_add n=207 row=decision/bundle-build-jobs area=infrastructure-patterns rec=52
name: Two build jobs for browser bundles
rule: Browser bundle builds run with two cargo build jobs.
why: to debug possible OOM
@row_add n=208 row=decision/stand-uses-bundled-font area=infrastructure-patterns rec=52
name: Stand shares the bundled font
rule: seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
@row_add n=209 row=decision/ci-fast-slow-split area=infrastructure-patterns rec=53
name: CI fast/slow split
rule: fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
@row_add n=210 row=decision/ci-read-only-permissions area=infrastructure-patterns rec=53
name: Read-only CI permissions
rule: CI runs under a workflow-level contents: read permission with no job-level grant.
@row_add n=211 row=decision/ci-concurrency area=infrastructure-patterns rec=53
name: Per-ref CI concurrency
rule: CI runs with per-ref concurrency and cancels a run in progress.
@row_add n=212 row=decision/locked-cargo-legs area=infrastructure-patterns rec=53
name: Locked cargo legs
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello.
@row_add n=213 row=decision/unrewritten-profiles area=infrastructure-patterns rec=53
name: Build profiles unrewritten
rule: CI jobs build the profiles unrewritten, with no opt-level rewrite step.
@row_add n=214 row=decision/linux-tested-by-fast-leg area=infrastructure-patterns rec=53
name: Linux tested by the fast leg
rule: Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
@row_add n=215 row=decision/ci-cache-save-refs area=infrastructure-patterns rec=53
name: CI cache save refs
rule: ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
@row_add n=216 row=decision/pinned-actions area=infrastructure-patterns rec=53
name: SHA-pinned actions
rule: Every ci.yml uses: is pinned to a 40-hex commit SHA and every rust-toolchain step names its toolchain.
guard: test_ci_workflows.py
@row_add n=217 row=decision/ci-failure-logs area=infrastructure-patterns rec=53
name: CI logs uploaded on failure
rule: Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days.
@row_none n=6 rec=54 type=4
relied: A failed, empty or non-CSS stylesheet stops blocking render, because a failed fetch's error answer takes the request out of the pending critical resources.
@row_none n=7 rec=55 type=4
relied: Every shell window event is followed by a Poll.
