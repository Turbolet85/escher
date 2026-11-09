@answer n=1 entity=keyboard-navigation#1.1 v=no
why: the claim still holds on dioxus_document.rs, since unkeyed_actionable's result and the actionable-key rule are unchanged and the session only adds record_reached and the graph reader there
@answer n=2 entity=keyboard-navigation#1.2 v=no
why: the flush clear_focus on inert nodes is blitz-dom mutator behaviour, which the session does not reach and inert handling is unchanged
@answer n=3 entity=keyboard-navigation#1.3 v=no
why: the focused-but-hidden limit is a blitz-dom residual the session does not touch
@answer n=4 entity=keyboard-navigation#1.4 v=no
why: the limit's CARRY sits on the «A11y re-read» entry, not the Handler coverage entry the session discharges, so it stands unchanged
@answer n=5 entity=keyboard-navigation#1.5 v=no
why: the Tab and Shift+Tab walks in blitz-dom keyboard.rs and document.rs are unchanged and not reached
@answer n=6 entity=keyboard-navigation#1.6 v=no
why: closing an open list on Tab is blitz-dom select behaviour the session does not reach
@answer n=7 entity=keyboard-navigation#1.7 v=no
why: focus staying on the select while its list is open is pinned by stand_keyboard tests, a file the session does not reach, and those orders did not move
@answer n=8 entity=keyboard-navigation#1.8 v=no
why: cached focusability recomputation in blitz-dom mutator.rs and focusability_updates.rs is not reached by the session
@answer n=9 entity=keyboard-navigation#1.9 v=no
why: the button and input disabled-state toggles are blitz-dom behaviour the session does not change
@answer n=10 entity=keyboard-navigation#1.10 v=no
why: the session adds a stand Enter and Space click assertion but does not change what this entry says of the is_disabled reads or the Enter and Space dispatch rules
@answer n=11 entity=keyboard-navigation#1.11 v=no
why: disabled, inert or hidden select handling in open lists is unchanged blitz-dom behaviour and its cited tests are not reached
@answer n=12 entity=keyboard-navigation#1.12 v=no
why: the file input's generated tabindex -1 is a blitz-dom mutator rule the session does not reach
@answer n=13 entity=keyboard-navigation#1.13 v=no
why: autofocus on flush is a blitz-dom mutator rule and its test file is not reached by the session
@answer n=14 entity=keyboard-navigation#1.14 v=no
why: setting the FOCUS and FOCUSRING states in blitz-dom is unchanged and not reached
@answer n=15 entity=keyboard-navigation#1.15 v=no
why: :focus and :focus-within matching in blitz-dom stylo.rs is unchanged and not reached
@answer n=16 entity=keyboard-navigation#1.16 v=no
why: focus and blur bubbling in blitz-traits events.rs is unchanged and not reached
@answer n=17 entity=keyboard-navigation#2.1 v=no
why: focus reset on removing the focused node is blitz-dom document.rs behaviour the session does not change
@answer n=18 entity=keyboard-navigation#2.2 v=no
why: clearing focus on a click to a non-interactive area is blitz-dom pointer.rs behaviour, not reached by the session
@answer n=19 entity=keyboard-navigation#2.3 v=no
why: closing an open list on Escape, a second click, an outside click or a scroll is unchanged blitz-dom select behaviour, not reached
@answer n=20 entity=keyboard-navigation#2.4 v=no
why: a pointer click focusing a button is a blitz-dom pointer rule, and the session's keyboard-activation assertion neither moves it nor the cited stand_keyboard checks
@answer n=21 entity=keyboard-navigation#2.5 v=no
why: a click on a select and the refused driver click on a closed option are unchanged, and the session's refused-step assertion moves neither
@answer n=22 entity=keyboard-navigation#2.6 v=no
why: pointer highlight over open-list rows is blitz-dom select behaviour the session does not change
@answer n=23 entity=keyboard-navigation#2.7 v=no
why: the multiple-select row click through click_select_row is unchanged blitz-dom behaviour and its cited tests are not reached
@answer n=24 entity=keyboard-navigation#2.8 v=no
why: double-click timing on the document clock is unchanged, since the session's reached record and stand checks do not alter click timing
@answer n=25 entity=keyboard-navigation#2.9 v=no
why: the summary toggle in blitz-dom pointer.rs is unchanged and not reached
