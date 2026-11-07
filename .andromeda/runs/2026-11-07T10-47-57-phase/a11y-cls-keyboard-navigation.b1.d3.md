@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability rule (tabindex, inert, disabled presence) governs the Tab walk to the key target and DioxusDocument::unkeyed_actionable, both of which the replayed attributes feed
@answer n=2 entity=keyboard-navigation#1.2 v=yes
why: the flush clears focus on a node under inert, and item 9 requires the replay to flush so this fixup runs once per applied buffer
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the limit concerns a focused control that stops showing without becoming inert, which the chunk neither handles nor tests
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: it is the owner of a residual limit and the stand's SC 2.4.3 re-assertion, neither touched by the chunk
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: Tab moving focus to the next node is the key-target precondition the inverted pins assert first (item 11)
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open list runs choose_current_and_close, but the chunk's Tab press has no open list
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on an open select concerns a select list the chunk neither opens nor changes
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: the cached focusability recompute on tabindex, href and disabled set or removal must run on the replayed attribute writes, since the Tab-focused key target's focusability depends on it
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: setting disabled on a form control removes its focusability and toggles its DISABLED state, and the falsy disabled removal of item 8 runs on both attribute paths the replay uses
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: disabled is keyed by presence, so a falsy Dioxus disabled value must remove the attribute rather than write a value, which the replay must keep (item 8)
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: the select list closing on inert, disabled, hidden or removal is a select-popup rule the scope does not name and the chunk does not exercise
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex is not reached by any chunk code or test
@answer n=13 entity=keyboard-navigation#1.13 v=yes
why: autofocus is applied on the mutator flush, which the buffer replay calls, so the replay's flush must keep the autofocus pass and its false-value rule
@answer n=14 entity=keyboard-navigation#1.14 v=yes
why: a render-time set_focus (item 2) is what sets the FOCUS and FOCUSRING states, and blur removes them, so the call the chunk adds carries this
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within matching is a style rule, and the chunk changes no style
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur bubbling concerns event dispatch the chunk neither adds nor reads
@answer n=17 entity=keyboard-navigation#2.1 v=yes
why: removing the focused node resets focus to body, so replayed removals must keep it and the tree's focus must not fall back to an unbuilt node (item 10)
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area concerns a pointer path the chunk does not drive
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open list on Escape, click or scroll concerns the select popup, which the chunk does not reach
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a click focusing a stand button through set_focus_to and reading as the tree's focus in both layout modes is a focus and tree check the chunk re-asserts (items 2 and 4, S2)
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focusing it and clearing its typeahead concerns a select click the chunk does not drive
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer over a row of an open list concerns the select popup, which the chunk does not reach
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: a click on a multiple select's row concerns select pointer handling the chunk does not change or drive
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: the double-click rule on the virtual clock concerns pointer clicks the chunk does not drive
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a details summary by click concerns a pointer path the chunk does not reach
