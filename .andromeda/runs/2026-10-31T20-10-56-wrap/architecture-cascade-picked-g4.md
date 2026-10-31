# architecture · picked accounting · group 4 of 8

## Picked
standard-contracts#1.1 · proposed · its pointer to what marks the changed set gains a select list's open and close, the new kind of site the report names at standard-contracts:9
standard-contracts#2.2 · proposed · beside `select_typeahead` now stand the `select_list` field, its readers and its resolve step (F10, F11, F14)
standard-contracts#2.3 · proposed · `focus_multiple_select` is retired for `focus_select`, `closed_select_of` answers `None` on an open list, and `is_multiple_select` and the new `is_select` have new callers
standard-contracts#3.1 · proposed · a group's label now paints as a header, and a single select's `expanded` comes from engine state with no `aria-expanded` read (F17)
standard-contracts#4.1 · proposed · the list adds default actions in `handle_pointerdown`, `handle_click`, the wheel and the keys, and a label's click on a single select no longer re-dispatches (F22)
standard-contracts#5.1 · proposed · `scroll_into_view` on a row stops at the list (F15), the list answers a hit before the walk, and `visible_region` answers its scrollport for a row or group
standard-contracts#6.1 · proposed · the snapshot's `bounds`, a named reader of the client rect, reads `select_list_bounds` first for a row (F18)
standard-contracts#6.2 · proposed · `clip_region`, stated unchanged, now takes an open row's region from `visible_region` alone (F21)
standard-contracts#6.4 · proposed · `hit_with_scrollbar`, stated unchanged, now answers `select_list_hit` first, while `hit_inner` stays unedited (F13)
standard-contracts#8.1 · not touched · `node/element.rs` and `node/node.rs` are outside the 41 and the list is a `BaseDocument` field, so no `NodeData` or `SpecialElementData` variant moved, and `node/mod.rs`'s two added lines leave its re-export claim true
standard-contracts#9.1 · proposed · `row_disabled` joins the disabled readings as the list's own; `is_disabled`'s claim stands
standard-contracts#9.2 · not touched · `node/select.rs` is outside the 41 and `node::select` stays crate-private with no public type, so the claim holds on the reached `node/mod.rs`; the new module's rows follow #9.3
standard-contracts#9.3 · proposed · the new `node::select_list` module, its public read types and its rules follow this row (F12, F23, F24), `apply_select_choice` still the one write path
standard-contracts#9.6 · not touched · the report states `:open` and `flush_open_state` unmoved on the reached `stylo.rs`, so the row holds; what an author's `open` does on a select stands with F17 at #3.1
standard-contracts#12.1 · proposed · `paint_scene` now calls `paint_select_list`, with the new paint module and `stroke_text_as` beside it (F16)
standard-contracts#14.1 · proposed · the harness's unedited `scroll_into_view` helper no longer moves every box and the viewport on a row of an open list
