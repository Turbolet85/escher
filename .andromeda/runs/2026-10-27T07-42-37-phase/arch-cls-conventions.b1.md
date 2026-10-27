# Classified — arch · conventions.b1

## Answers
conventions#1.1 · yes · the fast and doc legs the scope re-runs hold the fmt, clippy and rustdoc gates of S3 and S4, and the reasoned local allow at layout/construct.rs:562 sits on a conflict surface
conventions#2.1 · yes · the workspace-dependency and metadata rules sit in the merge-surface manifests Cargo.toml, dioxus-native/Cargo.toml and blitz-shell/Cargo.toml, which a take's pins and any arboard removal edit (S6)
conventions#3.1 · yes · the module-level //! docs at scrolling.rs:1-2 (a conflict surface) and the spec URLs and public-item docs in stylo.rs and blitz-shell/src/lib.rs (merge surfaces) bind through the merges
conventions#4.1 · yes · the TODO/FIXME gap markers in scrolling.rs, layout/construct.rs, stylo.rs and events/pointer.rs and the section delimiters in runtime.rs and dom/element.rs sit on conflict or merge surfaces
conventions#5.1 · yes · the pub(crate) internals in document.rs and scrolling.rs, the __blitz_ natives in runtime.rs and the Dummy no-op in blitz-traits/src/shell.rs sit on conflict or merge surfaces
conventions#6.1 · yes · the tracing gates in mutator.rs, layout/construct.rs and blitz-shell/src/convert_events.rs, the desktop cfg list in blitz-shell/src/lib.rs and upstream's two new tracing::error! sites the scope re-reads sit on reached surfaces (S5)
conventions#7.1 · yes · the SAFETY comments in stylo.rs, the take-and-restore borrow work-arounds in layout/construct.rs, and the let-chains and accessor macro in node/node.rs and runtime.rs sit on merge or conflict surfaces
conventions#8.1 · yes · the unit-test modules at the file ends of mutator.rs, node/node.rs, document.rs and node/element.rs sit on reached surfaces, and S1 and S2 bind through the all.rs mod lines, the target pin and the re-run stand checks
conventions#9.1 · yes · the per-OS tally re-read the scope names must attribute recording::off_unix_tests, a cfg-not-unix check the windows job alone runs, so its rule binds though no escher-driver file is reached
conventions#9.2 · no · where a shared fixture lives is a convention every test-adding change owes, and tests/blitz-tests/tests/fixtures/mod.rs is not reached
conventions#9.3 · no · the MCP listing test and the 0.2.0 sweep's helper messages sit in files the take does not reach, and a re-run of the stand checks re-asserts results, not that message shape
conventions#10.1 · no · no apps/ or examples/ file is reached, and only a Dioxus 0.8 take or a carried apps/browser would reach them, both outside the reached list
conventions#11.1 · no · the Not-needed list names the example fixture forms and the HTML fixture conventions, and no examples/assets file is reached
