# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · yes · the scope names `<col span>` in 8b1a0a3ce, a page-reached reader the delta moves, under the closed-parser and clamped-numeric rules
input-validation#1.52 · no · `rowspan`'s clamp lives in table.rs, which the chunk does not reach and the scope does not name
input-validation#1.53 · yes · the size hint's cap, parse and select arms sit in the conflicted stylo.rs and mutator.rs and in select_popup's rows, which the chunk keeps
input-validation#1.54 · yes · the `<ol start>` read sits in construct.rs, a predicted conflict surface
input-validation#1.55 · no · the `attr_parsed` helper lives in element.rs, which the chunk does not reach and no scope line names
input-validation#1.56 · yes · the presence reading of `disabled` gates the select list's key and pointer opening, which the chunk keeps row by row in select_popup
input-validation#1.57 · yes · the `open` arms in mutator.rs set_attribute and clear_attribute sit on a predicted conflict surface
input-validation#1.58 · yes · the autofocus read at mutator.rs:1143-1155 sits on a predicted conflict surface
input-validation#1.59 · no · the case-insensitive `dir` match is pinned in dir_attribute.rs, which the chunk does not reach and no scope line names
input-validation#1.60 · yes · the canvas `src` u64 parse at mutator.rs:1451-1462 sits on a predicted conflict surface
input-validation#1.61 · yes · the stable element id rules meet the conflicted dioxus_document.rs and the stand_element_ids checks, and the id needles at 0 in the capture checks the chunk re-measures
input-validation#1.62 · yes · the name no-log rule is witnessed by the capture checks at 0 name needles the chunk re-measures, and select_popup's label-click row lies in the select surface the chunk keeps
input-validation#1.63 · no · description reads are pinned in accessibility_description and touch no reached file, no scope line and no witness the chunk re-runs
input-validation#1.64 · no · the aria-expanded, aria-disabled and aria-level closed parses sit in accessibility.rs, which the chunk does not reach, and no witness the scope names covers them
input-validation#1.65 · yes · the single select's `open` and `aria-expanded` steering nothing is pinned by a select_popup row the chunk keeps and re-runs
input-validation#1.66 · no · the control-state readings sit in accessibility.rs and their witnesses, accessibility_states, are not named in the scope
input-validation#1.67 · no · the native checkbox and radio checked-state write sits in accessibility.rs, which the chunk does not reach and no scope line names
input-validation#1.68 · yes · the inert focus clear in DocumentMutator::flush sits on a predicted conflict surface, and the inert reads in node.rs are conflicted too
input-validation#1.69 · yes · the select_list_allowed check set decides which select opens the list the chunk keeps and re-reads row by row in select_popup
input-validation#1.70 · yes · the inert focus clear in mutator.rs:1051-1063 sits on a predicted conflict surface
