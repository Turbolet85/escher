# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · add-to · NetHandler::error (F13) stands beside the per-kind bytes list and the stylesheet handler's error responds Err (F14); net.rs and document.rs are merge surfaces whose cited digits move too
standard-contracts#2.1 · yes · stale · the crate-root re-export citation at packages/blitz-dom/src/lib.rs:96 sits in a reached file, so its digit moves
standard-contracts#2.2 · yes · stale · document.rs is a reached merge surface, so the citations at document.rs:361-362, :735-749 and :1101-1149 move
standard-contracts#2.3 · yes · stale · document.rs is a reached merge surface, so the citations at document.rs:365-366 and :772-1100 move
standard-contracts#2.4 · yes · stale · document.rs is a reached merge surface, so the citations at document.rs:808-815, :905 and :1419 move
standard-contracts#2.5 · yes · stale · document.rs:1419-1421, in a reached merge surface, moves
standard-contracts#2.6 · yes · stale · document.rs:751-770, in a reached merge surface, moves
standard-contracts#2.7 · yes · stale · mutator.rs is a reached merge surface, so mutator.rs:1334-1343 moves
standard-contracts#3.1 · yes · stale · resolved_style now reads width and height sub-pixel and resolved_style.rs and document.rs (merge surfaces) move their cited digits; F11 restyle_for_attribute_change and restyle_for_child_list_change join the mutation list beside it
standard-contracts#3.2 · no · the single select's set_expanded and aria-expanded read sit in accessibility.rs, which the change does not reach
standard-contracts#3.3 · no · the tree's focus rule sits in accessibility.rs, not reached, and focus stays on the control whose block was dropped, so the built-nodes rule still holds
standard-contracts#3.4 · no · the native checkable toggle write sits in accessibility.rs, not reached, and the native checkedness read is unchanged
standard-contracts#4.1 · no · the dispatch it describes sits in events/ and blitz-traits events.rs, none reached, and the inert focus fixup is re-read and unmoved
standard-contracts#4.2 · no · the select list default actions sit in events/pointer.rs and events/keyboard.rs, none reached, and the session touches no list dispatch
standard-contracts#4.3 · yes · stale · the citation to tests/blitz-tests/tests/select_popup.rs:428-441 sits in a reached file, so its digits move
standard-contracts#4.4 · yes · stale · its claim that blitz-vibey-script's runtime synthesizes no change lives in bootstrap.rs, a merge surface, so it must be re-read, and its cited digits sit in a reached file
standard-contracts#5.1 · yes · add-to · TElement::borrow_data and the style-resolve context (F12) join the stylo.rs items resolve_undisplayed_style and resolve_stylist; node.rs and stylo.rs (merge surfaces) move the cited digits
standard-contracts#6.1 · yes · stale · get_client_bounding_rect's body in document.rs (merge surface) moves its cited lines, and dioxus_document.rs (merge surface) moves the get_client_rect reader with it
standard-contracts#6.2 · no · visible_region lives in scrolling.rs and the tests it cites, none reached, and no transform reader is touched by the session
standard-contracts#6.3 · yes · stale · the crate-root re-export citation at packages/blitz-dom/src/lib.rs:112 sits in a reached file, so its digit moves
standard-contracts#6.4 · yes · stale · the hit walk lives in node.rs and document.rs, both merge surfaces, so the cited digits move and the walk must be re-read
standard-contracts#6.5 · yes · stale · the non-invertible transform check at packages/blitz-dom/src/node/node.rs:1485-1490 sits in a merge surface, so its digits move
standard-contracts#7.1 · yes · add-to · the paint-tree StackingContext list gains the mix-blend-mode arm of Node's stacking-context read (F18), and the layout construction list gains IS_INLINE_WITH_ONLY_BLOCK_CHILDREN set in collect_layout_children_with_wrap (F20); construct.rs, a merge surface, moves its cited digits
