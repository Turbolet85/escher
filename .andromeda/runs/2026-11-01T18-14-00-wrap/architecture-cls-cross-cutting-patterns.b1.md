# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · the cited lines in `packages/blitz-dom/src/document.rs` (a merge surface) and `packages/blitz-dom/src/lib.rs` (reached) move with the change
cross-cutting-patterns#2.1 · no · the session adds no tracing, print or wall-clock site, and every file this entry cites (manifests, examples, escher-telemetry, WPT runner, vibey-script) is unreached, so the claim holds
cross-cutting-patterns#3.1 · yes · stale · the cited lines in the merge surfaces `mutator.rs`, `net.rs`, `document.rs` and `blitz-paint/src/render.rs`, and in reached `select_popup.rs`, move with the change
cross-cutting-patterns#4.1 · yes · stale · the cited lines 878 and 885-890 of `packages/blitz-dom/src/layout/construct.rs`, a merge surface, move with the change
cross-cutting-patterns#5.1 · yes · stale · the cited lines of `packages/dioxus-native-dom/src/dioxus_document.rs`, a merge surface, move with the change
cross-cutting-patterns#6.1 · yes · add-to · `append_text_to_node` now also damages its parent through `f76777766`, a damage rule this entry's propagation claim must carry; its cited merge-surface lines also move
