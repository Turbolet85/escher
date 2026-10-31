# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · no · rowspan's clamp sits in table layout (table.rs), which the chunk does not reach
input-validation#1.52 · no · `<ol start>` parsing sits in construct.rs, a list-numbering read the chunk does not reach
input-validation#1.53 · yes · the chunk's `size` read, if it goes through `attr_parsed`, takes its None-on-parse-failure fallback under the markup numeric rule; `disabled` is read by presence, not through it
input-validation#1.54 · yes · `disabled` by presence on a select (a can_be_disabled control): the chunk's key and pointer code must act on it on the select, its groups and its options when opening the list and choosing a row, and the Enter and Space arms it edits read it
input-validation#1.55 · no · the `open` attribute on details and dialog, with ElementState::OPEN and `:open`, is not read by the chunk; the select's open state is aria-expanded (1.62)
input-validation#1.56 · no · autofocus on a focusable element (mutator.rs); the chunk adds no autofocus read and its focus contract is a founder fork
input-validation#1.57 · no · `dir` is matched case-insensitively and the chunk reads no `dir`
input-validation#1.58 · no · canvas `src` u64 parsing; no canvas is in the chunk's reach
input-validation#1.59 · yes · the open list's option id: the anchored-path derivation, no author key able to make two ids equal, and lookup by `element_ids` and `get` with no NodeId kept, so the option is acted on by its id alone (its kept-check clause is S3's and is not the reason it binds)
input-validation#1.60 · yes · a select's `<label for>` re-dispatches its click to the select, and the group label and option names are page text that reach the snapshot and the carriers the list uses, with no panic path
input-validation#1.61 · no · `aria-describedby` and `aria-description` are read by the builder for any node; the chunk adds no description read or describedby target
input-validation#1.62 · yes · `aria-expanded` as a closed boolean carries the select's open state into the accessibility tree, the list's `expanded` reading a closed value, never the attribute's text (aria-disabled and aria-level are not reached)
input-validation#1.63 · yes · a select option's `selected` is its chosen state, read into the snapshot as the closed words `selected` and `unselected`; the open list's option states cross the carriers as closed words and booleans only
input-validation#1.64 · no · a native checkbox's or radio's checked state under a checkable role; the chunk's select and option rows are not native checkables
input-validation#1.65 · yes · `inert` by presence on the select, its groups and its options must be acted on when opening the list and choosing a row (focusability, hit testing and actionable reads)
input-validation#1.66 · no · the inert focus clear in DocumentMutator::flush and the tree's `focus` are engine effects the chunk neither calls nor reads; its inert acts are at open and choose, under 1.65
input-validation#1.67 · yes · pointer.rs, reached whole, holds the range pointerdown arm (452-456) and click arm (768-780) beside the new select branches, which must keep holding; the range arrow keys in keyboard.rs are another unit the chunk does not edit, a re-check
