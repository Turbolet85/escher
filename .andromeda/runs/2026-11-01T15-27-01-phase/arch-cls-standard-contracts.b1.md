# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the changed-set readers `has_changes` and `take_changed_nodes` that the mutator.rs resolution keeps, and the `ua_stylesheets` default sheet in default.css, a conflict surface
standard-contracts#2.1 · yes · the document clock invariant the Needs name, with the clock surface in scrolling.rs surviving the resolution
standard-contracts#2.2 · yes · the select model the Needs keep unchanged: `select_value` and the choice writers behind the one path `apply_select_choice`
standard-contracts#2.3 · yes · the select list field and its readers and writers, the open list the Needs keep unchanged
standard-contracts#2.4 · yes · `live_select_list` and `reconcile_select_list`, named in the Needs as the list's reader and its reconcile
standard-contracts#2.5 · yes · `resolve` calling `reconcile_select_list` after layout, the resolve order the Needs keep
standard-contracts#2.6 · yes · the select read surface and `closed_select_of` feeding the Dioxus bridge's snapshot, which dioxus_document.rs (a conflict surface) carries, with the re-exports kept
standard-contracts#2.7 · yes · the select record on a `multiple` write in mutator.rs, a conflict surface, and the select arm the Needs keep
standard-contracts#3.1 · yes · the mutator.rs write methods (a conflict surface) and the accessibility tree and state builder that the a11y target re-proves through `accessibility_*` and the a11y leg
standard-contracts#3.2 · yes · a single select's open list in the tree, `expanded` written from the list, part of the open-list model the Needs keep and re-proven by `accessibility_*`
standard-contracts#3.3 · yes · the tree's focus falling back to the window, named in the engine-debts merge surface
standard-contracts#3.4 · yes · the native checkbox's `toggled` in the tree, named in the engine-debts merge surface
standard-contracts#4.1 · yes · the inert focus fixup dispatching no events and the checkable `change` dispatch, both engine-debts merge rows
standard-contracts#4.2 · yes · the select list's default actions and the `apply_select_choice` path, re-read row by row in the select list merge surface
standard-contracts#4.3 · yes · the label arm's no-open rule, witnessed by select_popup.rs's `a_labels_click_focuses_the_select_and_opens_nothing`, which the chunk re-asserts as an escher proof
standard-contracts#4.4 · yes · the checkbox and radio `change` dispatch, named in the engine-debts merge surface
standard-contracts#5.1 · yes · `scroll_into_view`, `visible_region`, the padding-box hit stop and `Node::absolute_position`, the scrolled-box bounds the Needs keep (scrolling.rs a conflict surface)
standard-contracts#6.1 · yes · `get_client_bounding_rect` under transforms, named in the hit stops and scrolled-box bounds the Needs keep
standard-contracts#6.2 · yes · `visible_region` under transforms, named in the Needs and read by execute.rs `in_view` and `crop`
standard-contracts#6.3 · yes · `text_extent`, witnessed by blitz-tests' `text_extent` in both layout modes, which the chunk re-asserts, and read at a client rect the take moves
standard-contracts#6.4 · yes · `hit_inner` passing over a dropped child, the hit walk the Needs name, in node.rs (a conflict surface)
standard-contracts#6.5 · yes · a non-invertible transform's no hit, named in the engine-debts merge surface
standard-contracts#7.1 · yes · the layout traits that taffy 0.15 and `TaffyStyloStyle` implement, a dependency pin the take moves, with construct.rs and inline/build.rs as conflict surfaces
