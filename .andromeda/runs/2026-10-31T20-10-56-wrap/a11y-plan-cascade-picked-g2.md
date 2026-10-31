# a11y-plan · picked accounting · group 2 of 5

## Picked
keyboard-navigation#1.2 · not touched · `mutator.rs` is outside the reached files, so the focus fixup holds; a select made inert closing its list is a different path, carried by F6 beside #1.6
keyboard-navigation#1.3 · proposed · Tab and Shift+Tab still move focus as stated, and F4 and F7 add what they do with a list open beside it
keyboard-navigation#1.6 · proposed · the one disabled-ness key holds, and F6 adds what a disabled, inert or hidden select and a disabled row do with the list beside it
keyboard-navigation#1.9 · not touched · the report's `document.rs` additions write no focus state and `node.rs` is unchanged against the base, so the claim holds
keyboard-navigation#1.10 · not touched · `stylo.rs`'s one added block is the `size` hint at 1124-1139, so the three focus pseudo-classes hold
keyboard-navigation#2.1 · not touched · the removal arm gains only the clearing of the list's field at `document.rs:1408-1410`, so the reset to the body holds; F6 carries the list's close
keyboard-navigation#2.2 · proposed · the clear of focus holds, and F5 adds beside it that a click outside and the other closers close an open list
keyboard-navigation#2.3 · proposed · a click on a single select now also opens or closes its list and an open list's row takes a click, and F8 adds the pointer's highlight and press beside it
keyboard-navigation#2.4 · not touched · the report states `click_select_row` keeps its semantics; the `option` arm's new call serves only a row of an open list
keyboard-navigation#2.5 · not touched · `two_clicks_on_one_point_open_and_close_each_focusing` confirms each click still focuses, and the report moves no double-click rule
keyboard-navigation#2.6 · not touched · the report's `handle_click` edits are the `select`, `option`, `optgroup` and `label` arms, so the `summary` arm holds
keyboard-navigation#2.7 · not touched · the report states `:open` and `flush_open_state` are unmoved, and this row names `details` and `dialog` alone
keyboard-navigation#2.8 · not touched · the report's `handle_click` edits leave the checkbox and radio arms, so toggling and focusing hold
keyboard-navigation#2.9 · not touched · the report's `handle_click` edits leave the checkable arms and `checkable_change_event` is outside the reached files, so one `change` holds
keyboard-navigation#2.10 · not touched · `handle_pointerdown` gains only the list's return and `close_list_outside`, and no range arm is edited, so the click and press rules hold
keyboard-navigation#3.2 · not touched · the report's `keyboard.rs` edits are Tab's list step and the select and `multiple` select keys, so the range keys hold
keyboard-navigation#3.3 · proposed · Enter and Space now open the list and the painted list exists, so «open nothing» and «no popup is painted» are retired, and F2, F3, F9 and F10 stand beside it
keyboard-navigation#3.4 · proposed · `press_multiple_select_key` keeps its semantics, and F11 adds the row brought into view on a valid `size` beside it
