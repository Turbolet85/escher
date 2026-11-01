# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the changed-set readers `has_changes` and `take_changed_nodes` and their write sites are the mutator.rs contract the scope keeps (mutator.rs is a conflict surface); `ua_stylesheets` meets default.css, also a conflict surface
standard-contracts#2.1 · yes · the document clock invariant through taken code: `DocumentConfig.clock`, `Clock`, no `Instant::now()` outside `clock.rs`, and the clock surface in scrolling.rs, all named in the scope
standard-contracts#2.2 · yes · `select_value` and the choice writers (`set_select_choice`, `set_option_chosen`) are the select model's one path, which `apply_select_choice` drives and the bridge's snapshot `value` reads
standard-contracts#2.3 · yes · the open select list's readers and list functions (`open_select`, `reconcile_select_list`, `select_list_hit`) are the select model and open list the scope keeps unchanged
standard-contracts#2.4 · yes · `live_select_list` as the list-state reader, and `reconcile_select_list`'s read past it, are the open-list rule every resolution keeps
standard-contracts#2.5 · yes · `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, the resolve order the scope names
standard-contracts#2.6 · yes · `select_values`, `select_of` and the bridge's `closed_select_of` (no value for an open select) sit in the select model and the bridge's snapshot the take keeps
standard-contracts#2.7 · yes · a `multiple` write through the mutator records the select and `set_option_selectedness` leaves other options alone: the mutator's select arm, a conflict surface named in the scope
standard-contracts#3.1 · yes · the `mutate()` write sites and changed-set readers, and the accessibility tree's labels, states and descriptions that the a11y target is re-proven on through `accessibility_*` and the a11y leg
standard-contracts#3.2 · yes · a single select's `expanded` read from the open list, with no `aria-expanded` read on it: tree identity the a11y leg re-proves, beside the select model the scope keeps
standard-contracts#3.3 · yes · the tree's focus falls back to the window where `focus_node_id` is not among the built nodes, a named eight engine-debts merge surface row
standard-contracts#3.4 · yes · native `toggled` from `checkbox_input_checked()` and no `pressed` on a native control, a named eight engine-debts merge surface row
standard-contracts#4.1 · yes · the event driver, range `input`, Space and Enter activation, and `change` on a checkable control are default actions the select popup's key press and the engine-debts merge surface re-read
standard-contracts#4.2 · yes · a select list's pointer, click, wheel, Tab and key default actions: the select-list merge surface (sixteen files) and the skipped key press the scope edits
standard-contracts#4.3 · yes · the `handle_click` label arm focuses a bound single select and opens nothing: a select-list merge surface row re-read against the take
standard-contracts#4.4 · yes · `change` after `input` on a checkbox, and on a radio not checked before the click: a named engine-debts merge surface row
standard-contracts#5.1 · yes · the scroll API, `visible_region`, the hit walk's padding-box stop, `absolute_position`, `get_client_bounding_rect` and the scrolled-box bounds are the hit stops and bounds the scope keeps unchanged
standard-contracts#6.1 · yes · the client rect under transforms is `get_client_bounding_rect`, one of the scrolled-box bounds the scope names, with its readers moving with it
standard-contracts#6.2 · yes · `visible_region` under transforms, named in the scope's hit stops and scrolled-box bounds
standard-contracts#6.3 · no · `text_extent` is named in no reached file or scope line; its pinned blitz-tests are a re-check every change owes, run inside the fast leg
standard-contracts#6.4 · yes · the hit walk skipping a dropped node in `hit_inner` and the paint tree, named among the hit stops the scope keeps
standard-contracts#6.5 · yes · a non-invertible transform takes no hit, a named eight engine-debts merge surface row
standard-contracts#7.1 · yes · the Taffy trait implementations and the paint-tree API that the taffy 0.15 pin and the layout conflicts in construct.rs and writing_mode.rs carry
