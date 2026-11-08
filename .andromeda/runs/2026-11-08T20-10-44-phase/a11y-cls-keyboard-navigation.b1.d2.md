@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusable predicate, with disabled read by presence on a form control, is what DioxusDocument::unkeyed_actionable reads, the actionable rule the coverage list keys on
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup at flush is focus behaviour the task marks not needed, and the chunk changes no focus
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: a focused control that no longer shows keeps focus, a focus-retention limit the chunk does not read
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: residual res-6k56erh6 and its SC 2.4.3 re-assertion belong to the focus-retention limit, not this chunk
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab moving focus is the focus order the task marks not needed
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab closing an open select list before it moves focus is select-popup key handling, not needed
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on a select while its list is open is select-popup focus, not needed
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputed on tabindex, href or disabled change is focusability, not needed
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: the DISABLED and ENABLED states of button, input, select and textarea are the enabled value the snapshot reads and the driver refusal tests
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: the presence reading of disabled that gives enabled Some(false) is the value the driver's disabled refusal and the replayed actions answer depend on
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: a disabled row or an inert, disabled or hidden select in an open list is select-popup behaviour, not needed
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: tabindex -1 on a file input's generated inner button is focus order, not needed
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is focus order, not needed
@answer n=14 entity=keyboard-navigation#1.14 v=yes
why: the FOCUS and FOCUSRING element states set on focus and removed on blur are the focus value the snapshot model reads
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within CSS matching is styling, and the chunk paints nothing
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur not bubbling is event dispatch, which the coverage reads from recorded checks and does not compute
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to the body on removing the focused node is focus restoration, not needed
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: a click on a non-interactive area clearing focus is pointer focus behaviour the chunk does not change
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open list on Escape, outside click, second click or scroll is select-popup behaviour, not needed
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: click focusing a button and the engine's walk gate on disabled controls are engine pointer focus the chunk does not change, and the driver refusal is not this entity's rule
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: select click focusing, opening and closing the list, and the covered refusal are select-popup behaviour the chunk does not change
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer highlight and release on an open list's row is select-popup behaviour, not needed
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: a multiple select's row click and its disabled-row no-op are select interaction the chunk does not change
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the virtual clock is a pointer gesture the fresh replay reproduces, and the scope names no gesture
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a summary to toggle details and focus it is pointer activation, not needed
