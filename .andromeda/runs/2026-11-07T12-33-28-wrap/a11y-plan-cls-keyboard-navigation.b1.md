@answer n=1 entity=keyboard-navigation#1.1 v=no
why: focusability, the disabled presence reading, the inert focus fixup and unkeyed_actionable are unchanged by the session, which replays the same mutations in the same order.
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the flush-time clear of focus on inert nodes in blitz-dom mutator.rs is unchanged, and that file is not among the files the session reaches.
@answer n=3 entity=keyboard-navigation#1.3 v=yes mode=add-to
why: fact F1 joins this limit record: a task polled during a Dioxus render acts on the document as the previous render left it, so set_focus on a node made in the same render does not focus it.
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the owner of the inert-fixup limit, residual res-6k56erh6, is not one the session re-says, and its citation is unchanged.
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: Tab and Shift+Tab movement and focus_next_node and focus_prev_node are unchanged, and their files are not reached.
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: a list's Tab close in keyboard.rs and select_popup.rs is unchanged, and neither file is reached.
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: the select focus-stays-on-select walk and its stand_keyboard pins are unchanged, and the session does not reach those tests.
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: focusability recomputed on tabindex, href or disabled change is unchanged, and the session does not reach those mutator paths or tests.
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: the disabled states of button, input, select and textarea are unchanged, and the focusability_updates tests are not reached.
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the presence keying of disabled and its pointer and keyboard gates are unchanged by the session, and none of those files is reached.
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: the open and close rules for disabled, inert and hidden selects are unchanged, and their select_popup tests are not reached.
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated inner button tabindex lives in blitz-dom mutator.rs, which the session does not reach.
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is applied through the same mutations in the same order, and its mutator.rs code and test are not reached.
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: setting and clearing the FOCUS and FOCUSRING element states is unchanged, and document.rs and node.rs are not reached.
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: the :focus and :focus-within matching in stylo.rs is unchanged, and that file is not reached.
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur not bubbling while focusin and focusout bubble is set in blitz-traits events.rs, which the session does not change.
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: removing a focused node resets focus to body in blitz-dom document.rs, which the session does not change.
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: a click on a non-interactive area clearing focus in pointer.rs is unchanged, and that file is not reached.
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: the close paths of an open list are unchanged, and the pointer.rs, document.rs and select_popup files are not reached.
@answer n=20 entity=keyboard-navigation#2.4 v=yes mode=add-to
why: the probe's two pins now also assert engine focus, the snapshot's focused node and the tree's focus agreeing on focus-target after a spawned task's set_focus in both layout modes, a focus-agreement site this click-focus entry lists and must carry.
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select focusing it through set_focus_to is unchanged, and pointer.rs is not reached.
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: the pointer highlight of an open list's row and the release choice are unchanged, and that file is not reached.
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the click_select_row path for a multiple select is unchanged, and its pointer.rs and select tests are not reached.
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the document clock is unchanged, and the stand_virtual_clock test is not reached.
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: summary activation toggling and focusing in pointer.rs is unchanged, and that file is not reached.
