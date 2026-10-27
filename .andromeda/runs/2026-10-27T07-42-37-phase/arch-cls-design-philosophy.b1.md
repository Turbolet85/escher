# Classified — arch · design-philosophy.b1

## Answers
design-philosophy#1.1 · yes · the root manifest's description (Cargo.toml, a merge surface) and the blitz-traits no-unnecessary-dependencies rule, which the winit IME copy must keep
design-philosophy#2.1 · yes · the shell provider's new clipboard write must keep DummyShellProvider's no-op default (blitz-traits shell.rs, a merge surface), and runtime.rs keeps fetch on the embedder's net provider
design-philosophy#3.1 · yes · HTML dimension parsing (stylo.rs), touch-action (events/pointer.rs) and style-attribute mutation (node/element.rs) sit on merge surfaces, and Enter and Space activation is named in the scope
design-philosophy#4.1 · yes · the incremental and non-incremental layout oracle runs over the document.rs merge surface and the construct.rs, scrolling.rs and node.rs conflicts
design-philosophy#5.1 · yes · the headless harness (no window, GPU or compositor) is what the re-run stand checks and re-measured screenshot numbers rest on, with the test-home line under standing S1
