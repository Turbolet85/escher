@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the details and dialog open-state flip is engine state the read neither reads nor dispatches, and no scope item names it
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click toggling is what a click does, and the read dispatches no click
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click comes from a click the read never performs
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range click focus and value setting are the range pointer setting that not_needed item 11 excludes, and the read performs no click
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text-input arrow, Home, End and edit keys are key handling the read never dispatches (not_needed item 5)
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range-input key steps are key handling the read never dispatches (not_needed item 5)
@answer n=7 entity=keyboard-navigation#3.3 v=yes
why: the optgroup options being the select's own in tree order is the owner-served set the read lists under a select (scope item 9), while its key rules are not reached
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's move-key highlight is key handling, and the highlighted row is in no snapshot (scope item 9)
@answer n=9 entity=keyboard-navigation#3.5 v=yes
why: a click on a row of an open list chooses it and closes the list through click_list_row, which the owner-served row's verbs and causes must agree with (scope item 9)
@answer n=10 entity=keyboard-navigation#3.6 v=yes
why: the open list shows at most 20 rows and scrolls by its own scroll_y, which fixes which rows are in view and so the read's off-screen cause for them (scope item 9)
@answer n=11 entity=keyboard-navigation#3.7 v=yes
why: the list thumb takes no hit, and hit-testing decides the covered cause the read reports (scope item 8)
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple-select move keys and Shift or Ctrl spans are key handling the read never dispatches (not_needed item 5)
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: a key bringing the current row into view is key and scroll handling the read never dispatches
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and typeahead on a multiple select are key handling the read never dispatches (not_needed item 5)
@answer n=15 entity=keyboard-navigation#3.11 v=yes
why: a multiple select's disabled rows are never chosen, so the owner-served disabled row's refusal must agree with it (scope item 9), and Ctrl+A is not reached
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift spanning from the last chosen row is key handling the read never dispatches
@answer n=17 entity=keyboard-navigation#3.13 v=yes
why: a disabled optgroup's rows read as disabled through option_disabled, the one rule the owner-served row's disabled cause must agree with (scope item 9)
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the mutator's record of multiple-select writes and removals changes selectedness, which the read does not list
@answer n=19 entity=keyboard-navigation#3.15 v=yes
why: the driver's closed sets (verb table, causes, result fields) must hold no entry for a select's type, and the new read's verb entry is added to that same table
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the press help's Ctrl limit is another row of VERBS that only must keep holding, a re-check every change owes
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: a disabled attribute gates button and input but not summary or a, the keying the read's disabled cause follows (scope item 6), while the Enter and Space dispatch itself is not reached
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace routing is text-editing key handling the read never dispatches
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy shortcut with no text input focused is key handling the read never dispatches
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input is key handling the read never dispatches
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label click's default action on a bound select is a click the read never performs, and labels are not listed
