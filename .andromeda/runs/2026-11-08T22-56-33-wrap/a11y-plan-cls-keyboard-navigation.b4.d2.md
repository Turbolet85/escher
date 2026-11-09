@answer n=1 entity=keyboard-navigation#5.7 v=no
why: its Cells edit-commit rule lives in cells.rs, which the session did not reach
@answer n=2 entity=keyboard-navigation#5.8 v=no
why: its Cells grid arrow rule lives in stand_keyboard.rs, which the session did not reach
@answer n=3 entity=keyboard-navigation#5.9 v=no
why: its Cells editor rule lives in cells.rs and stand_keyboard.rs, neither reached by the session
@answer n=4 entity=keyboard-navigation#5.10 v=no
why: its CRUD list rule lives in crud.rs and stand_keyboard.rs, neither reached by the session
@answer n=5 entity=keyboard-navigation#5.11 v=no
why: it cites the Preact fixture, which the session did not reach
@answer n=6 entity=keyboard-navigation#5.12 v=no
why: it cites Android activity files, which the session did not reach
@answer n=7 entity=keyboard-navigation#6.1 v=no
why: it cites the docs.rs fixture asset, which the session did not reach
@answer n=8 entity=keyboard-navigation#6.2 v=no
why: it cites the google fixture asset, which the session did not reach
@answer n=9 entity=keyboard-navigation#6.3 v=no
why: it cites the google fixture asset, which the session did not reach
@answer n=10 entity=keyboard-navigation#6.4 v=no
why: it cites the google fixture asset, which the session did not reach
@answer n=11 entity=keyboard-navigation#6.5 v=no
why: it cites the graphite fixture asset, which the session did not reach
@answer n=12 entity=keyboard-navigation#6.6 v=no
why: it cites servo.css, which the session did not reach
@answer n=13 entity=keyboard-navigation#6.7 v=no
why: it cites the servo.html fixture, which the session did not reach
@answer n=14 entity=keyboard-navigation#6.8 v=no
why: it cites examples/form.rs, which the session did not reach
@answer n=15 entity=keyboard-navigation#7.1 v=no
why: its focus check lives in stand_snapshot_state.rs, which the session did not reach, and snapshot fields are unchanged
@answer n=16 entity=keyboard-navigation#7.2 v=yes mode=add-to
why: F2, stand_handler_coverage's new check that a keyboard activation exercises its control's click handler, joins the Enter and Space click rows here, and common/mod.rs, where this walk's readers live, is reached
@answer n=17 entity=keyboard-navigation#7.3 v=yes mode=stale
why: common/mod.rs, where LOAD_TAB_ORDER and CELLS_TAB_ORDER live, is reached whole by the session, so its cited lines 104-109 can have moved
@answer n=18 entity=keyboard-navigation#7.4 v=no
why: its range checks live in range_input_interaction.rs and stand_act_range.rs, neither reached by the session
@answer n=19 entity=keyboard-navigation#7.5 v=no
why: it cites text_range_no_change_event.rs, which the session did not reach
@answer n=20 entity=keyboard-navigation#7.6 v=no
why: the stand-repeat prefix pickup it names is unchanged, the new target joins by that same prefix, and this text names no stand_ target list
@answer n=21 entity=keyboard-navigation#7.7 v=yes mode=stale
why: cli_commands.rs and host_log.rs, which it cites by line, are reached and gained a test each (17 to 18 and 5 to 6), so the cited lines and counts can have moved
@answer n=22 entity=keyboard-navigation#7.8 v=no
why: its flight-type select rule lives in stand_keyboard.rs, which the session did not reach
@answer n=23 entity=keyboard-navigation#7.9 v=no
why: it cites key_press_after_container_lost_its_box.rs, which the session did not reach
@answer n=24 entity=keyboard-navigation#8.1 v=no
why: its tabindex search covers s06 files, none of which the session reached
@answer n=25 entity=keyboard-navigation#8.2 v=no
why: its tabindex search covers s10 files, none of which the session reached
