@answer n=1 entity=keyboard-navigation#2.10 v=no
why: its open-state rule lives in blitz-dom element and mutator code, which the session leaves alone; the change is confined to the Dioxus bridge and the probe
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click focus and toggle live in blitz-dom pointer events, which the session does not reach
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the change event after a checkbox or radio click lives in blitz-dom pointer events, which the session does not reach
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range-input click focus and value live in blitz-dom pointer events, outside the Dioxus bridge the session changed
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text-input key handling lives in blitz-dom node text code, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range-input arrow and Home/End keys live in blitz-dom keyboard events, which the session does not reach
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: single-select keys and typeahead live in blitz-dom keyboard and select code, which the session does not reach
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: open-list keys live in blitz-dom keyboard events and the select popup tests, none of them in a file the session reached
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: list choose-and-close writes through blitz-dom pointer and keyboard events, which the session does not reach
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling lives in blitz-dom scrolling.rs, which the session does not reach
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: list thumb painting and hit testing live in blitz-dom document.rs and its docs, which the session does not reach
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple-select arrow, Home and End keys live in blitz-dom keyboard and select code, which the session does not reach
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: multiple-select scroll-into-view lives in blitz-dom keyboard events, which the session does not reach
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple-select Space and character keys live in blitz-dom keyboard events, which the session does not reach
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: multiple-select disabled rows and the unmodelled Ctrl+A live in blitz-dom keyboard and select code, which the session does not reach
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: multiple-select Shift spanning lives in blitz-dom select code, which the session does not reach
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and disabled reads live in blitz-dom select and form code, which the session does not reach
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple-select write recording lives in blitz-dom mutator code, which the session does not reach
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the driver's type on a select lives in escher-driver execute.rs and its stand test, neither reached by the session
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the driver's press holding no Ctrl lives in escher-driver schema.rs, which the session does not reach
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: Enter and Space click dispatch lives in blitz-dom keyboard and events code and the stand keyboard tests, none reached by the session
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace left to the Apple keybindings lives in blitz-dom text.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy with no text input focused lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: label click focus on a single select lives in blitz-dom pointer events, which the session does not reach
