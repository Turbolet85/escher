# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · no · the focusability and focus-fixup claim is re-read and unmoved (keyboard-navigation.md:8), its cited lines in element.rs and node.rs hold, and inert_subtree stays green
keyboard-navigation#1.2 · yes · add-to · F4 and F5 join its hidden-takes-no-part reading (a focused hidden control keeps focus and takes keys, with no panic); mutator.rs:1123-1135 also moves with the 944 lines added from line 2 on
keyboard-navigation#1.3 · yes · stale · document.rs is a merge surface with 290 lines added from line 45 on, so the citation document.rs:2222-2235 moves
keyboard-navigation#1.4 · no · select_popup.rs:329-361 sits above the moved line 513 and events/keyboard.rs is unreached, so the Tab-with-list-open claim holds
keyboard-navigation#1.5 · no · the focus-stays-on-open-select claim is re-read and unmoved (keyboard-navigation.md:12), and stand_keyboard.rs is unreached
keyboard-navigation#1.6 · yes · stale · mutator.rs is a merge surface with 944 lines added from line 2 on, so the citations mutator.rs:334-343 and 489-496 move
keyboard-navigation#1.7 · no · the disabled toggle's cited lines (element.rs:479-486, 512-514; node.rs:839-861) are off the moved set, and the disabled taking-effect rows hold green on CI under the restyle narrowing
keyboard-navigation#1.8 · no · the presence reading and bare-disabled focus removal hold, the disabled taking-effect rows are green under the restyle narrowing, and element.rs:480-485, 516-520 and 743 are unmoved
keyboard-navigation#1.9 · yes · stale · document.rs:1419-1421 moves (past line 45), and select_popup.rs:478-516 spans line 513, past which the row-3 change moves digits
keyboard-navigation#1.10 · yes · stale · mutator.rs is a merge surface with lines added from line 2 on, so the citation mutator.rs:1558-1572 moves
keyboard-navigation#1.11 · yes · stale · mutator.rs:1117-1122 and 1215-1227 move with the 944 lines added from line 2 on
keyboard-navigation#1.12 · yes · stale · document.rs:2281-2286 moves (past line 45); the node.rs citation 775-813 holds
keyboard-navigation#1.13 · yes · stale · stylo.rs is a merge surface reached whole, and the report does not clear the :focus and :focus-within match at 491-495 on the changed file
keyboard-navigation#1.14 · no · blitz-traits/src/events.rs is not in the reached files, so the focus and blur bubbling claim has no reached rule
keyboard-navigation#2.1 · yes · stale · document.rs is a merge surface, and the citations document.rs:1378-1380 and 1396-1415 move (past line 45)
keyboard-navigation#2.2 · no · the click-clears-focus rule lives in events/pointer.rs, which the change does not reach
keyboard-navigation#2.3 · yes · stale · document.rs:902-918 moves (past line 45) under the merge surface
keyboard-navigation#2.4 · no · the pointer click focus rule lives in events/pointer.rs and stand_keyboard.rs, neither reached, and the stand's click-focus rows are unedited and green
keyboard-navigation#2.5 · no · the select-click focus rule lives in events/pointer.rs and stand_act_select.rs, neither reached
keyboard-navigation#2.6 · yes · stale · select_popup.rs:747-777 and 779-799 sit past the moved line 513
keyboard-navigation#2.7 · no · the multiple-select click rule lives in events/pointer.rs and two unreached test files
keyboard-navigation#2.8 · no · the double-click rule lives in events/pointer.rs and stand_virtual_clock.rs, neither reached
keyboard-navigation#2.9 · no · the summary activation rule lives in events/pointer.rs, which the change does not reach
keyboard-navigation#2.10 · yes · stale · mutator.rs is a merge surface with lines added from line 2 on, so the citations mutator.rs:356-359 and 502-505 move
keyboard-navigation#2.11 · no · the checkbox and radio click rule lives in events/pointer.rs and harness_smoke.rs, neither reached
