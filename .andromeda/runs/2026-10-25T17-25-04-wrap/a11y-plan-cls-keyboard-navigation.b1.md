# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · stale · its cited test stand_keyboard.rs:677-718 sits in a reached file the session re-pinned, so the citation can have moved while the engine focusability rule stands
keyboard-navigation#1.2 · no · the Tab walk in keyboard.rs and document.rs is an engine rule the session did not change, and the entity cites no reached file
keyboard-navigation#1.3 · no · cached focusability and roving tabindex live in mutator.rs and focusability_updates.rs, neither reached nor changed
keyboard-navigation#1.4 · no · the disabled toggles live in element.rs, node.rs and focusability_updates.rs, none reached nor changed
keyboard-navigation#1.5 · yes · stale · its cited test stand_keyboard.rs:677-718 sits in a reached file the session re-pinned, so the citation can have moved while the engine presence rule stands
keyboard-navigation#1.6 · no · the file input's inner button tabindex is set in mutator.rs, which the session does not reach
keyboard-navigation#1.7 · no · autofocus lives in mutator.rs and is pinned by autofocus_attribute.rs, neither reached nor changed
keyboard-navigation#1.8 · no · the FOCUS and FOCUSRING states are set in document.rs and node.rs, which the session does not change
keyboard-navigation#1.9 · no · :focus and :focus-within matching in stylo.rs is unchanged, and the select's focus-ring drop is a visual fact this entity does not state
keyboard-navigation#1.10 · no · focus and blur not bubbling is an event-type rule in blitz-traits events.rs, which the session does not change
keyboard-navigation#2.1 · no · focus reset on removing the focused node (document.rs, interaction_state_teardown.rs) is an engine rule the session did not change, and the entity cites no reached file
keyboard-navigation#2.2 · no · the focus-clearing walk (pointer.rs:969-972) is unchanged, and the shown-scrollbar pointerdown the session measured is a visual-design fact this entity does not state
keyboard-navigation#2.3 · yes · stale · its stand claim cites stand_keyboard.rs:638-665, a reached file the session re-pinned, and the stand's buttons lost the two retired flight mode buttons
keyboard-navigation#2.4 · no · the double-click window on the document clock (pointer.rs, stand_virtual_clock.rs) is unchanged, and the entity cites no reached file
keyboard-navigation#2.5 · no · summary toggling in pointer.rs is unchanged, and the entity cites no reached file
keyboard-navigation#2.6 · no · checkbox and radio click behaviour (pointer.rs, harness_smoke.rs) is unchanged, and the session names no checkbox or radio
keyboard-navigation#2.7 · no · a click on a range input (pointer.rs) is unchanged, and the session names no range input
keyboard-navigation#3.1 · no · text-input key handling in text.rs is unchanged, and the entity cites no reached file
keyboard-navigation#3.2 · no · range-input keys in keyboard.rs are unchanged, and the session names no range input
keyboard-navigation#3.3 · yes · add-to · F1 adds that the flight booker's flight type is a focusable single-choice select on the stand whose arrows move its choice in both layout modes, pinned by stand_keyboard, so the select model is exercised on a lean stand task
keyboard-navigation#3.4 · yes · stale · its count of 20 activation rows and its named flight-return stand button moved, the table now 16 rows with the two mode buttons gone, and its cited stand_keyboard.rs lines moved
keyboard-navigation#3.5 · no · the macOS Backspace bindings in text.rs are unchanged, and the entity cites no reached file
keyboard-navigation#3.6 · no · the copy shortcut in keyboard.rs is unchanged, and the entity cites no reached file
keyboard-navigation#3.7 · no · implicit form submission in keyboard.rs is unchanged, and the session's swap of two buttons for a select is not a field that blocks it
keyboard-navigation#3.8 · no · a label's default click (pointer.rs:871-882) is unchanged, and the session names no label element on the select, which is labelled by aria-label
