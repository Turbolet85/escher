# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · the focusable definition at node.rs:563-580 sits in a conflict surface, and the Tab walk and autofocus it decides are the orders stand_keyboard states
keyboard-navigation#1.2 · yes · the flush's clear_focus at mutator.rs:1051-1063 is the focus fixup the Needs name, in a conflict surface, with inert_subtree's rows
keyboard-navigation#1.3 · yes · the Tab and Shift+Tab walk is what stand_keyboard's LOAD_TAB_ORDER, CELLS_TAB_ORDER and list walks state and re-measure in both layout modes, though keyboard.rs and document.rs are not reached
keyboard-navigation#1.4 · yes · Tab with an open list running choose_current_and_close is the select list's key and close path through the mutator and node conflicts, and the stand_keyboard list walks
keyboard-navigation#1.5 · yes · focus staying on the select while its list is open, and the walk after each close, is the list's focus that the stand_keyboard list walks the Needs name pin
keyboard-navigation#1.6 · yes · focusability recomputed at mutator.rs:350-359 and 515-522 lies in a conflict surface, and focusability_updates is named beside stand_keyboard
keyboard-navigation#1.7 · yes · the form-control disabled toggle at node.rs:828-850 lies in a conflict surface, and focusability_updates asserts the loss of focusability
keyboard-navigation#1.8 · yes · the presence-keyed is_disabled read gates the Enter and Space activation and focusability that the tier-0 target re-proves and focusability_updates asserts
keyboard-navigation#1.9 · yes · a select's list closing when made disabled, inert, hidden or removed is the select model's close path through the mutator and node conflicts, asserted by select_popup
keyboard-navigation#1.10 · yes · the file input's inner button tabindex at mutator.rs:1484-1498 lies in a conflict surface, so the take can move it
keyboard-navigation#1.11 · yes · the autofocus pick at mutator.rs:1143-1155 and 1045-1050 lies in a conflict surface, and the focus it sets on flush is the flush fixup in scope
keyboard-navigation#1.12 · yes · the FOCUS and FOCUSRING state writes at node.rs:764-802 lie in the node.rs conflict surface, and the :focus styling reads them
keyboard-navigation#1.13 · yes · the :focus and :focus-within matching at stylo.rs:488-492 lies in a conflict surface
keyboard-navigation#1.14 · no · focus and focusin/focusout bubbling is in blitz-traits events.rs, which the take does not reach, and no scope item or re-read check states event bubbling
keyboard-navigation#2.1 · no · removal's focus reset to body is in document.rs:1367-1404, which the take does not reach, and no scope item names removal focus
keyboard-navigation#2.2 · no · the click-outside focus clear is in pointer.rs:1093-1096, which the take does not reach, and no scope item names it
keyboard-navigation#2.3 · yes · Escape, a second click, a click outside and a page scroll closing a list with the value unchanged are the select's close paths the Needs name, and scrolling.rs's visible_region is in the conflict
keyboard-navigation#2.4 · yes · a click focusing a stand button and Tab continuing from it, and the disabled form control's walk stop, are stated and re-measured by stand_keyboard at 850-877 in both layout modes
keyboard-navigation#2.5 · yes · a click on a select focusing it and clearing its typeahead is the select's click path in the Needs, with the typeahead the clock invariant names and stand_act_select named beside stand_keyboard
keyboard-navigation#2.6 · yes · a pointer over a row moving the highlight, and a release choosing it, are the select list's clicks and the later key that goes on from the highlight, with select_popup's rows named in scope
keyboard-navigation#2.7 · yes · the multiple select's row click with Ctrl, Meta and Shift is the select model's clicks, and select_interaction and stand_act_select are named in scope
keyboard-navigation#2.8 · yes · the double click's 500 ms and 2 px window on the document's clock is the clock invariant the Needs name, witnessed by stand_virtual_clock
keyboard-navigation#2.9 · no · a summary's pointer click toggling open and focusing it is in pointer.rs:920-946, which the take does not reach, and no scope item names summary activation or open_state
keyboard-navigation#2.10 · yes · the OPEN state writes at mutator.rs:372-375 and 528-531 and the :open match at stylo.rs:510 lie in conflict surfaces
keyboard-navigation#2.11 · no · checkbox and radio clicks at pointer.rs:854-919 are in a file the take does not reach, and no scope item names them
