# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the `inert` focusability read and the known limit that a node going inert keeps focus, the focus-fixup debt and its residual line that the chunk settles and inert_subtree asserts
keyboard-navigation#1.2 · yes · Tab and Shift+Tab move focus to the next and previous focusable node, the walk that skips inert nodes and that the served pages' Tab-order rows press
keyboard-navigation#1.3 · no · the cached focusability recompute on `tabindex`, `href` or `disabled` set or cleared; the chunk sets none of them and mutator.rs is not reached
keyboard-navigation#1.4 · yes · DISABLED and ENABLED states from a form control's `disabled` attribute, held in node.rs, which the stand_contrast exempt rule reads as `enabled` Some(false)
keyboard-navigation#1.5 · yes · `disabled` presence as the one keyed reading of focusability, Enter and Space activation and the DISABLED state, which the served rows' two orders and the contrast exempt rule read
keyboard-navigation#1.6 · no · the file input's generated inner button `tabindex`; no reached file, row or scope names a file input
keyboard-navigation#1.7 · no · `autofocus` focus on flush; mutator.rs and autofocus_attribute.rs are not reached and the scope names no autofocus
keyboard-navigation#1.8 · yes · focusing sets FOCUS and FOCUSRING and blurring clears them in node.rs, the focus change that the fixup's blur and the windowed shell's focus redraw both take
keyboard-navigation#1.9 · no · `:focus`, `:focus-within` and `:focus-visible` matching in stylo.rs, which is not reached; focus-ring visuals are Not needed
keyboard-navigation#1.10 · no · focus and blur not bubbling in blitz-traits events.rs, which is not reached and not named by the scope
keyboard-navigation#2.1 · yes · removal resets focus to body (None) and runs blur side effects, the reset an HTML focus fixup on inert takes (the IME part is Not needed)
keyboard-navigation#2.2 · no · pointer focus clearing on a non-interactive click; no chunk row or fixup drives that click and the scope does not name it
keyboard-navigation#2.3 · yes · a pointer click focusing a button or select, the driver `click` on a flight booker option refused `covered` (stand_act_select), and the focus agreement with the tree after a click
keyboard-navigation#2.4 · no · double-click timing on the VirtualClock; the chunk's rows drive keys, not paired pointer clicks, and the scope does not name it
keyboard-navigation#2.5 · yes · activating a details' first summary toggles `open` and focuses it, the default action that Enter or Space on a summary reaches, the route the `:open` check flips `open` by
keyboard-navigation#2.6 · yes · `:open` on details and dialog from the OPEN state, the new check the scope names, and its flip through the summary's toggle
keyboard-navigation#2.7 · no · checkbox and radio click toggling and `input` dispatch; the scope names the space route to checkedness, and no `change` absence is stated here
keyboard-navigation#2.8 · no · range input focus and pointer fraction; the Not needed list excludes range inputs
keyboard-navigation#3.1 · no · text input editing keys; no debt, row or scope names them
keyboard-navigation#3.2 · no · range input step keys; the Not needed list excludes range inputs
keyboard-navigation#3.3 · yes · single-choice select keys and typeahead, one `input` then one `change` per change, and the stated limits on `multiple`, `optgroup` and the unpainted popup, which select_interaction re-reads
keyboard-navigation#3.4 · yes · Space on a native checkbox or radio as the route to checkedness, and Enter or Space on a button such as `load-fetch` with the app's handlers running before `handle_click`
keyboard-navigation#3.5 · no · macOS Backspace under the Apple keybindings; platform-specific and named by no debt
keyboard-navigation#3.6 · no · copy with no text input focused; named by no debt, row or scope
keyboard-navigation#3.7 · no · implicit form submission from a text input; named by no debt, row or scope
