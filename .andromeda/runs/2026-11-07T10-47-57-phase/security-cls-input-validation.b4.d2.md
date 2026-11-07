@answer n=1 entity=input-validation#1.76 v=no
why: checkbox and radio change events live in blitz-dom's click path, which the buffered render does not touch, and the checks it names (checkable_change_event, text_range_no_change_event) are not re-run
@answer n=2 entity=input-validation#1.77 v=yes
why: the single-choice select's selectedness is settled at the mutator's flush, which the buffered replay moves, and it names stand_act_select, a stand_* check the chunk re-runs as witness
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field and its reconcile path are document state the render sites do not reach, and no re-run check is named
@answer n=4 entity=input-validation#1.79 v=no
why: an open list's keys are resolved in blitz-dom's keyboard path, untouched by the chunk, and its pins (select_popup) are not re-run
@answer n=5 entity=input-validation#1.80 v=no
why: pointer handling in and around an open list is untouched by the chunk, and its pins (select_popup) are not re-run
@answer n=6 entity=input-validation#1.81 v=no
why: hostile markup under an open list is layout and paint in blitz-dom, untouched by the chunk, and its pin (select_popup) is not re-run
@answer n=7 entity=input-validation#1.82 v=yes
why: the multiple select's keys and clicks are pinned by stand_act_select, a stand_* check the chunk re-runs as witness
@answer n=8 entity=input-validation#1.83 v=yes
why: optgroup handling is pinned by stand_act_select, a stand_* check the chunk re-runs as witness
@answer n=9 entity=input-validation#1.84 v=no
why: the multiple select's chosen set leaves through form entries and event values, and dioxus_document.rs:31-45 is a unit outside the render sites and tests the chunk edits, with no re-run check named
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type on a select is in execute.rs and schema.rs, which the chunk does not touch
@answer n=11 entity=input-validation#1.86 v=no
why: the public select readers are in blitz-dom and the snapshot builder, none of which the buffered render changes
@answer n=12 entity=input-validation#1.87 v=no
why: the select list's public readers and types are blitz-dom surface the chunk does not change
@answer n=13 entity=input-validation#1.88 v=yes
why: the buffer holds input values and text in process, so the password and file input mask must hold through the replay, and the stand_* checks it names (stand_snapshot_state, stand_diff, stand_act_diff, stand_mcp_masked, and others) are re-run as witness
@answer n=14 entity=input-validation#1.89 v=no
why: aspect ratio handling in layout/replaced.rs is untouched by the chunk and names no re-run check
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height clamping in layout/inline is untouched by the chunk and names no re-run check
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent is a read-only reader the chunk does not change or call, and names no re-run check
@answer n=17 entity=input-validation#1.92 v=yes
why: visible_region is pinned by stand_act_transformed, a stand_* check the chunk re-runs as witness
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling in blitz-paint is untouched by the chunk, and its pin (inline_with_only_block_children_tests) is not re-run
@answer n=19 entity=input-validation#1.94 v=no
why: the select list's paint path is untouched by the chunk and names no re-run check
@answer n=20 entity=input-validation#1.95 v=no
why: the non-invertible transform hit test is untouched by the chunk, and its pin (hit_non_invertible_transform) is not re-run
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping in blitz-paint is untouched by the chunk and names no re-run check
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling in blitz-paint is untouched by the chunk and names no re-run check
@answer n=23 entity=input-validation#1.98 v=no
why: hidden input paint skipping in blitz-paint is untouched by the chunk and names no re-run check
@answer n=24 entity=input-validation#1.99 v=yes
why: the MountedData scroll_to and scroll calls the scope names during render reach the scroll target clamp and the missing-node no-op
@answer n=25 entity=input-validation#1.100 v=no
why: an open list's own scroll is pinned by select_popup and scroll_into_view, neither of which the scope names or re-runs
