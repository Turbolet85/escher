# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#2.12 · no · the checkbox and radio click's Change dispatch lives in pointer.rs and checkable_change_event.rs, neither reached nor named by the scope
keyboard-navigation#2.13 · no · the pointer-side range click focus and value set lives in pointer.rs, unreached; the scope names only the keyboard range arm beside the select's
keyboard-navigation#3.1 · no · text-input key handling lives in text.rs, unreached and unnamed; the controlled key-press run reads the node.rs:1775 panic, not these keys
keyboard-navigation#3.2 · yes · the keyboard range arm (keyboard.rs:106-111) is the range input's arm the scope keeps beside the select model and its list
keyboard-navigation#3.3 · yes · the single select's keys, Enter and Space opening the list, and the 1 s typeahead on the document's clock, which the clock invariant names, all lie in the select model the scope re-proves
keyboard-navigation#3.4 · yes · the open list's closed key match (press_open_list_key) is the list's keys the scope re-reads row by row
keyboard-navigation#3.5 · yes · choosing from the open list (Enter, Space, Tab, click) through apply_select_choice is the list's clicks and write path, which the mutator.rs conflict can move
keyboard-navigation#3.6 · yes · the list's key and wheel scroll writes scroll_y at scrolling.rs:669-672, a predicted conflict surface, with the scrolled-box bounds
keyboard-navigation#3.7 · yes · the list's painted, hit-less thumb is part of the select list's merge surface re-read row by row, and select_popup_painted measures it at the default sheet
keyboard-navigation#3.8 · yes · a multiple select's keys choose rows through the select's writes, which the mutator.rs conflict can move
keyboard-navigation#3.9 · yes · the multiple select's key-driven scroll calls scroll_into_view, a call the scope names through the scrolling.rs conflict
keyboard-navigation#3.10 · yes · multiple select Space flips and joins rows, and typeahead runs on the document clock, both in the select model the scope keeps unchanged
keyboard-navigation#3.11 · yes · disabled rows are never chosen on a multiple select, a rule of the select model's writes through mutator.rs
keyboard-navigation#3.12 · yes · Shift spanning from the last chosen row is the multiple select's model, seeded from markup and written through mutator.rs
keyboard-navigation#3.13 · yes · optgroup options in tree order, reached by keys and text, and option_disabled are the select model and the list's rows the scope re-reads
keyboard-navigation#3.14 · yes · record_select_of_multiple_write at mutator.rs:1262-1271 is a predicted conflict line
keyboard-navigation#3.15 · yes · the driver's type on a select (focus, closed list, cleared typeahead) is asserted by stand_act_select, which the scope names beside stand_keyboard
keyboard-navigation#3.16 · no · the driver's press holds no Ctrl (schema.rs, unreached); the residuals line it names is not one of the two units the take edits
keyboard-navigation#3.17 · yes · Enter and Space activating stand buttons is the tier-0 activation the standing checks re-prove through stand_keyboard and stand_act_keys
keyboard-navigation#3.18 · no · macOS Backspace is left to Apple's standard bindings in text.rs, unreached; no scope item names it
keyboard-navigation#3.19 · no · copy with no text input focused (keyboard.rs:50-68) is unreached and unnamed by the scope
keyboard-navigation#3.20 · no · implicit form submission from a text input (keyboard.rs:556-592) is unreached; the tier-0 Enter proof covers buttons, not this path
keyboard-navigation#3.21 · yes · a label click focusing a single select and opening nothing is the list's focus and open path, asserted by select_popup, which the scope names
keyboard-navigation#3.22 · yes · synthetic_click_event at node.rs:2015-2045 lies in node.rs, a predicted conflict surface, and builds the click that Enter and Space dispatch
keyboard-navigation#3.23 · no · winit key conversion lives in blitz-shell, unreached; a moved winit pin binds only on stand-asserted event values, and this rule names none
