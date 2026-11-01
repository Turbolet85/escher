# Classified — a11y-plan · keyboard-navigation.b2

## Answers
keyboard-navigation#2.12 · yes · stale · the script runtime's no-change-synthesis reading sits in bootstrap.rs, a merge surface reached whole, and the report does not show it holds there
keyboard-navigation#2.13 · no · the range-input click focus is in events/pointer.rs, which the session does not reach
keyboard-navigation#3.1 · no · text-input key handling is in text.rs, which the session does not reach
keyboard-navigation#3.2 · no · range-input keys are in events/keyboard.rs, unreached; the disabled-attribute read is covered by the restyle-narrowing reading that the report shows holds
keyboard-navigation#3.3 · yes · add-to · F5 (a focused select whose box is dropped still moves its choice on keys, the limit) and its owner F6 (residuals.md line, deferred-from-0.3.0.md line, CARRY on chk-ajxm4sin) join this select key row
keyboard-navigation#3.4 · no · the open-list key cites select_popup.rs:202-230 and 249-273, both before the row-3 skip at line 513, so the cited lines stand
keyboard-navigation#3.5 · no · the open-list choose cites select_popup.rs:294-327 and 363-375, before line 513, and pointer.rs and keyboard.rs are unreached
keyboard-navigation#3.6 · yes · stale · the cited select_popup.rs:866-907 sits past line 513, where the row-3 skip was removed, so the span moves
keyboard-navigation#3.7 · yes · stale · the cited document.rs:2436-2438 is past line 45 in a merge surface with 290 lines added, so the span moves; the thumb claim holds
keyboard-navigation#3.8 · no · the multiple-select keys are in keyboard.rs and select.rs, unreached; the disabled-row reading is covered by the restyle-narrowing reading that the report shows holds
keyboard-navigation#3.9 · yes · stale · the cited select_popup.rs:991-1028 sits past line 513, where the row-3 skip was removed, so the span moves
keyboard-navigation#3.10 · no · the typeahead and Space rules are in keyboard.rs and select_interaction.rs, neither reached
keyboard-navigation#3.11 · no · the disabled-row rule is in keyboard.rs and select.rs, unreached; the report shows the disabled claims hold
keyboard-navigation#3.12 · no · the Shift-anchor rule is in select.rs and select_interaction.rs, neither reached
keyboard-navigation#3.13 · no · the optgroup rule is in select.rs, form.rs and select_interaction.rs, none reached
keyboard-navigation#3.14 · yes · stale · the cited mutator.rs:1334-1343 sits in a merge surface with 944 lines added from line 2 on, so the span moves
keyboard-navigation#3.15 · no · the driver's type is in escher-driver execute.rs and stand_act_select.rs, which the session does not reach
keyboard-navigation#3.16 · no · the driver's Ctrl limit is in escher-driver schema.rs, unreached, and its residuals owner is unchanged
keyboard-navigation#3.17 · yes · add-to · F4 (a key press with focus on a control whose anonymous block was dropped no longer panics) joins the focused-node Enter and Space rule; node.rs, where the Click builder lives, is a merge surface
keyboard-navigation#3.18 · no · the macOS Backspace rule is in text.rs, which the session does not reach
keyboard-navigation#3.19 · no · the copy rule is in keyboard.rs, which the session does not reach
keyboard-navigation#3.20 · no · implicit submission is in keyboard.rs and form.rs, neither reached
keyboard-navigation#3.21 · no · the label click rule is in events/pointer.rs, which the session does not reach
keyboard-navigation#3.22 · yes · stale · node.rs is a merge surface reached whole with hunks at 1382-1384 above the cited span 2028-2058, so the span may move
keyboard-navigation#3.23 · no · key conversion is in blitz-shell convert_events.rs, which the session does not reach
