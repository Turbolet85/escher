@answer n=1 entity=keyboard-navigation#2.10 v=no
why: a details or dialog open state is native element state the graph does not read, and the chunk changes no open-state rule
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling and focus is pointer behaviour the chunk changes no rule of (not-needed 3 and 10)
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the engine's change event after a checkbox or radio click is event behaviour the graph does not read, and the chunk changes no click behaviour
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range pointer focus and the range pointer value setting are not needed by the chunk (not-needed 10)
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input keys are key behaviour the chunk does not change (not-needed 2), and the masking rule rules over values, not keys
@answer n=6 entity=keyboard-navigation#3.2 v=yes
why: the range's current value is held apart from the value attribute, so the control value the graph carries for a range must read that current value (its key rules are not needed)
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys, typeahead and the input and change dispatch are key behaviour the chunk does not change (not-needed 2)
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's own closed key match is key behaviour the chunk does not change (not-needed 2)
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: Enter, Space and Tab choosing a row in an open list is key behaviour the chunk does not change, and the stand's Tab orders are table values, not this rule
@answer n=10 entity=keyboard-navigation#3.6 v=yes
why: an open list shows at most 20 rows and a row brought into view writes the list's own scroll_y, which fixes the open list's row bounds the graph carries (scope item 10)
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list's painted thumb takes no hit and is no graph node, so no bounds or reading of the graph depends on it
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select arrow, Home and End keys are key behaviour the chunk does not change (not-needed 2)
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select by key is key behaviour the chunk does not change (not-needed 2)
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple select Space and typeahead are key behaviour the chunk does not change (not-needed 2)
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple select disabled rows and the unmodelled Ctrl+A are key behaviour the chunk does not change (not-needed 2 and 3)
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift spanning from the last chosen row is key behaviour the chunk does not change (not-needed 2)
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup key and text reach is key behaviour the chunk does not change, and its disabled read is the upstream disabled read (not-needed 2 and 4)
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple write and removal recording is mutator bookkeeping the chunk does not touch
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the closed-set rule covers no verb for a type on a select, and the chunk adds a graph read verb, not one for that type
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver press holding no Ctrl is a driver limit with its own residual, not a rule the graph or its carriers need
@answer n=21 entity=keyboard-navigation#3.17 v=no standing=S2
why: the stand key-click checks stated as 2 buttons by 2 keys, 4 rows, in both layout modes are the stand-check convention that S2 names
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace left to the Apple keybindings is key behaviour the chunk does not change
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused is key behaviour the chunk does not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input is key behaviour the chunk does not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label click focusing a single select and opening nothing is pointer focus and key behaviour the chunk does not change (not-needed 10)
