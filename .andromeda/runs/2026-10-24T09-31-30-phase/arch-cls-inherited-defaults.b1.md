# Classified — arch · inherited-defaults.b1

## Answers
inherited-defaults#1.1 · yes · the manifest merge and the Dioxus package sync touch the workspace version and in-repo `=` pins, and the new mimalloc dependency lands in the publish = false app and WPT crates
inherited-defaults#2.1 · yes · the tokio update must turn the red cargo-deny Dependency audit green, and the ci-leg fmt, clippy, doc, audit and coverage legs gate the merged tree
inherited-defaults#3.1 · yes · the Telemetry and logging Need keeps tracing gated per call site in the engine crates, and the sink's allowlist is re-checked against upstream's new println! in examples/multi_doc.rs
