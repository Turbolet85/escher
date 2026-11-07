@answer n=1 entity=input-validation#1.76 v=no
why: checkbox and radio change dispatch lives in blitz-dom handle_click, which the chunk does not edit or reach, and no re-run check named here witnesses it
@answer n=2 entity=input-validation#1.77 v=yes
why: names stand_act_select, a stand_* check the chunk re-runs as its witness of unchanged behaviour
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field in blitz-dom document.rs is not a unit the chunk edits and no re-run check names it
@answer n=4 entity=input-validation#1.79 v=no
why: open list key handling is blitz-dom keyboard code, pinned by select_popup tests the chunk does not re-run
@answer n=5 entity=input-validation#1.80 v=no
why: pointer handling inside an open select list is blitz-dom code, pinned by select_popup tests the chunk does not re-run
@answer n=6 entity=input-validation#1.81 v=no
why: hostile markup under an open list is blitz-dom select layout, pinned by select_popup tests the chunk does not re-run
@answer n=7 entity=input-validation#1.82 v=yes
why: names stand_act_select, a stand_* check the chunk re-runs as its witness of unchanged behaviour
@answer n=8 entity=input-validation#1.83 v=yes
why: names stand_act_select, a stand_* check the chunk re-runs as its witness of unchanged behaviour
@answer n=9 entity=input-validation#1.84 v=no
why: multiple_select_values in dioxus_document.rs lines 31-45 is an unedited unit of an edited file, a re-check every change owes, and no re-run check names it
@answer n=10 entity=input-validation#1.85 v=no
why: driver type on a select runs through execute.rs and schema.rs, which the chunk does not reach
@answer n=11 entity=input-validation#1.86 v=no
why: the public select readers are blitz-dom and snapshot API, and lib.rs lines 35-38 are re-exports outside the edited module-list unit, with no re-run check naming them
@answer n=12 entity=input-validation#1.87 v=no
why: the open select list public surface is blitz-dom API the chunk does not reach, and no re-run check names it
@answer n=13 entity=input-validation#1.88 v=yes
why: the password and file input MASKED_VALUE snapshot value is named by task items 2 and 7 as a value the buffer and the snapshot must keep, and the stand_* checks named here are re-run
@answer n=14 entity=input-validation#1.89 v=no
why: the degenerate aspect ratio discard in replaced.rs is a single layout guard the chunk does not touch, and incremental_oracle checks identity between layout modes rather than this rule
@answer n=15 entity=input-validation#1.90 v=no
why: the inline box height clamp in layout inline code is untouched by the chunk and no re-run check names it
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent is a read-only blitz-dom reader the chunk does not reach, and no re-run check names it
@answer n=17 entity=input-validation#1.92 v=yes
why: names stand_act_transformed, a stand_* check the chunk re-runs as its witness of unchanged behaviour
@answer n=18 entity=input-validation#1.93 v=no
why: paint culling of non-finite geometry is blitz-paint code the chunk does not touch, and no re-run check names it
@answer n=19 entity=input-validation#1.94 v=no
why: the select list paint in blitz-paint is untouched by the chunk and no re-run check names it
@answer n=20 entity=input-validation#1.95 v=no
why: the non-invertible transform hit test is blitz-dom node code the chunk does not touch, and its pin hit_non_invertible_transform is not re-run
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping is blitz-paint code the chunk does not touch and no re-run check names it
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling is blitz-paint code the chunk does not touch and no re-run check names it
@answer n=23 entity=input-validation#1.98 v=no
why: hidden input paint skipping is blitz-paint code the chunk does not touch and no re-run check names it
@answer n=24 entity=input-validation#1.99 v=yes
why: the scope names the MountedData scroll_to and scroll operations that now run during render, and this entity states the missing-node no-op guard those calls rely on, which the new path must keep without adding a panic
@answer n=25 entity=input-validation#1.100 v=no
why: the open list's own scroll_into_view path is not one of the MountedData scroll verbs the scope names, and its pins are select_popup tests the chunk does not re-run
