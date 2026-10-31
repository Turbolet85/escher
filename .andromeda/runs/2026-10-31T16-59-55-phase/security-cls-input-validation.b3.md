# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · yes · the Markup numeric attributes rule the task names for `size`'s parse, its clamp to a bound (rowspan is the example, the bound is the rule)
input-validation#1.52 · yes · the Markup numeric attributes rule the task names for `size`'s parse, a numeric parse with a fallback (`<ol start>` is the example)
input-validation#1.53 · yes · `attr_parsed` returns None on parse failure, the default fallback the chunk's `size` read must act on, and `disabled` read by presence rather than through it
input-validation#1.54 · yes · `disabled` read by presence on select, groups and options, which the chunk's key and pointer code must act on when opening the list and choosing a row
input-validation#1.55 · no · `open` on details or dialog, which the chunk does not read; the list's open state is an overlay, and element.rs and mutator.rs are not reached
input-validation#1.56 · no · autofocus on a focusable element, not read by the list; the focus and key contract is a founder fork
input-validation#1.57 · no · `dir` matching, not read or written by the list
input-validation#1.58 · no · canvas `src`, not touched by the chunk
input-validation#1.59 · yes · the open list's option stable id, which no author `id` may make equal to another and which reaches no log, the id the scope names
input-validation#1.60 · yes · a click on a select's label re-dispatches to the select, which the list's open path must receive, and a group's label is page text leaving only through the ratified carriers
input-validation#1.61 · no · descriptions are written by the accessibility builder, which the chunk does not reach; the list adds no description and no carrier for one
input-validation#1.62 · yes · `aria-expanded` as a closed boolean parse carries the list's open state into the tree, and `aria-disabled` moves no snapshot reading of `disabled`
input-validation#1.63 · yes · a select option's `selected` is its chosen state with no `aria-selected` read, which the option rows and the `selected` carrier must keep
input-validation#1.64 · no · a native checkbox or radio checked state under a checkable role, not touched by the chunk; its Space edit is for a single select only
input-validation#1.65 · yes · `inert` by presence on the select, its groups and options, which `hit_inner` refuses and the list's open and choose code must act on
input-validation#1.66 · no · the focus an inert subtree loses and the tree's `focus` are engine behaviour the chunk does not change; the open list's focus contract is a fork
input-validation#1.67 · yes · the arrows, `Home` and `End` on a focused range input in keyboard.rs, which the chunk's arrow edit on a single select must leave intact beside them
