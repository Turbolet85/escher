# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusability rule sits in node/element.rs (merge surface) and node/node.rs (conflict surface), which the Needs keep so inert and disabled controls stay unfocusable under stand_keyboard's Tab tables
keyboard-navigation#1.2 · yes · focus_next_node and focus_prev_node sit in document.rs (merge surface), the Tab walk the stand_keyboard Tab-order tables read
keyboard-navigation#1.3 · yes · the cached-focusability recompute on tabindex, href and disabled changes sits in mutator.rs (conflict surface), beside the tabindex rules the Needs keep unchanged
keyboard-navigation#1.4 · yes · disabled form controls toggling their states and losing focusability sit in node/element.rs and node/node.rs (merge and conflict surfaces)
keyboard-navigation#1.5 · yes · the presence reading of disabled sits in node/element.rs and events/pointer.rs (merge surfaces), read by the focusability, click and activation gates
keyboard-navigation#1.6 · yes · the file input's generated tabindex of -1 sits in mutator.rs (conflict surface), a tabindex element rule the Needs keep unchanged
keyboard-navigation#1.7 · yes · the autofocus focus-on-flush rule sits in mutator.rs (conflict surface), a focus-order rule the conflict resolution there must keep
keyboard-navigation#1.8 · yes · focusing and blurring set and clear FOCUS and FOCUSRING in document.rs and node/node.rs (merge and conflict surfaces)
keyboard-navigation#1.9 · yes · :focus, :focus-within and :focus-visible matching sits in stylo.rs (merge surface), which 53e9d81c6's :open pseudo-class also reaches
keyboard-navigation#1.10 · yes · focus and blur not bubbling, and focusin and focusout bubbling, sit in blitz-traits events.rs (merge surface)
keyboard-navigation#2.1 · yes · focus reset on removing the focused node and its blur and IME teardown sit in document.rs (merge surface)
keyboard-navigation#2.2 · yes · clearing focus on a click on a non-interactive area sits in events/pointer.rs (merge surface)
keyboard-navigation#2.3 · yes · a button, input or select click focusing it and submitting its form owner sits in events/pointer.rs, the focus-restoration-on-click rule the Needs name
keyboard-navigation#2.4 · yes · the double-click window read on the document's clock sits in events/pointer.rs, which 0980cce5f edits and the Needs name
keyboard-navigation#2.5 · yes · summary activation toggling and focusing sits in events/pointer.rs (merge surface), beside the details :open reach the Needs name
keyboard-navigation#2.6 · yes · checkbox and radio click toggling and focus sit in events/pointer.rs (merge surface)
keyboard-navigation#2.7 · yes · a range click focusing without moving its value sits in events/pointer.rs, the range-click rule the Needs name
keyboard-navigation#3.1 · yes · text-input keys, copy, cut, paste and select-all included, sit in node/text.rs (merge surface), and the Needs name the clipboard move that reaches them
keyboard-navigation#3.2 · yes · the range input's keys sit in keyboard.rs (not reached), which the Needs keep through the merges into driver.rs, pointer.rs and mutator.rs's range arm
keyboard-navigation#3.3 · yes · the select's arrows, Home and End and typeahead sit in keyboard.rs and select.rs (not reached), which the Needs keep through mutator.rs's select arm and the chunk's select model
keyboard-navigation#3.4 · yes · Enter and Space activation sits in keyboard.rs (not reached), which the Needs name and stand_keyboard's SC 2.1.1 rows re-prove
keyboard-navigation#3.5 · yes · the macOS Backspace mapping sits in node/text.rs (merge surface), so a merge can move it
keyboard-navigation#3.6 · yes · copy with no text input focused sits in keyboard.rs (not reached), and the clipboard need names copy only in a focused field, so this reading is a fork over it
keyboard-navigation#3.7 · yes · the Enter-driven implicit submit of a text input's form sits in keyboard.rs (not reached), under the Enter activation the Needs keep, a reading the scope leaves open and so a fork over it
keyboard-navigation#3.8 · yes · a label's default click that focuses its select sits in events/pointer.rs (merge surface)
