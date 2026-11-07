@answer n=1 entity=keyboard-navigation#2.10 v=yes
why: the replay's falsy removal of the boolean open attribute must take the mutator removal that runs flush_open_state, so :open tracks the replayed write
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click focus is a pointer path the scope does not name and the chunk does not drive
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click is a pointer path the scope does not name
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input click focus and value are pointer behaviour the scope does not name
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input key handling is not named by the scope, which names Tab focus and a key-handler-spawned task only
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range arrow and Home/End steps are an engine key path the scope does not name, and its disabled reading is the engine's, unchanged
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select key handling and its changed-set marks are not named by the scope
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open list's key match is outside the scope, which names no open list
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a row from an open list is outside the scope, which names no open list
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: keyboard scrolling of a list writes the list's own scroll_y, a path the chunk's MountedData calls do not reach
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list thumb's paint and hit rules are not touched by the chunk
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: arrow, Home and End handling on a multiple select is a key path the scope does not name
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select's key is outside the scope
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character handling on a multiple select is a key path the scope does not name
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: disabled-row choice and Ctrl+A on a multiple select are not named by the scope
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row is a key path the scope does not name
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and option_disabled reads are untouched and not named by the scope
@answer n=18 entity=keyboard-navigation#3.14 v=yes
why: the mutator's multiple write and removal record the select, and the replay's falsy removal of the boolean multiple attribute must take that same path
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select is a driver verb outside the chunk's harness
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holding no Ctrl is a driver limit outside the chunk
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: the Enter and Space default click is not named by the scope, which names Tab focus and a key-handler-spawned task
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace keybinding is platform text editing the chunk does not touch
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy without a focused text input is outside the chunk
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input is outside the chunk
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label click focusing a select is a pointer path the scope does not name
