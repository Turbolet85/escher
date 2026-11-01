# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the root manifest's crate description and the separately usable crates bind: Cargo.toml is a conflict surface, and the take's workspace and members surface is re-read
design-philosophy#2.1 · yes · the script engine's no-fetch rule, bootstrap globals and report_js_error bind through the conflicts in bootstrap.rs and events.rs, and the Dioxus bridge's VirtualDom integration through dioxus_document.rs
design-philosophy#3.1 · yes · spec-cited behaviour that lives in conflict surfaces binds: the HTML dimension parsing in stylo.rs beside the size hint, the text paint in text.rs, and the engine-debts merge rows
design-philosophy#4.1 · yes · the incremental/non-incremental oracle the chunk re-asserts, the changed-set and scrolling primitive in conflict surfaces, and persistent interaction state naming DOM nodes only, which the key-press fix stays bound by
design-philosophy#5.1 · yes · the headless harness and blitz-tests' integration-test placement, which the carried upstream test files and the pinned target list must honour
