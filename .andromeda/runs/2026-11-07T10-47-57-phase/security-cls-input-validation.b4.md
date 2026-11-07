@answer n=1 entity=input-validation#1.76 v=no
why: the checkbox and radio change event is handle_click's arm, which the chunk's render path and MountedData calls do not reach.
@answer n=2 entity=input-validation#1.77 v=yes
why: the replay writes selected and multiple through MutationWriter, so the select settlement at the mutator's flush must still run once over the whole buffer.
@answer n=3 entity=input-validation#1.78 v=no
why: the open select list field and reconcile_select_list are blitz-dom list state that no render-path or MountedData call the chunk changes reads.
@answer n=4 entity=input-validation#1.79 v=no
why: keyboard routing of an open select list, which no chunk code or named scope call reaches.
@answer n=5 entity=input-validation#1.80 v=no
why: pointer press and close handling of an open select list, a path the chunk does not add or change.
@answer n=6 entity=input-validation#1.81 v=no
why: list box sizing and the 20-row cap under hostile markup, a layout rule the chunk does not change.
@answer n=7 entity=input-validation#1.82 v=no
why: keys and clicks on a multiple select, where the chunk routes no input events.
@answer n=8 entity=input-validation#1.83 v=no
why: the optgroup record and group role come from the mutator's own writes, which the replay repeats in the original call order as scope item 4 requires.
@answer n=9 entity=input-validation#1.84 v=no
why: the multiple select's set leaves through multiple_select_values in dioxus_document.rs lines 31-45, a unit the chunk does not edit, on the event path.
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type and focus_select in execute.rs, which the chunk does not touch.
@answer n=11 entity=input-validation#1.86 v=no
why: the select readers in document.rs, snapshot.rs and the lib.rs re-exports are untouched, and the chunk adds no select reader or re-export.
@answer n=12 entity=input-validation#1.87 v=no
why: the open select list's public readers and close_select_list in document.rs, which the chunk does not call or change.
@answer n=13 entity=input-validation#1.88 v=yes
why: the replay carries password and file input values through the buffer, which must stay masked in the snapshot, its diff and its text as scope items 2 and 7 require.
@answer n=14 entity=input-validation#1.89 v=no
why: replaced-element aspect-ratio layout, which no render-path or MountedData call the chunk changes reaches.
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height clamping in the line breaker, untouched by the chunk.
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent is a read-only lint geometry reader called only by Session::lint, which the chunk does not reach.
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region's callers are execute.rs in_view and crop and lint.rs clip_region, and get_client_rect does not reach it.
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling before painting, and the chunk paints nothing.
@answer n=19 entity=input-validation#1.94 v=no
why: paint_select_list's box and row painting, which the chunk does not reach.
@answer n=20 entity=input-validation#1.95 v=no
why: Node::hit_inner's non-invertible transform rule, not reached by the render or MountedData path.
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping in paint, untouched by the chunk.
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling in paint, untouched by the chunk.
@answer n=23 entity=input-validation#1.98 v=no
why: the paint skip for input type=hidden, untouched by the chunk.
@answer n=24 entity=input-validation#1.99 v=yes
why: scroll_to and scroll are MountedData verbs the scope names, and the clamping and missing-node no-effect rule is what they carry.
@answer n=25 entity=input-validation#1.100 v=no
why: scroll_into_view on an open select list row, a verb the scope does not name.
