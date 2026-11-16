@row_set n=1 row=decision/driver-spans-only cell=rule seen=1 detector=none
value: The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no subscriber, reads no env var and has no default state location.
basis: fct-x3q3dkxj
because: Session::layout is a driver read that runs no step and opens no span, so the row's one-span-per-call rule must name the run path it holds for.
@row_add n=1 row=contract/base-document/layout-reason area=standard-contracts fact=F1 detector=D-arch-resources
boundary: BaseDocument
item: layout_reason
promise: Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor); no log, no panic.
basis: fct-x3q3dkxj
because: blitz-dom gains a new public reader on every document, a contract at the engine's boundary that no row registers.
@row_add n=2 row=contract/layout-reason/placed-by area=standard-contracts fact=F2 detector=D-arch-resources
boundary: blitz-dom layout reason
item: PlacedBy
promise: The placing word is closed: Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList.
basis: fct-x3q3dkxj
because: The layout reason's placing words are a new closed set at the engine's public boundary.
@row_add n=3 row=contract/layout-reason/shift-by area=standard-contracts fact=F2 detector=D-arch-resources
boundary: blitz-dom layout reason
item: ShiftBy
promise: The shift word is closed: Scroll or Transform.
basis: fct-x3q3dkxj
because: The layout reason's shift words are a new closed set at the engine's public boundary.
@row_add n=4 row=contract/layout-reason/size-cause area=standard-contracts fact=F2 detector=D-arch-resources
boundary: blitz-dom layout reason
item: SizeCause
promise: The cause of a size is closed: Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout.
basis: fct-x3q3dkxj
because: The layout reason's size causes are a new closed set at the engine's public boundary.
@row_add n=5 row=contract/layout-reason/length area=standard-contracts fact=F2 detector=D-arch-resources
boundary: blitz-dom layout reason
item: Length
promise: A stated length is closed: Auto, None, Px, Percent (0 to 100), Calc in px, or a SizeKeyword of MinContent, MaxContent, FitContent or Stretch.
basis: fct-x3q3dkxj
because: The layout reason's length forms are a new closed set at the engine's public boundary.
@row_add n=6 row=contract/layout-reason/size-cause-rule area=standard-contracts fact=F3 detector=none
boundary: blitz-dom layout reason
item: size cause rule
promise: The cause of a size is a stated rule, first match wins, read at 1/64 px; a height percentage counts as definite only when its container's computed height is not auto.
basis: fct-e4q2j7s6
because: The reader's cause of a size is a stated rule other parts rely on, and the definite-percentage reading is a value form the chunk fixed.
@row_add n=7 row=contract/blitz-dom-crate-root/layout-reason-types area=standard-contracts detector=D-arch-resources
boundary: blitz-dom crate root
item: LayoutReason, Placed, Shift, SizeReason, FlexInputs and their enums
promise: The layout reason's structs and its non-exhaustive enums PlacedBy, ShiftBy, SizeCause, Length and SizeKeyword are public read types re-exported from the crate root.
basis: fct-x3q3dkxj
because: blitz-dom's root gains new public re-exports that no row of its crate root registers.
@row_add n=8 row=contract/dioxus-native-dom-api/layout-reason-types area=standard-contracts detector=D-arch-resources
boundary: dioxus-native-dom public API
item: layout reason types and BoundingRect
promise: Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect.
basis: fct-x3q3dkxj
because: dioxus-native-dom's public API gains new re-exports, a public surface no row registers.
@row_add n=9 row=contract/driver-session/layout area=standard-contracts fact=F4 detector=D-arch-resources
boundary: escher-driver session
item: Session::layout
promise: An in-process read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id as the run path does, refusing stale or not-found, records the screen's ids, runs no step, pump, tick or settle and opens no span.
basis: fct-x3q3dkxj
because: The driver gains a new public session read, a contract at the driver's boundary that no row registers.
@row_add n=10 row=decision/layout-read-in-process-only area=standard-contracts fact=F5 detector=none
name: Layout read stays in process
rule: No verb, MCP tool, session socket call or kept check reaches the driver's layout read; it is answered in process only.
state: provisional
basis: fct-u7f3beyp
because: The absence of every carrier of the layout read is applied provisional on the delegate's answer, pending the founder's own word at the epoch boundary.
@row_add n=11 row=contract/driver-layout/placer-app area=standard-contracts fact=F6 detector=none
boundary: escher-driver layout answer
item: Placer::App
promise: A reserved placing word carrying a canvas name, built by no engine path; an app-placed canvas element is to take it.
basis: fct-x3q3dkxj
because: The driver's layout answer reserves a closed placing word for an app-placed canvas that future work must use.
@row_add n=12 row=contract/driver-layout/unexplained-element area=standard-contracts fact=F7 detector=none
boundary: escher-driver layout answer
item: unexplained element
promise: An element the engine reader cannot explain reads Placer::Engine(Block) with its snapshot bounds, no container and both causes Layout, in process only; before the read crosses any carrier it takes a closed word of its own.
basis: fct-woiwjiy7
because: The driver maps an unexplained element to the Block word in process, and that reading is ruled unfit to cross a carrier as it stands.
@row_add n=13 row=decision/closed-select-option-layout-reason area=established-decisions fact=F8 detector=none
name: Closed select option reads its select's box
rule: A closed select's option reads the placing word Select at its select's box although the default sheet gives it display none; a closed select's optgroup, display contents, reads no layout reason.
basis: fct-gy4gjjmp
because: The layout reason departs from its has-a-box rule for a closed select's options, a reading future work on selects must respect.
@row_add n=14 row=decision/sixty-fourth-px-tolerance-inclusive area=conventions fact=F9 detector=none
name: A 1/64 px tolerance is inclusive
rule: A tolerance stated to 1/64 px is read inclusively, the absolute difference at most 1/64.
why: A sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
basis: fct-dawnxnas
because: The chunk fixed how a stated 1/64 px tolerance is read, a convention every geometry check follows.
