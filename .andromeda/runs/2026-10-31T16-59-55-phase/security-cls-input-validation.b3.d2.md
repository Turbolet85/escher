# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · no · rowspan's clamp in table.rs; the chunk reads no rowspan and touches no table layout
input-validation#1.52 · no · `<ol start>` parse; the chunk reads no ol start
input-validation#1.53 · yes · `size`, read here, goes through attr_parsed's None on parse failure and the default fallback
input-validation#1.54 · yes · disabled by presence on the select, its groups and options; the chunk's key and pointer code must act on it when opening the list and when choosing a row
input-validation#1.55 · no · the `open` attribute on details and dialog only; the list's open state is aria-expanded, not `open`
input-validation#1.56 · no · autofocus of a focusable element in mutator.rs; the chunk's scope names no autofocus
input-validation#1.57 · no · `dir` matching; the chunk reads no dir attribute
input-validation#1.58 · no · canvas `src`; the chunk touches no canvas
input-validation#1.59 · yes · an option's stable element id in the open list: acted on by that id alone, no author `id` makes two equal, no NodeId kept between calls
input-validation#1.60 · yes · a click on a select's label re-dispatches to the select, so it reaches the list's open path in pointer.rs; a group's label leaves escher only by the ratified name carriers
input-validation#1.61 · no · aria-describedby and aria-description; the chunk reads no description and no carrier of one
input-validation#1.62 · yes · aria-expanded's closed boolean parse carries the open list's state into the accessibility tree with no attribute text
input-validation#1.63 · yes · an option's `selected` is its chosen state, written through apply_select_choice and read by the snapshot's NodeState.selected with no aria-selected read; carriers are closed words and booleans
input-validation#1.64 · no · a native checkbox's or radio's checked state; the chunk touches no native checkable
input-validation#1.65 · yes · inert by presence on the select, its groups and options: hit testing, focusability and the list's key and pointer choice must refuse an inert node
input-validation#1.66 · no · inert focus clear in the mutator flush and the tree's focus field; neither unit is reached and the chunk's code makes no focus write for it
input-validation#1.67 · yes · the range keys and pointerdown arms at keyboard.rs:102-107 and pointer.rs:452-456 sit beside the chunk's new select arrow branches and must stay as they are
