# Classified — security-plan · input-validation.b3

## Answers
input-validation#1.51 · no · cites layout/mod.rs and layout/table.rs, neither reached by the session; the numeric-attribute reads are untouched
input-validation#1.52 · no · cites layout/table.rs only, not reached; the rowspan clamp is untouched
input-validation#1.53 · yes · stale · stylo.rs and default.css are merge surfaces the take reached whole, so the size reading and its cited lines may have moved
input-validation#1.54 · yes · stale · construct.rs is a merge surface the take reached whole, so the `<ol start>` read at :480-486 may have moved
input-validation#1.55 · yes · stale · element.rs is a merge surface the take reached whole, so attr_parsed's cited lines :502-505 and :742-744 may have moved
input-validation#1.56 · yes · stale · element.rs is a merge surface the take reached whole, so the disabled read it cites may have moved
input-validation#1.57 · yes · stale · element.rs, mutator.rs and stylo.rs are merge surfaces the take reached whole, so the open read's cited lines may have moved
input-validation#1.58 · yes · stale · mutator.rs is a merge surface the take reached whole, so the autofocus read at :1215-1227 may have moved
input-validation#1.59 · no · cites only dir_attribute.rs, a test file the take does not reach; the case-insensitive match is untouched
input-validation#1.60 · yes · stale · mutator.rs is a merge surface the take reached whole, so the canvas src read at :1525-1536 may have moved
input-validation#1.61 · no · its id derivation lives in element_id.rs and the readers it cites, none reached by the session; no author-key or id read changes
input-validation#1.62 · yes · stale · document.rs is a merge surface the take reached whole, and this entity cites its :695-733 and :3531-3545, which may have moved
input-validation#1.63 · no · cites accessibility.rs and its test, neither reached; the description reads are untouched
input-validation#1.64 · no · cites accessibility.rs and snapshot.rs, neither reached; the aria-expanded, aria-disabled and aria-level reads are untouched
input-validation#1.65 · yes · stale · flush_open_state in element.rs, a merge surface the take reached whole, is the rule its open-on-select claim rests on, so it may have moved
input-validation#1.66 · no · cites accessibility.rs, snapshot.rs and snapshot_text.rs, none reached; the control-state reads are untouched
input-validation#1.67 · no · cites accessibility.rs and its test, neither reached; the native checked-state write is untouched
input-validation#1.68 · yes · stale · mutator.rs and node.rs are merge surfaces the take reached whole, so the inert reads at mutator.rs:1123-1135 and node.rs:574-591 may have moved
input-validation#1.69 · yes · stale · document.rs is a merge surface the take reached whole, so the select_list_allowed cite at :772-806 may have moved
input-validation#1.70 · yes · stale · mutator.rs is a merge surface the take reached whole, so the inert focus clear cited at :1123-1135 may have moved
