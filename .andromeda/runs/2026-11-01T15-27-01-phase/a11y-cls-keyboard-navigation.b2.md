# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#2.12 · no · the checkbox and radio change dispatch in pointer.rs is named by no scope line, and the chunk's stand checks do not read it
keyboard-navigation#2.13 · yes · the range input's pointerdown arm in pointer.rs is the range input arm the scope names beside the list's click path, which sits in the same file (click_list_row)
keyboard-navigation#3.1 · no · text-input key handling in node/text.rs is named by no scope line; the stand checks reach text only through the driver's type
keyboard-navigation#3.2 · yes · the keyboard range arm the scope names beside the select keys: arrows, Home and End, the value held apart from the attribute, and the changed-set mark with one input event
keyboard-navigation#3.3 · yes · the single select's keys, the typeahead on the document clock's 1 s window, Enter and Space opening the list, and one input then change, all named by the select model, its list and the clock invariant
keyboard-navigation#3.4 · yes · the open list's own key match, press_open_list_key, the list's keys the scope names; select_popup asserts its rows
keyboard-navigation#3.5 · yes · the list's Enter, Space, Tab and click choice through choose_current_and_close, click_list_row and apply_select_choice, the list's close paths the scope names
keyboard-navigation#3.6 · yes · the list's scroll_y write in scrolling.rs, a predicted conflict surface, which scroll_into_view and the page scroll that closes a select list use
keyboard-navigation#3.7 · no · the painted thumb's no-hit and no-drag rule lives in document.rs, outside the reached files and the scope; the list's key, wheel and scroll moves bind through the entries above
keyboard-navigation#3.8 · yes · the select model's multiple-select arrow, Home and End rules, with Shift spanning and Ctrl moving, in keyboard.rs and select.rs beside the list's keys in the select list's merge surface
keyboard-navigation#3.9 · yes · a multiple select's key brings the current row into view through scroll_into_view, which the scope names for the list
keyboard-navigation#3.10 · yes · the multiple select's Space flip and join and its character match, the select model's key rules the scope names
keyboard-navigation#3.11 · yes · disabled rows never chosen on a multiple select, the select model's choice rule in select.rs that the list's rows read
keyboard-navigation#3.12 · yes · Shift with no cursor spans from the last chosen row, the select model's seeded-set rule in select.rs
keyboard-navigation#3.13 · yes · optgroup options in tree order and option_disabled, which the list's key walk and the disabled rows it paints read
keyboard-navigation#3.14 · yes · record_select_of_multiple_write in mutator.rs, a predicted conflict surface, is the select model's write path the scope names
keyboard-navigation#3.15 · yes · the driver's type focusing a select, closing its list and clearing typeahead, the list's close path and focus the scope names; stand_act_select sits beside the stand checks
keyboard-navigation#3.16 · no · the driver's press holds no Ctrl, a limit the driver help states and residuals.md owns with target next; the chunk changes no driver code and needs no press with Ctrl
keyboard-navigation#3.17 · yes · Enter and Space on a focused button dispatch its click, the tier-0 activation target, and the stand_keyboard rows the chunk re-reads
keyboard-navigation#3.18 · no · macOS Backspace in node/text.rs is named by no scope line and no reached file
keyboard-navigation#3.19 · no · copy with no text input focused in keyboard.rs is no select list row and names no scope line
keyboard-navigation#3.20 · no · implicit form submission from a text input is not named by the scope; the editor's commit is named, but not this mechanism
keyboard-navigation#3.21 · yes · a label click on a single select focuses it and opens no list, the list's open and focus path the scope names, asserted in pointer.rs beside select_popup
keyboard-navigation#3.22 · yes · synthetic_click_event in node.rs, a predicted conflict surface, is the click that Enter and Space activation builds on
keyboard-navigation#3.23 · no · winit key conversion in blitz-shell is outside the reached files, and the stand checks drive keyboard events without winit, so no check reads it
