@answer n=1 entity=keyboard-navigation#2.10 v=no
why: details and dialog open state and a summary's click toggle; the read walks the snapshot tree and dispatches no click
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click effects; the read performs no click and reads no toggled state
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click; no click is dispatched by the read
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range input pointer focus and value setting on click; the read performs no click and reads no value
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text input key handling; the read dispatches no key
@answer n=6 entity=keyboard-navigation#3.2 v=yes
why: the disabled attribute keyed by presence on an input range, which the read's disabled cause follows; its key steps are not needed
@answer n=7 entity=keyboard-navigation#3.3 v=yes
why: optgroup options are the select's own in tree order, so they are owner-served rows, and the select's disabled gating reads is_disabled
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: open-list key highlight movement; the read dispatches no key and lists no highlight
@answer n=9 entity=keyboard-navigation#3.5 v=yes
why: a click on an open list's row chooses it and closes the list, the owner-served click the scope names
@answer n=10 entity=keyboard-navigation#3.6 v=yes
why: an open list shows at most 20 rows and its own scroll_y bounds which owner-served rows are off-screen
@answer n=11 entity=keyboard-navigation#3.7 v=yes
why: the list's painted thumb takes no hit, which bears on what covered means for a target under it
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple select key selection and chord moves; the read dispatches no key
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: key-driven scroll into view gated by carries_valid_size; the read scrolls nothing
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: Space and typeahead choosing on a multiple select; keys the read does not dispatch
@answer n=15 entity=keyboard-navigation#3.11 v=yes
why: a multiple select's disabled rows are never chosen, the disabled-row-changes-nothing behaviour the owner-served rows and their causes must agree with
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row is a key step; the read reads no selectedness
@answer n=17 entity=keyboard-navigation#3.13 v=yes
why: rows of a disabled optgroup read disabled through option_disabled, the one rule the read's disabled cause must share, and the rows are the select's own
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: settling of a multiple select's selectedness on write; the read lists no selected state
@answer n=19 entity=keyboard-navigation#3.15 v=yes
why: the driver's type on any select focuses it and the driver's closed sets hold no verb, cause or result field for it, which the new verb entry and causes must keep
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the press verb's no-Ctrl limit is another verb-table row that must only keep holding, and the read adds no press change
@answer n=21 entity=keyboard-navigation#3.17 v=yes
why: a disabled attribute on a summary or an a gates neither key, so the read's disabled cause must not refuse those elements by presence
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace left to Apple keybindings; a text editing key the read does not dispatch
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the copy shortcut with no text input focused; no key is dispatched by the read
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission from a text input; no key is dispatched by the read
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: a label's default click and focus of its bound select; the read performs no click and lists no label
