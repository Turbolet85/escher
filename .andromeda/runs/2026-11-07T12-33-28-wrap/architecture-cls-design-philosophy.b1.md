@answer n=1 entity=design-philosophy#1.1 v=no
why: the dioxus-native-dom crate-description claim at lib.rs:3 sits above the new mod line at 21 and still holds, and Cargo.toml is not reached
@answer n=2 entity=design-philosophy#2.1 v=yes mode=stale
why: the 7-line insert in dioxus_document.rs moved the cited lines 58 and 84-85, while the DioxusDocument, handle_ui_event and poll claim still holds
@answer n=3 entity=design-philosophy#3.1 v=no
why: its spec and browser-engine citations all sit in blitz-dom, blitz-paint and other crates the change does not reach
@answer n=4 entity=design-philosophy#4.1 v=no
why: the buffered render adds no invalidation path and its cited blitz-dom pipeline citations sit in unreached files
@answer n=5 entity=design-philosophy#5.1 v=no
why: the headless harness claim and the tests/ directory location still hold, the probe file staying in tests/ and this row citing no count
