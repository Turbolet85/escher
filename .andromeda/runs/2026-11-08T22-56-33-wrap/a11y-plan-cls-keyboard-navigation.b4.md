@answer n=1 entity=keyboard-navigation#5.7 v=no
why: the session does not touch examples/seven_guis/src/tasks/cells.rs or its Enter and blur commit
@answer n=2 entity=keyboard-navigation#5.8 v=no
why: stand_keyboard.rs and the Cells grid arrows are not reached, and stand_keyboard and its stated orders did not move
@answer n=3 entity=keyboard-navigation#5.9 v=no
why: the Cells editor citations in cells.rs and stand_keyboard.rs are not reached by the session
@answer n=4 entity=keyboard-navigation#5.10 v=no
why: crud.rs, the CRUD list keys and their stand_keyboard checks are not reached by the session
@answer n=5 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC preact fixture is not in any reached file
@answer n=6 entity=keyboard-navigation#5.12 v=no
why: the Android activity files are not in any reached file
@answer n=7 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture under examples/assets is not in any reached file
@answer n=8 entity=keyboard-navigation#6.2 v=no
why: the google fixture under examples/assets is not in any reached file
@answer n=9 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction bindings are not in any reached file
@answer n=10 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible styling is not in any reached file
@answer n=11 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture is not in any reached file
@answer n=12 entity=keyboard-navigation#6.6 v=no
why: servo.css is not in any reached file
@answer n=13 entity=keyboard-navigation#6.7 v=no
why: servo.html is not in any reached file
@answer n=14 entity=keyboard-navigation#6.8 v=no
why: examples/form.rs is not in any reached file
@answer n=15 entity=keyboard-navigation#7.1 v=no
why: stand_snapshot_state.rs is not reached and its key-press focus check is unchanged
@answer n=16 entity=keyboard-navigation#7.2 v=yes mode=add-to
why: F2, the new stand_handler_coverage asserts that a keyboard activation exercises its control's click handler, joins the Enter and Space click rows; its common/mod.rs citations are reached too
@answer n=17 entity=keyboard-navigation#7.3 v=yes mode=stale
why: common/mod.rs is reached whole by the session, so its cited lines 104-109 may have moved
@answer n=18 entity=keyboard-navigation#7.4 v=no
why: range_input_interaction.rs and stand_act_range.rs are not reached by the session
@answer n=19 entity=keyboard-navigation#7.5 v=no
why: text_range_no_change_event.rs is not reached by the session
@answer n=20 entity=keyboard-navigation#7.6 v=yes mode=stale
why: the stand-repeat leg's stand_ prefix run now also takes stand_handler_coverage, so this by-prefix reading has moved
@answer n=21 entity=keyboard-navigation#7.7 v=yes mode=stale
why: cli_commands.rs and host_log.rs are reached whole and their test counts moved (17 to 18, 5 to 6), so the cited lines may have moved
@answer n=22 entity=keyboard-navigation#7.8 v=no
why: stand_keyboard.rs is not reached, and its Enter and Space rows are select rows, not click dispatch
@answer n=23 entity=keyboard-navigation#7.9 v=yes mode=stale
why: the stand-repeat leg's stand_ prefix run now also takes stand_handler_coverage, moving the by-prefix reading this text states
@answer n=24 entity=keyboard-navigation#8.1 v=no
why: the session adds no tabindex handling to any reached file, so the absence search still holds
@answer n=25 entity=keyboard-navigation#8.2 v=no
why: the session adds no tabindex handling to any reached file, so the absence search still holds
