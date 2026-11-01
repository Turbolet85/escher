# Classified — arch · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · the `accessibility` feature gating the Dioxus bridge's `accessibility_tree` override and its snapshot modules, which the take's dioxus_document.rs resolution keeps and the a11y leg runs under; the DocumentConfig and launch-config parts are not reached
cross-cutting-patterns#2.1 · yes · the escher-telemetry sink's target drop and field scrub, which the task names: format.rs is re-read against the taken delta's new log and tracing call sites and the target allowlist
cross-cutting-patterns#3.1 · yes · the root-context and shared `Rc<RefCell<BaseDocument>>` shape at dioxus_document.rs:86, the shell_provider use at scrolling.rs:320 and the Tab key asking for no redraw at mutator.rs:76-78, all in predicted conflict surfaces
cross-cutting-patterns#4.1 · yes · the `set_ime_enabled(true)` call at construct.rs:860 and 867-872, in a predicted conflict surface reached whole
cross-cutting-patterns#5.1 · yes · the mounted-listener timing in dioxus_document.rs and the JS microtask drain in events.rs:171-173, both conflict surfaces, with 6ef4b532d editing the mounted-events path
cross-cutting-patterns#6.1 · yes · the changed set's five write sites and readers in mutator.rs, and the dirty and damaged ancestor invariants in node.rs, both predicted conflict surfaces the scope names through the changed-set contract
