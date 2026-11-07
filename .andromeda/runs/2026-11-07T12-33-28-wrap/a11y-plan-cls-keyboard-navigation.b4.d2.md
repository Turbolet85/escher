@answer n=1 entity=keyboard-navigation#5.9 v=no
why: the Cells editor's Enter, commit and click-in-editor rules live in examples/seven_guis/src/tasks/cells.rs and the stand test, neither reached, and the session does not change them
@answer n=2 entity=keyboard-navigation#5.10 v=no
why: the CRUD list's keys, the tree's focus on the list and the People aria-label live in examples/seven_guis/src/tasks/crud.rs, not reached, and the session changes none of them
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC input autofocus is Preact page code in examples/preact/index.html, outside the Dioxus bridge the session changes
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: the Android Activities' focus requests are native Kotlin code the session does not reach
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture's tabindex=-1 toolbar markup is an example HTML asset the session does not reach
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: the google fixture's role=button and tabindex=0 markup is an example HTML asset the session does not reach
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown bindings are an example HTML asset the session does not reach
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: the google fixture's :focus-visible outline is an example stylesheet the session does not reach and no paint code moved
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture's newsletter outline and border-color focus marking are an example HTML asset the session does not reach
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: servo.css's .button:focus outline and box-shadow ring are an example stylesheet the session does not reach
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: the servo.html headings' tabindex=-1 is an example HTML asset the session does not reach
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: the form label for/id and nesting association is in examples/form.rs, which the session does not reach
@answer n=13 entity=keyboard-navigation#7.1 v=no
why: the stand key-press focus check reads only the snapshot node and the tree, in stand_snapshot_state.rs, which the session does not reach, so its claim still holds
@answer n=14 entity=keyboard-navigation#7.2 v=yes mode=add-to
why: the probe's two pins now assert engine focus, the snapshot's focused node and the tree's focus agreeing on focus-target after a spawned set_focus, a new place where the three focus readings are asserted to agree that this stand walk inventory lists
@answer n=15 entity=keyboard-navigation#7.3 v=no
why: LOAD_TAB_ORDER and CELLS_TAB_ORDER sit in the shared test module common/mod.rs, which the session does not reach, and their contents are unchanged
@answer n=16 entity=keyboard-navigation#7.4 v=no
why: range_input_interaction and stand_act_range test the engine's range key and click rules, which the session leaves unchanged
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event pins the engine's input and change firing, which the session leaves unchanged
@answer n=18 entity=keyboard-navigation#7.6 v=no
why: select_interaction and stand_act_select pin the engine's select key and typeahead rules, which the session leaves unchanged
@answer n=19 entity=keyboard-navigation#7.7 v=no
why: the select popup checks run on select_list and the popup tests, none reached, and the session does not change the select list
@answer n=20 entity=keyboard-navigation#7.8 v=no
why: the flight booker's select#flight-type arrows and Enter and Space rows are pinned in stand_keyboard.rs, which the session does not reach, and their behaviour is unchanged
@answer n=21 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box runs engine key presses on dropped anonymous blocks without the Dioxus bridge, and its file is not reached
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: the session adds no tabindex handling and the attribute list is unchanged, so the tabindex absence over the s06 files still holds
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: the session adds no tabindex handling and the attribute list is unchanged, so the tabindex absence over the s10 slice still holds
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: the session adds no focus or keyboard handling, since the engine's focus and key rules are unchanged, so the focus and keyboard absence over the s07 slice still holds
