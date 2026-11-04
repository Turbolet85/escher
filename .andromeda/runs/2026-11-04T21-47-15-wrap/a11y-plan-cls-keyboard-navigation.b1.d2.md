@answer n=1 entity=keyboard-navigation#1.1 v=no
why: its focusability rule lives in packages/blitz-dom/src/node/element.rs, which the session did not reach
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: its inert focus fixup lives in packages/blitz-dom/src/mutator.rs, which the session did not reach
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the focus-fixup limit lives in blitz-dom code and its keyboard test, none of them reached by the session
@answer n=4 entity=keyboard-navigation#1.4 v=yes mode=stale
why: the session moved residuals to store rows and no .andromeda/residuals.md file stands, so the path this line cites is stale
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab dispatch lives in packages/blitz-dom/src/events/keyboard.rs, which the session did not reach
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the choose-and-close Tab rule lives in blitz-dom keyboard.rs and select_popup.rs, neither reached by the session
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the open-list focus rule is pinned by stand_keyboard.rs tests the session did not touch
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability lives in packages/blitz-dom/src/mutator.rs and focusability_updates.rs, neither reached by the session
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: the disabled form-control states live in blitz-dom element.rs and node.rs, which the session did not change
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the disabled-ness keying lives in blitz-dom element.rs and pointer.rs, neither reached by the session
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: the open-list disabled and inert rules live in document.rs and select_popup.rs, neither reached by the session
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's inner button tabindex lives in packages/blitz-dom/src/mutator.rs, which the session did not change
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: the autofocus rule lives in mutator.rs and autofocus_attribute.rs, neither reached by the session
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the focus element states live in document.rs and node.rs, which the session did not change
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the focus pseudo-class matching lives in packages/blitz-dom/src/stylo.rs, which the session did not change
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: the focus and bubbling event rule lives in packages/blitz-traits/src/events.rs, which the session did not change
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removal lives in document.rs and interaction_state_teardown.rs, neither reached by the session
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: the click-clears-focus rule lives in packages/blitz-dom/src/events/pointer.rs, which the session did not change
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: the list close rules live in pointer.rs, document.rs and select_popup.rs, none reached by the session
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: the button click focus rule lives in pointer.rs and stand_keyboard.rs, neither reached by the session
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: the select click rule lives in pointer.rs and stand_act_select.rs, neither reached by the session
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: the row-hover highlight rule lives in pointer.rs and select_popup.rs, neither reached by the session
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple-select click rule lives in pointer.rs and select_interaction.rs, neither reached by the session
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: the double-click rule lives in pointer.rs and stand_virtual_clock.rs, neither reached by the session
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: the summary toggle rule lives in packages/blitz-dom/src/events/pointer.rs, which the session did not change
