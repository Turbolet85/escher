# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the inert skip and the presence-based disabled gate on a form control are v020-03 focusability witnesses, and the focus fixup limit is the task's residual
keyboard-navigation#1.2 · yes · Tab and Shift+Tab moving to the next and previous focusable node is the walk the inert witness needs to skip an inert subtree
keyboard-navigation#1.3 · yes · the focusability cache recomputed when disabled is set or removed is what the bare-disabled-after-creation witness rests on
keyboard-navigation#1.4 · yes · a button losing focusability when disabled is set, and the DISABLED and ENABLED state the snapshot's enabled reading takes, are v020-03 witnesses
keyboard-navigation#1.5 · yes · disabled presence on a form control alone is the focusability-by-presence v020-03 witness the task names
keyboard-navigation#1.6 · no · no Need names a file input's generated inner button or its tab stop
keyboard-navigation#1.7 · no · no Need names autofocus, and the fixture checks the chunk adds are for lint readings
keyboard-navigation#1.8 · no · the FOCUS and FOCUSRING states feed :focus and the focus ring, which the Not-needed list excludes, and the snapshot's focus reads the tree
keyboard-navigation#1.9 · no · the :focus, :focus-within and :focus-visible selectors are focus-ring styling, which the Not-needed list excludes
keyboard-navigation#1.10 · no · no Need names focus-event bubbling
keyboard-navigation#2.1 · no · no Need names removing a focused node, and the inert witness restores on the inert attribute's removal, not a node's
keyboard-navigation#2.2 · no · no Need names clearing focus on a non-interactive click, and the focus-on-click need rests on a control's click focusing (2.3)
keyboard-navigation#2.3 · yes · a click focusing a button or a select is the focus-on-click need, witnessed by the fx-before click that reads focused in both layout modes
keyboard-navigation#2.4 · yes · the double click on the virtual clock (500 ms, 2 px), each click still focusing its target, is the double-click witness reached through v020-06's clock and the password caret click
keyboard-navigation#2.5 · no · no Need names summary or details activation
keyboard-navigation#2.6 · no · no Need names a checkbox or radio click, and the native checkbox or radio witness reads the builder's state rather than a click
keyboard-navigation#2.7 · yes · a range input's click focus and pointerdown value are the click the range bullet's named witness, the Timer slider, rests on
keyboard-navigation#3.1 · no · no Need names a text input's arrow, Home, End, Delete or Backspace keys, and the password caret witness rests on the click and the double click
keyboard-navigation#3.2 · yes · the arrow, Home and End keys of a focused range input are the keyboard model the range bullet limits to the named Timer slider witness
keyboard-navigation#3.3 · yes · the select's arrows, Home, End, typeahead on the document's clock, and one input then one change are the v020-08 select witnesses
keyboard-navigation#3.4 · no · Enter and Space on each stand button is owned by the later A11y re-read entry, per the task's Not-needed list
keyboard-navigation#3.5 · no · no Need names the macOS Backspace keybinding
keyboard-navigation#3.6 · no · no Need names copying with no text input focused
keyboard-navigation#3.7 · no · no Need names implicit form submission from a text input
keyboard-navigation#3.8 · no · no Need names a label click focusing its select
