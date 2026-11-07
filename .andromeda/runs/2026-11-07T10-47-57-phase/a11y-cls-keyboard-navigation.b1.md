@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: focusability reads disabled by presence and the inert read, and unkeyed_actionable reads them too; the replay must keep the falsy-value writes and actionable keys (task items 7, 8)
@answer n=2 entity=keyboard-navigation#1.2 v=yes
why: the flush's inert focus fixup is what the replayed flush must still run once per applied buffer (task item 9)
@answer n=3 entity=keyboard-navigation#1.3 v=yes
why: the fixup's inert-only reach bounds the replayed flush and the inert and hidden writes it carries, which must neither widen nor narrow it
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: owner record of residual res-6k56erh6 for that limit; the chunk changes no SC 2.4.3 check and names no check it re-asserts
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: Tab moves focus to the next or previous node, the precondition the inverted pins read before the spawned task's set_focus (task item 11)
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with a select list open runs the select popup's choose-and-close; neither the chunk's pins nor the replay open a list
@answer n=7 entity=keyboard-navigation#1.7 v=yes
why: names the stand_keyboard Tab-walk checks with the list open, which the chunk re-asserts through stand_*
@answer n=8 entity=keyboard-navigation#1.8 v=yes
why: cached focusability is recomputed when tabindex, href or disabled is set or removed, the attribute paths the replay must route through (task item 8)
@answer n=9 entity=keyboard-navigation#1.9 v=yes
why: setting disabled on a button removes its focusability and toggles DISABLED, carried by the disabled writes the replay routes (task item 8)
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: disabled is keyed by presence, so a falsy disabled value the replay writes must remove the attribute on both paths (task item 8)
@answer n=11 entity=keyboard-navigation#1.11 v=yes
why: a select made inert, disabled, hidden or removed while open closes its list, a reaction to the attribute writes and removal the replay carries (task item 8)
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex is the engine's own reaction to a file input, not a write the chunk names
@answer n=13 entity=keyboard-navigation#1.13 v=yes
why: autofocus is applied on the flush the replay must still run, and it must hold in both layout modes
@answer n=14 entity=keyboard-navigation#1.14 v=yes
why: set_focus from a render-time task must set the FOCUS and FOCUSRING states the snapshot reads (task items 2, 6)
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within are style matching, and the chunk changes no style or paint
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and focusin/focusout bubbling is event dispatch the chunk does not change and no named check reads
@answer n=17 entity=keyboard-navigation#2.1 v=yes
why: removing the focused node resets focus to the body, the fallback a render-time set_focus before the buffer applies must not leave on an unbuilt node (task item 10)
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: a pointer click on a non-interactive area clears focus; no pointer path or named check is in the chunk
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: list close on Escape, click or page scroll is the select popup's pointer and wheel rule; the chunk's MountedData scroll calls need only not panic
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a stand button click focuses it and reads focused and the tree's focus in both layout modes, witnessed by stand_keyboard checks the chunk re-asserts (task items 3, 10)
@answer n=21 entity=keyboard-navigation#2.5 v=yes
why: names the stand_act_select click-focus and select-row checks, which the chunk re-asserts through stand_*
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer over an open list's row moves its highlight; no pointer path or named stand check is in the chunk
@answer n=23 entity=keyboard-navigation#2.7 v=yes
why: names the stand_act_select multiple-select row checks, which the chunk re-asserts through stand_*
@answer n=24 entity=keyboard-navigation#2.8 v=yes
why: names stand_virtual_clock's double-click check, a stand_* check the chunk re-asserts, read on the harness clock
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: activating a summary toggles details and focuses it through pointer handling the chunk does not touch
