@answer n=1 entity=keyboard-navigation#2.10 v=no
why: details open state and the summary toggle are not read or driven by the probe
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling is not on the probe's Counter or CRUD route
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: checkbox and radio change dispatch is not on the probe's route
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input click focus and value setting are not on the probe's route, which presses no range input
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input editing keys are not named by the scope, which drives Counter and CRUD by buttons and listbox rows
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input keys are not on the probe's route, which presses no range input
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select closed-list keys are not on the probe's route, which reads no select list
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's own keys are not on the probe's route, which reads no select list
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a list row through Enter, Space, Tab or click is not on the probe's route
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: the open list's row scrolling is not on the probe's route
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list's painted thumb is not on the probe's route
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select arrow, Home and End keys are not on the probe's route, which reads no select list
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: multiple select scroll into view is not on the probe's route
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple select Space and character choice is not on the probe's route
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple select disabled rows and Ctrl A are not on the probe's route
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: multiple select Shift span from a seeded set is not on the probe's route
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and disabled groups are not on the probe's route, which reads no select list
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the multiple write and removal record is not on the probe's route
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver type verb on a select is not named by the scope, which names press and press_with
@answer n=20 entity=keyboard-navigation#3.16 v=yes
why: the scope's press verb sends no Ctrl, so the probe's key presses cannot carry a Ctrl chord and must not assume one
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: Enter and Space on a stand button dispatch the Click whose app onclick runs before handle_click, the handler path links 3 and 4 read on Counter and CRUD (task item 7)
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace editing is a text input key the probe does not press
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy shortcut with no text input focused is not named by the scope and the probe copies nothing
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: a text input's implicit form submission is not on the probe's route
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click default actions are not on the probe's route, which clicks buttons and listbox rows
