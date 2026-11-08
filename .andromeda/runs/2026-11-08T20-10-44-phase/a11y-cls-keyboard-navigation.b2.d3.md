@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the details open flag and :open matching are not a state the snapshot model reads, which takes its states from accessibility_tree
@answer n=2 entity=keyboard-navigation#2.11 v=yes
why: a checkbox click toggles checked, a value the snapshot model reads and the replayed actions answer compares
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event dispatched after a click is engine event dispatch the scope does not name, and the coverage keys listeners, not dispatches
@answer n=4 entity=keyboard-navigation#2.13 v=yes
why: a range pointerdown sets the value the snapshot model reads, reached by the driver click the scope names, and the focus part is not needed
@answer n=5 entity=keyboard-navigation#3.1 v=yes
why: text input editing sets the value the replayed actions answer compares, reached by the driver type verb the scope names
@answer n=6 entity=keyboard-navigation#3.2 v=yes
why: range keys set the value the snapshot model reads and the replayed actions answer compares
@answer n=7 entity=keyboard-navigation#3.3 v=yes
why: a single select's keys set its value, which the snapshot model reads and the replayed answer compares
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open list's highlight moves and chooses nothing, so no value or state the snapshot reads changes
@answer n=9 entity=keyboard-navigation#3.5 v=yes
why: choosing a row by Enter, Space, Tab or click writes the select's value through apply_select_choice, which the driver click verb reaches
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling writes only the list's own scroll_y, which the snapshot model does not read
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list thumb is painted and takes no hit, outside the snapshot model and the scope
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select keys set row selection the scope does not name, and key behaviour the chunk does not change
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select changes no value or state the scope names
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character keys on a multiple select are key behaviour the chunk does not change
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: refusing disabled rows as a choice on a multiple select is a rule the scope does not name
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift seeding from the markup's selection is key behaviour the scope does not name
@answer n=17 entity=keyboard-navigation#3.13 v=yes
why: option_disabled is the one disabled reading that the snapshot's enabled state shares for optgroup rows
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple write recording is a mutator rule the chunk neither edits nor reads
@answer n=19 entity=keyboard-navigation#3.15 v=yes
why: the driver's type verb on a select is the verb the scope's disabled refusal names, and its closed-set rule gains the new refusal cause
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holds no Ctrl, and press is not named in the scope while its fix is owned by a residual
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: Enter and Space activation is key behaviour the chunk does not change, and the driver's disabled refusal happens before any dispatch
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace bindings are key handling the chunk does not change
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy without a focused text input is clipboard key handling the chunk does not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission is key handling the chunk does not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label click that focuses a select and opens nothing is a click rule on a label the scope does not name
