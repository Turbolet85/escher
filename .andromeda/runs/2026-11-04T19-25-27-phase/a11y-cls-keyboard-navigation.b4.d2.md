@answer n=1 entity=keyboard-navigation#5.9 v=no
why: Cells' grid keyboard model and editor commit, which the task drops as not needed; the probe runs on Counter or CRUD only.
@answer n=2 entity=keyboard-navigation#5.10 v=yes
why: the CRUD list crud-list, its arrow keys writing the selection and fields, aria-selected on option rows and the People name are the handler-to-state surface the probe reads (task item 8).
@answer n=3 entity=keyboard-navigation#5.11 v=no
why: TodoMVC Preact autofocus and Enter adding an item is a Preact example outside the seven_guis stand the probe uses.
@answer n=4 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus requests are a native-view platform path, which the headless probe does not reach.
@answer n=5 entity=keyboard-navigation#6.1 v=no
why: the docs.rs fixture's tabindex on toolbar containers; the probe reads no fixture, only the seven_guis stand.
@answer n=6 entity=keyboard-navigation#6.2 v=no
why: the google fixture's role=button and tabindex on non-button elements; the fixture is not reached by the probe.
@answer n=7 entity=keyboard-navigation#6.3 v=no
why: the google fixture's jsaction keydown binding is a different event path from the probe's key handlers.
@answer n=8 entity=keyboard-navigation#6.4 v=no
why: the google fixture's focus-visible outline is a focus ring, which the task drops as not needed.
@answer n=9 entity=keyboard-navigation#6.5 v=no
why: the graphite fixture's newsletter focus styling is a focus ring the task drops, and the fixture is not reached.
@answer n=10 entity=keyboard-navigation#6.6 v=no
why: servo.css's button focus outline and box-shadow ring are a focus ring the task drops as not needed.
@answer n=11 entity=keyboard-navigation#6.7 v=no
why: the servo.html heading tabindex -1 fixture; the probe reads no heading and no fixture.
@answer n=12 entity=keyboard-navigation#6.8 v=no
why: form label association by for/id and nesting in examples/form.rs; label reads are not needed (the builder's label-for read is dropped).
@answer n=13 entity=keyboard-navigation#7.1 v=yes
why: the stand_snapshot_state Tab and Shift+Tab focus check on the CRUD fields, with the snapshot's focused node equal to the tree's focus, is the focus reading and press verb the probe drives and reads (items 3 and 6).
@answer n=14 entity=keyboard-navigation#7.2 v=yes
why: stand_keyboard's Enter and Space dispatch as click, the CRUD list taking focus with its keys moving the selection, and the focus-agreement reads bind the press path and handler dispatch the probe drives (items 6, 7 and 8).
@answer n=15 entity=keyboard-navigation#7.3 v=no
why: the LOAD_TAB_ORDER and CELLS_TAB_ORDER tables belong to the load page and Cells, which the probe does not reach.
@answer n=16 entity=keyboard-navigation#7.4 v=no
why: the range input's keys, clicks and step checks are not needed (item 3), and the probe reads no range.
@answer n=17 entity=keyboard-navigation#7.5 v=no
why: input and change event firing on a text input and a range is not read by any of the four links, and the probe types into no field.
@answer n=18 entity=keyboard-navigation#7.6 v=no
why: the select list's keys, clicks and typeahead are not needed (item 2), and the probe reads no select list.
@answer n=19 entity=keyboard-navigation#7.7 v=no
why: the select popup checks are the select list's own behaviour, which the task drops as not needed (item 2).
@answer n=20 entity=keyboard-navigation#7.8 v=no
why: the flight booker's select arrows are select-list behaviour the task drops (item 2), and the flight booker is outside the Counter or CRUD stand the probe uses.
@answer n=21 entity=keyboard-navigation#7.9 v=yes
why: a key press reaching a button past a dropped anonymous block in both layout modes is the press verb's layout-touched reading, which the probe's key presses on the stand carry (item 11).
@answer n=22 entity=keyboard-navigation#8.1 v=no
why: an observed tabindex absence in 17 s06 files states a gap rather than a rule, and the probe adds no tabindex handling and reads none.
@answer n=23 entity=keyboard-navigation#8.2 v=no
why: an observed tabindex absence in 32 s10 slice files is a gap statement the probe neither needs nor changes.
@answer n=24 entity=keyboard-navigation#8.3 v=no
why: an observed focus and keyboard absence in 8 s07 slice files is a gap statement the chunk neither reads nor changes.
