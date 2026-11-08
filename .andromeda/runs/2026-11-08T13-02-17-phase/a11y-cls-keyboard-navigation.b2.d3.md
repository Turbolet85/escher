@answer n=1 entity=keyboard-navigation#2.10 v=yes
why: a closed details' contents are display none only where :open reads false, and the read's display-none exclusion depends on that reading
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling and focus is click behaviour the read never performs
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click, which the read does not dispatch
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input click focus and value setting, and the read performs no click
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input arrow, Home, End and editing keys, and the read dispatches no key
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range input stepping keys, and the read dispatches no key
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single select arrow, Home, End and typeahead keys, and the read dispatches no key
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: an open list's key match and highlight moves, and the highlight is in no snapshot while the read dispatches no key
@answer n=9 entity=keyboard-navigation#3.5 v=yes
why: a click on a row of an open list chooses it through click_list_row, and the owner-served row's click verdict must agree with that
@answer n=10 entity=keyboard-navigation#3.6 v=yes
why: an open list shows at most MAX_ROWS 20 rows, so a row outside that visible window is off-screen to the owner-served row's refusal cause
@answer n=11 entity=keyboard-navigation#3.7 v=yes
why: the list's painted thumb takes no hit, which bounds what covered means for a list row under hit-testing
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: Shift, Ctrl and Meta key behaviour on a multiple select, and the read dispatches no key
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: a key-driven scroll_into_view of a multiple select's current row, and the read runs no key and reads the rows as they stand
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and character keys on a multiple select, and the read dispatches no key
@answer n=15 entity=keyboard-navigation#3.11 v=yes
why: a multiple select's disabled rows are never chosen, the disabled row changing nothing that the owner-served row's verbs and causes must agree with
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row, a key rule the read does not run
@answer n=17 entity=keyboard-navigation#3.13 v=yes
why: optgroup options are the select's own in tree order, and a disabled group's rows read as disabled through option_disabled, which the read's disabled cause must match
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the mutator's record of a multiple write is engine selectedness bookkeeping the read does not take
@answer n=19 entity=keyboard-navigation#3.15 v=yes
why: the driver's closed sets that the change edits, VERBS and the cause words, must hold no verb or cause for a select's typing path
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: press holds no Ctrl, a limit in press's own help, another verb unit the change does not edit
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: Enter and Space click activation through keys, and the read dispatches no key while the disabled keying excludes summary and a as need 6 states
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace bindings to editor actions, and the read dispatches no key
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy shortcut with no text input focused, and the read dispatches no key
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input, and the read submits nothing
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click routing to a single select, and the read performs no click while labels own no options
