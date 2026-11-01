# Classified — a11y-plan · keyboard-navigation.b2

## Answers
keyboard-navigation#2.12 · no · the checkbox and radio change dispatch lives in events/pointer.rs, which the reached list does not name
keyboard-navigation#2.13 · no · range-input click focus lives in events/pointer.rs, not reached, and the session changes no range input
keyboard-navigation#3.1 · no · text-input key handling lives in node/text.rs and events/keyboard.rs, neither in the reached list
keyboard-navigation#3.2 · no · range-input arrow keys live in events/keyboard.rs, not reached, and the session changes no range input
keyboard-navigation#3.3 · yes · add-to · F5: a focused select whose anonymous block was dropped keeps focus and still takes its keys, its choice moving past that block, the limit the fixup's `is_inert()` reading leaves in place
keyboard-navigation#3.4 · no · press_open_list_key lives in events/keyboard.rs, not reached; the select_popup.rs citations at 202-230 and 249-273 sit above line 513, where the changed lines begin, and hold
keyboard-navigation#3.5 · no · the choose and close paths live in events/keyboard.rs and events/pointer.rs, not reached; select_popup.rs citations at 294-327 and 363-375 sit above line 513 and do not move
keyboard-navigation#3.6 · yes · stale · the row-3 skip removal moved tests/blitz-tests/tests/select_popup.rs past line 513, so the citation 866-907 moves
keyboard-navigation#3.7 · yes · stale · packages/blitz-dom/src/document.rs is a merge surface gaining 290 lines from line 45 on, so the citation 2436-2438 moves
keyboard-navigation#3.8 · no · multiple-select keys live in events/keyboard.rs, node/select.rs and select_interaction.rs, none in the reached list
keyboard-navigation#3.9 · yes · stale · the row-3 skip removal moved tests/blitz-tests/tests/select_popup.rs past line 513, so the citation 991-1028 moves
keyboard-navigation#3.10 · no · multiple-select Space and typeahead live in events/keyboard.rs and select_interaction.rs, none reached
keyboard-navigation#3.11 · no · disabled-row rules live in events/keyboard.rs and node/select.rs, not reached
keyboard-navigation#3.12 · no · Shift-span seeding lives in node/select.rs and select_interaction.rs, not reached
keyboard-navigation#3.13 · no · optgroup rules live in node/select.rs and form.rs, not reached
keyboard-navigation#3.14 · yes · stale · packages/blitz-dom/src/mutator.rs is a merge surface gaining 944 lines from line 2 on, so the citation 1334-1343 moves
keyboard-navigation#3.15 · no · the driver's type verb lives in escher-driver and tests/blitz-tests/tests/stand_act_select.rs, neither reached; the driver holds no edit
keyboard-navigation#3.16 · no · escher-driver's schema.rs is not reached and the driver holds no edit
keyboard-navigation#3.17 · yes · add-to · F4: a key press with focus on a hidden, `display:none` or CSS-hidden control whose anonymous block was dropped no longer panics, and the key still acts
keyboard-navigation#3.18 · no · Backspace on macOS lives in node/text.rs, not reached
keyboard-navigation#3.19 · no · the copy shortcut lives in events/keyboard.rs, not reached
keyboard-navigation#3.20 · no · implicit submission lives in events/keyboard.rs, not reached
keyboard-navigation#3.21 · no · the label click rule lives in events/pointer.rs, not reached, and the named select_popup test is cited by name, not by a line the change moved
keyboard-navigation#3.22 · yes · stale · packages/blitz-dom/src/node/node.rs is a merge surface reached whole, so the cited span 2028-2058 may move and the report does not show the claim unchanged there
keyboard-navigation#3.23 · no · winit key conversion lives in packages/blitz-shell/src/convert_events.rs, not reached
