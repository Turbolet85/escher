# Classified — security · input-validation.b4

## Answers
input-validation#1.65 · yes · mutator.rs (a conflict surface) and the reached element.rs and pointer.rs hold the select's rules, which the scope says a conflict resolution must leave unchanged
input-validation#1.66 · yes · the scope names the file input's chosen path in pointer.rs and its masked snapshot value, which must hold after pointer.rs moves
input-validation#1.67 · yes · the scope names degenerate aspect ratios as discarded, a layout value the take must keep
input-validation#1.68 · yes · the scope names inline-box heights kept finite, a layout value the take must keep
input-validation#1.69 · yes · the scope names text_extent staying read-only, and Session::lint is its one non-test caller
input-validation#1.70 · no · the culling sits in blitz-paint's render.rs, which the chunk does not reach and the scope never names
input-validation#1.71 · no · the gradient clamp sits in blitz-paint's gradient.rs, which the chunk does not reach and the scope never names
input-validation#1.72 · no · the radius scale-down sits in blitz-paint's kurbo_css/css_box.rs, which the chunk does not reach and the scope never names
input-validation#1.73 · no · the hidden-input paint skip sits in blitz-paint's render.rs, which the chunk does not reach and the scope never names
input-validation#1.74 · yes · scrolling.rs is a conflict surface, and the scope names scroll targets clamped to the range and a programmatic scroll to a missing node doing nothing
input-validation#1.75 · no · form.rs is not reached, and the scope names no method or enctype reading
input-validation#1.76 · yes · the exclusion of disabled controls from submission rests on the disabled reading the scope names by presence, which a conflict resolution must leave unchanged
input-validation#1.77 · no · form.rs is not reached, and the scope names no CRLF or submitted-value rule
input-validation#1.78 · yes · pointer.rs is reached whole, and the scope names its file-input path, where this accept rule sits
input-validation#1.79 · no · dioxus-native-dom's events.rs is not reached, and the scope names no form-validity reading
input-validation#1.80 · yes · dioxus_document.rs is reached and edited by 7fc9ee3af, and the scope names collision-free element ids that its data-dioxus-id parse feeds
input-validation#1.81 · no · mutation_writer.rs is not reached, and the scope names no inner-HTML rule
input-validation#1.82 · no · mutation_writer.rs is not reached, and the scope names no attribute-value-type rule
input-validation#1.83 · yes · the scope names disabled and inert by presence, and this falsy clear decides whether a Dioxus prop writes that presence, which the re-asserted stand checks read
input-validation#1.84 · yes · runtime.rs is a conflict surface, and the scope names the native APIs checking their arguments
input-validation#1.85 · yes · runtime.rs is a conflict surface, and the scope names fetch() taking GET or HEAD only
input-validation#1.86 · yes · dom/element.rs is reached whole and holds the selector SyntaxError, and the scope names the native APIs checking their arguments
input-validation#1.87 · yes · dom/element.rs is reached whole and holds the scroll-option throws, and the scope names the native APIs checking their arguments
input-validation#1.88 · yes · dom/element.rs is reached whole and holds the width and height reflection, and the scope names the native APIs coercing their arguments
input-validation#1.89 · yes · runtime.rs is a conflict surface and holds the timer coercion, and the scope names the native APIs coercing their arguments
