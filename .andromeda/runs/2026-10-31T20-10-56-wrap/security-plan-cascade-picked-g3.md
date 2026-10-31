# security-plan · picked accounting · group 3 of 4

## Picked
input-validation#1.61 · not touched · the report's one addition to `accessibility.rs` (238-242) writes a single select's `expanded`, and `NodeState` gains no field, so the description reads and their one exit hold
input-validation#1.62 · proposed · a single select now writes `set_expanded(open)` from engine state and reads no `aria-expanded`; F10 goes beside it
input-validation#1.63 · not touched · a chosen option's `selected` still comes from its state, `NodeState` gains no field and no redaction shape moved, so the state readings hold
input-validation#1.64 · not touched · the report's `accessibility.rs` addition is the single select's `expanded` alone, so the native toggle read holds
input-validation#1.65 · proposed · the `inert` rule itself holds; F5, an inert select opening no list, goes beside it
input-validation#1.66 · not touched · the tree's focus stays on the select with a list open (`the_tree_reads_the_list_open_on_the_select_its_focus_staying_there`) and the report moves no inert clear
input-validation#1.67 · not touched · `click-help` reads a click on a range no-change and the keyboard and pointer edits the report lists are select and list arms, so the range rules hold
input-validation#1.68 · proposed · the row's «a click on a `select` focusing it» now also opens or closes a single select's list, and the `option` arm gains `click_list_row`
input-validation#1.69 · not touched · the pointer edits the report lists name no checkbox or radio arm, so the engine's `change` rule holds
input-validation#1.70 · proposed · the retired limits, Enter and Space, a click's opening and `type`'s restatement end or move here; F4, F6, F7, F9 and F11 go beside it
input-validation#1.71 · proposed · `press_multiple_select_key` and `click_select_row` keep their semantics; F8, `size` on a `multiple` select, goes beside it
input-validation#1.72 · proposed · a group's `label` is now painted as a header and the clause reserving it as a name ends on his F7
input-validation#1.74 · proposed · `type` now covers any select, `focus_multiple_select` is `focus_select`, `Target.multiple` is `Target.select`
input-validation#1.75 · proposed · `is_select` joins the readers, the callers moved, `closed_select_of` answers `None` for an open list; F12 goes beside it
input-validation#1.76 · not touched · no redaction shape moved and the builder only reads `select_list_bounds` before the client rect, so the mask holds on the changed `snapshot.rs` and `execute.rs`
input-validation#1.80 · proposed · `visible_region` now answers the list's scrollport for an open list's row or group, and `clip_region` takes that region alone
input-validation#1.81 · proposed · the culling claim holds; the list's own paint values, past that cull, take a row beside it
input-validation#1.82 · not touched · `hit_inner` is not edited, so the claim holds; F5's transform clause is one proposal beside `inert`
input-validation#1.85 · not touched · the report's `render.rs` edit is the `paint_select_list` call alone (7, 246-247), so hidden inputs stay unpainted
input-validation#1.86 · proposed · the page's scroll rules hold; the list's own scroll and a wheel's close take a row beside them
input-validation#1.90 · not touched · the pointer edits the report lists are select and list arms, none on a file input, so `accept` stays no filter
