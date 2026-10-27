# a11y · picked accounting · group 3 of 6

## Picked
keyboard-navigation#1.1 · carried · its focusability rule lives in `node/element.rs` (merge surface) and `node/node.rs` (conflict surface), and `stand_keyboard`'s Tab tables that the chunk re-reads depend on it
keyboard-navigation#1.2 · carried · the Tab walk `focus_next_node` / `focus_prev_node` lives in `document.rs`, a merge surface, and `stand_keyboard` re-reads it
keyboard-navigation#1.3 · carried · focusability is recomputed in `mutator.rs`, a predicted conflict surface, and `focusability_updates` in the a11y leg witnesses it
keyboard-navigation#1.4 · carried · the disabled states live in `node/element.rs` and `node/node.rs`, both reached by the merge
keyboard-navigation#1.5 · carried · the presence reading of `disabled` spans `node/element.rs` and `events/pointer.rs`, both merge surfaces, and the scope keeps it unchanged
keyboard-navigation#1.6 · carried · the file input's inner `tabindex="-1"` lives in `mutator.rs`, a predicted conflict surface
keyboard-navigation#1.7 · carried · autofocus on flush lives in `mutator.rs`, a predicted conflict surface
keyboard-navigation#1.8 · carried · the `FOCUS`/`FOCUSRING` states are set in `document.rs` and `node/node.rs`, both reached by the merge
keyboard-navigation#1.9 · carried · `:focus`/`:focus-within` match in `stylo.rs`, a merge surface that `53e9d81c6`'s `:open` edits
keyboard-navigation#1.10 · carried · focus-event bubbling is set in `blitz-traits/src/events.rs`, a merge surface
keyboard-navigation#2.1 · carried · focus reset on removal lives in `document.rs`, and its IME teardown meets `7fc9ee3af`'s IME enable
keyboard-navigation#2.2 · carried · the focus clear on a non-interactive click lives in `events/pointer.rs`, which `0980cce5f` edits
keyboard-navigation#2.3 · carried · button and select click focus live in `events/pointer.rs` (merge surface), and `stand_keyboard` re-reads them
keyboard-navigation#2.4 · carried · the double-click window lives in `events/pointer.rs` and reads the document clock, whose invariant the chunk re-reads
keyboard-navigation#2.5 · carried · the `summary` toggle lives in `events/pointer.rs`, and `53e9d81c6`'s `<details>` `:open` reaches the same element
keyboard-navigation#2.6 · carried · checkbox and radio click focus live in `events/pointer.rs`, a merge surface
keyboard-navigation#2.7 · carried · range click focus and pointerdown live in `events/pointer.rs`, a merge surface, and the scope keeps the range model as it is
keyboard-navigation#3.1 · carried · text-input key handling lives in `node/text.rs`, a merge surface reached by `a09f46792`, and `527394724` moves clipboard keys
keyboard-navigation#3.2 · carried · the range keys' changes mark the changed set and pass through `mutator.rs`'s range arm, which the scope keeps as it is
keyboard-navigation#3.3 · carried · the select model is one the scope keeps unchanged through the `mutator.rs` conflict, and `select_interaction` / `stand_act_select` run in the a11y leg
keyboard-navigation#3.4 · carried · Enter/Space activation is queued by `events/driver.rs` (a merge surface) and the chunk re-reads it through `stand_keyboard`
