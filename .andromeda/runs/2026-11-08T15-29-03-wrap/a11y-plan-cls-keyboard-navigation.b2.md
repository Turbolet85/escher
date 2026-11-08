@answer n=1 entity=keyboard-navigation#2.10 v=no
why: its open-state rule lives in blitz-dom and open_state.rs, none of which the session reached
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click focus lives in blitz-dom pointer.rs and harness_smoke.rs, not reached
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after checkbox and radio clicks is blitz-dom pointer.rs behaviour, and the session left blitz-dom untouched
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range-input click focus is blitz-dom pointer.rs behaviour, which the session did not change
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text-input key handling lives in blitz-dom text.rs, which the session did not change
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range-input arrow, Home and End keys live in blitz-dom keyboard.rs, which the session did not change
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single-select key handling and typeahead live in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open list's key match lives in blitz-dom keyboard.rs, which the session did not change
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: list choice through Enter, Space, Tab and click lives in blitz-dom keyboard.rs and pointer.rs, which the session did not change
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: the list row limit and its scrolling live in blitz-dom scrolling.rs and select_popup.rs, none reached
@answer n=11 entity=keyboard-navigation#3.7 v=yes mode=stale
why: its citation docs/capabilities.md:248-250 sits in a reached file whose lines can have moved, while the thumb and key-move claim itself is unchanged
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple-select arrow, Home and End keys live in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll-into-view for a multiple select lives in blitz-dom keyboard.rs, which the session did not change
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple-select Space and character keys live in blitz-dom keyboard.rs, which the session did not change
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: disabled rows and Ctrl+A on a multiple select live in blitz-dom keyboard.rs and select.rs, which the session did not change
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: Shift span from the last chosen row lives in blitz-dom select.rs, which the session did not change
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and disabled groups live in blitz-dom select.rs and form.rs, which the session did not change
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple-select write and removal recording lives in blitz-dom mutator.rs, which the session did not change
@answer n=19 entity=keyboard-navigation#3.15 v=yes mode=stale
why: the driver's type arm in reached execute.rs now runs check_target first, and the cited execute.rs:645-652 and :251-255 lines can have moved
@answer n=20 entity=keyboard-navigation#3.16 v=yes mode=stale
why: its help citation schema.rs:270-275 sits in reached schema.rs, whose verb table and counts the session moved, so the line can have moved
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: Enter and Space click activation lives in blitz-dom keyboard.rs and the stand keyboard tests, none reached, and press is not an id-taking arm
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace handling lives in blitz-dom text.rs, which the session did not change
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy without a focused text input lives in blitz-dom keyboard.rs, which the session did not change
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission lives in blitz-dom keyboard.rs, which the session did not change
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click focus on a single select lives in blitz-dom pointer.rs, which the session did not change
