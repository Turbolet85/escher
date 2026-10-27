# Classified — security-plan · input-validation.b4

## Answers
input-validation#1.65 · yes · stale · mutator.rs, a conflict surface that gained lines at 368-371 and 520-523, shifts its cites at 1033-1257, and element.rs and dioxus_document.rs, both reached, hold its other cites
input-validation#1.66 · no · its cited snapshot, paint, pointer and driver files are not reached, and the windowed redraw route changes when a frame is requested, not what the mask paints
input-validation#1.67 · no · its cited replaced.rs aspect-ratio code is not reached
input-validation#1.68 · no · its cited inline/layout.rs line-breaker clamp is not reached
input-validation#1.69 · no · its cited text_extent.rs reader is not reached
input-validation#1.70 · no · its cited blitz-paint render.rs culling is not reached
input-validation#1.71 · no · its cited gradient.rs clamps are not reached
input-validation#1.72 · no · its cited kurbo_css css_box.rs radius scaling is not reached
input-validation#1.73 · no · its cited blitz-paint render.rs hidden-input skip is not reached
input-validation#1.74 · no · its cited blitz-dom scrolling.rs clamp and missing-node return are not reached, and the moved JS scroll caller calls the same engine method
input-validation#1.75 · no · its cited form.rs method and enctype parsing is not reached
input-validation#1.76 · no · its cited form.rs submitted-control filter is not reached
input-validation#1.77 · no · its cited form.rs CRLF normalization is not reached
input-validation#1.78 · no · its cited pointer.rs accept handling is not reached
input-validation#1.79 · no · its cited dioxus-native events.rs validity stub is not reached
input-validation#1.80 · yes · stale · dioxus_document.rs is a conflict surface, so its cite at 32-39 may have moved and its usize parse reading is unverified on the changed file
input-validation#1.81 · no · its cited mutation_writer.rs set_inner_html call is not reached
input-validation#1.82 · no · its cited mutation_writer.rs value-type match is not reached
input-validation#1.83 · no · its cited mutation_writer.rs falsy-clear list and test are not reached, and the 28-name list is unchanged
input-validation#1.84 · yes · stale · runtime.rs is a merge surface that keeps 151 lines after its code moved into the runtime/ modules, so the cite at 120-123 moves
input-validation#1.85 · yes · stale · runtime.rs is a merge surface that keeps 151 lines after its code moved into the runtime/ modules, so the cite at 164-168 moves and the method rule is not shown to move
input-validation#1.86 · yes · stale · dom/element.rs is a merge surface whose element bindings moved into dom/element/ modules, so the selector cite at 1450-1515 moves
input-validation#1.87 · yes · stale · dom/element.rs is a merge surface whose element bindings moved into dom/element/ modules, so the cites at 1181-1198, 1236-1241 and 1328-1350 move
input-validation#1.88 · yes · stale · dom/element.rs is a merge surface whose element bindings moved into dom/element/ modules, so the cites at 683-699 and 737-748 move
input-validation#1.89 · yes · stale · runtime.rs is a merge surface that keeps 151 lines after its code moved into the runtime/ modules, so the cites at 2069-2077 and 2396-2398 no longer hold
