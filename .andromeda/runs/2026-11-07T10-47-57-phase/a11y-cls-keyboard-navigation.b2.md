@answer n=1 entity=keyboard-navigation#2.10 v=yes
why: the replay's boolean open write must reach the mutator path that flushes the OPEN state, per task item 8's same-path routing
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox click focus and toggling sit on the pointer path the chunk leaves unchanged
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the Change dispatch after a checkbox or radio click is a pointer default action the chunk does not reach
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range click focus and value are on the pointer path the chunk does not touch
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input keys are the text path, unchanged by the buffered render
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input keys are keyboard default actions the chunk does not change
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys, typeahead and list opening are not changed by the chunk
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open list's key match is engine select behaviour the chunk does not change
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a list row through Enter, Space or Tab is select behaviour the chunk does not change
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling by key and wheel is select behaviour the chunk does not reach
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list thumb's paint and hit rules are not touched by the chunk
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select keys are select default actions the chunk does not change
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select's key is select behaviour the chunk does not reach
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character typeahead on a multiple select are select keys the chunk does not change
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: disabled rows of a multiple select and Ctrl+A are select rules the chunk does not reach
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row is select behaviour the chunk does not change
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and disabled reading are select rules the chunk does not change
@answer n=18 entity=keyboard-navigation#3.14 v=yes
why: the replay's multiple attribute write and removal must reach the mutator path that records the select, per task item 8's same-path routing
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select is a driver verb whose focus and close path the chunk does not change
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holding no Ctrl is a driver limit the chunk does not change
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: a pressed Enter or Space on the Tab-focused key target runs the Click that the key handler hears, a reading the scope leaves open
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace left to Apple bindings is text editing the chunk does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused is a clipboard key the chunk does not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input is a keyboard default action the chunk does not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label's click focusing a single select is a pointer path the chunk does not reach
