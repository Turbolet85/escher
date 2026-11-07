@answer n=1 entity=keyboard-navigation#5.9 v=yes
why: Cells' Enter, commit and focus return to the grid are checked by stand_keyboard 202-249, a re-asserted stand_* check on focus the chunk's render path moves
@answer n=2 entity=keyboard-navigation#5.10 v=yes
why: the CRUD list's tree focus on the list and its arrow keys are checked by stand_keyboard 932-992 and accessibility_active_descendant_unread, re-asserted focus and tree reads
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: the Preact TodoMVC autofocus is an external example page that the chunk's document and tests never load
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus concerns a native Android view the chunk does not touch
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: docs.rs fixture tabindex on toolbar containers is static fixture markup that no re-asserted check reads
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: google fixture role and tabindex markup is static fixture HTML that no re-asserted check names
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: google jsaction keydown bindings are fixture script that no re-asserted check reads
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: google focus-visible outline is painted style, which the chunk changes no style or paint of
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: graphite outline removal and border-colour focus marking are painted style the chunk does not change
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: servo.css focus-ring box-shadow is painted style the chunk does not change
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: servo.html heading tabindex is static fixture markup that no re-asserted check reads
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: form label for/id association is label binding the builder's name reads do not touch
@answer n=13 entity=keyboard-navigation#7.1 v=yes
why: the stand key-press focus check (Tab, Shift+Tab, first Tab to back-btn, snapshot focused and tree focus) in stand_snapshot_state 331-367 is a re-asserted stand_* check on focus the replay and set_focus move
@answer n=14 entity=keyboard-navigation#7.2 v=yes
why: stand_keyboard's tab_order walk, focus agreement, Enter and Space, CRUD keys and inert skip are re-asserted stand_* checks on the focus and tree the chunk changes
@answer n=15 entity=keyboard-navigation#7.3 v=yes
why: the re-asserted stand_keyboard walk reads LOAD_TAB_ORDER and CELLS_TAB_ORDER from this shared module, which must stay in place
@answer n=16 entity=keyboard-navigation#7.4 v=yes
why: stand_act_range's Timer slider focus, role and forward focus sequence is a re-asserted stand_* check on focus through the chunk's render path, while range_input_interaction itself is not reached
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: the text and range input and change event pins of text_range_no_change_event are neither focus or tree checks nor stand_* checks the scope re-asserts
@answer n=18 entity=keyboard-navigation#7.6 v=yes
why: stand_act_select's Tab walk, disabled skip and choose by click and key is a re-asserted stand_* check through the chunk's render path, while select_interaction itself is not reached
@answer n=19 entity=keyboard-navigation#7.7 v=yes
why: stand_act_popup (12) is a re-asserted stand_* check of the select list through the chunk's render path, while the other popup suites are not reached
@answer n=20 entity=keyboard-navigation#7.8 v=yes
why: the flight booker's flight-type select arrows and its Enter and Space rows in stand_keyboard 461-521 are re-asserted stand_* checks on focus and choice
@answer n=21 entity=keyboard-navigation#7.9 v=no
why: key_press_after_container_lost_its_box is an engine layout check with no stand_ prefix that the scope does not re-assert and the chunk does not render
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: a tabindex absence record over s06 audit slices, which the chunk does not reach and where it adds no tabindex handling
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: a tabindex absence record over s10 audit slices, which the chunk does not reach and where it adds no tabindex handling
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: a focus or keyboard absence record over s07 audit slices, which the chunk does not reach and where it adds no focus or keyboard handling of its own
