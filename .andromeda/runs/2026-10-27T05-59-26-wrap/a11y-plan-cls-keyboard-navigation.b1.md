# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · the focusability rule sits in blitz-dom's element.rs and node.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#1.2 · no · Tab and Shift+Tab walk through blitz-dom's keyboard.rs and document.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#1.3 · no · recomputed focusability after a tabindex, href or disabled write sits in blitz-dom's mutator.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#1.4 · no · a form control's disabled state sits in blitz-dom's element.rs and node.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#1.5 · no · disabled-by-presence reads sit in blitz-dom's element.rs, pointer.rs and keyboard.rs, none a reached file, so the documents-only change does not reach it
keyboard-navigation#1.6 · no · the file input's generated inner button tabindex sits in blitz-dom's mutator.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#1.7 · no · the autofocus rule sits in blitz-dom's mutator.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#1.8 · no · focus and blur element states sit in blitz-dom's document.rs and node.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#1.9 · no · :focus and :focus-within matching sits in blitz-dom's stylo.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#1.10 · no · focus and focusin bubbling sits in blitz-traits' events.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.1 · no · focus reset on removing the focused node sits in blitz-dom's document.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.2 · no · clearing focus on a non-interactive click sits in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.3 · no · the button and select click-focus rules sit in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.4 · no · the double-click window sits in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.5 · no · the summary toggle on click sits in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.6 · no · checkbox and radio click toggling sits in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#2.7 · no · a range input's click focus and pointerdown value sit in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.1 · no · text-input arrow, Home, End and edit keys sit in blitz-dom's node/text.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.2 · no · a range input's keys sit in blitz-dom's keyboard.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.3 · no · a select's keys sit in blitz-dom's keyboard.rs and node/select.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#3.4 · no · Enter and Space activation sits in blitz-dom's keyboard.rs and events/mod.rs, neither a reached file, so the documents-only change does not reach it
keyboard-navigation#3.5 · no · the macOS Backspace keybinding sits in blitz-dom's node/text.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.6 · no · copy with the action modifier and no text input focused sits in blitz-dom's keyboard.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.7 · no · implicit form submission sits in blitz-dom's keyboard.rs, not a reached file, so the documents-only change does not reach it
keyboard-navigation#3.8 · no · a label's click delegation sits in blitz-dom's pointer.rs, not a reached file, so the documents-only change does not reach it
