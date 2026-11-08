@answer n=1 entity=keyboard-navigation#2.10 v=no
why: details and dialog open state is engine element state the graph does not read, and the chunk changes no open behaviour
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox click toggling and focusing is pointer behaviour the chunk does not change, and pointer focus on click is not needed
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: change dispatch after a checkbox or radio click is engine event dispatch that the graph reads no event for
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range click focus and pointer value setting are not needed by the task, and the graph reads the snapshot's own value
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input edit keys are not changed by the chunk, and the masking reads the carried value, not the keys that edit it
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range keyboard handling is not needed by the task, and the graph's control value is the snapshot's own read
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys and their input and change dispatch are not needed, and the chunk changes no key behaviour
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open list's own key match is listbox keyboard handling the task does not need
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a row of an open list by key or click is listbox behaviour the task does not need
@answer n=10 entity=keyboard-navigation#3.6 v=yes
why: the scope names an open list's row bounds, and this entry fixes the list's MAX_ROWS of 20 and its own scroll_y, which those bounds follow
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the painted thumb takes no hit and is no node, so the graph's nodes and bounds do not reach it
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select arrow, Home and End keys are listbox keyboard handling the task does not need
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll into view on a multiple select with a valid size is keyboard handling the task does not need, and it is no open list's rows
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple select Space and character keys are listbox keyboard handling the task does not need
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple select disabled rows and the unmodelled Ctrl+A are listbox behaviour the task does not need
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span seeding on a multiple select is listbox keyboard handling the task does not need
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and option_disabled are select readings the task keeps unchanged, since disabled-ness keying is not needed
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: recording of a multiple write or removal is mutator select bookkeeping the chunk does not touch
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select and the closed-set emptiness for select typing are unchanged, and the new read verb adds no select entry
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press without Ctrl is a driver limit the chunk does not change
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: the seven_guis stand buttons whose hook-local signals move onto the registry, counter-increment, flight-book and crud-create among them, must still change the screen on Enter and Space as this entry states
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace is Apple keybinding behaviour the chunk does not touch
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused is keyboard copy behaviour the chunk does not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission is text input key behaviour the chunk does not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click default action and focus of the bound select are pointer behaviour the chunk does not change
