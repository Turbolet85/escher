@answer n=1 entity=keyboard-navigation#1.1 v=yes mode=stale
why: dioxus_document.rs is reached whole, and DioxusDocument::unkeyed_actionable, which this line names as a reader of the disabled presence read, lives in that file beside the new record_reached and reached_handlers
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup in DocumentMutator::flush sits in blitz-dom mutator.rs, which the session does not reach, and the task leaves the fixup unchanged
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the is_inert-only limit sits in blitz-dom and key_press_after_container_lost_its_box.rs, neither reached, and the task leaves focus handling unchanged
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the residual res-6k56erh6 and its CARRY on the «A11y re-read» entry name another owner, so the handler-coverage CARRY is not this line
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: the Tab and Shift+Tab walk in blitz-dom keyboard.rs and document.rs is not reached, and the task leaves Tab orders unchanged
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the Tab-closes-list rule sits in blitz-dom keyboard.rs and select_popup.rs, neither reached, and select handling is unchanged
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the select walk pins in stand_keyboard.rs are not reached, and the task leaves select keyboard handling and stand_keyboard's stated orders unchanged
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: focusability recompute sits in blitz-dom mutator.rs and focusability_updates.rs, neither reached, and focusability is unchanged
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: form-control disabled states sit in blitz-dom element.rs, node.rs and focusability_updates.rs, none reached, and focusability is unchanged
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the disabled presence reads sit in blitz-dom element.rs, pointer.rs and keyboard.rs, none reached, and the line lists no stand assertion about Enter or Space dispatching a Click
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: select-popup opening and closing sit in blitz-dom document.rs, pointer.rs and select_popup.rs, none reached, and select handling is unchanged
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's inner button tabindex sits in blitz-dom mutator.rs, which the session does not reach
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus handling sits in blitz-dom mutator.rs and autofocus_attribute.rs, neither reached, and the task leaves focus handling unchanged
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: focus element states sit in blitz-dom document.rs and node.rs, which the session does not reach
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the :focus and :focus-within matching sits in blitz-dom stylo.rs, which the session does not reach
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur bubbling sits in blitz-traits events.rs, which the session does not reach
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on node removal sits in blitz-dom document.rs and interaction_state_teardown.rs, neither reached, and the teardown is unchanged
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: the click-clears-focus rule sits in blitz-dom pointer.rs, which the session does not reach
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: list closing on Escape and outside clicks sits in blitz-dom pointer.rs, document.rs and select_popup.rs, none reached, and select handling is unchanged
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: pointer click focus and form submit sit in blitz-dom pointer.rs and stand_keyboard.rs, neither reached, and pointer dispatch is unchanged
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: select click focus sits in blitz-dom pointer.rs and stand_act_select.rs, neither reached, and select handling is unchanged
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: row highlight on pointer over sits in blitz-dom pointer.rs and select_popup.rs, neither reached, and no read moved
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: multiple-select row clicks sit in blitz-dom pointer.rs, select_interaction.rs and stand_act_select.rs, none reached, and the option arm is unchanged
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing sits in blitz-dom pointer.rs and stand_virtual_clock.rs, neither reached, and the reached record only writes beside the dispatch
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary toggling sits in blitz-dom pointer.rs, which the session does not reach
