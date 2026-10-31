# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the inert focus fixup not built, focus kept until it moves, SC 2.4.3 and its residual ownership, which the inert_subtree edit and the inert read in the focusable rule carry
keyboard-navigation#1.2 · yes · Tab and Shift+Tab walk to the next and previous focussable node, skipping inert nodes as inert_subtree asserts, which the served pages' Tab-order rows walk
keyboard-navigation#1.3 · no · the focusability cache recompute on tabindex, href or disabled is in mutator.rs, which the chunk does not reach, and the chunk names no runtime set or clear of those attributes
keyboard-navigation#1.4 · yes · disabling a button, input, select or textarea toggles the DISABLED and ENABLED states, the toggle's node.rs part (reached whole), which the served Tab-order rows and the contrast exemption read
keyboard-navigation#1.5 · yes · the presence reading of disabled gates focusability and the pointer and Enter and Space gates in events/ (reached whole), and the served rows' ENABLED_TAB_ORDER and contrast exemption read it
keyboard-navigation#1.6 · no · the file input's generated inner button is set in mutator.rs, which the chunk does not reach, and no Need or reached unit names a file input
keyboard-navigation#1.7 · no · autofocus on flush is in mutator.rs, which the chunk does not reach, and no Need or reached unit names autofocus
keyboard-navigation#1.8 · yes · focus and blur write the FOCUS and FOCUSRING states (node.rs:764-802, reached whole), which the focus agreement, the refresh path and the focus redraw route read; the focus-ring paint itself is Not needed
keyboard-navigation#1.9 · no · the :focus, :focus-within and :focus-visible matching is in stylo.rs, which the chunk does not reach or name, and no Need names those selectors
keyboard-navigation#1.10 · no · focus and blur bubbling is in blitz-traits events.rs, which the chunk does not reach or name
keyboard-navigation#2.1 · no · the removal teardown is in document.rs and interaction_state_teardown.rs, neither reached nor named, and its IME half is Not needed
keyboard-navigation#2.2 · yes · a click on a non-interactive area clears focus in events/pointer.rs (reached whole), one way focus moves, which the focus agreement and the kept-until-it-moves limit read
keyboard-navigation#2.3 · yes · pointer clicks focus buttons, submit inputs and selects, a driver click on an option is refused covered, and an accepted click on fx-before reads focused and is the tree's focus, which the focus agreement and the stand_act_select reading carry
keyboard-navigation#2.4 · yes · double-click timing in events/pointer.rs (reached whole), with its clause that each click still focuses its target
keyboard-navigation#2.5 · yes · activating a details' first summary toggles open and focuses the summary in events/pointer.rs (reached whole), the path that flips :open for the new check
keyboard-navigation#2.6 · yes · a details or dialog with open holds the OPEN state and summary activation flips :open, which the new :open check reads and no test asserts today
keyboard-navigation#2.7 · yes · checkbox and radio clicks toggle the native state, dispatch input and focus the control in events/pointer.rs (reached whole), the native checked state the edited toggled write reads and the input-only event record
keyboard-navigation#2.8 · no · a click on a range input focuses it and sets its value by pointer fraction, and no debt touches a range input (Not needed)
keyboard-navigation#3.1 · no · text input key handling is in node/text.rs, which the chunk does not reach, and this rule states no change event, so it carries none of the no-change record the Needs name
keyboard-navigation#3.2 · no · a focused range input's step keys, and no debt touches a range input (Not needed)
keyboard-navigation#3.3 · yes · a focused single-choice select's keys, typeahead, one input then one change, the disabled-select gate and the stated limits on multiple, optgroup and the unpainted popup, which the boot overlap and the select_interaction witness carry
keyboard-navigation#3.4 · yes · Enter and Space on a button, checkbox or radio dispatch the click and the app's handlers run before handle_click, which the served keyboard-activation row for load-fetch and the Space route to checkedness depend on
keyboard-navigation#3.5 · no · the macOS Backspace bindings are in node/text.rs, which the chunk does not reach, and the chunk adds no macOS-only check
keyboard-navigation#3.6 · yes · copy with no text input focused lives in events/keyboard.rs (reached whole), so the rule moves with any line of that file
keyboard-navigation#3.7 · yes · implicit form submission from a text input lives in events/keyboard.rs (reached whole), so the rule moves with any line of that file
