@answer n=1 entity=keyboard-navigation#2.10 v=no
why: its open-state rule lives in blitz-dom and tests/blitz-tests open_state.rs, none of which the session reaches
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: checkbox and radio click focus lives in blitz-dom pointer.rs, which the session does not reach
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the checkbox and radio change event is an engine dispatch in blitz-dom pointer.rs and its test, neither reached
@answer n=4 entity=keyboard-navigation#2.13 v=no
why: range-input click focus and value are blitz-dom pointer behaviour the session does not reach
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: text-input keys live in blitz-dom text.rs, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: range-input arrow and Home/End keys live in blitz-dom keyboard.rs, and the task marks range-input key paths unchanged
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: the single-select key path lives in blitz-dom keyboard.rs and select.rs, which the session does not reach, and the disabled rule it reads is unchanged
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the open-list key match lives in blitz-dom and select_popup tests, none of which the session reaches
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: choosing from an open list lives in blitz-dom keyboard.rs and pointer.rs, which the session does not reach
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: list scrolling lives in blitz-dom scrolling.rs, which the session does not reach
@answer n=11 entity=keyboard-navigation#3.7 v=yes mode=stale
why: docs/capabilities.md is reached whole and the session removed a not-built click line from it, so the cited lines 233-235 can have moved
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: multiple-select keys live in blitz-dom keyboard.rs and select.rs, which the session does not reach
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: scroll_into_view on a multiple select lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: multiple-select Space and character keys live in blitz-dom keyboard.rs, which the session does not reach
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: disabled rows and the unmodelled Ctrl+A on a multiple select live in blitz-dom, which the session does not reach
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: the Shift span from a seeded last chosen row lives in blitz-dom select.rs, which the session does not reach
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: optgroup option order and option_disabled live in blitz-dom select.rs and form.rs, which the session does not reach
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: multiple-write recording lives in blitz-dom mutator.rs, which the session does not reach
@answer n=19 entity=keyboard-navigation#3.15 v=yes mode=stale
why: execute.rs is reached whole and the driver's closed sets it cites moved (VERBS 12 to 13), so its cited lines 242-246 and 572-579 can have moved
@answer n=20 entity=keyboard-navigation#3.16 v=yes mode=stale
why: schema.rs is reached whole and its closed sets moved (VERBS 12 to 13, NODE_FIELDS 13 to 14), so the cited lines 270-275 can have moved
@answer n=21 entity=keyboard-navigation#3.17 v=yes mode=add-to
why: the Dioxus bridge now sets a SignalRegistry dispatch around the handler run this Enter and Space Click reaches, which F6 (use_graph_state writes resolved through the dispatch chain) joins
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: macOS Backspace bindings live in blitz-dom text.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: copy without a focused text input lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: implicit form submission lives in blitz-dom keyboard.rs, which the session does not reach
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: the label click default action lives in blitz-dom pointer.rs, which the session does not reach
