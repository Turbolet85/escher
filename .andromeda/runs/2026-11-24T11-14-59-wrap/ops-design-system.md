@row_add n=1 row=decision/one-axis-clip-paints-clipped-axis area=depth-strategy fact=F1 detector=none
name: One-axis overflow clip clips only its axis
rule: For an overflow clip on one axis only, paint clips that axis and leaves the other axis unclipped.
why: Taken from upstream, ratified as upstream's behaviour.
basis: fct-xuofjrvy (blitz-paint: clipped_axes clips only the clipped axis for one-axis overflow: clip); fct-pwiivh4c (design-system §Depth, C1 A)
because: The chunk takes upstream's one-axis overflow clip paint on the founder's C1 A, and no Depth Strategy row states how a clip on one axis paints.
@row_add n=2 row=contract/os-drag-and-drop/drag-events area=surface fact=F2 detector=none
boundary: OS drag-and-drop on the desktop shell
item: dragenter, dragover, dragleave and drop
promise: The desktop shell accepts OS drag-and-drop of files and data and delivers it as dragenter, dragover, dragleave and drop events on the node under the drag.
basis: fct-xuofjrvy (blitz-traits drag events, blitz-dom drag dispatch, blitz-shell drop source); fct-pr3l6fxd (OS drop input)
because: The chunk adds OS drag-and-drop input to the desktop-native surface, a new promise at that surface no row holds.
@row_add n=3 row=contract/os-drag-and-drop/drop-effect area=surface fact=F2 detector=none
boundary: OS drag-and-drop on the desktop shell
item: dropEffect
promise: A drop lands only where the dropEffect is allowed by the drag source's effectAllowed.
basis: fct-xuofjrvy (the DataTransfer model); fct-pr3l6fxd (OS drop input)
because: The chunk's DataTransfer model gates a drop on the source's effectAllowed, a rule a drop target on this surface must respect.
@row_add n=4 row=contract/os-drag-and-drop/custom-widget-drop area=surface fact=F2 detector=none
boundary: OS drag-and-drop on the desktop shell
item: custom widget drop
promise: A custom widget accepts a drop only by setting the dragover event's dropEffect.
basis: fct-xuofjrvy (a custom widget accepts a drop only by setting the DragOver event's dropEffect)
because: The custom widget boundary gains the one way a widget accepts a drop, which future widgets must follow.
@row_add n=5 row=decision/non-cancelable-ignores-prevent-default area=surface fact=F3 detector=none
name: preventDefault ignored on non-cancelable events
rule: An event that cannot be cancelled ignores preventDefault, so a handler's preventDefault does not cancel pointerleave.
why: Taken from upstream, ratified as upstream's behaviour.
guard: prevent_default_does_not_cancel_pointerleave
basis: fct-xuofjrvy (preventDefault ignored on a non-cancelable event)
because: The event driver now ignores preventDefault on a non-cancelable event, a rule of the surface's event handling no row states.
@row_add n=6 row=decision/br-after-text-joins-flex-item area=typography fact=F4 detector=none
name: A br after text joins the text's flex item
rule: A br after text joins the text's anonymous flex item, so it breaks the line inside flex and grid containers; a trailing br adds no line, and neither does a br between flex items.
why: Taken from upstream, ratified as upstream's behaviour.
guard: flex_br
basis: fct-xuofjrvy (a br after text joins the text's anonymous flex item); fct-xppretyh (flex_br target)
because: The chunk takes upstream's line breaking of a br inside flex and grid containers, and no Typography row states it.
