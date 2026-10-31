# Classified — a11y-plan · screen-reader-support.b1

## Answers
screen-reader-support#1.1 · yes · stale · accessibility.rs gains an insert at 238-242, so the TextRun citation at 269-273 moved
screen-reader-support#1.2 · no · the aria-label naming lines 158-171 sit above the accessibility.rs insert at 238-242 and the naming rule is unedited; its tests are unreached
screen-reader-support#1.3 · yes · stale · document.rs is reached and its citations 684-722 and 3499-3513 are not shown above its inserts, so they may have moved
screen-reader-support#1.4 · yes · add-to · the open list moves an open row's bounds to the painted row (closed_select_of None, select_list_bounds read first), so the closed-select clause needs the open case; F15 the tree writes expanded from the list's open state, F18 an option's or group's label reaches an open list only as painted pixels and the snapshot name, F19 what an AT reads of an open list is unmeasured, recorded unrunnable-here
screen-reader-support#1.5 · yes · stale · stand_act_select.rs is reached and its citation 320-361 is not shown above its inserts; a multiple select's size is now a height hint, its rows still read their own client rects
screen-reader-support#1.6 · no · evt.values reads dioxus_document.rs, which is unreached, and the session changes no multiple select value read
screen-reader-support#2.1 · no · the 14 controls' roles, names and author ids sit in unreached files and the session changes none of them
screen-reader-support#3.1 · yes · add-to · F16 the highlighted row is in no snapshot and F17 NodeState gained no expanded field; stand_snapshot_state citations after line 121 also moved
screen-reader-support#3.2 · yes · contradict · a single select now writes expanded from the open state (F15), so "under any other role a native control writes nothing" no longer holds
screen-reader-support#4.1 · no · the check's files and the stand's actionable tests are unreached, and the session adds no DOM element, role or key
screen-reader-support#4.2 · no · cells-grid and residuals.md are unreached and the session edits no Cells element
screen-reader-support#4.3 · yes · stale · stand_contrast.rs is reached and every citation from line 12 on moved, so 670-700 moved
screen-reader-support#5.1 · yes · stale · stand_snapshot_state.rs citations 369-409, 411-461 and 463-498 sit after line 121, which the change moved
screen-reader-support#6.1 · yes · stale · accessibility.rs:268 sits after the 238-242 insert, so the citation moved
screen-reader-support#6.2 · no · the root and focus lines 13, 104-110 and 115-123 sit above the 238-242 insert and the focus rule is unedited
screen-reader-support#6.3 · no · accessibility_roles.rs is unreached and the role-mapping goal is unchanged
screen-reader-support#7.1 · no · accesskit_xplat is outside the change and the adapter is unedited
screen-reader-support#8.1 · no · blitz-shell is outside the change
screen-reader-support#8.2 · yes · stale · dioxus-native-dom/src/lib.rs is reached and its lines 7-10 are not shown to hold
screen-reader-support#9.1 · no · the cited lang_attribute test is unreached and the claim names no edited file
screen-reader-support#9.2 · yes · stale · default.css is reached and its citations past the select-list insert moved; 14-36 are not shown above it
screen-reader-support#9.3 · no · the 11 fixtures under examples/assets are unedited
screen-reader-support#9.4 · no · the pages under examples/assets and examples/preact are unedited
screen-reader-support#10.1 · no · the s02 fixtures are unedited and the absence stands
screen-reader-support#11.1 · no · the github fixture under examples/assets is unedited
