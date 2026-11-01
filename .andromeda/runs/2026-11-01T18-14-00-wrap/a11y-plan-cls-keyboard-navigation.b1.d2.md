# Classified — a11y-plan · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · stale · element.rs and node.rs are merge surfaces, so the citations 742-774, 516-520, 743 and node.rs:574-591 are touched though the focusability rule holds
keyboard-navigation#1.2 · yes · add-to · F4 (no panic on a key press with focus on a dropped-block control), F5 (the fixup reads is_inert alone, so a hidden focused control keeps focus) and F6 (the limit's owners) join this line; mutator.rs is a merge surface, so citation 1123-1135 moves
keyboard-navigation#1.3 · yes · stale · document.rs is a merge surface with 290 lines added from line 45, so citation 2222-2235 moves
keyboard-navigation#1.4 · no · select_popup.rs's edit sits at line 513, below the 329-361 test, so that span holds; events/keyboard.rs is not reached
keyboard-navigation#1.5 · no · stand_keyboard.rs holds no edit and stays green, its rows unedited, so the focus-with-list claim is unchanged
keyboard-navigation#1.6 · yes · stale · mutator.rs is a merge surface, so citations 334-343 and 489-496 can move under the attribute-restyle narrowing
keyboard-navigation#1.7 · yes · stale · element.rs and node.rs are merge surfaces, so citations 479-486, 512-514 and 839-861 are touched
keyboard-navigation#1.8 · yes · stale · element.rs is a merge surface, so citations 480-485, 516-520 and 743 are touched; the presence reading holds
keyboard-navigation#1.9 · yes · stale · document.rs is a merge surface, so citation 1419-1421 moves; select_popup.rs lines 478-516 span the removed row-3 skip at 513
keyboard-navigation#1.10 · yes · stale · mutator.rs is a merge surface, so citation 1558-1572 moves
keyboard-navigation#1.11 · yes · stale · mutator.rs is a merge surface, so citations 1117-1122 and 1215-1227 move
keyboard-navigation#1.12 · yes · stale · document.rs and node.rs are merge surfaces, so citations 2281-2286 and 775-813 move
keyboard-navigation#1.13 · yes · stale · stylo.rs is a merge surface, its hunks at 231-238 and 771-776 sit above 491-495, which moves
keyboard-navigation#1.14 · no · packages/blitz-traits/src/events.rs is not in the reached files and its focus and bubble rule is unchanged
keyboard-navigation#2.1 · yes · stale · document.rs is a merge surface, so citations 1378-1380 and 1396-1415 can move
keyboard-navigation#2.2 · no · events/pointer.rs is not reached and the non-interactive click clear is unchanged
keyboard-navigation#2.3 · yes · stale · document.rs is a merge surface, so citation 902-918 can move; the list close rules hold
keyboard-navigation#2.4 · no · pointer.rs and stand_keyboard.rs are not reached, and stand_keyboard holds green with its rows unedited
keyboard-navigation#2.5 · no · pointer.rs and stand_act_select.rs are not reached and the select click path is unchanged
keyboard-navigation#2.6 · yes · stale · select_popup.rs lines 747-799 lie past the removed row-3 skip at 513, so the citations move
keyboard-navigation#2.7 · no · pointer.rs, select_interaction.rs and stand_act_select.rs are not reached and the multiple-select click path is unchanged
keyboard-navigation#2.8 · no · pointer.rs and stand_virtual_clock.rs are not reached and the double-click clock is unchanged
keyboard-navigation#2.9 · no · pointer.rs is not reached and the summary activation is unchanged
keyboard-navigation#2.10 · yes · stale · mutator.rs, element.rs and stylo.rs are merge surfaces, so citations 356-359, 502-505, 488-549 and stylo.rs:513 can move
keyboard-navigation#2.11 · no · pointer.rs and harness_smoke.rs are not reached and the checkbox and radio click path is unchanged
