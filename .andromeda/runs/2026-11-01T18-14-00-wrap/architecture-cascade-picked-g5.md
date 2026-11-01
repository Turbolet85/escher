# architecture · picked accounting · group 5 of 10

## Picked
standard-contracts#1.1 · proposed · line 9 lists `NetHandler::bytes` alone per resource kind; the report gives the stylesheet's handler an `error` that drops the pending request
standard-contracts#2.1 · not touched · only `blitz-dom/src/lib.rs`'s comment left, `pub use clock` beneath unmoved, and the wall-clock census reads 0, so the clock's claims hold
standard-contracts#2.2 · not touched · the report rules `snapshot_node_and` another symbol, no-change, and `document.rs`'s new text holds no row in the select readers or writers
standard-contracts#2.3 · not touched · `document.rs` is a merge surface but its new text holds no row in the list's machinery, and the select list's surface was re-read as recorded
standard-contracts#2.4 · not touched · `live_select_list` and its two reads past it hold no row of `document.rs`'s new text, and `select_popup` stays green with only its row-3 skip removed
standard-contracts#2.5 · not touched · `resolve.rs` is not reached and `clear_interaction_state_for_removed_node`'s select arm holds no row of `document.rs`'s new text
standard-contracts#2.6 · not touched · these select readers hold no row of `document.rs`'s new text, and the snapshot and `lib.rs` of dioxus-native-dom are not reached
standard-contracts#2.7 · not touched · the take narrows the mutator's restyle only; no new-text row falls on `record_select_of_multiple_write`, and `select_popup` stays green
standard-contracts#3.1 · proposed · line 28's `resolved_style_value` now reads `width` and `height` sub-pixel, and the mutation contract owes the restyle helpers a line
standard-contracts#4.3 · not touched · `pointer.rs` is not reached and `select_popup.rs` added line 513 alone, outside this test, which stays green
standard-contracts#4.4 · not touched · `pointer.rs` is not reached, and the bootstrap's one new script adds `URLSearchParams`, no synthesized `change`
standard-contracts#5.1 · proposed · the report rules line 57 no-change, but the Stylo readers on line 58 sit beside the new `borrow_data` filter and scoped context
standard-contracts#6.1 · not touched · `document.rs`'s new text holds no row in the client-rect readers, and `e42ee1f25`'s scaled bounds are `render_element`'s alone
standard-contracts#6.2 · not touched · `scrolling.rs` and `lint.rs` are not reached, so `visible_region` and its callers read as stated
standard-contracts#6.3 · not touched · `text_extent.rs` is not reached and `packages/escher-driver` holds no diff, so its one non-test caller stands
standard-contracts#6.4 · not touched · `node.rs`'s new text holds no row in `hit_inner`, whose merge partner `0980cce5f` stays untaken among the left-out sixteen
standard-contracts#6.5 · not touched · `hit_inner`'s transform arm holds no row of `node.rs`'s new text, so its answer stands
standard-contracts#7.1 · proposed · `StackingContext` is listed here and `Node`'s stacking-context arm now reads a `mix-blend-mode` other than `normal`
standard-contracts#8.1 · proposed · line 82's `Widget` trait owes the replay of existing attributes through `attribute_changed` on `set_custom_widget`
standard-contracts#9.1 · not touched · `element.rs`'s new text falls at 902-906 alone, outside `is_disabled`, so the one reading stands
standard-contracts#9.4 · not touched · `node::select` is not reached and `select_cursor` holds no row of `document.rs`'s new text
standard-contracts#9.5 · not touched · `node::select_list` is not reached, the report meets `SelectListView`'s constants as recorded, and `lib.rs` lost only a comment
standard-contracts#9.6 · not touched · `SelectList.fresh` lives in an unreached module and its test falls outside `select_popup.rs`'s one added line, green
standard-contracts#9.8 · proposed · `Node::out_of_flow`'s claim stands, and the new flag and public reader `is_inline_with_only_block_children` belong beside it
standard-contracts#9.9 · not touched · no new-text row falls on `classes`, `parse_classes`, the `class` arm or `has_class`, and the restyle narrowing is proposed at the mutation contract
