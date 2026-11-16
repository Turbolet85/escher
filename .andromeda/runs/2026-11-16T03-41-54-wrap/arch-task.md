@task_prose part=session
The session added a read-only layout explanation: why an element sits where it does at its size, read from the style and layout chain.
blitz-dom gained BaseDocument::layout_reason, a public reader on every document answering LayoutReason (placement word, container, offset, scroll and transform shifts, a size cause per axis), or None for an unresolved id, a non-element, an unstyled element or an element with no box.
escher-driver gained Session::layout, in process only, answering LayoutExplanation with String ids, closed words and numbers and no NodeId; it resolves its id through the run path and refuses stale or not-found.
No verb, MCP tool, socket call, kept check, argument, result field, refusal cause, node key, log field or span field was added; the absence of every carrier of the read is PROVISIONAL on the delegate's answer and v030-11 stays unclaimed.
Measured: the read changes nothing (snapshot, focus, diff, clock, pending ticks), writes no escher line at TRACE, and its offset chain holds to 1/64 px read inclusively; escher-driver line coverage moved to 97.99 % and every coverage floor holds.
Nothing was retired.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: new blitz-dom module holding the layout reason reader on BaseDocument and its public closed types (LayoutReason, Placed, Shift, SizeReason, FlexInputs, PlacedBy, ShiftBy, SizeCause, Length, SizeKeyword)
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: the blitz-dom crate root, its module list and public re-exports
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root and its public API re-exports of blitz-dom types beside NodeId
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: new escher-driver module holding Session::layout and its answer types (LayoutExplanation, Placement, Placer, ShiftReading)
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: the driver layout module's unit tests, a path child of that module
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver's run path, its id resolution (resolve, refusing stale or not-found) and the per-verb target
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root, its module list and public re-exports of the session API
@task_file n=8 path=docs/capabilities.md
what: the capability statement, what escher can and cannot do, pinned to the driver schema
@task_file n=9 path=tests/blitz-tests/tests/layout_reason.rs
what: new blitz-tests integration target proving every placing word and size cause of the engine's layout reason on fixtures, in both layout modes
@task_file n=10 path=tests/blitz-tests/tests/stand_layout.rs
what: new blitz-tests stand check of the driver's layout read on the seven_guis stand, in both layout modes, run by stand-repeat
@task_item n=1 part=touches
NodeId at every public API, and no NodeId crossing a session: the engine reader takes a NodeId while the driver read answers String ids only; both hold, and the driver session's NodeId contract item gains a new reader under it.
@task_item n=2 part=touches
The session boundary and its carriers: new carriers widen the boundary and the validated verb set widens only by ratification; the layout read reaches no carrier, VERBS still holds fourteen, and its absence on the carriers is a new PROVISIONAL absence beside the exploration-on-carriers and fixture-state-on-carriers absences.
@task_item n=3 part=touches
Driver spans and events: the decision that the driver opens spans and emits no event gains a read that opens no span at all; the command span is unmoved.
@task_item n=4 part=touches
Driver library without side channels and the driver's changes-nothing readers (the actions and lint verbs' side-effects items): the layout read runs no step, pump, tick or settle and changes nothing, a new in-process reader of the same kind.
@task_item n=5 part=touches
Refusal causes stale and not-found and the driver's id resolution: Session::layout resolves through the run path's resolve, which is now crate-visible beyond the run path; no cause was added and refusal causes stay closed at eight.
@task_item n=6 part=touches
The capability statement and its sections: it gained the layout read's size rule and the absence of a carrier for the layout read; the capability statement's pin to the driver schema stays green.
@task_item n=7 part=touches
CSSOM geometry snapped to 1/64 px: the layout read snaps its offset and rects to 1/64 px, and a stated 1/64 px tolerance is read inclusively.
@task_item n=8 part=touches
blitz-dom layout contract items (stylo_taffy exports, Taffy traits) and the decision on lazily boxed Taffy output: the reader resolves calc() through stylo_taffy's resolver and reads unrounded geometry, never the final layout.
@task_item n=9 part=touches
blitz-dom geometry readers (BoundingRect, get_client_bounding_rect, scrolled and transformed element bounds, client rect ignoring its own scroll): the layout reason holds BoundingRect fields and reads scroll and transform as shifts held by an ancestor; dioxus-native-dom now re-exports BoundingRect.
@task_item n=10 part=touches
Closed select options and the snapshot's closed select option: a closed select's option, though display none by the default sheet, reads Select at its select's box, while its optgroup reads None.
@task_item n=11 part=touches
Tables laid out as Taffy grid, display table as grid, anonymous blocks only in layout children, and text nodes never laid out: the reader names a Table placing word, reads an element held by an anonymous block with the element as container, and answers None for a text node.
@task_item n=12 part=touches
Sizing keywords on min and max as auto, and static boxes reporting auto insets: the read's Length words cover Auto, None, Px, Percent, Calc and the keywords, with an anchor size reading Auto (None for a max) and flex-basis content reading Auto.
@task_item n=13 part=touches
Non-exhaustive public types convention: PlacedBy, ShiftBy, SizeCause, Length, SizeKeyword and the driver's Placer are new non_exhaustive enums.
@task_item n=14 part=touches
Restricted crate internals: the driver's resolve became pub(crate) and Target gained a crate-visible node accessor; no signature outside the crate changed.
@task_item n=15 part=touches
Unit tests as path children of their module (driver unit tests in path children, split test modules stay children): the driver layout module's tests are a path child.
@task_item n=16 part=touches
One test target per file in blitz-tests and the blitz-tests manifest's one test table: two new targets, layout_reason and stand_layout.
@task_item n=17 part=touches
Scenarios in both layout modes and both layout modes giving identical layouts: both new targets run every case in incremental and full layout.
@task_item n=18 part=touches
Stand-repeat and its zero-failure budget, and stand-check helpers in non-target modules: stand_layout joins stand-repeat by its stand_ prefix and declares the shared session test module.
@task_item n=19 part=touches
Per-crate coverage floors: escher-driver line coverage fell from 98.05 % to 97.99 % and still meets its floor; a row holding escher-driver's floor value is to be checked against it.
@task_item n=20 part=touches
Direct engine changes flagged upstreamable: blitz-dom gains a new public reader and types, an engine change with no behaviour change to any document.
@task_item n=21 part=touches
Cold-agent bar and the cold-agent verdict: the cold-agent run cannot count the layout read while it stays in process.
@task_item n=22 part=touches
Cognitive complexity ceiling of 15: two new functions read 17 and were split to meet it.
@task_fact n=1 slug=standard-contracts
fact: BaseDocument::layout_reason(NodeId) answers a LayoutReason (placement word, container, offset, shifts, a size reason per axis with its cause) or None for an unresolved id, a non-element, an element with no computed styles, or an element with no box (display none, display contents, or under a display none ancestor); it has no log site and no panic path.
@task_fact n=2 slug=standard-contracts
fact: The layout reason's closed words: PlacedBy is Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select or SelectList; ShiftBy is Scroll or Transform; SizeCause is Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content or Layout; Length is Auto, None, Px, Percent (0 to 100), Calc in px or a SizeKeyword (MinContent, MaxContent, FitContent, Stretch).
@task_fact n=3 slug=standard-contracts
fact: The cause of a size is a stated rule, first match wins, read at 1/64 px; a height percentage counts as definite only when its container's computed height is not auto.
@task_fact n=4 slug=standard-contracts
fact: Session::layout(id) is an in-process driver read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId; it resolves its id as the run path does, refusing stale or not-found, records the screen's ids, and runs no step, pump, tick or settle and opens no span.
@task_fact n=5 slug=standard-contracts
fact: The layout read on the carriers is absent, PROVISIONAL on the founder's word at the epoch boundary: no verb, MCP tool, session socket call or kept check reaches Session::layout.
@task_fact n=6 slug=standard-contracts
fact: Placer::App with a canvas name is a reserved placing word of the driver's layout answer, built by no engine path; an app-placed canvas element is to take it.
@task_fact n=7 slug=standard-contracts
fact: An element the engine reader cannot explain reads Placer::Engine(Block) with its snapshot bounds, no container and both causes Layout, in process only; before the read crosses any carrier it is owed a closed word of its own.
@task_fact n=8 slug=established-decisions
fact: A closed select's option reads the placing word Select at its select's box although the default sheet gives it display none; a closed select's optgroup, display contents, reads no layout reason.
@task_fact n=9 slug=conventions
fact: A tolerance stated to 1/64 px is read inclusively, the absolute difference at most 1/64, since a sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
@task_item n=1 part=untouched
The session socket wire, its versions, bounds and errors: no message, version or reply changed.
@task_item n=2 part=untouched
The MCP role and server, its tool listing, tool results and handshake: no tool was added.
@task_item n=3 part=untouched
Kept check files, recording, replay and handler coverage: no kept verb or key was added.
@task_item n=4 part=untouched
Snapshot, node keys, snapshot text, diff and the actions, lint and graph verbs' answers: no node key, field or verb answer moved.
@task_item n=5 part=untouched
Telemetry bootstrap, scrub, line format and the command span's fields: no target, field or span was added.
@task_item n=6 part=untouched
Paint, hit testing, scrolling behaviour, the select list's machinery and DOM event dispatch: no document behaviour changed.
@task_item n=7 part=untouched
The browser app, WPT runner, scripting runtime, networking and history: not reached.
@task_item n=8 part=untouched
CI legs, workflow, dependencies, toolchain and occupied names (crates, binaries, env vars, ports): no crate, dependency, leg, name or listener was added.
@task_item n=9 part=untouched
Stable element id tiers and the UI graph: ids are read as before, no tier or registry changed.
