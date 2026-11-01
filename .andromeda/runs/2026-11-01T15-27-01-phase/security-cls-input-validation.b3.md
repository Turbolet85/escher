# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · yes · the `<col span>` floored-at-1 reader is one the take moves (8b1a0a3ce), under the closed-parser and clamped-numeric markup rules the scope names
input-validation#1.52 · no · the rowspan clamp lives in table.rs, which the take does not reach, and the scope names no moved rowspan reader
input-validation#1.53 · yes · the `size` hint's cap and the multiple-select row rules sit in stylo.rs and default.css, both conflict surfaces, and the scope names the ratified size widening
input-validation#1.54 · yes · the `<ol start>` parse-and-subtract rule lives in construct.rs, a predicted conflict surface
input-validation#1.55 · yes · attr_parsed's None on parse failure is the closed-parser behaviour the scope names for the moved markup readers (`<col span>`, SVG and CustomWidget attributes)
input-validation#1.56 · no · the `disabled` presence reading lives in element.rs, pointer.rs and keyboard.rs, none reached, and the scope names no disabled behaviour
input-validation#1.57 · yes · the `open` arms of set_attribute and clear_attribute sit in mutator.rs, a conflict surface, with the open arm's order after the range re-sanitize arm
input-validation#1.58 · yes · the autofocus rule lives in mutator.rs:1143-1155, a conflict surface
input-validation#1.59 · no · the case-insensitive dir match is witnessed only by dir_attribute.rs, which the chunk does not reach, and no named rule covers it
input-validation#1.60 · yes · the canvas `src` u64 parse lives in mutator.rs, a conflict surface
input-validation#1.61 · yes · the stable element id rules are the dioxus_document.rs identity surface the scope names, proven by stand_element_ids and the snapshot checks, and the sink drop of Stylo's id-bearing targets is the format.rs re-read the chunk owes
input-validation#1.62 · yes · the name needles at 0 on the capture checks are a value the chunk itself re-measures, so the entity binds through that witness
input-validation#1.63 · no · the description builder and its adapter-only exit sit in accessibility.rs, not reached, and no name or needle check covers description text; the sink re-read covers only new call sites of the taken delta
input-validation#1.64 · no · aria-expanded, aria-disabled and aria-level are read in accessibility.rs, not reached, and the scope names none of them
input-validation#1.65 · no · an author's open and aria-expanded steering nothing on a select is attribute and accessibility behaviour, not a select key or painted label, and its test row in select_popup.rs is an unedited row the edit does not name
input-validation#1.66 · no · the control-state readings sit in accessibility.rs and snapshot.rs, neither reached, and the scope names no state; the mask clause restates the masking rule the scope names on its own
input-validation#1.67 · no · the native checkbox and radio checked-state write sits in accessibility.rs, not reached, and no conflict or named item touches it
input-validation#1.68 · yes · Node::is_inert sits in node.rs and the inert focus clear in DocumentMutator::flush sits in mutator.rs, both conflict surfaces
input-validation#1.69 · yes · select_list_allowed decides which select opens a list and what a row takes, the select list's keys the scope keeps unchanged across its sixteen-file surface
input-validation#1.70 · yes · the inert focus clear in mutator.rs:1051-1063 is a conflict surface, and the tree's focus rule rides on it
