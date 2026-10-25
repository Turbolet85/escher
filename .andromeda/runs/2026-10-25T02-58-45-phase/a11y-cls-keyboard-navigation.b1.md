# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusability rule the Needs state: disabled counts by presence on a form control alone, nothing inert or under an inert element is focusable, and HTML's focus fixup stays unbuilt (the stated limit); the CRUD rows reading not focusable follows from the same rule
keyboard-navigation#1.2 · yes · Tab and Shift+Tab walk the focus order that stand_keyboard's tab_order and ENABLED_TAB_ORDER proof measures (SC 2.4.3)
keyboard-navigation#1.3 · yes · focusability is recomputed when disabled is set or removed, which the stand's enabled-state Tab walk (ENABLED_TAB_ORDER) depends on when disabled is removed
keyboard-navigation#1.4 · yes · names the form controls disabled counts on (button, input, select, textarea) and that setting disabled removes a button's focusability, both inside the statement's focusability claim
keyboard-navigation#1.5 · yes · disabled-ness is keyed by presence on a button, input, select or textarea alone, the rule the statement's focusability and activation gates rest on
keyboard-navigation#1.6 · no · the file input's generated inner button is not in any stand task (none holds a file input) and no Need claims a file-input focus stop
keyboard-navigation#1.7 · no · autofocus sets an initial focus that neither the Needs nor any stand task names, so it reaches no claim the statement makes
keyboard-navigation#1.8 · yes · focusing sets the FOCUS element state on the engine's focus that the Tab walk moves and the focus agreement the Needs name rests on; the FOCUSRING half is the focus ring the statement leaves unclaimed
keyboard-navigation#1.9 · no · CSS focus pseudo-class matching and the never-matching :focus-visible are styling and the focus ring, which the statement neither claims nor measures
keyboard-navigation#1.10 · no · focus and blur bubbling is event semantics that no Need, stand proof or statement claim names
keyboard-navigation#2.1 · no · focus reset on removing a focused node and the IME side effect are outside the Needs; the focus limit the statement states is the inert case in #1.1
keyboard-navigation#2.2 · no · a click on non-interactive ground clearing focus is not in the Needs, which carry click focus on controls only
keyboard-navigation#2.3 · yes · a click on a button focuses it and Tab continues from it (tests/blitz-tests/tests/stand_keyboard.rs:575-602, part of the stand's keyboard proof), and a click on a CRUD row or the list still clears focus, the rule behind the CRUD reach
keyboard-navigation#2.4 · no · the double-click window on the virtual clock is a pointer-timing read that no Need or a11y reach names; the clock reaches the Needs only through the select's typeahead
keyboard-navigation#2.5 · no · toggling a details element through a summary's click is outside the stand's proof, which names buttons alone; the statement's Enter and Space claim names which elements activate, not what they toggle
keyboard-navigation#2.6 · no · checkbox and radio click toggling is in neither the Needs nor the button-only stand proof; the native checkbox limit the Needs carry is the tree's, not the click's
keyboard-navigation#2.7 · yes · the range input's pointer model, a click that focuses without moving the value and a pointerdown that sets it, is named in the Needs
keyboard-navigation#3.1 · no · text-input editing keys are the text plan's; the Needs name no text-input key and the statement claims none
keyboard-navigation#3.2 · yes · the range input's keyboard model (arrows, Home and End, one input event per change, no key acting while it carries disabled) is named as a capability in the Needs
keyboard-navigation#3.3 · yes · the single-choice select's keyboard model (arrows, Home and End, 1 s typeahead on the document's clock, input then change) and its stated limits (multiple, optgroup, no popup) are named in the Needs
keyboard-navigation#3.4 · yes · Enter and a released Space dispatch the click on the focused elements the Needs name for keyboard activation, gated against disabled form controls, and read the stand's buttons as their clicks (SC 2.1.1)
keyboard-navigation#3.5 · no · macOS Backspace routing to Apple keybindings is a platform text-editing detail the stand's tier-0 proof does not reach
keyboard-navigation#3.6 · no · copy by the action modifier with no text input focused is a clipboard shortcut, not a tier-0 keyboard claim or a stand proof
keyboard-navigation#3.7 · no · a text input's implicit form submission is not an activation of a focused element in the Needs, and no stand proof or statement claim names it
keyboard-navigation#3.8 · no · a label's click running its control's default click is not in the Needs, which carry a label only as a name source
