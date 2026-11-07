@answer n=1 entity=keyboard-navigation#5.9 v=no
why: Cells' editor open and commit rules live in examples/seven_guis/src/tasks/cells.rs, which the session does not reach, and its synchronous MountedData calls are kept (task item 3)
@answer n=2 entity=keyboard-navigation#5.10 v=no
why: the CRUD list's keys and option-level focus live in crud.rs and the unchanged accessibility builder, outside the session's reached files
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC autofocus and Enter-to-add rule lives in examples/preact/index.html, which the session does not reach
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: the Android focus-request lines live in the Kotlin activity files, which the session does not reach
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture's tabindex lines live in examples/assets/docsrs_header.html, which the session does not reach and whose focusability rules are unchanged
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: the google fixture's role and tabindex lines live in examples/assets/google.html, which the session does not reach
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown bindings live in examples/assets/google.html, which the session does not reach
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible outline lives in examples/assets/google.html, which the session does not reach and paint is untouched
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture's focus styling lives in examples/assets/graphite.html, which the session does not reach and styles are untouched
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: the servo.css focus ring lives in examples/assets/servo.css, which the session does not reach and paint is untouched
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: the servo.html heading tabindex lines live in examples/assets/servo.html, which the session does not reach
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: the form label association lives in examples/form.rs, which the session does not reach and the AccessKit builder is unchanged
@answer n=13 entity=keyboard-navigation#7.1 v=no
why: the key-press focus check reads only the snapshot node and the tree's focus, which the session leaves unchanged, and its stand_snapshot_state test file is not reached
@answer n=14 entity=keyboard-navigation#7.2 v=yes mode=add-to
why: this section lists engine, snapshot and tree focus agreeing on the stand, and the probe's two pins now assert the same three readings agreeing on focus-target after a spawned task's set_focus, in both layout modes (task item 9)
@answer n=15 entity=keyboard-navigation#7.3 v=no
why: the LOAD_TAB_ORDER and CELLS_TAB_ORDER constants live in the shared stand test module, which the session does not reach and whose sequences are unchanged
@answer n=16 entity=keyboard-navigation#7.4 v=no
why: range_input_interaction and stand_act_range are not reached, and range key and click handling is untouched
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event is not reached, and the input and change event rules are untouched
@answer n=18 entity=keyboard-navigation#7.6 v=no
why: select_interaction and stand_act_select are not reached, and select key and typeahead rules are untouched
@answer n=19 entity=keyboard-navigation#7.7 v=no
why: the select popup tests and select_list.rs are not reached, and popup rules are untouched
@answer n=20 entity=keyboard-navigation#7.8 v=no
why: the flight booker's select arrows and the stand_keyboard count of 23 tests are unchanged, since the session adds no stand test
@answer n=21 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box is not reached, and anonymous-block handling is untouched
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: the session adds no tabindex handling, since focusability and the engine's key rules are unchanged, so the absence search still holds
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: the session adds no tabindex handling, since focusability and the engine's key rules are unchanged, so the absence search still holds
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: the session adds no focus or keyboard handling, since the change is the borrow and replay order and the engine's focus and key rules are unchanged
