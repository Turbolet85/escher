# security · picked accounting · group 4 of 6

## Picked
input-validation#1.67 · carried · `0980cce5f` edits `layout/replaced.rs`, where the aspect-ratio discard lives, so a take that carries it moves the rule's lines
input-validation#1.68 · carried · `0980cce5f` edits `layout/inline/layout.rs`, where the finite inline-box height lives, and the conflicted `inline/build.rs` sits beside it
input-validation#1.69 · not in scope · no upstream commit in the 27 reaches `text_extent.rs` and the chunk does not change it, so its read-only reader is a re-check every change owes
input-validation#1.74 · carried · `scrolling.rs` is a predicted conflict surface that `0980cce5f` rewrites, and the scope requires scroll clamping to hold
input-validation#1.76 · not in scope · no upstream commit in the 27 reaches `form.rs`, so the submitted-control filter is a re-check every change owes
input-validation#1.78 · carried · `events/pointer.rs` is reached whole (`0980cce5f`), and the file input's path handling lives there
input-validation#1.80 · carried · `dioxus_document.rs` is reached whole (`7fc9ee3af`; `286874cc1` under a Dioxus 0.8 take), and the `data-dioxus-id` parse lives there
input-validation#1.83 · carried · `286874cc1`, one of the take fork's options, rewrites `mutation_writer.rs`, where the falsy boolean-attribute removal (`inert` among them) lives
input-validation#1.84 · carried · `runtime.rs` is a conflict surface that `e6f4615ec` splits into modules, and the `Response` status check moves with it
input-validation#1.85 · carried · the `fetch()` method check moves with `runtime.rs`'s split, and `cee6b8731` edits the fetch path
input-validation#1.86 · carried · `dom/element.rs` is reached whole and `e6f4615ec` moves its selector code into `element/selectors.rs`
input-validation#1.87 · carried · the scroll-option checks in `dom/element.rs` move into `element/scrolling.rs` with the split
input-validation#1.88 · carried · the `width`/`height` reflection in `dom/element.rs` moves with the split
input-validation#1.89 · carried · the timer coercions in `runtime.rs` move into `runtime/timers.rs` with the split
input-validation#1.90 · carried · `CSS.registerProperty`'s checks in `runtime.rs` move into `runtime/css.rs` with the split
input-validation#1.91 · carried · `efdc5472a`, named by the scope, adds CSS property accessors whose setters call `setProperty`, so this rule governs them
input-validation#1.92 · not in scope · no upstream commit in the 27 reaches `dom/mod.rs` or `dom/node.rs`, so the node-argument checks are a re-check every change owes
input-validation#1.94 · not in scope · no upstream commit in the 27 reaches `dom/selection.rs` or `tests/selection.rs`, so the selection checks are a re-check every change owes
input-validation#1.109 · carried · the chunk re-asserts the stand checks that witness these inputs, and `286874cc1` edits `seven_guis/src/tasks/cells.rs`
