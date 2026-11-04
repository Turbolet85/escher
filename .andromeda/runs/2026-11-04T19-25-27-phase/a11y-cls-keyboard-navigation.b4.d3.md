@answer n=1 entity=keyboard-navigation#5.9 v=no
why: Cells' grid editor is not reached; the probe runs only Counter or CRUD on the stand
@answer n=2 entity=keyboard-navigation#5.10 v=yes
why: the CRUD listbox, its aria-selected rows, crud-list's tabindex and arrow and Home/End selection are the stand surface the handler-to-state links read
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: TodoMVC Preact input is not part of the seven_guis stand the probe runs on
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus requests are outside the headless stand the probe runs on
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture is an external asset the probe never loads; its tabindex is not read
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: the google fixture's role and tabindex on non-buttons is outside the seven_guis stand the probe reads
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown binding is an external fixture the probe does not run
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible outline is a visual rule on an unused fixture; the probe reads no colour or outline
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture's newsletter focus styling is on an unused fixture the probe does not read
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: the servo.css focus ring is a visual rule on an unused fixture; the probe reads no focus ring
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: servo.html heading tabindex is on an unused fixture the probe does not run
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: form label association by for/id and nesting is on examples/form.rs, which the probe does not run
@answer n=13 entity=keyboard-navigation#7.1 v=yes
why: stand_snapshot_state's Tab and Shift+Tab focus through press, with the snapshot node and tree reading focused, is the focus reading the probe's links rely on
@answer n=14 entity=keyboard-navigation#7.2 v=yes
why: stand_keyboard's Tab walk agreeing across engine focus, snapshot focused and tree focus, and Enter and Space reading as a button's click, are the press and focus paths the probe drives in both layout modes
@answer n=15 entity=keyboard-navigation#7.3 v=no
why: the load page and Cells Tab-order tables are not read by the Counter or CRUD probe
@answer n=16 entity=keyboard-navigation#7.4 v=no
why: the range input's keys and slider focus are not driven by the probe; the scope marks range input not needed
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: text input and range input and change events come from typing and range keys the probe never sends
@answer n=18 entity=keyboard-navigation#7.6 v=no
why: select_interaction and stand_act_select read select lists, which the probe reads none of
@answer n=19 entity=keyboard-navigation#7.7 v=no
why: select popup and its list checks are select list behaviour, which the probe does not read
@answer n=20 entity=keyboard-navigation#7.8 v=no
why: the flight booker's flight-type select arrows and list opening are select list behaviour outside the probe's Counter or CRUD surface
@answer n=21 entity=keyboard-navigation#7.9 v=yes
why: key presses through the harness reaching a button or control past a dropped anonymous block, in both layout modes, are the press path the probe's key presses rely on
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: a tabindex absence over upstream s06 slice files the chunk does not reach; the probe adds no tabindex handling
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: a tabIndex absence over upstream s10 slice files the chunk does not reach; the probe adds no tabindex handling
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: a focus and keyboard absence over upstream s07 slice files the chunk does not reach; the probe adds no focus handling
