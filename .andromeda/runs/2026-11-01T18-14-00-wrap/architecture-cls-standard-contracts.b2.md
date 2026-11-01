# Classified — architecture · standard-contracts.b2

## Answers
standard-contracts#8.1 · yes · add-to · existing attributes now replay on `set_custom_widget`, so the Widget trait's `attribute_changed` row carries the replay (custom_widget.rs is reached)
standard-contracts#9.1 · yes · stale · element.rs is a merge surface and `is_disabled` at element.rs:516-520 can have moved
standard-contracts#9.2 · no · `row_disabled` lives in node/select_list.rs, which the session did not reach
standard-contracts#9.3 · no · the select model in node/select.rs and node/mod.rs is untouched and not reached
standard-contracts#9.4 · yes · stale · document.rs is a merge surface and holds the `select_cursor` field this entry names, so it can have moved
standard-contracts#9.5 · yes · stale · lib.rs is reached and the re-export cited at lines 106-109 can have moved
standard-contracts#9.6 · yes · stale · select_popup.rs is reached and the test cited at lines 1101-1118 can have moved
standard-contracts#9.7 · no · the list sizing at node/select_list.rs:218-263 is not reached
standard-contracts#9.8 · yes · add-to · F18 Node's stacking-context arm reads a mix-blend-mode other than normal, and F20 adds the NodeFlags bit and public reader `is_inline_with_only_block_children`, both in node.rs (merge surface)
standard-contracts#9.9 · yes · add-to · F12 TElement::borrow_data filters data with no primary style, beside the has_class and each_class reads in stylo.rs (merge surface)
standard-contracts#9.10 · yes · stale · the set_attribute, clear_attribute and stylo.rs citations sit in element.rs, mutator.rs and stylo.rs, all merge surfaces, so their lines can have moved
standard-contracts#10.1 · yes · add-to · F13 NetHandler::error stands beside bytes as a default no-op, and F14 the stylesheet handler's error responds Err, both in blitz-traits net.rs (merge surface)
standard-contracts#11.1 · yes · stale · blitz-net lib.rs is a merge surface, its error arm now calls the handler's error with the waker moved after it (F13), and its cited lines have moved
standard-contracts#12.1 · yes · add-to · F16 render_element pushes and pops a viewport-wide blend layer, with paint_select_list still painting last, in render.rs and lib.rs (merge surfaces)
standard-contracts#12.2 · no · the body of paint_select_list in render/select_list.rs is not reached
standard-contracts#12.3 · no · text.rs is not reached and the embolden feature line does not move
standard-contracts#13.1 · no · blitz-shell is not reached
standard-contracts#14.1 · yes · add-to · F19 text_contrast's None list gains a mix-blend-mode element on the walk, and F15 the CountedHandler forwards error so a failed load ends the settle wait (contrast.rs and settle.rs are reached)
standard-contracts#15.1 · no · harness.rs, where the harness clock and pump live, is not reached
standard-contracts#16.1 · no · examples/seven_guis holds no diff
standard-contracts#17.1 · no · examples/seven_guis holds no diff and the bundled font is untouched
standard-contracts#18.1 · no · examples/seven_guis holds no diff
standard-contracts#18.2 · no · examples/seven_guis holds no diff
standard-contracts#18.3 · no · examples/seven_guis holds no diff
standard-contracts#19.1 · no · packages/escher-telemetry holds no diff
