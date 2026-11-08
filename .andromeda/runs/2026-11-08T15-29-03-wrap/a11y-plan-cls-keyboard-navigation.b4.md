@answer n=1 entity=keyboard-navigation#5.7 v=no
why: Cells edit commit on Enter or blur lives in cells.rs, which the session did not reach, and keyboard activation is unchanged.
@answer n=2 entity=keyboard-navigation#5.8 v=no
why: Cells arrow and scroll_to checks live in stand_keyboard.rs, which the session did not reach, and Tab orders are unchanged.
@answer n=3 entity=keyboard-navigation#5.9 v=no
why: the Cells editor Enter flow lives in cells.rs and stand_keyboard.rs, neither reached, and keyboard activation is unchanged.
@answer n=4 entity=keyboard-navigation#5.10 v=no
why: the CRUD list keys and roles live in crud.rs and its pinned test, neither reached, and keyboard navigation is unchanged.
@answer n=5 entity=keyboard-navigation#5.11 v=no
why: the TodoMVC Preact autofocus lives in examples/preact, which the session did not reach.
@answer n=6 entity=keyboard-navigation#5.12 v=no
why: the Android focusable views live in the Kotlin activities, which the session did not reach.
@answer n=7 entity=keyboard-navigation#6.1 v=no
why: the docs.rs tabindex fixture lives in examples/assets HTML, which the session did not reach.
@answer n=8 entity=keyboard-navigation#6.2 v=no
why: the google role and tabindex fixture lives in examples/assets/google.html, which the session did not reach.
@answer n=9 entity=keyboard-navigation#6.3 v=no
why: the google jsaction keydown bindings live in examples/assets/google.html, which the session did not reach.
@answer n=10 entity=keyboard-navigation#6.4 v=no
why: the google focus-visible outline lives in examples/assets/google.html, which the session did not reach.
@answer n=11 entity=keyboard-navigation#6.5 v=no
why: the graphite focus outline lives in examples/assets/graphite.html, which the session did not reach.
@answer n=12 entity=keyboard-navigation#6.6 v=no
why: the servo focus ring lives in examples/assets/servo.css, which the session did not reach.
@answer n=13 entity=keyboard-navigation#6.7 v=no
why: the servo heading tabindex lives in examples/assets/servo.html, which the session did not reach.
@answer n=14 entity=keyboard-navigation#6.8 v=no
why: form label association lives in examples/form.rs, which the session did not reach.
@answer n=15 entity=keyboard-navigation#7.1 v=no
why: the snapshot focused reading and its stand_snapshot_state check are unchanged, and that file is not reached.
@answer n=16 entity=keyboard-navigation#7.2 v=yes mode=stale
why: the shared tab_order, focused and ENABLED_TAB_ORDER readers live in common/mod.rs, a file the session reached whole, and this entity cites its line ranges, which may have moved.
@answer n=17 entity=keyboard-navigation#7.3 v=yes mode=stale
why: LOAD_TAB_ORDER and CELLS_TAB_ORDER are cited at common/mod.rs lines 104-109 in a file the session reached whole, so those lines may have moved.
@answer n=18 entity=keyboard-navigation#7.4 v=no
why: the range key checks and stand_act_range live in files the session did not reach, and the driver click on the slider still succeeds with the stand run at 253 passed.
@answer n=19 entity=keyboard-navigation#7.5 v=no
why: the text and range input and change event pins live in a file the session did not reach, and no driver refusal is involved.
@answer n=20 entity=keyboard-navigation#7.6 v=no
why: the covered refusal stays the cause for a closed-select option under check_target, and the stand run reads 253 passed.
@answer n=21 entity=keyboard-navigation#7.7 v=yes mode=stale
why: this entity cites one test each in cli_commands.rs, host_log.rs and mcp_log.rs, files the session reached whole, so those counts and line ranges may have moved.
@answer n=22 entity=keyboard-navigation#7.8 v=no
why: the flight-type select arrows are pinned in stand_keyboard.rs, which the session did not reach, and focusability is unchanged.
@answer n=23 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box.rs is not reached by the session, and keyboard focus is unchanged.
@answer n=24 entity=keyboard-navigation#8.1 v=no
why: the tabindex absence holds because the session leaves focusability unchanged and adds no tabindex handling.
@answer n=25 entity=keyboard-navigation#8.2 v=no
why: the tabindex absence over the s10 slice holds because the session leaves focusability unchanged and adds no tabindex handling.
