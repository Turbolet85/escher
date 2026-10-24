# a11y · picked accounting · group 1 of 2

## Picked
a11y-scope-summary#1.1 · carried · the audit's tokio fix and every pin move must leave the workspace AccessKit dependency and member standing
a11y-scope-summary#1.2 · not in scope · the adapter crate is outside upstream's delta and the tokio fix is a lock update, not a change to the crate
a11y-scope-summary#1.3 · carried · `06ecec886` touches `accessibility.rs` and blitz-dom's manifest, so the default `accessibility` feature must hold
a11y-scope-summary#1.6 · carried · the tokio advisory reaches through blitz-shell's default `accessibility` edge, which the fix must keep rather than drop
a11y-scope-summary#1.8 · not in scope · `NodeId::as_u64` interop is untouched by the delta
a11y-scope-summary#1.9 · carried · `286874cc1` moves every Dioxus package and syncs both Dioxus crates' manifests, so the feature's reach and the stand's adapter-free graph must hold
a11y-scope-summary#1.11 · carried · upstream's new blitz-tests targets build under that manifest, which must keep its `accessibility` features
a11y-scope-summary#1.12 · not in scope · the input fixture is outside the delta
a11y-strategy#1.1 · carried · `286874cc1` replaces upstream's side of `dioxus_document.rs`, so the override reaching the platform tree must survive the take
a11y-strategy#1.2 · carried · `mutator.rs` is a predicted conflict that holds the changed-set insert, whose contract must stay unchanged
a11y-strategy#1.3 · not in scope · the unhandled shell events are outside the delta
a11y-strategy#1.4 · not in scope · the shell's window-event forwarding is outside the delta
a11y-strategy#2.1 · carried · `06ecec886` touches `accessibility.rs`, so the HTML-AAM role mapping must read as before
a11y-strategy#2.2 · carried · the semantic-page assertion is the standing reading of that mapping on the merged builder
a11y-strategy#2.3 · carried · the builder's three closed-set state reads and the option's chosen state (CARRY 5) must survive the merge into `accessibility.rs`
a11y-strategy#2.4 · carried · a merge into `accessibility.rs` must keep one read per attribute, as the last sync did
a11y-strategy#3.1 · not in scope · the platform module selection in accesskit_xplat is outside the delta
a11y-strategy#4.1 · carried · `286874cc1` syncs dioxus-native-dom's `lib.rs`, whose feature gates the override, the snapshot and its forms
aria-patterns-roles#1.1 · not in scope · a landmark row of the mapping the HTML-AAM line carries, no delta commit naming it
aria-patterns-roles#1.2 · not in scope · a landmark row of the mapping the HTML-AAM line carries, no delta commit naming it
aria-patterns-roles#2.1 · carried · `role_from_name` lives in `accessibility.rs`, which `06ecec886` touches
aria-patterns-roles#2.2 · not in scope · it restates the state reads carried by the strategy's state line
aria-patterns-roles#2.3 · not in scope · a button mapping row the HTML-AAM line carries
aria-patterns-roles#2.4 · not in scope · a link mapping row the HTML-AAM line carries
aria-patterns-roles#2.5 · carried · the select's role is part of the select-by-id merge surface (CARRY 5) in `accessibility.rs`
aria-patterns-roles#2.6 · not in scope · a list and table mapping row the HTML-AAM line carries
aria-patterns-roles#2.7 · not in scope · a form-input mapping row the HTML-AAM line carries
aria-patterns-roles#2.8 · not in scope · a generic and text mapping row the HTML-AAM line carries
aria-patterns-roles#2.9 · carried · the option's exception from `display: none` is CARRY 5's tree surface, and the Stylo move can move computed `display`
keyboard-navigation#1.1 · carried · `e4ca5b9b5` edits `ElementData` and `mutator.rs` conflicts, so focusability by `inert` and `is_disabled` must hold (SC 2.4.3)
keyboard-navigation#1.2 · carried · the Tab walk is the SC 2.4.3 surface the stand walk re-proves on the merged engine
keyboard-navigation#1.3 · carried · the focusability recompute lives in `mutator.rs`, a predicted conflict
keyboard-navigation#1.4 · not in scope · restated by the one-way disabled-ness line, carried there
keyboard-navigation#1.5 · carried · disabled-ness keyed one way is CARRY 3's `disabled` row, whose ground `pointer.rs` and `node/element.rs` edits reach
keyboard-navigation#1.6 · not in scope · upstream's own file-input line in `mutator.rs`, no escher reading on it
keyboard-navigation#1.7 · not in scope · autofocus on flush is upstream's own reading, no delta commit naming it
keyboard-navigation#1.8 · not in scope · the focus element states are outside the delta
keyboard-navigation#2.1 · not in scope · focus reset on removal is upstream's behaviour, no escher check named by the scope
keyboard-navigation#2.2 · not in scope · the unmatched clear of focus is outside the delta
keyboard-navigation#2.3 · carried · `06ecec886` edits `events/pointer.rs`, where a button's and a select's click focus (CARRY 5) live
keyboard-navigation#2.4 · carried · the double click reads the document's clock, CARRY 4's surface in `events/pointer.rs`
keyboard-navigation#2.5 · not in scope · summary activation is outside the delta
keyboard-navigation#2.6 · not in scope · checkbox and radio clicks are outside the delta
keyboard-navigation#2.7 · carried · the range click and pointerdown arms live in `events/pointer.rs`, which the delta edits
keyboard-navigation#3.2 · not in scope · the range key arm in `events/keyboard.rs` is outside upstream's delta
keyboard-navigation#3.3 · carried · the select's keys and typeahead are CARRY 5's model, read by its two checks
keyboard-navigation#3.4 · carried · Enter and Space activation is SC 2.1.1, re-proven by `stand_keyboard` on the merged engine
keyboard-navigation#3.8 · carried · the labelable `<label>` click is CARRY 5's surface in `events/pointer.rs`
keyboard-navigation#3.9 · not in scope · the synthetic click builder is untouched by the delta's `node/node.rs` hunks
keyboard-navigation#4.4 · carried · `286874cc1` rewrites `dioxus_document.rs` and `events.rs` and deletes `keyboard_compat.rs`, so key events must still reach Dioxus
keyboard-navigation#4.5 · carried · focus events forward through the same synced `dioxus_document.rs`
keyboard-navigation#4.6 · not in scope · the `set_focus` TODO is a recorded gap, not a reading the take must keep
keyboard-navigation#4.9 · carried · `286874cc1` syncs `mutation_writer.rs` from Dioxus, where the falsy boolean-attribute predicate lives
keyboard-navigation#5.7 · carried · `286874cc1` edits `cells.rs`, whose Enter commit is read by `stand_non_lean_tasks`
keyboard-navigation#5.8 · not in scope · `crud.rs` is outside the delta, its keys re-read by the stand walk carried under the harness line
keyboard-navigation#7.1 · carried · the key-press focus check is a standing proof the take must leave passing
keyboard-navigation#7.2 · carried · `stand_keyboard` is the named reading of focus order and activation after the take
keyboard-navigation#7.3 · carried · the range check reads the pointer arms the delta's `events/pointer.rs` edits reach
keyboard-navigation#7.4 · carried · `select_interaction` and `stand_act_select` are the readings that show a moved select model
visual-design-verification#1.3 · carried · `scrolling.rs` is a predicted conflict and the scrollbar's hover and drag read there
visual-design-verification#1.4 · carried · `stand_contrast`'s rows are named as the reading that shows a move from `3954ecb85` or the Stylo move
visual-design-verification#1.5 · carried · the lint's contrast reading at boot is a standing proof on the merged engine
visual-design-verification#3.9 · not in scope · the caret paint in blitz-paint is outside the delta
visual-design-verification#8.1 · carried · `f102663b8` edits hit testing in `node/node.rs`, CARRY 3's `inert` ground
visual-design-verification#8.2 · not in scope · upstream's own visibility hit rule, no escher reading on it
visual-design-verification#8.3 · not in scope · upstream's own `pointer-events` hit rule, no escher reading on it
