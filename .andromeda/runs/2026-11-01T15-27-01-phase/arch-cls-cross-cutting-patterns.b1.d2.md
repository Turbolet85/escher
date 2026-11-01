# Classified — arch · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · the feature wiring of the root Cargo.toml (a conflict surface) and the `accessibility` gate of the `accessibility_tree` override that the a11y proof runs through
cross-cutting-patterns#2.1 · yes · the telemetry sink's target allowlist and field scrub in escher-telemetry format.rs, which the chunk re-reads, and the log call sites the taken delta adds
cross-cutting-patterns#3.1 · yes · dioxus_document.rs (a conflict surface) keeps the blitz document as Rc<RefCell<BaseDocument>> and the dioxus root contexts, and the redraw requests the changed-set contract drives
cross-cutting-patterns#4.1 · yes · construct.rs (a conflict surface) holds the set_ime_enabled(true) call in create_text_editor for a focused input before its editor exists
cross-cutting-patterns#5.1 · yes · events.rs and dioxus_document.rs (conflict surfaces) carry the JS microtask drain, the zero-handler event skip and the mounted-listener order that the resolutions must keep
cross-cutting-patterns#6.1 · yes · mutator.rs and node.rs (conflict surfaces) hold the changed-set write sites and readers, and the interaction-state and dirty-ancestor invariants the take must keep
