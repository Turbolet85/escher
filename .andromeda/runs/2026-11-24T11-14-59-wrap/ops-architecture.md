@row_set n=1 row=decision/taffy-layout cell=rule seen=2 detector=D-arch-decisions
value: Taffy 0.16 from crates.io does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
basis: fct-xlmy5ueo
because: Taffy moved from 0.15 on upstream's pinned git source to 0.16.0 from crates.io, so the row's version and source no longer hold.
@row_set n=2 row=decision/no-git-dependencies cell=except seen=3 detector=D-arch-decisions
value:
basis: fct-x3oubhd5
because: The workspace's one git dependency, Taffy's git pin, is retired and the lock holds no git source, so the Taffy exception no longer stands.
@row_set n=3 row=contract/layout/stylo-taffy-exports cell=promise seen=2 detector=D-arch-resources
value: stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt; its convert module's float and clear converters take the box's direction as a second argument, an ltr flag, and its calc resolver takes Taffy's CalcHandle.
basis: fct-xuofjrvy
because: stylo_taffy's calc resolver now takes taffy::CalcHandle in place of a raw pointer, a changed signature at the crate's boundary the row does not name.
@row_set n=4 row=decision/one-axis-clip-readings-kept cell=rule seen=1 detector=none
value: A box clipped on one axis stops the hit at its whole padding box while the visible region and paint narrow only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed.
basis: fct-xuofjrvy
because: Paint now clips only the clipped axis for a one-axis overflow clip (C1 A), so the re-stated row names paint beside the visible region.
@row_set n=5 row=decision/default-action-event-order cell=rule seen=2 detector=none
value: Default-action events are queued and processed in order, handlers running before the default action unless cancelled, and preventDefault on a non-cancelable event cancels nothing.
basis: fct-xuofjrvy
because: The taken upstream change ignores preventDefault on a non-cancelable event (P1 A), a clause of what cancels a default action the row leaves unsaid.
@row_set n=6 row=decision/handlers-before-default-action cell=rule seen=2 detector=none
value: Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled, and preventDefault on a non-cancelable event cancels nothing.
basis: fct-xuofjrvy
because: The taken upstream change ignores preventDefault on a non-cancelable event (P1 A), a clause of what cancels a default action the row leaves unsaid.
@row_add n=1 row=decision/os-drop-input area=established-decisions fact=F1 detector=D-arch-resources
name: OS drag-and-drop input through the shell's drop source
rule: blitz-shell's winit drop source turns OS drags into the UiEvent variants DragOver, DragLeave and Drop, and a dropped file arrives only through a file URI.
why: ratified
basis: fct-xuofjrvy
because: The merge brings OS drag-and-drop input as a new input path into every windowed document, a ratified decision no row states.
@row_add n=2 row=decision/dropped-file-read-unrestricted area=established-decisions fact=F2 detector=none
name: Dioxus dropped-file reader reads without a path restriction
rule: The Dioxus dropped-file reader reads a dropped file through std::fs with no path restriction.
why: upstream's design, ratified
basis: fct-xuofjrvy
because: The Dioxus bridge gains a dropped-file reader whose unrestricted file read is a ratified design choice no row states.
@row_add n=3 row=contract/blitz-traits-data-transfer/blitz-data-transfer area=standard-contracts fact=F3 detector=D-arch-resources
boundary: blitz-traits DataTransfer model
item: BlitzDataTransfer
promise: Holds string and file items with their types and files, effectAllowed and dropEffect, and a store mode, the operations named by BlitzDragOperation and BlitzDragOperations.
basis: fct-xuofjrvy
because: blitz-traits gains the shared DataTransfer model, a new public type at the crate's boundary unregistered in Standard Contracts.
@row_add n=4 row=contract/blitz-traits-data-transfer/blitz-drag-event area=standard-contracts fact=F3 detector=D-arch-resources
boundary: blitz-traits DataTransfer model
item: BlitzDragEvent
promise: Carries the pointer coordinates and the shared data transfer of a drag.
basis: fct-xuofjrvy
because: blitz-traits gains BlitzDragEvent, the payload of the new drag UI events, unregistered in Standard Contracts.
@row_add n=5 row=contract/ui-event/drag-over area=standard-contracts fact=F4 detector=D-arch-resources
boundary: UiEvent
item: DragOver
promise: Dispatches DOM dragenter and dragover to the node under the drag.
basis: fct-xuofjrvy
because: UiEvent gains the DragOver variant mapped to DOM drag events, a new event at the boundary no row states.
@row_add n=6 row=contract/ui-event/drag-leave area=standard-contracts fact=F4 detector=D-arch-resources
boundary: UiEvent
item: DragLeave
promise: Fires DOM dragleave, also when the drag leaves the document.
basis: fct-xuofjrvy
because: UiEvent gains the DragLeave variant mapped to DOM dragleave, a new event at the boundary no row states.
@row_add n=7 row=contract/ui-event/drop area=standard-contracts fact=F4 detector=D-arch-resources
boundary: UiEvent
item: Drop
promise: Dispatches DOM drop to the node under the drag; a drop effect takes only where the drag source allows it.
basis: fct-xuofjrvy
because: UiEvent gains the Drop variant mapped to DOM drop, with the rule that the drag source must allow the drop effect, a new event at the boundary no row states.
@row_add n=8 row=contract/dioxus-drag-events/native-drag-data area=standard-contracts fact=F5 detector=D-arch-resources
boundary: Dioxus drag events
item: NativeDragData
promise: Implements Dioxus's NativeDataTransfer, HasFileData, HasDataTransferData, HasDragData and ModifiersInteraction, so a Dioxus drag handler reads the data transfer, the dropped files and the keyboard modifiers.
basis: fct-xuofjrvy
because: dioxus-native-dom gains the drag event data a Dioxus handler receives, a new boundary item unregistered in Standard Contracts.
@row_add n=9 row=contract/nodes-widgets/widget-drop-acceptance area=standard-contracts detector=D-arch-resources
boundary: blitz-dom nodes and widgets
item: Widget drop acceptance
promise: A custom widget accepts a drop only by setting the DragOver event's dropEffect.
basis: fct-xuofjrvy
because: The Widget trait's event handling gains how a widget accepts a drop, a promise at the widget boundary no row states.
@row_add n=10 row=decision/br-in-flex-text-item area=established-decisions fact=F6 detector=none
name: A br between text breaks the line in its flex item
rule: A br between text in a flex or grid container breaks the line inside the text's anonymous item, also when hoisted from display contents, while a trailing br and a br between flex items add no line.
why: an upstream behaviour change taken, ratified for its take
guard: flex_br
basis: fct-xuofjrvy
because: The taken upstream change joins a br after text into the text's anonymous flex item, a layout behaviour for every document no row states.
@row_none n=1 fact=F7 type=5
what: An inventory of two root examples, their build-leg compilation and one example's refused previews; the examples' code and the build leg hold it.
