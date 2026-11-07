@answer n=1 entity=keyboard-navigation#1.1 v=no
why: the session keeps focusability, the disabled presence rule and unkeyed_actionable unchanged, so the claim holds on the changed dioxus_document.rs
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the inert focus fixup at flush is unchanged and the replay applies the same mutations in the same order
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the inert focus fixup and its ratified limit are unchanged by the buffered render
@answer n=4 entity=keyboard-navigation#1.4 v=yes mode=add-to
why: carries F1, the new limit that a task polled during a Dioxus render acts on the document as the previous render left it, recorded as an open residual row (target: next)
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab key handling in blitz-dom events is an engine key rule the session leaves unchanged
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: the open-list Tab close is select keyboard handling, which the session leaves unchanged
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on an open select and the stand_keyboard walk tests are unmoved by the session
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputation is an engine focusability rule the session leaves unchanged
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: button, input, select and textarea disabled-ness and its focusability effect are unchanged by the session
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the presence-based disabled rule and the falsy boolean-attribute bridge are untouched, the replay only reorders nothing
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: open, close and disabled-row rules of select lists are unchanged by the session
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex is a blitz-dom mutator rule the replay leaves unchanged
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is an engine rule, and the replay applies the same mutations in the same order
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: the FOCUS and FOCUSRING element states on focus and blur are unchanged by the session
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within matching is stylo and style code, which the session does not move
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and focusin event bubbling lives in blitz-traits events, which the session does not touch
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset to the body on removal is an engine focus rule the session leaves unchanged
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click on a non-interactive area is an engine pointer rule the session leaves unchanged
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: select list closing on Escape, outside clicks and scroll is unchanged by the session
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: click focus on buttons is an engine focus rule and the stand's click-focus inventory is unmoved, so no list of the three focus readings agreeing gains a fact here
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focusing it through set_focus_to is an unchanged engine rule
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: pointer highlight over an open list's rows is an unchanged engine pointer rule
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: click_select_row on multiple selects is an unchanged engine rule and the select tests are unmoved
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the document clock and the harness VirtualClock are unchanged by the session
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary toggling and focus is an engine pointer rule the session leaves unchanged
