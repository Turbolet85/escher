@answer n=1 entity=screen-reader-support#1.1 v=no
why: TextRun and label-by-text reading live in blitz-dom accessibility.rs, which the session does not reach
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label naming is blitz-dom accessibility.rs and its tests, none reached, and roles and names read as before
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label association lives in blitz-dom accessibility.rs and document.rs, neither reached
@answer n=4 entity=screen-reader-support#1.4 v=yes mode=stale
why: the session inserts NodeState.clickable in snapshot.rs above the cited lines 155-167, 257-263 and 276-281, moving them; the select reading itself is unchanged
@answer n=5 entity=screen-reader-support#1.5 v=no
why: expanded comes from blitz-dom accessibility.rs and select_popup tests, neither reached
@answer n=6 entity=screen-reader-support#1.6 v=yes mode=stale
why: docs/capabilities.md, reached by the removal of the click-field not-built line, is cited at lines 230-232, which may have moved
@answer n=7 entity=screen-reader-support#1.7 v=no
why: it states an unmeasured assistive-technology reading and cites only chunk reports, with the adapter untouched
@answer n=8 entity=screen-reader-support#1.8 v=no
why: its cited tests are stand_act_select.rs and blitz-dom, not reached, and the multiple-select row bounds reading is unchanged
@answer n=9 entity=screen-reader-support#1.9 v=yes mode=stale
why: dioxus_document.rs is reached by the DioxusEventHandler dispatch change and the cited lines 33-47 in it may have moved
@answer n=10 entity=screen-reader-support#2.1 v=yes mode=add-to
why: the served-id grammar stands since every existing id reads as before, and F5 joins its author_id checks: stand_graph asserts that no served id holds the reserved separator ///
@answer n=11 entity=screen-reader-support#3.1 v=yes mode=stale
why: the snapshot model's seven fields are now eight with clickable after focused (F1), and the graph is a further reader of the model (F4)
@answer n=12 entity=screen-reader-support#3.2 v=yes mode=stale
why: NODE_FIELDS reads fourteen now, not thirteen, with clickable added and no expanded key still absent
@answer n=13 entity=screen-reader-support#3.3 v=no
why: option and group labels leave an open list only through screenshots and the name, neither changed, and no reached file is cited
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checked and toggled mapping is the untouched HTML-AAM and ARIA state mapping, and no reached file is cited
@answer n=15 entity=screen-reader-support#4.1 v=yes mode=contradict
why: it says no snapshot field says an element reacts to a click and NODE_FIELDS reads thirteen, but clickable (F1) is now that field, with its per-task CLICKABLE counts in stand_non_lean_tasks (F3)
@answer n=16 entity=screen-reader-support#4.2 v=yes mode=contradict
why: it says no snapshot field is added and NODE_FIELDS reads thirteen, but clickable is added and NODE_FIELDS reads fourteen, reading true on counter-increment and Cells' 676 cells and false on counter-value and cells-grid (F2)
@answer n=17 entity=screen-reader-support#4.3 v=yes mode=add-to
why: the over-report note stands with the mark unchanged, and F1 joins it: clickable reads the vdom walk's current listeners exactly, so a removed listener reads not clickable
@answer n=18 entity=screen-reader-support#4.4 v=no
why: cells-grid's aria-label and no role are the untouched Cells role semantics under residual res-wvatzhpe
@answer n=19 entity=screen-reader-support#4.5 v=no
why: the actionable-key check still returns nothing on the load page, and its Tab, keyboard and contrast rows are unchanged
@answer n=20 entity=screen-reader-support#5.1 v=no
why: roles, attribute states, Tab-reached rows, key paths and the unread aria-activedescendant are unchanged, and the actionable check still returns nothing
@answer n=21 entity=screen-reader-support#6.1 v=no
why: tag names are set in blitz-dom accessibility.rs, which is not reached
@answer n=22 entity=screen-reader-support#6.2 v=no
why: the Window root and focus fallback are blitz-dom accessibility.rs, which is not reached
@answer n=23 entity=screen-reader-support#6.3 v=no
why: the role-mapping test goal is stated in a test file the session does not reach
@answer n=24 entity=screen-reader-support#7.1 v=no
why: the accesskit_xplat handler routing is untouched and the crate is not reached
@answer n=25 entity=screen-reader-support#8.1 v=no
why: adapter creation in blitz-shell is untouched and that crate is not reached
