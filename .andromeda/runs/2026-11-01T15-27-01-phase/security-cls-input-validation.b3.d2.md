# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · yes · the `<col span>` reader the delta moves (table.rs) binds under the scope's closed-parser and clamped-numeric rules; the rows, cols and replaced-element parts are unmoved
input-validation#1.52 · no · rowspan's clamp lives in table.rs, which the chunk does not reach, and the scope names no rowspan reader
input-validation#1.53 · yes · the multiple-select size hint, its 65534 cap, its default.css rules and its scroll_into_view call are the ratified widening the scope keeps in stylo.rs
input-validation#1.54 · yes · the `ol start` parse and its minus-one sit in construct.rs, a conflict surface
input-validation#1.55 · yes · attr_parsed's return-None on parse failure is the closed-parser rule the scope names for the moved SVG and `<col span>` readers
input-validation#1.56 · yes · the disabled presence reading governs the range arms the scope keeps within their rules, and the stand_keyboard witness is re-run by the chunk
input-validation#1.57 · yes · the open arm of mutator.rs set_attribute and the `:open` match in stylo.rs are conflict surfaces the scope's range and select arm clause keeps
input-validation#1.58 · yes · autofocus's presence rule sits in mutator.rs, a conflict surface
input-validation#1.59 · no · dir's case-insensitive match cites only dir_attribute.rs, which the chunk does not reach, and the scope names no dir reader
input-validation#1.60 · yes · the canvas src u64 parse sits in mutator.rs, a conflict surface
input-validation#1.61 · yes · the stable element id's rules sit in dioxus_document.rs, a conflict surface the scope names, with the no-NodeId crossing and the sink's 0-needle capture checks
input-validation#1.62 · yes · the aria-labelledby and label name reads feed the sink's 0-needle name capture the scope re-runs, and a select's label click opening nothing is select-list behaviour the scope keeps
input-validation#1.63 · no · the aria-description builder sits in accessibility.rs, which the chunk does not reach, and the scope names no description reader or check
input-validation#1.64 · no · aria-expanded, aria-disabled and aria-level sit in accessibility.rs, unreached, and the scope's select-list clause names keys and painted labels, not these states
input-validation#1.65 · yes · an author's open and aria-expanded on a select steer nothing, a select-list behaviour the scope keeps unchanged across the take
input-validation#1.66 · no · the aria-selected, aria-pressed, aria-checked and aria-invalid readers sit in accessibility.rs, unreached, and the scope names none of them
input-validation#1.67 · no · the native checkbox and radio checked state sits in accessibility.rs, unreached, and the scope names no such reader
input-validation#1.68 · yes · inert's presence rule and its focus clear sit in mutator.rs flush and node.rs hit_inner, both conflict surfaces
input-validation#1.69 · yes · the select list's open gate and row rules are the select-list behaviour the scope keeps unchanged, its skipped row lifting only with the panic's fix
input-validation#1.70 · yes · the tree's focus and the inert focus clear sit in mutator.rs flush, a conflict surface
