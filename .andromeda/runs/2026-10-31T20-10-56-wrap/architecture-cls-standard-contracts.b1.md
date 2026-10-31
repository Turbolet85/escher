# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · add-to · F12 joins the crate-root public re-export list (lib.rs:77-135) with the SelectListView and SelectListRow read types
standard-contracts#2.1 · no · the Clock re-export at lib.rs:98 is not among the report's moved citations and clock.rs is not reached
standard-contracts#2.2 · yes · add-to · F10 puts the select_list field and its public readers beside select_typeahead, and F11 places the crate-private live_select_list reader beside the choice writers
standard-contracts#2.3 · yes · stale · clear_select_typeahead's caller is now focus_select, not focus_multiple_select, and the driver's select test is now is_select, so locate no longer calls is_multiple_select (F18 adds the public re-export)
standard-contracts#2.4 · no · mutator.rs is not reached and no moved citation or retired name touches the multiple-write records
standard-contracts#3.1 · yes · contradict · group labels now paint (F7), so the optgroup's "unpainted" is false, and a single select's expanded comes from engine state so its aria-expanded is no longer read (F17)
standard-contracts#3.2 · no · accessibility.rs is reached, but the focus rule's cited lines are not among the moved citations and the list's highlight is absent from the tree (F5), so focus stays on the select
standard-contracts#3.3 · no · the native checkable state write is not named by the session and accessibility.rs's moved citations do not include 199-224
standard-contracts#4.1 · yes · add-to · F22 adds that a label's click focuses a bound single select and opens nothing; Enter and a released Space on a single select now open or choose, and a press in the list starts no text drag
standard-contracts#4.2 · no · the checkbox and radio change dispatch at pointer.rs:865-908 is not among the session's moved citations and no checkable path is named as changed
standard-contracts#5.1 · yes · contradict · scroll_into_view on an open list's row writes only the list's scroll_y and returns (F15), and hit_with_scrollbar answers the list first (F13) with visible_region answering the list's scrollport for a row
standard-contracts#6.1 · yes · add-to · F18 adds that the snapshot's bounds for an open list's row read select_list_bounds before the client rect, a new case among the readers of get_client_bounding_rect
standard-contracts#6.2 · yes · stale · visible_region answers the list's scrollport for an open row and lint.rs clip_region takes that region alone (F21), so the callers-unchanged reading has moved
standard-contracts#6.3 · no · text_extent.rs is not reached, and its one caller Session::lint and its limits are not named as moved
standard-contracts#6.4 · yes · stale · hit_with_scrollbar, a named caller of hit_inner, now answers the list first, so "its callers hit and hit_with_scrollbar are unchanged" no longer holds
standard-contracts#6.5 · no · the non-invertible transform path at node.rs:1472-1477 sits in node.rs, which is not reached, and no transform change is named
standard-contracts#7.1 · no · no layout module, Taffy trait or paint-tree path moved and the list is no layout box
standard-contracts#8.1 · yes · add-to · F12 adds node::select_list with its two public read types to the node module re-exports at node/mod.rs:18-37
standard-contracts#9.1 · yes · add-to · row_disabled joins the disabled readings beside is_disabled for an open list's rows, which the pointer gates read
standard-contracts#9.2 · yes · add-to · node::select_list stands beside node::select with two public read types, while node::select keeps no public type
standard-contracts#9.3 · no · node/select.rs is not reached and the report keeps node::select's rules standing, and the select_cursor field in document.rs is not named as moved
standard-contracts#9.4 · no · node.rs and layout/mod.rs are not reached, and no moved citation names the out_of_flow record or its style-traversal write
