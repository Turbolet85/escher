@answer n=1 entity=keyboard-navigation#1.1 v=no
why: the session reaches no focusability or Tab-walk path, and DioxusDocument::unkeyed_actionable, the one reached read this entity names, is re-read and unmoved, still reading disabled presence
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup in the mutator lives in blitz-dom, which the session does not reach, and it changes no inert or flush path
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the inert-only fixup limit lives in blitz-dom, which the session does not reach, and no focus or box path moved
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: residual res-6k56erh6 and its SC 2.4.3 re-assertion stay open and untouched, and the session neither moves nor closes that limit
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab dispatch in blitz-dom keyboard.rs is unreached, and the report leaves the Tab orders unchanged
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the choose-then-move Tab with a list open lives in blitz-dom and select_popup, neither reached, and no select key path changed
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the list-open focus hold is pinned in stand_keyboard.rs, which the session does not reach, and no select key path changed
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recompute lives in blitz-dom mutator.rs, unreached, and the session changes no tabindex, href or disabled path
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: disable-state toggling for button, input, select and textarea lives in blitz-dom element.rs and node.rs, unreached, and the session alters none of it
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the session adds a snapshot reader of is_disabled (clickable) but no keyboard or focus reader, and the presence rule itself is unchanged
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: the open-list disabled and inert behaviour lives in blitz-dom document.rs and select_popup, neither reached, and no select key path changed
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated tabindex=-1 lives in blitz-dom mutator.rs, which the session does not reach
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: the autofocus choice lives in blitz-dom mutator.rs and autofocus_attribute, neither reached, and the report leaves autofocus unmoved
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states are set in blitz-dom document.rs and node.rs, unreached, and no focus path moved
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within matching lives in blitz-dom stylo.rs, which the session does not reach, and it changes no focus state
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus, blur and focusin/focusout bubbling are defined in blitz-traits events.rs, which the session does not reach, and no event type changed
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removal lives in blitz-dom document.rs and interaction_state_teardown, neither reached, and the session alters no teardown path
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click to a non-interactive area lives in blitz-dom pointer.rs, which the session does not reach
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: list close on Escape, click, outside press and scroll lives in blitz-dom pointer.rs and document.rs, neither reached, and no select popup path moved
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: the click-focus and walk-matched rule lives in blitz-dom pointer.rs, unreached, and the Dioxus bridge's dispatch wrap runs only around handler calls, leaving focus as it was
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: select click focus and popup opening live in blitz-dom pointer.rs and stand_act_select, neither reached, and the report leaves select clicks unchanged
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: the pointer highlight over open list rows lives in blitz-dom pointer.rs and select_popup, neither reached, and no select path changed
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple-select row click lives in blitz-dom pointer.rs and select_interaction, neither reached, and the report moves no select click or key path
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: the double-click window lives in blitz-dom pointer.rs and the clock in stand_virtual_clock.rs, neither reached, and the session changes no click timing or clock
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary toggle focus lives in blitz-dom pointer.rs, which the session does not reach, and no details or summary path moved
