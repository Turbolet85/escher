@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability rule the Tab walk reads decides whether the input the test Tab-focuses is reached, and DioxusDocument::unkeyed_actionable reads disabled (items 7, 11)
@answer n=2 entity=keyboard-navigation#1.2 v=yes
why: the flush fixup that gives up focus under inert is the one item 9 says the replayed buffer's flush must still run once per applied buffer
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the hidden-node limit (a focused display-none control keeps focus) is outside the fixup scope, which names inert only, and the chunk changes no visibility rule
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: it records the ownership of that hidden-node limit by residual res-6k56erh6; the chunk neither touches the limit nor records the residual
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: Tab moving focus to the next focussable node is the key-target precondition the inverted pins assert first (item 11)
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open list runs select choose-and-close; the chunk opens no list and changes no select key handling
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the focus walk with a select's list open is select-popup behaviour the chunk does not reach and its re-asserted behaviours (document, ids, snapshot and diff, mounted order, layout identity) do not cover
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: the replayed set-attribute writes must go through the path that recomputes cached focusability on tabindex, href or disabled, which Tab and the falsy disabled removal depend on (items 8, 11)
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: the falsy disabled removal on both MutationWriter attribute paths must still toggle the DISABLED and ENABLED states of the form controls (item 8)
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: falsy Dioxus disabled values are removed as attributes, so the presence reading decides whether a replayed control is disabled and unfocusable (item 8)
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: a select made inert, disabled, hidden or removed while open closes its list; the chunk opens no list and changes no select handling (not-needed item 4)
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex is not reached by the buffer and no chunk test touches a file input
@answer n=13 entity=keyboard-navigation#1.13 v=yes
why: autofocus is applied on the flush the replayed buffer must still run once per applied buffer, so a focus change made at flush must reach the changed set (items 3, 9)
@answer n=14 entity=keyboard-navigation#1.14 v=yes
why: the FOCUS and FOCUSRING states set on focus are what the named input's focus reads back as in the engine, the tree and the snapshot (items 2, 6)
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within styling; the chunk changes no style, colour or paint (not-needed item 1)
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur non-bubbling with focusin and focusout bubbling is event dispatch the chunk neither changes nor asserts
@answer n=17 entity=keyboard-navigation#2.1 v=yes
why: removing a focused node resets focus to the body; a replayed removal of the focused node runs this fallback, the case item 10 guards so the tree's focus never lands on an unbuilt node
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click to a non-interactive area is pointer handling the chunk does not touch and its tests do not click
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: Escape, click, outside press and scroll closing an open list is select-popup behaviour the chunk does not reach
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: the stand's tree focus after an accepted click on fx-before, in both layout modes, is the tree-focus value item 4 and item 10 check on the headless stand
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focusing it and clearing its typeahead is select pointer handling the chunk does not change
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: pointer highlight over an open list's rows is select-popup behaviour the chunk does not change or test
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: a click on a multiple select's row is select pointer handling the chunk does not change
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the VirtualClock is pointer handling; the chunk's tests use a key press and set_focus, not two clicks
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: a click on a summary toggling its details is pointer handling the chunk does not change
