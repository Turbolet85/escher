# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · focusability rule read by the Tab orders of stand_keyboard, by unkeyed_actionable (dioxus_document.rs conflict) and by the is_bool_attr bridge handling
keyboard-navigation#1.2 · yes · the focus fixup at the mutator.rs flush for inert nodes, named in scope, with inert_subtree re-read
keyboard-navigation#1.3 · yes · Tab and Shift+Tab walk that stand_keyboard's stated Tab orders read
keyboard-navigation#1.4 · yes · Tab with a list open runs choose_current_and_close, the select list's keys and close path named in scope
keyboard-navigation#1.5 · yes · focus staying on the select while its list is open, pinned by stand_keyboard rows that are re-read
keyboard-navigation#1.6 · yes · cached focusability recompute lives in mutator.rs, a predicted conflict surface
keyboard-navigation#1.7 · yes · disabled toggling DISABLED/ENABLED and button focusability, with the node.rs conflict surface carrying lines 828-850
keyboard-navigation#1.8 · yes · disabled presence read by focusability, Enter and Space activation, as stand_keyboard's disabled rows assert at lines 889-930
keyboard-navigation#1.9 · yes · the select list's open gates (disabled, inert, hidden, no row) and close on inert or disabled, named in the select model scope
keyboard-navigation#1.10 · yes · the file input's generated button tabindex rule lives in mutator.rs, a predicted conflict surface
keyboard-navigation#1.11 · yes · autofocus focus on flush lives in mutator.rs, a predicted conflict surface
keyboard-navigation#1.12 · yes · FOCUS and FOCUSRING states set on focus and removed on blur, with node.rs (conflict surface) carrying lines 764-802
keyboard-navigation#1.13 · yes · :focus and :focus-within matching lives in stylo.rs, a predicted conflict surface
keyboard-navigation#1.14 · no · focus and focusin/focusout bubbling lives in blitz-traits events.rs, unreached, and no scoped check reads bubbling
keyboard-navigation#2.1 · no · focus reset on removing the focused node lives in document.rs and interaction_state_teardown.rs, neither reached nor named; the scope names only the flush fixup for inert nodes
keyboard-navigation#2.2 · no · clearing focus on a non-interactive click lives in pointer.rs, unreached, and no scoped check names it apart from the 2.4 restatement
keyboard-navigation#2.3 · yes · Escape, second click, outside click, page scroll and wheel closing an open list, the select model's close paths named in scope and scroll_into_view for a page scroll
keyboard-navigation#2.4 · yes · button and submit click focus and the disabled-control stop, restated in stand_keyboard (re-read) and stand_act_obstructed
keyboard-navigation#2.5 · yes · click on a select focusing it and clearing typeahead, the select list's clicks and click_lands_on_row named in scope
keyboard-navigation#2.6 · yes · pointer highlight and release-to-choose on the list's rows, read row by row against the select list's merge surface
keyboard-navigation#2.7 · yes · multiple select row click through handle_click's option arm, the select model's clicks named in scope
keyboard-navigation#2.8 · yes · double click window read on the document's clock, the clock invariant named in scope and the VirtualClock stand check
keyboard-navigation#2.9 · no · summary activation toggling details lives in pointer.rs, unreached, and no scoped check names summary
keyboard-navigation#2.10 · yes · OPEN state and :open matching live in mutator.rs and stylo.rs, both predicted conflict surfaces
keyboard-navigation#2.11 · no · checkbox and radio click toggling lives in pointer.rs, unreached, and no scoped check names it
