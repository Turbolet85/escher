# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · its inert presence reading (Node::is_inert) takes an inert node out of the snapshot the lint walks, so the helpers leave that exclusion to the snapshot; its Tab walk is not reached
keyboard-navigation#1.2 · no · Tab and Shift+Tab focus movement; the lint and its listing read no focus order
keyboard-navigation#1.3 · no · cached focusability recompute on tabindex, href or disabled change; the lint reads no focusability
keyboard-navigation#1.4 · no · the DISABLED and ENABLED element-state toggle and button focusability; the lint reads enabled from the snapshot, which reads the disabled attribute, not these states
keyboard-navigation#1.5 · yes · the disabled presence the snapshot's enabled carries, which the lint's contrast exempt rule (enabled Some(false)) and stand_contrast keep, exempting the CRUD buttons disabled at boot
keyboard-navigation#1.6 · no · a file input's generated inner button; no stand task has a file input and file-input masking is Not needed
keyboard-navigation#1.7 · no · autofocus focusing on flush; the lint reads role, name, bounds and enabled from the snapshot, never a focused state
keyboard-navigation#1.8 · no · FOCUS and FOCUSRING states on focus and blur; the lint reads role, name, bounds and enabled from the snapshot, never a focused state
keyboard-navigation#1.9 · no · :focus and :focus-within matching; the lint reads role, name, bounds and enabled from the snapshot, never a focused state
keyboard-navigation#1.10 · no · focus and blur not bubbling; the lint reads no event
keyboard-navigation#2.1 · no · focus reset when the focused node is removed; Not needed (focus restoration and interaction state on node removal, clear_interaction_state_for_removed_node untouched)
keyboard-navigation#2.2 · no · a click on a non-interactive area clearing focus; the chunk reads no focus state
keyboard-navigation#2.3 · no · click focus and the covered refusal of a select option; the flight booker overlap reads the options' bounds from the snapshot, not a click
keyboard-navigation#2.4 · no · double-click detection on the document clock; each click still focuses its target as one click does, and the count reads nothing the lint or listing asserts
keyboard-navigation#2.5 · no · summary toggling of details on activation; no lint input or listing row names an open state
keyboard-navigation#2.6 · no · checkbox and radio click toggling; no checked state is a lint input
keyboard-navigation#2.7 · no · a range input's click focus and pointerdown value; the timer is read at boot and its value is no lint input
keyboard-navigation#3.1 · no · text-input key editing (arrows, Home and End, Delete and Backspace, clipboard); the scope names no such key press
keyboard-navigation#3.2 · no · range input arrow, Home and End keys; the scope names no key press and the range value is no lint input
keyboard-navigation#3.3 · no · select ArrowDown, ArrowUp, Home, End and typeahead; the flight booker overlap reads the options' bounds from the snapshot, not keys or a popup
keyboard-navigation#3.4 · no · Enter and Space activation of buttons, Not needed (§5); the scope names no key press, and its witnesses sit in the a11y leg's green run, which reads no lint input
keyboard-navigation#3.5 · no · macOS Backspace left to Apple keybindings, a platform keybinding the chunk does not reach
keyboard-navigation#3.6 · no · action-modifier C copy with no text input focused; the lint and listing read no clipboard
keyboard-navigation#3.7 · no · implicit form submission from a text input's generated submit; the scope names no key press or form submit
keyboard-navigation#3.8 · no · a label's click running its labelable element's default click; the scope names no label click
