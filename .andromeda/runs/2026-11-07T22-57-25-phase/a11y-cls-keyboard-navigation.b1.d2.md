@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability definition that DioxusDocument::unkeyed_actionable reads, which the scope names as the actionable-key rule
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the flush focus fixup on inert nodes is a focus rule the chunk does not change and no task item reads it
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the kept-focus limit on a hidden control is a focus rule the chunk does not change and is owned by a residual
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the limit's ownership by residual res-6k56erh6 is a deferred-work record the chunk's code and tests do not touch
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: the Tab walk over focusable nodes is the verb that carries the seven_guis Tab orders the chunk must keep in the shared tables
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with a select list open is select keyboard handling, which the task marks not needed and the chunk leaves unchanged
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on an open select is select keyboard handling, not a Tab order that the shared tables state
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: cached focusability recomputed on tabindex, href or disabled change is an input of the Tab orders the chunk re-asserts
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: setting disabled removes a button's focusability, an input of the stand Tab orders the chunk re-asserts
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: the disabled-presence reading that focusability carries and unkeyed_actionable reads, so the actionable-key rule binds it
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: open-list behaviour of disabled, inert or hidden selects is select keyboard handling the chunk leaves unchanged
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex is named by no scope unit and no shared table
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is a focus rule the chunk does not change and no shared table states
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states are focus state the chunk neither reads nor changes
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within matching is styling the chunk does not touch
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur not bubbling while focusin and focusout bubble is event dispatch the chunk does not change
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to the body when the focused node is removed is a focus rule the chunk does not change and no scope unit names
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click to a non-interactive area is pointer focus the task marks not needed
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open select list on Escape or an outside click is select behaviour the chunk leaves unchanged
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: a click focusing a button and submitting its form is pointer focus on click, which the task marks not needed
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focusing it and opening its list is select pointer handling the chunk leaves unchanged
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer highlighting a row of an open list is select pointer handling the chunk leaves unchanged
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: a click on a multiple select's row is select pointer handling the chunk leaves unchanged
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double click timing on the document clock is pointer timing the task marks not needed
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a summary to toggle its details is a behaviour that no scope unit names
