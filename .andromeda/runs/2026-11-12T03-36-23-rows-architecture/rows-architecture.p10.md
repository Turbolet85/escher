@row_add n=1 row=decision/scroll-into-view-whole-chain area=standard-contracts rec=1
name: Scroll into view moves the whole containing-block chain
rule: Scrolling an element into view scrolls every scrolling box on its containing-block chain, innermost first, then the viewport, through the one existing method for every caller; a row of an open select list moves only the list's own scroll offset.
why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method added.
@row_add n=2 row=decision/nested-scroll-written-instantly area=standard-contracts rec=1
name: Nested scrolling boxes are written instantly
rule: When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes.
why: The document holds one scroll animation at a time.
@row_add n=3 row=decision/visibility-from-geometry-not-hit area=standard-contracts rec=1
name: Visibility is read from geometry, never from a hit
rule: Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
@row_add n=4 row=decision/hit-stops-at-paint-clip area=standard-contracts rec=1
name: Hit walk stops where paint clips
rule: A node that paint clips (overflow not visible on either axis, or contain: paint, the root excepted) stops the hit walk at its padding box: outside it none of its children or inline content is reached, while the node itself is still answered inside its border box.
why: Uses paint's own clipping predicate so hit and paint agree; ratified.
guard: hit_clipped_at_scrolling_box
@row_add n=5 row=decision/one-axis-clip-readings-kept area=standard-contracts rec=1
name: One-axis clip readings kept as measured
rule: A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed.
why: Ratified to measure and record the difference rather than move either reader.
guard: stand_act_obstructed
@row_add n=6 row=decision/absolute-position-ancestor-scroll-only area=standard-contracts rec=1
name: Absolute position subtracts only ancestors' scroll
rule: The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands.
why: Ratified.
guard: scrolled_box_absolute_position
@row_add n=7 row=decision/client-rect-under-transforms area=standard-contracts rec=2,1
name: Client rect read through transforms
rule: The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one.
why: Matches CSSOM View getBoundingClientRect; ratified.
guard: transformed_client_rect
@row_none n=1 rec=3 type=4
relied: The driver's in-view check and crop read the visible region through transforms, a rotation reading the rotated box's bounding box.
@row_add n=8 row=contract/basedocument-readers/text-extent area=standard-contracts rec=4
boundary: BaseDocument readers
item: text_extent
promise: Returns the content box and the laid-out text extent of an inline root, or None for an unresolved id, a node that is no inline root or one with no inline layout; mutates nothing and never logs, prints or panics; RTL and vertical text read as LTR lines, a transformed box at its client rect.
@row_none n=2 rec=5 type=4
relied: The hit, the driver's covered check, hover and pointerdown pass over child nodes dropped since the last resolve, and an open select list answers a hit first.
@row_none n=3 rec=6 type=4
relied: A subtree under a non-invertible transform answers no hit.
@row_none n=4 rec=7 type=4
@row_none n=5 rec=8 type=5
@row_none n=6 rec=9 type=4
@row_none n=7 rec=10 type=5
@row_add n=9 row=decision/one-disabled-reading area=standard-contracts rec=11
name: One shared disabled reading
rule: Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
@row_none n=8 rec=12 type=5
@row_none n=9 rec=13 type=5
@row_add n=10 row=decision/select-choice-single-write-path area=standard-contracts rec=14
name: Select choice has one write path
rule: Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
@row_add n=11 row=contract/blitz-dom-crate-root/select-list-view-types area=standard-contracts rec=15
boundary: blitz-dom crate root
item: SelectListView and SelectListRow
promise: The open select list's view and rows are public read types re-exported from the crate root.
@row_add n=12 row=decision/select-list-colours-engine-constants area=standard-contracts rec=15
name: Select list colours are engine constants
rule: The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens.
@row_add n=13 row=decision/fresh-list-survives-focus-restyle area=standard-contracts rec=16
name: A just-opened list survives the focus restyle
rule: A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened.
why: A click focuses and opens in one step, before the focus rule's restyle.
guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
@row_none n=10 rec=17 type=4
relied: An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
@row_none n=11 rec=18 type=4
relied: The layout pass reads a node's out-of-flow state from the flag recorded by the style traversal.
@row_none n=12 rec=19 type=4
@row_none n=13 rec=20 type=4
relied: Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
@row_none n=14 rec=21 type=4
relied: The :open pseudo-class matches a details or dialog carrying an open attribute in every document.
@row_add n=14 row=contract/net-provider/is-noop area=standard-contracts rec=22
boundary: NetProvider
item: is_noop
promise: Defaults to false; when true, callers register no pending-critical resources.
@row_add n=15 row=contract/net-handler/error area=standard-contracts rec=22
boundary: NetHandler
item: error
promise: Receives the URL and error text of a failed load; its default does nothing.
@row_add n=16 row=contract/shell-provider/default-methods area=standard-contracts rec=22
boundary: ShellProvider
item: default methods
promise: Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list.
@row_add n=17 row=contract/viewport/scale area=standard-contracts rec=22
boundary: Viewport
item: scale
promise: The scale is the hidpi scale multiplied by the zoom.
@row_add n=18 row=contract/blitz-net-provider/fetch area=standard-contracts rec=23
boundary: blitz-net Provider
item: fetch
promise: Calls the handler's bytes with the final URL on a success or the handler's error with the URL and error text on a failure, then wakes the waker for the document.
@row_add n=19 row=decision/password-input-painted-masked area=standard-contracts rec=24
name: Password inputs paint masked
rule: A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets.
why: Ratified.
@row_add n=20 row=decision/password-caret-placed-by-bullets area=standard-contracts rec=24
name: Password caret placed by bullets
rule: A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value.
why: Ratified.
guard: password_caret_click
@row_none n=15 rec=25 type=4
@row_none n=16 rec=26 type=4
@row_none n=17 rec=27 type=4
relied: An open select list paints last, past the blend layers.
@row_none n=18 rec=28 type=9
@row_none n=19 rec=29 type=4
relied: The windowed shell's redraw requests travel through its event loop.
@row_add n=21 row=decision/harness-reexports-key-types area=standard-contracts rec=30
name: Harness re-exports keyboard types
rule: The test harness re-exports the keyboard types a key press is named with.
why: A crate depending on the harness names a key without a keyboard dependency of its own.
@row_add n=22 row=contract/test-harness/screenshot area=standard-contracts rec=30
boundary: Test harness
item: screenshot
promise: Resolves at the harness's own time without tick or pump, paints the viewport at its scale and device size through blitz-paint on the CPU renderer, composites over opaque white, and never prints, panics or logs.
@row_add n=23 row=contract/screenshot-error/empty-rect area=standard-contracts rec=30
boundary: ScreenshotError
item: EmptyRect
promise: Returned when the requested rectangle, clipped to the viewport, is empty.
@row_add n=24 row=contract/screenshot-error/fonts-unavailable area=standard-contracts rec=30
boundary: ScreenshotError
item: FontsUnavailable
promise: Returned with nothing painted when the document cannot paint text.
@row_add n=25 row=contract/test-harness/contrast-ratio area=standard-contracts rec=30
boundary: Test harness
item: contrast_ratio
promise: Returns the WCAG 2.x ratio (L1 + 0.05) / (L2 + 0.05), lighter over darker, from relative luminance with alpha ignored.
@row_add n=26 row=contract/test-harness/text-contrast area=standard-contracts rec=30
boundary: Test harness
item: text_contrast
promise: Reads text colour, size and weight and the composited background without changing anything; None when no text, missing styles, or an element on the walk with opacity below 1, a filter, a background image or a blend mode; large text needs 3.0, else 4.5.
@row_add n=27 row=contract/test-harness/settle area=standard-contracts rec=30
boundary: Test harness
item: settle
promise: Runs passes until one is quiet, then returns NotSettled with Loads if a request or render-blocking resource is pending, else Settled with its pass count and whether animating; after 64 passes with none quiet returns NotSettled with the last pass's class.
@row_add n=28 row=contract/test-harness/busy area=standard-contracts rec=30
boundary: Test harness
item: Busy
promise: Names only a class, Render, Layout or Loads, with no node, element, name, URL or value; NotSettled displays one fixed message per class.
@row_add n=29 row=decision/settle-never-waits-on-time-or-loads area=standard-contracts rec=30
name: Settle never waits on loads or time
rule: Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
@row_add n=30 row=decision/input-helpers-do-not-settle area=standard-contracts rec=30
name: Input helpers do not settle
rule: Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
@row_add n=31 row=decision/harness-counts-loads-at-net-provider area=standard-contracts rec=30
name: Harness counts loads at the net provider
rule: A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone.
why: Loads are counted where every request crosses.
@row_add n=32 row=decision/one-virtual-clock area=standard-contracts rec=31
name: One virtual clock per harness
rule: The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock.
why: Ratified.
@row_add n=33 row=decision/stand-pinned-environment area=standard-contracts rec=32
name: Stand pins its environment
rule: Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
@row_add n=34 row=decision/stand-offline area=standard-contracts rec=32
name: Stand boots offline
rule: Every stand boot is offline, with no net provider and no base URL.
guard: the_stand_options_stay_offline
except: The load page boot replaces the net provider with its own network stand-in.
@row_add n=35 row=decision/session-moves-clock-not-time-step area=standard-contracts rec=32
name: Session, not the time step, moves the clock
rule: A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock.
why: Ratified.
@row_add n=36 row=decision/stand-typography-dejavu-sans area=standard-contracts rec=33
name: Stand typography is DejaVu Sans
rule: The stand's one bundled font, DejaVu Sans, is registered for every generic family with system fonts off in every stand boot; the typography is stated in the stand contract, not on the design-system product surface.
@row_add n=37 row=decision/cells-not-a-lean-task area=standard-contracts rec=34
name: Cells is not a lean task
rule: The Cells task boots through its own stand entry and is kept out of the lean task set.
why: Its screen passes the snapshot text's budget and reads in parts.
@row_none n=20 rec=35 type=4
relied: The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
@row_none n=21 rec=36 type=4
relied: The load page's network stand-in answers its one stylesheet after 200 ms and is counted as a load by the harness.
@row_add n=38 row=decision/graph-signal-hook-in-seven-guis area=standard-contracts rec=37
name: Graph signal hook lives in seven_guis
rule: The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge.
why: dioxus-native-dom names no crate that defines Signal.
@row_add n=39 row=contract/telemetry-init/init area=standard-contracts rec=38
boundary: Telemetry init
item: init
promise: Installs the stderr sink and returns Installed, or AlreadyInstalled with nothing changed on a second call.
@row_add n=40 row=contract/telemetry-init/foreign-subscriber area=standard-contracts rec=38
boundary: Telemetry init
item: ForeignSubscriber
promise: Returned as an error, never a panic, when another subscriber or log logger is already global.
@row_add n=41 row=contract/telemetry-init/service-identity area=standard-contracts rec=38
boundary: Telemetry init
item: ServiceIdentity
promise: The service name and version are taken from the calling crate's package name and version.
@row_add n=42 row=contract/telemetry-line/event-line area=standard-contracts rec=38
boundary: Telemetry line
item: event line
promise: One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
@row_add n=43 row=contract/telemetry-line/span-close-line area=standard-contracts rec=38
boundary: Telemetry line
item: span close line
promise: One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
@row_add n=44 row=contract/telemetry-scrub/engine-targets area=standard-contracts rec=38
boundary: Telemetry scrub
item: engine targets
promise: An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
@row_add n=45 row=contract/telemetry-scrub/escher-targets area=standard-contracts rec=38
boundary: Telemetry scrub
item: escher targets
promise: An event under the escher_ prefix prints every field except content fields, which are redacted.
@row_add n=46 row=contract/telemetry-scrub/other-targets area=standard-contracts rec=38
boundary: Telemetry scrub
item: other targets
promise: An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
@row_add n=47 row=contract/telemetry-scrub/content-span-fields area=standard-contracts rec=38
boundary: Telemetry scrub
item: content span fields
promise: A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
@row_add n=48 row=contract/telemetry-panic/panic-event area=standard-contracts rec=38
boundary: Telemetry panic event
item: panic event
promise: A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook.
