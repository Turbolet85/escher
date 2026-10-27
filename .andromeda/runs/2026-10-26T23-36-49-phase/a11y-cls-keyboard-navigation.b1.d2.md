# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · its inert skip is pinned by inert_subtree.rs, a named stand-repeat a11y file the chunk re-runs, and its disabled gate by stand_keyboard.rs:677-718
keyboard-navigation#1.2 · yes · the forward Tab walk is what stand_keyboard.rs's boot and enabled-state Tab-order tables assert, an a11y file the chunk re-runs
keyboard-navigation#1.3 · yes · the focusability cache recomputed on a tabindex or disabled change is what stand_keyboard.rs's enabled-state Tab order walks once a stand button's disabled changes
keyboard-navigation#1.4 · yes · the DISABLED and ENABLED toggle feeds the snapshot's enabled reading, which the lint's contrast exemption reads, and stand_keyboard.rs's enabled-state Tab order
keyboard-navigation#1.5 · yes · the presence reading sets the enabled input of the lint's contrast exemption on disabled stand controls, and stand_keyboard.rs:677-718 pins it
keyboard-navigation#1.6 · no · the file input's generated inner button sits on no stand screen the chunk reads, and no re-run file names it
keyboard-navigation#1.7 · no · autofocus is witnessed only by autofocus_attribute.rs, which no named re-run covers, and the lint reads no focus state
keyboard-navigation#1.8 · yes · the FOCUS state is the focused reading stand_keyboard.rs:638-665 asserts after a click, while the FOCUSRING half is the focus-ring surface the task does not reach
keyboard-navigation#1.9 · no · the lint reads no focus pseudo-class style, stand_contrast's stated colour states are hover and active, and the focus ring is an unreached surface
keyboard-navigation#1.10 · no · focus and blur bubbling reaches neither the lint's snapshot walk nor any named re-run file
keyboard-navigation#2.1 · no · the chunk removes no focused node (the twelve Creates only add rows), and interaction_state_teardown.rs is named in no re-run
keyboard-navigation#2.2 · no · a click on a non-interactive area is witnessed by no named re-run file, and no re-run clicks one
keyboard-navigation#2.3 · yes · the stand's Create is a button, and stand_keyboard.rs:638-665 asserts the focus a click on a stand button gives
keyboard-navigation#2.4 · no · two clicks on one point read as a double click, which still focuses each target as one click does, so the Create rows and findings read no click count; its witness stand_virtual_clock.rs is named in no re-run
keyboard-navigation#2.5 · no · summary activation is witnessed by no named re-run file, and the lint reads only role, name, bounds and enabled from the snapshot
keyboard-navigation#2.6 · no · checkbox and radio toggling is witnessed by harness_smoke.rs, which no re-run names, and the lint reads no checked state
keyboard-navigation#2.7 · no · the range click and pointerdown move a value the lint never reads, and range_input_interaction.rs is named in no re-run
keyboard-navigation#3.1 · no · text-input editing keys are witnessed by no named re-run file, and the lint presses no key
keyboard-navigation#3.2 · no · the range arrow, Home and End keys are witnessed by range_input_interaction.rs, which no re-run names, and the lint reads no value
keyboard-navigation#3.3 · yes · select_interaction.rs, named among the stand-repeat a11y files the chunk re-runs, pins this select key model
keyboard-navigation#3.4 · yes · stand_keyboard.rs pins Enter and Space on each stand button (155-211, 344-349, 365-396), an a11y file the chunk re-runs, so this activation is re-asserted
keyboard-navigation#3.5 · no · Backspace under the Apple bindings is macOS-only, reached by no stand screen, lint walk or named re-run
keyboard-navigation#3.6 · no · copy with no text input focused is witnessed by no named re-run file, and no lint or stand_lint step copies text
keyboard-navigation#3.7 · no · implicit form submission from a text input is witnessed by no named re-run file, and the lint presses no key
keyboard-navigation#3.8 · no · a label's default click is witnessed by no named re-run file, and the snapshot's names come from label text, not from a click
