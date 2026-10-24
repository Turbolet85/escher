# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the merged engine and Dioxus code must pass fmt, workspace clippy under `-D warnings`, blitz-dom's own clippy line and the rustdoc `-D warnings` leg (ci-leg fast and doc)
conventions#2.1 · yes · upstream's new mimalloc dependency, the Dioxus package sync and the manifest merge land through `[workspace.dependencies]`, `default-features = false` on in-repo crates and `publish = false` on example and app crates
conventions#3.1 · yes · the take's new modules (the inline layout split, `mutation_buffer.rs`) and its new root example `examples/multi_doc.rs` must open with `//!` docs and carry `///` on public items
conventions#4.1 · yes · the "WARNING: DO NOT REORDER" block in `mutation_writer.rs` and the TODO/FIXME markers in the conflict files (`layout/construct.rs`, `layout/inline.rs`, `scrolling.rs`, `events/pointer.rs`) must survive the Dioxus 0.8, layout and scrolling merges
conventions#5.1 · yes · the crate-private `node::select` and the layout split's internals stay `pub(crate)` or `pub(super)`, and the Stylo 0.23 and Taffy-main moves reach the `stylo`/`parley` aliases and stylo_taffy's feature-gated `#[inline]` conversions
conventions#6.1 · yes · tracing stays gated per call site with its fallback in the engine crates the take edits, and the Dioxus `trace!` macro and feature-list docs move with the 0.8 re-sync
conventions#7.1 · yes · each `unsafe` block the take brings or moves keeps its `SAFETY:` comment or `allow`, and the take-out-and-restore borrow workarounds in `construct.rs` and `inline.rs` survive the layout split
conventions#8.1 · yes · one target per file with `tests/all.rs` held at `test = false`, both layout modes for every scenario, and the `#[path]` test children of `dioxus_document.rs` and `document.rs` kept through the merge, which the new targets follow
conventions#9.1 · no · the take changes no escher-driver code or test file (the task's Not-needed list), so `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` stay as they are
conventions#9.2 · no · the take changes no fixture page or the shared `fixtures/mod.rs` (fixture forms are on the Not-needed list), and the re-run checks read the module unchanged
conventions#10.1 · yes · the Dioxus re-sync and upstream's `examples/seven_guis/src/tasks/cells.rs` edit must keep the example shape (`fn main` calling `dioxus_native::launch`, `#[component]`, hooks, `const CSS`), and the GUI binaries keep their `windows_subsystem` line while mimalloc lands in the browser
conventions#11.1 · no · the take changes no page under `examples/assets` and adds none (fixture forms are on the Not-needed list)
