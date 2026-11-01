# Classified — design · depth-strategy.b1

## Answers
depth-strategy#1.1 · no · browser.css z-index values; the scope names no app file and lists the browser depth values as not needed
depth-strategy#1.2 · no · browser menu box-shadow in browser.css; app CSS is not reached and is listed as not needed
depth-strategy#1.3 · no · a pointer to layout-templates for the seven_guis card and dialog shadows; it states no value that cells.rs or another reached file holds
depth-strategy#1.4 · no · todomvc shadows; fixture values of examples other than seven_guis are not in scope
depth-strategy#1.5 · no · z-index values of the wgpu_texture, custom_widget and transforms examples; those fixtures are not reached and their values are not in scope
depth-strategy#1.6 · yes · the default.css dialog backdrop rgba value; default.css is a reached conflict surface whose recorded engine default values are read against the resolved sheet
depth-strategy#2.1 · no · paint-build order lives in unreached resolve.rs; the lib.rs:89 re-export is a re-check, since the take edits lib.rs only at lines 96-97
depth-strategy#2.2 · yes · the stacking-context root rule lives in node.rs, a conflict surface, and the isolation and mix-blend-mode additions of 6fca6308d reach it
depth-strategy#3.1 · yes · paint-versus-hit child order and the clipping stop at the padding box live in node.rs hit testing, which the Needs name
depth-strategy#3.2 · yes · the open select list answers hit first and a button under it is refused covered, which the Needs require for rows answered before the walk
depth-strategy#3.3 · yes · a non-invertible transform answers no hit for its subtree, the rule the 0980cce5f hit_inner conflict in node.rs must keep
depth-strategy#3.4 · yes · visible_region reads transforms in scrolling.rs, a conflict surface named by the Needs for into-view scrolling
depth-strategy#4.1 · no · the z-index auto paint level lives in unreached render.rs and paint_order.rs; a re-check the fast leg already owes
depth-strategy#4.2 · yes · the open select list paints last in paint_scene, part of the select list paint the scope re-reads row by row
depth-strategy#4.3 · no · outset box-shadow clipping lives in unreached box_shadow.rs; no shadow surface is reached
depth-strategy#4.4 · no · inset box-shadow cut lives in unreached box_shadow.rs; no shadow surface is reached
depth-strategy#4.5 · no · opacity, filter and backdrop-filter layering lives in unreached render.rs; the compositing change named is mix-blend-mode and isolation only
depth-strategy#4.6 · yes · the open select list paints whole past opacity and clipping ancestors, the select list paint that the select_popup_painted witness re-reads
depth-strategy#5.1 · no · filter and z-index values of example fixtures; fixture values of examples other than seven_guis are not in scope
depth-strategy#5.2 · no · google and graphite fixture shadows and z-index; fixture values of unnamed examples are not in scope
depth-strategy#5.3 · no · hr, servo, preact and box_shadow example values; fixture values of unnamed examples are not in scope
depth-strategy#6.1 · no · a search absence for z-index and shadow handling; it states no rule the chunk must honour
depth-strategy#6.2 · no · a search absence for shadow or elevation tokens; the chunk defines no token set
depth-strategy#6.3 · no · a search absence for shadow and elevation definitions; it states no rule the chunk must honour
depth-strategy#6.4 · no · a search absence for radius, shadow and z-index tokens; the chunk defines no token set
