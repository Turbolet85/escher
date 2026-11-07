@answer n=1 entity=keyboard-navigation#5.9 v=yes
why: Cells' Enter returning focus to the grid is witnessed by stand_keyboard.rs:202-249, a stand_ check the chunk re-asserts through its focus path
@answer n=2 entity=keyboard-navigation#5.10 v=yes
why: the CRUD tree's focus is the list and never the option, asserted by stand_keyboard.rs:932-992, a re-asserted stand_ check and the tree-focus rule the chunk keeps
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: the Preact TodoMVC example is a non-Dioxus page the chunk neither changes nor re-asserts
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus code sits outside the Dioxus bridge and every file the chunk reaches
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture's tabindex on toolbar containers is an HTML asset no re-asserted check reads, and the chunk adds no tabindex handling
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: the google fixture's role and tabindex on non-buttons is an HTML asset no re-asserted check reads
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown bindings are an asset no re-asserted check reads, and the chunk changes no event binding
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: the google fixture's :focus-visible outline is a style rule, and the chunk changes no style or paint
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture's outline and border-colour focus marking is a style rule, and the chunk changes no style or paint
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: servo.css's button outline and box-shadow focus ring is a style rule, and the chunk changes no style or paint
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: the servo.html headings' tabindex -1 is a fixture no stand or re-asserted check reads
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: label association by for/id and nesting is name binding the chunk does not touch, and examples/form.rs is not reached
@answer n=13 entity=keyboard-navigation#7.1 v=yes
why: the stand's Tab and Shift+Tab focus check (crud-name to crud-surname, first Tab to back-btn) reads the snapshot's focused node and the tree's focus in both modes, stand_snapshot_state.rs:331-367, the Tab precondition the inverted pins depend on
@answer n=14 entity=keyboard-navigation#7.2 v=yes
why: stand_keyboard's Tab and Shift+Tab walks in both modes, with engine, snapshot and tree focus agreeing at each step, are re-asserted stand_ checks that the buffered replay could move
@answer n=15 entity=keyboard-navigation#7.3 v=yes
why: LOAD_TAB_ORDER and CELLS_TAB_ORDER are the stated Tab sequence values that the re-asserted stand_keyboard walk reads
@answer n=16 entity=keyboard-navigation#7.4 v=yes
why: stand_act_range, a re-asserted stand_ check, keeps the Timer slider focused after two Tabs and an arrow step with its focus sequence unchanged, so that focus path binds; range_input_interaction's key table is component-internal and does not
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event pins input and change event firing, witnessed by a non-stand check the chunk neither re-asserts nor touches, with no focus or tree rule
@answer n=18 entity=keyboard-navigation#7.6 v=yes
why: stand_act_select walks the Tab order fx-before to fx-after and skips the disabled fx-off, a re-asserted stand_ Tab focus walk; select_interaction's key tables are component-internal and do not bind
@answer n=19 entity=keyboard-navigation#7.7 v=yes
why: stand_act_popup, a re-asserted stand_ check run in both modes, reads the option elements the buffered replay writes, so it must still pass
@answer n=20 entity=keyboard-navigation#7.8 v=yes
why: the flight booker's focusable select#flight-type as the second Tab stop, its focus and arrows in both modes, is pinned by the re-asserted stand_keyboard check
@answer n=21 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box sits in a non-stand file the chunk does not re-assert, and its anonymous-block structure comes from the same mutations replayed in the same order
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: a tabindex absence search over s06 files the entity does not name in any reached unit; the chunk adds no tabindex handling
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: a tabindex absence search over the s10 slice files, none named as a reached unit; the chunk adds no tabindex handling
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: a focus and keyboard absence search over s07 slice files the entity does not name as a reached unit, so it states no rule over the chunk's edited units
