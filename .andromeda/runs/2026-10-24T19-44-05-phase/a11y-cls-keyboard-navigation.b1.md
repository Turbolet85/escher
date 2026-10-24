# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusability rule that sets each lean task's Tab order at boot and in the enabled states, which stand_keyboard re-asserts
keyboard-navigation#1.2 · yes · Tab and Shift+Tab moving focus to the next and previous focusable node is the walk stand_keyboard drives in both layout modes
keyboard-navigation#1.3 · yes · focusability recomputed when disabled is set or removed is what the enabled-state Tab orders rest on, a falsy disabled from the Dioxus bridge removing the attribute
keyboard-navigation#1.4 · yes · a button with disabled set losing its focusability is what keeps the disabled stand buttons out of the boot Tab order
keyboard-navigation#1.5 · yes · disabled-ness read by presence on a form control, for :disabled, the pointer gates, the activation keys and focusability, is a Needs bullet
keyboard-navigation#1.6 · no · the generated inner button of a file input, which no Tab order the chunk asserts reaches, since the Needs name no file input on a lean stand task
keyboard-navigation#1.7 · no · autofocus focus on flush, which no Needs bullet names, since the boot Tab order rests on focusability alone
keyboard-navigation#1.8 · no · the FOCUS and FOCUSRING element states, since the chunk reads focus as the engine's focus, the snapshot's focused node and the tree's focus, and measures no focus indicator
keyboard-navigation#1.9 · no · :focus, :focus-within and :focus-visible matching, since the contrast state rows name hover and active, not focus, and the chunk changes no focus indicator
keyboard-navigation#1.10 · no · focus and blur not bubbling against focusin and focusout bubbling, since the chunk asserts the focused node and not the propagation of focus events
keyboard-navigation#2.1 · no · focus reset to body on removal of a focused node and the IME teardown, which no Needs bullet names
keyboard-navigation#2.2 · yes · a click on a non-interactive area clearing focus is what keeps a CRUD row click clearing focus, a Needs bullet
keyboard-navigation#2.3 · yes · a pointer click focusing a button, a submit, reset or button input, or a select is a Needs bullet, asserted at stand_keyboard.rs:575-602
keyboard-navigation#2.4 · yes · the double-click window read on the document's clock is a Needs bullet
keyboard-navigation#2.5 · no · a summary's pointer toggle and focus, since no stand control is named as a details summary and the Enter and Space re-read asserts stand buttons only
keyboard-navigation#2.6 · no · checkbox and radio clicks toggling and moving focus, since the Needs' checkbox carve-out concerns the builder's reading of checkedness, not the click, and the activation re-read asserts buttons only
keyboard-navigation#2.7 · yes · a click focusing a range input without moving its value, with the pointerdown that sets it, is the Timer slider's pointer model bullet
keyboard-navigation#3.1 · no · text-input editing keys (arrows, Home and End, Delete and Backspace, clipboard, word movement), which no Needs bullet names
keyboard-navigation#3.2 · yes · the arrows stepping a focused range input, and Home and End taking it to its bounds, is the Timer slider's keyboard model bullet
keyboard-navigation#3.3 · yes · the single-choice select's arrow, Home, End and typeahead keys, with one input then one change per change, is the select bullet, which names the flight booker's select as the new stand_keyboard row
keyboard-navigation#3.4 · yes · Enter and Space dispatching a Click on a focused stand button, a disabled form control excluded, is a Needs bullet asserted at stand_keyboard.rs:154-210, 281-286 and 302-333
keyboard-navigation#3.5 · no · macOS Backspace left to the Apple keybindings, which no Needs bullet names
keyboard-navigation#3.6 · no · copy of a selection with the action modifier when no text input is focused, which no Needs bullet names
keyboard-navigation#3.7 · no · implicit form submission from a text input, which no Needs bullet names
keyboard-navigation#3.8 · no · a label click running its bound element's default click, since the Needs assert a select's own click focus, not a label's
