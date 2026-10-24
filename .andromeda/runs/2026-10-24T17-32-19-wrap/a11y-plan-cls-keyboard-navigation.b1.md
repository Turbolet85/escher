# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · the session adds inert_subtree only to stand-repeat's named list and this focusability row places no runner; its rule and the focus-fixup limit it cites from inert_subtree.rs are unmoved (Untouched, §5 engine rules)
keyboard-navigation#1.2 · no · the Tab and Shift+Tab walk is an engine rule in keyboard.rs and document.rs, neither reached and both unchanged (Untouched, §5 engine rules)
keyboard-navigation#1.3 · no · the focusability cache rule lives in mutator.rs and focusability_updates.rs, neither reached, and the session moves no tabindex, href or disabled behaviour
keyboard-navigation#1.4 · no · form-control disabled-ness is an engine rule in element.rs and node.rs, neither reached, and its behaviour is unmoved (Untouched, §5 engine rules)
keyboard-navigation#1.5 · no · the presence-keyed disabled reading is an engine rule across element.rs, pointer.rs and keyboard.rs, none reached; its cited stand_keyboard and focusability_updates tests run unchanged
keyboard-navigation#1.6 · no · the file input's generated button tabindex is set in mutator.rs, not reached, and no behaviour or runner of it moves
keyboard-navigation#1.7 · no · autofocus is an engine rule in mutator.rs, not reached; autofocus_attribute.rs is not among the ten named targets and runs unchanged
keyboard-navigation#1.8 · no · focus element states are set in document.rs and node.rs, neither reached, and the session adds no focus behaviour
keyboard-navigation#1.9 · no · :focus and :focus-within matching is an engine rule in stylo.rs, not reached and unmoved
keyboard-navigation#1.10 · no · focus and blur not bubbling is an event-type rule in blitz-traits events.rs, not reached and unmoved
keyboard-navigation#2.1 · no · focus reset on node removal is an engine rule in document.rs; interaction_state_teardown.rs is not reached and not among the ten named targets
keyboard-navigation#2.2 · no · a click on a non-interactive area clearing focus is an engine rule in pointer.rs, not reached and unmoved
keyboard-navigation#2.3 · no · click-focus on button, input and select is an engine rule (Untouched, §5); its cited stand checks run unchanged, the reached fixtures/mod.rs change only adds ITEMS and many_items at lines 64-86, and CI#40114977936 is green
keyboard-navigation#2.4 · yes · add-to · stand-repeat's named list now runs virtual_clock_engine, the test behind this §5 double-click row the Touches name; the double-click behaviour is unmoved, so the row must carry that its test also runs in stand-repeat
keyboard-navigation#2.5 · no · summary activation toggling is an engine rule in pointer.rs, not reached and unmoved
keyboard-navigation#2.6 · no · checkbox and radio click toggling is an engine rule in pointer.rs, not reached; harness_smoke.rs is not among the ten named targets
keyboard-navigation#2.7 · no · a click focusing a range input is an engine rule in pointer.rs, not reached; no range test is among the ten named targets
keyboard-navigation#3.1 · no · text-input key handling is an engine rule in text.rs, not reached and unmoved
keyboard-navigation#3.2 · no · range arrow, Home and End keys are an engine rule in keyboard.rs, not reached; no range test is among the ten named targets
keyboard-navigation#3.3 · yes · add-to · stand-repeat's named list now runs select_interaction, the test behind this §5 single-choice select row the Touches name; the keys are unmoved, so the row must carry that its test also runs in stand-repeat
keyboard-navigation#3.4 · no · Enter and Space activation is an engine rule in keyboard.rs and events/mod.rs, neither reached; its cited stand_keyboard and stand_act_keys tests are not among the ten named targets
keyboard-navigation#3.5 · no · macOS Backspace bindings are an engine rule in text.rs, not reached and unmoved
keyboard-navigation#3.6 · no · the copy shortcut with no focused text input is an engine rule in keyboard.rs, not reached and unmoved
keyboard-navigation#3.7 · no · implicit form submission is an engine rule in keyboard.rs, not reached and unmoved
keyboard-navigation#3.8 · no · a label's default click focusing its select is an engine rule in pointer.rs, not reached; the session's select row is #3.3, not this label row
