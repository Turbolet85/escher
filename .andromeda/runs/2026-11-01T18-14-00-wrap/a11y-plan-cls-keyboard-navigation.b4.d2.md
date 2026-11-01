# Classified — a11y-plan · keyboard-navigation.b4

## Answers
keyboard-navigation#5.12 · no · the Android focus request sits in apps and examples files the session does not reach
keyboard-navigation#6.1 · no · the docs.rs fixture under examples/assets is not reached and holds no hunk
keyboard-navigation#6.2 · no · the google fixture under examples/assets is not reached
keyboard-navigation#6.3 · no · the google fixture's jsaction bindings are not reached
keyboard-navigation#6.4 · no · the google fixture's focus-visible style is not reached
keyboard-navigation#6.5 · no · the graphite fixture is not reached
keyboard-navigation#6.6 · no · servo.css is not reached
keyboard-navigation#6.7 · no · servo.html is not reached
keyboard-navigation#6.8 · no · examples/form.rs is not reached
keyboard-navigation#7.1 · no · the stand snapshot check in stand_snapshot_state.rs is not reached, and focus stays on the focused control after the change
keyboard-navigation#7.2 · yes · stale · document.rs gained 290 lines from line 45 on, so its root-fallback citation at document.rs:674-677 moves; the stand walk's rows stay green and unedited
keyboard-navigation#7.3 · no · common/mod.rs is not reached and its shared module holds no hunk
keyboard-navigation#7.4 · no · range_input_interaction.rs and stand_act_range.rs are not reached
keyboard-navigation#7.5 · yes · add-to · the new key_press_after_container_lost_its_box check joins this record of non-stand key-press tests: F4 (a key press on a control whose anonymous block was dropped no longer panics) and F7 (3 tests in both layout modes, outside the stand legs)
keyboard-navigation#7.6 · no · select_interaction.rs and stand_act_select.rs are not reached and their rows hold
keyboard-navigation#7.7 · no · select_popup.rs is reached, but the session removes a skip and adds no test, so the 25-test count and the both-modes run hold; select_list.rs is not reached
keyboard-navigation#7.8 · no · stand_keyboard.rs is not reached and its 23-test count holds
keyboard-navigation#8.1 · no · the session adds no tabindex handling, and the reached list names no file of the s06 slice
keyboard-navigation#8.2 · no · the session adds no tabindex handling, and the reached list names no file of the s10 slice
keyboard-navigation#8.3 · no · the session adds no focus or keyboard token, and the panic fix clears a layout field only
