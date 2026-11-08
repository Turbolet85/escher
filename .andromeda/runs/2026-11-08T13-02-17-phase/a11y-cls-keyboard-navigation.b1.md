@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability rule with tabindex, natively focusable elements and inert and disabled form controls excluded decides the focusable part the read lists, and unkeyed_actionable reads it
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the flush focus fixup on a focused node that becomes inert moves focus only, and the read lists no focus and runs no step
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: a focused control that no longer shows keeps focus and takes keys, and the read neither lists focus nor dispatches keys
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the residual res-6k56erh6 and its SC 2.4.3 re-assertion are that residual's own check, which this chunk does not re-assert
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: the Tab walk moves focus by key dispatch, which the read does not perform
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab closing an open list is key handling, which the read does not exercise
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on an open select is Tab-walk and key behaviour, which the read does not exercise
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: the focusable part each listed node reports must follow tabindex, href and disabled set or removed after creation, so the cached focusability the read takes must be current
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: a disabled button, input, select or textarea loses its focusability, which the read's focusable part excludes
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: disabled-ness keyed by attribute presence on button, input, select and textarea is the rule the read's disabled cause must match exactly
@answer n=11 entity=keyboard-navigation#1.11 v=yes
why: a disabled row changing nothing and a disabled or hidden select opening nothing are the owner-served verbs and causes the read must agree with
@answer n=12 entity=keyboard-navigation#1.12 v=yes
why: the tabindex -1 on a file input's generated inner button is the value the focusability rule reads for that listed node
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on mount moves focus, which the read neither lists nor changes
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states feed focus styling and the focus field, and the read lists no focus
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the focus pseudo-classes are paint, and the read paints nothing
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus event bubbling is event dispatch, which the read does not perform
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removing the focused node is a removal step, and the read runs no step
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area is a pointer-click effect, and the read performs no click
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open list by Escape, a second click, an outside click or a scroll is pointer and key handling the read does not exercise
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a disabled form control stops the click walk while a disabled ancestor div does not, the disabled gate the read's disabled cause must match exactly
@answer n=21 entity=keyboard-navigation#2.5 v=yes
why: an option of a closed single select takes no click and a row of an open list takes one, the select-as-owner rule the owner-served node and its covered cause must agree with
@answer n=22 entity=keyboard-navigation#2.6 v=yes
why: a release over a row of an open list chooses that row, the owner-served row click the read's verb must agree with
@answer n=23 entity=keyboard-navigation#2.7 v=yes
why: a multiple select's row click and a disabled row changing nothing bind the owner-served rows the read lists
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the virtual clock is pointer timing, and the read performs no click
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary activation toggling a details element is an activation the read does not perform, and the summary's focusability is already held by the rule in 1.1
