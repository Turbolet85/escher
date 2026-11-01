# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the Dioxus bridge's accessibility_tree override in dioxus_document.rs, a conflict surface, and the default UA sheet parsed from default.css, also a conflict surface
standard-contracts#2.1 · yes · the clock invariant through taken code: no Instant::now or SystemTime::now outside clock.rs, and the clock surface in scrolling.rs, which the task says survives the resolution
standard-contracts#2.2 · yes · select_value and its choice writers sit on the one apply_select_choice path, which the scope keeps unchanged
standard-contracts#2.3 · yes · the select list's state field and open-list methods, the open list the scope keeps unchanged by any resolution
standard-contracts#2.4 · yes · live_select_list and reconcile_select_list, named in the scope's select model and resolve order
standard-contracts#2.5 · yes · resolve's reconcile_select_list call before refresh_hover, which the scope names as the resolve order
standard-contracts#2.6 · yes · closed_select_of in the Dioxus snapshot, which the bridge resolution in dioxus_document.rs keeps alongside the open list
standard-contracts#2.7 · yes · the mutator's select record on a multiple write, in mutator.rs, a conflict surface whose select arm the scope keeps unchanged
standard-contracts#3.1 · yes · the mutate() write methods live in mutator.rs, a conflict surface whose changed-set contract the scope keeps; the selector and CSSOM readers reach the re-proven legs
standard-contracts#3.2 · yes · a single select's set_expanded from the open list, an accessibility-tree state the re-proven accessibility_* checks and the a11y leg witness
standard-contracts#3.3 · yes · the tree's focus falling back to the window, named in the engine-debts merge surface row
standard-contracts#3.4 · yes · native toggled written from checkbox_input_checked, named in the engine-debts merge surface row
standard-contracts#4.1 · yes · focus-change events, the inert focus fixup, Space and Enter activation and the range input's input event, named by the scope's inert fixup, the a11y target's stand_keyboard and the merge surface
standard-contracts#4.2 · yes · the select list's default actions in pointer.rs and keyboard.rs, the select-list merge surface re-read row by row
standard-contracts#4.3 · yes · the label arm of handle_click in pointer.rs, on the select-list merge surface; its witness sits in select_popup.rs, which the chunk edits
standard-contracts#4.4 · yes · change on a checkbox and a radio, named in the engine-debts merge surface row
standard-contracts#5.1 · yes · scroll_into_view, visible_region, the hit walk and Node::absolute_position, named in the scope's hit stops and scrolled-box bounds, with node.rs a conflict surface
standard-contracts#6.1 · yes · get_client_bounding_rect under transforms and its readers, named in the scope's hit stops and scrolled-box bounds
standard-contracts#6.2 · yes · visible_region under transforms, named in the scope's hit stops and scrolled-box bounds
standard-contracts#6.3 · yes · text_extent, witnessed by blitz-tests' text_extent check that the re-asserted fast leg runs, over inline layout the take's inline/build.rs conflict surface feeds
standard-contracts#6.4 · yes · hit_inner's skip of dropped nodes in node.rs, a conflict surface, and hit_with_scrollbar answering the select list first, both named in scope
standard-contracts#6.5 · yes · a non-invertible transform's no hit, named in the engine-debts merge surface row, with the rule in node.rs, a conflict surface
standard-contracts#7.1 · yes · the layout traits and the ConstructionTask types in construct.rs, a conflict surface, under the taffy 0.15 pin the take carries
