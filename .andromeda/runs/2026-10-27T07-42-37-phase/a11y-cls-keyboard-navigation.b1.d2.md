# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusability gates (inert, disabled form control, tabindex, native-focusable) in element.rs and node.rs, which the take's conflict resolutions keep unfocusable
keyboard-navigation#1.2 · yes · the Tab and Shift+Tab walk in document.rs (merge surface), the order stand_keyboard's Tab tables read
keyboard-navigation#1.3 · yes · cached focusability recomputed on tabindex, href and disabled in mutator.rs (conflict surface, reached whole)
keyboard-navigation#1.4 · yes · disabled form controls, their DISABLED and ENABLED states and their removed focusability in element.rs and node.rs (merge and conflict surfaces)
keyboard-navigation#1.5 · yes · the presence-keyed is_disabled reading in element.rs and pointer.rs (merge surfaces), read by focusability and the click and key gates
keyboard-navigation#1.6 · yes · the file input's inner button tabindex=-1 in mutator.rs (conflict surface, reached whole)
keyboard-navigation#1.7 · yes · autofocus of the latest mounted focusable node in mutator.rs (conflict surface, reached whole)
keyboard-navigation#1.8 · yes · FOCUS and FOCUSRING states set and cleared in document.rs and node.rs (merge and conflict surfaces), the focus the tree and snapshot read
keyboard-navigation#1.9 · yes · the :focus and :focus-within matching in stylo.rs (merge surface)
keyboard-navigation#1.10 · yes · the focus and blur non-bubbling flags in blitz-traits events.rs (merge surface)
keyboard-navigation#2.1 · yes · focus reset and blur side effects on removal in document.rs (merge surface), with the IME disable on a removed text input
keyboard-navigation#2.2 · yes · the unmatched-click focus clear in pointer.rs (merge surface), inside the click-focus Need
keyboard-navigation#2.3 · yes · button and select click focus and form submit in pointer.rs, named by the Focus restoration on click Need
keyboard-navigation#2.4 · yes · the double-click window read on the document clock in pointer.rs, the file 0980cce5f edits and the Focus restoration Need names
keyboard-navigation#2.5 · yes · the first summary's toggle and focus in pointer.rs (merge surface)
keyboard-navigation#2.6 · yes · checkbox and radio click toggling and focus in pointer.rs (merge surface)
keyboard-navigation#2.7 · yes · a range click focusing with no value move in pointer.rs, named by the Focus restoration Need
keyboard-navigation#3.1 · yes · text-input keys and copy, cut, paste and select-all in node/text.rs (merge surface), inside the clipboard-keys Need
keyboard-navigation#3.2 · yes · the range input's arrow, Home and End keys, named by the Keyboard handling Need
keyboard-navigation#3.3 · yes · the single-choice select's keys and typeahead, named by the Keyboard handling Need
keyboard-navigation#3.4 · yes · Enter and Space activation, named by the Keyboard handling Need and re-asserted by stand_keyboard's key rows
keyboard-navigation#3.5 · yes · the macOS Backspace mapping in node/text.rs (merge surface)
keyboard-navigation#3.6 · yes · copy with no field focused, its clipboard write moved by 527394724 to winit's API under the default features, the carrier the clipboard Need names
keyboard-navigation#3.7 · yes · Enter's implicit form submission in keyboard.rs (not reached), inside the Enter activation the Keyboard handling Need names
keyboard-navigation#3.8 · yes · the label click's default action in pointer.rs (merge surface), the click focus the Focus restoration Need names
