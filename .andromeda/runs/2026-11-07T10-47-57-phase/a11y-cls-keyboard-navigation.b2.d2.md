@answer n=1 entity=keyboard-navigation#2.10 v=yes
why: the replay's open attribute writes must go through the mutator so flush_open_state runs for a details or dialog, and open is a bridge boolean attribute
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling is the pointer path, which the chunk does not drive
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the Change event after a click is the pointer path's dispatch, which the chunk does not drive
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input click focus and value are the pointer path, which the chunk does not drive
@answer n=5 entity=keyboard-navigation#3.1 v=yes
why: the key target focused by Tab is an input the witness names, and the key the inverted pins press is unnamed, so a text input's key default action is a fork the test may hit
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input arrow and Home/End keys act on a range input the scope does not name
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys, list opening and typeahead are not reached by a Tab-focused text input press
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open select list's own key match is not reached by the chunk
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a list row through Enter, Space or Tab is the select path, which the chunk does not drive
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling by key and wheel writes the list's own scroll_y, which the chunk does not touch
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the painted list thumb takes no hit and is not reached by the chunk
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select navigation keys are not reached by a Tab-focused text input press
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select with a valid size is not reached by the chunk
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple select Space and character keys are not reached by the chunk
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple select disabled rows and the unmodelled Ctrl+A are not reached by the chunk
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row on a multiple select is not reached by the chunk
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and group disabling are select rules the chunk does not touch
@answer n=18 entity=keyboard-navigation#3.14 v=yes
why: the replay routes every attribute write and removal through the mutator's path, so a multiple write or removal must still record the select and leave other options alone
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type verb focusing a select is not a verb the chunk uses
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the press verb's no-Ctrl limit is not reached, because the chunk presses Tab with no modifier and builds no multiple set
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: the scope names the press on a Tab-focused key target, and the unnamed key may be Enter or Space on a button, so its Click default action is a fork the test may hit
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: Backspace on macOS is a platform keybinding the chunk does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy shortcut with no text input focused is not reached by the chunk
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission needs a form, and the scope names no form
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click focus and re-dispatch on a select is the label path, which the chunk does not drive
