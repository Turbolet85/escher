@answer n=1 entity=keyboard-navigation#2.10 v=yes
why: a summary's click toggling its details is a native click reaction the reacts-to-click field must report; the open flag and :open match do not bind
@answer n=2 entity=keyboard-navigation#2.11 v=yes
why: checkbox and radio clicks toggle state, a native click reaction the reacts-to-click field must report; the focus-on-click part is out of scope
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click is engine dispatch that the graph does not read, its handlers being app listeners
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input pointer focus and pointer value setting are pointer behaviour the scope names as not needed
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text-input editing keys, a key behaviour the chunk does not change, and the masking rule does not reach typing
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input arrow and Home End keys, a key behaviour the chunk does not change
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select keys and typeahead writes, a key behaviour the chunk does not change
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list closed key match, select keyboard handling the chunk does not change
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing a popup row by key or click is the engine select write path painted over the page, not a screen node the graph reads
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling by key and wheel writes the list's own scroll_y, a behaviour the chunk does not change
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the list thumb is paint and hit behaviour the scope leaves unchanged
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select arrow, Home, End and span keys, a key behaviour the chunk does not change
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll into view on a multiple select key, a key behaviour the chunk does not change
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character choice on a multiple select, a key behaviour the chunk does not change
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple select disabled rows and the unmodelled Ctrl A, selection behaviour the chunk does not change
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row, a selection key behaviour the chunk does not change
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option disabled reading through the select's shared rule, which the chunk does not change and the scope excludes as disabled-ness keying
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the mutator's multiple select write record, which no graph reading or new code of the chunk reaches
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select and the closed sets' lack of entries for it, which the chunk's new verb does not alter
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holds no Ctrl and its help text, and the chunk adds no press behaviour
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: the Enter and Space Click set for button, summary, submit reset and button inputs, a with href, and checkbox or radio on Space, none disabled, is what the reacts-to-click field and role derivation must report for native elements
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace keybinding mapping in text editing, which the chunk does not touch
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused, clipboard key behaviour the chunk does not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input, a key behaviour the chunk does not change
@answer n=25 entity=keyboard-navigation#3.21 v=yes
why: a label click relays to its labelable control, a native click reaction the reacts-to-click field must report; the select focus part is pointer behaviour out of scope
