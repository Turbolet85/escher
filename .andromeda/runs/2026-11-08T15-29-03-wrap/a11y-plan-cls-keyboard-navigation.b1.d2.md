@answer n=1 entity=keyboard-navigation#1.1 v=no
why: the session changed no focusability rule, and unkeyed_actionable still reads focusable through the one Readings rule while the cited blitz-dom files are not reached
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the flush focus fixup lives in blitz-dom mutator.rs, which the session did not reach
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the fixup's inert-only reading lives in blitz-dom, which the session did not change
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: residual res-6k56erh6 is not named by the session and its claim is unchanged
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: the Tab and Shift+Tab walks in blitz-dom keyboard.rs and document.rs are unchanged
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the Tab handling with an open list in blitz-dom keyboard.rs is unchanged
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the select focus hold and the stand_keyboard walks are unchanged, as the session untouched list states
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: focusability recompute on tabindex, href and disabled in blitz-dom mutator.rs is unchanged
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: disabled toggling of form controls in blitz-dom element.rs is unchanged
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: disabled-ness reads ElementData::is_disabled in blitz-dom, which the session did not change
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: the open-list rules for disabled, inert and hidden selects in blitz-dom are unchanged
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex in blitz-dom mutator.rs is unchanged
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: the autofocus flush rule in blitz-dom mutator.rs is unchanged
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: focusing and blurring element states in blitz-dom are unchanged
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the :focus and :focus-within matching in blitz-dom stylo.rs is unchanged
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and focusin bubbling in blitz-traits events.rs is unchanged
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removing the focused node in blitz-dom document.rs is unchanged
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clicking a non-interactive area clearing focus in blitz-dom pointer.rs is unchanged
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open list on Escape, outside clicks and scroll in blitz-dom is unchanged
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: the accepted driver click on fx-before still focuses it, the session changed only the refusal decision through check_target, and the pointer focus path is unchanged
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: the driver click on a closed select option is still refused covered under the shared check_target, the cause unchanged, and the select focus path is unchanged
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: pointer hover and release on rows of an open list in blitz-dom pointer.rs is unchanged
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple select row click path click_select_row in blitz-dom is unchanged
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: the double click on the document clock is unchanged, since check_target decides only refusals before the accepted click path
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary toggling and focusing in blitz-dom pointer.rs is unchanged
