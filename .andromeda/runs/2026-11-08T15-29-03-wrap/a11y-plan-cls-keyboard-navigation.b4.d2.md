@answer n=1 entity=keyboard-navigation#5.7 v=no
why: the Cells commit on Enter or blur is in examples/seven_guis/src/tasks/cells.rs, which the session does not reach
@answer n=2 entity=keyboard-navigation#5.8 v=no
why: the Cells tab stop and scroll_to walk is pinned in stand_keyboard.rs, which the session does not reach, and keyboard walks are unchanged
@answer n=3 entity=keyboard-navigation#5.9 v=no
why: the Cells editor and its Enter commit are pinned in cells.rs and stand_keyboard.rs, neither reached, and keyboard activation is unchanged
@answer n=4 entity=keyboard-navigation#5.10 v=no
why: the CRUD list citations sit in crud.rs and accessibility_active_descendant_unread.rs, neither reached, and focusability and the builder are unchanged
@answer n=5 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC Preact fixture index.html is not among the reached files
@answer n=6 entity=keyboard-navigation#5.12 v=no
why: the Android MainActivity sources are not among the reached files
@answer n=7 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture header is an unreached HTML asset, and its tabindex markup is unchanged
@answer n=8 entity=keyboard-navigation#6.2 v=no
why: the google fixture is an unreached HTML asset, and its role and tabindex markup is unchanged
@answer n=9 entity=keyboard-navigation#6.3 v=no
why: the google jsaction keydown bindings sit in an unreached HTML asset, and event routing is unchanged
@answer n=10 entity=keyboard-navigation#6.4 v=no
why: the google focus-visible outline sits in an unreached HTML asset, and focus styling is unchanged
@answer n=11 entity=keyboard-navigation#6.5 v=no
why: the graphite focus styling sits in an unreached HTML asset, and it is unchanged
@answer n=12 entity=keyboard-navigation#6.6 v=no
why: servo.css is not among the reached files, and its focus ring is unchanged
@answer n=13 entity=keyboard-navigation#6.7 v=no
why: servo.html is not among the reached files, and its heading tabindex markup is unchanged
@answer n=14 entity=keyboard-navigation#6.8 v=no
why: examples/form.rs is not among the reached files, and label association is unchanged
@answer n=15 entity=keyboard-navigation#7.1 v=no
why: the Tab and Shift+Tab focus check sits in stand_snapshot_state.rs, which the session does not reach, and focus behaviour is unchanged
@answer n=16 entity=keyboard-navigation#7.2 v=yes mode=stale
why: the session reached tests/blitz-tests/tests/common/mod.rs whole, and this entity cites its tab_order and focused readers at lines 47-63, 65-91 and 110-130, which may have moved
@answer n=17 entity=keyboard-navigation#7.3 v=yes mode=stale
why: the session reached tests/blitz-tests/tests/common/mod.rs whole, and this entity cites LOAD_TAB_ORDER and CELLS_TAB_ORDER at lines 104-109, which may have moved
@answer n=18 entity=keyboard-navigation#7.4 v=no
why: the range input checks sit in unreached range test files, and range key behaviour is unchanged
@answer n=19 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event.rs is not among the reached files, and its input and change behaviour is unchanged
@answer n=20 entity=keyboard-navigation#7.6 v=no
why: the select checks sit in unreached select_interaction.rs and stand_act_select.rs, the covered refusal cause is unchanged, and the CI leg is unchanged
@answer n=21 entity=keyboard-navigation#7.7 v=yes mode=stale
why: the session reached examples/seven_guis/tests/cli_commands.rs, host_log.rs and mcp_log.rs whole, and this entity cites their test lines 421-467, 284-396 and 72-118, which may have moved
@answer n=22 entity=keyboard-navigation#7.8 v=no
why: the flight type select sits in unreached stand_keyboard.rs, and its test count is not moved
@answer n=23 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box.rs is not among the reached files, and its leg wiring is unchanged
@answer n=24 entity=keyboard-navigation#8.1 v=no
why: the session leaves focusability and tabindex handling unchanged, so the tabindex absence still holds
@answer n=25 entity=keyboard-navigation#8.2 v=no
why: the session leaves focusability and tabindex handling unchanged, so the tabindex absence over the s10 slice still holds
