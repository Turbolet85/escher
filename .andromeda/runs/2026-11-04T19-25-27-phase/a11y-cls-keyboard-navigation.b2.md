@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the open state of details and dialog and a summary's toggle sit on no Counter or CRUD path, and the probe reads no element's open state
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling is on no Counter or CRUD path, the probe clicking buttons and listbox rows only
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the checkbox and radio change dispatch is not reached, the probe driving no checkbox or radio
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input click focus and pointer value setting are not needed (task item 3)
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: the probe types a name into crud-name, and no arrow, Home/End, Delete/Backspace, Enter or clipboard key in a text input is on its path
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: a range input's keys are not needed (task item 3)
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: a single select's typeahead and open-on-Enter or Space are select list behaviour the probe does not read (task item 2)
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open select list's closed key match is select list behaviour the probe does not read (task item 2)
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a row of an open select list is select list behaviour the probe does not read (task item 2), and the CRUD listbox is app-built rows, not a select
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: a select list's MAX_ROWS window and key scrolling are select list behaviour the probe does not read (task item 2)
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: a select list's painted thumb is select list behaviour the probe does not read (task item 2)
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: a multiple select's arrow, Home, End, Shift and Ctrl keys are select list behaviour the probe does not read (task item 2), and the CRUD list is app-built option rows, not a multiple select
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: a multiple select's valid-size scroll is select list behaviour the probe does not read (task item 2)
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: a multiple select's Space flip and character choice are select list behaviour the probe does not read (task item 2)
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: a multiple select's disabled rows and Ctrl+A are select list behaviour the probe does not read (task item 2)
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: a multiple select's Shift span from its last chosen row is select list behaviour the probe does not read (task item 2)
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and disabled groups in a select are select list behaviour the probe does not read (task item 2)
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: a multiple select's write record and option selectedness are mutator behaviour on a select the probe does not reach
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select is not on the probe's path, and the chunk adds no verb to the driver's closed sets
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holds no Ctrl, a limit on building a non-adjacent multiple select set that the probe does not build
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: Enter and Space on a focused button dispatch a Click whose app handlers run before handle_click, the path that links (3) and (4) read (task item 7)
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace keybindings are platform-specific, and the headless probe on the stand presses no Backspace on its path
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused is on no probe path, and the probe copies nothing
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: a text input's implicit form submission is on no probe path, the probe pressing Enter on a focused button and no Enter in a text input
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click focus and re-dispatch to a single select are on no probe path, the probe reading no label and no select
