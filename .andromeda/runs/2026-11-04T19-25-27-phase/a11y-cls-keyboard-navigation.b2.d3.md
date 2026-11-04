@answer n=1 entity=keyboard-navigation#2.10 v=no
why: details open-state flushing and summary toggling are not read by the probe, which uses no details element; the summary's Enter click is covered by keyboard-navigation#3.17
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling is on no Counter or CRUD path the probe reads
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the checkbox and radio change dispatch is on no control the probe presses, and it is an engine change event rather than a click listener reading
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input focus and pointer value setting are not needed, the probe reads no range input
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: the probe types into crud-name but presses no editing key (arrows, Home, End, Delete, Backspace, clipboard) on a text input, and Enter goes only to buttons
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: the range input's keys are not needed, no range input is on the probe's path
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: a single select's list keys and list opening are on the flight booker's select, which the Counter and CRUD probe does not reach
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's closed key match is the select list's behaviour, which is not needed, and the probe opens no list
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a list row with Enter, Space, Tab or a click is the select list's behaviour, which is not needed
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: the list's MAX_ROWS and key or wheel scrolling belong to the select popup, which the probe does not read
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the select list's painted thumb is not on the probe's path, which reads no select list
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: the CRUD listbox's rows carry aria-selected and take their arrow keys through the app's handler, not the engine's multiple-select keys
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view for a multiple select with a valid size does not apply, since the CRUD list is not a native multiple select
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space flipping rows on a multiple select does not apply, since the CRUD list is not a native multiple select
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: a multiple select's disabled rows are never chosen, a rule the probe does not reach
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift spanning from the last chosen row of a multiple select is not on the probe's path
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order in a select is not reached by the Counter or CRUD probe
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the mutator's multiple-select write record concerns native select options, which the probe does not write
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type focusing a select concerns a select the probe never types into, and the probe adds no verb
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver press holding no Ctrl limits only a non-adjacent multiple select set, which the probe does not build
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: Enter and Space on the focused Counter or CRUD buttons dispatch the Click whose app handlers run before handle_click, the path links (3) and (4) read
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace handling by the Apple standard keybindings is not on the probe's path
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copying selected text with no text input focused is not pressed by the probe
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input is not pressed, since the probe's Enter goes to buttons
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label click focusing a single select is not on the probe's path, which reads no select
