# design · picked accounting · group 4 of 6

## Picked
spacing#2.4 · carried · the seven_guis fixture's `cells.rs` is a conflict surface, and this line says where the stand's padding values are read
spacing#2.5 · carried · the rule lives in `default.css`, a predicted conflict surface
spacing#2.6 · carried · the rule lives in `default.css`, a predicted conflict surface, and rules over the select model's rows
spacing#2.7 · carried · the rule lives in `stylo.rs` and `default.css`, both conflict surfaces, and the scope names the `size` hint
spacing#3.1 · carried · the rule lives in `stylo.rs`, a predicted conflict surface
spacing#3.2 · carried · the rule lives in `stylo.rs`, a predicted conflict surface
spacing#3.4 · carried · the select list stays unchanged and its merge surface is re-read row by row, which this geometry belongs to
spacing#3.5 · carried · `place` belongs to the select list whose merge surface the chunk re-reads row by row and keeps unchanged
spacing#3.6 · carried · the rule lives in `node/node.rs`, a predicted conflict surface
spacing#3.8 · carried · the rule lives in blitz-paint `text.rs`, a predicted conflict surface that `f76777766` can reach
depth-strategy#1.6 · carried · the rule lives in `default.css`, a predicted conflict surface
depth-strategy#2.2 · carried · the rule lives in `node/node.rs`, a conflict surface, and `6fca6308d` adds `isolation`
depth-strategy#3.1 · carried · the scope keeps the hit stops through the `node/node.rs` conflict
depth-strategy#3.2 · carried · the scope names the select list's rows answered before the walk, and the list's merge surface is re-read
depth-strategy#3.3 · carried · the scope names `0980cce5f`'s `hit_inner` conflict with the non-invertible answer
depth-strategy#3.4 · carried · the scope names `visible_region` on the `scrolling.rs` conflict surface and re-reads its merge row
depth-strategy#4.2 · carried · the list's paint order belongs to the merge surface the chunk re-reads row by row and keeps unchanged
depth-strategy#4.6 · carried · the rule is witnessed by `select_popup_painted`, the font-embolden check the chunk re-reads, and the list's merge surface holds it
border-radius#1.3 · carried · the seven_guis fixture's `cells.rs` is a conflict surface, and this line says where the stand's radii are read
border-radius#1.5 · carried · the rule lives in `default.css`, a predicted conflict surface
border-radius#1.6 · not in scope · `form_controls.rs` is no reached file and neither the scope nor a re-read check names the checkbox frame's radius, a re-check every change owes
border-radius#1.7 · not in scope · the thumb's radius in `render.rs` is named by no reached file, scope line or re-read check, a re-check every change owes
border-radius#2.1 · not in scope · per-corner radii in `render.rs` are named by no reached file, scope line or re-read check, a re-check every change owes
border-radius#2.3 · carried · the rule lives in `stylo.rs`, a predicted conflict surface
