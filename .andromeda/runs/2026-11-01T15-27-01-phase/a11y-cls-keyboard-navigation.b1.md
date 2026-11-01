# Classified — a11y · keyboard-navigation.b1

## Answers
keyboard-navigation#1.1 · yes · focusability by form-control presence and tabindex feeds the Tab walk, unkeyed_actionable (dioxus_document.rs conflict) and the focus order stand_keyboard re-reads
keyboard-navigation#1.2 · yes · the mutator's flush clear_focus on a focused inert node is the focus fixup at the mutator's flush, with inert_subtree re-read
keyboard-navigation#1.3 · yes · Tab and Shift+Tab moving focus next and previous is the focus order stand_keyboard's Tab orders re-prove
keyboard-navigation#1.4 · yes · Tab with an open list closing it first is the select list's keys and close paths, within the list walks stand_keyboard re-reads
keyboard-navigation#1.5 · yes · focus staying on the select while its list is open, and the walk after each close, are the list walks stand_keyboard re-reads
keyboard-navigation#1.6 · yes · focusability recomputed on tabindex, href or disabled writes lives in mutator.rs (conflict surface) and is the focus order's cached state
keyboard-navigation#1.7 · yes · disabled toggling DISABLED/ENABLED and button focusability read node.rs:828-850 (conflict surface) and the Tab walk
keyboard-navigation#1.8 · yes · disabled presence gating Enter and Space activation and focusability is the tier-0 activation and focus order the scope names
keyboard-navigation#1.9 · yes · a select's list closing when made inert, disabled, hidden or removed is the select model's close paths through the mutator and node conflicts
keyboard-navigation#1.10 · yes · the file input's inner button tabindex=-1 lives in mutator.rs (conflict surface), so the rule binds
keyboard-navigation#1.11 · yes · autofocus on flush lives in mutator.rs (conflict surface, lines 1045-1050 and 1143-1155), which the chunk edits through its conflict
keyboard-navigation#1.12 · yes · FOCUS and FOCUSRING element states lives in node.rs:764-802 (conflict surface), read by the focus order and the snapshot's focused node
keyboard-navigation#1.13 · yes · :focus and :focus-within matching lives in stylo.rs:488-492 (conflict surface), read by the focused-cell style of Cells
keyboard-navigation#1.14 · no · focus and blur bubbling in blitz-traits events is a component the chunk only uses; the scope names no event bubbling and no reached file holds it
keyboard-navigation#2.1 · no · focus reset to body on removal lives in document.rs, which is unreached; the scope names only the flush's inert fixup and the select close, not removal
keyboard-navigation#2.2 · no · a non-interactive click clearing focus lives in pointer.rs, which is unreached; the scope names no such click and the hit stops are inert-only
keyboard-navigation#2.3 · yes · Escape, a second click, an outside click, a page scroll and a wheel closing a list are the select's close paths and the scroll_into_view use, both named
keyboard-navigation#2.4 · yes · a click on a button focusing it, ending the walk matched, and the disabled-control gate are the stand_keyboard click-focus rows re-read in both layout modes
keyboard-navigation#2.5 · yes · a click on a select focusing it, clearing typeahead, opening or closing the list and the option arm are the select list's clicks named in scope, with stand_act_select beside stand_keyboard
keyboard-navigation#2.6 · yes · a pointer over an open list's row moving the highlight that a later key continues from, and the release choosing the row, are the list's keys and clicks named in scope
keyboard-navigation#2.7 · yes · a multiple select's row click with Ctrl, Meta or Shift is the select model's clicks, witnessed by stand_act_select and select_interaction named beside stand_keyboard
keyboard-navigation#2.8 · yes · the double-click window read on the document's clock is the clock invariant the scrolling.rs conflict names
keyboard-navigation#2.9 · yes · activating a summary to toggle and focus it is the Enter and Space activation and the focus order the tier-0 target names
keyboard-navigation#2.10 · yes · the OPEN state from the open attribute lives in mutator.rs:372-375 and 528-531 and stylo.rs:510, all conflict surfaces
keyboard-navigation#2.11 · no · checkbox and radio click toggling and focus lives in pointer.rs and harness_smoke.rs, both unreached; the scope names no checkbox or radio behaviour
