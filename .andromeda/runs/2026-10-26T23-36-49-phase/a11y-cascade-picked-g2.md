# a11y · picked accounting · group 2 of 3

## Picked
keyboard-navigation#1.1 · carried · its disabled-by-presence and inert focusability rule is witnessed by the `stand_keyboard` and `inert_subtree` rows (v020-03) that the refactored requirement sweep re-asserts inside stand-repeat
keyboard-navigation#1.2 · not in scope · the engine's Tab walk is a used component, left unchanged, and the scope does not name it; no sweep row cites it
keyboard-navigation#1.3 · not in scope · the recompute of cached focusability on mutation is engine code the chunk does not reach, and its `focusability_updates` witness is no sweep row
keyboard-navigation#1.4 · not in scope · which elements can be disabled is an engine state the chunk only uses, and its `focusability_updates` witness is no sweep row
keyboard-navigation#1.5 · carried · its presence-on-a-form-control reading is witnessed by `stand_keyboard`'s disabled-attribute test, a v020-03 row the sweep re-asserts inside stand-repeat
keyboard-navigation#1.8 · not in scope · the focus element states are an engine behaviour the chunk neither reaches nor re-asserts
keyboard-navigation#2.3 · not in scope · a pointer click's focus is a used engine behaviour the scope does not name, and no edit reaches `handle_click`
keyboard-navigation#3.3 · not in scope · the select's key and typeahead model is engine code left unchanged, and the entity cites only engine source, no sweep row
keyboard-navigation#3.4 · not in scope · Enter and Space activation is 0.1.0 engine behaviour with no 0.2.0 sweep row, and no engine code changes
keyboard-navigation#4.9 · not in scope · the Dioxus bridge's falsy-attribute removal is untouched, and its `dioxus_falsy_*` witnesses are no sweep row
keyboard-navigation#5.8 · carried · its pin `accessibility_active_descendant_unread` is a v020-03 witness the sweep re-asserts as a target stand-repeat names
keyboard-navigation#7.2 · carried · it records `stand_keyboard`'s disabled-attribute and inert-subtree rows, v020-03 witnesses the sweep re-asserts inside stand-repeat
keyboard-navigation#7.4 · carried · it records `select_interaction` in stand-repeat's named `--test` list and `stand_act_select` by the `stand_` prefix, both of which the sweep re-asserts for v020-08
keyboard-navigation#7.5 · not in scope · no sweep row names the flight-type arrows test or `stand_keyboard`'s test count, and `stand_keyboard` is not edited
visual-design-verification#1.4 · carried · a helper factored out of `findings` must keep `text_contrast`'s ratio, its required ratios and the exempt class, with no second formula
visual-design-verification#1.5 · carried · it states the lint's `contrast` reading and `stand_lint`'s boot and planted readings, which are re-read on the refactored `lint.rs`
visual-design-verification#1.6 · carried · the flight booker's boot `overlap` and `stand_lint`'s `[0, 1, 0, 0]` must still read after `findings` is factored, the residual kept as it is
visual-design-verification#1.7 · not in scope · the select's user-agent and author contrast pair is a `stand_contrast` row, and that check and the stand's CSS are neither edited nor re-read
visual-design-verification#8.1 · carried · the snapshot's exclusions and its select-option exception bound the lint's walk, whose findings must not change
