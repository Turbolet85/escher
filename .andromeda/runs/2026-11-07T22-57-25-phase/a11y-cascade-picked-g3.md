@account n=1 entity=keyboard-navigation#1.1 v=not_in_scope
why: the focusability rule and its focus fixup, which the chunk leaves alone and its click reading does not read, so it stays a re-check every change owes
@account n=2 entity=keyboard-navigation#1.5 v=not_in_scope
why: the Tab and Shift+Tab walk, which the chunk does not change; the stand's Tab orders only have to keep holding, a re-check every change owes
@account n=3 entity=keyboard-navigation#1.6 v=not_in_scope
why: Tab choosing and closing an open list, a key behaviour the chunk does not reach
@account n=4 entity=keyboard-navigation#1.7 v=not_in_scope
why: focus staying on the select while its list is open, a focus behaviour the chunk does not change or re-measure
@account n=5 entity=keyboard-navigation#1.8 v=not_in_scope
why: recomputing cached focusability on attribute writes, an engine mutator path the chunk only uses and does not rely on
@account n=6 entity=keyboard-navigation#1.9 v=carried
why: names which controls can be disabled, and a disabled control's click is gated, which the «reacts to a click» reading has to account for
@account n=7 entity=keyboard-navigation#1.10 v=carried
why: the disabled-ness key gates `ClickTarget::Disabled` and `handle_click`'s walk, so it decides whether a disabled control reacts to a click, which the new field must report
@account n=8 entity=keyboard-navigation#1.11 v=carried
why: a disabled row or a disabled, inert, hidden or empty select reacts to no click, a fork over what the new field reports
@account n=9 entity=keyboard-navigation#1.16 v=not_in_scope
why: how focus events bubble, which the graph does not read and the chunk does not change
@account n=10 entity=keyboard-navigation#2.4 v=carried
why: a native button's click focuses it and ends the walk with no app handler involved, a fork over whether the «reacts to a click» reading counts native activation
@account n=11 entity=keyboard-navigation#2.5 v=carried
why: a select's click and the refused click on an option of a closed single select decide which select nodes react to a click
@account n=12 entity=keyboard-navigation#2.7 v=carried
why: a multiple select's row reacting to a click through `click_select_row` is a node the new field has to classify
@account n=13 entity=keyboard-navigation#2.9 v=carried
why: a summary reacts to a click natively, toggling its details, a fork over the new field's reading
@account n=14 entity=keyboard-navigation#2.10 v=carried
why: a summary's click toggling `open` through the mutator is the native click reaction the new field's reading must account for
@account n=15 entity=keyboard-navigation#2.11 v=carried
why: checkbox and radio clicks toggle state and dispatch `input` natively, a fork over whether the new field counts them as reacting to a click
