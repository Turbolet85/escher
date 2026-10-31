# Classified — architecture · standard-contracts.b1

## Answers

standard-contracts#1.1 · yes · add-to · open and close write the changed set, a new kind of site that the changed-set sentence leaves to its Cross-cutting statement (touch :9), and the crate root re-exports `SelectListView` and `SelectListRow` (F12)
standard-contracts#2.1 · no · the list reads no clock, the clock contract is unedited, and the Clock re-export at lib.rs:98 is not among the moved citations
standard-contracts#2.2 · no · select_value and the choice writers are unchanged, and apply_select_choice remains the one write path
standard-contracts#2.3 · yes · stale · clear_select_typeahead's one non-test caller is now focus_select, not the retired focus_multiple_select, and is_multiple_select is reached only through closed_select_of since locate tests is_select; public open_select, select_list_bounds, select_list_view, select_row_layout and close_select_list join the readers and writer (F10), and is_select re-exported beside closed_select_of with select_list_bounds read before the client rect for a row (F18)
standard-contracts#2.4 · no · mutator.rs is not reached, so the multiple-write record and its removal settling stand
standard-contracts#3.1 · yes · contradict · an optgroup's label is now painted as a header, so "named in the tree and unpainted" no longer holds, and a single select's aria-expanded comes from engine state with no attribute read (F17)
standard-contracts#3.2 · no · the tree focus write is unchanged, the list adds no tree node, and its highlighted row is absent from the tree
standard-contracts#3.3 · no · the native checkable toggle path is unedited and the list adds no native checkable control
standard-contracts#4.1 · yes · stale · Enter and a released Space on a single select now open or choose a row instead of doing nothing, so the Click sentence no longer covers a single select; a press in the list starts no text drag; type fires no DOM focus events and focus_select closes any list (F20); a label's click focuses a bound single select and opens nothing (F22)
standard-contracts#4.2 · no · the checkable change arms are unedited, and the select choice's change is already stated under standard-contracts#4.1
standard-contracts#5.1 · yes · contradict · scroll_into_view on a row of an open list writes only the list's scroll_y and returns (F15), so "scrolls every scrolling box on its target's containing-block chain" no longer holds for it; hit_with_scrollbar answers the list first and visible_region answers the list's scrollport for an open row or group (F13)
standard-contracts#6.1 · yes · add-to · the snapshot's builder reads select_list_bounds before get_client_bounding_rect for a row of an open list, so the snapshot's bounds reader gains a list branch (F18)
standard-contracts#6.2 · yes · stale · the claim that callers execute.rs in_view and crop and lint.rs clip_region are unchanged no longer holds, since clip_region takes an open row's region from visible_region alone with no contain paint cut (F21), and visible_region answers the list's scrollport for an open row
standard-contracts#6.3 · no · text_extent reads inline layout the list does not have, and the reader and its one caller are unchanged
standard-contracts#6.4 · yes · stale · hit_with_scrollbar, named here as a caller that is unchanged, now answers select_list_hit first (F13), while hit_inner stays unedited so the dropped-node skip holds
standard-contracts#6.5 · no · hit_inner is unedited and the list adds no transform, so the non-invertible-transform rule holds
standard-contracts#7.1 · no · no layout module, Taffy trait or table path moved, and the list is no layout box
standard-contracts#8.1 · yes · add-to · node/mod.rs gains the select_list module beside select, and node::select_list's two public read types join the node module's exports (F12)
standard-contracts#9.1 · yes · add-to · row_disabled joins the disabled readings beside is_disabled for a list row
standard-contracts#9.2 · yes · add-to · node::select_list, with two public read types, stands beside node::select, which keeps no public type (F12)
standard-contracts#9.3 · yes · add-to · the list's crate-private live_select_list reader, with reconcile_select_list and the removal arm as its exceptions and hover_list_row writing current directly (F11); SelectList.fresh holding a just-opened list against a :focus restyle (F23); MAX_ROWS 20 with the placement and width rules (F24)
standard-contracts#9.4 · no · out_of_flow's writers and readers in node.rs and layout/mod.rs are unreached, and the list is no positioned box
