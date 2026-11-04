@answer n=1 entity=keyboard-navigation#1.1 v=yes
why: the focusability and Tab-walk rule decides whether the probe's stand button takes focus, and unkeyed_actionable reads its disabled presence, which needs item 2 names
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup at flush; the probe makes no node inert, and the inert focus fixup is not_needed item 6
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: a focused control that no longer shows keeps focus; hidden content is not_needed item 7 and the probe hides nothing
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the residual ledger line for the inert-focus limit; it is not the click reading that needs item 2 names
@answer n=5 entity=keyboard-navigation#1.5 v=yes
why: the Tab walk moves focus onto the button the probe presses Enter or Space on, reached through the keyboard press path (needs item 6)
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: Tab with an open list closes it first; select list behaviour is not_needed item 2
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus held on a select while its list is open; select list behaviour is not_needed item 2
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability on tabindex, href or disabled set or cleared; the probe writes no attribute path
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: disabled states on button, input, select and textarea, and focusability lost on a set disabled; the probe writes no disabled attribute
@answer n=10 entity=keyboard-navigation#1.10 v=yes
why: Enter and Space activation of the focused control is gated by ElementData::is_disabled, and that Enter and Space dispatch is named by needs item 7
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: an open select list's disabled rows and disabled, inert or hidden select; select list behaviour is not_needed item 2
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex; the probe reads no file input
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush under the feature; the probe names no autofocus element and sets no focus by it
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: FOCUS and FOCUSRING element states on focus and blur; the probe reads focus off accessibility_tree (needs item 3), not element states
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-visible matching in styling; the probe reads no style and no focus outline (Cells' focus outline is not_needed item 10)
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur not bubbling, focusin and focusout bubbling; the probe adds no focus listener
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to body when the focused node is removed; the probe removes no node
@answer n=18 entity=keyboard-navigation#2.2 v=yes
why: a click on a CRUD row, which is not focusable, clears focus through this rule, and needs item 8 names the row's click
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: Escape, second click, outside click and scroll closing an open list; select list behaviour is not_needed item 2
@answer n=20 entity=keyboard-navigation#2.4 v=yes
why: a click on a stand button focuses it and Tab continues from it, so the probe's Enter and Space dispatch to the focused button needs that focus (needs items 6 and 7)
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focuses it and opens or closes its list; select list behaviour is not_needed item 2
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: a pointer over an enabled row of an open list moves its highlight; select list behaviour is not_needed item 2
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: a click on a multiple select's row; the CRUD list is a listbox, not a select element, and select list behaviour is not_needed item 2
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: the double-click reading on one point; the probe makes no repeated click on one point and reads no text selection
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: a first summary of a details element toggles and focuses on activation; the probe uses no details element
