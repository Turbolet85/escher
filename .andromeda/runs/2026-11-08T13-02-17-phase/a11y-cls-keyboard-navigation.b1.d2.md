@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability rule (tabindex, natively focusable elements, inert and disabled form controls excluded) decides the focusable part of the listed set, item 7, with presence-keyed disabled per item 6
@answer n=2 entity=keyboard-navigation#1.2 v=yes
why: a focused node that becomes inert loses focus at flush, moving the focus the read reports under the inert exclusion of item 8
@answer n=3 entity=keyboard-navigation#1.3 v=yes
why: a focused control that display none drops from the snapshot keeps focus, so the read's focus field and listed set must handle a focused node outside the tree (item 8)
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: it records the owner of the 1.3 limit and a SC 2.4.3 re-assertion that the chunk neither re-reads nor re-asserts
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab move focus through key dispatch, and the read dispatches no key
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open list runs choose_current_and_close, a key dispatch the read never performs
@answer n=7 entity=keyboard-navigation#1.7 v=yes
why: focus stays on the select while its list is open, the state under which the owner-served rows of an open list are read (item 9, with the focus field of item 1); the Tab walk part is key handling
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: focusability is recomputed when tabindex, href or disabled change, the cache under the focusable part the read lists (item 7)
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: disabled on button, input, select and textarea removes focusability and sets the DISABLED state the read's disabled cause keys (items 5 and 6)
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: disabled-ness is keyed by presence of the attribute on button, input, select or textarea, which the read's disabled cause must match exactly (item 6)
@answer n=11 entity=keyboard-navigation#1.11 v=yes
why: a disabled row changes nothing and a disabled, inert, hidden or removed select opens or closes nothing, the owner sub-target rule the served rows must agree with (items 8 and 9)
@answer n=12 entity=keyboard-navigation#1.12 v=yes
why: the file input's generated inner button carries tabindex -1, a focusability instance the listed set reads under item 7, beside the file input value masked under item 11
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus is no input the read names; the read takes focus as the tree reports it and sets none
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states feed style and focus ring paint, and the read takes focus from the tree and paints nothing
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: focus, focus-within and focus-visible selector matching is style, and the read paints nothing
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur bubbling is event dispatch, which the read never performs
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to body on removing the focused node concerns a removal the read never runs and no unit the scope names
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area is pointer focus restoration, which the read performs no click for (not needed)
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: Escape, click, outside press, scroll and wheel closing an open list is dismissal dispatch, which the read never performs
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a disabled form control stops the walk and a click on a CRUD row still clears focus, the row click the listed CRUD rows must agree with (items 5 and 10)
@answer n=21 entity=keyboard-navigation#2.5 v=yes
why: an option of a closed single select takes no click and refuses covered, while an open list row takes a click, the owner-served rows and their covered cause the read reports (item 9)
@answer n=22 entity=keyboard-navigation#2.6 v=yes
why: pointer over an enabled open list row moves its highlight, and the highlighted row is in no snapshot, both named by item 9
@answer n=23 entity=keyboard-navigation#2.7 v=yes
why: a multiple select's row click chooses or toggles the row and a disabled row changes nothing, the rows the read lists from their own client rects (item 9)
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing is a click's timing, and the read performs no click
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: a summary's activation toggling details on click is click behaviour the read neither performs nor reports
