@answer n=1 entity=input-validation#1.51 v=no
why: the SVG width and height rule lives in packages/blitz-dom/src/stylo.rs, which the session did not change, and its lock and ignore moves reach no markup parsing
@answer n=2 entity=input-validation#1.52 v=no
why: the border and body margin parses at packages/blitz-dom/src/stylo.rs are unchanged, and the session's changed files are only the new probe test, Cargo.lock and deny.toml
@answer n=3 entity=input-validation#1.53 v=no
why: the textarea, replaced-element, col span and colspan parses sit in blitz-dom layout files that the session did not change
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp at packages/blitz-dom/src/layout/table.rs:630-634 is unchanged, and no session change touches table layout
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple-select size rule lives in blitz-dom's select_list, stylo and keyboard sources and in select_popup.rs, none changed by the session
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse at packages/blitz-dom/src/layout/construct.rs:480-486 is unchanged by the session
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed in packages/blitz-dom/src/node/element.rs is unchanged, and the session adds no attribute parse
@answer n=8 entity=input-validation#1.58 v=no
why: the presence reading of disabled lives in blitz-dom's element.rs and pointer.rs and in the focusability and stand tests, all unchanged by the session
@answer n=9 entity=input-validation#1.59 v=no
why: the open presence reading in element.rs and mutator.rs and the open_state tests are unchanged, and the session touches no open-state code
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus is read in packages/blitz-dom/src/mutator.rs:1215-1227, which the session did not change
@answer n=11 entity=input-validation#1.61 v=no
why: the dir match is a parse in blitz-dom code and in dir_attribute.rs, neither changed by the session
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src u64 parse at packages/blitz-dom/src/mutator.rs:1525-1536 is unchanged by the session
@answer n=13 entity=input-validation#1.63 v=no
why: the set_custom_widget attribute replay in element.rs and mutator.rs is unchanged, behind a feature the session does not touch
@answer n=14 entity=input-validation#1.64 v=no
why: element_id and the id's exits sit in blitz-dom and dioxus-native-dom sources the session did not change, and the Dioxus 0.7.10 versions it re-confirms move no id rule
@answer n=15 entity=input-validation#1.65 v=no
why: the aria-label and label-for reads in accessibility.rs and document.rs are unchanged, and the session adds no accessible-name code
@answer n=16 entity=input-validation#1.66 v=no
why: the aria-describedby and aria-description reads in accessibility.rs are unchanged by the session
@answer n=17 entity=input-validation#1.67 v=no
why: the aria-expanded, aria-disabled and aria-level parses in accessibility.rs and snapshot.rs are unchanged by the session
@answer n=18 entity=input-validation#1.68 v=no
why: the select open and aria-expanded steering lives in accessibility.rs and select_popup.rs, neither changed by the session
@answer n=19 entity=input-validation#1.69 v=no
why: the control-state readings in accessibility.rs and snapshot.rs are unchanged, and the session adds no state or role read
@answer n=20 entity=input-validation#1.70 v=no
why: the native checkbox and radio checked-state write at packages/blitz-dom/src/accessibility.rs:199-224 is unchanged by the session
@answer n=21 entity=input-validation#1.71 v=no
why: the inert presence check in node.rs, mutator.rs and accessibility.rs is unchanged, and inert_subtree.rs is not touched
@answer n=22 entity=input-validation#1.72 v=no
why: select_list_allowed in packages/blitz-dom/src/document.rs and the select_popup pins are unchanged by the session
@answer n=23 entity=input-validation#1.73 v=no
why: the tree focus write in accessibility.rs and the inert focus clear in mutator.rs are unchanged by the session
@answer n=24 entity=input-validation#1.74 v=no
why: the range parse, sanitize and default actions in range.rs, keyboard.rs, pointer.rs and mutator.rs are unchanged, and the session adds no input control code
@answer n=25 entity=input-validation#1.75 v=no
why: the Enter and Space activation arms in keyboard.rs, events/mod.rs and pointer.rs are unchanged by the session
