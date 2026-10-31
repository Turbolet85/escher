# a11y-plan · picked accounting · group 2 of 5

## Picked
keyboard-navigation#1.1 · proposed · its known limit says the focus fixup is not built and a node made inert keeps focus; the report's F3 builds it, `focus-fixup` changes this line, and the fixup's fact goes beside it
keyboard-navigation#1.2 · not touched · `keyboard.rs` and `document.rs` gain select and transform lines alone, and `stand_keyboard`'s walks of the stated sequences read green, so the Tab and Shift+Tab claim holds on both files
keyboard-navigation#1.3 · not touched · `mutator.rs`'s reported lines record a select and clear focus at flush, with no arm joining the attribute writes, so the recompute on `tabindex`, `href` and `disabled` holds
keyboard-navigation#1.4 · not touched · `node.rs`'s one reported hunk is `hit_inner`, and the report has `disabled` reading as before, so the form-control claim holds
keyboard-navigation#1.5 · proposed · the form-control key stands, and the fact of an option's one disabled rule under its group, `option_disabled`, goes beside it
keyboard-navigation#1.6 · not touched · the report's by-line list of `mutator.rs` adds nothing in the file input's block and this chunk holds no merge, so the inner button's `tabindex` holds
keyboard-navigation#1.7 · not touched · the fixup added at flush clears only a focused node reading inert and an inert autofocus candidate is already skipped, so the autofocus claim holds on `mutator.rs`
keyboard-navigation#1.8 · not touched · F3 clears focus through the standing `clear_focus` with no DOM `blur`, and `document.rs`'s and `node.rs`'s reported lines leave the focus and blur states alone, so the claim holds
keyboard-navigation#2.1 · not touched · the report's fixup is a separate path for a node made inert and `document.rs`'s reported lines leave removal alone, so the removal claim holds
keyboard-navigation#2.2 · not touched · the new `option` arm takes a `multiple` select's rows alone and the report keeps a closed select's option without a click arm, so a click on a non-interactive area still clears focus
keyboard-navigation#2.3 · proposed · `handle_click` gains an `option` arm, so «an option is not clicked» narrows to a closed select's option at `option-click`, and the click rule's two facts go beside it
keyboard-navigation#2.4 · not touched · the report adds no wall-clock read and `pointer.rs`'s reported lines leave the double-click gate alone, so the claim holds
keyboard-navigation#2.5 · not touched · `open_state`'s click and Enter rows read a summary flipping its `details`, so the claim holds on `pointer.rs`
keyboard-navigation#2.6 · proposed · «no test asserts `:open`» is retired by `open_state` 4, the report's `no-assert-gap` naming this line
keyboard-navigation#2.7 · proposed · still true of `input`, focus and state, and the engine's `change` after `input` goes beside it as a new fact
keyboard-navigation#2.8 · not touched · `text_range_no_change_event` reads a range's pointerdown firing `input` and no `change`, and `pointer.rs`'s reported lines leave the range arms alone, so the claim holds
keyboard-navigation#3.2 · not touched · `keyboard.rs`'s reported lines are the select's and `text_range_no_change_event` reads a range's key firing `input` and no `change`, so the range keys' claim holds
