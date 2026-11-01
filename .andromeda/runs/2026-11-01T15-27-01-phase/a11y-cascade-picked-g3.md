# a11y · picked accounting · group 3 of 8

## Picked
keyboard-navigation#1.1 · carried · the rule reads through the `node.rs` conflict surface and `unkeyed_actionable` in the `dioxus_document.rs` one, and `stand_keyboard`, which the chunk re-reads, witnesses it
keyboard-navigation#1.2 · carried · the focus fixup lives in `mutator.rs`, a conflict surface, and `inert_subtree`, which the chunk re-reads, witnesses it
keyboard-navigation#1.3 · carried · Tab and Shift+Tab focus movement is the SC 2.4.3 focus order the chunk re-proves through `stand_keyboard`
keyboard-navigation#1.4 · carried · Tab with a list open is one of the list's keys the scope keeps unchanged through the conflicts and re-reads on the select list's merge surface
keyboard-navigation#1.5 · carried · focus on the select and the walk's hold are stated by `stand_keyboard` rows the chunk re-reads, and belong to the select model the scope names
keyboard-navigation#1.6 · carried · the recomputing arms live in `mutator.rs`, a conflict surface, and `focusability_updates` re-runs in the a11y leg
keyboard-navigation#1.7 · carried · the state toggling lives in `node.rs`, a conflict surface, and `focusability_updates` re-runs in the a11y leg
keyboard-navigation#1.8 · carried · disabled-ness gates the Enter and Space activation and focusability that the chunk re-proves, `stand_keyboard` and `focusability_updates` witnessing it
keyboard-navigation#1.9 · carried · the list's disabled row, open gates and close paths are the select model the scope keeps unchanged and re-reads on its merge surface
keyboard-navigation#1.10 · carried · the inner button's `tabindex` lives in `mutator.rs`, a conflict surface
keyboard-navigation#1.11 · carried · autofocus at the flush lives in `mutator.rs`, a conflict surface
keyboard-navigation#1.12 · carried · the focus element states live in `node.rs`, a conflict surface
keyboard-navigation#1.13 · carried · the focus pseudo-class matching lives in `stylo.rs`, a conflict surface
keyboard-navigation#2.3 · carried · the list's close paths are named by the scope's select model, and a page scroll's close runs through the `scrolling.rs` conflict
keyboard-navigation#2.4 · carried · a click's focus on a button is stated by `stand_keyboard` rows the chunk re-reads, and the delta's hit-testing hunks reach the path to it
keyboard-navigation#2.5 · carried · a click on a select is the select model's, `stand_act_select` re-running in the a11y leg
keyboard-navigation#2.6 · carried · the pointer over and on a row is among the list's clicks the scope keeps and re-reads on its merge surface
keyboard-navigation#2.7 · carried · `click_select_row` is an engine-debts change whose merge surface the chunk re-reads, `select_interaction` and `stand_act_select` re-running in the a11y leg
keyboard-navigation#2.8 · carried · the double click's window on the document's clock is the clock invariant the scope keeps through the `scrolling.rs` conflict
keyboard-navigation#2.9 · carried · summary activation is part of the Enter and Space activation the chunk re-proves, and its toggle writes `open` through the `mutator.rs` conflict surface
keyboard-navigation#2.10 · carried · the `open` arms live in `mutator.rs` and the `:open` match in `stylo.rs`, both conflict surfaces
