@answer n=1 entity=keyboard-navigation#1.1 v=no
why: unkeyed_actionable still reads focusable through the one Readings rule, so the disabled-presence reading holds unchanged
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the focus clear in DocumentMutator::flush is blitz-dom engine code the session did not change
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the inert-only focus fixup limit is blitz-dom engine code the session did not change
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: residual res-6k56erh6 owns a focus limit the session did not touch
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: the Tab and Shift+Tab walks in blitz-dom are unchanged by the session
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the Tab-with-open-list close is blitz-dom event code the session did not change
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the select-list focus walk pinned in stand_keyboard is unchanged and that file is not reached
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputation in the blitz-dom mutator is unchanged
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: the disabled toggle of element states is blitz-dom element code the session did not change
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the disabled-presence reading and its walk gates are blitz-dom code the session left unchanged
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: select popup open and close rules are blitz-dom document and pointer code the session did not change
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input inner button tabindex is set in blitz-dom mutator code the session did not change
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus selection on flush is blitz-dom mutator code the session did not change
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: focus and blur element states are blitz-dom code the session did not touch
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the focus and focus-within matching in blitz-dom stylo code is unchanged
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur bubbling in blitz-traits events is unchanged
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: the focus reset on removal of the focused node is blitz-dom document code the session did not change
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area is blitz-dom pointer code the session did not change
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: list close on Escape, outside click and scroll is blitz-dom pointer and document code the session did not change
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: an accepted driver click still focuses the button because check_target only moves the refusal decision and keeps the same causes
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a flight booker option still refuses covered through check_target, the same cause the session reports for that option
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer over an open list row is blitz-dom pointer code the session did not change
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple-select row click is blitz-dom pointer code the session did not change
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: two driver clicks on one point still dispatch as before because check_target keeps the same refusal order and the clock is unchanged
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: the summary toggle on click is blitz-dom pointer code the session did not change
