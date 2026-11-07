@answer n=1 entity=keyboard-navigation#2.10 v=no
why: the session reaches only the Dioxus bridge files and the probe, not the open-state flush, element or mutator code these lines cite, and the replay keeps the same mutations in the same order
@answer n=2 entity=keyboard-navigation#2.11 v=no
why: the session touches no pointer click or checkbox and radio focus code in packages/blitz-dom/src/events/pointer.rs
@answer n=3 entity=keyboard-navigation#2.12 v=no
why: the session does not change handle_click's checkbox and radio change dispatch or the change-event tests it cites
@answer n=4 entity=keyboard-navigation#2.13 v=yes mode=add-to
why: F1, a task polled during a Dioxus render acts on the document as the previous render left it, so MountedData set_focus on a node created in that render does not focus it (set_focus_to returns false), joins this focus-restoration group as an open residual row
@answer n=5 entity=keyboard-navigation#3.1 v=no
why: the session changes no text input key handling in packages/blitz-dom/src/node/text.rs
@answer n=6 entity=keyboard-navigation#3.2 v=no
why: the session changes no range key handling in packages/blitz-dom/src/events/keyboard.rs, and the engine's key rules stay unchanged
@answer n=7 entity=keyboard-navigation#3.3 v=no
why: the session changes no single select key, typeahead or list-opening rule, which the task holds unchanged
@answer n=8 entity=keyboard-navigation#3.4 v=no
why: the session changes no open-list key match or highlight rule in packages/blitz-dom/src/events/keyboard.rs
@answer n=9 entity=keyboard-navigation#3.5 v=no
why: the session changes no choose-and-close path, apply_select_choice or click_list_row, that these lines cite
@answer n=10 entity=keyboard-navigation#3.6 v=no
why: the session changes no list row cap, scroll_y write or scrolling.rs code these lines cite
@answer n=11 entity=keyboard-navigation#3.7 v=no
why: the session changes no list thumb paint or hit code, and the task holds paint and styles unchanged
@answer n=12 entity=keyboard-navigation#3.8 v=no
why: the session changes no multiple select key rule in keyboard.rs or select.rs
@answer n=13 entity=keyboard-navigation#3.9 v=no
why: the session changes no scroll_into_view gate or multiple select row semantics
@answer n=14 entity=keyboard-navigation#3.10 v=no
why: the session changes no Space or character rule for a multiple select
@answer n=15 entity=keyboard-navigation#3.11 v=no
why: the session changes no disabled-row rule or Ctrl+A modelling in the select code
@answer n=16 entity=keyboard-navigation#3.12 v=no
why: the session changes no Shift anchor rule in packages/blitz-dom/src/node/select.rs
@answer n=17 entity=keyboard-navigation#3.13 v=no
why: the session changes no optgroup option ordering or option_disabled rule
@answer n=18 entity=keyboard-navigation#3.14 v=no
why: the session changes no multiple write or removal record in mutator.rs, and the replay applies the same mutations in the same order
@answer n=19 entity=keyboard-navigation#3.15 v=no
why: the session reaches no driver code in packages/escher-driver/src/execute.rs or its closed sets
@answer n=20 entity=keyboard-navigation#3.16 v=no
why: the session reaches no driver press schema in packages/escher-driver/src/schema.rs, and its owner res-hfvjj77l is not one the wrap re-says
@answer n=21 entity=keyboard-navigation#3.17 v=no
why: Enter and Space activation is untouched by the task, and the Click dispatch in events/keyboard.rs and events/mod.rs is not among the session's reached files
@answer n=22 entity=keyboard-navigation#3.18 v=no
why: the session changes no macOS Backspace handling in packages/blitz-dom/src/node/text.rs
@answer n=23 entity=keyboard-navigation#3.19 v=no
why: the session changes no copy shortcut code in packages/blitz-dom/src/events/keyboard.rs
@answer n=24 entity=keyboard-navigation#3.20 v=no
why: the session changes no implicit form submission rule for text inputs in packages/blitz-dom/src/events/keyboard.rs
@answer n=25 entity=keyboard-navigation#3.21 v=no
why: the session changes no label click path in packages/blitz-dom/src/events/pointer.rs
