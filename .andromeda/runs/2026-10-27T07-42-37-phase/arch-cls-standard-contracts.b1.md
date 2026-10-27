# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the changed set (`has_changes` until `take_changed_nodes` drains it) and the `Document` impls in document.rs, a merge surface, which the Needs' changed-set and shell-refresh items bind
standard-contracts#2.1 · yes · the document clock (`DocumentConfig.clock`) and its invariant, which clock.rs is re-read for and the Needs keep over upstream's rewritten scrolling code
standard-contracts#2.2 · yes · `select_value` and the crate-private choice writers in document.rs, a merge surface, behind the one `apply_select_choice` path the select model keeps
standard-contracts#3.1 · yes · `doc.mutate()` and the changed-set writes it makes in mutator.rs, a conflict surface, with the a11y builder's name and state reads the Needs keep
standard-contracts#4.1 · yes · the event driver, pointer handler and traits events (merge surfaces) that carry the Enter and Space activation, button focus and range arms the Needs keep
standard-contracts#5.1 · yes · `scroll_into_view` and `visible_region` in scrolling.rs, a conflict surface, and the hit walk and `Node::absolute_position` in node/node.rs, which the scrolling and hit-test Needs keep
standard-contracts#6.1 · yes · `get_client_bounding_rect` in document.rs (merge surface) and the second body of `physical_unrounded_geometry` in layout/writing_mode.rs (conflict surface), the scrolled-box bounds the Needs re-measure
standard-contracts#6.2 · yes · the fast leg's `text_extent` check and `stand_lint`, both re-run in both layout modes, re-assert its extents and None cases, and the inline layout it reads is built in layout/inline/build.rs (conflict surface)
standard-contracts#6.3 · yes · the hit walk's skip of a dropped node lives in node/node.rs (conflict surface), which the Needs' hit-test contract names
standard-contracts#7.1 · yes · `LayoutPassState` as the one Taffy-trait implementer the Needs name, `ConstructionTask` in layout/construct.rs (conflict surface), and the paint-tree API the one incremental pipeline keeps
standard-contracts#8.1 · yes · `SpecialElementData` with `RangeInput` in element.rs and `GeneratedTextInputEvent` in text.rs (merge surfaces), and the CSSOM View accessors in node/node.rs (conflict surface)
standard-contracts#9.1 · yes · `ElementData::is_disabled` in element.rs (merge surface) is the one disabled reading the Needs name for focus, the pointer gates and the activation keys
standard-contracts#9.2 · yes · the select model, one of the fork's direct engine changes re-read per row against the take (residual line 36), and node/select.rs is its crate-private rules module
standard-contracts#9.3 · yes · `Node::out_of_flow` and `record_out_of_flow` in node/node.rs (conflict surface), read by the layout pass and written by the style traversal
standard-contracts#9.4 · yes · `ElementData::classes` is written by `set_attribute` in mutator.rs (conflict surface) and read by Stylo's `has_class` in stylo.rs (merge surface)
standard-contracts#10.1 · yes · `ShellProvider` in blitz-traits/src/shell.rs (merge surface) carries the clipboard write and the IME default the Needs keep for the shell and windowing
standard-contracts#11.1 · no · no blitz-html or blitz-net file is reached, the scope names neither's parse, API or fetch behaviour, and Not needed drops blitz-net networking
