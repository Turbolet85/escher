@answer n=1 entity=keyboard-navigation#5.8 v=no
why: the Cells grid's Tab stop, arrows and scroll_to are re-read as unchanged (no key path changed, Cells' Tab order as before) and the cited stand_keyboard.rs is not reached
@answer n=2 entity=keyboard-navigation#5.9 v=yes mode=stale
why: cells.rs is reached whole by the use_graph_signal wrap and the cited editor lines cells.rs:474-482 may have moved
@answer n=3 entity=keyboard-navigation#5.10 v=yes mode=stale
why: crud.rs is reached whole by the use_graph_signal wrap and the cited crud.rs lines (109-230) may have moved, the CRUD list keys unchanged
@answer n=4 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC Preact index.html is not in the session's change set
@answer n=5 entity=keyboard-navigation#5.12 v=no
why: the Android MainActivity files are not in the session's change set
@answer n=6 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture under examples/assets is not in the session's change set
@answer n=7 entity=keyboard-navigation#6.2 v=no
why: the google fixture under examples/assets is not in the session's change set
@answer n=8 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction bindings under examples/assets are not in the session's change set
@answer n=9 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible outline under examples/assets is not in the session's change set
@answer n=10 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture under examples/assets is not in the session's change set
@answer n=11 entity=keyboard-navigation#6.6 v=no
why: servo.css under examples/assets is not in the session's change set
@answer n=12 entity=keyboard-navigation#6.7 v=no
why: servo.html under examples/assets is not in the session's change set
@answer n=13 entity=keyboard-navigation#6.8 v=no
why: examples/form.rs is not in the session's change set
@answer n=14 entity=keyboard-navigation#7.1 v=no
why: the session adds clickable beside focused and leaves the focused reading and stand_snapshot_state.rs unchanged
@answer n=15 entity=keyboard-navigation#7.2 v=yes mode=stale
why: common/mod.rs is reached whole by the session and the cited tab_order, ENABLED_TAB_ORDER and LOAD/CELLS lines (47-119) may have moved, the Tab orders re-read as before
@answer n=16 entity=keyboard-navigation#7.3 v=yes mode=stale
why: common/mod.rs is reached whole and the cited shared-module lines 93-98 may have moved, the LOAD_TAB_ORDER and CELLS_TAB_ORDER values re-read as before
@answer n=17 entity=keyboard-navigation#7.4 v=no
why: the range-input checks live in unreached files and no key path changed
@answer n=18 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event.rs is not in the session's change set
@answer n=19 entity=keyboard-navigation#7.6 v=no
why: select_interaction.rs and stand_act_select.rs are unreached and the select key paths are unchanged
@answer n=20 entity=keyboard-navigation#7.7 v=no
why: the select popup and select_list files are unreached by the session
@answer n=21 entity=keyboard-navigation#7.8 v=no
why: the flight booker select's name, role and Tab stop are re-read as unchanged and stand_keyboard.rs is unreached
@answer n=22 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box.rs is not in the session's change set
@answer n=23 entity=keyboard-navigation#8.1 v=no
why: focusability is re-read as unchanged and the session adds no tabindex handling
@answer n=24 entity=keyboard-navigation#8.2 v=no
why: focusability is re-read as unchanged and the session adds no tabindex handling in the s10 slice
@answer n=25 entity=keyboard-navigation#8.3 v=no
why: the session adds no focus or keyboard handling, and its bridge dispatch change is event plumbing with no keyboard path moved
